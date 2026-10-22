//! NEON saturating instructions.

use super::V128;
unsafe extern "C" {
    fn rust_isa_neon_sqadd_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqsub_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqadd_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqsub_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqsub_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqadd_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqsub_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqadd_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqsub_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqadd_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqsub_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqadd_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqsub_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqadd_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqsub_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqabs_s8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqneg_s8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqabs_s16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqneg_s16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqabs_s32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqneg_s32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmulh_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrdmulh_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmulh_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrdmulh_s32(a: *const u8, b: *const u8, out: *mut u8);
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

binary!(
    /// `sqadd` on 8-bit signed lanes.
    sqadd_s8,
    rust_isa_neon_sqadd_s8
);

binary!(
    /// `sqsub` on 8-bit signed lanes.
    sqsub_s8,
    rust_isa_neon_sqsub_s8
);

binary!(
    /// `sqadd` on 16-bit signed lanes.
    sqadd_s16,
    rust_isa_neon_sqadd_s16
);

binary!(
    /// `sqsub` on 16-bit signed lanes.
    sqsub_s16,
    rust_isa_neon_sqsub_s16
);

binary!(
    /// `sqadd` on 32-bit signed lanes.
    sqadd_s32,
    rust_isa_neon_sqadd_s32
);

binary!(
    /// `sqsub` on 32-bit signed lanes.
    sqsub_s32,
    rust_isa_neon_sqsub_s32
);

binary!(
    /// `sqadd` on 64-bit signed lanes.
    sqadd_s64,
    rust_isa_neon_sqadd_s64
);

binary!(
    /// `sqsub` on 64-bit signed lanes.
    sqsub_s64,
    rust_isa_neon_sqsub_s64
);

binary!(
    /// `uqadd` on 8-bit unsigned lanes.
    uqadd_u8,
    rust_isa_neon_uqadd_u8
);

binary!(
    /// `uqsub` on 8-bit unsigned lanes.
    uqsub_u8,
    rust_isa_neon_uqsub_u8
);

binary!(
    /// `uqadd` on 16-bit unsigned lanes.
    uqadd_u16,
    rust_isa_neon_uqadd_u16
);

binary!(
    /// `uqsub` on 16-bit unsigned lanes.
    uqsub_u16,
    rust_isa_neon_uqsub_u16
);

binary!(
    /// `uqadd` on 32-bit unsigned lanes.
    uqadd_u32,
    rust_isa_neon_uqadd_u32
);

binary!(
    /// `uqsub` on 32-bit unsigned lanes.
    uqsub_u32,
    rust_isa_neon_uqsub_u32
);

binary!(
    /// `uqadd` on 64-bit unsigned lanes.
    uqadd_u64,
    rust_isa_neon_uqadd_u64
);

binary!(
    /// `uqsub` on 64-bit unsigned lanes.
    uqsub_u64,
    rust_isa_neon_uqsub_u64
);

unary!(
    /// `sqabs` on 8-bit signed lanes.
    sqabs_s8,
    rust_isa_neon_sqabs_s8
);

unary!(
    /// `sqneg` on 8-bit signed lanes.
    sqneg_s8,
    rust_isa_neon_sqneg_s8
);

unary!(
    /// `sqabs` on 16-bit signed lanes.
    sqabs_s16,
    rust_isa_neon_sqabs_s16
);

unary!(
    /// `sqneg` on 16-bit signed lanes.
    sqneg_s16,
    rust_isa_neon_sqneg_s16
);

unary!(
    /// `sqabs` on 32-bit signed lanes.
    sqabs_s32,
    rust_isa_neon_sqabs_s32
);

unary!(
    /// `sqneg` on 32-bit signed lanes.
    sqneg_s32,
    rust_isa_neon_sqneg_s32
);

binary!(
    /// `sqdmulh` on 16-bit signed lanes: saturating doubling multiply high.
    sqdmulh_s16,
    rust_isa_neon_sqdmulh_s16
);

binary!(
    /// `sqrdmulh` on 16-bit signed lanes: saturating rounding doubling multiply high.
    sqrdmulh_s16,
    rust_isa_neon_sqrdmulh_s16
);

binary!(
    /// `sqdmulh` on 32-bit signed lanes: saturating doubling multiply high.
    sqdmulh_s32,
    rust_isa_neon_sqdmulh_s32
);

binary!(
    /// `sqrdmulh` on 32-bit signed lanes: saturating rounding doubling multiply high.
    sqrdmulh_s32,
    rust_isa_neon_sqrdmulh_s32
);
