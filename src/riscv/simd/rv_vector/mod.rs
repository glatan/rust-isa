//! RISC-V V extension.
//!
//! The V extension is unstable. The caller must compile with
//! `-C target-feature=+v`; the wrappers do not carry `#[target_feature]`.

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_rv_vector_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadd.vv`
///
/// Add the four 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.vv`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rv_vector_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_mul_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_splat_i32(value: i32, out: *mut u8);
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
    /// `vadd.vv` on 8-bit lanes.
    add_i8,
    rust_isa_rv_vector_add_i8
);
binary!(
    /// `vadd.vv` on 16-bit lanes.
    add_i16,
    rust_isa_rv_vector_add_i16
);
binary!(
    /// `vadd.vv` on 64-bit lanes.
    add_i64,
    rust_isa_rv_vector_add_i64
);
binary!(
    /// `vsub.vv` on 8-bit lanes.
    sub_i8,
    rust_isa_rv_vector_sub_i8
);
binary!(
    /// `vsub.vv` on 16-bit lanes.
    sub_i16,
    rust_isa_rv_vector_sub_i16
);
binary!(
    /// `vsub.vv` on 32-bit lanes.
    sub_i32,
    rust_isa_rv_vector_sub_i32
);
binary!(
    /// `vsub.vv` on 64-bit lanes.
    sub_i64,
    rust_isa_rv_vector_sub_i64
);
binary!(
    /// `vmul.vv` on 8-bit lanes, keeping the low 8 bits.
    mul_i8,
    rust_isa_rv_vector_mul_i8
);
binary!(
    /// `vmul.vv` on 16-bit lanes, keeping the low 16 bits.
    mul_i16,
    rust_isa_rv_vector_mul_i16
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_rv_vector_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_rv_vector_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_rv_vector_xor
);
binary!(
    /// `vmseq.vv` on 32-bit lanes. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_rv_vector_cmpeq_i32
);
binary!(
    /// `vmsgt.vv` on signed 32-bit lanes. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_rv_vector_cmpgt_i32
);
binary!(
    /// `vfadd.vv` on 32-bit float lanes.
    fadd,
    rust_isa_rv_vector_fadd
);
binary!(
    /// `vfsub.vv` on 32-bit float lanes.
    fsub,
    rust_isa_rv_vector_fsub
);
binary!(
    /// `vfmul.vv` on 32-bit float lanes.
    fmul,
    rust_isa_rv_vector_fmul
);
binary!(
    /// `vfdiv.vv` on 32-bit float lanes.
    fdiv,
    rust_isa_rv_vector_fdiv
);

/// Broadcast a 32-bit value to all four lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_splat_i32(value, out.as_mut_ptr());
    }
    out
}
