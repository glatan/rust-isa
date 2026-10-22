//! NEON immediate shift instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_sshr_s8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_srshr_s8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_ssra_s8(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_srsra_s8(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sshr_s16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_srshr_s16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_ssra_s16(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_srsra_s16(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sshr_s32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_srshr_s32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_ssra_s32(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_srsra_s32(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sshr_s64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_srshr_s64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_ssra_s64(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_srsra_s64(a: *const u8, count: i32, c: *const u8, out: *mut u8);
    fn rust_isa_neon_shrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn2_i16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn2_i32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshrun2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqrshrun2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqshrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn_i64(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_uqrshrn2_i64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shll_i8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shll_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shll_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sli_s8(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sri_s8(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sli_s16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sri_s16(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sli_s32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sri_s32(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sli_s64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sri_s64(a: *const u8, b: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshlu_s8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshlu_s16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshlu_s32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sqshlu_s64(a: *const u8, count: i32, out: *mut u8);
}

macro_rules! shift {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), count as i32, out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! shift_acc {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), count as i32, b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! shift_narrow2 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, low: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), low.as_ptr(), count as i32, out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! binary_count {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), count as i32, out.as_mut_ptr()); }
            out
        }
    };
}

shift!(
    /// `sshr` on 8-bit signed lanes by `count` bits.
    sshr_s8,
    rust_isa_neon_sshr_s8
);

shift!(
    /// `srshr` on 8-bit signed lanes by `count` bits, rounding.
    srshr_s8,
    rust_isa_neon_srshr_s8
);

shift_acc!(
    /// `ssra` adds `a >> count` to `b` on 8-bit signed lanes.
    ssra_s8,
    rust_isa_neon_ssra_s8
);

shift_acc!(
    /// `srsra` adds `a >> count` to `b` on 8-bit signed lanes, rounding.
    srsra_s8,
    rust_isa_neon_srsra_s8
);

shift!(
    /// `sshr` on 16-bit signed lanes by `count` bits.
    sshr_s16,
    rust_isa_neon_sshr_s16
);

shift!(
    /// `srshr` on 16-bit signed lanes by `count` bits, rounding.
    srshr_s16,
    rust_isa_neon_srshr_s16
);

shift_acc!(
    /// `ssra` adds `a >> count` to `b` on 16-bit signed lanes.
    ssra_s16,
    rust_isa_neon_ssra_s16
);

shift_acc!(
    /// `srsra` adds `a >> count` to `b` on 16-bit signed lanes, rounding.
    srsra_s16,
    rust_isa_neon_srsra_s16
);

shift!(
    /// `sshr` on 32-bit signed lanes by `count` bits.
    sshr_s32,
    rust_isa_neon_sshr_s32
);

shift!(
    /// `srshr` on 32-bit signed lanes by `count` bits, rounding.
    srshr_s32,
    rust_isa_neon_srshr_s32
);

shift_acc!(
    /// `ssra` adds `a >> count` to `b` on 32-bit signed lanes.
    ssra_s32,
    rust_isa_neon_ssra_s32
);

shift_acc!(
    /// `srsra` adds `a >> count` to `b` on 32-bit signed lanes, rounding.
    srsra_s32,
    rust_isa_neon_srsra_s32
);

shift!(
    /// `sshr` on 64-bit signed lanes by `count` bits.
    sshr_s64,
    rust_isa_neon_sshr_s64
);

shift!(
    /// `srshr` on 64-bit signed lanes by `count` bits, rounding.
    srshr_s64,
    rust_isa_neon_srshr_s64
);

shift_acc!(
    /// `ssra` adds `a >> count` to `b` on 64-bit signed lanes.
    ssra_s64,
    rust_isa_neon_ssra_s64
);

shift_acc!(
    /// `srsra` adds `a >> count` to `b` on 64-bit signed lanes, rounding.
    srsra_s64,
    rust_isa_neon_srsra_s64
);

shift!(
    /// `shrn` shifts 16-bit lanes right by `count` and narrows to 8-bit lanes.
    shrn_i16,
    rust_isa_neon_shrn_i16
);

shift_narrow2!(
    /// `shrn2` narrows the shifted high 16-bit lanes into the upper half of the result.
    shrn2_i16,
    rust_isa_neon_shrn2_i16
);

shift!(
    /// `shrn` shifts 32-bit lanes right by `count` and narrows to 16-bit lanes.
    shrn_i32,
    rust_isa_neon_shrn_i32
);

shift_narrow2!(
    /// `shrn2` narrows the shifted high 32-bit lanes into the upper half of the result.
    shrn2_i32,
    rust_isa_neon_shrn2_i32
);

shift!(
    /// `shrn` shifts 64-bit lanes right by `count` and narrows to 32-bit lanes.
    shrn_i64,
    rust_isa_neon_shrn_i64
);

