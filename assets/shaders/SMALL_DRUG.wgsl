#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

struct Time {
    padding_1: u32,
    padding_2: u32,
    padding_3: u32,
    time: u32,
}

@group(1) @binding(2)
var<uniform> time: Time;

let PI: f32 = 3.14159265358979323846;

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
    let offset_strength = 0.004;
    let time_offset = sin(time / 100.0) / 100.0  * noise(uv * 10.0);

    // Sample each color channel with an arbitrary shift
    var co_color = vec4<f32>(
        textureSample(texture, our_sampler, uv + vec2<f32>(offset_strength, -offset_strength)).r,
        textureSample(texture, our_sampler, uv + vec2<f32>(-offset_strength, 0.0)).g,
        textureSample(texture, our_sampler, uv + vec2<f32>(0.0, offset_strength)).b,
        1.0
    );

    var output_color = vec4<f32>(
        mix(co_color.r, noise(uv + vec2<f32>(offset_strength + sin(time / 7000.0) / 50.0, -offset_strength) * 500.0), 0.3),
        mix(co_color.g, noise(uv + vec2<f32>(-offset_strength - sin(time / 7000.0) / 50.0, 0.0) * 500.0), 0.3),
        mix(co_color.b, noise(uv + vec2<f32>(0.0, offset_strength + sin(time / 7000.0) / 50.0) * 500.0), 0.3),
        1.0
    );

    // var output_color = vec4<f32>(
    //     mix(co_color.r, co_color.g, (sin(time / 1000.0 + PI) / 2.0 + 0.5) * noise(uv * 10.0 + time / 1000.0)),
    //     mix(co_color.g, co_color.b, (sin(time / 2000.0 - PI/3.4) / 2.0 + 0.5) * noise(uv * 10.0 + time / 1000.0)),
    //     mix(co_color.b, co_color.r, (sin(time / 1324.0 + PI * 321.3) / 2.0 + 0.5) * noise(uv * 10.0 + time / 1000.0)),
    //     1.0
    // );

    return output_color;
}