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

fn update_block_material(mut materials: ResMut<Assets<BlockMaterial>>, query: Query<(&mut Transform, &DirectionalLight)>, _ambient_light: Option<Res<AmbientLight>>){
    for (transform, light) in query.iter() {
        for (_handle, material) in materials.iter_mut() {
            material.sun_direction = -transform.forward();
            material.sun_color = light.color;
            if let Some(light) = &_ambient_light {
                material.ambient_color = light.color;
            }
        }
    }
}