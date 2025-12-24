// Chromatic aberration post-processing shader
// RGB channel separation based on distance from center

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct ChromaticAberrationSettings {
    intensity: f32,
    center_x: f32,
    center_y: f32,
    _padding: f32,
};

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var screen_sampler: sampler;
@group(0) @binding(2)
var<uniform> settings: ChromaticAberrationSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(settings.center_x, settings.center_y);
    let direction = in.uv - center;
    let dist = length(direction);

    // Intensity increases toward edges (squared falloff)
    let edge_factor = pow(dist, 2.0);
    let offset = direction * settings.intensity * edge_factor;

    // Sample RGB channels at different offsets
    // Red shifts outward, blue shifts inward
    let r = textureSample(screen_texture, screen_sampler, in.uv + offset).r;
    let g = textureSample(screen_texture, screen_sampler, in.uv).g;
    let b = textureSample(screen_texture, screen_sampler, in.uv - offset).b;

    return vec4<f32>(r, g, b, 1.0);
}
