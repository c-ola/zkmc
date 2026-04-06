#[derive(Clone, Debug)]
pub enum Spline {
    Fixed(f32),
    Node(Box<SplineNode>),
}

#[derive(Clone, Default, Debug)]
pub struct SplineNode {
    pub typ: usize,         // The climate parameter index (0=Cont, 1=Eros, etc.)
    pub loc: Vec<f32>,      // Sample points (knots)
    pub val: Vec<Spline>,   // Values (or nested splines) at knots
    pub der: Vec<f32>,      // Derivatives at knots
}

impl Default for Spline {
    fn default() -> Self {
        Self::Node(Box::new(SplineNode::default()))
    }
}

impl Spline {
    /// The core interpolation logic from your C code (getSpline)
    pub fn sample(&self, climate_params: &[f32; 4]) -> f32 {
        match self {
            Spline::Fixed(v) => *v,
            Spline::Node(node) => {
                let f = climate_params[node.typ];
                let len = node.loc.len();

                // 1. Find the interval [i-1, i] containing f
                let mut i = 0;
                while i < len && node.loc[i] < f {
                    i += 1;
                }

                // 2. Handle values outside the defined range (linear extrapolation)
                if i == 0 || i == len {
                    let idx = if i == 0 { 0 } else { len - 1 };
                    let val = node.val[idx].sample(climate_params);
                    return val + node.der[idx] * (f - node.loc[idx]);
                }

                // 3. Cubic Hermite Interpolation
                let g = node.loc[i - 1]; // x1
                let h = node.loc[i];     // x2
                let gap = h - g;
                let k = (f - g) / gap;   // t (0.0 to 1.0)

                let n = node.val[i - 1].sample(climate_params); // y1
                let o = node.val[i].sample(climate_params);     // y2
                
                let l = node.der[i - 1]; // d1
                let m = node.der[i];     // d2

                // These are the Hermite basis calculations simplified for performance
                let p = l * gap - (o - n);
                let q = -m * gap + (o - n);
                
                // Final interpolation formula
                lerp(k, n, o) + k * (1.0 - k) * lerp(k, p, q)
            }
        }
    }
}

#[inline(always)]
fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

pub fn init_biome_noise() -> Spline {
    // Constants from your C code
    let sp1 = create_land_spline(-0.15, 0.00, 0.0, 0.1, 0.00, -0.03, false);
    let sp2 = create_land_spline(-0.10, 0.03, 0.1, 0.1, 0.01, -0.03, false);
    let sp3 = create_land_spline(-0.10, 0.03, 0.1, 0.7, 0.01, -0.03, true);
    let sp4 = create_land_spline(-0.05, 0.03, 0.1, 1.0, 0.01, 0.01, true);

    Spline::Node(Box::new(SplineNode {
        typ: 0, // SP_CONTINENTALNESS
        loc: vec![-1.10, -1.02, -0.51, -0.44, -0.18, -0.16, -0.15, -0.10, 0.25, 1.00],
        val: vec![
            Spline::Fixed(0.044),
            Spline::Fixed(-0.2222),
            Spline::Fixed(-0.2222),
            Spline::Fixed(-0.12),
            Spline::Fixed(-0.12),
            sp1.clone(),
            sp1,
            sp2,
            sp3,
            sp4,
        ],
        der: vec![0.0; 10],
    }))
}

