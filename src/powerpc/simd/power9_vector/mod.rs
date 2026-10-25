//! PowerPC Power9 vector instructions.
//!
//! Power9 vector instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the `power9-vector` target
//! feature. The C file is compiled with clang and `-mcpu=power9`.
//!
//! The values are [`V128`].
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::powerpc::simd::altivec::V128;
//! use rust_isa::powerpc::simd::power9_vector::neg_i32;
//!
//! let value = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { neg_i32(value) }.to_bytes(), [0; 16]);
//! ```
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::powerpc::simd::power9_vector::*;
//! use rust_isa::powerpc::simd::altivec::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { abs_i32(v128) };
//! let _ = unsafe { neg_i32(v128) };
//! let _ = unsafe { neg_i64(v128) };
//! let _ = unsafe { vabsd(v128, v128) };
//! let _ = unsafe { vabsduw(v128, v128) };
//! let _ = unsafe { vctzb(v128) };
//! let _ = unsafe { vctzd(v128) };
//! let _ = unsafe { vctzh(v128) };
//! let _ = unsafe { vctzlsbb(v128) };
//! let _ = unsafe { vctzw(v128) };
//! let _ = unsafe { vextsb2d(v128) };
//! let _ = unsafe { vextsb2w(v128) };
//! let _ = unsafe { vextsh2d(v128) };
//! let _ = unsafe { vextsh2w(v128) };
//! let _ = unsafe { vextsw2d(v128) };
//! let _ = unsafe { vnegd(v128) };
//! let _ = unsafe { vnegw(v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::powerpc::simd::power9_vector::abs_i32 as _;
//! use rust_isa::powerpc::simd::power9_vector::neg_i32 as _;
//! use rust_isa::powerpc::simd::power9_vector::neg_i64 as _;
//! use rust_isa::powerpc::simd::power9_vector::vabsd as _;
//! use rust_isa::powerpc::simd::power9_vector::vabsduw as _;
//! use rust_isa::powerpc::simd::power9_vector::vctzb as _;
//! use rust_isa::powerpc::simd::power9_vector::vctzd as _;
//! use rust_isa::powerpc::simd::power9_vector::vctzh as _;
//! use rust_isa::powerpc::simd::power9_vector::vctzlsbb as _;
//! use rust_isa::powerpc::simd::power9_vector::vctzw as _;
//! use rust_isa::powerpc::simd::power9_vector::vextsb2d as _;
//! use rust_isa::powerpc::simd::power9_vector::vextsb2w as _;
//! use rust_isa::powerpc::simd::power9_vector::vextsh2d as _;
//! use rust_isa::powerpc::simd::power9_vector::vextsh2w as _;
//! use rust_isa::powerpc::simd::power9_vector::vextsw2d as _;
//! use rust_isa::powerpc::simd::power9_vector::vnegd as _;
//! use rust_isa::powerpc::simd::power9_vector::vnegw as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power9_vector_neg_i32(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_neg_i64(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_abs_i32(a: *const u8, out: *mut u8);
}

/// `vnegw`.
///
/// # Safety
///
/// The caller must enable the `power9-vector` target feature.
#[inline(always)]
pub unsafe fn neg_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power9_vector_neg_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vnegd`.
///
/// # Safety
///
/// The caller must enable the `power9-vector` target feature.
#[inline(always)]
pub unsafe fn neg_i64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power9_vector_neg_i64(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// Absolute value of each signed 32-bit lane.
///
/// # Safety
///
/// The caller must enable the `power9-vector` target feature.
#[inline(always)]
pub unsafe fn abs_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_power9_vector_abs_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_power9_vector_vnegw(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vnegd(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vabsduw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vabsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vctzlsbb(a: *const u8) -> u32;
    fn rust_isa_power9_vector_vextsb2w(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vextsh2w(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vextsb2d(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vextsh2d(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vextsw2d(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vctzb(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vctzh(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vctzw(a: *const u8, out: *mut u8);
    fn rust_isa_power9_vector_vctzd(a: *const u8, out: *mut u8);
}

macro_rules! unary9 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the `power9-vector` target feature.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! binary9 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the `power9-vector` target feature.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

unary9!(
    /// `vnegw`
    vnegw,
    rust_isa_power9_vector_vnegw
);
unary9!(
    /// `vnegd`
    vnegd,
    rust_isa_power9_vector_vnegd
);
binary9!(
    /// `vabsduw`
    vabsduw,
    rust_isa_power9_vector_vabsduw
);
binary9!(
    /// Absolute difference of signed 32-bit lanes (subtract and absolute value).
    vabsd,
    rust_isa_power9_vector_vabsd
);
unary9!(
    /// `vextsb2w`
    vextsb2w,
    rust_isa_power9_vector_vextsb2w
);
unary9!(
    /// `vextsh2w`
    vextsh2w,
    rust_isa_power9_vector_vextsh2w
);
unary9!(
    /// `vextsb2d`
    vextsb2d,
    rust_isa_power9_vector_vextsb2d
);
unary9!(
    /// `vextsh2d`
    vextsh2d,
    rust_isa_power9_vector_vextsh2d
);
unary9!(
    /// `vextsw2d`
    vextsw2d,
    rust_isa_power9_vector_vextsw2d
);
unary9!(
    /// `vctzb`
    vctzb,
    rust_isa_power9_vector_vctzb
);
unary9!(
    /// `vctzh`
    vctzh,
    rust_isa_power9_vector_vctzh
);
unary9!(
    /// `vctzw`
    vctzw,
    rust_isa_power9_vector_vctzw
);
unary9!(
    /// `vctzd`
    vctzd,
    rust_isa_power9_vector_vctzd
);

/// `vctzlsbb`
///
/// Return the number of trailing zero bits in the least significant byte.
///
/// # Safety
///
/// The caller must enable the `power9-vector` target feature.
#[inline(always)]
pub unsafe fn vctzlsbb(a: V128) -> u32 {
    unsafe { rust_isa_power9_vector_vctzlsbb(a.as_ptr()) }
}
