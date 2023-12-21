use bevy::utils::HashMap;
include!("registry.rs");

#[derive(Eq, PartialEq, Hash)]
pub enum BlockTypes{
    Air,
    Full
}

#[derive(Debug, Clone)]
pub struct BlockData{
    solid: bool,
    vertices: Vec<[f32; 3]>,
    triangles: Vec<[u32 ; 6]>,
}

impl BlockData{
    pub fn create_block_mesh(pos: &[i32; 3], chunk: &mut ChunkData, vertex_index: &mut u32, block_type: &BlockTypes){
        if let Some(block_data) = &BLOCK_REGISTRY.get(block_type) {
            for (index, triangle_group) in block_data.triangles.iter().enumerate() {
                let next_pos = [
                    DIRECTIONS[index][0] + pos[0],
                    DIRECTIONS[index][1] + pos[1],
                    DIRECTIONS[index][2] + pos[2],
                ];
                if ChunkData::can_place_face(&next_pos, &chunk.size, &chunk){
                    for triangle in triangle_group{
                        let vertex = block_data.vertices[*triangle as usize];
                        let modified_vertex = [
                            vertex[0] + pos[0] as f32,
                            vertex[1] + pos[1] as f32,
                            vertex[2] + pos[2] as f32
                        ];
                        chunk.vertices.push(modified_vertex);
                        chunk.triangles.push(*vertex_index);
                        *vertex_index += 1;
                    }
                }
            }
        }
    }

    pub fn place_block(chunk: &mut ChunkData, block_type: &BlockTypes, pos: [i32; 3]){
        if let Some(block_data) = BLOCK_REGISTRY.get(block_type) {
            chunk.blocks.insert(pos, block_data.clone()); 
        }
    }
}