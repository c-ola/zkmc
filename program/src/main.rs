#![no_main]
sp1_zkvm::entrypoint!(main);
//use sha2::{Digest, Sha256};

use mc_lib::{minecraft::chunk::{ChunkGeneratorStructureState, ConcentricRingPlacement}}; 

pub fn main() {
    let seed = sp1_zkvm::io::read::<i64>();
    let target_x = sp1_zkvm::io::read::<i32>();
    let target_z = sp1_zkvm::io::read::<i32>();
    let chunk_x = target_x >> 4;
    let chunk_z = target_z >> 4;

    let placement = ConcentricRingPlacement::stronghold_default();
    let generator = ChunkGeneratorStructureState::new(seed); 

    let mut found = false;
    generator.iter_raw_stronghold_positions(placement, (chunk_x, chunk_z), (7, 7), |pos| {
        let diff_x = pos.x.abs_diff(chunk_x);
        let diff_z = pos.y.abs_diff(chunk_z);
        if diff_x <= 7 && diff_z <= 7 {
            println!("Found chunk {}, {}", pos.x, pos.y);
            found = true;
            return false;
        }
        true
    });
    // expensive ahh
    /*generator.search_stronghold_positions_with(placement, (chunk_x, chunk_z), (7, 7), |pos| {
        if chunk_x == pos.x && chunk_z == pos.y {
            println!("Found chunk {}, {}", pos.x, pos.y);
            found = true;
            return false;
        }
        true
    });*/

    assert!(found, "The provided coordinates are not a valid stronghold for this seed.");

    // this actually leaks info so its bad
    /*
    let mut hasher = Sha256::new();
    hasher.update(target_x.to_le_bytes());
    hasher.update(target_z.to_le_bytes());
    let pos_hash: [u8; 32] = hasher.finalize().into();
    */
    sp1_zkvm::io::commit(&seed);
    //sp1_zkvm::io::commit(&pos_hash);
}
