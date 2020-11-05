pub mod simd;

pub fn part<T>(array: &mut [T], left: usize, right: usize) -> usize
where T: PartialOrd + std::fmt::Display
{
    let mut i = left;
    for j in left .. right {
        if &array[j] <= &array[right] {
            array.swap(i, j);
            i += 1;
        }
    };
    array.swap(i, right);
    return i;
}

pub fn qselect<T>(array: &mut [T], left: usize, right: usize, nth: usize)
where T: PartialOrd + std::fmt::Display
{
    if left < right {
        let i = part(array, left, right);
        if nth < i {
            if i != 0 { qselect(array, left, i-1, nth) };
        } else {
            qselect(array, i+1, right, nth);
        }
    }
}

#[inline]
pub fn part_indirect<T, F>(array: &mut [usize], left: usize, right: usize, f: &F) -> usize
where T: PartialOrd, F: Fn(usize) -> T
{
    let mut i = left;
    for j in left..right {
        if f(array[j]) <= f(array[right]) {
            array.swap(i, j);
            i += 1;
        }
    };
    array.swap(i, right);
    return i;
}

#[inline]
pub fn qselect_indirect<T, F>(array: &mut [usize], left: usize, right: usize, nth: usize, key: &F)
where T: PartialOrd, F: Fn(usize) -> T
{
    if left < right {
        let i = part_indirect(array, left, right, key);
        if nth < i {
            qselect_indirect(array, left, i-1, nth, key);
        } else {
            qselect_indirect(array, i+1, right, nth, key);
        }
    }
}

//sample code of lomuto partitioning
#[allow(dead_code)]
fn lomuto<T>(array: &mut [T], left: usize, right: usize) -> usize
where T: PartialOrd
{
    let mut i = left;
    for j in left .. right {
        if &array[j] <= &array[right] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, right);
    return i;
}

//sample code of hoare partitioning
#[allow(dead_code)]
fn hoare<T>(array: &mut[T], left: usize, right: usize) -> usize
where T: PartialOrd
{
    let mut i:i64 = left as i64 - 1;
    let mut j:i64 = right as i64 + 1;
    loop {
        i += 1;
        while &array[i as usize] < &array[left] {
            i += 1;
        }
        j = j - 1;
        while &array[j as usize] > &array[left] {
            j -= 1;
        }
        if i >= j {
            return j as usize;
        }
        array.swap(i as usize, j as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoare() {
        let mut v : Vec<u64> = vec![2,15,6,7,1,16,10,11,13,9,12,3,4,8,5,14];
        let len = v.len();
        let i = hoare(&mut v, 0, len-1);
        print!("vec: {:?}, i:{}\n", v, i);
        assert_eq!(i, 0);
    }

    #[test]
    fn test_lomuto() {
        let mut v : Vec<u64> = vec![2,15,6,7,1,16,10,11,13,9,12,3,4,8,5,14];
        let len = v.len();
        let i = lomuto(&mut v, 0, len-1);
        print!("vec: {:?}, i:{}\n", v, i);
        assert_eq!(i, 13);
    }

    #[test]
    fn test_direct() {
        for i in 0..10 {
            let mut v : Vec<u64> = vec![1, 9, 2, 8, 3, 7, 4, 6, 5, 0];
            let len = v.len();
            qselect(&mut v, 0, len-1, i);
            print!("vec: {:?}\n", v);
        }
    }

    #[test]
    fn test_simd_direct() {
        let mut rng = thread_rng();
        let rmin = -10000 as f32;
        let rmax = 10000 as f32;

        let min_node_numbers : usize = 1_000;
        let max_node_numbers : usize = 1_001;

        let mut total = 0u64;
        let mut counter = 0;
        while counter < 100 {
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

            print!("pass: nth_{} ({}.{:09}[sec]), t:{}\n",
            nth, end.as_secs(), end.subsec_nanos(), total/counter);
        }
    }

    #[test]
    fn test_indirect() {
        //print!("{:?}\n", b);
        //let mut v = vec![1,2,3,4,5,6,7,8,9];

        let b : Vec<f64> = vec![10.0,90.0,20.0,80.0,30.0,70.0,40.0,60.0,0.0,12.0];
        for i in 0..10 {
            let mut v : Vec<usize> = vec![0,1,2,3,4,5,6,7,8,9];
            let len = v.len();
            qselect_indirect(&mut v, 0, len-1, i, &|x| b[x]);
            print!("vec: {:?}\n", v);
            /*
            for j in v.iter().map(|x| &b[*x]) {
                print!("{},", j);
            }
            println!("");
            */
        }
    }

    use rand::prelude::*;
    #[test]
    fn test_simd_indirect() {
        let mut rng = thread_rng();
        let rmin = -10000 as f32;
        let rmax = 10000 as f32;

        let min_node_numbers : usize = 1_000;
        let max_node_numbers : usize = 1_001;

        let mut total = 0u64;
        let mut counter = 0;
        while counter < 100 {
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
            let mut indices: Vec<i32> = (0 .. vec.len()).map(|x| x as i32).collect();

            let start = std::time::Instant::now();
            let _ = simd::qselect_indirect(&mut indices, left, right, nth, &|x| vec[x]);
            let end = start.elapsed();

            for i in 0 .. nth {
                assert!(vec[indices[i] as usize] <= vec[indices[nth] as usize]);
            }
            for i in nth .. vec.len() {
                assert!(vec[indices[i] as usize] >= vec[indices[nth] as usize]);
            }
            total += end.subsec_nanos() as u64;

            print!("count: {}, nodes: {}    , pass: nth_{}   ({}.{:09}[sec]), t:{}\n",
            counter, node_numbers, nth, end.as_secs(), end.subsec_nanos(), total/counter);
        }
    }
}

