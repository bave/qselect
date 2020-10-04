# qselect

quick_select (nth_element)

## example
```rust
extern crate qselect;

fn main()
{
    print!("\nqselect\n");
    for nth in 0..9 {
        let mut v : Vec<usize> = vec![10,90,20,80,30,70,40,60,50];
        let len = v.len();
        qselect(&mut v, 0, len-1, nth);
        print!("{:?}\n", v);
    }


    print!("\nindirect qselect\n");
    let ref_v : Vec<usize> = vec![10,90,20,80,30,70,40,60,50];
    for nth in 0..9 {
        let mut v : Vec<usize> = vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8];
        let len = v.len();
        qselect_indirect(&mut v, 0, len-1, nth, &|x| ref_v[x]);
        print!("ref_vec: {:?}\n", v);
    }
    print!("{:?}\n", ref_v);
}
```

## run
```bash
$ cargo run
qselect
[10, 20, 30, 40, 50, 70, 80, 60, 90]
[10, 20, 30, 40, 50, 70, 80, 60, 90]
[10, 20, 30, 40, 50, 70, 80, 60, 90]
[10, 20, 30, 40, 50, 70, 80, 60, 90]
[10, 20, 30, 40, 50, 60, 80, 70, 90]
[10, 20, 30, 40, 50, 60, 70, 80, 90]
[10, 20, 30, 40, 50, 60, 70, 80, 90]
[10, 20, 30, 40, 50, 60, 70, 80, 90]
[10, 20, 30, 40, 50, 70, 80, 60, 90]

indirect qselect
ref_vec: [0, 2, 4, 6, 8, 5, 3, 7, 1]
ref_vec: [0, 2, 4, 6, 8, 5, 3, 7, 1]
ref_vec: [0, 2, 4, 6, 8, 5, 3, 7, 1]
ref_vec: [0, 2, 4, 6, 8, 5, 3, 7, 1]
ref_vec: [0, 2, 4, 6, 8, 7, 3, 5, 1]
ref_vec: [0, 2, 4, 6, 8, 7, 5, 3, 1]
ref_vec: [0, 2, 4, 6, 8, 7, 5, 3, 1]
ref_vec: [0, 2, 4, 6, 8, 7, 5, 3, 1]
ref_vec: [0, 2, 4, 6, 8, 5, 3, 7, 1]
[10, 90, 20, 80, 30, 70, 40, 60, 50]
```
