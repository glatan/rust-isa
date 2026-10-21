//! MSA.
//!
//! MSA is unstable. The caller must compile with
//! `-C target-feature=+msa,+fp64`.

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_msa_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `addv.w`
///
/// Add the four 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mulv.w`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_msa_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_splat_i32(value: i32, out: *mut u8);
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
    /// `addv.b`.
    add_i8,
    rust_isa_msa_add_i8
);
binary!(
    /// `addv.h`.
    add_i16,
    rust_isa_msa_add_i16
);
binary!(
    /// `addv.d`.
    add_i64,
    rust_isa_msa_add_i64
);
binary!(
    /// `subv.b`.
    sub_i8,
    rust_isa_msa_sub_i8
);
binary!(
    /// `subv.h`.
    sub_i16,
    rust_isa_msa_sub_i16
);
binary!(
    /// `subv.w`.
    sub_i32,
    rust_isa_msa_sub_i32
);
binary!(
    /// `subv.d`.
    sub_i64,
    rust_isa_msa_sub_i64
);
binary!(
    /// `mulv.h`, keeping the low 16 bits.
    mul_i16,
    rust_isa_msa_mul_i16
);
binary!(
    /// `mulv.w`, keeping the low 32 bits.
    mul_i32,
    rust_isa_msa_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_msa_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_msa_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_msa_xor
);
binary!(
    /// `ceq.w`. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_msa_cmpeq_i32
);
binary!(
    /// `clt.s.w`. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_msa_cmpgt_i32
);
binary!(
    /// `fadd.w`.
    fadd,
    rust_isa_msa_fadd
);
binary!(
    /// `fsub.w`.
    fsub,
    rust_isa_msa_fsub
);
binary!(
    /// `fmul.w`.
    fmul,
    rust_isa_msa_fmul
);
binary!(
    /// `fdiv.w`.
    fdiv,
    rust_isa_msa_fdiv
);

/// Broadcast a 32-bit value to all four lanes.
///
/// # Safety
///
/// The caller must enable MSA on the CPU and in the target features.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_splat_i32(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_msa_fadd_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fsub_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fmul_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fdiv_f64(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary_f64 {
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

binary_f64!(
    /// `fadd.d` on 64-bit float lanes.
    fadd_f64,
    rust_isa_msa_fadd_f64
);
binary_f64!(
    /// `fsub.d` on 64-bit float lanes.
    fsub_f64,
    rust_isa_msa_fsub_f64
);
binary_f64!(
    /// `fmul.d` on 64-bit float lanes.
    fmul_f64,
    rust_isa_msa_fmul_f64
);
binary_f64!(
    /// `fdiv.d` on 64-bit float lanes.
    fdiv_f64,
    rust_isa_msa_fdiv_f64
);
