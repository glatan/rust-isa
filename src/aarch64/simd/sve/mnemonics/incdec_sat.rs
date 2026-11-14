//! SVE saturating increment/decrement instructions.

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

/// `sqincd` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqincd_scalar(value: i64, imm: i64) -> i64 {
    value.saturating_add(imm.wrapping_mul(count_d() as i64))
}

/// `sqincd` vector form.
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
pub unsafe fn sqincd_vector(a: &[i64], imm: i64, out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_d() as i64);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `sqdecd` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqdecd_scalar(value: i64, imm: i64) -> i64 {
    value.saturating_sub(imm.wrapping_mul(count_d() as i64))
}

/// `sqdecd` vector form.
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
pub unsafe fn sqdecd_vector(a: &[i64], imm: i64, out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_d() as i64);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}

/// `sqincw` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqincw_scalar(value: i32, imm: i32) -> i32 {
    value.saturating_add(imm.wrapping_mul(count_w() as i32))
}

/// `sqincw` vector form.
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
pub unsafe fn sqincw_vector(a: &[i32], imm: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_w() as i32);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `sqdecw` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqdecw_scalar(value: i32, imm: i32) -> i32 {
    value.saturating_sub(imm.wrapping_mul(count_w() as i32))
}

/// `sqdecw` vector form.
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
pub unsafe fn sqdecw_vector(a: &[i32], imm: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_w() as i32);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}

/// `sqinch` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqinch_scalar(value: i16, imm: i16) -> i16 {
    value.saturating_add(imm.wrapping_mul(count_h() as i16))
}

/// `sqinch` vector form.
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
pub unsafe fn sqinch_vector(a: &[i16], imm: i16, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_h() as i16);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `sqdech` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqdech_scalar(value: i16, imm: i16) -> i16 {
    value.saturating_sub(imm.wrapping_mul(count_h() as i16))
}

/// `sqdech` vector form.
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
pub unsafe fn sqdech_vector(a: &[i16], imm: i16, out: &mut [i16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_h() as i16);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}

/// `uqincd` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqincd_scalar(value: u64, imm: u64) -> u64 {
    value.saturating_add(imm.wrapping_mul(count_d() as u64))
}

/// `uqincd` vector form.
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
pub unsafe fn uqincd_vector(a: &[u64], imm: u64, out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_d() as u64);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `uqdecd` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqdecd_scalar(value: u64, imm: u64) -> u64 {
    value.saturating_sub(imm.wrapping_mul(count_d() as u64))
}

/// `uqdecd` vector form.
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
pub unsafe fn uqdecd_vector(a: &[u64], imm: u64, out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_d() as u64);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}

/// `uqincw` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqincw_scalar(value: u32, imm: u32) -> u32 {
    value.saturating_add(imm.wrapping_mul(count_w() as u32))
}

/// `uqincw` vector form.
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
pub unsafe fn uqincw_vector(a: &[u32], imm: u32, out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_w() as u32);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `uqdecw` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqdecw_scalar(value: u32, imm: u32) -> u32 {
    value.saturating_sub(imm.wrapping_mul(count_w() as u32))
}

/// `uqdecw` vector form.
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
pub unsafe fn uqdecw_vector(a: &[u32], imm: u32, out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_w() as u32);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}

/// `uqinch` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqinch_scalar(value: u16, imm: u16) -> u16 {
    value.saturating_add(imm.wrapping_mul(count_h() as u16))
}

/// `uqinch` vector form.
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
pub unsafe fn uqinch_vector(a: &[u16], imm: u16, out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_h() as u16);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `uqdech` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqdech_scalar(value: u16, imm: u16) -> u16 {
    value.saturating_sub(imm.wrapping_mul(count_h() as u16))
}

/// `uqdech` vector form.
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
pub unsafe fn uqdech_vector(a: &[u16], imm: u16, out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let step = imm.wrapping_mul(count_h() as u16);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}

/// `sqincp` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqincp_scalar(mask: &[u32], value: i64, imm: i64) -> i64 {
    let active = mask.iter().filter(|&&lane| lane != 0).count() as i64;
    value.saturating_add(imm.wrapping_mul(active))
}

/// `sqincp` vector form.
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
pub unsafe fn sqincp_vector(mask: &[u32], a: &[i64], imm: i64, out: &mut [i64]) {
    assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let active = mask.iter().filter(|&&lane| lane != 0).count() as i64;
    let step = imm.wrapping_mul(active);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `sqdecp` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqdecp_scalar(mask: &[u32], value: i64, imm: i64) -> i64 {
    let active = mask.iter().filter(|&&lane| lane != 0).count() as i64;
    value.saturating_sub(imm.wrapping_mul(active))
}

/// `sqdecp` vector form.
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
pub unsafe fn sqdecp_vector(mask: &[u32], a: &[i64], imm: i64, out: &mut [i64]) {
    assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let active = mask.iter().filter(|&&lane| lane != 0).count() as i64;
    let step = imm.wrapping_mul(active);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}

/// `uqincp` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqincp_scalar(mask: &[u32], value: u64, imm: u64) -> u64 {
    let active = mask.iter().filter(|&&lane| lane != 0).count() as u64;
    value.saturating_add(imm.wrapping_mul(active))
}

/// `uqincp` vector form.
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
pub unsafe fn uqincp_vector(mask: &[u32], a: &[u64], imm: u64, out: &mut [u64]) {
    assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let active = mask.iter().filter(|&&lane| lane != 0).count() as u64;
    let step = imm.wrapping_mul(active);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_add(step);
    }
}

/// `uqdecp` scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uqdecp_scalar(mask: &[u32], value: u64, imm: u64) -> u64 {
    let active = mask.iter().filter(|&&lane| lane != 0).count() as u64;
    value.saturating_sub(imm.wrapping_mul(active))
}

/// `uqdecp` vector form.
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
pub unsafe fn uqdecp_vector(mask: &[u32], a: &[u64], imm: u64, out: &mut [u64]) {
    assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    let active = mask.iter().filter(|&&lane| lane != 0).count() as u64;
    let step = imm.wrapping_mul(active);
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = a_elem.saturating_sub(step);
    }
}
