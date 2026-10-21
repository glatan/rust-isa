//! NEON pairwise and reduction instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_addp_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addp_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addp_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addv_s8(a: *const u8) -> i8;
    fn rust_isa_neon_addv_s16(a: *const u8) -> i16;
    fn rust_isa_neon_addv_s32(a: *const u8) -> i32;
    fn rust_isa_neon_smaxv_s8(a: *const u8) -> i8;
    fn rust_isa_neon_smaxv_s16(a: *const u8) -> i16;
    fn rust_isa_neon_smaxv_s32(a: *const u8) -> i32;
    fn rust_isa_neon_sminv_s8(a: *const u8) -> i8;
    fn rust_isa_neon_sminv_s16(a: *const u8) -> i16;
    fn rust_isa_neon_sminv_s32(a: *const u8) -> i32;
    fn rust_isa_neon_umaxv_u8(a: *const u8) -> u8;
    fn rust_isa_neon_umaxv_u16(a: *const u8) -> u16;
    fn rust_isa_neon_umaxv_u32(a: *const u8) -> u32;
    fn rust_isa_neon_uminv_u8(a: *const u8) -> u8;
    fn rust_isa_neon_uminv_u16(a: *const u8) -> u16;
    fn rust_isa_neon_uminv_u32(a: *const u8) -> u32;
}

/// `addp_s8`
///
/// Add adjacent 8-bit lane pairs.
#[inline(always)]
pub fn addp_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_addp_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `addp_s16`
///
/// Add adjacent 16-bit lane pairs.
#[inline(always)]
pub fn addp_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_addp_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `addp_s32`
///
/// Add adjacent 32-bit lane pairs.
#[inline(always)]
pub fn addp_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_addp_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `addv_s8`
///
/// Add all signed lanes.
#[inline(always)]
pub fn addv_s8(a: V128) -> i8 {
    unsafe { rust_isa_neon_addv_s8(a.as_ptr()) }
}

/// `addv_s16`
///
/// Add all signed lanes.
#[inline(always)]
pub fn addv_s16(a: V128) -> i16 {
    unsafe { rust_isa_neon_addv_s16(a.as_ptr()) }
}

/// `addv_s32`
///
/// Add all signed lanes.
#[inline(always)]
pub fn addv_s32(a: V128) -> i32 {
    unsafe { rust_isa_neon_addv_s32(a.as_ptr()) }
}

/// `smaxv_s8`
///
/// Take the maximum signed lane.
#[inline(always)]
pub fn smaxv_s8(a: V128) -> i8 {
    unsafe { rust_isa_neon_smaxv_s8(a.as_ptr()) }
}

/// `smaxv_s16`
///
/// Take the maximum signed lane.
#[inline(always)]
pub fn smaxv_s16(a: V128) -> i16 {
    unsafe { rust_isa_neon_smaxv_s16(a.as_ptr()) }
}

/// `smaxv_s32`
///
/// Take the maximum signed lane.
#[inline(always)]
pub fn smaxv_s32(a: V128) -> i32 {
    unsafe { rust_isa_neon_smaxv_s32(a.as_ptr()) }
}

/// `sminv_s8`
///
/// Take the minimum signed lane.
#[inline(always)]
pub fn sminv_s8(a: V128) -> i8 {
    unsafe { rust_isa_neon_sminv_s8(a.as_ptr()) }
}

/// `sminv_s16`
///
/// Take the minimum signed lane.
#[inline(always)]
pub fn sminv_s16(a: V128) -> i16 {
    unsafe { rust_isa_neon_sminv_s16(a.as_ptr()) }
}

/// `sminv_s32`
///
/// Take the minimum signed lane.
#[inline(always)]
pub fn sminv_s32(a: V128) -> i32 {
    unsafe { rust_isa_neon_sminv_s32(a.as_ptr()) }
}

/// `umaxv_u8`
///
/// Take the maximum unsigned lane.
#[inline(always)]
pub fn umaxv_u8(a: V128) -> u8 {
    unsafe { rust_isa_neon_umaxv_u8(a.as_ptr()) }
}

/// `umaxv_u16`
///
/// Take the maximum unsigned lane.
#[inline(always)]
pub fn umaxv_u16(a: V128) -> u16 {
    unsafe { rust_isa_neon_umaxv_u16(a.as_ptr()) }
}

/// `umaxv_u32`
///
/// Take the maximum unsigned lane.
#[inline(always)]
pub fn umaxv_u32(a: V128) -> u32 {
    unsafe { rust_isa_neon_umaxv_u32(a.as_ptr()) }
}

/// `uminv_u8`
///
/// Take the minimum unsigned lane.
#[inline(always)]
pub fn uminv_u8(a: V128) -> u8 {
    unsafe { rust_isa_neon_uminv_u8(a.as_ptr()) }
}

/// `uminv_u16`
///
/// Take the minimum unsigned lane.
#[inline(always)]
pub fn uminv_u16(a: V128) -> u16 {
    unsafe { rust_isa_neon_uminv_u16(a.as_ptr()) }
}

/// `uminv_u32`
///
/// Take the minimum unsigned lane.
#[inline(always)]
pub fn uminv_u32(a: V128) -> u32 {
    unsafe { rust_isa_neon_uminv_u32(a.as_ptr()) }
}
