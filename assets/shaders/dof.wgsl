// Depth of Field post-process shader
// Bokeh-style blur based on circle of confusion
// Will be fully integrated in Prompt 39

struct DofSettings {
    focus_distance: f32,
    aperture: f32,
    max_blur: f32,
    near_blur_distance: f32,
    far_blur_distance: f32,
    enabled: f32,
    _padding: vec2<f32>,
};

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;

@group(0) @binding(1)
var screen_sampler: sampler;

@group(0) @binding(2)
var depth_texture: texture_depth_2d;

@group(0) @binding(3)
var<uniform> settings: DofSettings;

// Circular bokeh kernel (13 samples)
const BOKEH_SAMPLES: i32 = 13;
const BOKEH_OFFSETS: array<vec2<f32>, 13> = array<vec2<f32>, 13>(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(0.5, 0.866),
    vec2<f32>(-0.5, 0.866),
    vec2<f32>(-1.0, 0.0),
    vec2<f32>(-0.5, -0.866),
    vec2<f32>(0.5, -0.866),
    vec2<f32>(0.7, 0.7),
    vec2<f32>(-0.7, 0.7),
    vec2<f32>(-0.7, -0.7),
    vec2<f32>(0.7, -0.7),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(0.0, -1.0),
);

fn linearize_depth(depth: f32) -> f32 {
    let near = 0.1;
    let far = 1000.0;
    return near * far / (far - depth * (far - near));
}

fn calculate_coc(depth: f32) -> f32 {
    let linear_depth = linearize_depth(depth);
    let distance_from_focus = abs(linear_depth - settings.focus_distance);

    let blur_range = select(
        settings.far_blur_distance,
        settings.near_blur_distance,
        linear_depth < settings.focus_distance
    );

    let coc = min(distance_from_focus / blur_range, 1.0) * settings.max_blur;
    return coc;
}

@fragment
fn fragment(@builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Passthrough until render node integration
    return textureSample(screen_texture, screen_sampler, uv);
}
