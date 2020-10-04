
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
    let i = part(&mut a, 0, l-1);
    print!("{}\n", i);

}


