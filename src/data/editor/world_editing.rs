include!("materials.rs");

pub struct Highlighter{}
impl Highlighter{
    pub fn create<MaterialHolder: Material>(
        mut commands: Commands,
        mut materials: ResMut<Assets<MaterialHolder>>,
        mesh: Handle<Mesh>,
        material: MaterialHolder
    ){
        commands.spawn(MaterialMeshBundle {
            mesh: mesh,
            material: materials.add(material),
            transform: Transform::from_xyz(1.0, 2.5, 2.6)
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, -45.0, 0.0)),
            ..Default::default()
        });
    }
}

pub struct AreaHighlighter{}
impl AreaHighlighter{
    pub fn create(
        commands: Commands,
        materials: ResMut<Assets<AreaHighlighterMaterial>>,
        asset_server: Res<AssetServer>,
    ){
        let mesh_handle = asset_server.load("editor/models/basic_block.glb#Mesh0/Primitive0");
        let texture_handle = asset_server.load("editor/textures/area_highlighter.png");
        
        Highlighter::create(commands, materials, mesh_handle, AreaHighlighterMaterial {
            color: Color::WHITE,
            texture: Some(texture_handle.clone()),
            scale: Vec3::new(0.0, 0.0, 0.0)
        });
    }
}

pub struct BlockHighlighter{}
impl BlockHighlighter{
    pub fn create(
        commands: Commands,
        materials: ResMut<Assets<BlockHighlighterMaterial>>,
        asset_server: Res<AssetServer>,
    ){
        let mesh_handle = asset_server.load("editor/models/basic_block.glb#Mesh0/Primitive0");
        let texture_handle = asset_server.load("editor/textures/block_highlighter.png");
        
        Highlighter::create(commands, materials, mesh_handle, BlockHighlighterMaterial {
            color: Color::WHITE,
            texture: Some(texture_handle.clone()),
        });
    }
}
