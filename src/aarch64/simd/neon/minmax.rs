//! NEON min/max instructions.

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

// Historical names kept as aliases for the concrete instructions.

/// Signed minimum on 8-bit lanes. Alias for [`smin_s8`].
#[inline(always)]
pub fn min_i8(a: V128, b: V128) -> V128 {
    smin_s8(a, b)
}

/// Signed maximum on 8-bit lanes. Alias for [`smax_s8`].
#[inline(always)]
pub fn max_i8(a: V128, b: V128) -> V128 {
    smax_s8(a, b)
}

/// Signed minimum on 16-bit lanes. Alias for [`smin_s16`].
#[inline(always)]
pub fn min_i16(a: V128, b: V128) -> V128 {
    smin_s16(a, b)
}

/// Signed maximum on 16-bit lanes. Alias for [`smax_s16`].
#[inline(always)]
pub fn max_i16(a: V128, b: V128) -> V128 {
    smax_s16(a, b)
}

/// Signed minimum on 32-bit lanes. Alias for [`smin_s32`].
#[inline(always)]
pub fn min_i32(a: V128, b: V128) -> V128 {
    smin_s32(a, b)
}

/// Signed maximum on 32-bit lanes. Alias for [`smax_s32`].
#[inline(always)]
pub fn max_i32(a: V128, b: V128) -> V128 {
    smax_s32(a, b)
}
