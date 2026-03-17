use std::collections::{HashMap, HashSet};

use crate::{JavaUtilRandom, minecraft::QuartPos, noise::BiomeNoise};

pub struct Biomes {
    pub is_overworld: HashSet<String>,
    pub stronghold_biased: HashSet<String>,
    pub biome_noise: BiomeNoise,
}

pub mod biomes {
pub type BiomeID = i16;

    pub const NONE: BiomeID = -1;

    // --- Base Biomes (0-39) ---
    pub const OCEAN: BiomeID = 0;
    pub const PLAINS: BiomeID = 1;
    pub const DESERT: BiomeID = 2;
    pub const MOUNTAINS: BiomeID = 3;
    pub const EXTREME_HILLS: BiomeID = MOUNTAINS;
    pub const FOREST: BiomeID = 4;
    pub const TAIGA: BiomeID = 5;
    pub const SWAMP: BiomeID = 6;
    pub const SWAMPLAND: BiomeID = SWAMP;
    pub const RIVER: BiomeID = 7;
    pub const NETHER_WASTES: BiomeID = 8;
    pub const HELL: BiomeID = NETHER_WASTES;
    pub const THE_END: BiomeID = 9;
    pub const SKY: BiomeID = THE_END;

    pub const FROZEN_OCEAN: BiomeID = 10;
    pub const FROZEN_RIVER: BiomeID = 11;
    pub const SNOWY_TUNDRA: BiomeID = 12;
    pub const ICE_PLAINS: BiomeID = SNOWY_TUNDRA;
    pub const SNOWY_MOUNTAINS: BiomeID = 13;
    pub const ICE_MOUNTAINS: BiomeID = SNOWY_MOUNTAINS;
    pub const MUSHROOM_FIELDS: BiomeID = 14;
    pub const MUSHROOM_ISLAND: BiomeID = MUSHROOM_FIELDS;
    pub const MUSHROOM_FIELD_SHORE: BiomeID = 15;
    pub const MUSHROOM_ISLAND_SHORE: BiomeID = MUSHROOM_FIELD_SHORE;
    pub const BEACH: BiomeID = 16;
    pub const DESERT_HILLS: BiomeID = 17;
    pub const WOODED_HILLS: BiomeID = 18;
    pub const FOREST_HILLS: BiomeID = WOODED_HILLS;
    pub const TAIGA_HILLS: BiomeID = 19;

    pub const MOUNTAIN_EDGE: BiomeID = 20;
    pub const EXTREME_HILLS_EDGE: BiomeID = MOUNTAIN_EDGE;
    pub const JUNGLE: BiomeID = 21;
    pub const JUNGLE_HILLS: BiomeID = 22;
    pub const JUNGLE_EDGE: BiomeID = 23;
    pub const DEEP_OCEAN: BiomeID = 24;
    pub const STONE_SHORE: BiomeID = 25;
    pub const STONE_BEACH: BiomeID = STONE_SHORE;
    pub const SNOWY_BEACH: BiomeID = 26;
    pub const COLD_BEACH: BiomeID = SNOWY_BEACH;
    pub const BIRCH_FOREST: BiomeID = 27;
    pub const BIRCH_FOREST_HILLS: BiomeID = 28;
    pub const DARK_FOREST: BiomeID = 29;
    pub const ROOFED_FOREST: BiomeID = DARK_FOREST;

    pub const SNOWY_TAIGA: BiomeID = 30;
    pub const COLD_TAIGA: BiomeID = SNOWY_TAIGA;
    pub const SNOWY_TAIGA_HILLS: BiomeID = 31;
    pub const COLD_TAIGA_HILLS: BiomeID = SNOWY_TAIGA_HILLS;
    pub const GIANT_TREE_TAIGA: BiomeID = 32;
    pub const MEGA_TAIGA: BiomeID = GIANT_TREE_TAIGA;
    pub const GIANT_TREE_TAIGA_HILLS: BiomeID = 33;
    pub const MEGA_TAIGA_HILLS: BiomeID = GIANT_TREE_TAIGA_HILLS;
    pub const WOODED_MOUNTAINS: BiomeID = 34;
    pub const EXTREME_HILLS_PLUS: BiomeID = WOODED_MOUNTAINS;
    pub const SAVANNA: BiomeID = 35;
    pub const SAVANNA_PLATEAU: BiomeID = 36;
    pub const BADLANDS: BiomeID = 37;
    pub const MESA: BiomeID = BADLANDS;
    pub const WOODED_BADLANDS_PLATEAU: BiomeID = 38;
    pub const MESA_PLATEAU_F: BiomeID = WOODED_BADLANDS_PLATEAU;
    pub const BADLANDS_PLATEAU: BiomeID = 39;
    pub const MESA_PLATEAU: BiomeID = BADLANDS_PLATEAU;