// Helper to replicate createLandSpline
fn create_land_spline(f: f32, g: f32, h: f32, i: f32, j: f32, k: f32, bl: bool) -> Spline {
    let sp1 = create_spline_38219(lerp_f(i, 0.6, 1.5), bl);
    let sp2 = create_spline_38219(lerp_f(i, 0.6, 1.0), bl);
    let sp3 = create_spline_38219(i, bl);
    
    let ih = 0.5 * i;
    let sp4 = create_flat_offset_spline(f - 0.15, ih, ih, ih, i * 0.6, 0.5);
    let sp5 = create_flat_offset_spline(f, j * i, g * i, ih, i * 0.6, 0.5);
    let sp6 = create_flat_offset_spline(f, j, j, g, h, 0.5);
    
    let mut erosion_spline = SplineNode {
        typ: 1, // SP_EROSION
        loc: vec![-0.85, -0.7, -0.4, -0.35, -0.1, 0.2],
        val: vec![sp1, sp2, sp3, sp4, sp5, sp6.clone()],
        der: vec![0.0; 6],
    };

    if bl {
        // Implementation for the "bl" (biased/large) variant logic
        let sp8 = Spline::Node(Box::new(SplineNode {
            typ: 2, // SP_RIDGES
            loc: vec![-1.0, -0.4, 0.0],
            val: vec![Spline::Fixed(f), sp6.clone(), Spline::Fixed(h + 0.07)],
            der: vec![0.0, 0.0, 0.0],
        }));
        erosion_spline.loc.extend_from_slice(&[0.4, 0.45, 0.55, 0.58]);
        erosion_spline.val.extend_from_slice(&[sp6.clone(), sp8.clone(), sp8, sp6]);
        erosion_spline.der.extend_from_slice(&[0.0; 4]);
    }

    let sp9 = create_flat_offset_spline(-0.02, k, k, g, h, 0.0);
    erosion_spline.loc.push(0.7);
    erosion_spline.val.push(sp9);
    erosion_spline.der.push(0.0);

    Spline::Node(Box::new(erosion_spline))
}
fn get_offset_value(weirdness: f32, continentalness: f32) -> f32 {
    let f0 = 1.0 - (1.0 - continentalness) * 0.5;
    let f1 = 0.5 * (1.0 - continentalness);
    let f2 = (weirdness + 1.17) * 0.46082947;
    let off = f2 * f0 - f1;

    if weirdness < -0.7 {
        if off > -0.2222 { off } else { -0.2222 }
    } else {
        if off > 0.0 { off } else { 0.0 }
    }
}

fn create_spline_38219(f: f32, bl: bool) -> Spline {
    let mut sp = SplineNode {
        typ: 2, // SP_RIDGES
        loc: vec![],
        val: vec![],
        der: vec![],
    };

    let i = get_offset_value(-1.0, f);
    let k = get_offset_value(1.0, f);
    let l_factor = 1.0 - (1.0 - f) * 0.5;
    let u_val = 0.5 * (1.0 - f);
    let l = u_val / (0.46082947 * l_factor) - 1.17;

    if l > -0.65 && l < 1.0 {
        let u = get_offset_value(-0.65, f);
        let p = get_offset_value(-0.75, f);
        let q = (p - i) * 4.0;
        let r = get_offset_value(l, f);
        let s = (k - r) / (1.0 - l);

        // addSplineVal equivalents
        sp.loc.extend_from_slice(&[-1.0, -0.75, -0.65, l - 0.01, l, 1.0]);
        sp.val.push(Spline::Fixed(i));
        sp.val.push(Spline::Fixed(p));
        sp.val.push(Spline::Fixed(u));
        sp.val.push(Spline::Fixed(r));
        sp.val.push(Spline::Fixed(r));
        sp.val.push(Spline::Fixed(k));
        sp.der.extend_from_slice(&[q, 0.0, 0.0, 0.0, s, s]);
    } else {
        let u = (k - i) * 0.5;
        if bl {
            sp.loc.extend_from_slice(&[-1.0, 0.0, 1.0]);
            sp.val.push(Spline::Fixed(if i > 0.2 { i } else { 0.2 }));
            sp.val.push(Spline::Fixed(lerp_f(0.5, i, k)));
            sp.val.push(Spline::Fixed(k));
            sp.der.extend_from_slice(&[0.0, u, u]);
        } else {
            sp.loc.extend_from_slice(&[-1.0, 1.0]);
            sp.val.push(Spline::Fixed(i));
            sp.val.push(Spline::Fixed(k));
            sp.der.extend_from_slice(&[u, u]);
        }
    }
    Spline::Node(Box::new(sp))
}

fn create_flat_offset_spline(f: f32, g: f32, h: f32, i: f32, j: f32, k: f32) -> Spline {
    let mut l = 0.5 * (g - f);
    if l < k { l = k; }
    let m = 5.0 * (h - g);

    Spline::Node(Box::new(SplineNode {
        typ: 2, // SP_RIDGES
        loc: vec![-1.0, -0.4, 0.0, 0.4, 1.0],
        val: vec![
            Spline::Fixed(f),
            Spline::Fixed(g),
            Spline::Fixed(h),
            Spline::Fixed(i),
            Spline::Fixed(j),
        ],
        der: vec![
            l,
            if l < m { l } else { m },
            m,
            2.0 * (i - h),
            0.7 * (j - i),
        ],
    }))
}

#[inline(always)]
fn lerp_f(t: f32, a: f32, b: f32) -> f32 { a + t * (b - a) }
