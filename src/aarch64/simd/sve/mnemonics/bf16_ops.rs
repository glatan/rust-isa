//! SVE bf16 dot and matrix multiply instructions.

unsafe extern "C" {
    fn rust_isa_sve_bfdot_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `bfdot` on `f32` accumulators and bf16 inputs.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE2 with bf16 and the CPU must support it.
#[target_feature(enable = "sve2,bf16")]
#[inline]
pub unsafe fn bfdot_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE bf16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE bf16 input is too short");
    unsafe {
        rust_isa_sve_bfdot_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_bfmlalb_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `bfmlalb` on `f32` accumulators and bf16 inputs.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE2 with bf16 and the CPU must support it.
#[target_feature(enable = "sve2,bf16")]
#[inline]
pub unsafe fn bfmlalb_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE bf16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE bf16 input is too short");
    unsafe {
        rust_isa_sve_bfmlalb_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_bfmlalt_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `bfmlalt` on `f32` accumulators and bf16 inputs.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE2 with bf16 and the CPU must support it.
#[target_feature(enable = "sve2,bf16")]
#[inline]
pub unsafe fn bfmlalt_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE bf16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE bf16 input is too short");
    unsafe {
        rust_isa_sve_bfmlalt_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_bfmmla_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `bfmmla` on `f32` accumulators and bf16 inputs.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE2 with bf16 and the CPU must support it.
#[target_feature(enable = "sve2,bf16")]
#[inline]
pub unsafe fn bfmmla_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE bf16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE bf16 input is too short");
    unsafe {
        rust_isa_sve_bfmmla_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
