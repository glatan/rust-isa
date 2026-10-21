//! NEON floating-point instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fabs(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fneg(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fsqrt(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fadd_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmul_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fsub_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fdiv_f64(a: *const u8, b: *const u8, out: *mut u8);
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

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// Subtract 32-bit float lanes.
    fsub,
    rust_isa_neon_fsub
);
binary!(
    /// Divide 32-bit float lanes.
    fdiv,
    rust_isa_neon_fdiv
);
binary!(
    /// Return the minimum of each 32-bit float lane. NaN handling follows the
    /// AArch64 `FMIN` instruction.
    fmin,
    rust_isa_neon_fmin
);
binary!(
    /// Return the maximum of each 32-bit float lane. NaN handling follows the
    /// AArch64 `FMAX` instruction.
    fmax,
    rust_isa_neon_fmax
);
unary!(
    /// Clear the sign bit of each 32-bit float lane.
    fabs,
    rust_isa_neon_fabs
);
unary!(
    /// Flip the sign bit of each 32-bit float lane.
    fneg,
    rust_isa_neon_fneg
);
unary!(
    /// Square root of each 32-bit float lane.
    fsqrt,
    rust_isa_neon_fsqrt
);
binary!(
    /// Add 64-bit float lanes.
    add_f64,
    rust_isa_neon_fadd_f64
);
binary!(
    /// Multiply 64-bit float lanes.
    mul_f64,
    rust_isa_neon_fmul_f64
);
binary!(
    /// Subtract 64-bit float lanes.
    sub_f64,
    rust_isa_neon_fsub_f64
);
binary!(
    /// Divide 64-bit float lanes.
    div_f64,
    rust_isa_neon_fdiv_f64
);
