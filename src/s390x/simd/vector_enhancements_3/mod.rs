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
//! facility.

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
