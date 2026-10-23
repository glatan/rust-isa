//! Move mask instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_movmskps(a: *const u8) -> i32;
    fn rust_isa_sse2_movmskpd(a: *const u8) -> i32;
    fn rust_isa_sse2_pmovmskb(a: *const u8) -> i32;
}

/// `movmskps`
///
/// Collect the sign bit of each 32-bit float lane into the low four bits.
#[inline(always)]
pub fn movmskps(a: V128) -> u32 {
    unsafe { rust_isa_sse2_movmskps(a.as_ptr()) as u32 }
}

/// `movmskpd`
///
/// Collect the sign bit of each 64-bit float lane into the low two bits.
#[inline(always)]
pub fn movmskpd(a: V128) -> u32 {
    unsafe { rust_isa_sse2_movmskpd(a.as_ptr()) as u32 }
}

/// `pmovmskb`
///
/// Collect the most significant bit of each byte into the low 16 bits.
#[inline(always)]
pub fn pmovmskb(a: V128) -> u32 {
    unsafe { rust_isa_sse2_pmovmskb(a.as_ptr()) as u32 }
}
