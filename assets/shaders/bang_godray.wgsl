// God ray shader - volumetric light beams
// Will be expanded in Prompt 30

struct GodrayUniforms {
    light_position: vec2<f32>,
    intensity: f32,
    decay: f32,
    density: f32,
    samples: i32,
    _padding: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: GodrayUniforms;

@group(0) @binding(1)
var screen_texture: texture_2d<f32>;

@group(0) @binding(2)
var screen_sampler: sampler;

@fragment
fn fragment(@builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Placeholder: passthrough
    return textureSample(screen_texture, screen_sampler, uv);
}
