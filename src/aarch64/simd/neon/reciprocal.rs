//! NEON reciprocal instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_frecpe_f32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frsqrte_f32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frecps_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_frsqrts_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_frecpe_f64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frsqrte_f64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frecps_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_frsqrts_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urecpe_u32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ursqrte_u32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_frecpx_f32(a: f32) -> f32;
    fn rust_isa_neon_frecpx_f64(a: f64) -> f64;
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

unary!(
    /// `frecpe` estimate on 32-bit float lanes.
    frecpe_f32,
    rust_isa_neon_frecpe_f32
);

unary!(
    /// `frsqrte` estimate on 32-bit float lanes.
    frsqrte_f32,
    rust_isa_neon_frsqrte_f32
);

binary!(
    /// `frecps` step on 32-bit float lanes.
    frecps_f32,
    rust_isa_neon_frecps_f32
);

binary!(
    /// `frsqrts` step on 32-bit float lanes.
    frsqrts_f32,
    rust_isa_neon_frsqrts_f32
);

unary!(
    /// `frecpe` estimate on 64-bit float lanes.
    frecpe_f64,
    rust_isa_neon_frecpe_f64
);

unary!(
    /// `frsqrte` estimate on 64-bit float lanes.
    frsqrte_f64,
    rust_isa_neon_frsqrte_f64
);

binary!(
    /// `frecps` step on 64-bit float lanes.
    frecps_f64,
    rust_isa_neon_frecps_f64
);

binary!(
    /// `frsqrts` step on 64-bit float lanes.
    frsqrts_f64,
    rust_isa_neon_frsqrts_f64
);

unary!(
    /// `urecpe` estimate on unsigned 32-bit lanes.
    urecpe_u32,
    rust_isa_neon_urecpe_u32
);

unary!(
    /// `ursqrte` estimate on unsigned 32-bit lanes.
    ursqrte_u32,
    rust_isa_neon_ursqrte_u32
);

/// `frecpx` on a 32-bit float: reciprocal exponent.
#[inline(always)]
pub fn frecpx_f32(a: f32) -> f32 {
    unsafe { rust_isa_neon_frecpx_f32(a) }
}

/// `frecpx` on a 64-bit float: reciprocal exponent.
#[inline(always)]
pub fn frecpx_f64(a: f64) -> f64 {
    unsafe { rust_isa_neon_frecpx_f64(a) }
}
