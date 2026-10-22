//! Integer arithmetic instructions.

use super::V256;
unsafe extern "C" {
    fn rust_isa_avx2_vpaddsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpaddsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpaddusb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpaddusw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsubsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsubsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsubusb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsubusw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpminub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpminsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpminsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpminud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpminsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaxub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaxsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaxsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaxud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaxsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmulhw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmulhuw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmulhrsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaddwd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmaddubsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsadbw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsignb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsignw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsignd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpaddb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsubb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpaddw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsubw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpaddd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsubd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpaddq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpsubq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmulld(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx2_vpmullw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddsb`
///
/// Add the 32 signed 8-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddsb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpaddsw`
///
/// Add the 16 signed 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddsw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpaddusb`
///
/// Add the 32 unsigned 8-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddusb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddusb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpaddusw`
///
/// Add the 16 unsigned 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddusw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddusw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsubsb`
///
/// Subtract the 32 signed 8-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubsb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsubsw`
///
/// Subtract the 16 signed 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubsw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsubusb`
///
/// Subtract the 32 unsigned 8-bit lanes with saturation at zero.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubusb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubusb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsubusw`
///
/// Subtract the 16 unsigned 16-bit lanes with saturation at zero.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubusw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubusw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpminub`
///
/// Take the minimum of the 32 unsigned 8-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpminub(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpminub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpminsb`
///
/// Take the minimum of the 32 signed 8-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpminsb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpminsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpminsw`
///
/// Take the minimum of the 16 signed 16-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpminsw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpminsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpminud`
///
/// Take the minimum of the eight unsigned 32-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpminud(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpminud(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpminsd`
///
/// Take the minimum of the eight signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpminsd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpminsd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaxub`
///
/// Take the maximum of the 32 unsigned 8-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaxub(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaxub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaxsb`
///
/// Take the maximum of the 32 signed 8-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaxsb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaxsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaxsw`
///
/// Take the maximum of the 16 signed 16-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaxsw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaxsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaxud`
///
/// Take the maximum of the eight unsigned 32-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaxud(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaxud(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaxsd`
///
/// Take the maximum of the eight signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaxsd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaxsd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmulhw`
///
/// Multiply the 16 signed 16-bit lanes and keep the high 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmulhw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmulhw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmulhuw`
///
/// Multiply the 16 unsigned 16-bit lanes and keep the high 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmulhuw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmulhuw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmulhrsw`
///
/// Multiply the 16 signed 16-bit lanes, add `0x4000`, shift right by 15, and
/// keep the low 16 bits. The result wraps only when both inputs are
/// `i16::MIN`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmulhrsw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmulhrsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaddwd`
///
/// Multiply signed 16-bit lane pairs, add each pair, and store the results in 32-bit lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaddwd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaddwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmaddubsw`
///
/// Multiply unsigned 8-bit lanes by signed 8-bit lanes, add each pair, and saturate to signed 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmaddubsw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmaddubsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsadbw`
///
/// Sum the absolute differences of each 8-byte block into a 64-bit lane.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsadbw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsadbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsignb`
///
/// Copy, negate, or zero each 8-bit lane according to the sign of `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsignb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsignb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsignw`
///
/// Copy, negate, or zero each 16-bit lane according to the sign of `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsignw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsignw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsignd`
///
/// Copy, negate, or zero each 32-bit lane according to the sign of `b`.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsignd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsignd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpaddb`
///
/// Add the 32 byte lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsubb`
///
/// Subtract the 32 byte lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubb(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpaddw`
///
/// Add the 16 16-bit integer lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsubw`
///
/// Subtract the 16 16-bit integer lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpaddd`
///
/// Add the eight 32-bit integer lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsubd`
///
/// Subtract the eight 32-bit integer lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpaddq`
///
/// Add the four 64-bit integer lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpaddq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpaddq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpsubq`
///
/// Subtract the four 64-bit integer lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpsubq(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpsubq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmulld`
///
/// Multiply the eight 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmulld(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmulld(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpmullw`
///
/// Multiply the 16 16-bit integer lanes, keeping the low 16 bits.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpmullw(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpmullw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
