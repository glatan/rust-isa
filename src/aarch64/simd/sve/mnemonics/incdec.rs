//! SVE increment/decrement instructions.

#[inline(always)]
fn count_b() -> i64 {
    let lanes: usize;
    unsafe {
        core::arch::asm!("cntb {0}", out(reg) lanes, options(nomem, nostack));
    }
    lanes as i64
}

#[inline(always)]
fn count_h() -> i64 {
    let lanes: usize;
    unsafe {
        core::arch::asm!("cnth {0}", out(reg) lanes, options(nomem, nostack));
    }
    lanes as i64
}

#[inline(always)]
fn count_w() -> i64 {
    let lanes: usize;
    unsafe {
        core::arch::asm!("cntw {0}", out(reg) lanes, options(nomem, nostack));
    }
    lanes as i64
}

#[inline(always)]
fn count_d() -> i64 {
    let lanes: usize;
    unsafe {
        core::arch::asm!("cntd {0}", out(reg) lanes, options(nomem, nostack));
    }
    lanes as i64
}

/// `incd` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn incd_scalar(value: i64, imm: i64) -> i64 {
    value.wrapping_add(imm.wrapping_mul(count_d() as i64))
}

/// `incd` vector form.
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
pub unsafe fn incd_vector(a: &[i64], imm: i64, out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_d() as i64);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_add(step);
    }
}

/// `decd` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn decd_scalar(value: i64, imm: i64) -> i64 {
    value.wrapping_sub(imm.wrapping_mul(count_d() as i64))
}

/// `decd` vector form.
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
pub unsafe fn decd_vector(a: &[i64], imm: i64, out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_d() as i64);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_sub(step);
    }
}

/// `incw` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn incw_scalar(value: i32, imm: i32) -> i32 {
    value.wrapping_add(imm.wrapping_mul(count_w() as i32))
}

/// `incw` vector form.
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
pub unsafe fn incw_vector(a: &[i32], imm: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_w() as i32);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_add(step);
    }
}

/// `decw` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn decw_scalar(value: i32, imm: i32) -> i32 {
    value.wrapping_sub(imm.wrapping_mul(count_w() as i32))
}

/// `decw` vector form.
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
pub unsafe fn decw_vector(a: &[i32], imm: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_w() as i32);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_sub(step);
    }
}

/// `inch` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn inch_scalar(value: i16, imm: i16) -> i16 {
    value.wrapping_add(imm.wrapping_mul(count_h() as i16))
}

/// `inch` vector form.
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
pub unsafe fn inch_vector(a: &[i16], imm: i16, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_h() as i16);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_add(step);
    }
}

/// `dech` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn dech_scalar(value: i16, imm: i16) -> i16 {
    value.wrapping_sub(imm.wrapping_mul(count_h() as i16))
}

/// `dech` vector form.
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
pub unsafe fn dech_vector(a: &[i16], imm: i16, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_h() as i16);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_sub(step);
    }
}

/// `incp` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn incp_scalar(mask: &[u32], value: i64, imm: i64) -> i64 {
    let active = mask.iter().filter(|&&lane| lane != 0).count() as i64;
    value.wrapping_add(imm.wrapping_mul(active))
}

/// `incp` vector form.
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
pub unsafe fn incp_vector(mask: &[u32], a: &[i64], imm: i64, out: &mut [i64]) {
    assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let active = mask.iter().filter(|&&lane| lane != 0).count() as i64;
    let step = imm.wrapping_mul(active);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_add(step);
    }
}

/// `decp` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn decp_scalar(mask: &[u32], value: i64, imm: i64) -> i64 {
    let active = mask.iter().filter(|&&lane| lane != 0).count() as i64;
    value.wrapping_sub(imm.wrapping_mul(active))
}

/// `decp` vector form.
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
pub unsafe fn decp_vector(mask: &[u32], a: &[i64], imm: i64, out: &mut [i64]) {
    assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let active = mask.iter().filter(|&&lane| lane != 0).count() as i64;
    let step = imm.wrapping_mul(active);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_sub(step);
    }
}
