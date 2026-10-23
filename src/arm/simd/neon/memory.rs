//! NEON memory, permute, and miscellaneous operations.
//!
//! `vtbl` and `vtbx` use a 16-byte table and an 8-byte index and result,
//! because AArch32 has no 128-bit table lookup instruction. The other
//! operations use [`V128`].
//!
//! The caller must compile the whole crate with `-C target-feature=+neon`.

use super::{V64, V128};

unsafe extern "C" {
    fn rust_isa_arm_neon_vld1_u8(p: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vld2_u8(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_arm_neon_vld3_u8(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8);
    fn rust_isa_arm_neon_vld4_u8(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8, o3: *mut u8);
    fn rust_isa_arm_neon_vmov_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vpadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vpmax_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vpmin_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrev16_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrev32_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vrev64_u8(a: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vswp_u8(a: *const u8, b: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_arm_neon_vtbl_u8(table: *const u8, idx: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vtbx_u8(table: *const u8, idx: *const u8, src: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_vext_u8(a: *const u8, b: *const u8, n: i32, out: *mut u8);
    fn rust_isa_arm_neon_vshr_s16(a: *const u8, n: i32, out: *mut u8);
}

/// `vld1.8`. Load 16 bytes.
///
/// # Safety
///
/// `p` must be valid for reading 16 bytes, and the caller must compile with
/// NEON enabled.
#[inline(always)]
pub unsafe fn vld1(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vld1_u8(p, out.as_mut_ptr());
    }
    out
}

/// `vld2.8`. Load 32 bytes and deinterleave them into two vectors.
///
/// # Safety
///
/// `p` must be valid for reading 32 bytes, and the caller must compile with
/// NEON enabled.
#[inline(always)]
pub unsafe fn vld2(p: *const u8) -> (V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vld2_u8(p, a.as_mut_ptr(), b.as_mut_ptr());
    }
    (a, b)
}

/// `vld3.8`. Load 48 bytes and deinterleave them into three vectors.
///
/// # Safety
///
/// `p` must be valid for reading 48 bytes, and the caller must compile with
/// NEON enabled.
#[inline(always)]
pub unsafe fn vld3(p: *const u8) -> (V128, V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    let mut c = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vld3_u8(p, a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    }
    (a, b, c)
}

/// `vld4.8`. Load 64 bytes and deinterleave them into four vectors.
///
/// # Safety
///
/// `p` must be valid for reading 64 bytes, and the caller must compile with
/// NEON enabled.
#[inline(always)]
pub unsafe fn vld4(p: *const u8) -> (V128, V128, V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    let mut c = V128::from_bytes([0; 16]);
    let mut d = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vld4_u8(
            p,
            a.as_mut_ptr(),
            b.as_mut_ptr(),
            c.as_mut_ptr(),
            d.as_mut_ptr(),
        );
    }
    (a, b, c, d)
}

/// `vmov`. Copy a 128-bit value.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vmov(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vmov_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpadd.s32`. Add adjacent lane pairs of each operand.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vpadd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vpadd_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmax.s32`. Take the maximum of adjacent lane pairs of each operand.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vpmax(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vpmax_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmin.s32`. Take the minimum of adjacent lane pairs of each operand.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vpmin(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vpmin_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrev16.8`. Reverse the byte pairs in each 16-bit element.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vrev16(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vrev16_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrev32.8`. Reverse the bytes in each 32-bit element.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vrev32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vrev32_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrev64.8`. Reverse the bytes in each 64-bit element.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vrev64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vrev64_u8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vswp`. Swap two 128-bit values.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vswp(a: V128, b: V128) -> (V128, V128) {
    let mut out_a = V128::from_bytes([0; 16]);
    let mut out_b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vswp_u8(
            a.as_ptr(),
            b.as_ptr(),
            out_a.as_mut_ptr(),
            out_b.as_mut_ptr(),
        );
    }
    (out_a, out_b)
}

/// `vtbl.8`. Look up 8 bytes in the 16-byte table `table` with `idx`. An index
/// outside `0..=15` produces zero.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vtbl(table: V128, idx: V64) -> V64 {
    let mut out = V64::from_bytes([0; 8]);
    unsafe {
        rust_isa_arm_neon_vtbl_u8(table.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vtbx.8`. Look up 8 bytes in the 16-byte table `table` with `idx`, keeping
/// the bytes of `src` where the index is outside `0..=15`.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vtbx(table: V128, idx: V64, src: V64) -> V64 {
    let mut out = V64::from_bytes([0; 8]);
    unsafe {
        rust_isa_arm_neon_vtbx_u8(table.as_ptr(), idx.as_ptr(), src.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext.8`. Extract 16 bytes starting at byte `N` of the concatenation of `a`
/// and `b`. `N` must be in `0..=15`.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vext<const N: u32>(a: V128, b: V128) -> V128 {
    const { assert!(N <= 15, "extract immediate must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vext_u8(a.as_ptr(), b.as_ptr(), N as i32, out.as_mut_ptr());
    }
    out
}

/// `vshr.s16`. Shift each signed 16-bit lane right by `N`. `N` must be in
/// `1..=16`.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn vshr<const N: u32>(a: V128) -> V128 {
    const { assert!(N >= 1 && N <= 16, "shift immediate out of range") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_vshr_s16(a.as_ptr(), N as i32, out.as_mut_ptr());
    }
    out
}
