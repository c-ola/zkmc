use std::{collections::HashSet, f64::consts::PI};

use crate::{JavaUtilRandom, minecraft::{LegacyRandomSource, RandomSource, RandomState, biome::Biomes}};

#[derive(Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// the level_seed is the same as the rings_seed on normal worlds, rings_seed is 0 on superflat
pub struct ChunkGeneratorStructureState {
    //random_state: RandomState, // this gets used in the biome shit
    level_seed: i64,
    rings_seed: i64,
    // maybe store ring positions here
}

impl ChunkGeneratorStructureState {
    pub fn new(seed: i64) -> Self {
        Self {
            rings_seed: seed,
            level_seed: seed,
        }
    }

    // how do i test this ??
    pub fn generate_ring_positions(&self, ring_placement: ConcentricRingPlacement) -> Vec<ChunkPos> {
        let distance = ring_placement.distance;
        let count = ring_placement.count;
        let mut spread = ring_placement.spread;
        let mut random_source = JavaUtilRandom::with_seed(self.rings_seed);
        let mut d: f64 = random_source.next_double() * PI * 2.0;
        let mut l: i32 = 0;
        let mut m: i32 = 0;

        let mut list = Vec::new();
        let biomes = Biomes::new();

        for n in 0..count {
            let i_f64 = distance as f64;
            let e: f64 = 4.0 * i_f64 + i_f64 * m as f64 * 6.0 + (random_source.next_double() - 0.5) * (i_f64 * 2.5);
            let o: i32 = (d.cos() * e).round() as i32;
            let p: i32 = (d.sin() * e).round() as i32;
            let mut random_source2 = random_source.fork();
            list.push({
                // theres some logic here to detect biomes
                // it doesn't actually seem too difficult to implement but im gonna ignore for now
                // TODO: add the biome logic
                //let pair = biome_nois
                let section_to_block_coord = |i: i32, j: i32| -> i32 {
                    (i << 4) + j
                };
                let biome_pos = biomes.find_biome_horizontal(
                    section_to_block_coord(o, 8),
                    0,
                    section_to_block_coord(p, 8),
                    112,
                    1,
                    &mut random_source2,
                    false
                    );
                if let Some(pos) = biome_pos {
                    let (x, y) = (pos.0 >> 4, pos.2 >> 4);
                    ChunkPos::new(x, y)
                } else {
                    ChunkPos::new(o, p)
                }
            });

            d += (PI * 2.0) / spread as f64;
            l += 1;
            if l == spread {
                m += 1;
                l = 0;
                spread += 2 * spread / (m + 1);
                spread = spread.min(count - n);
                d += random_source.next_double() * PI * 2.0;
            }
        }

        list
    }
}

pub struct ConcentricRingPlacement {
    distance: i32,
    count: i32,
    spread: i32,
    preferred_biomes: HashSet<String>,
}

impl ConcentricRingPlacement {
    pub fn new(distance: i32, count: i32, spread: i32, preferred_biomes: HashSet<String>) -> Self {
        Self {
            distance, count, spread, preferred_biomes
        }
    }
}
