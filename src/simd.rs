
//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]

// compile command
// RUSTFLAGS="-C target-feature=+avx2,+popcnt,+bmi2" cargo run --release

#[cfg(target_arch = "x86_64")]
#[cfg(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2"))]
use std::arch::x86_64::*;

#[cfg(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2"))]
const S : usize = 8;

#[inline]
fn part(array: &mut [f32], left: usize, right: usize) -> usize
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

#[inline]
fn part_indirect<F>(array: &mut [i32], left: usize, right: usize, f:&F) -> usize
where F: Fn(usize) -> f32
{
    let mut i = left;
    for j in left..right {
        if f(array[j] as usize) <= f(array[right] as usize) {
            array.swap(i, j);
            i += 1;
        }
    };
    array.swap(i, right);
    return i;
}

#[inline]
unsafe fn mm256_mask_compressstoreu_ps(array: *mut f32, mask: u8, src: __m256)
{
    #![cfg(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2"))]

    let convert = _pext_u64(0x0706050403020100, _pdep_u64(mask as u64, 0x0101010101010101)*0xFF);
    let permute = _mm256_cvtepu8_epi32(_mm_cvtsi64_si128(convert as i64));
    let compress = _mm256_permutevar8x32_ps(src, permute);

    let imm8 = _popcnt64(mask as i64);
    let ret = match imm8 {
        //0 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0x00) },
        0 => { return; },
        1 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0x01) },
        2 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0x03) },
        3 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0x07) },
        4 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0x0f) },
        5 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0x1f) },
        6 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0x3f) },
        7 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0x7f) },
        8 => { _mm256_blend_ps(_mm256_loadu_ps(array), compress, 0xff) },
        _ => { assert!(false); _mm256_loadu_ps(array) },
    };
    _mm256_storeu_ps(array, ret);
}

#[inline]
unsafe fn  mm256_mask_compressstoreu_epi32(array: *mut i32, mask: u8, src: __m256i)
{
    #![cfg(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2"))]

    let convert = _pext_u64(0x0706050403020100, _pdep_u64(mask as u64, 0x0101010101010101)*0xFF);
    let permute = _mm256_cvtepu8_epi32(_mm_cvtsi64_si128(convert as i64));
    let compress = _mm256_permutevar8x32_epi32(src, permute);

    let imm8 = _popcnt64(mask as i64);
    let ret = match imm8 {
        //0 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0x00) },
        0 => { return; },
        1 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0x01) },
        2 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0x03) },
        3 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0x07) },
        4 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0x0f) },
        5 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0x1f) },
        6 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0x3f) },
        7 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0x7f) },
        8 => { _mm256_blend_epi32(_mm256_loadu_si256(array as *mut __m256i), compress, 0xff) },
        _ => { assert!(false); _mm256_loadu_si256(array as *mut __m256i) },
    };
    _mm256_storeu_si256(array as *mut __m256i, ret);
}

#[inline]
unsafe fn _mm256_store_ps(array: &mut[f32], lw: usize, rw: usize, mask: i32, val: __m256)
-> (usize, usize)
{
    #![cfg(all(target_feature="popcnt",target_feature="avx2"))]

    let low = _popcnt64(mask as i64) as usize;
    let high = S - low;

    //#[cfg(all(target_feature="avx512vl",target_feature="avx512f"))]
    //_mm256_mask_compressstoreu_ps(array.as_mut_ptr().add(lw), mask as u8, val);
    //#[cfg(not(all(target_feature="avx512vl",target_feature="avx512f")))]
    mm256_mask_compressstoreu_ps(array.as_mut_ptr().add(lw), mask as u8, val);

    let lw = lw + low;
    let rw = rw - high;

    //#[cfg(all(target_feature="avx512vl",target_feature="avx512f"))]
    //_mm256_mask_compressstoreu_ps(array.as_mut_ptr().add(rw), !mask as u8, val);
    //#[cfg(not(all(target_feature="avx512vl",target_feature="avx512f")))]
    mm256_mask_compressstoreu_ps(array.as_mut_ptr().add(rw), !mask as u8, val);

    (lw, rw)
}