    // --- 1.13 Update Aquatic ---
    pub const SMALL_END_ISLANDS: BiomeID = 40;
    pub const END_MIDLANDS: BiomeID = 41;
    pub const END_HIGHLANDS: BiomeID = 42;
    pub const END_BARRENS: BiomeID = 43;
    pub const WARM_OCEAN: BiomeID = 44;
    pub const LUKEWARM_OCEAN: BiomeID = 45;
    pub const COLD_OCEAN: BiomeID = 46;
    pub const DEEP_WARM_OCEAN: BiomeID = 47;
    pub const WARM_DEEP_OCEAN: BiomeID = DEEP_WARM_OCEAN;
    pub const DEEP_LUKEWARM_OCEAN: BiomeID = 48;
    pub const LUKEWARM_DEEP_OCEAN: BiomeID = DEEP_LUKEWARM_OCEAN;
    pub const DEEP_COLD_OCEAN: BiomeID = 49;
    pub const COLD_DEEP_OCEAN: BiomeID = DEEP_COLD_OCEAN;
    pub const DEEP_FROZEN_OCEAN: BiomeID = 50;
    pub const FROZEN_DEEP_OCEAN: BiomeID = DEEP_FROZEN_OCEAN;

    // --- Legacy Alpha/Beta Biomes ---
    pub const SEASONAL_FOREST: BiomeID = 51;
    pub const RAINFOREST: BiomeID = 52;
    pub const SHRUBLAND: BiomeID = 53;

    pub const THE_VOID: BiomeID = 127;

    // --- Mutated Variants (+128) ---
    const MUTATION: BiomeID = 128;
    pub const SUNFLOWER_PLAINS: BiomeID = PLAINS + MUTATION;
    pub const DESERT_LAKES: BiomeID = DESERT + MUTATION;
    pub const GRAVELLY_MOUNTAINS: BiomeID = MOUNTAINS + MUTATION;
    pub const FLOWER_FOREST: BiomeID = FOREST + MUTATION;
    pub const TAIGA_MOUNTAINS: BiomeID = TAIGA + MUTATION;
    pub const SWAMP_HILLS: BiomeID = SWAMP + MUTATION;
    pub const ICE_SPIKES: BiomeID = SNOWY_TUNDRA + MUTATION;
    pub const MODIFIED_JUNGLE: BiomeID = JUNGLE + MUTATION;
    pub const MODIFIED_JUNGLE_EDGE: BiomeID = JUNGLE_EDGE + MUTATION;
    pub const TALL_BIRCH_FOREST: BiomeID = BIRCH_FOREST + MUTATION;
    pub const TALL_BIRCH_HILLS: BiomeID = BIRCH_FOREST_HILLS + MUTATION;
    pub const DARK_FOREST_HILLS: BiomeID = DARK_FOREST + MUTATION;
    pub const SNOWY_TAIGA_MOUNTAINS: BiomeID = SNOWY_TAIGA + MUTATION;
    pub const GIANT_SPRUCE_TAIGA: BiomeID = GIANT_TREE_TAIGA + MUTATION;
    pub const GIANT_SPRUCE_TAIGA_HILLS: BiomeID = GIANT_TREE_TAIGA_HILLS + MUTATION;
    pub const MODIFIED_GRAVELLY_MOUNTAINS: BiomeID = WOODED_MOUNTAINS + MUTATION;
    pub const SHATTERED_SAVANNA: BiomeID = SAVANNA + MUTATION;
    pub const SHATTERED_SAVANNA_PLATEAU: BiomeID = SAVANNA_PLATEAU + MUTATION;
    pub const ERODED_BADLANDS: BiomeID = BADLANDS + MUTATION;
    pub const MODIFIED_WOODED_BADLANDS_PLATEAU: BiomeID = WOODED_BADLANDS_PLATEAU + MUTATION;
    pub const MODIFIED_BADLANDS_PLATEAU: BiomeID = BADLANDS_PLATEAU + MUTATION;

    // --- 1.14 Village & Pillage ---
    pub const BAMBOO_JUNGLE: BiomeID = 168;
    pub const BAMBOO_JUNGLE_HILLS: BiomeID = 169;

