//! NEON conversion instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_scvtf_f32_s32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ucvtf_f32_u32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_scvtf_f64_s64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ucvtf_f64_u64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fcvtzs_s32_f32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fcvtzu_u32_f32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fcvtzs_s64_f64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_fcvtzu_u64_f64(a: *const u8, out: *mut u8);
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

unary!(
    /// `scvtf` from signed 32-bit lanes to f32 lanes.
    scvtf_f32_s32,
    rust_isa_neon_scvtf_f32_s32
);

unary!(
    /// `ucvtf` from unsigned 32-bit lanes to f32 lanes.
    ucvtf_f32_u32,
    rust_isa_neon_ucvtf_f32_u32
);

unary!(
    /// `scvtf` from signed 64-bit lanes to f64 lanes.
    scvtf_f64_s64,
    rust_isa_neon_scvtf_f64_s64
);

unary!(
    /// `ucvtf` from unsigned 64-bit lanes to f64 lanes.
    ucvtf_f64_u64,
    rust_isa_neon_ucvtf_f64_u64
);

unary!(
    /// `fcvtzs` from f32 lanes to signed 32-bit lanes, rounding toward zero.
    fcvtzs_s32_f32,
    rust_isa_neon_fcvtzs_s32_f32
);

unary!(
    /// `fcvtzu` from f32 lanes to unsigned 32-bit lanes, rounding toward zero.
    fcvtzu_u32_f32,
    rust_isa_neon_fcvtzu_u32_f32
);

unary!(
    /// `fcvtzs` from f64 lanes to signed 64-bit lanes, rounding toward zero.
    fcvtzs_s64_f64,
    rust_isa_neon_fcvtzs_s64_f64
);

unary!(
    /// `fcvtzu` from f64 lanes to unsigned 64-bit lanes, rounding toward zero.
    fcvtzu_u64_f64,
    rust_isa_neon_fcvtzu_u64_f64
);