shift_narrow2!(
    /// `shrn2` narrows the shifted high 64-bit lanes into the upper half of the result.
    shrn2_i64,
    rust_isa_neon_shrn2_i64
);

shift!(
    /// `sqshrn` shifts 16-bit lanes right by `count` and saturates to 8-bit lanes.
    sqshrn_i16,
    rust_isa_neon_sqshrn_i16
);

shift_narrow2!(
    /// `sqshrn2` saturating narrows into the upper half of the result.
    sqshrn2_i16,
    rust_isa_neon_sqshrn2_i16
);

shift!(
    /// `sqshrun` shifts signed 16-bit lanes right by `count` and saturates to unsigned 8-bit lanes.
    sqshrun_i16,
    rust_isa_neon_sqshrun_i16
);

shift_narrow2!(
    /// `sqshrun2` saturating narrows signed lanes to unsigned lanes into the upper half.
    sqshrun2_i16,
    rust_isa_neon_sqshrun2_i16
);

shift!(
    /// `sqrshrn` shifts 16-bit lanes right by `count` with rounding and saturating narrow.
    sqrshrn_i16,
    rust_isa_neon_sqrshrn_i16
);

shift_narrow2!(
    /// `sqrshrn2` rounding saturating narrows into the upper half of the result.
    sqrshrn2_i16,
    rust_isa_neon_sqrshrn2_i16
);

shift!(
    /// `sqrshrun` shifts signed 16-bit lanes right by `count` with rounding and saturating unsigned narrow.
    sqrshrun_i16,
    rust_isa_neon_sqrshrun_i16
);

shift_narrow2!(
    /// `sqrshrun2` rounding saturating unsigned narrows into the upper half.
    sqrshrun2_i16,
    rust_isa_neon_sqrshrun2_i16
);

shift!(
    /// `uqshrn` shifts unsigned 16-bit lanes right by `count` and saturating narrow.
    uqshrn_i16,
    rust_isa_neon_uqshrn_i16
);

shift_narrow2!(
    /// `uqshrn2` saturating narrows unsigned lanes into the upper half.
    uqshrn2_i16,
    rust_isa_neon_uqshrn2_i16
);

shift!(
    /// `uqrshrn` shifts unsigned 16-bit lanes right by `count` with rounding and saturating narrow.
    uqrshrn_i16,
    rust_isa_neon_uqrshrn_i16
);

shift_narrow2!(
    /// `uqrshrn2` rounding saturating unsigned narrows into the upper half.
    uqrshrn2_i16,
    rust_isa_neon_uqrshrn2_i16
);

shift!(
    /// `sqshrn` shifts 32-bit lanes right by `count` and saturates to 16-bit lanes.
    sqshrn_i32,
    rust_isa_neon_sqshrn_i32
);

shift_narrow2!(
    /// `sqshrn2` saturating narrows into the upper half of the result.
    sqshrn2_i32,
    rust_isa_neon_sqshrn2_i32
);

shift!(
    /// `sqshrun` shifts signed 32-bit lanes right by `count` and saturates to unsigned 16-bit lanes.
    sqshrun_i32,
    rust_isa_neon_sqshrun_i32
);

shift_narrow2!(
    /// `sqshrun2` saturating narrows signed lanes to unsigned lanes into the upper half.
    sqshrun2_i32,
    rust_isa_neon_sqshrun2_i32
);

shift!(
    /// `sqrshrn` shifts 32-bit lanes right by `count` with rounding and saturating narrow.
    sqrshrn_i32,
    rust_isa_neon_sqrshrn_i32
);

shift_narrow2!(
    /// `sqrshrn2` rounding saturating narrows into the upper half of the result.
    sqrshrn2_i32,
    rust_isa_neon_sqrshrn2_i32
);

shift!(
    /// `sqrshrun` shifts signed 32-bit lanes right by `count` with rounding and saturating unsigned narrow.
    sqrshrun_i32,
    rust_isa_neon_sqrshrun_i32
);

shift_narrow2!(
    /// `sqrshrun2` rounding saturating unsigned narrows into the upper half.
    sqrshrun2_i32,
    rust_isa_neon_sqrshrun2_i32
);

shift!(
    /// `uqshrn` shifts unsigned 32-bit lanes right by `count` and saturating narrow.
    uqshrn_i32,
    rust_isa_neon_uqshrn_i32
);

shift_narrow2!(
    /// `uqshrn2` saturating narrows unsigned lanes into the upper half.
    uqshrn2_i32,
    rust_isa_neon_uqshrn2_i32
);

shift!(
    /// `uqrshrn` shifts unsigned 32-bit lanes right by `count` with rounding and saturating narrow.
    uqrshrn_i32,
    rust_isa_neon_uqrshrn_i32
);

shift_narrow2!(
    /// `uqrshrn2` rounding saturating unsigned narrows into the upper half.
    uqrshrn2_i32,
    rust_isa_neon_uqrshrn2_i32
);

