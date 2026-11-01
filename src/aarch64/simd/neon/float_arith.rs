//! NEON floating-point arithmetic.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_fmaxnm_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmaxnm_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fminnm_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fminnm_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmulx_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmulx_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fabd_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fabd_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmla_f32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_fmla_f64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_fmls_f32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_fmls_f64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
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
    fn rust_isa_neon_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_fmul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `fmaxnm_f32`
///
/// Take the maximum of float lanes, preferring numeric values.
#[inline(always)]
pub fn fmaxnm_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmaxnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmaxnm_f64`
///
/// Take the maximum of float lanes, preferring numeric values.
#[inline(always)]
pub fn fmaxnm_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmaxnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fminnm_f32`
///
/// Take the minimum of float lanes, preferring numeric values.
#[inline(always)]
pub fn fminnm_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fminnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fminnm_f64`
///
/// Take the minimum of float lanes, preferring numeric values.
#[inline(always)]
pub fn fminnm_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fminnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmla_f32`
///
/// Multiply f32 lanes and add the result to the accumulator.
#[inline(always)]
pub fn fmla_f32(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmla_f32(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmla_f64`
///
/// Multiply f64 lanes and add the result to the accumulator.
#[inline(always)]
pub fn fmla_f64(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmla_f64(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmls_f32`
///
/// Multiply f32 lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn fmls_f32(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmls_f32(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmls_f64`
///
/// Multiply f64 lanes and subtract the result from the accumulator.
#[inline(always)]
pub fn fmls_f64(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmls_f64(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmulx_f32`
///
/// Multiply float lanes with the extended multiply behavior.
#[inline(always)]
pub fn fmulx_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmulx_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmulx_f64`
///
/// Multiply float lanes with the extended multiply behavior.
#[inline(always)]
pub fn fmulx_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmulx_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fabd_f32`
///
/// Take the absolute difference of float lanes.
#[inline(always)]
pub fn fabd_f32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fabd_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fabd_f64`
///
/// Take the absolute difference of float lanes.
#[inline(always)]
pub fn fabd_f64(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fabd_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
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
    fadd_f64,
    rust_isa_neon_fadd_f64
);
binary!(
    /// Multiply 64-bit float lanes.
    fmul_f64,
    rust_isa_neon_fmul_f64
);
binary!(
    /// Subtract 64-bit float lanes.
    fsub_f64,
    rust_isa_neon_fsub_f64
);
binary!(
    /// Divide 64-bit float lanes.
    fdiv_f64,
    rust_isa_neon_fdiv_f64
);

/// `fadd`
///
/// Add the four 32-bit float lanes.
#[inline(always)]
pub fn fadd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fadd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fmul`
///
/// Multiply the four 32-bit float lanes.
#[inline(always)]
pub fn fmul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_fmul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
