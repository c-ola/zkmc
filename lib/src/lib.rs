pub mod rng;
pub mod util;
pub mod minecraft;
pub mod noise;
pub mod tree;

use crate::minecraft::chunk::{ChunkGeneratorStructureState, ChunkPos, ConcentricRingPlacement};

pub fn generate_strongholds_test() {
    // preferred_biomes is something else ill figure it out later
    let placement = ConcentricRingPlacement::new(32, 128, 3);
    let seed = -6152149964729591252;
    //let seed = 1;
    let chunk_generator_structure_state = ChunkGeneratorStructureState::new(seed); 
    let positions = chunk_generator_structure_state.generate_ring_positions(placement);
    for (i, position) in positions.iter().enumerate() {
        println!("{i}: {}, {}:{}, {}", ((position.x * 16)) + 4, (((position.y * 16)) + 4), position.x, position.y);
    }
}

pub fn generate_strongholds(seed: u64) -> Vec<ChunkPos> {
    let placement = ConcentricRingPlacement::new(32, 1, 3);
    let chunk_generator_structure_state = ChunkGeneratorStructureState::new(seed as i64); 
    let positions = chunk_generator_structure_state.generate_ring_positions(placement);
    positions
}
