//! NEON shift instructions.

use super::V128;
unsafe extern "C" {
    fn rust_isa_neon_shl_i8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shl_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shl_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shl_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shr_i8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shr_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shr_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sar_i8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sar_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sar_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srshl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqshl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrshl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srshl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqshl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrshl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrshl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sshl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srshl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqshl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqrshl_s64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ushl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urshl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqshl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqrshl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ushl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urshl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqshl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqrshl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ushl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urshl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqshl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqrshl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ushl_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urshl_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqshl_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uqrshl_u64(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! shift {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), count as i32, out.as_mut_ptr());
            }
            out
        }
    };
}

shift!(
    /// Shift 8-bit lanes left by `count` bits. `count` must be less than 8.
    shl_i8,
    rust_isa_neon_shl_i8
);
shift!(
    /// Shift 16-bit lanes left by `count` bits. `count` must be less than 16.
    shl_i16,
    rust_isa_neon_shl_i16
);
shift!(
    /// Shift 32-bit lanes left by `count` bits. `count` must be less than 32.
    shl_i32,
    rust_isa_neon_shl_i32
);
shift!(
    /// Shift 64-bit lanes left by `count` bits. `count` must be less than 64.
    shl_i64,
    rust_isa_neon_shl_i64
);
shift!(
    /// Shift 8-bit lanes right by `count` bits, filling with zero.
    shr_i8,
    rust_isa_neon_shr_i8
);
shift!(
    /// Shift 16-bit lanes right by `count` bits, filling with zero.
    shr_i16,
    rust_isa_neon_shr_i16
);
shift!(
    /// Shift 32-bit lanes right by `count` bits, filling with zero.
    shr_i32,
    rust_isa_neon_shr_i32
);
shift!(
    /// Shift signed 8-bit lanes right by `count` bits, filling with the sign.
    sar_i8,
    rust_isa_neon_sar_i8
);
shift!(
    /// Shift signed 16-bit lanes right by `count` bits, filling with the sign.
    sar_i16,
    rust_isa_neon_sar_i16
);
shift!(
    /// Shift signed 32-bit lanes right by `count` bits, filling with the sign.
    sar_i32,
    rust_isa_neon_sar_i32
);

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

binary!(
    /// `shl` on 8-bit signed lanes by the signed counts in `b`.
    shl_s8,
    rust_isa_neon_shl_s8
);

binary!(
    /// `sshl` on 8-bit signed lanes by the signed counts in `b`.
    sshl_s8,
    rust_isa_neon_sshl_s8
);

binary!(
    /// `srshl` on 8-bit signed lanes by the signed counts in `b`, rounding.
    srshl_s8,
    rust_isa_neon_srshl_s8
);

binary!(
    /// `sqshl` on 8-bit signed lanes by the signed counts in `b`.
    sqshl_s8,
    rust_isa_neon_sqshl_s8
);

binary!(
    /// `sqrshl` on 8-bit signed lanes by the signed counts in `b`, rounding.
    sqrshl_s8,
    rust_isa_neon_sqrshl_s8
);

binary!(
    /// `shl` on 16-bit signed lanes by the signed counts in `b`.
    shl_s16,
    rust_isa_neon_shl_s16
);

binary!(
    /// `sshl` on 16-bit signed lanes by the signed counts in `b`.
    sshl_s16,
    rust_isa_neon_sshl_s16
);

binary!(
    /// `srshl` on 16-bit signed lanes by the signed counts in `b`, rounding.
    srshl_s16,
    rust_isa_neon_srshl_s16
);

binary!(
    /// `sqshl` on 16-bit signed lanes by the signed counts in `b`.
    sqshl_s16,
    rust_isa_neon_sqshl_s16
);

binary!(
    /// `sqrshl` on 16-bit signed lanes by the signed counts in `b`, rounding.
    sqrshl_s16,
    rust_isa_neon_sqrshl_s16
);

binary!(
    /// `shl` on 32-bit signed lanes by the signed counts in `b`.
    shl_s32,
    rust_isa_neon_shl_s32
);

binary!(
    /// `sshl` on 32-bit signed lanes by the signed counts in `b`.
    sshl_s32,
    rust_isa_neon_sshl_s32
);