    // --- 1.16 Nether Update ---
    pub const SOUL_SAND_VALLEY: BiomeID = 170;
    pub const CRIMSON_FOREST: BiomeID = 171;
    pub const WARPED_FOREST: BiomeID = 172;
    pub const BASALT_DELTAS: BiomeID = 173;

    // --- 1.17 Caves & Cliffs Pt I ---
    pub const DRIPSTONE_CAVES: BiomeID = 174;
    pub const LUSH_CAVES: BiomeID = 175;

    // --- 1.18 Caves & Cliffs Pt II ---
    pub const MEADOW: BiomeID = 177;
    pub const GROVE: BiomeID = 178;
    pub const SNOWY_SLOPES: BiomeID = 179;
    pub const JAGGED_PEAKS: BiomeID = 180;
    pub const FROZEN_PEAKS: BiomeID = 181;
    pub const STONY_PEAKS: BiomeID = 182;

    // 1.18 Rebranding Aliases
    pub const OLD_GROWTH_BIRCH_FOREST: BiomeID = TALL_BIRCH_FOREST;
    pub const OLD_GROWTH_PINE_TAIGA: BiomeID = GIANT_TREE_TAIGA;
    pub const OLD_GROWTH_SPRUCE_TAIGA: BiomeID = GIANT_SPRUCE_TAIGA;
    pub const SNOWY_PLAINS: BiomeID = SNOWY_TUNDRA;
    pub const SPARSE_JUNGLE: BiomeID = JUNGLE_EDGE;
    pub const STONY_SHORE: BiomeID = STONE_SHORE;
    pub const WINDSWEPT_HILLS: BiomeID = MOUNTAINS;
    pub const WINDSWEPT_FOREST: BiomeID = WOODED_MOUNTAINS;
    pub const WINDSWEPT_GRAVELLY_HILLS: BiomeID = GRAVELLY_MOUNTAINS;
    pub const WINDSWEPT_SAVANNA: BiomeID = SHATTERED_SAVANNA;
    pub const WOODED_BADLANDS: BiomeID = WOODED_BADLANDS_PLATEAU;

    // --- 1.19 The Wild Update ---
    pub const DEEP_DARK: BiomeID = 183;
    pub const MANGROVE_SWAMP: BiomeID = 184;

    // --- 1.20 Trails & Tales ---
    pub const CHERRY_GROVE: BiomeID = 185;

    // --- 1.21 Winter Drop ---
    pub const PALE_GARDEN: BiomeID = 186;
    pub struct BiomeFilter {
        bits: [u64; 4],
    }

    impl BiomeFilter {
        pub fn new(ids: &[BiomeID]) -> Self {
            let mut filter = Self { bits: [0; 4] };
            for &id in ids {
                if id >= 0 {
                    let idx = (id as usize) / 64;
                    let bit = (id as usize) % 64;
                    filter.bits[idx] |= 1 << bit;
                }
            }
            filter
        }

        #[inline(always)]
        pub fn contains(&self, id: BiomeID) -> bool {
            if id < 0 || id > 255 { return false; }
            let idx = (id as usize) / 64;
            let bit = (id as usize) % 64;
            (self.bits[idx] & (1 << bit)) != 0
        }

    }

    use std::sync::OnceLock;

    static OVERWORLD_FILTER: OnceLock<BiomeFilter> = OnceLock::new();

    pub fn is_overworld(id: BiomeID) -> bool {
        let filter = OVERWORLD_FILTER.get_or_init(|| {
            BiomeFilter::new(&[
                OCEAN, PLAINS, DESERT, MOUNTAINS, FOREST, TAIGA, SWAMP, RIVER,
                FROZEN_OCEAN, FROZEN_RIVER, SNOWY_TUNDRA, SNOWY_MOUNTAINS,
                MUSHROOM_FIELDS, MUSHROOM_FIELD_SHORE, BEACH, DESERT_HILLS,
                WOODED_HILLS, TAIGA_HILLS, MOUNTAIN_EDGE, JUNGLE, JUNGLE_HILLS,
                JUNGLE_EDGE, DEEP_OCEAN, STONE_SHORE, SNOWY_BEACH, BIRCH_FOREST,
                BIRCH_FOREST_HILLS, DARK_FOREST, SNOWY_TAIGA, SNOWY_TAIGA_HILLS,
                GIANT_TREE_TAIGA, GIANT_TREE_TAIGA_HILLS, WOODED_MOUNTAINS,
                SAVANNA, SAVANNA_PLATEAU, BADLANDS, WOODED_BADLANDS_PLATEAU,
                BADLANDS_PLATEAU, WARM_OCEAN, LUKEWARM_OCEAN, COLD_OCEAN,
                DEEP_WARM_OCEAN, DEEP_LUKEWARM_OCEAN, DEEP_COLD_OCEAN,
                DEEP_FROZEN_OCEAN, BAMBOO_JUNGLE, BAMBOO_JUNGLE_HILLS,
                DRIPSTONE_CAVES, LUSH_CAVES, MEADOW, GROVE, SNOWY_SLOPES,
                JAGGED_PEAKS, FROZEN_PEAKS, STONY_PEAKS, DEEP_DARK, 
                MANGROVE_SWAMP, CHERRY_GROVE, PALE_GARDEN
            ])
        });
        filter.contains(id)
    }

