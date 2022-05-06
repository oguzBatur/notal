// Vertex Shader
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    
    
};

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index));
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragment Shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}