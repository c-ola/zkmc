type FixedT = fixed::types::I64F64;

use crate::{minecraft::{biome_tree::btree21, climate::{Climate, quantize}}, rng::{RandomSource, Xoroshiro}, tree::{get_np_dist, get_resulting_node}, util::{Spline, init_biome_noise}};

#[inline(always)]
pub fn lerp(part: FixedT, from: FixedT, to: FixedT) -> FixedT {
    from + part * (to - from)
}


#[inline(always)]
fn grad(hash: u8, x: FixedT, y: FixedT, z: FixedT) -> FixedT {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };

    (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
}

#[derive(Debug, Clone)]
pub struct Perlin {
    d: [u8; 512],
    a: FixedT,
    b: FixedT,
    c: FixedT,
    d2: FixedT,
    t2: FixedT,
    i2: FixedT,
}

impl Perlin {
    pub fn x_init(xrng: &mut Xoroshiro) -> Self {
        let a = xrng.next_f64() * 256.0;
        let b = xrng.next_f64() * 256.0;
        let c = xrng.next_f64() * 256.0;
        let mut d = [0u8; 512];
        for i in 0..256 {
            d[i] = i as u8;
        }
        for i in 0..256usize {
            let j = xrng.next_i32_bound(256 - i as i32) as usize + i;
            let n = d[i];
            d[i] = d[j];
            d[j] = n;
            d[i + 256] = d[i];
        }

        let mut d2 = b;
        let i2= d2 as i32 - (d2 < 0.0) as i32;
        d2 -= i2 as f64;
        let t2 = d2 * d2 * d2 * (d2 * (d2 * 6.0 - 15.0) + 10.0);
        let i2 = (i2 & 0xff) as i32;
        let a = FixedT::from_num(a);
        let b = FixedT::from_num(b);
        let c = FixedT::from_num(c);
        let d2 = FixedT::from_num(d2);
        let t2 = FixedT::from_num(t2);
        let i2 = FixedT::from_num(i2);

        Self {
            d,
            a, b, c,
            d2, t2, i2,
        }
    }

