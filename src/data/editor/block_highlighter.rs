include!("materials.rs");

pub struct BlockHighlighter{
    vertices: Vec<[f32; 3]>,
    triangles: Vec<[u32 ; 6]>,
    uvs: Vec<[f32; 2]>
}

impl BlockHighlighter{
    pub fn new() -> BlockHighlighter{
        let highlighter_block = BlockHighlighter{
            vertices: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [1.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [1.0, 1.0, -1.0],
                [0.0, 1.0, -1.0],

            ],
            triangles: vec![
                [2, 3, 7, 7, 3, 6], // Up Face
                [5, 4, 0, 0, 4, 1], // Down Face
                [0, 1, 2, 2, 1, 3], // North Face
                [4, 5, 6, 6, 5, 7], // South Face
                [1, 4, 3, 3, 4, 6], // West Face
                [5, 0, 7, 7, 0, 2], // East Face
            ],
            uvs: vec![
                [0.0, 0.0],
                [1.0, 0.0],
                [0.0, 1.0],
                [0.0, 1.0],
                [1.0, 0.0],
                [1.0, 1.0],
            ],
        };
        highlighter_block
    }
    pub fn create_block_highlighter_mesh() -> Mesh{
        let mut vertex_index: i32 = 0;
        let block_highlighter = BlockHighlighter::new();
        let block_highlighter_mesh: Mesh;
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut triangles: Vec<u32> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();

        for triangle_group in block_highlighter.triangles{
            for (index, triangle) in triangle_group.iter().enumerate(){
                let vertex = block_highlighter.vertices[*triangle as usize];
                vertices.push(vertex);
                triangles.push(vertex_index as u32);
                uvs.push(block_highlighter.uvs[index]);
                vertex_index += 1;
            }
        }
        block_highlighter_mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone())
        .with_computed_flat_normals().with_indices(Some(Indices::U32(triangles)))
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        block_highlighter_mesh
    }
    pub fn create_block_highlighter(
        mut commands: Commands,  mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<BlockHighlighterMaterial>>,
        asset_server: Res<AssetServer>
    ){
        let block_highlighter_texture_handle: Handle<Image> = asset_server.load("editor/textures/block_highlighter.png");

        commands.spawn(MaterialMeshBundle{
            mesh: meshes.add(Self::create_block_highlighter_mesh()),
            material: materials.add(BlockHighlighterMaterial{
                color: Color::WHITE,
                texture: Some(block_highlighter_texture_handle.clone()),
            }),
            transform: Transform::from_xyz(1.0, 2.5, 2.6).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, -45.0, 0.0)),
            ..Default::default()
        });
    }
}