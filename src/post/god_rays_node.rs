//! God rays render node
//!
//! Implements a render graph node for radial light scattering.
//! Creates volumetric light beams emanating from bang center.

use bevy::{
    core_pipeline::{
        core_3d::graph::Core3d,
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId,
            ColorTargetState, ColorWrites, FragmentState, MultisampleState, Operations,
            PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType,
        },
        renderer::{RenderContext, RenderDevice},
        view::ViewTarget,
        RenderApp,
    },
};

use super::chromatic_node::ChromaticAberrationLabel;
use super::vignette_node::VignetteLabel;

/// Label for the god rays node in the render graph
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct GodRaysLabel;

/// Component that controls god ray settings per camera
/// Layout must match the WGSL GodRaySettings struct
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct GodRaysSettings {
    /// Light source position in screen space (0-1)
    pub light_position: Vec2,
    /// Effect intensity (0 = off)
    pub intensity: f32,
    /// Decay per sample (0.9-0.99)
    pub decay: f32,
    /// Ray density
    pub density: f32,
    /// Number of samples
    pub samples: i32,
    /// Exposure multiplier
    pub exposure: f32,
    pub _padding: f32,
}

impl Default for GodRaysSettings {
    fn default() -> Self {
        Self {
            light_position: Vec2::new(0.5, 0.5),
            intensity: 0.0, // Off by default
            decay: 0.96,
            density: 0.8,
            samples: 50,
            exposure: 0.3,
            _padding: 0.0,
        }
    }
}

/// Plugin that sets up god ray post-processing render node
pub struct GodRaysRenderPlugin;

impl Plugin for GodRaysRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<GodRaysSettings>::default(),
            UniformComponentPlugin::<GodRaysSettings>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<GodRaysNode>>(Core3d, GodRaysLabel);
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // Add edges in finish() after all nodes are registered
        // CA → GodRays → Vignette
        render_app
            .add_render_graph_edge(Core3d, ChromaticAberrationLabel, GodRaysLabel)
            .add_render_graph_edge(Core3d, GodRaysLabel, VignetteLabel);

        render_app.init_resource::<GodRaysPipeline>();
    }
}

/// The render node for god rays
#[derive(Default)]
pub struct GodRaysNode;

impl ViewNode for GodRaysNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static DynamicUniformIndex<GodRaysSettings>,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let god_rays_pipeline = world.resource::<GodRaysPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let settings_uniforms = world.resource::<ComponentUniforms<GodRaysSettings>>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(god_rays_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "god_rays_bind_group",
            &god_rays_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &god_rays_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("god_rays_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

/// Pipeline resource for god rays
#[derive(Resource)]
pub struct GodRaysPipeline {
    pub layout: BindGroupLayout,
    pub sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for GodRaysPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "god_rays_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<GodRaysSettings>(true),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world.load_asset("shaders/god_rays.wgsl");

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("god_rays_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::Rgba16Float,
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}
