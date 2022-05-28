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

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return material.color * 0.1 * input.uv.xyxy * f32(material.frame / u32(1.25) % u32(30));
}