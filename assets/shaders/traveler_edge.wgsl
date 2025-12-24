// Traveler edge shader - glowing wireframe with pulse

#import bevy_pbr::forward_io::VertexOutput

struct EdgeMaterial {
    color: vec4<f32>,
    glow_intensity: f32,
    pulse_phase: f32,
    time: f32,
    line_width: f32,
};

@group(2) @binding(0)
var<uniform> material: EdgeMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Animated pulse
    let pulse = sin(material.time * 6.28318 + material.pulse_phase) * 0.5 + 0.5;

    // Edge intensity
    let intensity = material.glow_intensity * (0.7 + pulse * 0.3);

    // Glow color
    let glow = material.color.rgb * intensity;

    // Additive blending will be handled by blend mode
    return vec4<f32>(glow, material.color.a * intensity);
}
