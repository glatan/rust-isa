//! s390x vector enhancements facility 3.
//!
//! These instructions extend the vector facility with 128-bit (qword) lanes,
//! element-mask generation, blending, and vector division. Every operation is
//! `unsafe` and the caller must enable the vector-enhancements-3 facility. The
//! values are [`V128`]. The byte order is big-endian.
//!
//! The qword forms treat the whole 128-bit value as one lane. The blend mask
//! uses the sign bit of each element, matching the instruction. The generated
//! masks contain all-ones for selected elements and zero otherwise. Signed
//! division and remainder are undefined for a zero divisor and for the
//! minimum-value divided by -1 case.
//!
//! Instruction behavior follows the [IBM z/Architecture Principles of Operation](https://www.ibm.com/docs/en/module_1678991624569/pdf/SA22-7832-14.pdf).
//!
//! The C file is compiled with clang and `-march=z17`. Instructions not listed
//! in this module are not exposed.
//!
//! The value tests are not executed because QEMU does not implement this
//! facility. The `vblendq` and `veval` wrappers use inline assembly and have
//! not been value-tested.

//! # Examples
//!
//! ```no_run
//! use rust_isa::s390x::simd::vector::V128;
//! use rust_isa::s390x::simd::vector_enhancements_3::vblendq;
//!
//! let a = V128::from_bytes([1; 16]);
//! let b = V128::from_bytes([2; 16]);
//! let mask = V128::from_bytes([0; 16]);
//! let _ = unsafe { vblendq(a, b, mask) };
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::s390x::simd::vector_enhancements_3::*;
//! use rust_isa::s390x::simd::vector::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! unsafe {
//!     let _ = unsafe { vacccq(v128, v128, v128) };
//!     let _ = unsafe { vaccq(v128, v128) };
//!     let _ = unsafe { vacq(v128, v128, v128) };
//!     let _ = unsafe { vaq(v128, v128) };
//!     let _ = unsafe { vavglq(v128, v128) };
//!     let _ = unsafe { vavgq(v128, v128) };
//!     let _ = unsafe { vblendb(v128, v128, v128) };
//!     let _ = unsafe { vblendf(v128, v128, v128) };
//!     let _ = unsafe { vblendg(v128, v128, v128) };
//!     let _ = unsafe { vblendh(v128, v128, v128) };
//!     let _ = unsafe { vblendq(v128, v128, v128) };
//!     let _ = unsafe { vclzq(v128) };
//!     let _ = unsafe { vctzq(v128) };
//!     let _ = unsafe { vdf(v128, v128) };
//!     let _ = unsafe { vdg(v128, v128) };
//!     let _ = unsafe { vdlf(v128, v128) };
//!     let _ = unsafe { vdlg(v128, v128) };
//!     let _ = unsafe { vdlq(v128, v128) };
//!     let _ = unsafe { vdq(v128, v128) };
//!     let _ = unsafe { veval::<0>(v128, v128, v128) };
//!     let _ = unsafe { vgemb(v128) };
//!     let _ = unsafe { vgemf(v128) };
//!     let _ = unsafe { vgemg(v128) };
//!     let _ = unsafe { vgemh(v128) };
//!     let _ = unsafe { vgemq(v128) };
//!     let _ = unsafe { vlcq(v128) };
//!     let _ = unsafe { vlpq(v128) };
//!     let _ = unsafe { vmaeg(v128, v128, v128) };
//!     let _ = unsafe { vmahg(v128, v128, v128) };
//!     let _ = unsafe { vmahq(v128, v128, v128) };
//!     let _ = unsafe { vmaleg(v128, v128, v128) };
//!     let _ = unsafe { vmalg(v128, v128, v128) };
//!     let _ = unsafe { vmalhg(v128, v128, v128) };
//!     let _ = unsafe { vmalhq(v128, v128, v128) };
//!     let _ = unsafe { vmalog(v128, v128, v128) };
//!     let _ = unsafe { vmalq(v128, v128, v128) };
//!     let _ = unsafe { vmaog(v128, v128, v128) };
//!     let _ = unsafe { vmeg(v128, v128) };
//!     let _ = unsafe { vmhg(v128, v128) };
//!     let _ = unsafe { vmhq(v128, v128) };
//!     let _ = unsafe { vmleg(v128, v128) };
//!     let _ = unsafe { vmlg(v128, v128) };
//!     let _ = unsafe { vmlhg(v128, v128) };
//!     let _ = unsafe { vmlhq(v128, v128) };
//!     let _ = unsafe { vmlog(v128, v128) };
//!     let _ = unsafe { vmlq(v128, v128) };
//!     let _ = unsafe { vmnlq(v128, v128) };
//!     let _ = unsafe { vmnq(v128, v128) };
//!     let _ = unsafe { vmog(v128, v128) };
//!     let _ = unsafe { vmxlq(v128, v128) };
//!     let _ = unsafe { vmxq(v128, v128) };
//!     let _ = unsafe { vrf(v128, v128) };
//!     let _ = unsafe { vrg(v128, v128) };
//!     let _ = unsafe { vrlf(v128, v128) };
//!     let _ = unsafe { vrlg(v128, v128) };
//!     let _ = unsafe { vrlq(v128, v128) };
//!     let _ = unsafe { vrq(v128, v128) };
//!     let _ = unsafe { vsq(v128, v128) };
//!     let _ = unsafe { vuphg(v128) };
//!     let _ = unsafe { vuplg(v128) };
//!     let _ = unsafe { vuplhg(v128) };
//!     let _ = unsafe { vupllg(v128) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::s390x::simd::vector_enhancements_3::vacccq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vaccq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vacq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vaq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vavglq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vavgq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vblendb as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vblendf as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vblendg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vblendh as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vblendq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vclzq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vctzq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vdf as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vdg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vdlf as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vdlg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vdlq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vdq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::veval as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vgemb as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vgemf as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vgemg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vgemh as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vgemq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vlcq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vlpq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmaeg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmahg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmahq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmaleg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmalg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmalhg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmalhq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmalog as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmalq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmaog as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmeg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmhg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmhq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmleg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmlg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmlhg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmlhq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmlog as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmlq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmnlq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmnq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmog as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmxlq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vmxq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vrf as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vrg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vrlf as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vrlg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vrlq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vrq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vsq as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vuphg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vuplg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vuplhg as _;
//! use rust_isa::s390x::simd::vector_enhancements_3::vupllg as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::vector::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_s390x_ve3_vaq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vsq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmalq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmxq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmnq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmxlq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmnlq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vavgq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vavglq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vclzq(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vctzq(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vlcq(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vlpq(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vuphg(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vuplg(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vuplhg(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vupllg(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vblendb(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vblendh(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vblendf(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vblendg(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vgemb(mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vgemh(mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vgemf(mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vgemg(mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vgemq(mask: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vdq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vrq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vdlq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vrlq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vdf(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vdg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vdlf(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vdlg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vrf(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vrg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vrlf(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vrlg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmalg(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmaeg(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmaleg(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmaog(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmalog(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmhg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmhq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmlhg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmlhq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmlg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmlq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmeg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmleg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmog(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmlog(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vaccq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vacq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vacccq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmahg(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmalhg(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmahq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_ve3_vmalhq(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancements-3 facility.
        #[target_feature(enable = "vector-enhancements-3")]
        #[inline]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) };
            out
        }
    };
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancements-3 facility.
        #[target_feature(enable = "vector-enhancements-3")]
        #[inline]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()) };
            out
        }
    };
}