    static STRONGHOLD_FILTER: OnceLock<BiomeFilter> = OnceLock::new();

pub fn is_stronghold_biased(id: BiomeID) -> bool {
    let filter = STRONGHOLD_FILTER.get_or_init(|| {
        BiomeFilter::new(&[
            PLAINS,
            SUNFLOWER_PLAINS,
            SNOWY_PLAINS,
            ICE_SPIKES,
            DESERT,
            FOREST,
            FLOWER_FOREST,
            BIRCH_FOREST,
            DARK_FOREST,
            PALE_GARDEN,
            OLD_GROWTH_BIRCH_FOREST,
            OLD_GROWTH_PINE_TAIGA,
            OLD_GROWTH_SPRUCE_TAIGA,
            TAIGA,
            SNOWY_TAIGA,
            SAVANNA,
            SAVANNA_PLATEAU,
            WINDSWEPT_HILLS,
            WINDSWEPT_GRAVELLY_HILLS,
            WINDSWEPT_FOREST,
            WINDSWEPT_SAVANNA,
            JUNGLE,
            SPARSE_JUNGLE,
            BAMBOO_JUNGLE,
            BADLANDS,
            ERODED_BADLANDS,
            WOODED_BADLANDS,
            MEADOW,
            CHERRY_GROVE,
            GROVE,
            SNOWY_SLOPES,
            FROZEN_PEAKS,
            JAGGED_PEAKS,
            STONY_PEAKS,
            MUSHROOM_FIELDS,
            DRIPSTONE_CAVES,
            LUSH_CAVES,
        ])
    });
    filter.contains(id)
}
}


impl Biomes {
    pub fn new() -> Self {
        let is_overworld = vec![
            "minecraft:mushroom_fields",
            "minecraft:deep_frozen_ocean",
            "minecraft:frozen_ocean",
            "minecraft:deep_cold_ocean",
            "minecraft:cold_ocean",
            "minecraft:deep_ocean",
            "minecraft:ocean",
            "minecraft:deep_lukewarm_ocean",
            "minecraft:lukewarm_ocean",
            "minecraft:warm_ocean",
            "minecraft:stony_shore",
            "minecraft:swamp",
            "minecraft:mangrove_swamp",
            "minecraft:snowy_slopes",
            "minecraft:snowy_plains",
            "minecraft:snowy_beach",
            "minecraft:windswept_gravelly_hills",
            "minecraft:grove",
            "minecraft:windswept_hills",
            "minecraft:snowy_taiga",
            "minecraft:windswept_forest",
            "minecraft:taiga",
            "minecraft:plains",
            "minecraft:meadow",
            "minecraft:beach",
            "minecraft:forest",
            "minecraft:old_growth_spruce_taiga",
            "minecraft:flower_forest",
            "minecraft:birch_forest",
            "minecraft:dark_forest",
            "minecraft:pale_garden",
            "minecraft:savanna_plateau",
            "minecraft:savanna",
            "minecraft:jungle",
            "minecraft:badlands",
            "minecraft:desert",
            "minecraft:wooded_badlands",
            "minecraft:jagged_peaks",
            "minecraft:stony_peaks",
            "minecraft:frozen_river",
            "minecraft:river",
            "minecraft:ice_spikes",
            "minecraft:old_growth_pine_taiga",
            "minecraft:sunflower_plains",
            "minecraft:old_growth_birch_forest",
            "minecraft:sparse_jungle",
            "minecraft:bamboo_jungle",
            "minecraft:eroded_badlands",
            "minecraft:windswept_savanna",
            "minecraft:cherry_grove",
            "minecraft:frozen_peaks",
            "minecraft:dripstone_caves",
            "minecraft:lush_caves",
            "minecraft:deep_dark"
                ].into_iter().map(|x| x.to_string()).collect();
        let stronghold_biased = vec![
            "minecraft:plains",
            "minecraft:sunflower_plains",
            "minecraft:snowy_plains",
            "minecraft:ice_spikes",
            "minecraft:desert",
            "minecraft:forest",
            "minecraft:flower_forest",
            "minecraft:birch_forest",
            "minecraft:dark_forest",
            "minecraft:pale_garden",
            "minecraft:old_growth_birch_forest",
            "minecraft:old_growth_pine_taiga",
            "minecraft:old_growth_spruce_taiga",
            "minecraft:taiga",
            "minecraft:snowy_taiga",
            "minecraft:savanna",
            "minecraft:savanna_plateau",
            "minecraft:windswept_hills",
            "minecraft:windswept_gravelly_hills",
            "minecraft:windswept_forest",
            "minecraft:windswept_savanna",
            "minecraft:jungle",
            "minecraft:sparse_jungle",
            "minecraft:bamboo_jungle",
            "minecraft:badlands",
            "minecraft:eroded_badlands",
            "minecraft:wooded_badlands",
            "minecraft:meadow",
            "minecraft:cherry_grove",
            "minecraft:grove",
            "minecraft:snowy_slopes",
            "minecraft:frozen_peaks",
            "minecraft:jagged_peaks",
            "minecraft:stony_peaks",
            "minecraft:mushroom_fields",
            "minecraft:dripstone_caves",
            "minecraft:lush_caves"
                ].into_iter().map(|x| x.to_string()).collect();
        let mut biome_noise = BiomeNoise::default();
        biome_noise.set_seed(1, false);
        Self {
            is_overworld,
            stronghold_biased,
            biome_noise,
        }
    }
}

