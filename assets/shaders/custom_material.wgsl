
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct ShaderMaterial {
    color: vec4<f32>;
    frame: u32;
};

[[group(1), binding(0)]]
var<uniform> material: ShaderMaterial;

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    let p = input.uv - .5;
    let d = sdCircle(p, .25 + .25*sin(f32(material.frame)/20.0));
    return mix(
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(0.1, 0.1, 0.1, 1.0),
        smoothStep(0.0, 0.002, abs(d)));
}