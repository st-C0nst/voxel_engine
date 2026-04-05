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

// we can define the block model as well here..
// ig for this early version we should really instance our block model
// a bunch of times, instancing should make it easy to change the block color
// ideally we only send voxel model data to the gpu then handle color logic on gpu side