impl Biomes {
    pub fn find_biome_horizontal(&self, x: i32, y: i32, z: i32, radius: i32, random_source: &mut JavaUtilRandom) -> Option<(i32, i32, i32)> {
        let x = QuartPos::from_block(x);
        let y = QuartPos::from_block(y);
        let z = QuartPos::from_block(z);
        let radius = QuartPos::from_block(radius);
        let mut res = None;
        let mut count = 0;
        let mut dat = Some(0);
        for j in -radius..(radius+1) {
            let z_d = z + j;
            for i in -radius..(radius+1) {
                let x_d = x + i;
                let biome_id = self.get_noise_biome(x_d, y, z_d, &mut dat);

                if biomes::is_stronghold_biased(biome_id as i16) {
                    if count == 0 || random_source.next_int_bound(count + 1) == 0  {
                        let block_pos = (QuartPos::to_block(x_d), y, QuartPos::to_block(z_d));
                        res = Some(block_pos);
                    }
                    count += 1;
                }
            }
        }
        res
    }
    // x, y, z, radius, increment
    pub fn find_biome_horizontal_ex(&self, i: i32, j: i32, k: i32, l: i32, m: i32, random_source: &mut JavaUtilRandom, bl: bool) -> Option<(i32, i32, i32)> {
        //println!("{}, {}, {}, {}", i, j, k, l);
        let n = QuartPos::from_block(i);
        let o = QuartPos::from_block(k);
        let p = QuartPos::from_block(l);
        let q = QuartPos::from_block(j);
        let mut pair = None;
        let mut r = 0;
        let s = if bl { 0 } else { p };
        let mut t = s;
        let mut dat = Some(0u64);
        while t <= p {
            for u in (-t..(t+1)).step_by(m as usize) {
                let bl2 = u.abs() == t;
                for v in (-t..(t+1)).step_by(m as usize) {
                    if bl {
                        let bl3 = v.abs() == t;
                        if !bl3 && !bl2 {
                            continue
                        }
                    }

                    let w = n + v;
                    let x = o + u;
                    let biome_id = self.get_noise_biome(w, q, x, &mut dat);
                    if biomes::is_stronghold_biased(biome_id as i16) {
                        //println!("{u}:{v}:{biome_id}:{}",random_source.seed);
                        if r == 0 || random_source.next_int_bound(r + 1) == 0  {
                            let block_pos = (QuartPos::to_block(w), j, QuartPos::to_block(x));
                            if bl {
                                return Some(block_pos)
                            }
                            pair = Some(block_pos);
                            //println!("{r}: {w}, {x}, {pair:?}");
                        }
                        r += 1;
                    }
                }
            }
            //println!();
            t += m;
        }
        pair
    }

    pub fn get_noise_biome(&self, x: i32, y: i32, z: i32, dat: &mut Option<u64>) -> i32 {
        self.biome_noise.sample(x, y, z, 0, dat)
    }

}
