//! SVE byte increment/decrement instructions.

#[inline(always)]
fn count_b() -> i64 {
    let lanes: usize;
    unsafe {
        core::arch::asm!("cntb {0}", out(reg) lanes, options(nomem, nostack));
    }
    lanes as i64
}

/// `incb` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn incb_scalar(value: i8, imm: i8) -> i8 {
    value.wrapping_add(imm.wrapping_mul(count_b() as i8))
}

/// `incb` vector form.
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
pub unsafe fn incb_vector(a: &[i8], imm: i8, out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_b() as i8);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_add(step);
    }
}

/// `decb` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn decb_scalar(value: i8, imm: i8) -> i8 {
    value.wrapping_sub(imm.wrapping_mul(count_b() as i8))
}

/// `decb` vector form.
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
pub unsafe fn decb_vector(a: &[i8], imm: i8, out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_b() as i8);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.wrapping_sub(step);
    }
}

/// `sqincb` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqincb_scalar(value: i8, imm: i8) -> i8 {
    value.saturating_add(imm.wrapping_mul(count_b() as i8))
}

/// `sqincb` vector form.
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
pub unsafe fn sqincb_vector(a: &[i8], imm: i8, out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_b() as i8);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `sqdecb` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqdecb_scalar(value: i8, imm: i8) -> i8 {
    value.saturating_sub(imm.wrapping_mul(count_b() as i8))
}

/// `sqdecb` vector form.
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
pub unsafe fn sqdecb_vector(a: &[i8], imm: i8, out: &mut [i8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_b() as i8);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}

/// `uqincb` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqincb_scalar(value: u8, imm: u8) -> u8 {
    value.saturating_add(imm.wrapping_mul(count_b() as u8))
}

/// `uqincb` vector form.
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
pub unsafe fn uqincb_vector(a: &[u8], imm: u8, out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_b() as u8);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `uqdecb` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqdecb_scalar(value: u8, imm: u8) -> u8 {
    value.saturating_sub(imm.wrapping_mul(count_b() as u8))
}

/// `uqdecb` vector form.
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
pub unsafe fn uqdecb_vector(a: &[u8], imm: u8, out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_b() as u8);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}
