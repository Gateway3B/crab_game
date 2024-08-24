// water_shader.wgsl

// Vertex input structure
struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

// Vertex output structure
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) blend_color: vec4<f32>,
};

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

// Vertex shader entry point
@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Calculate wave effect
    let time =  f32(input.vertex_index) * 0.1;
    let wave_height = 1 * sin(input.position.x * 4.0 + input.position.z * 4.0 + time);
    let wave = vec3<f32>(0.0, wave_height, 0.0);
    output.position = vec4<f32>(input.position + wave, 1.0);
    output.blend_color = input.blend_color;

    return output;
}

// Fragment shader entry point
@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 1.0, 0.5); // Water color
}