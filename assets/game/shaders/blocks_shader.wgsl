#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_normal_local_to_world}

struct BlockMaterial {
    sun_color: vec4<f32>,
    sun_dir: vec3<f32>, 
    ambient_color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: BlockMaterial;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) object_normal: vec3<f32>,
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = get_model_matrix(input.instance_index);
    let normal_matrix = transpose(model_matrix);

    out.clip_position = mesh_position_local_to_clip(model_matrix,vec4<f32>(input.position, 1.0));
    out.object_normal = input.normal;
    return out;
}

@fragment
fn fragment(out: VertexOutput) -> @location(0) vec4<f32> {
    let NdotL: f32 = dot(normalize(material.sun_dir), out.world_normal);
    let color: vec4<f32> = NdotL * material.sun_color + material.ambient_color;

    return color;
}