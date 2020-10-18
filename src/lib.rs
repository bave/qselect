
// lomuto partitioning
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

//sample code of lomuto partitioning
fn lomuto<T>(array: &mut [T], left: usize, right: usize) -> usize
where T: PartialOrd
{
    let mut i = left;
    for j in left .. (right-1) {
        if &array[j] <= &array[right] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, right);
    return i;
}

//sample code of hoare partitioning
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
        let mut v : Vec<u64> = vec![10, 1, 9, 2, 8, 3, 7, 4, 6, 5];
        print!("vec: {:?}\n", v);
        let len = v.len();
        let i = hoare(&mut v, 0, len-1);
        print!("vec: {:?}, i:{}\n", v, i);
    }

    #[test]
    fn test_lomuto() {
        let mut v : Vec<u64> = vec![10, 1, 9, 2, 8, 3, 7, 4, 6, 5];
        print!("vec: {:?}\n", v);
        let len = v.len();
        let i = lomuto(&mut v, 0, len-1);
        print!("vec: {:?}, i:{}\n", v, i);
    }

    #[test]
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

        let b : Vec<f64> = vec![10.0,90.0,20.0,80.0,30.0,70.0,40.0,60.0,50.0];
        for i in 0..10 {
            let mut v : Vec<usize> = vec![0,1,2,3,4,5,6,7,8];
            let len = v.len();
            qselect_indirect(&mut v, 0, len-1, i, &|x| b[x]);
            print!("vec :{:?}\n", v);
        }
        print!("base: {:?}\n", b);
    }
}

