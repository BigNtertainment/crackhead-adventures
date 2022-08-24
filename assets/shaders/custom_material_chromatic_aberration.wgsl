#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> strength: f32;

@group(1) @binding(2)
var<uniform> time: Time

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy / vec2<f32>(view.width, view.height);
    let offset_strength = 0.002;

    // Sample each color channel with an arbitrary shift\
    var effect: i32 = 1;
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv + vec2<f32>(offset_strength, -offset_strength) * strength).r,
        textureSample(texture, our_sampler, uv + vec2<f32>(-offset_strength, 0.0) * strength).g,
        textureSample(texture, our_sampler, uv + vec2<f32>(0.0, offset_strength) * strength).b,
        1.0
        );

    return output_color;
}