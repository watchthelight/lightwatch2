// Bang core explosion shader - white-hot expanding explosion
// Dynamic temperature-based color, radial glow, pulsing intensity

#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view

struct BangCoreMaterial {
    time: f32,
    intensity: f32,
    temperature: f32,
    expansion: f32,
    color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material: BangCoreMaterial;

// Temperature to color (blackbody approximation)
// 1.0 = white hot, 0.0 = dark red
fn temperature_to_color(temp: f32) -> vec3<f32> {
    // White -> Amber -> Red transition
    let white = vec3<f32>(1.0, 0.98, 0.95);
    let amber = vec3<f32>(0.91, 0.64, 0.27);
    let red = vec3<f32>(0.7, 0.2, 0.1);

    if (temp > 0.5) {
        let t = (temp - 0.5) * 2.0;
        return mix(amber, white, t);
    } else {
        let t = temp * 2.0;
        return mix(red, amber, t);
    }
}

// Noise function for organic flickering
fn hash(p: vec3<f32>) -> f32 {
    var p3 = fract(p * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise3d(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(
            mix(hash(i), hash(i + vec3<f32>(1.0, 0.0, 0.0)), u.x),
            mix(hash(i + vec3<f32>(0.0, 1.0, 0.0)), hash(i + vec3<f32>(1.0, 1.0, 0.0)), u.x),
            u.y
        ),
        mix(
            mix(hash(i + vec3<f32>(0.0, 0.0, 1.0)), hash(i + vec3<f32>(1.0, 0.0, 1.0)), u.x),
            mix(hash(i + vec3<f32>(0.0, 1.0, 1.0)), hash(i + vec3<f32>(1.0, 1.0, 1.0)), u.x),
            u.y
        ),
        u.z
    );
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    // View direction for radial effects
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);

    // Core density - stronger toward center (view-aligned surfaces)
    let depth_factor = max(dot(view_dir, normal), 0.0);
    let core_density = pow(depth_factor, 0.5);

    // Edge glow (Fresnel-like)
    let edge = 1.0 - depth_factor;
    let edge_glow = pow(edge, 2.0);

    // Temperature-based color
    let base_color = temperature_to_color(material.temperature);

    // Pulsing effect - faster at high intensity
    let pulse_speed = 8.0 + material.intensity * 4.0;
    let pulse = sin(material.time * pulse_speed) * 0.1 + 0.9;

    // Noise-based flickering for organic feel
    let noise_pos = in.world_position.xyz * 2.0 + material.time * 3.0;
    let flicker = noise3d(noise_pos) * 0.2 + 0.8;

    // Secondary noise layer for turbulence
    let turbulence_pos = in.world_position.xyz * 5.0 - material.time * 1.5;
    let turbulence = noise3d(turbulence_pos) * 0.15 + 0.85;

    // Combine for hot interior with glowing edge
    let interior = core_density * 1.5;
    let exterior = edge_glow * 0.8;
    let combined = (interior + exterior) * pulse * flicker * turbulence;

    // Intensity multiplier (can exceed 1.0 for bloom)
    let brightness = combined * material.intensity;

    // Hot center burns white, edges show temperature color
    let center_heat = pow(depth_factor, 2.0) * material.temperature;
    let final_color = mix(base_color, vec3<f32>(1.0, 1.0, 1.0), center_heat * 0.7);

    // Apply brightness (HDR values trigger bloom)
    let hdr_color = final_color * brightness * 2.0;

    // Alpha based on intensity and position
    let alpha = min(brightness * 0.8 + 0.2 * material.intensity, 1.0);

    return vec4<f32>(hdr_color, alpha);
}
