#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_functions::{
    get_world_from_local,
    mesh2d_position_local_to_clip,
}
#import noisy_bevy::voronoise

struct BackgroundMaterial {
    params: vec2<f32>
}

@group(2) @binding(0) var<uniform> material: BackgroundMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) object_position: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position of the mesh to screen position
    let model = get_world_from_local(vertex.instance_index);
    out.clip_position = mesh2d_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    out.object_position = vertex.position.xy;
    return out;
}

// Fragment shader function
@fragment
fn fragment(
    vertex_output: VertexOutput,
) -> @location(0) vec4<f32> {
    // Scale factor for the input position
    let freq_scale: f32 = material.params.x;

    // Input position scaled by frequency
    let p = vertex_output.clip_position.xy * freq_scale * 0.5;

    let noise = voronoise(p + globals.time, 1.0, 0.0);
    let color = vec3(noise);

    return vec4<f32>(color, 1.0);
}