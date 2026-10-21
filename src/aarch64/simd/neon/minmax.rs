//! NEON min/max and multiply-accumulate instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_smax_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smax_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smax_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smin_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smin_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smin_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umax_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umax_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umax_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umin_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umin_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umin_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smaxp_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smaxp_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smaxp_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sminp_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sminp_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sminp_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umaxp_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umaxp_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umaxp_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uminp_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uminp_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uminp_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_mla_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mla_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mla_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mls_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mls_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_mls_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `smax_s8`
///
/// Take the signed maximum of each lane.
#[inline(always)]
pub fn smax_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smax_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `smax_s16`
///
/// Take the signed maximum of each lane.
#[inline(always)]
pub fn smax_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smax_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `smax_s32`
///
/// Take the signed maximum of each lane.
#[inline(always)]
pub fn smax_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smax_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `smin_s8`
///
/// Take the signed minimum of each lane.
#[inline(always)]
pub fn smin_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smin_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `smin_s16`
///
/// Take the signed minimum of each lane.
#[inline(always)]
pub fn smin_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smin_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `smin_s32`
///
/// Take the signed minimum of each lane.
#[inline(always)]
pub fn smin_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smin_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umax_u8`
///
/// Take the unsigned maximum of each lane.
#[inline(always)]
pub fn umax_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umax_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umax_u16`
///
/// Take the unsigned maximum of each lane.
#[inline(always)]
pub fn umax_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umax_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umax_u32`
///
/// Take the unsigned maximum of each lane.
#[inline(always)]
pub fn umax_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umax_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umin_u8`
///
/// Take the unsigned minimum of each lane.
#[inline(always)]
pub fn umin_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umin_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umin_u16`
///
/// Take the unsigned minimum of each lane.
#[inline(always)]
pub fn umin_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umin_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umin_u32`
///
/// Take the unsigned minimum of each lane.
#[inline(always)]
pub fn umin_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umin_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `smaxp_s8`
///
/// Take the signed maximum of adjacent lane pairs.
#[inline(always)]
pub fn smaxp_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smaxp_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `smaxp_s16`
///
/// Take the signed maximum of adjacent lane pairs.
#[inline(always)]
pub fn smaxp_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smaxp_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `smaxp_s32`
///
/// Take the signed maximum of adjacent lane pairs.
#[inline(always)]
pub fn smaxp_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_smaxp_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `sminp_s8`
///
/// Take the signed minimum of adjacent lane pairs.
#[inline(always)]
pub fn sminp_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_sminp_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `sminp_s16`
///
/// Take the signed minimum of adjacent lane pairs.
#[inline(always)]
pub fn sminp_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_sminp_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `sminp_s32`
///
/// Take the signed minimum of adjacent lane pairs.
#[inline(always)]
pub fn sminp_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_sminp_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umaxp_u8`
///
/// Take the unsigned maximum of adjacent lane pairs.
#[inline(always)]
pub fn umaxp_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umaxp_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umaxp_u16`
///
/// Take the unsigned maximum of adjacent lane pairs.
#[inline(always)]
pub fn umaxp_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umaxp_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `umaxp_u32`
///
/// Take the unsigned maximum of adjacent lane pairs.
#[inline(always)]
pub fn umaxp_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_umaxp_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uminp_u8`
///
/// Take the unsigned minimum of adjacent lane pairs.
#[inline(always)]
pub fn uminp_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uminp_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uminp_u16`
///
/// Take the unsigned minimum of adjacent lane pairs.
#[inline(always)]
pub fn uminp_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uminp_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uminp_u32`
///
/// Take the unsigned minimum of adjacent lane pairs.
#[inline(always)]
pub fn uminp_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uminp_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mla_s8`
///
/// Multiply signed lanes and add the result to the accumulator.
#[inline(always)]
pub fn mla_s8(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mla_s8(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mla_s16`
///
/// Multiply signed lanes and add the result to the accumulator.
#[inline(always)]
pub fn mla_s16(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mla_s16(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mla_s32`
///
/// Multiply signed lanes and add the result to the accumulator.
#[inline(always)]
pub fn mla_s32(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mla_s32(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mls_s8`
///
/// Multiply signed lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn mls_s8(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mls_s8(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mls_s16`
///
/// Multiply signed lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn mls_s16(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mls_s16(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mls_s32`
///
/// Multiply signed lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn mls_s32(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mls_s32(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}
