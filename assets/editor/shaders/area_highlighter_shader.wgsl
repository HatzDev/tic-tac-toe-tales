#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_normal_local_to_world}
#import bevy_pbr::mesh_view_bindings::globals
#import "game/shaders/shader_utils.wgsl"::rotate_uvs
#import "game/shaders/shader_utils.wgsl"::flip_uv_y
#import "game/shaders/shader_utils.wgsl"::flip_uv_x
#import "game/shaders/shader_utils.wgsl"::tile

struct AreaHighlighterMaterial {
    color: vec4<f32>,
    scale: vec3<f32>
};

@group(1) @binding(0)
var<uniform> material: AreaHighlighterMaterial;
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
    @location(2) uvs: vec2<f32>,
    @location(3) object_normal: vec3<f32>
   
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
    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var uvs: array<array<vec2<f32>, 4>, 3>;
    var textures: array<array<vec4<f32>, 4>, 3>;
    let background_color: vec4<f32> = vec4(0.0, 0.1, 1.0, 0.3);
    let time: f32 = globals.time * -0.5;

    uvs[0] = set_uvs(in.uvs, vec2<f32>(material.scale.z, material.scale.y), time);
    uvs[1] = set_uvs(in.uvs, vec2<f32>(material.scale.x, material.scale.y), -time);
    uvs[2] = set_uvs(in.uvs, vec2<f32>(material.scale.x, material.scale.z), time);

    for (var i = 0; i < 4; i++) {
        textures[0][i] = textureSample(texture, texture_sampler, uvs[0][i]);
        textures[1][i] = textureSample(texture, texture_sampler, uvs[1][i]);
        textures[2][i] = textureSample(texture, texture_sampler, uvs[2][i]);
    }

    if(abs(in.object_normal.x) == 1.0){
        color += set_textures(in.uvs, vec2<f32>(material.scale.z, material.scale.y), textures[0]);
    }else if(abs(in.object_normal.z) == 1.0){
        color += set_textures(in.uvs, vec2<f32>(material.scale.x, material.scale.y), textures[1]);
    }else{
        color += set_textures(in.uvs, vec2<f32>(material.scale.x, material.scale.z), textures[2]);
    }

    color = color * material.color;
    color = mix(background_color, color, color.a);

    return color;
}

fn set_uvs(uvs: vec2<f32>, axis: vec2<f32>, time: f32) -> array<vec2<f32>, 4> {
    var output_uvs: array<vec2<f32>, 4>;

    output_uvs[0] = tile(flip_uv_x(uvs), axis, vec2<f32>(0.0, -time));
    output_uvs[1] = tile(uvs, axis, vec2<f32>(0.0, time));
    output_uvs[2] = rotate_uvs(tile(flip_uv_y(uvs), axis, vec2<f32>(-time, 0.0)), -90.0);
    output_uvs[3] = rotate_uvs(tile(uvs, axis, vec2<f32>(time, 0.0)), -90.0);

    return output_uvs;
}

fn set_textures(uvs: vec2<f32>, axis: vec2<f32>, textures: array<vec4<f32>, 4>) -> vec4<f32>{
    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    if((1.0 - uvs.x) * axis.x < 1.0){
        color = max(color, textures[0]);
    }
    if(uvs.x * axis.x < 1.0){
        color = max(color, textures[1]);
    }
    if((1.0 - uvs.y) * axis.y < 1.0){
        color = max(color, textures[2]);
    }
    if(uvs.y * axis.y < 1.0){
        color = max(color, textures[3]);
    }

    return color;
}
