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
