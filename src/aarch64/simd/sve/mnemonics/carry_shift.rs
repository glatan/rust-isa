//! SVE add/subtract with carry and rounding shifts.

unsafe extern "C" {
    fn rust_isa_sve_adclb_u32(
        a: *const u32,
        b: *const u32,
        c: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `adclb`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn adclb_u32(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_adclb_u32(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_adclt_u32(
        a: *const u32,
        b: *const u32,
        c: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `adclt`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn adclt_u32(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_adclt_u32(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sbclb_u32(
        a: *const u32,
        b: *const u32,
        c: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `sbclb`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sbclb_u32(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sbclb_u32(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_sbclt_u32(
        a: *const u32,
        b: *const u32,
        c: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `sbclt`
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sbclt_u32(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sbclt_u32(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

/// `asrd` on `i32` lanes.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn asrd_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert!((0..32).contains(&shift), "shift must be in 0..31");
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = *a_elem >> shift;
    }
}

/// `asrr` on `i32` lanes.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn asrr_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert!((0..32).contains(&shift), "shift must be in 0..31");
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = if shift == 0 {
            *a_elem
        } else {
            (*a_elem >> shift) + ((*a_elem >> (shift - 1)) & 1)
        };
    }
}

/// `lslr` on `i32` lanes.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lslr_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert!((0..32).contains(&shift), "shift must be in 0..31");
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = *a_elem << shift;
    }
}

/// `lsrr` on `i32` lanes.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsrr_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert!((0..32).contains(&shift), "shift must be in 0..31");
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = if shift == 0 {
            *a_elem
        } else {
            ((*a_elem as u32 >> shift) as i32) + (((*a_elem as u32 >> (shift - 1)) & 1) as i32)
        };
    }
}