shift!(
    /// `sqshrn` shifts 64-bit lanes right by `count` and saturates to 32-bit lanes.
    sqshrn_i64,
    rust_isa_neon_sqshrn_i64
);

shift_narrow2!(
    /// `sqshrn2` saturating narrows into the upper half of the result.
    sqshrn2_i64,
    rust_isa_neon_sqshrn2_i64
);

shift!(
    /// `sqshrun` shifts signed 64-bit lanes right by `count` and saturates to unsigned 32-bit lanes.
    sqshrun_i64,
    rust_isa_neon_sqshrun_i64
);

shift_narrow2!(
    /// `sqshrun2` saturating narrows signed lanes to unsigned lanes into the upper half.
    sqshrun2_i64,
    rust_isa_neon_sqshrun2_i64
);

shift!(
    /// `sqrshrn` shifts 64-bit lanes right by `count` with rounding and saturating narrow.
    sqrshrn_i64,
    rust_isa_neon_sqrshrn_i64
);

shift_narrow2!(
    /// `sqrshrn2` rounding saturating narrows into the upper half of the result.
    sqrshrn2_i64,
    rust_isa_neon_sqrshrn2_i64
);

shift!(
    /// `sqrshrun` shifts signed 64-bit lanes right by `count` with rounding and saturating unsigned narrow.
    sqrshrun_i64,
    rust_isa_neon_sqrshrun_i64
);

shift_narrow2!(
    /// `sqrshrun2` rounding saturating unsigned narrows into the upper half.
    sqrshrun2_i64,
    rust_isa_neon_sqrshrun2_i64
);

shift!(
    /// `uqshrn` shifts unsigned 64-bit lanes right by `count` and saturating narrow.
    uqshrn_i64,
    rust_isa_neon_uqshrn_i64
);

shift_narrow2!(
    /// `uqshrn2` saturating narrows unsigned lanes into the upper half.
    uqshrn2_i64,
    rust_isa_neon_uqshrn2_i64
);

shift!(
    /// `uqrshrn` shifts unsigned 64-bit lanes right by `count` with rounding and saturating narrow.
    uqrshrn_i64,
    rust_isa_neon_uqrshrn_i64
);

shift_narrow2!(
    /// `uqrshrn2` rounding saturating unsigned narrows into the upper half.
    uqrshrn2_i64,
    rust_isa_neon_uqrshrn2_i64
);

shift!(
    /// `shll` shifts 8-bit lanes left by `count` and widens to 16-bit lanes.
    shll_i8,
    rust_isa_neon_shll_i8
);

shift!(
    /// `shll` shifts 16-bit lanes left by `count` and widens to 32-bit lanes.
    shll_i16,
    rust_isa_neon_shll_i16
);

shift!(
    /// `shll` shifts 32-bit lanes left by `count` and widens to 64-bit lanes.
    shll_i32,
    rust_isa_neon_shll_i32
);

binary_count!(
    /// `sli` shifts `a` left by `count` and inserts the low bits of `b`.
    sli_s8,
    rust_isa_neon_sli_s8
);

binary_count!(
    /// `sri` shifts `a` right by `count` and inserts into the upper bits of `b`.
    sri_s8,
    rust_isa_neon_sri_s8
);

binary_count!(
    /// `sli` shifts `a` left by `count` and inserts the low bits of `b`.
    sli_s16,
    rust_isa_neon_sli_s16
);

binary_count!(
    /// `sri` shifts `a` right by `count` and inserts into the upper bits of `b`.
    sri_s16,
    rust_isa_neon_sri_s16
);

binary_count!(
    /// `sli` shifts `a` left by `count` and inserts the low bits of `b`.
    sli_s32,
    rust_isa_neon_sli_s32
);

binary_count!(
    /// `sri` shifts `a` right by `count` and inserts into the upper bits of `b`.
    sri_s32,
    rust_isa_neon_sri_s32
);

binary_count!(
    /// `sli` shifts `a` left by `count` and inserts the low bits of `b`.
    sli_s64,
    rust_isa_neon_sli_s64
);

binary_count!(
    /// `sri` shifts `a` right by `count` and inserts into the upper bits of `b`.
    sri_s64,
    rust_isa_neon_sri_s64
);

shift!(
    /// `sqshlu` shifts signed 8-bit lanes left by `count` and saturates to unsigned.
    sqshlu_s8,
    rust_isa_neon_sqshlu_s8
);

shift!(
    /// `sqshlu` shifts signed 16-bit lanes left by `count` and saturates to unsigned.
    sqshlu_s16,
    rust_isa_neon_sqshlu_s16
);

shift!(
    /// `sqshlu` shifts signed 32-bit lanes left by `count` and saturates to unsigned.
    sqshlu_s32,
    rust_isa_neon_sqshlu_s32
);

shift!(
    /// `sqshlu` shifts signed 64-bit lanes left by `count` and saturates to unsigned.
    sqshlu_s64,
    rust_isa_neon_sqshlu_s64
);
