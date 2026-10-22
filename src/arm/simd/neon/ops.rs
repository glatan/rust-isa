//! Additional NEON operations for 32-bit ARM.
//!
//! NEON is unstable on this target. Every operation is `unsafe` and the caller
//! must compile the whole crate with `-C target-feature=+neon`.

use super::V128;

unsafe extern "C" {
    fn rust_isa_arm_neon_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_orr(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_eor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpeq_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpeq_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpgt_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpgt_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_splat_i8(value: i8, out: *mut u8);
    fn rust_isa_arm_neon_splat_i16(value: i16, out: *mut u8);
    fn rust_isa_arm_neon_splat_i32(value: i32, out: *mut u8);
    fn rust_isa_arm_neon_zip1_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_uzp1_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_trn1_u8(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with NEON enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `vadd.i8`.
    add_i8,
    rust_isa_arm_neon_add_i8
);
binary!(
    /// `vadd.i16`.
    add_i16,
    rust_isa_arm_neon_add_i16
);
binary!(
    /// `vadd.i64`.
    add_i64,
    rust_isa_arm_neon_add_i64
);
binary!(
    /// `vsub.i8`.
    sub_i8,
    rust_isa_arm_neon_sub_i8
);
binary!(
    /// `vsub.i16`.
    sub_i16,
    rust_isa_arm_neon_sub_i16
);
binary!(
    /// `vsub.i32`.
    sub_i32,
    rust_isa_arm_neon_sub_i32
);
binary!(
    /// `vsub.i64`.
    sub_i64,
    rust_isa_arm_neon_sub_i64
);
binary!(
    /// `vmul.i16`.
    mul_i16,
    rust_isa_arm_neon_mul_i16
);
binary!(
    /// `vmul.i32`.
    mul_i32,
    rust_isa_arm_neon_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_arm_neon_and
);
binary!(
    /// Bitwise OR.
    orr,
    rust_isa_arm_neon_orr
);
binary!(
    /// Bitwise XOR.
    eor,
    rust_isa_arm_neon_eor
);
binary!(
    /// `vceq.i8`. A true lane is all bits one.
    cmpeq_i8,
    rust_isa_arm_neon_cmpeq_i8
);
binary!(
    /// `vceq.i16`. A true lane is all bits one.
    cmpeq_i16,
    rust_isa_arm_neon_cmpeq_i16
);
binary!(
    /// `vceq.i32`. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_arm_neon_cmpeq_i32
);
binary!(
    /// `vcgt.s8`. A true lane is all bits one.
    cmpgt_i8,
    rust_isa_arm_neon_cmpgt_i8
);
binary!(
    /// `vcgt.s16`. A true lane is all bits one.
    cmpgt_i16,
    rust_isa_arm_neon_cmpgt_i16
);
binary!(
    /// `vcgt.s32`. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_arm_neon_cmpgt_i32
);
binary!(
    /// `vadd.f32`.
    fadd,
    rust_isa_arm_neon_fadd
);
binary!(
    /// `vmul.f32`.
    fmul,
    rust_isa_arm_neon_fmul
);
binary!(
    /// `vsub.f32`.
    fsub,
    rust_isa_arm_neon_fsub
);
/// Broadcast an 8-bit value to all lanes.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn splat_i8(value: i8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_splat_i8(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 16-bit value to all lanes.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn splat_i16(value: i16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_splat_i16(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 32-bit value to all lanes.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_splat_i32(value, out.as_mut_ptr());
    }
    out
}

/// `vzip.8`
///
/// Interleave the low halves of two byte vectors.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn zip1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_zip1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vuzp.8`
///
/// Take the even bytes from two vectors and concatenate them.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn uzp1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_uzp1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vtrn.8`
///
/// Transpose the even bytes of two vectors.
///
/// # Safety
///
/// The caller must enable NEON.
#[inline(always)]
pub unsafe fn trn1(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_trn1_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
