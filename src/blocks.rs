use voxel_engine::BlockType;
// lets just make a simple def of some blocks, for now we will just use solid colours
fn get_block_color(block: BlockType) -> [f32; 3] {
    match block {
        BlockType::Air => [0.0, 0.0, 0.0],
        BlockType::Grass => [0.0, 1.0, 0.0],
        BlockType::Dirt => [0.5, 0.3, 0.0],
        BlockType::Stone => [0.5, 0.5, 0.5],
    }
}
