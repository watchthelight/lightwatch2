// Bang core explosion shader
// Will be expanded in Prompt 28

struct BangCoreUniforms {
    time: f32,
    intensity: f32,
    radius: f32,
    color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: BangCoreUniforms;

@fragment
fn fragment(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    // Placeholder: bright white core
    return vec4<f32>(1.0, 1.0, 1.0, uniforms.intensity);
}
