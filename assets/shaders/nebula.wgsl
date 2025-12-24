// Nebula raymarching shader - cosmic background
// Will be expanded in Prompt 24

struct NebulaUniforms {
    time: f32,
    density: f32,
    color_shift: f32,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: NebulaUniforms;

@fragment
fn fragment(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    // Placeholder: deep space black
    return vec4<f32>(0.0, 0.0, 0.02, 1.0);
}