#[inline]
unsafe fn _mm256_store_ps_remaining(array: &mut[f32], lw: usize, rw: usize, mask: i32, val: __m256, remaining: usize)
-> (usize, usize)
{
    #![cfg(all(target_feature="popcnt",target_feature="avx2"))]

    let mask_low = (mask & !(0xFF << remaining)) as u8; 
    let low = _popcnt64(mask_low as i64) as usize;
    let mask_high = (!mask & !(0xFF << remaining)) as u8;
    let high = _popcnt64(mask_high as i64) as usize;

    //#[cfg(all(target_feature="avx512vl",target_feature="avx512f"))]
    //_mm256_mask_compressstoreu_ps(array.as_mut_ptr().add(lw), mask_low as u8, val);
    //#[cfg(not(all(target_feature="avx512vl",target_feature="avx512f")))]
    mm256_mask_compressstoreu_ps(array.as_mut_ptr().add(lw), mask_low as u8, val);

    let lw = lw + low;
    let rw = rw - high;

    //#[cfg(all(target_feature="avx512vl",target_feature="avx512f"))]
    //_mm256_mask_compressstoreu_ps(array.as_mut_ptr().add(rw), mask_high as u8, val);
    //#[cfg(not(all(target_feature="avx512vl",target_feature="avx512f")))]
    mm256_mask_compressstoreu_ps(array.as_mut_ptr().add(rw), mask_high as u8, val);

    (lw, rw)
}

#[inline]
unsafe fn spart(array: &mut [f32], l: usize, r: usize) -> usize
{
    #![cfg(target_feature="avx2")]

    let pivot = array[r];
    let mut left = l;
    let mut right = r;

    if right-left+1 <= S<<1 {
        return part(array, left, right);
    }

    let pivotvec = _mm256_set1_ps(pivot);

    let left_val : __m256 = _mm256_loadu_ps(array.as_ptr().add(left) as *const f32);
    let mut left_idx = left;
    left += S;

    let mut right_idx= right+1;
    right -= S-1;
    let right_val : __m256 = _mm256_loadu_ps(array.as_ptr().add(right) as *const f32);

    while left + S <= right {
        let free_left = left - left_idx;
        let free_right = right_idx - right;

        let val = if free_left <= free_right {
            let tmp : __m256 = _mm256_loadu_ps(array.as_ptr().add(left) as *const f32);
            left += S;
            tmp
        } else {
            right -= S;
            let tmp : __m256 = _mm256_loadu_ps(array.as_ptr().add(right) as *const f32);
            tmp
        };
        let mask = _mm256_movemask_ps(_mm256_cmp_ps(val, pivotvec, _CMP_LE_OQ));
        let (lw, rw) = _mm256_store_ps(array, left_idx, right_idx, mask, val);
        left_idx = lw;
        right_idx = rw;
    }
    {
        let remaining = right - left;
        let val : __m256 = _mm256_loadu_ps(array.as_ptr().add(left) as *const f32);
        let mask = _mm256_movemask_ps(_mm256_cmp_ps(val, pivotvec, _CMP_LE_OQ));
        let (lw, rw) = _mm256_store_ps_remaining(array, left_idx, right_idx, mask, val, remaining);

        let mask_l = _mm256_movemask_ps(_mm256_cmp_ps(left_val, pivotvec, _CMP_LE_OQ));
        let (lw, rw) = _mm256_store_ps(array, lw, rw, mask_l, left_val);

        let mask_r = _mm256_movemask_ps(_mm256_cmp_ps(right_val, pivotvec, _CMP_LE_OQ));
        let (lw, _) = _mm256_store_ps(array, lw, rw, mask_r, right_val);

        left_idx = lw;
    }
    left_idx - 1
}

#[cfg(target_arch = "x86_64")]
pub fn qselect(array: &mut [f32], left: usize, right: usize, nth: usize)
{
    if left < right {
        #[cfg(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2"))]
        let i = unsafe { spart(array, left, right) };
        #[cfg(not(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2")))]
        let i = part(array, left, right);

        if nth < i {
            qselect(array, left, i-1, nth);
        } else {
            qselect(array, i+1, right, nth);
        }
    }
}

#[inline]
unsafe fn _mm256_store_epi32(array: &mut[i32], lw: usize, rw: usize, mask: i32, val: __m256i)
-> (usize, usize)
{
    #![cfg(all(target_feature="popcnt",target_feature="avx2"))]

    let low = _popcnt64(mask as i64) as usize;
    let high = S - low;
    mm256_mask_compressstoreu_epi32(array.as_mut_ptr().add(lw), mask as u8, val);
    let lw = lw + low;
    let rw = rw - high;
    mm256_mask_compressstoreu_epi32(array.as_mut_ptr().add(rw), !mask as u8, val);
    (lw, rw)
}

#[inline]
unsafe fn _mm256_store_epi32_remaining(array: &mut[i32], lw: usize, rw: usize, mask: i32, val: __m256i, remaining: usize)
-> (usize, usize)
{
    #![cfg(all(target_feature="popcnt",target_feature="avx2"))]

    let mask_low = (mask & !(0xFF << remaining)) as u8; 
    let low = _popcnt64(mask_low as i64) as usize;
    let mask_high = (!mask & !(0xFF << remaining)) as u8;
    let high = _popcnt64(mask_high as i64) as usize;
    mm256_mask_compressstoreu_epi32(array.as_mut_ptr().add(lw), mask_low as u8, val);
    let lw = lw + low;
    let rw = rw - high;
    mm256_mask_compressstoreu_epi32(array.as_mut_ptr().add(rw), mask_high as u8, val);
    (lw, rw)
}

