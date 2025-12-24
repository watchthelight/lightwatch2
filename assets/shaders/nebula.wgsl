// Nebula raymarched background shader
// Creates volumetric cloud-like nebula using 3D noise

#import bevy_pbr::{
    forward_io::VertexOutput,
}

struct NebulaMaterial {
    color1: vec4<f32>,
    color2: vec4<f32>,
    time: f32,
    intensity: f32,
    drift_speed: f32,
    noise_scale: f32,
};

@group(2) @binding(0)
var<uniform> material: NebulaMaterial;

// 3D hash function
fn hash3(p: vec3<f32>) -> vec3<f32> {
    var q = vec3<f32>(
        dot(p, vec3<f32>(127.1, 311.7, 74.7)),
        dot(p, vec3<f32>(269.5, 183.3, 246.1)),
        dot(p, vec3<f32>(113.5, 271.9, 124.6))
    );
    return fract(sin(q) * 43758.5453123) * 2.0 - 1.0;
}

// 3D gradient noise
fn noise3d(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);

    // Quintic interpolation for smooth gradients
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);

    return mix(
        mix(
            mix(dot(hash3(i + vec3<f32>(0.0, 0.0, 0.0)), f - vec3<f32>(0.0, 0.0, 0.0)),
                dot(hash3(i + vec3<f32>(1.0, 0.0, 0.0)), f - vec3<f32>(1.0, 0.0, 0.0)), u.x),
            mix(dot(hash3(i + vec3<f32>(0.0, 1.0, 0.0)), f - vec3<f32>(0.0, 1.0, 0.0)),
                dot(hash3(i + vec3<f32>(1.0, 1.0, 0.0)), f - vec3<f32>(1.0, 1.0, 0.0)), u.x), u.y),
        mix(
            mix(dot(hash3(i + vec3<f32>(0.0, 0.0, 1.0)), f - vec3<f32>(0.0, 0.0, 1.0)),
                dot(hash3(i + vec3<f32>(1.0, 0.0, 1.0)), f - vec3<f32>(1.0, 0.0, 1.0)), u.x),
            mix(dot(hash3(i + vec3<f32>(0.0, 1.0, 1.0)), f - vec3<f32>(0.0, 1.0, 1.0)),
                dot(hash3(i + vec3<f32>(1.0, 1.0, 1.0)), f - vec3<f32>(1.0, 1.0, 1.0)), u.x), u.y), u.z);
}

// Fractal Brownian Motion - 4 octaves (unrolled for performance)
fn fbm(p: vec3<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var pos = p;

    value += amplitude * noise3d(pos);
    amplitude *= 0.5;
    pos *= 2.0;

    value += amplitude * noise3d(pos);
    amplitude *= 0.5;
    pos *= 2.0;

    value += amplitude * noise3d(pos);
    amplitude *= 0.5;
    pos *= 2.0;

    value += amplitude * noise3d(pos);

    return value;
}

// Simple raymarch for nebula volume
fn raymarch_nebula(uv: vec2<f32>) -> vec4<f32> {
    var accumulated_color = vec3<f32>(0.0);
    var accumulated_alpha = 0.0;

    // Create ray from UV
    let aspect = 1920.0 / 1080.0;
    let rd = normalize(vec3<f32>(
        (uv.x - 0.5) * aspect,
        (uv.y - 0.5),
        -1.0
    ));

    let step_size = 3.0;
    let max_steps = 24;

    // Drift offset over time
    let drift = vec3<f32>(
        material.time * material.drift_speed * 0.1,
        material.time * material.drift_speed * 0.05,
        material.time * material.drift_speed * 0.03
    );

    for (var i = 0; i < max_steps; i = i + 1) {
        let t = f32(i) * step_size + 20.0;
        let pos = rd * t;

        // Sample noise at position with drift
        let sample_pos = pos * material.noise_scale + drift;
        let density = fbm(sample_pos);

        // Only positive densities contribute
        if (density > 0.0) {
            // Color gradient based on position
            let color_mix = (noise3d(pos * 0.02 + drift * 0.5) + 1.0) * 0.5;
            let nebula_color = mix(material.color1.rgb, material.color2.rgb, color_mix);

            // Accumulate with front-to-back compositing
            let sample_alpha = density * 0.08 * material.intensity;
            accumulated_color += nebula_color * sample_alpha * (1.0 - accumulated_alpha);
            accumulated_alpha += sample_alpha * (1.0 - accumulated_alpha);

            if (accumulated_alpha > 0.9) {
                break;
            }
        }
    }

    return vec4<f32>(accumulated_color, accumulated_alpha);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Get nebula contribution
    let nebula = raymarch_nebula(in.uv);

    // Subtle cosmic background gradient
    let bg_gradient = mix(
        vec3<f32>(0.02, 0.01, 0.03),
        vec3<f32>(0.01, 0.02, 0.04),
        in.uv.y
    );

    // Combine background with nebula
    let final_color = bg_gradient + nebula.rgb;

    return vec4<f32>(final_color, 1.0);
}
