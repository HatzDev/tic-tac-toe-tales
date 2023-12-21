use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct BlockMaterial{
    #[uniform(0)]
    sun_color: Color,
    #[uniform(0)]
    sun_direction: Vec3,
    #[uniform(0)]
    ambient_color: Color,
}

impl Material for BlockMaterial {
    fn fragment_shader() -> ShaderRef{
        "shaders/blocks_shader.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef{
        "shaders/blocks_shader.wgsl".into()
    }
}