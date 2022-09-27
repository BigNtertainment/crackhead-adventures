#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

struct Time {
    time: u32,
    padding_1: u32,
    padding_2: u32,
    padding_3: u32,
}

@group(1) @binding(2)
var<uniform> time: Time;

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

fn random (st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy, vec2(13.23142, 53.41223))) * 4234234.35234);
}

fn noise (st: vec2<f32>) -> f32 {
    let i: vec2<f32> = floor(st);
    let f:vec2<f32> = fract(st);

    let a: f32 = random(i);
    let b: f32 = random(i + vec2<f32>(1.0, 0.0));
    let c: f32 = random(i + vec2<f32>(0.0, 1.0));
    let d: f32 = random(i + vec2<f32>(1.0, 1.0));

    let u: vec2<f32> = f*f*(3.0 - 2.0*f);

    return mix(a, b, u.x) +
        (c - a)* u.y * (1.0 - u.x) +
        (d - b) * u.x * u.y;
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let time = f32(time.time);
    // Get screen position with coordinates from 0 to 1
    let uv = (position.xy / vec2<f32>(view.width, view.height));
    let offset_strength = 0.002;
    let time_offset = sin(time / 100.0) / 100.0  * noise(uv * 10.0 + time / 1000.0);

    // Sample each color channel with an arbitrary shift
    var output_color1 = vec4<f32>(
        textureSample(texture, our_sampler, uv + vec2<f32>(offset_strength, -offset_strength)).r,
        textureSample(texture, our_sampler, uv + vec2<f32>(-offset_strength + time_offset, 0.0)).g,
        textureSample(texture, our_sampler, uv + vec2<f32>(0.0, offset_strength + time_offset)).b,
        1.0
        );

    var output_color: vec4<f32> = adjust_hue(output_color1, 123.0);

    return output_color;
}