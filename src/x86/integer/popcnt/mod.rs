//! POPCNT.

unsafe extern "C" {
    fn rust_isa_popcnt_popcnt_u32(a: u32) -> u32;
    fn rust_isa_popcnt_popcnt_u64(a: u64) -> u64;
}

/// `popcnt` on 32-bit values.
///
/// # Safety
///
/// The caller must enable POPCNT.
#[target_feature(enable = "popcnt")]
#[inline]
pub unsafe fn popcnt_u32(a: u32) -> u32 {
    unsafe { rust_isa_popcnt_popcnt_u32(a) }
}

/// `popcnt` on 64-bit values.
///
/// # Safety
///
/// The caller must enable POPCNT.
#[target_feature(enable = "popcnt")]
#[inline]
pub unsafe fn popcnt_u64(a: u64) -> u64 {
    unsafe { rust_isa_popcnt_popcnt_u64(a) }
}
