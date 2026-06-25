// Dry users will hate me :)
#![allow(unsafe_op_in_unsafe_fn)] // Make the compiler shut up :)
use std::arch::x86_64::*;

pub fn add_f32_slices(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { add_f32_sse2(a, b, out) };
            return;
        }
    }
    add_f32_scalar(a, b, out);
}

#[inline]
fn add_f32_scalar(a: &[f32], b: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn add_f32_sse2(a: &[f32], b: &[f32], out: &mut [f32]) {
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        let vb = _mm_loadu_ps(b.as_ptr().add(off));
        _mm_storeu_ps(out.as_mut_ptr().add(off), _mm_add_ps(va, vb));
    }
    let rem = chunks * 4;
    add_f32_scalar(&a[rem..], &b[rem..], &mut out[rem..]);
}

pub fn sub_f32_slices(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { sub_f32_sse2(a, b, out) };
            return;
        }
    }
    sub_f32_scalar(a, b, out);
}

#[inline]
fn sub_f32_scalar(a: &[f32], b: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = a[i] - b[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn sub_f32_sse2(a: &[f32], b: &[f32], out: &mut [f32]) {
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        let vb = _mm_loadu_ps(b.as_ptr().add(off));
        _mm_storeu_ps(out.as_mut_ptr().add(off), _mm_sub_ps(va, vb));
    }
    let rem = chunks * 4;
    sub_f32_scalar(&a[rem..], &b[rem..], &mut out[rem..]);
}

pub fn mul_f32_slices(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { mul_f32_sse2(a, b, out) };
            return;
        }
    }
    mul_f32_scalar(a, b, out);
}

#[inline]
fn mul_f32_scalar(a: &[f32], b: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = a[i] * b[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn mul_f32_sse2(a: &[f32], b: &[f32], out: &mut [f32]) {
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        let vb = _mm_loadu_ps(b.as_ptr().add(off));
        _mm_storeu_ps(out.as_mut_ptr().add(off), _mm_mul_ps(va, vb));
    }
    let rem = chunks * 4;
    mul_f32_scalar(&a[rem..], &b[rem..], &mut out[rem..]);
}

pub fn div_f32_slices(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { div_f32_sse2(a, b, out) };
            return;
        }
    }
    div_f32_scalar(a, b, out);
}

#[inline]
fn div_f32_scalar(a: &[f32], b: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = a[i] / b[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn div_f32_sse2(a: &[f32], b: &[f32], out: &mut [f32]) {
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        let vb = _mm_loadu_ps(b.as_ptr().add(off));
        _mm_storeu_ps(out.as_mut_ptr().add(off), _mm_div_ps(va, vb));
    }
    let rem = chunks * 4;
    div_f32_scalar(&a[rem..], &b[rem..], &mut out[rem..]);
}

pub fn add_assign_f32_slices(a: &mut [f32], b: &[f32]) {
    assert_eq!(a.len(), b.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { add_assign_f32_sse2(a, b) };
            return;
        }
    }
    add_assign_f32_scalar(a, b);
}

#[inline]
fn add_assign_f32_scalar(a: &mut [f32], b: &[f32]) {
    for i in 0..a.len() {
        a[i] += b[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn add_assign_f32_sse2(a: &mut [f32], b: &[f32]) {
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        let vb = _mm_loadu_ps(b.as_ptr().add(off));
        _mm_storeu_ps(a.as_mut_ptr().add(off), _mm_add_ps(va, vb));
    }
    let rem = chunks * 4;
    add_assign_f32_scalar(&mut a[rem..], &b[rem..]);
}

pub fn sub_assign_f32_slices(a: &mut [f32], b: &[f32]) {
    assert_eq!(a.len(), b.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { sub_assign_f32_sse2(a, b) };
            return;
        }
    }
    sub_assign_f32_scalar(a, b);
}

#[inline]
fn sub_assign_f32_scalar(a: &mut [f32], b: &[f32]) {
    for i in 0..a.len() {
        a[i] -= b[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn sub_assign_f32_sse2(a: &mut [f32], b: &[f32]) {
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        let vb = _mm_loadu_ps(b.as_ptr().add(off));
        _mm_storeu_ps(a.as_mut_ptr().add(off), _mm_sub_ps(va, vb));
    }
    let rem = chunks * 4;
    sub_assign_f32_scalar(&mut a[rem..], &b[rem..]);
}

pub fn mul_assign_f32_slices(a: &mut [f32], b: &[f32]) {
    assert_eq!(a.len(), b.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { mul_assign_f32_sse2(a, b) };
            return;
        }
    }
    mul_assign_f32_scalar(a, b);
}

#[inline]
fn mul_assign_f32_scalar(a: &mut [f32], b: &[f32]) {
    for i in 0..a.len() {
        a[i] *= b[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn mul_assign_f32_sse2(a: &mut [f32], b: &[f32]) {
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        let vb = _mm_loadu_ps(b.as_ptr().add(off));
        _mm_storeu_ps(a.as_mut_ptr().add(off), _mm_mul_ps(va, vb));
    }
    let rem = chunks * 4;
    mul_assign_f32_scalar(&mut a[rem..], &b[rem..]);
}

pub fn div_assign_f32_slices(a: &mut [f32], b: &[f32]) {
    assert_eq!(a.len(), b.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { div_assign_f32_sse2(a, b) };
            return;
        }
    }
    div_assign_f32_scalar(a, b);
}

#[inline]
fn div_assign_f32_scalar(a: &mut [f32], b: &[f32]) {
    for i in 0..a.len() {
        a[i] /= b[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn div_assign_f32_sse2(a: &mut [f32], b: &[f32]) {
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        let vb = _mm_loadu_ps(b.as_ptr().add(off));
        _mm_storeu_ps(a.as_mut_ptr().add(off), _mm_div_ps(va, vb));
    }
    let rem = chunks * 4;
    div_assign_f32_scalar(&mut a[rem..], &b[rem..]);
}

pub fn neg_f32_slice(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { neg_f32_sse2(a, out) };
            return;
        }
    }
    neg_f32_scalar(a, out);
}

#[inline]
fn neg_f32_scalar(a: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = -a[i];
    }
}

#[target_feature(enable = "sse2")]
unsafe fn neg_f32_sse2(a: &[f32], out: &mut [f32]) {
    let sign_mask = _mm_set1_ps(-0.0f32);
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        _mm_storeu_ps(out.as_mut_ptr().add(off), _mm_xor_ps(va, sign_mask));
    }
    let rem = chunks * 4;
    neg_f32_scalar(&a[rem..], &mut out[rem..]);
}

pub fn mul_f32_slice_scalar(a: &[f32], b: f32, out: &mut [f32]) {
    assert_eq!(a.len(), out.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { mul_f32_scalar_sse2(a, b, out) };
            return;
        }
    }
    for i in 0..a.len() { out[i] = a[i] * b; }
}

