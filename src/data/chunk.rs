pub struct ChunkData{
    size: [i32; 3],
    vertices: Vec<[f32; 3]>,
    triangles: Vec<u32>,
    blocks: HashMap<[i32; 3], BlockData>
}

impl ChunkData{
    pub fn create_chunk_mesh() -> ChunkData {
        let mut chunk = ChunkData {
            size: [16, 16, 16],
            vertices: Vec::new(),
            triangles: Vec::new(),
            blocks: HashMap::new()
        };
        Self::initial_chunk_blocks(&mut chunk);
        Self::refresh_chunk_mesh(&mut chunk);

        chunk
    }

    pub fn initial_chunk_blocks(chunk: &mut ChunkData){
        for x in 0..chunk.size[0] {
            for y in 0..chunk.size[1] {
                for z in 0..chunk.size[2] {
                    BlockData::place_block(chunk, &BlockTypes::Full, [x, y, z]);
                }
            }
        }
    }

    pub fn create_chunk(mut commands: Commands,  mut meshes: ResMut<Assets<Mesh>>,  mut materials: ResMut<Assets<BlockMaterial>>){
        let chunk_data = ChunkData::create_chunk_mesh();
        let chunk_mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk_data.vertices).with_computed_flat_normals()
        .with_indices(Some(Indices::U32(chunk_data.triangles)));

        commands.spawn(MaterialMeshBundle{
            mesh: meshes.add(chunk_mesh),
            material: materials.add(BlockMaterial{
                sun_color: Color::WHITE,
                sun_direction: Vec3::new(0.0, 0.0, 0.0),
                ambient_color: Color::WHITE,
            }),
            transform: Transform::from_xyz(0.4, -22.7, -38.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, -45.0, 0.0)),
            ..Default::default()
        });
    }

    pub fn refresh_chunk_mesh(mut chunk: &mut ChunkData){
        let mut chunk_data = Vec::new();
        let mut vertex_index = 0;

        for (pos, block) in &chunk.blocks {
            chunk_data.push((pos.clone(), block.block_type.clone()));
        }

        for (pos, block_type) in chunk_data {
            BlockData::create_block_mesh(&block_type, &pos, &mut chunk, &mut vertex_index);
        }
    }

    pub fn can_place_face(pos: &[i32; 3], chunk_size: &[i32; 3], chunk: &ChunkData) -> bool {
        pos.iter().enumerate().any(|(i, &p)| p < 0 || p > chunk_size[i] - 1) ||
        chunk.blocks.get(pos).map_or(false, |block| !block.solid)
    }

}