    #[inline]
    pub fn sample(&self, d1: FixedT, d2: FixedT, d3: FixedT, yamp: FixedT, ymin: FixedT) -> FixedT {
        let mut d1 = d1 + self.a; // Assumes self.a, self.b, self.c are also FixedT
        let mut d2 = d2 + self.b;
        let mut d3 = d3 + self.c;

        // .to_num::<i32>() truncates toward zero, matching f64 `as i32`
        let mut i1 = d1.to_num::<i32>() - if d1 < FixedT::ZERO { 1 } else { 0 };
        let mut i2 = d2.to_num::<i32>() - if d2 < FixedT::ZERO { 1 } else { 0 };
        let mut i3 = d3.to_num::<i32>() - if d3 < FixedT::ZERO { 1 } else { 0 };

        d1 -= FixedT::from_num(i1);
        d2 -= FixedT::from_num(i2);
        d3 -= FixedT::from_num(i3);

        // Pre-define constants for the fade curve
        let six = FixedT::from_num(6);
        let fifteen = FixedT::from_num(15);
        let ten = FixedT::from_num(10);

        let t1 = d1 * d1 * d1 * (d1 * (d1 * six - fifteen) + ten);
        let t2 = d2 * d2 * d2 * (d2 * (d2 * six - fifteen) + ten);
        let t3 = d3 * d3 * d3 * (d3 * (d3 * six - fifteen) + ten);

        if yamp > FixedT::ZERO {
            let yclamp = if ymin < d2 { ymin } else { d2 };
            let div = yclamp / yamp;
            let div_floor = div.to_num::<i32>() - if div < FixedT::ZERO { 1 } else { 0 };
            d2 -= FixedT::from_num(div_floor) * yamp;
        }

        i1 &= 0xff;
        i2 &= 0xff;
        i3 &= 0xff;

        let a1 = (self.d[i1 as usize] as i32 + i2) as usize;
        let a2 = (self.d[a1] as i32 + i3) as usize;
        let a3 = (self.d[a1 + 1] as i32 + i3) as usize;

        let b1 = (self.d[i1 as usize + 1] as i32 + i2) as usize;
        let b2 = (self.d[b1] as i32 + i3) as usize;
        let b3 = (self.d[b1 + 1] as i32 + i3) as usize;

        let one = FixedT::ONE;

        let mut l1 = grad(self.d[a2], d1, d2, d3);
        let l2 = grad(self.d[b2], d1 - one, d2, d3);
        let mut l3 = grad(self.d[a3], d1, d2 - one, d3);
        let l4 = grad(self.d[b3], d1 - one, d2 - one, d3);
        let mut l5 = grad(self.d[a2 + 1], d1, d2, d3 - one);
        let l6 = grad(self.d[b2 + 1], d1 - one, d2, d3 - one);
        let mut l7 = grad(self.d[a3 + 1], d1, d2 - one, d3 - one);
        let l8 = grad(self.d[b3 + 1], d1 - one, d2 - one, d3 - one);

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
pub struct Octave {
    // perlin, amplitude, lacuranity
    pub octaves: Vec<(Perlin, FixedT, FixedT)>,
}

impl Octave {
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
    pub fn x_init(xrng: &mut Xoroshiro, amplitudes: &[f64], omin: i32) -> Self {
        let len = amplitudes.len() as i32;
        let mut lacuna: f64 = 2.0_f64.powi(omin);
        let mut persist: f64 = 2.0_f64.powi(len - 1) / ((1 << len) - 1) as f64;
        let lo = xrng.next_u64();
        let hi = xrng.next_u64();

        let mut octaves = Vec::new();
        for i in 0..len as usize {
            if amplitudes[i as usize] == 0.0 {
                lacuna *= 2.0;
                persist *= 0.5;
                continue;
            }
            let lo = lo ^ Self::MD5_OCTAVE_N[12 + omin as usize + i].0;
            let hi = hi ^ Self::MD5_OCTAVE_N[12 + omin as usize + i].1;
            let mut pxrng = Xoroshiro::from_parts(lo, hi);
            let perlin = Perlin::x_init(&mut pxrng);
            //octave.amplitude = (amplitudes[i] * persist) as f32;
            //octave.lacuranity = lacuna as f32;
            octaves.push((perlin, FixedT::from_num(amplitudes[i] * persist), FixedT::from_num(lacuna)));
            lacuna *= 2.0;
            persist *= 0.5;
        }
        Self { octaves }
    }

    #[inline]
    pub fn sample(&self, x: FixedT, y: FixedT, z: FixedT) -> FixedT {
        let mut v = FixedT::ZERO;
        for (perlin,lacuranity, amplitude) in &self.octaves {
            let lf = lacuranity;
            //let pv = octave.sample(x as f32 * lf, y as f32 * lf, z as f32 * lf, 0.0, 0.0);
            let pv = perlin.sample(x * lf, y * lf, z * lf, FixedT::ZERO, FixedT::ZERO);
            //let pv = octave.sample_xz(x * lf, z * lf);
            v += amplitude * pv;
        }
        v
    }

    #[inline]
    pub fn sample_xz(&self, x: FixedT, z: FixedT) -> FixedT {
        let mut v = FixedT::ZERO;
        for (perlin, lacunarity, amplitude) in &self.octaves {
            let pv = perlin.sample(x * lacunarity, FixedT::ZERO, z * lacunarity, FixedT::ZERO, FixedT::ZERO);
            v += amplitude * pv;
        }
        v
    }
}

#[derive(Default, Clone, Debug)]
pub struct DoublePerlin {
    amplitude: FixedT,
    oct_a: Octave,
    oct_b: Octave,
}

impl DoublePerlin {
    pub fn x_init(xrng: &mut Xoroshiro, amplitudes: &[f64], omin: i32) -> Self {
        let oct_a = Octave::x_init(xrng, amplitudes, omin);
        let oct_b = Octave::x_init(xrng, amplitudes, omin);
        let first = amplitudes.iter().position(|&x| x != 0.0).unwrap_or(0);
        let last = amplitudes.iter().rposition(|&x| x != 0.0).unwrap_or(0);
        let span_length = if amplitudes[first] != 0.0 {
            last - first + 1
        } else {
            0
        };

        let amplitude = (5.0 / 3.0) * (span_length as f64 / (span_length + 1) as f64);
        Self {
            oct_a,
            oct_b,
            amplitude: FixedT::from_num(amplitude)
        }
    }

    #[inline]
    pub fn sample(&self, x: FixedT, y: FixedT, z: FixedT) -> FixedT {
        let f = FixedT::from_num(337.0 / 331.0);
        let mut v = FixedT::ZERO;
        v += self.oct_a.sample(x, y, z);
        v += self.oct_b.sample(x * f, y * f, z * f);
        v * self.amplitude
    }

    #[inline]
    pub fn sample_xz(&self, x: FixedT, z: FixedT) -> FixedT {
        let f = FixedT::from_num(337.0 / 331.0);
        let mut v = FixedT::ZERO;
        v += self.oct_a.sample_xz(x, z);
        v += self.oct_b.sample_xz(x * f, z * f);
        v * self.amplitude
    }
}


#[derive(Default, Clone, Debug)]
pub struct BiomeNoise {
    shift: DoublePerlin,
    temperature: DoublePerlin,
    humidity: DoublePerlin,
    continentalness: DoublePerlin,
    erosion: DoublePerlin,
    weirdness: DoublePerlin,
    spline: Spline,
}

impl BiomeNoise {
    pub fn set_seed(&mut self, seed: i64, large: bool) {
        let mut xoroshiro = Xoroshiro::with_seed(seed as u64);
        let xlo = xoroshiro.next_u64();
        let xhi = xoroshiro.next_u64();
        self.init_climates(xlo, xhi, large);
        self.spline = init_biome_noise();
    }

    pub fn init_climates(&mut self, xlo: u64, xhi: u64, large: bool) {
        // shift, minecraft:offset
        let options: [(Vec<f64>, u64, u64, i32); _] = [
            (
                vec![1.5, 0.0, 1.0, 0.0, 0.0, 0.0],
                if large {
                    0x944b0073edf549db
                } else {
                    0x5c7e6b29735f0d7f
                },
                if large {
                    0x4ff44347e9d22b96
                } else {
                    0xf7d86f1bbc734988
                },
                if large { -12 } else { -10 },
            ), // temperature
            (
                vec![1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                if large {
                    0x71b8ab943dbd5301
                } else {
                    0x81bb4d22e8dc168e
                },
                if large {
                    0xbb63ddcf39ff7a2b
                } else {
                    0xf1c8b4bea16303cd
                },
                if large { -10 } else { -8 },
            ), // humidity
            (
                vec![1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0],
                if large {
                    0x9a3f51a113fce8dc
                } else {
                    0x83886c9d0ae3a662
                },
                if large {
                    0xee2dbd157e5dcdad
                } else {
                    0xafa638a61b42e8ad
                },
                if large { -11 } else { -9 },
            ), // continentalness
            (
                vec![1.0, 1.0, 0.0, 1.0, 1.0],
                if large {
                    0x8c984b1f8702a951
                } else {
                    0xd02491e6058f6fd8
                },
                if large {
                    0xead7b1f92bae535f
                } else {
                    0x4792512c94c17a80
                },
                if large { -11 } else { -9 },
            ), // erosion
            (
                vec![1.0, 1.0, 1.0, 0.0],
                0x080518cf6af25384,
                0x3f3dfb40a54febd5,
                -3,
            ), // shift
            (
                vec![1.0, 2.0, 1.0, 0.0, 0.0, 0.0],
                0xefc8ef4d36102b34,
                0x1beeeb324a0f24ea,
                -7,
            ), // weirdness
        ];
        let mut climates = Vec::new();
        for opt in options {
            let lo = xlo ^ opt.1;
            let hi = xhi ^ opt.2;
            let mut xrng = Xoroshiro::from_parts(lo, hi);
            climates.push(DoublePerlin::x_init(&mut xrng, &opt.0, opt.3));
        }
        self.temperature = climates[0].clone();
        self.humidity = climates[1].clone();
        self.continentalness = climates[2].clone();
        self.erosion = climates[3].clone();
        self.shift = climates[4].clone();
        self.weirdness = climates[5].clone();
    }

    pub fn sample(&self, x: i32, y: i32, z: i32, flags: u32, dat: &mut Option<u64>) -> i32 {
        //let mut d = FixedT::ZERO;
        let mut d=  0.0;
        let mut x = FixedT::from_num(x);
        //let mut y = FixedT::from_num(y);
        let mut z = FixedT::from_num(z);
        let mut px = x;
        let mut pz = z;

        let four = FixedT::from_num(4);

        if !(flags & 0x1 != 0) { // NO_SHIFT
            px += self.shift.sample_xz(x, z) * four;
            pz += self.shift.sample(z, x, FixedT::ZERO) * four;
        }

        let c = self.continentalness.sample_xz(px, pz);
        let e = self.erosion.sample_xz(px, pz);
        let w = self.weirdness.sample_xz(px, pz);

        if !(flags & 0x2 != 0) { // NO_DEPTH
            let np_param = [
                c.to_num(),
                e.to_num(),
                -3.0 * (((w.to_num::<f32>()).abs() - 0.6666667).abs() - 0.33333334),
                w.to_num()
            ];
            let off: f64 = (self.spline.sample(&np_param) + 0.015) as f64;
            d = 1.0 - (y * 4) as f64 / 128.0 - 83.0 / 160.0 + off;
        }

        let t = self.temperature.sample_xz(px, pz);
        let h = self.humidity.sample_xz(px, pz);
        //let np: Vec<_> = [t, h, c, e, d, w].iter().map(|f| (10000.0 * f) as i64 as u64).collect();
        //let l_np: Vec<_> = [t, h, c, e, d, w].iter().map(|f| (10000.0 * f) as i64).collect();
        //println!("{l_np:?}");
        let quantized = [
            quantize(t.to_num()),
            quantize(h.to_num()),
            quantize(c.to_num()),
            quantize(e.to_num()),
            quantize(d),
            quantize(w.to_num()),
        ];

        Self::p2overworld(&quantized, dat)
    }

    pub fn p2overworld(np: &Climate, dat: &mut Option<u64>) -> i32 {
        let idx = if let Some(d) = dat {
            let alt = *d as i32;
            let ds = get_np_dist(np, alt);
            //println!("ds={ds}");
            let idx = get_resulting_node(np, 0, alt, ds, 0);
            *d = idx as u64;
            idx
        } else {
            get_resulting_node(np, 0, 0, -1i64 as u64, 0)
        };
        //println!("{idx}");
        //panic!("");
        let node = btree21::NODES[idx as usize];
        ((node >> 48) & 0xff) as i32
    }
}
