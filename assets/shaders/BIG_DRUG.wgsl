// https://github.com/Defernus/bevy_old_tv_shader

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

// sume constants
let screen_shape_factor:f32 = .5;
let rows: f32 = 256.0;
let brightness: f32 = 3.0;
let edges_transition_size: f32 = 0.2;
let channels_mask_min: f32 = 0.025;

fn get_uv(pos: vec2<f32>) -> vec2<f32> {
    return pos / vec2(view.width, view.height);
}

fn apply_screen_shape(uv: vec2<f32>, factor: f32) -> vec2<f32> {
    let uv = uv - vec2(0.5, 0.5);
    let uv = uv * (uv.yx * uv.yx * factor + 1.0);
    return uv + vec2(0.5, 0.5);
}

fn pixelate(uv: vec2<f32>, size: vec2<f32>) -> vec2<f32> {
    return floor(uv * size) / size;
}

fn get_texture_color(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(texture, our_sampler, uv);
}

fn apply_pixel_rows(color: vec4<f32>, uv: vec2<f32>, rows: f32) -> vec4<f32> {
    return color;
}

fn apply_pixel_cols(color: vec4<f32>, uv: vec2<f32>, cols: f32) -> vec4<f32> {
    return color;
}

fn apply_screen_edges(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let ratio = view.width / view.height;

    let edge_x = min(uv.x / edges_transition_size, (1.0 - uv.x) / edges_transition_size);
    let edge_y = min(uv.y / edges_transition_size / ratio, (1.0 - uv.y) / edges_transition_size / ratio);

    let edge = vec2(
        max(edge_x, 0.0),
        max(edge_y, 0.0),
    );
    let f = min(edge.x, edge.y);
    let f = min(f, 1.0);

    return vec4(color.xyz * f, 1.0);
} 

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

fn random(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy, vec2(13.23142, 53.41223))) * 4234234.35234);
}

fn noise(st: vec2<f32>) -> f32 {
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

fn cool_sine(x: f32, t: f32) -> f32 {
    let amplitude = 1.;
    let frequency = 1.;
    let y = sin(x * frequency);
    // float t = 0.01*(-u_time*130.0);
    // let t = 1.920;
    let y = sin(x*frequency*2.1 + t)*4.5 + y;
    let y = sin(x*frequency*1.72 + t*1.121)*4.0 + y;
    let y = sin(x*frequency*2.221 + t*0.437)*5.0 + y;
    let y = sin(x*frequency*3.1122+ t*4.269)*2.5 + y;
    let y = amplitude*0.06 * y;
    return y;
}

fn apply_brightness(color: vec4<f32>) -> vec4<f32> {
    return color * vec4(vec3(brightness), 1.0);
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let time = f32(time.time);

    let uv = get_uv(position.xy);
    let uv = apply_screen_shape(uv, screen_shape_factor);

    let cols = rows * view.width / view.height;

    let texture_uv = uv;
    // let texture_uv = pixelate(texture_uv, vec2(cols, rows));

    let color = get_texture_color(texture_uv);

    let color = apply_pixel_rows(color, uv, rows);
    let color = apply_pixel_cols(color, uv, cols);

    let color = apply_brightness(color);
    let color = apply_screen_edges(color, uv);

    let hue = 6.2839 * noise(uv + vec2<f32>(time / 2200.0)) 
        * cool_sine(uv.x + random(vec2<f32>(12.,12.1212)), (-time)/2000.0)
        * cool_sine(uv.y + random(vec2<f32>(65456.,1546456.)), (time)/2400.0);

    let color = adjust_hue(color, hue*2.);

    return color;
}