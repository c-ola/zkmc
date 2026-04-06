use std::{f64::consts::PI};

#[cfg(not(target_os = "zkvm"))]
use rayon::prelude::*;

use crate::{minecraft::{biome::Biomes}, rng::JavaUtilRandom};
use crate::{util::Section};
use crate::rng::RandomSource;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for ChunkPos {
    fn from(value: (i32, i32)) -> Self {
        ChunkPos {
            x: value.0,
            y: value.1
        }
    }
}

/*pub struct StrongholdGenerator {
    world_seed: i64,
    distance: i32,
    count: i32,
    spread: i32,
    random_source: JavaUtilRandom,
    angle: f64,
    l: i32,
    m: i32,
    n: i32,
}

impl StrongholdGenerator {
    pub fn new(seed: i64) -> Self {
        let mut random_source = JavaUtilRandom::with_seed(seed);
        let placement = ConcentricRingPlacement::stronghold_default();
        Self {
            world_seed: seed,
            distance: placement.distance,
            count: placement.count,
            spread: placement.spread,
            angle: random_source.next_f64() * PI * 2.0,
            random_source,
            l: 0,
            m: 0,
            n: 0
        }
    }

    pub fn snapped(chunk_pos: ChunkPos) -> ChunkPos {
        let biome_pos: Option<(i32, i32, i32)> = biomes.find_biome_horizontal(
            Section::to_block_coord_ex(o, 8),
            0,
            Section::to_block_coord_ex(p, 8),
            112,
            &mut random_source2,
        );
    }
}

impl Iterator for StrongholdGenerator {
    type Item = ChunkPos;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.count {
            return None
        }

        let d = self.distance as f64;
        let e: f64 = 4.0 * d + d * self.m as f64 * 6.0 + (self.random_source.next_f64() - 0.5) * (d * 2.5);
        let o: i32 = (self.angle.cos() * e).round() as i32;
        let p: i32 = (self.angle.sin() * e).round() as i32;

        let _random_source2 = self.random_source.fork();
        let chunk_pos = ChunkPos::new(o, p);

        self.angle += (PI * 2.0) / self.spread as f64;
        self.l += 1;
        if self.l == self.spread {
            self.m += 1;
            self.l = 0;
            self.spread += 2 * self.spread / (self.m + 1);
            self.spread = self.spread.min(self.count - self.n);
            self.angle += self.random_source.next_f64() * PI * 2.0;
        }

        Some(chunk_pos)
    }
}*/


