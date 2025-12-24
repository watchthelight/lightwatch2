// God rays - Screen-space radial blur effect
// Samples from pixel toward light source with decay

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct GodRaySettings {
    light_position: vec2<f32>,  // Screen space (0-1)
    intensity: f32,
    decay: f32,
    density: f32,
    samples: i32,
    exposure: f32,
    _padding: f32,
};

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var screen_sampler: sampler;
@group(0) @binding(2)
var<uniform> settings: GodRaySettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Direction from pixel to light source
    let delta = settings.light_position - in.uv;
    let dist = length(delta);

    // Normalize and scale by density
    let ray_dir = delta / f32(settings.samples) * settings.density;

    var uv = in.uv;
    var accumulated_color = vec3<f32>(0.0);
    var decay_factor = 1.0;

    // Raymarch from pixel toward light
    for (var i = 0; i < settings.samples; i++) {
        uv += ray_dir;

        // Clamp UV to valid range
        let clamped_uv = clamp(uv, vec2<f32>(0.0), vec2<f32>(1.0));

        // Sample screen at offset
        let sample_color = textureSample(screen_texture, screen_sampler, clamped_uv);

        // Accumulate with decay
        accumulated_color += sample_color.rgb * decay_factor;
        decay_factor *= settings.decay;
    }

    // Original screen color
    let original = textureSample(screen_texture, screen_sampler, in.uv);

    // Blend god rays with original
    let god_rays = accumulated_color * settings.exposure * settings.intensity;

    // Distance-based falloff from light source
    let falloff = 1.0 - smoothstep(0.0, 0.8, dist);

    let final_color = original.rgb + god_rays * falloff;

    return vec4<f32>(final_color, original.a);
}
