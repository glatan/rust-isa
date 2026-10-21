//! NEON byte shuffles.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_zip1_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uzp1_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_trn1_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_zip2_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uzp2_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_trn2_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_tbx(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `zip1`
///
/// Interleave the low halves of the two byte vectors.
#[inline(always)]
pub fn zip1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_zip1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uzp1`
///
/// Take the even bytes from the two vectors and concatenate them.
#[inline(always)]
pub fn uzp1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uzp1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `trn1`
///
/// Transpose the even bytes of the two vectors.
#[inline(always)]
pub fn trn1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_trn1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `tbx`
///
/// Look up bytes in `a` with the indices in `idx`. An index outside `0..=15`
/// leaves the corresponding result byte zero.
#[inline(always)]
pub fn tbx(a: V128, idx: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_tbx(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `zip2`
///
/// Interleave the high halves of the two byte vectors.
#[inline(always)]
pub fn zip2(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_zip2_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uzp2`
///
/// Take the odd bytes from the two vectors and concatenate them.
#[inline(always)]
pub fn uzp2(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uzp2_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `trn2`
///
/// Transpose the odd bytes of the two vectors.
#[inline(always)]
pub fn trn2(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_trn2_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
