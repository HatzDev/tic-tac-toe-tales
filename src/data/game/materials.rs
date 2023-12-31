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
        "game/shaders/blocks_shader.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef{
        "game/shaders/blocks_shader.wgsl".into()
    }
}

fn update_block_material(
    mut materials: ResMut<Assets<BlockMaterial>>,
    query: Query<(&Handle<BlockMaterial>, &mut Transform, &DirectionalLight)>,
    ambient_light: Option<Res<AmbientLight>>
){
    for (material_handle, transform, light) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle){
            material.sun_direction = -transform.forward();
            material.sun_color = light.color;
            if let Some(light) = &ambient_light {
                material.ambient_color = light.color;
            }
        }
    }
}