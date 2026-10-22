//! F16C.
//!
//! F16C is not part of the x86-64 baseline. Every operation is `unsafe` and the
//! caller must enable F16C with `#[target_feature(enable = "f16c")]` or a global
//! target feature.
//!
//! Half-precision values use the IEEE 754 binary16 layout in the low 16 bits of
//! each 32-bit lane. The `vcvtps2ph` functions take a rounding mode in `IMM`:
//! 0 is nearest, 1 is down, 2 is up, 3 is truncate, and 4 uses MXCSR.
//! A larger value is a compile-time error.

use super::avx2::V256;
use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_f16c_vcvtph2ps(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtph2ps_v256(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_0(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_1(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_2(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_3(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_4(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_v256_0(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_v256_1(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_v256_2(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_v256_3(a: *const u8, out: *mut u8);
    fn rust_isa_f16c_vcvtps2ph_v256_4(a: *const u8, out: *mut u8);
}

/// `vcvtph2ps`
///
/// Convert four 16-bit half-precision lanes to four 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable F16C.
#[target_feature(enable = "f16c")]
#[inline]
pub unsafe fn vcvtph2ps(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_f16c_vcvtph2ps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtph2ps` over 256 bits.
///
/// Convert eight 16-bit half-precision lanes to eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable F16C.
#[target_feature(enable = "f16c")]
#[inline]
pub unsafe fn vcvtph2ps_v256(a: V128) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_f16c_vcvtph2ps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcvtps2ph::<IMM>`
///
/// Convert four 32-bit float lanes to four 16-bit half-precision lanes using
/// the rounding mode selected by `IMM`. `IMM` must be in `0..=4`: 0 is
/// round-to-nearest-even, 1 is round-down, 2 is round-up, 3 is truncate, and 4
/// uses the current MXCSR rounding mode.
///
/// # Safety
///
/// The caller must enable F16C.
#[target_feature(enable = "f16c")]
#[inline]
pub unsafe fn vcvtps2ph<const IMM: u8>(a: V128) -> V128 {
    const { assert!(IMM <= 4, "vcvtps2ph rounding mode must be in 0..=4") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match IMM {
        0 => call!(rust_isa_f16c_vcvtps2ph_0),
        1 => call!(rust_isa_f16c_vcvtps2ph_1),
        2 => call!(rust_isa_f16c_vcvtps2ph_2),
        3 => call!(rust_isa_f16c_vcvtps2ph_3),
        4 => call!(rust_isa_f16c_vcvtps2ph_4),
        _ => unreachable!(),
    }
}

/// `vcvtps2ph::<IMM>` over 256 bits.
///
/// Convert eight 32-bit float lanes to eight 16-bit half-precision lanes using
/// the rounding mode selected by `IMM`. `IMM` must be in `0..=4`: 0 is
/// round-to-nearest-even, 1 is round-down, 2 is round-up, 3 is truncate, and 4
/// uses the current MXCSR rounding mode.
///
/// # Safety
///
/// The caller must enable F16C.
#[target_feature(enable = "f16c")]
#[inline]
pub unsafe fn vcvtps2ph_v256<const IMM: u8>(a: V256) -> V128 {
    const { assert!(IMM <= 4, "vcvtps2ph rounding mode must be in 0..=4") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match IMM {
        0 => call!(rust_isa_f16c_vcvtps2ph_v256_0),
        1 => call!(rust_isa_f16c_vcvtps2ph_v256_1),
        2 => call!(rust_isa_f16c_vcvtps2ph_v256_2),
        3 => call!(rust_isa_f16c_vcvtps2ph_v256_3),
        4 => call!(rust_isa_f16c_vcvtps2ph_v256_4),
        _ => unreachable!(),
    }
}
