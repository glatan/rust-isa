//! PowerPC Power8 vector instructions.
//!
//! Power8 vector instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the Power8 feature set. The
//! C file is compiled with clang and `-mcpu=power8`.
//!
//! The values are [`V128`]. The module contains the 64-bit integer add and
//! subtract (`vaddudm`, `vsubudm`), 32-bit multiply (`vmuluwm`) and the even
//! halfword widening multiply.
//!
//! `vctzd` was introduced by Power ISA 3.0. It is re-exported here because the
//! Power8 extension table lists it, and calling it requires the Power9 vector
//! feature set.
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::powerpc::simd::altivec::V128;
//! use rust_isa::powerpc::simd::power8_vector::vaddudm;
//!
//! let value = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { vaddudm(value, value) }.to_bytes(), [0; 16]);
//! ```
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::powerpc::simd::power8_vector::*;
//! use rust_isa::powerpc::simd::altivec::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { add_u64(v128, v128) };
//! let _ = unsafe { mul_even_u16(v128, v128) };
//! let _ = unsafe { mul_u32(v128, v128) };
//! let _ = unsafe { sub_u64(v128, v128) };
//! let _ = unsafe { vaddudm(v128, v128) };
//! let _ = unsafe { vclzd(v128) };
//! let _ = unsafe { vctzd(v128) };
//! let _ = unsafe { vmaxsd(v128, v128) };
//! let _ = unsafe { vmaxud(v128, v128) };
//! let _ = unsafe { vminsd(v128, v128) };
//! let _ = unsafe { vminud(v128, v128) };
//! let _ = unsafe { vmuleuh(v128, v128) };
//! let _ = unsafe { vmuleuw(v128, v128) };
//! let _ = unsafe { vmulouw(v128, v128) };
//! let _ = unsafe { vmuluwm(v128, v128) };
//! let _ = unsafe { vpksdss(v128, v128) };
//! let _ = unsafe { vpksdus(v128, v128) };
//! let _ = unsafe { vpkudum(v128, v128) };
//! let _ = unsafe { vpopcntd(v128) };
//! let _ = unsafe { vsubudm(v128, v128) };
//! let _ = unsafe { vupkhsw(v128) };
//! let _ = unsafe { vupklsw(v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::powerpc::simd::power8_vector::add_u64 as _;
//! use rust_isa::powerpc::simd::power8_vector::mul_even_u16 as _;
//! use rust_isa::powerpc::simd::power8_vector::mul_u32 as _;
//! use rust_isa::powerpc::simd::power8_vector::sub_u64 as _;
//! use rust_isa::powerpc::simd::power8_vector::vaddudm as _;
//! use rust_isa::powerpc::simd::power8_vector::vclzd as _;
//! use rust_isa::powerpc::simd::power8_vector::vctzd as _;
//! use rust_isa::powerpc::simd::power8_vector::vmaxsd as _;
//! use rust_isa::powerpc::simd::power8_vector::vmaxud as _;
//! use rust_isa::powerpc::simd::power8_vector::vminsd as _;
//! use rust_isa::powerpc::simd::power8_vector::vminud as _;
//! use rust_isa::powerpc::simd::power8_vector::vmuleuh as _;
//! use rust_isa::powerpc::simd::power8_vector::vmuleuw as _;
//! use rust_isa::powerpc::simd::power8_vector::vmulouw as _;
//! use rust_isa::powerpc::simd::power8_vector::vmuluwm as _;
//! use rust_isa::powerpc::simd::power8_vector::vpksdss as _;
//! use rust_isa::powerpc::simd::power8_vector::vpksdus as _;
//! use rust_isa::powerpc::simd::power8_vector::vpkudum as _;
//! use rust_isa::powerpc::simd::power8_vector::vpopcntd as _;
//! use rust_isa::powerpc::simd::power8_vector::vsubudm as _;
//! use rust_isa::powerpc::simd::power8_vector::vupkhsw as _;
//! use rust_isa::powerpc::simd::power8_vector::vupklsw as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power8_vector_add_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_sub_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_mul_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_mul_even_u16(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the Power8 vector feature set.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `vaddudm`.
    add_u64,
    rust_isa_power8_vector_add_u64
);
binary!(
    /// `vsubudm`.
    sub_u64,
    rust_isa_power8_vector_sub_u64
);
binary!(
    /// `vmuluwm`.
    mul_u32,
    rust_isa_power8_vector_mul_u32
);
binary!(
    /// `vmuleuh`.
    mul_even_u16,
    rust_isa_power8_vector_mul_even_u16
);

unsafe extern "C" {
    fn rust_isa_power8_vector_vmuleuw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vmulouw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vmaxud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vminud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vmaxsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vminsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vpkudum(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vpksdus(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vpksdss(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vupkhsw(a: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vupklsw(a: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vclzd(a: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vpopcntd(a: *const u8, out: *mut u8);
}

macro_rules! bin8 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the Power8 vector feature set.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! un8 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the Power8 vector feature set.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
bin8!(
    /// `vmuleuw`.
    vmuleuw,
    rust_isa_power8_vector_vmuleuw
);
bin8!(
    /// `vmulouw`.
    vmulouw,
    rust_isa_power8_vector_vmulouw
);
bin8!(
    /// `vmaxud`.
    vmaxud,
    rust_isa_power8_vector_vmaxud
);
bin8!(
    /// `vminud`.
    vminud,
    rust_isa_power8_vector_vminud
);
bin8!(
    /// `vmaxsd`.
    vmaxsd,
    rust_isa_power8_vector_vmaxsd
);
bin8!(
    /// `vminsd`.
    vminsd,
    rust_isa_power8_vector_vminsd
);
bin8!(
    /// `vpkudum`.
    vpkudum,
    rust_isa_power8_vector_vpkudum
);
bin8!(
    /// `vpksdus`.
    vpksdus,
    rust_isa_power8_vector_vpksdus
);
bin8!(
    /// `vpksdss`.
    vpksdss,
    rust_isa_power8_vector_vpksdss
);
un8!(
    /// `vupkhsw`.
    vupkhsw,
    rust_isa_power8_vector_vupkhsw
);
un8!(
    /// `vupklsw`.
    vupklsw,
    rust_isa_power8_vector_vupklsw
);
un8!(
    /// `vclzd`.
    vclzd,
    rust_isa_power8_vector_vclzd
);
un8!(
    /// `vpopcntd`.
    vpopcntd,
    rust_isa_power8_vector_vpopcntd
);

/// `vaddudm`.
///
/// # Safety
///
/// The caller must enable the Power8 vector feature set.
#[inline(always)]
pub unsafe fn vaddudm(a: V128, b: V128) -> V128 {
    unsafe { add_u64(a, b) }
}
/// `vsubudm`.
///
/// # Safety
///
/// The caller must enable the Power8 vector feature set.
#[inline(always)]
pub unsafe fn vsubudm(a: V128, b: V128) -> V128 {
    unsafe { sub_u64(a, b) }
}
/// `vmuluwm`.
///
/// # Safety
///
/// The caller must enable the Power8 vector feature set.
#[inline(always)]
pub unsafe fn vmuluwm(a: V128, b: V128) -> V128 {
    unsafe { mul_u32(a, b) }
}
/// `vmuleuh`.
///
/// # Safety
///
/// The caller must enable the Power8 vector feature set.
#[inline(always)]
pub unsafe fn vmuleuh(a: V128, b: V128) -> V128 {
    unsafe { mul_even_u16(a, b) }
}

pub use super::power9_vector::vctzd;