// the level_seed is the same as the rings_seed on normal worlds, rings_seed is 0 on superflat
#[allow(unused)]
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

    pub fn iter_raw_stronghold_positions<F>(
        &self, 
        ring_placement: ConcentricRingPlacement,
        target_chunk: (i32, i32),
        range: (u32, u32),
        mut callback: F,
    ) where
        F: FnMut(ChunkPos) -> bool,
    {
        let distance = ring_placement.distance;
        let count = ring_placement.count;
        let mut spread = ring_placement.spread;
        let mut random_source = JavaUtilRandom::with_seed(self.rings_seed);
        let mut angle: f64 = random_source.next_f64() * PI * 2.0;
        let mut l: i32 = 0;
        let mut m: i32 = 0;

        for n in 0..count {
            let i_f64 = distance as f64;
            let e: f64 = 4.0 * i_f64 + i_f64 * m as f64 * 6.0 + (random_source.next_f64() - 0.5) * (i_f64 * 2.5);
            let o: i32 = (angle.cos() * e).round() as i32;
            let p: i32 = (angle.sin() * e).round() as i32;
            let diff_x = o.abs_diff(target_chunk.0);
            let diff_z = p.abs_diff(target_chunk.1);

            let _random_source2 = random_source.fork();
            // im basically doing the diffing thing twice but whatever
            if diff_x <= range.0 && diff_z <= range.1 {
                let chunk_pos = ChunkPos::new(o, p);

                if !callback(chunk_pos) {
                    break;
                }
            }

            angle += (PI * 2.0) / spread as f64;
            l += 1;
            if l == spread {
                m += 1;
                l = 0;
                spread += 2 * spread / (m + 1);
                spread = spread.min(count - n);
                angle += random_source.next_f64() * PI * 2.0;
            }
        }

    }

    pub fn search_stronghold_positions_with<F>(
        &self, 
        ring_placement: ConcentricRingPlacement,
        target_chunk: (i32, i32),
        range: (u32, u32),
        mut callback: F,
    ) where
        F: FnMut(ChunkPos) -> bool,
    {
        let distance = ring_placement.distance;
        let count = ring_placement.count;
        let mut spread = ring_placement.spread;
        let mut random_source = JavaUtilRandom::with_seed(self.rings_seed);
        let mut angle: f64 = random_source.next_f64() * PI * 2.0;
        let mut l: i32 = 0;
        let mut m: i32 = 0;

        //println!("target={target_chunk:?}, range={range:?}");
        let biomes = Biomes::new(self.level_seed);

        for n in 0..count {
            let i_f64 = distance as f64;
            let e: f64 = 4.0 * i_f64 + i_f64 * m as f64 * 6.0 + (random_source.next_f64() - 0.5) * (i_f64 * 2.5);
            let o: i32 = (angle.cos() * e).round() as i32;
            let p: i32 = (angle.sin() * e).round() as i32;
            let diff_x = o.abs_diff(target_chunk.0);
            let diff_z = p.abs_diff(target_chunk.1);
            //println!("o, p={o}, {p}, diff={diff_x}, {diff_z}");

            let mut random_source2 = random_source.fork();
            if diff_x <= range.0 && diff_z <= range.1 {
                let chunk_pos = {
                    let biome_pos: Option<(i32, i32, i32)> = biomes.find_biome_horizontal(
                        Section::to_block_coord_ex(o, 8),
                        0,
                        Section::to_block_coord_ex(p, 8),
                        112,
                        &mut random_source2,
                    );

                    if let Some(pos) = biome_pos {
                        let (x, y) = (pos.0 >> 4, pos.2 >> 4);
                        ChunkPos::new(x, y)
                    } else {
                        ChunkPos::new(o, p)
                    }
                };

                if !callback(chunk_pos) {
                    break;
                }
            }

            angle += (PI * 2.0) / spread as f64;
            l += 1;
            if l == spread {
                m += 1;
                l = 0;
                spread += 2 * spread / (m + 1);
                spread = spread.min(count - n);
                angle += random_source.next_f64() * PI * 2.0;
            }
        }
    }

    pub fn generate_ring_positions_with<F>(
        &self, 
        ring_placement: ConcentricRingPlacement,
        mut callback: F,
    ) where
        F: FnMut(ChunkPos) -> bool,
    {
        let distance = ring_placement.distance;
        let count = ring_placement.count;
        let mut spread = ring_placement.spread;
        let mut random_source = JavaUtilRandom::with_seed(self.rings_seed);
        let mut angle: f64 = random_source.next_f64() * PI * 2.0;
        let mut l: i32 = 0;
        let mut m: i32 = 0;

        let biomes = Biomes::new(self.level_seed);

        for n in 0..count {
            let i_f64 = distance as f64;
            let e: f64 = 4.0 * i_f64 + i_f64 * m as f64 * 6.0 + (random_source.next_f64() - 0.5) * (i_f64 * 2.5);
            let o: i32 = (angle.cos() * e).round() as i32;
            let p: i32 = (angle.sin() * e).round() as i32;
            let mut random_source2 = random_source.fork();
            let chunk_pos = {
                let biome_pos: Option<(i32, i32, i32)> = biomes.find_biome_horizontal(
                    Section::to_block_coord_ex(o, 8),
                    0,
                    Section::to_block_coord_ex(p, 8),
                    112,
                    &mut random_source2,
                );

                if let Some(pos) = biome_pos {
                    let (x, y) = (pos.0 >> 4, pos.2 >> 4);
                    ChunkPos::new(x, y)
                } else {
                    ChunkPos::new(o, p)
                }
            };

            if !callback(chunk_pos) {
                break;
            }

            angle += (PI * 2.0) / spread as f64;
            l += 1;
            if l == spread {
                m += 1;
                l = 0;
                spread += 2 * spread / (m + 1);
                spread = spread.min(count - n);
                angle += random_source.next_f64() * PI * 2.0;
            }
        }

    }

    pub fn generate_ring_positions(&self, ring_placement: ConcentricRingPlacement) -> Vec<ChunkPos> {
        let mut list = Vec::with_capacity(ring_placement.count as usize);

        self.generate_ring_positions_with(ring_placement, |pos| {
            list.push(pos);
            true
        });

        list
    }

    #[cfg(not(target_os = "zkvm"))]
    pub fn generate_ring_positions_parallel(&self, ring_placement: ConcentricRingPlacement) -> Vec<ChunkPos> {
        let distance = ring_placement.distance;
        let count = ring_placement.count;
        let mut spread = ring_placement.spread;
        let mut random_source = JavaUtilRandom::with_seed(self.rings_seed);
        let mut angle: f64 = random_source.next_f64() * PI * 2.0;
        let mut l: i32 = 0;
        let mut m: i32 = 0;

        let mut placements = Vec::new();

        for n in 0..count {
            let i_f64 = distance as f64;
            let e: f64 = 4.0 * i_f64 + i_f64 * m as f64 * 6.0 + (random_source.next_f64() - 0.5) * (i_f64 * 2.5);
            let o: i32 = (angle.cos() * e).round() as i32;
            let p: i32 = (angle.sin() * e).round() as i32;
            let random_source2 = random_source.fork();
            placements.push((o, p, random_source2));
            angle += (PI * 2.0) / spread as f64;
            l += 1;
            if l == spread {

                m += 1;
                l = 0;
                spread += 2 * spread / (m + 1);
                spread = spread.min(count - n);
                angle += random_source.next_f64() * PI * 2.0;
            }
        }

        let biomes = Biomes::new(self.level_seed);

        let list: Vec<ChunkPos> = placements.into_par_iter().map(|(o, p, mut random_source2)| {
            let biome_pos: Option<(i32, i32, i32)> = biomes.find_biome_horizontal(
                Section::to_block_coord_ex(o, 8),
                0,
                Section::to_block_coord_ex(p, 8),
                112,
                &mut random_source2,
            );

            if let Some(pos) = biome_pos {
                let (x, y) = (pos.0 >> 4, pos.2 >> 4);
                ChunkPos::new(x, y)
            } else {
                ChunkPos::new(o, p)
            }
        })
        .collect();

        list
    }
}

pub struct ConcentricRingPlacement {
    distance: i32,
    count: i32,
    spread: i32,
}

impl ConcentricRingPlacement {
    pub fn stronghold_default() -> Self {
        Self {
            distance: 32,
            count: 128,
            spread: 3,
        }
    }
    pub fn new(distance: i32, count: i32, spread: i32) -> Self {
        Self {
            distance, count, spread
        }
    }
}
