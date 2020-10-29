#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate qselect;
use self::qselect::*;

use rand::prelude::*;

#[cfg(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2"))]
fn main() {
    let mut rng = thread_rng();
    let rmin = -10000 as f32;
    let rmax = 10000 as f32;

    let min_node_numbers : usize = 1_000;
    let max_node_numbers : usize = 1_000_000;

    let mut total = 0u64;
    let mut counter = 0;
    loop {
        counter += 1;
        let node_numbers = rng.gen_range(min_node_numbers, max_node_numbers) as usize;
        let mut vec : Vec<f32> = Vec::new();
        for _ in 0 .. node_numbers {
            let x: f32 = rng.gen_range(rmin, rmax) as f32;
            vec.push(x);
        }

        let left  : usize = 0;
        let right : usize = vec.len() - 1;
        let nth: usize = rng.gen_range(0, node_numbers) as usize;

        let start = std::time::Instant::now();
        let _ = simd::qselect(&mut vec, left, right, nth);
        let end = start.elapsed();


        for i in 0 .. nth {
            assert!(vec[i] <= vec[nth]);
        }
        for i in nth .. vec.len() {
            assert!(vec[i] >= vec[nth]);
        }
        total += end.subsec_nanos() as u64;

        print!("count:{} nodes:{}, nth:{} ({}.{:09}[sec]), t:{}\n",
        counter, node_numbers, nth, end.as_secs(), end.subsec_nanos(), total/counter);
    }
}

#[cfg(not(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2")))]
fn main() {
    print!("RUSTFLAGS=\"-C target-feature=+avx2,+popcnt,+bmi2\" cargo run --release  --bin simd_direct\n");
}
