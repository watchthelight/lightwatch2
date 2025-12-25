//! Film grain render node
//!
//! Implements a render graph node for film grain post-processing.
//! Adds animated noise for cinematic texture and organic feel.

use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
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

use super::vignette_node::VignetteLabel;

/// Label for the film grain node in the render graph
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct FilmGrainLabel;

/// Component that controls film grain settings per camera
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct FilmGrainSettings {
    /// Grain intensity (0.0 = none, higher = more visible grain)
    pub intensity: f32,
    /// Time value for animated noise
    pub time: f32,
    /// Response to luminance (0.0 = uniform, 1.0 = less grain in bright areas)
    pub response: f32,
    pub _padding: f32,
}

impl FilmGrainSettings {
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity,
            time: 0.0,
            response: 0.5, // Default: reduce grain in bright areas
            _padding: 0.0,
        }
    }
}

/// Plugin that sets up film grain post-processing
pub struct FilmGrainPlugin;

impl Plugin for FilmGrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<FilmGrainSettings>::default(),
            UniformComponentPlugin::<FilmGrainSettings>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<FilmGrainNode>>(Core3d, FilmGrainLabel);
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // Add edges in finish() after all nodes are registered
        // Vignette → FilmGrain → End
        render_app.add_render_graph_edges(
            Core3d,
            (
                VignetteLabel,
                FilmGrainLabel,
                Node3d::EndMainPassPostProcessing,
            ),
        );

        render_app.init_resource::<FilmGrainPipeline>();
    }
}

/// The render node for film grain
#[derive(Default)]
pub struct FilmGrainNode;

impl ViewNode for FilmGrainNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static DynamicUniformIndex<FilmGrainSettings>,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let grain_pipeline = world.resource::<FilmGrainPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let settings_uniforms = world.resource::<ComponentUniforms<FilmGrainSettings>>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(grain_pipeline.pipeline_id) else {
            return Ok(());
        };

        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "film_grain_bind_group",
            &grain_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &grain_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("film_grain_pass"),
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

/// Pipeline resource for film grain
#[derive(Resource)]
pub struct FilmGrainPipeline {
    pub layout: BindGroupLayout,
    pub sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for FilmGrainPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "film_grain_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<FilmGrainSettings>(true),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world.load_asset("shaders/film_grain.wgsl");

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("film_grain_pipeline".into()),
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
