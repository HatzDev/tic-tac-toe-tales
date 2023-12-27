#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_normal_local_to_world}
#import bevy_pbr::mesh_view_bindings::globals
#import "game/shaders/shader_utils.wgsl"::rotateUVs
#import "game/shaders/shader_utils.wgsl"::flipUVsY
#import "game/shaders/shader_utils.wgsl"::flipUVsX

struct BlockMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: BlockMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(3) object_normal: vec3<f32>,
    @location(2) uvs: vec2<f32>
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    let model_matrix = get_model_matrix(in.instance_index);
    let normal_matrix = transpose(model_matrix);

    out.clip_position = mesh_position_local_to_clip(model_matrix,vec4<f32>(in.position * -1.0, 1.0));
    out.world_normal = mesh_normal_local_to_world(in.normal, in.instance_index);
    out.object_normal = in.normal;
    out.uvs = vec2(in.uv.x, 1.0 - in.uv.y);
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32>;
    var uvs: array<vec2<f32>, 4>;
    var textures: array<vec4<f32>, 8>;

    uvs[0] = vec2<f32>(in.uvs.x, fract(in.uvs.y - globals.time));
    uvs[1] = rotateUVs(vec2<f32>(in.uvs.x, fract(in.uvs.y + globals.time)), -180.0);
    uvs[2] = rotateUVs(vec2<f32>(fract(in.uvs.x - globals.time), in.uvs.y), 90.0);
    uvs[3] = rotateUVs(vec2<f32>(fract(in.uvs.x + globals.time), in.uvs.y), -90.0);

    for (var i = 0; i < 4; i++) {
        textures[i] = textureSample(texture, texture_sampler, uvs[i]);
        textures[i + 4] = textureSample(texture, texture_sampler, flipUVsX(uvs[i]));
    }
    
    if (abs(in.object_normal.x) == 1.0) {
        color = textures[4] + textures[5] + textures[6] + textures[7];
    }else if(abs(in.object_normal.z) == 1.0) {
        color = textures[0] + textures[1] + textures[2] + textures[3];
    }else{
        color = textures[0] + textures[1] + textures[6] + textures[7];
    }
    
    if (color.a < 1.0) {
        discard;
    }
    
    return color * material.color;
}