
pub fn part<T>(array: &mut [T], left: usize, right: usize) -> usize
where T: Ord
{
    let mut i = left;
    for j in left..right {
        if &array[j] <= &array[right] {
            array.swap(i, j);
            i += 1;
        }
    };
    array.swap(i, right);
    return i;
}


pub fn qselect<T>(array: &mut [T], left: usize, right: usize, nth: usize)
where T: Ord
{
    if right <= left { return; }
    let i = part(array, left, right);
    match nth < i {
        true => {
            qselect(array, left, i-1, nth);
        },
        false => {
            qselect(array, i+1, right, nth);
        },
    }
}

pub fn part_indirect<F>(array: &mut [usize], left: usize, right: usize, f: &F) -> usize
where F: Fn(usize) -> usize
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

pub fn qselect_indirect<F>(array: &mut [usize], left: usize, right: usize, nth: usize, key: &F)
where F: Fn(usize) -> usize
{
    if right <= left { return; }
    let i = part_indirect(array, left, right, key);
    match nth < i {
        true => {
            qselect_indirect(array, left, i-1, nth, key);
        },
        false => {
            qselect_indirect(array, i+1, right, nth, key);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {
        for i in 0..10 {
            let mut v : Vec<u64> = vec![10, 1, 9, 2, 8, 3, 7, 4, 6, 5];
            let len = v.len();
            qselect(&mut v, 0, len-1, i);
            print!("vec: {:?}\n", v);
        }
    }

    #[test]
    fn test_indirect() {
        //print!("{:?}\n", b);
        //let mut v = vec![1,2,3,4,5,6,7,8,9];

        let b : Vec<usize> = vec![10,90,20,80,30,70,40,60,50];
        for i in 0..10 {
            let mut v : Vec<usize> = vec![0,1,2,3,4,5,6,7,8];
            let len = v.len();
            qselect_indirect(&mut v, 0, len-1, i, &|x| b[x]);
            print!("vec :{:?}\n", v);
        }
        print!("base: {:?}\n", b);
    }
}

