//! MOVBE.
//!
//! MOVBE is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
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

//! # Examples
//!
//! ```
//! use rust_isa::x86::integer::movbe::{movbe_load_u32, movbe_store_u32};
//!
//! let mut bytes = [0u8; 4];
//! unsafe { movbe_store_u32(bytes.as_mut_ptr(), 0x1234_5678) };
//! assert_eq!(bytes, [0x12, 0x34, 0x56, 0x78]);
//! assert_eq!(unsafe { movbe_load_u32(bytes.as_ptr()) }, 0x1234_5678);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::integer::movbe::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { movbe_load_u16(storage.0.as_ptr().cast()) };
//! let _ = unsafe { movbe_load_u32(storage.0.as_ptr().cast()) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { movbe_load_u64(storage.0.as_ptr().cast()) };
//! let _ = unsafe { movbe_store_u16(storage.0.as_mut_ptr().cast(), 0) };
//! let _ = unsafe { movbe_store_u32(storage.0.as_mut_ptr().cast(), 0) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { movbe_store_u64(storage.0.as_mut_ptr().cast(), 0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::integer::movbe::movbe_load_u16 as _;
//! use rust_isa::x86::integer::movbe::movbe_load_u32 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::integer::movbe::movbe_load_u64 as _;
//! use rust_isa::x86::integer::movbe::movbe_store_u16 as _;
//! use rust_isa::x86::integer::movbe::movbe_store_u32 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::integer::movbe::movbe_store_u64 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

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