macro_rules! ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancements-3 facility.
        #[target_feature(enable = "vector-enhancements-3")]
        #[inline]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) };
            out
        }
    };
}

binary!(
    /// `vaq`.
    ///
    /// Add the 128-bit lanes.
    vaq,
    rust_isa_s390x_ve3_vaq
);
binary!(
    /// `vsq`.
    ///
    /// Subtract the 128-bit lanes.
    vsq,
    rust_isa_s390x_ve3_vsq
);
ternary!(
    /// `vmalq`.
    ///
    /// Multiply the 128-bit lanes and add `acc`.
    vmalq,
    rust_isa_s390x_ve3_vmalq
);
binary!(
    /// `vmxq`.
    ///
    /// Maximum of the signed 128-bit lanes.
    vmxq,
    rust_isa_s390x_ve3_vmxq
);
binary!(
    /// `vmnq`.
    ///
    /// Minimum of the signed 128-bit lanes.
    vmnq,
    rust_isa_s390x_ve3_vmnq
);
binary!(
    /// `vmxlq`.
    ///
    /// Maximum of the unsigned 128-bit lanes.
    vmxlq,
    rust_isa_s390x_ve3_vmxlq
);
binary!(
    /// `vmnlq`.
    ///
    /// Minimum of the unsigned 128-bit lanes.
    vmnlq,
    rust_isa_s390x_ve3_vmnlq
);
binary!(
    /// `vavgq`.
    ///
    /// Average of the signed 128-bit lanes.
    vavgq,
    rust_isa_s390x_ve3_vavgq
);
binary!(
    /// `vavglq`.
    ///
    /// Average of the unsigned 128-bit lanes.
    vavglq,
    rust_isa_s390x_ve3_vavglq
);
unary!(
    /// `vclzq`.
    ///
    /// Count leading zero bits in the 128-bit lane.
    vclzq,
    rust_isa_s390x_ve3_vclzq
);
unary!(
    /// `vctzq`.
    ///
    /// Count trailing zero bits in the 128-bit lane.
    vctzq,
    rust_isa_s390x_ve3_vctzq
);
unary!(
    /// `vlcq`.
    ///
    /// Negate the signed 128-bit lane.
    vlcq,
    rust_isa_s390x_ve3_vlcq
);
unary!(
    /// `vlpq`.
    ///
    /// Absolute value of the signed 128-bit lane.
    vlpq,
    rust_isa_s390x_ve3_vlpq
);
unary!(
    /// `vuphg`.
    ///
    /// Unpack the high 64-bit lane with sign extension.
    vuphg,
    rust_isa_s390x_ve3_vuphg
);
unary!(
    /// `vuplg`.
    ///
    /// Unpack the low 64-bit lane with sign extension.
    vuplg,
    rust_isa_s390x_ve3_vuplg
);
unary!(
    /// `vuplhg`.
    ///
    /// Unpack the high 64-bit lane with zero extension.
    vuplhg,
    rust_isa_s390x_ve3_vuplhg
);
unary!(
    /// `vupllg`.
    ///
    /// Unpack the low 64-bit lane with zero extension.
    vupllg,
    rust_isa_s390x_ve3_vupllg
);

