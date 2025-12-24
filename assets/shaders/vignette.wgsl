// Vignette post-processing shader
// Smooth radial darkening toward edges

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct VignetteSettings {
    intensity: f32,
    midpoint: f32,
    softness: f32,
    _padding: f32,
};

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var screen_sampler: sampler;
@group(0) @binding(2)
var<uniform> settings: VignetteSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let original = textureSample(screen_texture, screen_sampler, in.uv);

    // Distance from center (normalized to corners = 1.0)
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(in.uv - center) * 1.414;

    // Smooth vignette falloff
    let vignette = 1.0 - smoothstep(settings.midpoint, settings.midpoint + settings.softness, dist);
    let vignette_factor = mix(1.0, vignette, settings.intensity);

    return vec4<f32>(original.rgb * vignette_factor, original.a);
}
