// Voxel engine code - work in progress
// This file contains incomplete voxel/chunk code that will be implemented later

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
  Air,
  Dirt,
  Stone,
  Grass,
}

pub struct VoxelLocalCoords {
  pub x: usize,
  pub y: usize,
  pub z: usize,
}

pub struct VoxelGlobalCoords {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

impl VoxelGlobalCoords {
  pub fn to_local_coords(&self) -> VoxelLocalCoords {
    VoxelLocalCoords {
      x: self.x as usize % CHUNK_SIZE,
      y: self.y as usize % CHUNK_SIZE,
      z: self.z as usize % CHUNK_SIZE,
    }
  }
  pub fn from_local_coords(chunk_coords: ChunkCoords, local_coords: VoxelLocalCoords) -> Self {
    VoxelGlobalCoords {
      x: chunk_coords.x * CHUNK_SIZE as i32 + local_coords.x as i32,
      y: chunk_coords.y * CHUNK_SIZE as i32 + local_coords.y as i32,
      z: chunk_coords.z * CHUNK_SIZE as i32 + local_coords.z as i32,
    }
  }
}

pub struct ChunkCoords {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

#[derive(Clone, Copy)]
pub struct Voxel {
  pub block_type: BlockType,
}

pub struct Chunk {
  pub voxels: [Voxel; CHUNK_VOLUME],
}

impl Chunk {
  pub fn index(local_coords: VoxelLocalCoords) -> usize {
    local_coords.x + local_coords.y * CHUNK_SIZE + local_coords.z * CHUNK_SIZE * CHUNK_SIZE
  }

  pub fn get(&self, local_coords: VoxelLocalCoords) -> Voxel {
    self.voxels[Self::index(local_coords)]
  }
  
  pub fn set(&mut self, local_coords: VoxelLocalCoords, v: Voxel) {
    self.voxels[Self::index(local_coords)] = v;
  }

  // TODO: Implement chunk generation
  // pub fn generate_chunk() -> Self {
  //   ...
  // }
}

// TODO: Implement World struct with HashMap
// struct World {
//   pub chunks: HashMap<ChunkCoords, Chunk>
// }

// TODO: Implement terrain generation
// impl World {
//   pub fn flat_terrain(global_coords: VoxelGlobalCoords) -> Voxel {
//     ...
//   }
// }
