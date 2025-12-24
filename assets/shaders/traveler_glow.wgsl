// Traveler glow shader - pulsing emissive effect
// Will be expanded in Prompt 19

#import bevy_pbr::forward_io::VertexOutput

struct TravelerGlowMaterial {
    base_color: vec4<f32>,
    pulse_intensity: f32,
    time: f32,
    pulse_phase: f32,
    _padding: f32,
};

@group(2) @binding(0)
var<uniform> material: TravelerGlowMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pulse = sin(material.time * 6.28318 + material.pulse_phase) * 0.5 + 0.5;
    let glow = material.base_color * (1.0 + pulse * material.pulse_intensity);
    return glow;
}
