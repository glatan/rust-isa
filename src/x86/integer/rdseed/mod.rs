//! RDSEED.
//!
//! RDSEED is not part of the x86-64 baseline. Every operation is `unsafe` and
//! the caller must enable RDSEED with `#[target_feature(enable = "rdseed")]` or
//! a global target feature.
//!
//! RDSEED is intended to seed another random-number generator. The functions
//! operate on 32-bit and 64-bit scalar values. They return `None` when the
//! hardware reports that no seed is available; the caller may retry.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

unsafe extern "C" {
    fn rust_isa_rdseed_u32(out: *mut u32) -> i32;
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_rdseed_u64(out: *mut u64) -> i32;
}

/// `rdseed` with a 32-bit destination.
///
/// # Safety
///
/// The caller must enable RDSEED.
#[target_feature(enable = "rdseed")]
#[inline]
pub unsafe fn rdseed_u32() -> Option<u32> {
    let mut value = 0u32;
    if unsafe { rust_isa_rdseed_u32(&mut value) } != 0 {
        Some(value)
    } else {
        None
    }
}

/// `rdseed` with a 64-bit destination.
///
/// # Safety
///
/// The caller must enable RDSEED.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "rdseed")]
#[inline]
pub unsafe fn rdseed_u64() -> Option<u64> {
    let mut value = 0u64;
    if unsafe { rust_isa_rdseed_u64(&mut value) } != 0 {
        Some(value)
    } else {
        None
    }
}