macro_rules! blend {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// The mask selects `b` where the sign bit of the element is set and
        /// `a` otherwise.
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancements-3 facility.
        #[target_feature(enable = "vector-enhancements-3")]
        #[inline]
        pub unsafe fn $name(a: V128, b: V128, mask: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr()) };
            out
        }
    };
}

blend!(
    /// `vblendb`.
    vblendb,
    rust_isa_s390x_ve3_vblendb
);
blend!(
    /// `vblendh`.
    vblendh,
    rust_isa_s390x_ve3_vblendh
);
blend!(
    /// `vblendf`.
    vblendf,
    rust_isa_s390x_ve3_vblendf
);
blend!(
    /// `vblendg`.
    vblendg,
    rust_isa_s390x_ve3_vblendg
);

macro_rules! gem {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// The result contains all-ones for selected elements.
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancements-3 facility.
        #[target_feature(enable = "vector-enhancements-3")]
        #[inline]
        pub unsafe fn $name(mask: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(mask.as_ptr(), out.as_mut_ptr()) };
            out
        }
    };
}

gem!(
    /// `vgemb`.
    vgemb,
    rust_isa_s390x_ve3_vgemb
);
gem!(
    /// `vgemh`.
    vgemh,
    rust_isa_s390x_ve3_vgemh
);
gem!(
    /// `vgemf`.
    vgemf,
    rust_isa_s390x_ve3_vgemf
);
gem!(
    /// `vgemg`.
    vgemg,
    rust_isa_s390x_ve3_vgemg
);
gem!(
    /// `vgemq`.
    vgemq,
    rust_isa_s390x_ve3_vgemq
);

