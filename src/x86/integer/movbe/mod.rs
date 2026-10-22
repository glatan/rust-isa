//! MOVBE.
//!
//! MOVBE is not part of the x86-64 baseline. Every operation is `unsafe` and
//! the caller must enable MOVBE with `#[target_feature(enable = "movbe")]` or a
//! global target feature.
//!
//! The functions operate on 16-bit, 32-bit, and 64-bit scalar values. The load
//! functions read a big-endian value and return it as a native integer. The
//! store functions write a native integer in big-endian order.
//! The memory operations do not require alignment.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

unsafe extern "C" {
    fn rust_isa_movbe_load_u16(p: *const u8) -> u16;
    fn rust_isa_movbe_store_u16(p: *mut u8, value: u16);
    fn rust_isa_movbe_load_u32(p: *const u8) -> u32;
    fn rust_isa_movbe_store_u32(p: *mut u8, value: u32);
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_movbe_load_u64(p: *const u8) -> u64;
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_movbe_store_u64(p: *mut u8, value: u64);
}

/// `movbe` load of a 16-bit value.
///
/// # Safety
///
/// The caller must enable MOVBE. `p` must be valid for reading 2 bytes.
#[target_feature(enable = "movbe")]
#[inline]
pub unsafe fn movbe_load_u16(p: *const u8) -> u16 {
    unsafe { rust_isa_movbe_load_u16(p) }
}

/// `movbe` store of a 16-bit value.
///
/// # Safety
///
/// The caller must enable MOVBE. `p` must be valid for writing 2 bytes.
#[target_feature(enable = "movbe")]
#[inline]
pub unsafe fn movbe_store_u16(p: *mut u8, value: u16) {
    unsafe {
        rust_isa_movbe_store_u16(p, value);
    }
}

/// `movbe` load of a 32-bit value.
///
/// # Safety
///
/// The caller must enable MOVBE. `p` must be valid for reading 4 bytes.
#[target_feature(enable = "movbe")]
#[inline]
pub unsafe fn movbe_load_u32(p: *const u8) -> u32 {
    unsafe { rust_isa_movbe_load_u32(p) }
}

/// `movbe` store of a 32-bit value.
///
/// # Safety
///
/// The caller must enable MOVBE. `p` must be valid for writing 4 bytes.
#[target_feature(enable = "movbe")]
#[inline]
pub unsafe fn movbe_store_u32(p: *mut u8, value: u32) {
    unsafe {
        rust_isa_movbe_store_u32(p, value);
    }
}

/// `movbe` load of a 64-bit value.
///
/// # Safety
///
/// The caller must enable MOVBE. `p` must be valid for reading 8 bytes.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "movbe")]
#[inline]
pub unsafe fn movbe_load_u64(p: *const u8) -> u64 {
    unsafe { rust_isa_movbe_load_u64(p) }
}

/// `movbe` store of a 64-bit value.
///
/// # Safety
///
/// The caller must enable MOVBE. `p` must be valid for writing 8 bytes.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "movbe")]
#[inline]
pub unsafe fn movbe_store_u64(p: *mut u8, value: u64) {
    unsafe {
        rust_isa_movbe_store_u64(p, value);
    }
}
