//! PowerPC Power9 vector instructions.
//!
//! Power9 vector instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the Power9 feature set. The
//! C file is compiled with `-mcpu=power9`.
//!
//! The values are [`V128`].
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).

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
/// The caller must enable the Power9 vector feature set.
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
/// The caller must enable the Power9 vector feature set.
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
/// The caller must enable the Power9 vector feature set.
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
        /// The caller must enable the Power9 vector feature set.
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
        /// The caller must enable the Power9 vector feature set.
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
    /// `vabsd`
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
/// The caller must enable the Power9 vector feature set.
#[inline(always)]
pub unsafe fn vctzlsbb(a: V128) -> u32 {
    unsafe { rust_isa_power9_vector_vctzlsbb(a.as_ptr()) }
}
