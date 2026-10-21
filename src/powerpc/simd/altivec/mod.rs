//! Altivec.
//!
//! The caller must compile with `-C target-feature=+altivec,+vsx`.

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_altivec_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadduwm`
///
/// Add the four 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with Altivec and VSX enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuluwm`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with Altivec and VSX enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_altivec_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_permute(a: *const u8, b: *const u8, index: *const u8, out: *mut u8);
    fn rust_isa_altivec_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
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
    /// `vaddubm` on 8-bit lanes.
    add_i8,
    rust_isa_altivec_add_i8
);
binary!(
    /// `vadduhm` on 16-bit lanes.
    add_i16,
    rust_isa_altivec_add_i16
);
binary!(
    /// `vaddudm` (Power8) on 64-bit lanes.
    add_i64,
    rust_isa_altivec_add_i64
);
binary!(
    /// `vsububm` on 8-bit lanes.
    sub_i8,
    rust_isa_altivec_sub_i8
);
binary!(
    /// `vsubuhm` on 16-bit lanes.
    sub_i16,
    rust_isa_altivec_sub_i16
);
binary!(
    /// `vsubuwm` on 32-bit lanes.
    sub_i32,
    rust_isa_altivec_sub_i32
);
binary!(
    /// `vsubudm` (Power8) on 64-bit lanes.
    sub_i64,
    rust_isa_altivec_sub_i64
);
binary!(
    /// `vmuluhm` on 16-bit lanes.
    mul_i16,
    rust_isa_altivec_mul_i16
);
binary!(
    /// `vmuluwm` on 32-bit lanes.
    mul_i32,
    rust_isa_altivec_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_altivec_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_altivec_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_altivec_xor
);
binary!(
    /// `vcmpequw` on 32-bit lanes. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_altivec_cmpeq_i32
);
binary!(
    /// `vcmpgtsw` on signed 32-bit lanes. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_altivec_cmpgt_i32
);
binary!(
    /// Vector float add.
    fadd,
    rust_isa_altivec_fadd
);
binary!(
    /// Vector float subtract.
    fsub,
    rust_isa_altivec_fsub
);
binary!(
    /// Vector float multiply.
    fmul,
    rust_isa_altivec_fmul
);
binary!(
    /// Vector float divide.
    fdiv,
    rust_isa_altivec_fdiv
);

/// `vperm`
///
/// Permute bytes from the 32-byte concatenation of `a` and `b`. Each index
/// byte selects one of the 32 input bytes.
///
/// # Safety
///
/// The caller must enable Altivec.
#[inline(always)]
pub unsafe fn permute(a: V128, b: V128, index: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_permute(a.as_ptr(), b.as_ptr(), index.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// Broadcast a 32-bit value to all four lanes.
///
/// # Safety
///
/// The caller must enable Altivec.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_splat_i32(value, out.as_mut_ptr());
    }
    out
}
