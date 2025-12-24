// Bang shockwave shader - expanding ring distortion
// Will be expanded in Prompt 31

struct ShockwaveUniforms {
    time: f32,
    radius: f32,
    thickness: f32,
    distortion: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: ShockwaveUniforms;

@group(0) @binding(1)
var screen_texture: texture_2d<f32>;

@group(0) @binding(2)
var screen_sampler: sampler;

@fragment
fn fragment(@builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Placeholder: passthrough
    return textureSample(screen_texture, screen_sampler, uv);
}