#[inline]
pub unsafe fn load_indirect<F>(val: __m256i, f: &F) -> __m256
where F: Fn(usize) -> f32
{
    #![cfg(target_feature="avx2")]
    /*
    print!("[{}, {}, {}, {}, {}, {}, {}, {}]\n", 
           _mm256_extract_epi32(val, 0),
           _mm256_extract_epi32(val, 1),
           _mm256_extract_epi32(val, 2),
           _mm256_extract_epi32(val, 3),
           _mm256_extract_epi32(val, 4),
           _mm256_extract_epi32(val, 5),
           _mm256_extract_epi32(val, 6),
           _mm256_extract_epi32(val, 7),
           );
    */
    let e0 = f(_mm256_extract_epi32(val, 0) as usize);
    let e1 = f(_mm256_extract_epi32(val, 1) as usize);
    let e2 = f(_mm256_extract_epi32(val, 2) as usize);
    let e3 = f(_mm256_extract_epi32(val, 3) as usize);
    let e4 = f(_mm256_extract_epi32(val, 4) as usize);
    let e5 = f(_mm256_extract_epi32(val, 5) as usize);
    let e6 = f(_mm256_extract_epi32(val, 6) as usize);
    let e7 = f(_mm256_extract_epi32(val, 7) as usize);
    _mm256_set_ps(e7, e6, e5, e4, e3, e2, e1, e0)
}

#[inline]
unsafe fn spart_indirect<F>(array: &mut [i32], l: usize, r: usize, f:&F) -> usize
where F: Fn(usize) -> f32
{
    #![cfg(target_feature="avx2")]

    let pivot = f(array[r] as usize);
    let mut left = l;
    let mut right = r;

    if right-left+1 <= S<<1 {
        return part_indirect(array, left, right, f);
    }

    let pivotvec = _mm256_set1_ps(pivot);

    let left_val : __m256i = _mm256_loadu_si256(array.as_ptr().add(left) as *const __m256i);
    let mut left_idx = left;
    left += S;

    let mut right_idx= right+1;
    right -= S-1;
    let right_val : __m256i = _mm256_loadu_si256(array.as_ptr().add(right) as *const __m256i);

    while left + S <= right {
        let free_left = left - left_idx;
        let free_right = right_idx - right;

        let val = if free_left <= free_right {
            let tmp : __m256i = _mm256_loadu_si256(array.as_ptr().add(left) as *const __m256i);
            left += S;
            tmp
        } else {
            right -= S;
            let tmp : __m256i = _mm256_loadu_si256(array.as_ptr().add(right) as *const __m256i);
            tmp
        };
        let mask = _mm256_movemask_ps(_mm256_cmp_ps(load_indirect(val, f), pivotvec, _CMP_LE_OQ));

        let (lw, rw) = _mm256_store_epi32(array, left_idx, right_idx, mask, val);
        left_idx = lw;
        right_idx = rw;
    }
    {
        let remaining = right - left;
        let val : __m256i = _mm256_loadu_si256(array.as_ptr().add(left) as *const __m256i);
        let mask = _mm256_movemask_ps(_mm256_cmp_ps(load_indirect(val, f), pivotvec, _CMP_LE_OQ));
        let (lw, rw) = _mm256_store_epi32_remaining(array, left_idx, right_idx, mask, val, remaining);
        let mask_l = _mm256_movemask_ps(_mm256_cmp_ps(load_indirect(left_val, f), pivotvec, _CMP_LE_OQ));
        let (lw, rw) = _mm256_store_epi32(array, lw, rw, mask_l, left_val);
        let mask_r = _mm256_movemask_ps(_mm256_cmp_ps(load_indirect(right_val, f), pivotvec, _CMP_LE_OQ));
        let (lw, _) = _mm256_store_epi32(array, lw, rw, mask_r, right_val);

        left_idx = lw;
    }
    left_idx - 1
}

#[cfg(target_arch = "x86_64")]
pub fn qselect_indirect<F>(array: &mut [i32], left: usize, right: usize, nth: usize, key: &F)
where F: Fn(usize) -> f32
{
    if left < right {
        #[cfg(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2"))]
        let i = unsafe { spart_indirect(array, left, right, key) };
        #[cfg(not(all(target_feature="popcnt",target_feature="avx2",target_feature="bmi2")))]
        let i = part_indirect(array, left, right, key);
        if nth < i {
            qselect_indirect(array, left, i-1, nth, key);
        } else {
            qselect_indirect(array, i+1, right, nth, key);
        }
    }
}

