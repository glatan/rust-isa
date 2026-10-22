//! RDRAND.
//!
//! RDRAND is not part of the x86-64 baseline. Every operation is `unsafe` and
//! the caller must enable RDRAND with `#[target_feature(enable = "rdrand")]` or
//! a global target feature.
//!
//! The functions operate on 32-bit and 64-bit scalar values. They return `None`
//! when the hardware reports that no random value is available. The caller may
//! retry. The quality of the random value is the responsibility of the CPU and
//! the caller.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

unsafe extern "C" {
    fn rust_isa_rdrand_u32(out: *mut u32) -> i32;
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_rdrand_u64(out: *mut u64) -> i32;
}

/// `rdrand` with a 32-bit destination.
///
/// # Safety
///
/// The caller must enable RDRAND.
#[target_feature(enable = "rdrand")]
#[inline]
pub unsafe fn rdrand_u32() -> Option<u32> {
    let mut value = 0u32;
    if unsafe { rust_isa_rdrand_u32(&mut value) } != 0 {
        Some(value)
    } else {
        None
    }
}

/// `rdrand` with a 64-bit destination.
///
/// # Safety
///
/// The caller must enable RDRAND.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "rdrand")]
#[inline]
pub unsafe fn rdrand_u64() -> Option<u64> {
    let mut value = 0u64;
    if unsafe { rust_isa_rdrand_u64(&mut value) } != 0 {
        Some(value)
    } else {
        None
    }
}
