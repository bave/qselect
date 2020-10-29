//echo "1 2 3 4" | cargo run --release  --bin part

extern crate qselect;
use self::qselect::*;

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main()
{
    let mut a : Vec<usize> = read_vec();
    let l = a.len();
    qselect(&mut a, 0, l-1, 5);
    print!("{:?}\n", a);
}


