// Film grain post-processing shader
// Procedural noise with brightness-responsive intensity

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct FilmGrainSettings {
    intensity: f32,
    time: f32,
    response: f32,
    _padding: f32,
};

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var screen_sampler: sampler;
@group(0) @binding(2)
var<uniform> settings: FilmGrainSettings;

// High-quality hash function for grain
fn hash(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

// Animated grain noise
fn grain(uv: vec2<f32>, time: f32) -> f32 {
    // Use different time offsets for x and y to avoid patterns
    let seed = uv + vec2<f32>(time * 0.1, time * 0.17);
    return hash(seed * 1000.0) * 2.0 - 1.0;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let original = textureSample(screen_texture, screen_sampler, in.uv);

    // Generate grain noise
    let noise = grain(in.uv, settings.time);

    // Photographic response: reduce grain in bright areas
    let luminance = dot(original.rgb, vec3<f32>(0.299, 0.587, 0.114));
    let grain_amount = settings.intensity * (1.0 - luminance * settings.response);

    // Apply grain (additive)
    let grained = original.rgb + vec3<f32>(noise * grain_amount);

    return vec4<f32>(grained, original.a);
}
