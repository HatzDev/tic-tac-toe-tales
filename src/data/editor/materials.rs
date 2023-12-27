use bevy::render::render_resource::*;
use bevy::pbr::MaterialPipeline;
use bevy::pbr::MaterialPipelineKey;
use bevy::render::mesh::InnerMeshVertexBufferLayout;
use bevy::utils::Hashed;

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
}