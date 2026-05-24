//! Basic fragment shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};

struct Light {
    direction: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
};

@group(0) @binding(2)
var<uniform> light: Light;

@group(2) @binding(0)
var base_color: vec4<f32>;

@group(2) @binding(1)
var base_texture: texture_2d<f32>;

@group(2) @binding(2)
var base_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture
    let texture_color = textureSample(base_texture, base_sampler, in.tex_coords);
    
    // Blend with base color
    let color = texture_color * base_color;
    
    // Calculate diffuse lighting
    let norm = normalize(in.normal);
    let light_dir = normalize(-light.direction);
    let diff = max(dot(norm, light_dir), 0.0);
    
    // Apply lighting
    let illuminated = color.rgb * light.color * light.intensity * diff;
    
    return vec4<f32>(illuminated, color.a);
}