binary!(
    /// `srshl` on 32-bit signed lanes by the signed counts in `b`, rounding.
    srshl_s32,
    rust_isa_neon_srshl_s32
);

binary!(
    /// `sqshl` on 32-bit signed lanes by the signed counts in `b`.
    sqshl_s32,
    rust_isa_neon_sqshl_s32
);

binary!(
    /// `sqrshl` on 32-bit signed lanes by the signed counts in `b`, rounding.
    sqrshl_s32,
    rust_isa_neon_sqrshl_s32
);

binary!(
    /// `shl` on 64-bit signed lanes by the signed counts in `b`.
    shl_s64,
    rust_isa_neon_shl_s64
);

binary!(
    /// `sshl` on 64-bit signed lanes by the signed counts in `b`.
    sshl_s64,
    rust_isa_neon_sshl_s64
);

binary!(
    /// `srshl` on 64-bit signed lanes by the signed counts in `b`, rounding.
    srshl_s64,
    rust_isa_neon_srshl_s64
);

binary!(
    /// `sqshl` on 64-bit signed lanes by the signed counts in `b`.
    sqshl_s64,
    rust_isa_neon_sqshl_s64
);

binary!(
    /// `sqrshl` on 64-bit signed lanes by the signed counts in `b`, rounding.
    sqrshl_s64,
    rust_isa_neon_sqrshl_s64
);

binary!(
    /// `ushl` on 8-bit unsigned lanes by the signed counts in `b`.
    ushl_u8,
    rust_isa_neon_ushl_u8
);

binary!(
    /// `urshl` on 8-bit unsigned lanes by the signed counts in `b`, rounding.
    urshl_u8,
    rust_isa_neon_urshl_u8
);

binary!(
    /// `uqshl` on 8-bit unsigned lanes by the signed counts in `b`.
    uqshl_u8,
    rust_isa_neon_uqshl_u8
);

binary!(
    /// `uqrshl` on 8-bit unsigned lanes by the signed counts in `b`, rounding.
    uqrshl_u8,
    rust_isa_neon_uqrshl_u8
);

binary!(
    /// `ushl` on 16-bit unsigned lanes by the signed counts in `b`.
    ushl_u16,
    rust_isa_neon_ushl_u16
);

binary!(
    /// `urshl` on 16-bit unsigned lanes by the signed counts in `b`, rounding.
    urshl_u16,
    rust_isa_neon_urshl_u16
);

binary!(
    /// `uqshl` on 16-bit unsigned lanes by the signed counts in `b`.
    uqshl_u16,
    rust_isa_neon_uqshl_u16
);

binary!(
    /// `uqrshl` on 16-bit unsigned lanes by the signed counts in `b`, rounding.
    uqrshl_u16,
    rust_isa_neon_uqrshl_u16
);

binary!(
    /// `ushl` on 32-bit unsigned lanes by the signed counts in `b`.
    ushl_u32,
    rust_isa_neon_ushl_u32
);

binary!(
    /// `urshl` on 32-bit unsigned lanes by the signed counts in `b`, rounding.
    urshl_u32,
    rust_isa_neon_urshl_u32
);

binary!(
    /// `uqshl` on 32-bit unsigned lanes by the signed counts in `b`.
    uqshl_u32,
    rust_isa_neon_uqshl_u32
);

binary!(
    /// `uqrshl` on 32-bit unsigned lanes by the signed counts in `b`, rounding.
    uqrshl_u32,
    rust_isa_neon_uqrshl_u32
);

binary!(
    /// `ushl` on 64-bit unsigned lanes by the signed counts in `b`.
    ushl_u64,
    rust_isa_neon_ushl_u64
);

binary!(
    /// `urshl` on 64-bit unsigned lanes by the signed counts in `b`, rounding.
    urshl_u64,
    rust_isa_neon_urshl_u64
);

binary!(
    /// `uqshl` on 64-bit unsigned lanes by the signed counts in `b`.
    uqshl_u64,
    rust_isa_neon_uqshl_u64
);

binary!(
    /// `uqrshl` on 64-bit unsigned lanes by the signed counts in `b`, rounding.
    uqrshl_u64,
    rust_isa_neon_uqrshl_u64
);
