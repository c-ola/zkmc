use core::f64;

use crate::{JavaUtilRandom, minecraft::{QuartPos, biome_btree::btree21}, spline::{Spline, init_biome_noise}, tree::{get_np_dist, get_resulting_node}, xoroshiro::Xoroshiro};



pub struct ClimateSampler {
    
}

impl ClimateSampler {
    pub fn sample(i: i32, j: i32, k: i32) { //idk what it returns 
        let l = QuartPos::to_block(i);
        let m = QuartPos::to_block(j);
        let n = QuartPos::to_block(k);
        todo!()
    }
}

pub fn maintain_precision(x: f64) -> f64 {
    return x;// - f64::floor(x / 33554432.0 + 0.5) * 33554432.0;
}

pub fn lerp(part: f64, from: f64, to: f64) -> f64 {
    from + part * (to - from)
}

pub fn indexed_lerp(idx: usize, a: f64, b: f64, c: f64) -> f64 {
    match idx & 0xf {
        0 => a + b,
        1 => -a + b,
        2 => a - b,
        3 => -a - b,
        4 => a + c,
        5 => -a + c,
        6 => a - c,
        7 => -a - c,
        8 => b + c,
        9 => -b + c,
        10 => b - c,
        11 => -b - c,
        12 => a + b,
        13 => -b + c,
        14 => -a + b,
        15 => -b - c,
        _ => unreachable!()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PerlinNoise {
    d: [u8; 512],
    a: f64,
    b: f64,
    c: f64,
    amplitude: f64,
    lacuranity: f64,
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self {
            d: [0u8; 512],
            a: 0.0,
            b: 0.0,
            c: 0.0,
            amplitude: 0.0,
            lacuranity: 0.0
        }
    }
}

impl PerlinNoise {
    pub fn init(random: &mut JavaUtilRandom) -> Self {
        let a = random.next_double();
        let b = random.next_double();
        let c = random.next_double();
        let mut d = [0u8; 512];
        for i in 0..256 {
            d[i] = i as u8;
        }
        for i in 0..256usize {
            let j = random.next_int_bound(256 - i as i32) as usize + i;
            let n = d[i];
            d[i] = d[j];
            d[j] = n;
            d[i + 256] = d[i];
        }
        Self {
            a, b, c,
            amplitude: 1.0,
            lacuranity: 1.0,
            d
        }
    }

    pub fn x_init(xrng: &mut Xoroshiro) -> Self {
        let a = xrng.next_double() * 256.0;
        let b = xrng.next_double() * 256.0;
        let c = xrng.next_double() * 256.0;
        let mut d = [0u8; 512];
        for i in 0..256 {
            d[i] = i as u8;
        }
        for i in 0..256usize {
            let j = (xrng.next_int((256 - i) as u32) + i as i32) as usize;
            let n = d[i];
            d[i] = d[j];
            d[j] = n;
            d[i + 256] = d[i];
        }

        Self {
            a, b, c,
            amplitude: 1.0,
            lacuranity: 1.0,
            d
        }
    }

    pub fn sample(&self, d1: f64, d2: f64, d3: f64, yamp: f64, ymin: f64) -> f64 {
        let mut d1 = d1 + self.a;
        let mut d2 = d2 + self.b;
        let mut d3 = d3 + self.c;
        let mut i1 = d1 as i32 - (d1 < 0.0) as i32;
        let mut i2 = d2 as i32 - (d2 < 0.0) as i32;
        let mut i3 = d3 as i32 - (d3 < 0.0) as i32;
        d1 -= i1 as f64;
        d2 -= i2 as f64;
        d3 -= i3 as f64;
        let t1 = d1 * d1 * d1 * (d1 * (d1 * 6.0 - 15.0) + 10.0);
        let t2 = d2 * d2 * d2 * (d2 * (d2 * 6.0 - 15.0) + 10.0);
        let t3 = d3 * d3 * d3 * (d3 * (d3 * 6.0 - 15.0) + 10.0);
        if yamp > 0.0 {
            let yclamp = if ymin < d2 {ymin} else {d2};
            d2 -= (yclamp/yamp).floor() * yamp;
        }
        i1 &= 0xff;
        i2 &= 0xff;
        i3 &= 0xff;

        let a1 = self.d[i1 as usize] as i32 + i2;
        let a2 = self.d[a1 as usize] as i32 + i3;
        let a3 = self.d[a1 as usize + 1] as i32 + i3;
        let b1 = self.d[i1 as usize + 1] as i32 + i2;
        let b2 = self.d[b1 as usize] as i32 + i3;
        let b3 = self.d[b1 as usize + 1] as i32 + i3;
        let mut l1 = indexed_lerp(self.d[a2 as usize] as usize,   d1,   d2,   d3);
        let l2 = indexed_lerp(self.d[b2 as usize] as usize,   d1-1.0, d2,   d3);
        let mut l3 = indexed_lerp(self.d[a3 as usize] as usize,   d1,   d2-1.0, d3);
        let l4 = indexed_lerp(self.d[b3 as usize] as usize,   d1-1.0, d2-1.0, d3);
        let mut l5 = indexed_lerp(self.d[a2 as usize+1] as usize, d1,   d2,   d3-1.0);
        let l6 = indexed_lerp(self.d[b2 as usize+1] as usize, d1-1.0, d2,   d3-1.0);
        let mut l7 = indexed_lerp(self.d[a3 as usize+1] as usize, d1,   d2-1.0, d3-1.0);
        let l8 = indexed_lerp(self.d[b3 as usize+1] as usize, d1-1.0, d2-1.0, d3-1.0);
        l1 = lerp(t1, l1, l2);
        l3 = lerp(t1, l3, l4);
        l5 = lerp(t1, l5, l6);
        l7 = lerp(t1, l7, l8);

        l1 = lerp(t2, l1, l3);
        l5 = lerp(t2, l5, l7);

        lerp(t3, l1, l5)
    }
}

#[derive(Debug, Default, Clone)]
pub struct OctaveNoise {
    pub octaves: Vec<PerlinNoise>,
}

impl OctaveNoise {
    const MD5_OCTAVE_N: [(u64, u64); 13] = [
        (0xb198de63a8012672, 0x7b84cad43ef7b5a8), // md5 "octave_-12"
        (0x0fd787bfbc403ec3, 0x74a4a31ca21b48b8), // md5 "octave_-11"
        (0x36d326eed40efeb2, 0x5be9ce18223c636a), // md5 "octave_-10"
        (0x082fe255f8be6631, 0x4e96119e22dedc81), // md5 "octave_-9"
        (0x0ef68ec68504005e, 0x48b6bf93a2789640), // md5 "octave_-8"
        (0xf11268128982754f, 0x257a1d670430b0aa), // md5 "octave_-7"
        (0xe51c98ce7d1de664, 0x5f9478a733040c45), // md5 "octave_-6"
        (0x6d7b49e7e429850a, 0x2e3063c622a24777), // md5 "octave_-5"
        (0xbd90d5377ba1b762, 0xc07317d419a7548d), // md5 "octave_-4"
        (0x53d39c6752dac858, 0xbcd1c5a80ab65b3e), // md5 "octave_-3"
        (0xb4a24d7a84e7677b, 0x023ff9668e89b5c4), // md5 "octave_-2"
        (0xdffa22b534c5f608, 0xb9b67517d3665ca9), // md5 "octave_-1"
        (0xd50708086cef4d7c, 0x6e1651ecc7f43309), // md5 "octave_0"
    ];
    pub fn x_init(xrng: &mut Xoroshiro, amplitudes: &[f64], omin: i32) -> Self{
        let len = amplitudes.len() as i32;
        let mut lacuna: f64 = 2.0_f64.powi(omin);
        let mut persist: f64 = 2.0_f64.powi(len - 1) / ((1 << len) - 1) as f64;
        let lo = xrng.next_long();
        let hi = xrng.next_long();

        let mut octaves = Vec::new();
        for i in 0..len as usize {
            if amplitudes[i as usize] == 0.0 {
                lacuna *= 2.0;
                persist *= 0.5;
                continue
            }
            let lo = lo ^ Self::MD5_OCTAVE_N[12  + omin as usize + i].0;
            let hi = hi ^ Self::MD5_OCTAVE_N[12  + omin as usize + i].1;
            let mut pxrng = Xoroshiro::from_parts(lo, hi);
            let mut octave = PerlinNoise::x_init(&mut pxrng);
            octave.amplitude = amplitudes[i] * persist;
            octave.lacuranity = lacuna;
            octaves.push(octave);
            lacuna *= 2.0;
            persist *= 0.5;
        }
        Self {
            octaves
        }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut v = 0.0;
        for i in 0..self.octaves.len() {
            let octave = &self.octaves[i];
            let lf = octave.lacuranity;
            let ax = maintain_precision(x * lf);
            let ay = maintain_precision(y * lf);
            let az = maintain_precision(z * lf);
            let pv = octave.sample(ax, ay, az, 0.0, 0.0);
            v += octave.amplitude * pv;
        }
        v
    }
}

#[derive(Debug, Default, Clone)]
pub struct DoublePerlinNoise {
    amplitude: f64,
    oct_a: OctaveNoise,
    oct_b: OctaveNoise,
}

impl DoublePerlinNoise {
    pub fn x_init(xrng: &mut Xoroshiro, amplitudes: &[f64], omin: i32) -> Self {
        let oct_a = OctaveNoise::x_init(xrng, amplitudes, omin);
        let oct_b = OctaveNoise::x_init(xrng, amplitudes, omin);
        let mut len = amplitudes.len();
        for i in (0..len).rev() {
            if amplitudes[i] != 0.0 {
                break
            } else {
                len -= 1;
            }
        }
        for i in 0..len {
            if amplitudes[i] != 0.0 {
                break
            } else {
                len -= 1;
            }
        }
        let amplitude = (5.0 / 3.0) * (len as f64/ (len + 1) as f64);
        Self {
            oct_a, oct_b, amplitude
        }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let f = 337.0 / 331.0;
        let mut v = 0.0;
        v += self.oct_a.sample(x, y, z);
        v += self.oct_b.sample(x*f, y*f, z*f);
        v * self.amplitude
    }
}

#[derive(Default, Debug)]
pub struct BiomeNoise {
    shift: DoublePerlinNoise,
    temperature: DoublePerlinNoise,
    humidity: DoublePerlinNoise,
    continentalness: DoublePerlinNoise,
    erosion: DoublePerlinNoise,
    weirdness: DoublePerlinNoise,
    spline: Spline,
    /*oct: [PerlinNoise; 2 * 23],
    sp: Vec<Spline>,
    ss: SplineStack,*/
}

impl BiomeNoise {
    pub fn set_seed(&mut self, seed: i64, large: bool) {
        let mut xoroshiro = Xoroshiro::with_seed(seed as u64);
        let xlo = xoroshiro.next_long();
        let xhi = xoroshiro.next_long();
        self.init_climates(xlo, xhi, large);
        self.spline = init_biome_noise();
    }

    pub fn init_climates(&mut self, xlo: u64, xhi: u64, large: bool) {
        // shift, minecraft:offset
        let options: [(Vec<f64>, u64, u64, i32); _] = [
            (vec![1.5, 0.0, 1.0, 0.0, 0.0, 0.0], if large { 0x944b0073edf549db } else { 0x5c7e6b29735f0d7f} , if large {0x4ff44347e9d22b96} else {0xf7d86f1bbc734988}, if large {-12} else {-10}), // temperature
            (vec![1.0, 1.0, 0.0, 0.0, 0.0, 0.0], if large {0x71b8ab943dbd5301} else {0x81bb4d22e8dc168e}, if large {0xbb63ddcf39ff7a2b} else {0xf1c8b4bea16303cd}, if large {-10} else {-8}), // humidity
            (vec![1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0], if large {0x9a3f51a113fce8dc} else {0x83886c9d0ae3a662}, if large {0xee2dbd157e5dcdad} else {0xafa638a61b42e8ad}, if large {-11} else {-9}), // continentalness
            (vec![1.0, 1.0, 0.0, 1.0, 1.0], if large {0x8c984b1f8702a951} else {0xd02491e6058f6fd8}, if large {0xead7b1f92bae535f} else {0x4792512c94c17a80}, if large {-11} else {-9}), // erosion
            (vec![1.0, 1.0, 1.0, 0.0], 0x080518cf6af25384, 0x3f3dfb40a54febd5, -3), // shift
            (vec![1.0, 2.0, 1.0, 0.0, 0.0, 0.0], 0xefc8ef4d36102b34, 0x1beeeb324a0f24ea, -7), // weirdness
        ];
        let mut climates = Vec::new();
        for opt in options {
            let lo = xlo ^ opt.1;
            let hi = xhi ^ opt.2;
            let mut xrng = Xoroshiro::from_parts(lo, hi);
            climates.push(DoublePerlinNoise::x_init(&mut xrng, &opt.0, opt.3));
        }
        self.temperature = climates[0].clone();
        self.humidity = climates[1].clone();
        self.continentalness = climates[2].clone();
        self.erosion = climates[3].clone();
        self.shift = climates[4].clone();
        self.weirdness = climates[5].clone();
    }

    pub fn sample(&self, x: i32, y: i32, z: i32, flags: u32, dat: &mut Option<u64>) -> i32 {
        let mut d = 0.0;
        let mut px = x as f64;
        let mut pz = z as f64;
        
        if !(flags & 0x1 != 0) { // no shift
            px += self.shift.sample(x as f64, 0.0, z as f64) * 4.0;
            pz += self.shift.sample(z as f64, x as f64, 0.0) * 4.0;
        }

        let c = self.continentalness.sample(px, 0.0, pz);
        let e = self.erosion.sample(px, 0.0, pz);
        let w = self.weirdness.sample(px, 0.0, pz);

        if !(flags & 0x2 != 0) { // no_depth, not needed?
            let np_param = [
                c as f32,
                e as f32,
                -3.0 * (((w as f32).abs() - 0.6666667).abs() - 0.33333334),
                w as f32,
            ];
            let off: f64 = (self.spline.sample(&np_param) + 0.015) as f64;

            // double py = y + sampleDoublePerlin(&bn->shift, y, z, x) * 4.0;
            d = 1.0 - (y * 4) as f64 / 128.0 - 83.0 / 160.0 + off;
        }

        let t = self.temperature.sample(px, 0.0, pz);
        let h = self.humidity.sample(px, 0.0, pz);
        let np: Vec<_> = [t, h, c, e, d, w].iter().map(|f| (10000.0 * f) as i64 as u64).collect();
        //let l_np: Vec<_> = [t, h, c, e, d, w].iter().map(|f| (10000.0 * f) as i64).collect();
        //println!("{l_np:?}");

        Self::p2overworld(np, dat)
    }

    pub fn p2overworld(np: Vec<u64>, dat: &mut Option<u64>) -> i32 {
        let idx = if let Some(d) = dat {
            let alt = *d as i32;
            let ds = get_np_dist(&np, alt);
            //println!("ds={ds}");
            let idx = get_resulting_node(&np, 0, alt, ds, 0);
            *d = idx as u64;
            idx
        } else {
            get_resulting_node(&np, 0, 0, -1i64 as u64, 0)
        };
        //println!("{idx}");
        //panic!("");
        let node = btree21::NODES[idx as usize];
        ((node >> 48) & 0xff) as i32
    }
}
