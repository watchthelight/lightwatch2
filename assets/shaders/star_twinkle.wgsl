// Star twinkle shader - flickering starfield
// Will be expanded in Prompt 23

struct StarUniforms {
    time: f32,
    twinkle_speed: f32,
    brightness: f32,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: StarUniforms;

@fragment
fn fragment(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    // Placeholder: simple white point
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
