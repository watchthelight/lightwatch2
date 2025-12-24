// Traveler glow shader - inner glow, Fresnel rim, pulse animation

#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view

struct TravelerGlowMaterial {
    base_color: vec4<f32>,
    emissive: vec4<f32>,
    pulse_intensity: f32,
    pulse_phase: f32,
    time: f32,
    fresnel_power: f32,
    inner_glow_strength: f32,
    rim_color: vec4<f32>,
    grief_amount: f32,
    _padding: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> material: TravelerGlowMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    // Base color
    var color = material.base_color;

    // View direction for Fresnel
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);

    // Fresnel effect (rim lighting)
    let fresnel = pow(1.0 - max(dot(view_dir, normal), 0.0), material.fresnel_power);
    let rim = fresnel * material.rim_color;

    // Pulse animation
    let pulse = sin(material.time * 6.28318 * 0.5 + material.pulse_phase) * 0.5 + 0.5;
    let pulse_glow = pulse * material.pulse_intensity;

    // Inner glow - stronger toward center (approximated by view alignment)
    let depth_factor = max(dot(view_dir, normal), 0.0);
    let inner_glow = depth_factor * material.inner_glow_strength;

    // Combine emissive components
    var emissive = material.emissive.rgb;
    emissive += rim.rgb * 0.5;
    emissive += emissive * pulse_glow;
    emissive += emissive * inner_glow * 0.3;

    // Apply grief (desaturation)
    if (material.grief_amount > 0.0) {
        let gray = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
        color = vec4<f32>(
            mix(color.rgb, vec3<f32>(gray), material.grief_amount * 0.5),
            color.a
        );
        emissive *= 1.0 - material.grief_amount * 0.5;
    }

    // Final color
    let final_color = color.rgb + emissive;

    return vec4<f32>(final_color, color.a);
}
