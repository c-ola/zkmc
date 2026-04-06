use core::f32;

use crate::{
    minecraft::{
        biome_tree::btree21,
        climate::{Climate, quantize_f32},
    }, rng::{JavaUtilRandom, RandomSource, Xoroshiro}, util::{Spline, init_biome_noise}, tree::{get_np_dist, get_resulting_node}
};

#[inline(always)]
pub fn lerp(part: f32, from: f32, to: f32) -> f32 {
    from + part * (to - from)
}

#[inline(always)]
fn grad(hash: u8, x: f32, y: f32, z: f32) -> f32 {
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

#[inline(always)]
pub fn lerp_f32(part: f32, from: f32, to: f32) -> f32 {
    from + part * (to - from)
}

#[inline(always)]
fn grad_f32(hash: u8, x: f32, y: f32, z: f32) -> f32 {
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



#[derive(Debug, Copy, Clone)]
pub struct PerlinNoise {
    d: [u8; 512],
    a: f32,
    b: f32,
    c: f32,
    amplitude: f32,
    lacuranity: f32,
    d2: f32,
    t2: f32,
    i2: i32,
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self {
            d: [0u8; 512],
            a: 0.0,
            b: 0.0,
            c: 0.0,
            amplitude: 0.0,
            lacuranity: 0.0,
            d2: 0.0,
            t2: 0.0,
            i2: 0,
        }
    }
}

impl PerlinNoise {
    pub fn init(random: &mut JavaUtilRandom) -> Self {
        let a = random.next_f64();
        let b = random.next_f64();
        let c = random.next_f64();
        let mut d = [0u8; 512];
        for i in 0..256 {
            d[i] = i as u8;
        }
        for i in 0..256usize {
            let j = random.next_i32_bound(256 - i as i32) as usize + i;
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
        Self {
            a: a as f32,
            b: b as f32,
            c: c as f32,
            amplitude: 1.0,
            lacuranity: 1.0,
            d,
            d2: d2 as f32,
            t2: t2 as f32,
            i2
        }
    }

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

        Self {
            a: a as f32,
            b: b as f32,
            c: c as f32,
            amplitude: 1.0,
            lacuranity: 1.0,
            d,
            d2: d2 as f32,
            t2: t2 as f32,
            i2
        }
    }

    #[inline(always)]
    pub fn sample(&self, d1: f32, d2: f32, d3: f32, yamp: f32, ymin: f32) -> f32 {
        let mut d1 = d1 + self.a;
        let mut d2 = d2 + self.b;
        let mut d3 = d3 + self.c;
        let mut i1 = d1 as i32 - (d1 < 0.0) as i32;
        let mut i2 = d2 as i32 - (d2 < 0.0) as i32;
        let mut i3 = d3 as i32 - (d3 < 0.0) as i32;
        d1 -= i1 as f32;
        d2 -= i2 as f32;
        d3 -= i3 as f32;
        let t1 = d1 * d1 * d1 * (d1 * (d1 * 6.0 - 15.0) + 10.0);
        let t2 = d2 * d2 * d2 * (d2 * (d2 * 6.0 - 15.0) + 10.0);
        let t3 = d3 * d3 * d3 * (d3 * (d3 * 6.0 - 15.0) + 10.0);
        if yamp > 0.0 {
            let yclamp = if ymin < d2 { ymin } else { d2 };
            let div = yclamp / yamp;
            let div_floor = div as i32 - (div < 0.0) as i32;
            d2 -= (div_floor as f32) * yamp;
        }
        i1 &= 0xff;
        i2 &= 0xff;
        i3 &= 0xff;

        // doing this safely is faster than with unchecked
        let a1 = (self.d[i1 as usize] as i32 + i2) as usize;
        let a2 = (self.d[a1] as i32 + i3) as usize;
        let a3 = (self.d[a1 + 1] as i32 + i3) as usize;

        let b1 = (self.d[i1 as usize + 1] as i32 + i2) as usize;
        let b2 = (self.d[b1] as i32 + i3) as usize;
        let b3 = (self.d[b1 + 1] as i32 + i3) as usize;

        let mut l1 = grad(self.d[a2], d1, d2, d3);
        let l2 = grad(self.d[b2], d1 - 1.0, d2, d3);
        let mut l3 = grad(self.d[a3], d1, d2 - 1.0, d3);
        let l4 = grad(self.d[b3], d1 - 1.0, d2 - 1.0, d3);
        let mut l5 = grad(self.d[a2 + 1], d1, d2, d3 - 1.0);
        let l6 = grad(self.d[b2 + 1], d1 - 1.0, d2, d3 - 1.0);
        let mut l7 = grad(self.d[a3 + 1], d1, d2 - 1.0, d3 - 1.0);
        let l8 = grad(self.d[b3 + 1], d1 - 1.0, d2 - 1.0, d3 - 1.0);
        l1 = lerp(t1, l1, l2);
        l3 = lerp(t1, l3, l4);
        l5 = lerp(t1, l5, l6);
        l7 = lerp(t1, l7, l8);

        l1 = lerp(t2, l1, l3);
        l5 = lerp(t2, l5, l7);

        lerp(t3, l1, l5)

        // This is safe because i1,i2,i3 are < 256 and d is 512
        /*unsafe {
        let d = &self.d;
        let a1 = (*d.get_unchecked(i1 as usize) as i32 + i2) as usize;
        let a2 = (*d.get_unchecked(a1) as i32 + i3) as usize;
        let a3 = (*d.get_unchecked(a1 + 1) as i32 + i3) as usize;

        let b1 = (*d.get_unchecked(i1 as usize + 1) as i32 + i2) as usize;
        let b2 = (*d.get_unchecked(b1) as i32 + i3) as usize;
        let b3 = (*d.get_unchecked(b1 + 1) as i32 + i3) as usize;

        let mut l1 = grad(*d.get_unchecked(a2), d1, d2, d3);
        let l2 = grad(*d.get_unchecked(b2), d1 - 1.0, d2, d3);
        let mut l3 = grad(*d.get_unchecked(a3), d1, d2 - 1.0, d3);
        let l4 = grad(*d.get_unchecked(b3), d1 - 1.0, d2 - 1.0, d3);
        let mut l5 = grad(*d.get_unchecked(a2 + 1), d1, d2, d3 - 1.0);
        let l6 = grad(*d.get_unchecked(b2 + 1), d1 - 1.0, d2, d3 - 1.0);
        let mut l7 = grad(*d.get_unchecked(a3 + 1), d1, d2 - 1.0, d3 - 1.0);
        let l8 = grad(*d.get_unchecked(b3 + 1), d1 - 1.0, d2 - 1.0, d3 - 1.0);
        l1 = lerp(t1, l1, l2);
        l3 = lerp(t1, l3, l4);
        l5 = lerp(t1, l5, l6);
        l7 = lerp(t1, l7, l8);

        l1 = lerp(t2, l1, l3);
        l5 = lerp(t2, l5, l7);

        lerp(t3, l1, l5)
        }*/
    }

    #[inline(always)]
    pub fn sample_xz(&self, d1: f32, d3: f32) -> f32 {
        let mut d1 = d1 + self.a;
        let mut d3 = d3 + self.c;

        let mut i1 = d1 as i32 - (d1 < 0.0) as i32;
        let mut i3 = d3 as i32 - (d3 < 0.0) as i32;

        d1 -= i1 as f32;
        d3 -= i3 as f32;

        let t1 = d1 * d1 * d1 * (d1 * (d1 * 6.0 - 15.0) + 10.0);
        let t3 = d3 * d3 * d3 * (d3 * (d3 * 6.0 - 15.0) + 10.0);

        i1 &= 0xff;
        i3 &= 0xff;

        let a1 = (self.d[i1 as usize] as i32 + self.i2) as usize;
        let a2 = (self.d[a1] as i32 + i3) as usize;
        let a3 = (self.d[a1 + 1] as i32 + i3) as usize;

        let b1 = (self.d[i1 as usize + 1] as i32 + self.i2) as usize;
        let b2 = (self.d[b1] as i32 + i3) as usize;
        let b3 = (self.d[b1 + 1] as i32 + i3) as usize;

        let mut l1 = grad(self.d[a2], d1, self.d2, d3);
        let l2 = grad(self.d[b2], d1 - 1.0, self.d2, d3);
        let mut l3 = grad(self.d[a3], d1, self.d2 - 1.0, d3);
        let l4 = grad(self.d[b3], d1 - 1.0, self.d2 - 1.0, d3);
        let mut l5 = grad(self.d[a2 + 1], d1, self.d2, d3 - 1.0);
        let l6 = grad(self.d[b2 + 1], d1 - 1.0, self.d2, d3 - 1.0);
        let mut l7 = grad(self.d[a3 + 1], d1, self.d2 - 1.0, d3 - 1.0);
        let l8 = grad(self.d[b3 + 1], d1 - 1.0, self.d2 - 1.0, d3 - 1.0);
        
        l1 = lerp(t1, l1, l2);
        l3 = lerp(t1, l3, l4);
        l5 = lerp(t1, l5, l6);
        l7 = lerp(t1, l7, l8);

        l1 = lerp(self.t2, l1, l3);
        l5 = lerp(self.t2, l5, l7);

        lerp(t3, l1, l5)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PerlinNoiseF32 {
    d: [u8; 512],
    a: f32,
    b: f32,
    c: f32,
    amplitude: f32,
    lacuranity: f32,
}

impl Default for PerlinNoiseF32 {
    fn default() -> Self {
        Self {
            d: [0u8; 512],
            a: 0.0,
            b: 0.0,
            c: 0.0,
            amplitude: 0.0,
            lacuranity: 0.0,
        }
    }
}

impl PerlinNoiseF32 {
    pub fn init(random: &mut JavaUtilRandom) -> Self {
        let a = random.next_f64() as f32;
        let b = random.next_f64() as f32;
        let c = random.next_f64() as f32;
        let mut d = [0u8; 512];
        for i in 0..256 {
            d[i] = i as u8;
        }
        for i in 0..256usize {
            let j = random.next_i32_bound(256 - i as i32) as usize + i;
            let n = d[i];
            d[i] = d[j];
            d[j] = n;
            d[i + 256] = d[i];
        }
        Self {
            a,
            b,
            c,
            amplitude: 1.0,
            lacuranity: 1.0,
            d,
        }
    }

    pub fn x_init(xrng: &mut Xoroshiro) -> Self {
        let a = xrng.next_f64() as f32 * 256.0;
        let b = xrng.next_f64() as f32 * 256.0;
        let c = xrng.next_f64() as f32 * 256.0;
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

        Self {
            a,
            b,
            c,
            amplitude: 1.0,
            lacuranity: 1.0,
            d,
        }
    }

    #[inline(always)]
    pub fn sample(&self, d1: f32, d2: f32, d3: f32, yamp: f32, ymin: f32) -> f32 {
        let mut d1 = d1 + self.a;
        let mut d2 = d2 + self.b;
        let mut d3 = d3 + self.c;
        let mut i1 = d1 as i32 - (d1 < 0.0) as i32;
        let mut i2 = d2 as i32 - (d2 < 0.0) as i32;
        let mut i3 = d3 as i32 - (d3 < 0.0) as i32;
        d1 -= i1 as f32;
        d2 -= i2 as f32;
        d3 -= i3 as f32;
        let t1 = d1 * d1 * d1 * (d1 * (d1 * 6.0 - 15.0) + 10.0);
        let t2 = d2 * d2 * d2 * (d2 * (d2 * 6.0 - 15.0) + 10.0);
        let t3 = d3 * d3 * d3 * (d3 * (d3 * 6.0 - 15.0) + 10.0);
        if yamp > 0.0 {
            let yclamp = if ymin < d2 { ymin } else { d2 };
            let div = yclamp / yamp;
            let div_floor = div as i32 - (div < 0.0) as i32;
            d2 -= (div_floor as f32) * yamp;
        }
        i1 &= 0xff;
        i2 &= 0xff;
        i3 &= 0xff;

        // doing this safely is faster than with unchecked
        let a1 = (self.d[i1 as usize] as i32 + i2) as usize;
        let a2 = (self.d[a1] as i32 + i3) as usize;
        let a3 = (self.d[a1 + 1] as i32 + i3) as usize;

        let b1 = (self.d[i1 as usize + 1] as i32 + i2) as usize;
        let b2 = (self.d[b1] as i32 + i3) as usize;
        let b3 = (self.d[b1 + 1] as i32 + i3) as usize;

        let mut l1 = grad_f32(self.d[a2], d1, d2, d3);
        let l2 = grad_f32(self.d[b2], d1 - 1.0, d2, d3);
        let mut l3 = grad_f32(self.d[a3], d1, d2 - 1.0, d3);
        let l4 = grad_f32(self.d[b3], d1 - 1.0, d2 - 1.0, d3);
        let mut l5 = grad_f32(self.d[a2 + 1], d1, d2, d3 - 1.0);
        let l6 = grad_f32(self.d[b2 + 1], d1 - 1.0, d2, d3 - 1.0);
        let mut l7 = grad_f32(self.d[a3 + 1], d1, d2 - 1.0, d3 - 1.0);
        let l8 = grad_f32(self.d[b3 + 1], d1 - 1.0, d2 - 1.0, d3 - 1.0);
        l1 = lerp_f32(t1, l1, l2);
        l3 = lerp_f32(t1, l3, l4);
        l5 = lerp_f32(t1, l5, l6);
        l7 = lerp_f32(t1, l7, l8);

        l1 = lerp_f32(t2, l1, l3);
        l5 = lerp_f32(t2, l5, l7);

        lerp_f32(t3, l1, l5)
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
    pub fn x_init(xrng: &mut Xoroshiro, amplitudes: &[f32], omin: i32) -> Self {
        let len = amplitudes.len() as i32;
        let mut lacuna: f32 = 2.0_f32.powi(omin);
        let mut persist: f32 = 2.0_f32.powi(len - 1) / ((1 << len) - 1) as f32;
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
            let mut octave = PerlinNoise::x_init(&mut pxrng);
            //octave.amplitude = (amplitudes[i] * persist) as f32;
            //octave.lacuranity = lacuna as f32;
            octave.amplitude = amplitudes[i] * persist;
            octave.lacuranity = lacuna;
            octaves.push(octave);
            lacuna *= 2.0;
            persist *= 0.5;
        }
        Self { octaves }
    }

    #[inline]
    pub fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        let mut v = 0.0;
        for octave in &self.octaves {
            let lf = octave.lacuranity;
            //let pv = octave.sample(x as f32 * lf, y as f32 * lf, z as f32 * lf, 0.0, 0.0);
            let pv = octave.sample(x * lf, y * lf, z * lf, 0.0, 0.0);
            //let pv = octave.sample_xz(x * lf, z * lf);
            v += octave.amplitude * pv;
        }
        v as f32
    }

    #[inline]
    pub fn sample_xz(&self, x: f32, z: f32) -> f32 {
        let mut v = 0.0;
        for octave in &self.octaves {
            let lf = octave.lacuranity;
            let pv = octave.sample_xz(x * lf, z * lf);
            v += octave.amplitude * pv;
        }
        v as f32
    }
}

#[derive(Debug, Default, Clone)]
pub struct DoublePerlinNoise {
    amplitude: f32,
    oct_a: OctaveNoise,
    oct_b: OctaveNoise,
}

impl DoublePerlinNoise {
    pub fn x_init(xrng: &mut Xoroshiro, amplitudes: &[f32], omin: i32) -> Self {
        let oct_a = OctaveNoise::x_init(xrng, amplitudes, omin);
        let oct_b = OctaveNoise::x_init(xrng, amplitudes, omin);
        let first = amplitudes.iter().position(|&x| x != 0.0).unwrap_or(0);
        let last = amplitudes.iter().rposition(|&x| x != 0.0).unwrap_or(0);
        let span_length = if amplitudes[first] != 0.0 {
            last - first + 1
        } else {
            0
        };

        let amplitude = (5.0 / 3.0) * (span_length as f32 / (span_length + 1) as f32);
        Self {
            oct_a,
            oct_b,
            amplitude,
        }
    }

    pub fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        let f = 337.0 / 331.0;
        let mut v = 0.0;
        v += self.oct_a.sample(x, y, z);
        v += self.oct_b.sample(x * f, y * f, z * f);
        v * self.amplitude
    }

    pub fn sample_xz(&self, x: f32, z: f32) -> f32 {
        let f = 337.0 / 331.0;
        let mut v = 0.0;
        v += self.oct_a.sample_xz(x, z);
        v += self.oct_b.sample_xz(x * f, z * f);
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
        let options: [(Vec<f32>, u64, u64, i32); _] = [
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
        let mut px = x as f32;
        let mut pz = z as f32;

        if !(flags & 0x1 != 0) { // NO_SHIFT
            px += self.shift.sample_xz(x as f32, z as f32) * 4.0;
            pz += self.shift.sample(z as f32, x as f32, 0.0) * 4.0;
        }

        let c = self.continentalness.sample_xz(px, pz);
        let e = self.erosion.sample_xz(px, pz);
        let w = self.weirdness.sample_xz(px, pz);

        if !(flags & 0x2 != 0) { // NO_DEPTH
            let np_param = [
                c as f32,
                e as f32,
                -3.0 * (((w as f32).abs() - 0.6666667).abs() - 0.33333334),
                w as f32,
            ];
            let off: f32 = (self.spline.sample(&np_param) + 0.015) as f32;
            d = 1.0 - (y * 4) as f32 / 128.0 - 83.0 / 160.0 + off;
        }

        let t = self.temperature.sample_xz(px, pz);
        let h = self.humidity.sample_xz(px, pz);
        //let np: Vec<_> = [t, h, c, e, d, w].iter().map(|f| (10000.0 * f) as i64 as u64).collect();
        //let l_np: Vec<_> = [t, h, c, e, d, w].iter().map(|f| (10000.0 * f) as i64).collect();
        //println!("{l_np:?}");
        let quantized = [
            quantize_f32(t),
            quantize_f32(h),
            quantize_f32(c),
            quantize_f32(e),
            quantize_f32(d),
            quantize_f32(w),
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

