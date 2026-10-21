//! SSSE3.
//!
//! SSSE3 is not part of the x86-64 baseline. Every operation is `unsafe` and
//! the caller must enable SSSE3 with `#[target_feature(enable = "ssse3")]` or a
//! global target feature.
//!
//! The values are [`V128`]. Byte shuffles use the low four bits of each index;
//! an index with the high bit set produces zero.

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_ssse3_pabsb(a: *const u8, out: *mut u8);
    fn rust_isa_ssse3_pabsw(a: *const u8, out: *mut u8);
    fn rust_isa_ssse3_pabsd(a: *const u8, out: *mut u8);
    fn rust_isa_ssse3_pshufb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_phaddw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_phaddd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_phaddsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_phsubw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_phsubd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_phsubsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_pmaddubsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_pmulhrsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_psignb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_psignw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_psignd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_0(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_1(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_2(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_3(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_4(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_5(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_6(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_7(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_9(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_10(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_11(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_12(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_13(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_14(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_15(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_17(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_18(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_19(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_20(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_21(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_22(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_23(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_24(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_25(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_26(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_27(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_28(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_29(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_30(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_ssse3_palignr_31(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pabsb`
///
/// Replace each signed byte lane with its absolute value. `i8::MIN` becomes
/// `i8::MIN` (all bits except the sign bit are cleared).
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn pabsb(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_pabsb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pabsw`
///
/// Replace each signed 16-bit lane with its absolute value.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn pabsw(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_pabsw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pabsd`
///
/// Replace each signed 32-bit lane with its absolute value.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn pabsd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_pabsd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pshufb`
///
/// Shuffle bytes in `a` using the low four bits of each index in `b`. An index
/// byte with the high bit set produces zero.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn pshufb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_pshufb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `phaddw`
///
/// Add adjacent signed 16-bit lanes.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn phaddw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_phaddw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `phaddd`
///
/// Add adjacent signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn phaddd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_phaddd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `phaddsw`
///
/// Add adjacent signed 16-bit lanes with signed saturation.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn phaddsw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_phaddsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `phsubw`
///
/// Subtract adjacent signed 16-bit lanes.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn phsubw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_phsubw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `phsubd`
///
/// Subtract adjacent signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn phsubd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_phsubd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `phsubsw`
///
/// Subtract adjacent signed 16-bit lanes with signed saturation.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn phsubsw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_phsubsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmaddubsw`
///
/// Multiply unsigned bytes by signed bytes, add adjacent products, and saturate
/// the result to signed 16 bits.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn pmaddubsw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_pmaddubsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmulhrsw`
///
/// Multiply signed 16-bit lanes, round the high 16 bits, and saturate.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn pmulhrsw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_pmulhrsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `psignb`
///
/// Apply the sign of each byte in `b` to the byte in `a`. Zero in `b` produces
/// zero; a negative sign negates the value.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn psignb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_psignb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `psignw`
///
/// Apply the sign of each 16-bit lane in `b` to the lane in `a`.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn psignw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_psignw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `psignd`
///
/// Apply the sign of each 32-bit lane in `b` to the lane in `a`.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn psignd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_ssse3_psignd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `palignr::<IMM>`
///
/// Concatenate `a` (high) and `b` (low), then shift the 32-byte result right by
/// `IMM` bytes and return the low 16 bytes.
///
/// `IMM` must be in `0..=31`; a larger value is a compile-time error.
///
/// # Safety
///
/// The caller must enable SSSE3.
#[target_feature(enable = "ssse3")]
#[inline]
pub unsafe fn palignr<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 31, "palignr immediate must be in 0..=31") };
    macro_rules! call {
        ($f:ident) => {{
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $f(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }};
    }
    match IMM {
        0 => call!(rust_isa_ssse3_palignr_0),
        1 => call!(rust_isa_ssse3_palignr_1),
        2 => call!(rust_isa_ssse3_palignr_2),
        3 => call!(rust_isa_ssse3_palignr_3),
        4 => call!(rust_isa_ssse3_palignr_4),
        5 => call!(rust_isa_ssse3_palignr_5),
        6 => call!(rust_isa_ssse3_palignr_6),
        7 => call!(rust_isa_ssse3_palignr_7),
        8 => call!(rust_isa_ssse3_palignr_8),
        9 => call!(rust_isa_ssse3_palignr_9),
        10 => call!(rust_isa_ssse3_palignr_10),
        11 => call!(rust_isa_ssse3_palignr_11),
        12 => call!(rust_isa_ssse3_palignr_12),
        13 => call!(rust_isa_ssse3_palignr_13),
        14 => call!(rust_isa_ssse3_palignr_14),
        15 => call!(rust_isa_ssse3_palignr_15),
        16 => call!(rust_isa_ssse3_palignr_16),
        17 => call!(rust_isa_ssse3_palignr_17),
        18 => call!(rust_isa_ssse3_palignr_18),
        19 => call!(rust_isa_ssse3_palignr_19),
        20 => call!(rust_isa_ssse3_palignr_20),
        21 => call!(rust_isa_ssse3_palignr_21),
        22 => call!(rust_isa_ssse3_palignr_22),
        23 => call!(rust_isa_ssse3_palignr_23),
        24 => call!(rust_isa_ssse3_palignr_24),
        25 => call!(rust_isa_ssse3_palignr_25),
        26 => call!(rust_isa_ssse3_palignr_26),
        27 => call!(rust_isa_ssse3_palignr_27),
        28 => call!(rust_isa_ssse3_palignr_28),
        29 => call!(rust_isa_ssse3_palignr_29),
        30 => call!(rust_isa_ssse3_palignr_30),
        31 => call!(rust_isa_ssse3_palignr_31),
        _ => unreachable!(),
    }
}