binary!(
    /// `vdq`.
    ///
    /// Divide the signed 128-bit lanes.
    vdq,
    rust_isa_s390x_ve3_vdq
);
binary!(
    /// `vrq`.
    ///
    /// Remainder of the signed 128-bit lanes.
    vrq,
    rust_isa_s390x_ve3_vrq
);
binary!(
    /// `vdlq`.
    ///
    /// Divide the unsigned 128-bit lanes.
    vdlq,
    rust_isa_s390x_ve3_vdlq
);
binary!(
    /// `vrlq`.
    ///
    /// Remainder of the unsigned 128-bit lanes.
    vrlq,
    rust_isa_s390x_ve3_vrlq
);

binary!(
    /// `vdf`.
    ///
    /// Divide the signed 32-bit lanes.
    vdf,
    rust_isa_s390x_ve3_vdf
);
binary!(
    /// `vdg`.
    ///
    /// Divide the signed 64-bit lanes.
    vdg,
    rust_isa_s390x_ve3_vdg
);
binary!(
    /// `vdlf`.
    ///
    /// Divide the unsigned 32-bit lanes.
    vdlf,
    rust_isa_s390x_ve3_vdlf
);
binary!(
    /// `vdlg`.
    ///
    /// Divide the unsigned 64-bit lanes.
    vdlg,
    rust_isa_s390x_ve3_vdlg
);
binary!(
    /// `vrf`.
    ///
    /// Remainder of the signed 32-bit lanes.
    vrf,
    rust_isa_s390x_ve3_vrf
);
binary!(
    /// `vrg`.
    ///
    /// Remainder of the signed 64-bit lanes.
    vrg,
    rust_isa_s390x_ve3_vrg
);
binary!(
    /// `vrlf`.
    ///
    /// Remainder of the unsigned 32-bit lanes.
    vrlf,
    rust_isa_s390x_ve3_vrlf
);
binary!(
    /// `vrlg`.
    ///
    /// Remainder of the unsigned 64-bit lanes.
    vrlg,
    rust_isa_s390x_ve3_vrlg
);

