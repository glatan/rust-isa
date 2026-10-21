//! LSX.
//!
//! LSX is part of the LoongArch baseline. The arithmetic operations are safe.

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_lsx_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadd.w`
///
/// Add the four 32-bit integer lanes.
#[inline(always)]
pub fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.w`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
#[inline(always)]
pub fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `vadd.b` on 8-bit lanes.
    add_i8,
    rust_isa_lsx_add_i8
);
binary!(
    /// `vadd.h` on 16-bit lanes.
    add_i16,
    rust_isa_lsx_add_i16
);
binary!(
    /// `vadd.d` on 64-bit lanes.
    add_i64,
    rust_isa_lsx_add_i64
);
binary!(
    /// `vsub.b` on 8-bit lanes.
    sub_i8,
    rust_isa_lsx_sub_i8
);
binary!(
    /// `vsub.h` on 16-bit lanes.
    sub_i16,
    rust_isa_lsx_sub_i16
);
binary!(
    /// `vsub.w` on 32-bit lanes.
    sub_i32,
    rust_isa_lsx_sub_i32
);
binary!(
    /// `vsub.d` on 64-bit lanes.
    sub_i64,
    rust_isa_lsx_sub_i64
);
binary!(
    /// `vmul.h` on 16-bit lanes, keeping the low 16 bits.
    mul_i16,
    rust_isa_lsx_mul_i16
);
binary!(
    /// `vmul.w` on 32-bit lanes, keeping the low 32 bits.
    mul_i32,
    rust_isa_lsx_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_lsx_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_lsx_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_lsx_xor
);
binary!(
    /// `vseq.w` on 32-bit lanes. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_lsx_cmpeq_i32
);
binary!(
    /// `vslt.w` on signed 32-bit lanes. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_lsx_cmpgt_i32
);
binary!(
    /// `vfadd.s` on 32-bit float lanes.
    fadd,
    rust_isa_lsx_fadd
);
binary!(
    /// `vfsub.s` on 32-bit float lanes.
    fsub,
    rust_isa_lsx_fsub
);
binary!(
    /// `vfmul.s` on 32-bit float lanes.
    fmul,
    rust_isa_lsx_fmul
);
binary!(
    /// `vfdiv.s` on 32-bit float lanes.
    fdiv,
    rust_isa_lsx_fdiv
);

/// Broadcast a 32-bit value to all four lanes.
#[inline(always)]
pub fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_splat_i32(value, out.as_mut_ptr());
    }
    out
}

/// `vadd.w`.
#[inline(always)]
pub fn vadd(a: V128, b: V128) -> V128 {
    add(a, b)
}

/// `vsub.w`.
#[inline(always)]
pub fn vsub(a: V128, b: V128) -> V128 {
    sub_i32(a, b)
}

/// `vmul.w`.
#[inline(always)]
pub fn vmul(a: V128, b: V128) -> V128 {
    mul(a, b)
}

/// `vand.v`.
#[inline(always)]
pub fn vand(a: V128, b: V128) -> V128 {
    and(a, b)
}

/// `vor.v`.
#[inline(always)]
pub fn vor(a: V128, b: V128) -> V128 {
    or(a, b)
}

/// `vxor.v`.
#[inline(always)]
pub fn vxor(a: V128, b: V128) -> V128 {
    xor(a, b)
}

/// `vabs.w`.
#[inline(always)]
pub fn vabs(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for lane in 0..4 {
        let value = i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.wrapping_abs().to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vneg.w`.
#[inline(always)]
pub fn vneg(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for lane in 0..4 {
        let value = i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.wrapping_neg().to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vdiv.s`.
#[inline(always)]
pub fn vdiv(a: V128, b: V128) -> V128 {
    fdiv(a, b)
}

/// `vadd.d`.
#[inline(always)]
pub fn vadd_d(a: V128, b: V128) -> V128 {
    add_i64(a, b)
}

/// `vsub.d`.
#[inline(always)]
pub fn vsub_d(a: V128, b: V128) -> V128 {
    sub_i64(a, b)
}

/// `vadd.b`.
#[inline(always)]
pub fn vadd_b(a: V128, b: V128) -> V128 {
    add_i8(a, b)
}

/// `vadd.h`.
#[inline(always)]
pub fn vadd_h(a: V128, b: V128) -> V128 {
    add_i16(a, b)
}

/// `vsub.b`.
#[inline(always)]
pub fn vsub_b(a: V128, b: V128) -> V128 {
    sub_i8(a, b)
}

/// `vsub.h`.
#[inline(always)]
pub fn vsub_h(a: V128, b: V128) -> V128 {
    sub_i16(a, b)
}

/// `vmul.h`.
#[inline(always)]
pub fn vmul_h(a: V128, b: V128) -> V128 {
    mul_i16(a, b)
}
