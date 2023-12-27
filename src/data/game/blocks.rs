use bevy::utils::HashMap;

static DIRECTIONS: [[i32; 3]; 6] = [
    [0, 1, 0],  // Up Direction
    [0, -1, 0], // Down Direction
    [0, 0, 1],  // North Direction
    [0, 0, -1], // South Direction
    [1, 0, 0],  // West Direction
    [-1, 0, 0], // East Direction
];

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum BlockTypes{
    Air,
    Full
}

#[derive(Debug, Clone)]
pub struct BlockData{
    solid: bool,
    vertices: Vec<[f32; 3]>,
    triangles: Vec<[u32 ; 6]>,
    uvs: Vec<[f32; 2]>,
    block_type: BlockTypes,
}

impl BlockData{
    pub fn new(block_type: &BlockTypes) -> BlockData{
        let full_block = BlockData{
            solid: true,
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
            block_type: BlockTypes::Full
        };
        let air_block = BlockData{
            solid: false,
            vertices: vec![],
            triangles: vec![],
            uvs: vec![],
            block_type: BlockTypes::Air
        };
        match block_type {
            BlockTypes::Full => full_block,
            BlockTypes::Air => air_block
        }
    }

    pub fn create_block_mesh(block_type: &BlockTypes, pos: &[i32; 3], chunk: &mut ChunkData, vertex_index: &mut u32){
        let block_data = BlockData::new(block_type);
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
                    chunk.uvs.push(block_data.uvs[index]);
                    *vertex_index += 1;
                }
            }
        }
    }

    pub fn place_block(chunk: &mut ChunkData, block_type: &BlockTypes, pos: [i32; 3]){
        let block_data = BlockData::new(block_type);
        chunk.blocks.insert(pos, block_data.clone());
    }
}