/// `vblendq`.
///
/// Select the 128-bit lane. The mask selects `b` where its sign bit is set
/// and `a` otherwise.
///
/// # Safety
///
/// The caller must enable the vector-enhancements-3 facility.
#[target_feature(enable = "vector-enhancements-3")]
#[inline]
pub unsafe fn vblendq(a: V128, b: V128, mask: V128) -> V128 {
    let a = u128::from_be_bytes(a.to_bytes());
    let b = u128::from_be_bytes(b.to_bytes());
    let mask = u128::from_be_bytes(mask.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vblendq {out}, {b}, {a}, {mask}",
            out = lateout(vreg) out,
            a = in(vreg) a,
            b = in(vreg) b,
            mask = in(vreg) mask,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `veval`.
///
/// Evaluate the byte lanes with the immediate control value.
///
/// # Safety
///
/// The caller must enable the vector-enhancements-3 facility.
#[target_feature(enable = "vector-enhancements-3")]
#[inline]
pub unsafe fn veval<const IMM: u8>(a: V128, b: V128, c: V128) -> V128 {
    let a = u128::from_be_bytes(a.to_bytes());
    let b = u128::from_be_bytes(b.to_bytes());
    let c = u128::from_be_bytes(c.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "veval {out}, {a}, {b}, {c}, {imm}",
            out = lateout(vreg) out,
            a = in(vreg) a,
            b = in(vreg) b,
            c = in(vreg) c,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

ternary!(
    /// `vmalg`.
    ///
    /// Multiply the signed 64-bit lanes and add `acc`; the low 64 bits are kept.
    vmalg,
    rust_isa_s390x_ve3_vmalg
);
ternary!(
    /// `vmaeg`.
    ///
    /// Multiply the even 64-bit lanes and add `acc` into a 128-bit lane.
    vmaeg,
    rust_isa_s390x_ve3_vmaeg
);
ternary!(
    /// `vmaleg`.
    ///
    /// Multiply the even 64-bit lanes, add `acc`, and keep the low 128 bits.
    vmaleg,
    rust_isa_s390x_ve3_vmaleg
);
ternary!(
    /// `vmaog`.
    ///
    /// Multiply the odd 64-bit lanes and add `acc` into a 128-bit lane.
    vmaog,
    rust_isa_s390x_ve3_vmaog
);
ternary!(
    /// `vmalog`.
    ///
    /// Multiply the odd 64-bit lanes, add `acc`, and keep the low 128 bits.
    vmalog,
    rust_isa_s390x_ve3_vmalog
);
binary!(
    /// `vmhg`.
    ///
    /// High 64 bits of the signed 64-bit lane products.
    vmhg,
    rust_isa_s390x_ve3_vmhg
);
binary!(
    /// `vmhq`.
    ///
    /// High 128 bits of the signed 128-bit lane product.
    vmhq,
    rust_isa_s390x_ve3_vmhq
);
binary!(
    /// `vmlhg`.
    ///
    /// High 64 bits of the unsigned 64-bit lane products.
    vmlhg,
    rust_isa_s390x_ve3_vmlhg
);
binary!(
    /// `vmlhq`.
    ///
    /// High 128 bits of the unsigned 128-bit lane product.
    vmlhq,
    rust_isa_s390x_ve3_vmlhq
);
binary!(
    /// `vmlg`.
    ///
    /// Low 64 bits of the 64-bit lane products.
    vmlg,
    rust_isa_s390x_ve3_vmlg
);
binary!(
    /// `vmlq`.
    ///
    /// Low 128 bits of the 128-bit lane product.
    vmlq,
    rust_isa_s390x_ve3_vmlq
);
binary!(
    /// `vmeg`.
    ///
    /// Multiply the even 64-bit lanes with sign extension.
    vmeg,
    rust_isa_s390x_ve3_vmeg
);
binary!(
    /// `vmleg`.
    ///
    /// Multiply the even 64-bit lanes with zero extension.
    vmleg,
    rust_isa_s390x_ve3_vmleg
);
binary!(
    /// `vmog`.
    ///
    /// Multiply the odd 64-bit lanes with sign extension.
    vmog,
    rust_isa_s390x_ve3_vmog
);
binary!(
    /// `vmlog`.
    ///
    /// Multiply the odd 64-bit lanes with zero extension.
    vmlog,
    rust_isa_s390x_ve3_vmlog
);
binary!(
    /// `vaccq`.
    ///
    /// Add the 128-bit lanes and return the carry.
    vaccq,
    rust_isa_s390x_ve3_vaccq
);
ternary!(
    /// `vacq`.
    ///
    /// Add the 128-bit lanes with `acc` as the carry input.
    vacq,
    rust_isa_s390x_ve3_vacq
);
ternary!(
    /// `vacccq`.
    ///
    /// Add the 128-bit lanes with `acc` as the carry input and return the carry.
    vacccq,
    rust_isa_s390x_ve3_vacccq
);

ternary!(
    /// `vmahg`.
    ///
    /// High 64 bits of the signed 64-bit lane products, added to `acc`.
    vmahg,
    rust_isa_s390x_ve3_vmahg
);
ternary!(
    /// `vmalhg`.
    ///
    /// High 64 bits of the unsigned 64-bit lane products, added to `acc`.
    vmalhg,
    rust_isa_s390x_ve3_vmalhg
);
ternary!(
    /// `vmahq`.
    ///
    /// High 128 bits of the signed 128-bit lane product, added to `acc`.
    vmahq,
    rust_isa_s390x_ve3_vmahq
);
ternary!(
    /// `vmalhq`.
    ///
    /// High 128 bits of the unsigned 128-bit lane product, added to `acc`.
    vmalhq,
    rust_isa_s390x_ve3_vmalhq
);
