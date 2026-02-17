@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    var x: f32;
    var y: f32;

    if in_vertex_index == 0u {
        x = 0.0;
        y = 0.5;
    } else if in_vertex_index == 1u {
        x = -0.5;
        y = -0.5;
    } else {
        x = 0.5;
        y = -0.5;
    }

    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.5, 0.2, 1.0);
}
