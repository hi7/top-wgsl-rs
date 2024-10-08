// Vertex shader

[[block]]
struct Uniforms {
    offset: vec2<f32>;
    aspect: vec2<f32>;
    scale: vec2<f32>;
};
[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>((model.position + uniforms.offset) * uniforms.scale * uniforms.aspect, 0.0, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
