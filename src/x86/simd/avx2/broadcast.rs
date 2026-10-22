use super::V256;
unsafe extern "C" {
    fn rust_isa_avx2_vpbroadcastb(value: u8, out: *mut u8);
    fn rust_isa_avx2_vpbroadcastw(value: u16, out: *mut u8);
    fn rust_isa_avx2_vpbroadcastq(value: u64, out: *mut u8);
    fn rust_isa_avx2_vpbroadcastd(value: u32, out: *mut u8);
}

/// `vpbroadcastb`
///
/// Broadcast a byte to all 32 byte lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpbroadcastb(value: u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpbroadcastb(value, out.as_mut_ptr());
    }
    out
}

/// `vpbroadcastw`
///
/// Broadcast a 16-bit value to all 16 word lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpbroadcastw(value: u16) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpbroadcastw(value, out.as_mut_ptr());
    }
    out
}

/// `vpbroadcastq`
///
/// Broadcast a 64-bit value to all four qword lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpbroadcastq(value: u64) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpbroadcastq(value, out.as_mut_ptr());
    }
    out
}

/// `vpbroadcastd`
///
/// Broadcast a 32-bit integer to the eight lanes.
///
/// # Safety
///
/// The caller must enable AVX2.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn vpbroadcastd(value: u32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_vpbroadcastd(value, out.as_mut_ptr());
    }
    out
}
