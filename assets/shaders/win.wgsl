#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> time: u32;

fn adjust_hue(color: vec4<f32>, hueAdjust: f32) -> vec4<f32> {
    let kRGBToYPrime: vec4<f32> = vec4<f32>(0.299, 0.587, 0.114, 0.0);
    let kRGBToI: vec4<f32> = vec4<f32>(0.596, -0.275, -0.321, 0.0);
    let kRGBToQ: vec4<f32> = vec4<f32>(0.212, -0.523, 0.311, 0.0);

    let kYIQToR: vec4<f32> = vec4<f32>(1.0, 0.956, 0.621, 0.0);
    let kYIQToG: vec4<f32> = vec4<f32>(1.0, -0.272, -0.647, 0.0);
    let kYIQToB: vec4<f32> = vec4<f32>(1.0, -1.107, 1.704, 0.0);

    // convert to YIQ
    let YPrime: f32 = dot(color, kRGBToYPrime);
    let I: f32 = dot(color, kRGBToI);
    let Q: f32 = dot(color, kRGBToQ);

    // calculate the heu and chroma
    let hue: f32 = atan2(Q, I);
    let chroma: f32 = sqrt(I * I + Q * Q);

    let hue = hue + hueAdjust;

    let Q = chroma * sin(hue);
    let I = chroma * cos(hue);

    let yIQ: vec4<f32> = vec4<f32>(YPrime, I, Q, 0.0);

    // color.r = dot(yIQ, kYIQToR);
    // color.g = dot(yIQ, kYIQToG);
    // color.b = dot(yIQ, kYIQToB);

    return vec4<f32>(dot(yIQ, kYIQToR), dot(yIQ, kYIQToG), dot(yIQ, kYIQToB), color.a);
} 

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let time = f32(time);
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy / vec2<f32>(view.width, view.height);

    // Sample each color channel with an arbitrary shift
    var output_color1 = vec4<f32>(textureSample(texture, our_sampler, uv));

    var output_color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    return output_color;
}