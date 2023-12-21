#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref BLOCK_REGISTRY: HashMap<BlockTypes, BlockData> = {
        let mut registry = HashMap::new();
        registry.insert(BlockTypes::Full, BlockData {
            solid: true,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 1.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            triangles: vec![
                [2, 1, 5, 5, 1, 6], // Up Face
                [7, 0, 3, 3, 4, 7], // Down Face
                [0, 1, 3, 3, 1, 2], // North Face
                [4, 5, 7, 7, 5, 6], // South Face
                [7, 6, 0, 0, 6, 1], // West Face
                [3, 2, 4, 4, 2, 5], // East Face
            ],
        });
        registry
    };
}

static DIRECTIONS: [[i32; 3]; 6] = [
    [0, 1, 0],  // Up Direction
    [0, -1, 0], // Down Direction
    [0, 0, -1], // North Direction
    [0, 0, 1],  // South Direction
    [-1, 0, 0], // West Direction
    [1, 0, 0],  // East Direction
];