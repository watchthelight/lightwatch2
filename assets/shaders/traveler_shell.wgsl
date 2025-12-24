// Traveler shell shader - translucent outer layer with Fresnel

#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view

struct ShellMaterial {
    base_color: vec4<f32>,
    refraction_strength: f32,
    thickness: f32,
    ior: f32,
    pulse_intensity: f32,
    time: f32,
    _padding: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> material: ShellMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);

    // Fresnel for transparency variation
    let fresnel = pow(1.0 - abs(dot(view_dir, normal)), 3.0);

    // Chromatic aberration at edges
    let aberration = fresnel * 0.02;

    // Base transparency
    var alpha = material.base_color.a * (0.3 + fresnel * 0.4);

    // Pulsing transparency
    let pulse = sin(material.time * 3.14159 + in.world_position.x) * 0.5 + 0.5;
    alpha *= 0.8 + pulse * 0.2 * material.pulse_intensity;

    // Color with slight iridescence
    var color = material.base_color.rgb;
    color.r += aberration;
    color.b -= aberration * 0.5;

    return vec4<f32>(color, alpha);
}
