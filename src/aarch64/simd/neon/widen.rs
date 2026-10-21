//! NEON widening add/subtract instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_saddl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddw_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddw_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddw_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddw_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddw_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddw_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubw_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubw_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubw_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubw_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubw_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubw_u32(a: *const u8, b: *const u8, out: *mut u8);
}

/// `saddl_s8`
///
/// Add the low half of signed lanes and widen the result.
#[inline(always)]
pub fn saddl_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddl_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddl_s16`
///
/// Add the low half of signed lanes and widen the result.
#[inline(always)]
pub fn saddl_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddl_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddl_s32`
///
/// Add the low half of signed lanes and widen the result.
#[inline(always)]
pub fn saddl_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddl_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddl_u8`
///
/// Add the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn uaddl_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddl_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddl_u16`
///
/// Add the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn uaddl_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddl_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddl_u32`
///
/// Add the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn uaddl_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddl_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubl_s8`
///
/// Subtract the low half of signed lanes and widen the result.
#[inline(always)]
pub fn ssubl_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubl_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubl_s16`
///
/// Subtract the low half of signed lanes and widen the result.
#[inline(always)]
pub fn ssubl_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubl_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubl_s32`
///
/// Subtract the low half of signed lanes and widen the result.
#[inline(always)]
pub fn ssubl_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubl_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubl_u8`
///
/// Subtract the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn usubl_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubl_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubl_u16`
///
/// Subtract the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn usubl_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubl_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubl_u32`
///
/// Subtract the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn usubl_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubl_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddw_s8`
///
/// Add the low half of signed lanes to widened signed lanes.
#[inline(always)]
pub fn saddw_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddw_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddw_s16`
///
/// Add the low half of signed lanes to widened signed lanes.
#[inline(always)]
pub fn saddw_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddw_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddw_s32`
///
/// Add the low half of signed lanes to widened signed lanes.
#[inline(always)]
pub fn saddw_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddw_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddw_u8`
///
/// Add the low half of unsigned lanes to widened unsigned lanes.
#[inline(always)]
pub fn uaddw_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddw_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddw_u16`
///
/// Add the low half of unsigned lanes to widened unsigned lanes.
#[inline(always)]
pub fn uaddw_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddw_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddw_u32`
///
/// Add the low half of unsigned lanes to widened unsigned lanes.
#[inline(always)]
pub fn uaddw_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddw_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubw_s8`
///
/// Subtract the low half of signed lanes from widened signed lanes.
#[inline(always)]
pub fn ssubw_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubw_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubw_s16`
///
/// Subtract the low half of signed lanes from widened signed lanes.
#[inline(always)]
pub fn ssubw_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubw_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubw_s32`
///
/// Subtract the low half of signed lanes from widened signed lanes.
#[inline(always)]
pub fn ssubw_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubw_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubw_u8`
///
/// Subtract the low half of unsigned lanes from widened unsigned lanes.
#[inline(always)]
pub fn usubw_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubw_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubw_u16`
///
/// Subtract the low half of unsigned lanes from widened unsigned lanes.
#[inline(always)]
pub fn usubw_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubw_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubw_u32`
///
/// Subtract the low half of unsigned lanes from widened unsigned lanes.
#[inline(always)]
pub fn usubw_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubw_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