#[target_feature(enable = "sse2")]
unsafe fn mul_f32_scalar_sse2(a: &[f32], b: f32, out: &mut [f32]) {
    let vb = _mm_set1_ps(b);
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        _mm_storeu_ps(out.as_mut_ptr().add(off), _mm_mul_ps(va, vb));
    }
    let rem = chunks * 4;
    for i in rem..a.len() { out[i] = a[i] * b; }
}

pub fn div_f32_slice_scalar(a: &[f32], b: f32, out: &mut [f32]) {
    assert_eq!(a.len(), out.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { div_f32_scalar_sse2(a, b, out) };
            return;
        }
    }
    for i in 0..a.len() { out[i] = a[i] / b; }
}

#[target_feature(enable = "sse2")]
unsafe fn div_f32_scalar_sse2(a: &[f32], b: f32, out: &mut [f32]) {
    let vb = _mm_set1_ps(b);
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        _mm_storeu_ps(out.as_mut_ptr().add(off), _mm_div_ps(va, vb));
    }
    let rem = chunks * 4;
    for i in rem..a.len() { out[i] = a[i] / b; }
}

pub fn mul_assign_f32_slice_scalar(a: &mut [f32], b: f32) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { mul_assign_f32_scalar_sse2(a, b) };
            return;
        }
    }
    for x in a.iter_mut() { *x *= b; }
}

#[target_feature(enable = "sse2")]
unsafe fn mul_assign_f32_scalar_sse2(a: &mut [f32], b: f32) {
    let vb = _mm_set1_ps(b);
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        _mm_storeu_ps(a.as_mut_ptr().add(off), _mm_mul_ps(va, vb));
    }
    let rem = chunks * 4;
    for x in a[rem..].iter_mut() { *x *= b; }
}

pub fn div_assign_f32_slice_scalar(a: &mut [f32], b: f32) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { div_assign_f32_scalar_sse2(a, b) };
            return;
        }
    }
    for x in a.iter_mut() { *x /= b; }
}

#[target_feature(enable = "sse2")]
unsafe fn div_assign_f32_scalar_sse2(a: &mut [f32], b: f32) {
    let vb = _mm_set1_ps(b);
    let chunks = a.len() / 4;
    for i in 0..chunks {
        let off = i * 4;
        let va = _mm_loadu_ps(a.as_ptr().add(off));
        _mm_storeu_ps(a.as_mut_ptr().add(off), _mm_div_ps(va, vb));
    }
    let rem = chunks * 4;
    for x in a[rem..].iter_mut() { *x /= b; }
}

pub fn mat4_mul_f32_slices(a: &[f32], b: &[f32], out: &mut [f32]) {
    // a and b are column-major flat [f32; 16]
    assert_eq!(a.len(), 16);
    assert_eq!(b.len(), 16);
    assert_eq!(out.len(), 16);
    for c in 0..4 {
        for r in 0..4 {
            let mut sum = 0.0f32;
            for k in 0..4 {
                sum += a[k * 4 + r] * b[c * 4 + k];
            }
            out[c * 4 + r] = sum;
        }
    }
}
