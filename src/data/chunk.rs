pub struct ChunkData{
    size: [i32; 3],
    vertices: Vec<[f32; 3]>,
    triangles: Vec<u32>,
    blocks: HashMap<[i32; 3], BlockData>
}

impl ChunkData{
    pub fn create_chunk() -> ChunkData {
        let mut vertex_index = 0;
        let mut chunk = ChunkData {
            size: [16, 16, 16],
            vertices: Vec::new(),
            triangles: Vec::new(),
            blocks: HashMap::new()
        };
        Self::initial_chunk_blocks(&mut chunk);

        for x in 0..chunk.size[0] {
            for y in 0..chunk.size[1] {
                for z in 0..chunk.size[2] {
                    BlockData::create_block_mesh(&[x, y, z], &mut chunk, &mut vertex_index, &BlockTypes::Full);
                }
            }
        }
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

    pub fn can_place_face(pos: &[i32; 3], chunk_size: &[i32; 3], chunk: &ChunkData) -> bool {
        pos.iter().enumerate().any(|(i, &p)| p < 0 || p > chunk_size[i] - 1) ||
        chunk.blocks.get(pos).map_or(false, |block| !block.solid)
    }
    


}