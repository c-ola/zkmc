use crate::minecraft::biome_tree::{btree21};
/*pub fn get_np_dist(np: &[u64], idx: i32) -> u64 {
    let mut ds = 0u64;
    let node = btree21::NODES[idx as usize];
    for i in 0..6 {
        let idx = ((node >> 8*i) & 0xff) as usize;
        let a = np[i] as i64 - btree21::PARAM[idx][1] as i64;
        let b = btree21::PARAM[idx][0] as i64 - np[i] as i64;
        //println!("a={a}, b={b}");
        let d = if a > 0 {
            a as u64
        } else if b > 0 {
            b as u64
        } else {
            0
        };
        let d2 = d * d;
        ds += d2;
    }
    ds
}


pub fn get_resulting_node(np: &[u64], idx: i32, alt: i32, ds: u64, depth: i32) -> i32 {
    let steps: [u32; _] = btree21::STEPS;
    //println!("{idx}, {alt}, {ds}, {depth}");
    if steps[depth as usize] == 0 {
        return idx;
    }
    let mut step: u32;
    let mut depth = depth;
    loop {
        step = steps[depth as usize];
        depth += 1;
        if !(idx as usize + step as usize >= btree21::NODES.len()) {
            break
        }
    }
    //println!("{step}, {depth}");
    let mut ds = ds;
    let node: u64 = btree21::NODES[idx as usize];
    let mut inner = node >> 48;
    let mut leaf = alt;
    for _ in 0..btree21::ORDER {
        //println!("{inner}");
        let ds_inner: u64 = get_np_dist(np, inner as i32);
        //println!("ds_inner={ds_inner}");
        if ds_inner < ds {
            let leaf2 = get_resulting_node(np, inner as i32, leaf, ds, depth);
            let ds_leaf2: u64 = if inner == leaf2 as u64 {
                ds_inner
            } else {
                get_np_dist(np, leaf2) as u64
            };

            if ds_leaf2 < ds {
                ds = ds_leaf2;
                leaf = leaf2;
            }
        }

        inner += step as u64;
        if inner as usize >= btree21::NODES.len() {
            break
        }
    }
    leaf
}*/

#[inline(always)]
pub fn get_np_dist(np: &[u64; 6], idx: i32) -> u64 {
    let mut ds = 0u64;
    let node = btree21::NODES[idx as usize];

    // The compiler will unroll this completely
    for i in 0..6 {
        let p_idx = ((node >> (8 * i)) & 0xff) as usize;
        
        // p_idx is masked to 0..255, so bounds checks here usually auto-elide 
        // if btree21::PARAM has exactly 256 elements.
        let min = btree21::PARAM[p_idx][0] as i64;
        let max = btree21::PARAM[p_idx][1] as i64;
        let val = np[i] as i64;

        // Branchless distance to bounding box
        let a = val - max;
        let b = min - val;
        let d = a.max(b).max(0) as u64;

        ds += d * d;
    }
    ds
}

pub fn get_resulting_node(np: &[u64; 6], idx: i32, alt: i32, mut ds: u64, mut depth: i32) -> i32 {
    let steps = btree21::STEPS;
    
    if steps[depth as usize] == 0 {
        return idx;
    }

    let mut step: usize;
    let nodes_len = btree21::NODES.len();
    
    // Find the next valid traversal step
    loop {
        step = steps[depth as usize] as usize;
        depth += 1;
        if (idx as usize) + step < nodes_len {
            break;
        }
    }

    let node = btree21::NODES[idx as usize];
    let mut inner = (node >> 48) as i32;
    let mut leaf = alt;

    for _ in 0..btree21::ORDER {
        let ds_inner = get_np_dist(np, inner);

        // AABB Culling: Only traverse if the bounding box is closer than our current best
        if ds_inner < ds {
            let leaf2 = get_resulting_node(np, inner, leaf, ds, depth);
            
            let ds_leaf2 = if inner == leaf2 {
                ds_inner
            } else {
                get_np_dist(np, leaf2)
            };

            if ds_leaf2 < ds {
                ds = ds_leaf2;
                leaf = leaf2;
            }
        }

        inner += step as i32;
        if inner as usize >= nodes_len {
            break;
        }
    }
    leaf
}
