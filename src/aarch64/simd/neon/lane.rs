//! NEON lane construction and extraction.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_splat_i8(value: i8, out: *mut u8);
    fn rust_isa_neon_splat_i16(value: i16, out: *mut u8);
    fn rust_isa_neon_splat_i32(value: i32, out: *mut u8);
    fn rust_isa_neon_splat_i64(value: i64, out: *mut u8);
    fn rust_isa_neon_splat_f32(value: f32, out: *mut u8);
    fn rust_isa_neon_splat_f64(value: f64, out: *mut u8);
    fn rust_isa_neon_extract_i32_0(a: *const u8) -> i32;
    fn rust_isa_neon_extract_i32_1(a: *const u8) -> i32;
    fn rust_isa_neon_extract_i32_2(a: *const u8) -> i32;
    fn rust_isa_neon_extract_i32_3(a: *const u8) -> i32;
    fn rust_isa_neon_replace_i32_0(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_replace_i32_1(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_replace_i32_2(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_replace_i32_3(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_dup_s8_0(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_1(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_2(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_3(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_4(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_5(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_6(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_7(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_9(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_10(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_11(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_12(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_13(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_14(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s8_15(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s8_0(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_1(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_2(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_3(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_4(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_5(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_6(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_7(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_8(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_9(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_10(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_11(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_12(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_13(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_14(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_ins_s8_15(a: *const u8, value: i8, out: *mut u8);
    fn rust_isa_neon_dup_s16_0(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s16_1(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s16_2(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s16_3(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s16_4(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s16_5(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s16_6(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s16_7(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s16_0(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_ins_s16_1(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_ins_s16_2(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_ins_s16_3(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_ins_s16_4(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_ins_s16_5(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_ins_s16_6(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_ins_s16_7(a: *const u8, value: i16, out: *mut u8);
    fn rust_isa_neon_dup_s32_0(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s32_1(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s32_2(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s32_3(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s32_0(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_ins_s32_1(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_ins_s32_2(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_ins_s32_3(a: *const u8, value: i32, out: *mut u8);
    fn rust_isa_neon_dup_s64_0(a: *const u8, out: *mut u8);
    fn rust_isa_neon_dup_s64_1(a: *const u8, out: *mut u8);
    fn rust_isa_neon_ins_s64_0(a: *const u8, value: i64, out: *mut u8);
    fn rust_isa_neon_ins_s64_1(a: *const u8, value: i64, out: *mut u8);
}

/// Broadcast an 8-bit value to all lanes.
#[inline(always)]
pub fn splat_i8(value: i8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_splat_i8(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 16-bit value to all lanes.
#[inline(always)]
pub fn splat_i16(value: i16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_splat_i16(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 32-bit value to all lanes.
#[inline(always)]
pub fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_splat_i32(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 64-bit value to both lanes.
#[inline(always)]
pub fn splat_i64(value: i64) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_splat_i64(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 32-bit float to all lanes.
#[inline(always)]
pub fn splat_f32(value: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_splat_f32(value, out.as_mut_ptr());
    }
    out
}

/// Broadcast a 64-bit float to both lanes.
#[inline(always)]
pub fn splat_f64(value: f64) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_splat_f64(value, out.as_mut_ptr());
    }
    out
}

/// `extract::<LANE>`
///
/// Return the selected 32-bit lane. `LANE` must be in `0..=3`; a larger value
/// is a compile-time error.
#[inline(always)]
pub fn extract_i32<const LANE: u8>(a: V128) -> i32 {
    const { assert!(LANE <= 3, "NEON 32-bit lane must be in 0..=3") };
    macro_rules! call {
        ($f:ident) => {
            unsafe { $f(a.as_ptr()) }
        };
    }
    match LANE {
        0 => call!(rust_isa_neon_extract_i32_0),
        1 => call!(rust_isa_neon_extract_i32_1),
        2 => call!(rust_isa_neon_extract_i32_2),
        3 => call!(rust_isa_neon_extract_i32_3),
        _ => unreachable!(),
    }
}

/// `replace::<LANE>`
///
/// Replace the selected 32-bit lane. `LANE` must be in `0..=3`; a larger value
/// is a compile-time error.
#[inline(always)]
pub fn replace_i32<const LANE: u8>(a: V128, value: i32) -> V128 {
    const { assert!(LANE <= 3, "NEON 32-bit lane must be in 0..=3") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_replace_i32_0),
        1 => call!(rust_isa_neon_replace_i32_1),
        2 => call!(rust_isa_neon_replace_i32_2),
        3 => call!(rust_isa_neon_replace_i32_3),
        _ => unreachable!(),
    }
}

/// `dup` on 8-bit lanes: broadcast lane `LANE`.
///
/// `LANE` must be in `0..16`.
#[inline(always)]
pub fn dup_i8<const LANE: u8>(a: V128) -> V128 {
    const { assert!(LANE < 16, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_dup_s8_0),
        1 => call!(rust_isa_neon_dup_s8_1),
        2 => call!(rust_isa_neon_dup_s8_2),
        3 => call!(rust_isa_neon_dup_s8_3),
        4 => call!(rust_isa_neon_dup_s8_4),
        5 => call!(rust_isa_neon_dup_s8_5),
        6 => call!(rust_isa_neon_dup_s8_6),
        7 => call!(rust_isa_neon_dup_s8_7),
        8 => call!(rust_isa_neon_dup_s8_8),
        9 => call!(rust_isa_neon_dup_s8_9),
        10 => call!(rust_isa_neon_dup_s8_10),
        11 => call!(rust_isa_neon_dup_s8_11),
        12 => call!(rust_isa_neon_dup_s8_12),
        13 => call!(rust_isa_neon_dup_s8_13),
        14 => call!(rust_isa_neon_dup_s8_14),
        15 => call!(rust_isa_neon_dup_s8_15),
        _ => unreachable!(),
    }
}

/// `ins` on 8-bit lanes: replace lane `LANE` with `value`.
///
/// `LANE` must be in `0..16`.
#[inline(always)]
pub fn ins_i8<const LANE: u8>(a: V128, value: i8) -> V128 {
    const { assert!(LANE < 16, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_ins_s8_0),
        1 => call!(rust_isa_neon_ins_s8_1),
        2 => call!(rust_isa_neon_ins_s8_2),
        3 => call!(rust_isa_neon_ins_s8_3),
        4 => call!(rust_isa_neon_ins_s8_4),
        5 => call!(rust_isa_neon_ins_s8_5),
        6 => call!(rust_isa_neon_ins_s8_6),
        7 => call!(rust_isa_neon_ins_s8_7),
        8 => call!(rust_isa_neon_ins_s8_8),
        9 => call!(rust_isa_neon_ins_s8_9),
        10 => call!(rust_isa_neon_ins_s8_10),
        11 => call!(rust_isa_neon_ins_s8_11),
        12 => call!(rust_isa_neon_ins_s8_12),
        13 => call!(rust_isa_neon_ins_s8_13),
        14 => call!(rust_isa_neon_ins_s8_14),
        15 => call!(rust_isa_neon_ins_s8_15),
        _ => unreachable!(),
    }
}

/// `dup` on 16-bit lanes: broadcast lane `LANE`.
///
/// `LANE` must be in `0..8`.
#[inline(always)]
pub fn dup_i16<const LANE: u8>(a: V128) -> V128 {
    const { assert!(LANE < 8, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_dup_s16_0),
        1 => call!(rust_isa_neon_dup_s16_1),
        2 => call!(rust_isa_neon_dup_s16_2),
        3 => call!(rust_isa_neon_dup_s16_3),
        4 => call!(rust_isa_neon_dup_s16_4),
        5 => call!(rust_isa_neon_dup_s16_5),
        6 => call!(rust_isa_neon_dup_s16_6),
        7 => call!(rust_isa_neon_dup_s16_7),
        _ => unreachable!(),
    }
}

/// `ins` on 16-bit lanes: replace lane `LANE` with `value`.
///
/// `LANE` must be in `0..8`.
#[inline(always)]
pub fn ins_i16<const LANE: u8>(a: V128, value: i16) -> V128 {
    const { assert!(LANE < 8, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_ins_s16_0),
        1 => call!(rust_isa_neon_ins_s16_1),
        2 => call!(rust_isa_neon_ins_s16_2),
        3 => call!(rust_isa_neon_ins_s16_3),
        4 => call!(rust_isa_neon_ins_s16_4),
        5 => call!(rust_isa_neon_ins_s16_5),
        6 => call!(rust_isa_neon_ins_s16_6),
        7 => call!(rust_isa_neon_ins_s16_7),
        _ => unreachable!(),
    }
}

/// `dup` on 32-bit lanes: broadcast lane `LANE`.
///
/// `LANE` must be in `0..4`.
#[inline(always)]
pub fn dup_i32<const LANE: u8>(a: V128) -> V128 {
    const { assert!(LANE < 4, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_dup_s32_0),
        1 => call!(rust_isa_neon_dup_s32_1),
        2 => call!(rust_isa_neon_dup_s32_2),
        3 => call!(rust_isa_neon_dup_s32_3),
        _ => unreachable!(),
    }
}

/// `ins` on 32-bit lanes: replace lane `LANE` with `value`.
///
/// `LANE` must be in `0..4`.
#[inline(always)]
pub fn ins_i32<const LANE: u8>(a: V128, value: i32) -> V128 {
    const { assert!(LANE < 4, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_ins_s32_0),
        1 => call!(rust_isa_neon_ins_s32_1),
        2 => call!(rust_isa_neon_ins_s32_2),
        3 => call!(rust_isa_neon_ins_s32_3),
        _ => unreachable!(),
    }
}

/// `dup` on 64-bit lanes: broadcast lane `LANE`.
///
/// `LANE` must be in `0..2`.
#[inline(always)]
pub fn dup_i64<const LANE: u8>(a: V128) -> V128 {
    const { assert!(LANE < 2, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_dup_s64_0),
        1 => call!(rust_isa_neon_dup_s64_1),
        _ => unreachable!(),
    }
}

/// `ins` on 64-bit lanes: replace lane `LANE` with `value`.
///
/// `LANE` must be in `0..2`.
#[inline(always)]
pub fn ins_i64<const LANE: u8>(a: V128, value: i64) -> V128 {
    const { assert!(LANE < 2, "NEON lane is out of range") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), value, out.as_mut_ptr());
            }
            out
        }};
    }
    match LANE {
        0 => call!(rust_isa_neon_ins_s64_0),
        1 => call!(rust_isa_neon_ins_s64_1),
        _ => unreachable!(),
    }
}
