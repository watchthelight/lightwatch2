//! Vignette render node
//!
//! Implements a render graph node for vignette post-processing.
//! Darkens screen edges for cinematic focus.

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

/// Label for the vignette node in the render graph
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct VignetteLabel;

/// Component that controls vignette settings per camera
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct VignetteSettings {
    pub intensity: f32,
    pub midpoint: f32,
    pub softness: f32,
    pub _padding: f32,
}

impl VignetteSettings {
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity,
            midpoint: 0.4,
            softness: 0.3,
            _padding: 0.0,
        }
    }
}

/// Plugin that sets up vignette post-processing
pub struct VignettePlugin;

impl Plugin for VignettePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<VignetteSettings>::default(),
            UniformComponentPlugin::<VignetteSettings>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // Just add the node - edges are handled by god_rays_node (CA → GodRays → Vignette)
        // and film_grain_node (Vignette → FilmGrain → End)
        render_app
            .add_render_graph_node::<ViewNodeRunner<VignetteNode>>(Core3d, VignetteLabel);
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<VignettePipeline>();
    }
}

/// The render node for vignette
#[derive(Default)]
pub struct VignetteNode;

impl ViewNode for VignetteNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static DynamicUniformIndex<VignetteSettings>,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let vignette_pipeline = world.resource::<VignettePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let settings_uniforms = world.resource::<ComponentUniforms<VignetteSettings>>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(vignette_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "vignette_bind_group",
            &vignette_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &vignette_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("vignette_pass"),
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

/// Pipeline resource for vignette
#[derive(Resource)]
pub struct VignettePipeline {
    pub layout: BindGroupLayout,
    pub sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for VignettePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "vignette_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<VignetteSettings>(true),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world.load_asset("shaders/vignette.wgsl");

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("vignette_pipeline".into()),
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
