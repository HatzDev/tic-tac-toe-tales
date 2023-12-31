use bevy::render::render_resource::*;
use bevy::pbr::MaterialPipeline;
use bevy::pbr::MaterialPipelineKey;
use bevy::render::mesh::InnerMeshVertexBufferLayout;
use bevy::utils::Hashed;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect)]
#[reflect(Debug, Asset)]
pub struct AreaHighlighterMaterial{
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    texture: Option<Handle<Image>>,
    #[uniform(0)]
    scale: Vec3
}

impl Material for AreaHighlighterMaterial{
    fn fragment_shader() -> ShaderRef{
        "editor/shaders/area_highlighter_shader.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef{
        "editor/shaders/area_highlighter_shader.wgsl".into()
    }
    fn specialize(_pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &Hashed<InnerMeshVertexBufferLayout>,
        _key: MaterialPipelineKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

pub fn update_area_highlighter_material(
    mut materials: ResMut<Assets<AreaHighlighterMaterial>>,
    query: Query<(&Handle<AreaHighlighterMaterial>, &Transform)>
) {
    for (material_handle, transform) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.scale = transform.scale;
        }
    }
}

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect)]
pub struct BlockHighlighterMaterial{
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    texture: Option<Handle<Image>>,
}

impl Material for BlockHighlighterMaterial{
    fn fragment_shader() -> ShaderRef{
        "editor/shaders/block_highlighter_shader.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef{
        "editor/shaders/block_highlighter_shader.wgsl".into()
    }
    fn specialize(_pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &Hashed<InnerMeshVertexBufferLayout>,
        _key: MaterialPipelineKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}