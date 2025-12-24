// Chromatic aberration post-process shader
// Will be expanded in Prompt 39

struct ChromaticUniforms {
    intensity: f32,
    offset: vec2<f32>,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: ChromaticUniforms;

@group(0) @binding(1)
var screen_texture: texture_2d<f32>;

@group(0) @binding(2)
var screen_sampler: sampler;

@fragment
fn fragment(@builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Placeholder: passthrough
    return textureSample(screen_texture, screen_sampler, uv);
}
