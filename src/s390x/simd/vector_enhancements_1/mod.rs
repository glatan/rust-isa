//! s390x vector enhancements facility 1.
//!
//! The vector-enhancement facilities are not part of the s390x baseline. Every
//! operation is `unsafe` and the caller must compile with the corresponding
//! enhancement enabled. The C file is compiled with clang and `-march=z14`. The
//! instructions without a clang intrinsic use `asm!` with the
//! `vector-enhancements-1` target feature.
//!
//! The values are [`V128`]. `cntlz` and `popcnt` operate on the lane type in
//! the function name. Immediate operands are const generics unless documented
//! otherwise. The byte order is big-endian.
//!
//! Instruction behavior follows the [IBM z/Architecture Principles of Operation](https://www.ibm.com/docs/en/module_1678991624569/pdf/SA22-7832-14.pdf).
//!
//! Instructions not listed in this module are not exposed.

use super::vector::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_s390x_enh_cntlz_i32(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_popcnt_i8(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_popcnt_i32(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_fma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vclzf` on 32-bit lanes.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[inline(always)]
pub unsafe fn cntlz_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_enh_cntlz_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpopct` on byte lanes.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[inline(always)]
pub unsafe fn popcnt_i8(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_enh_popcnt_i8(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpopctf` on 32-bit lanes.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[inline(always)]
pub unsafe fn popcnt_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_enh_popcnt_i32(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfma` on binary floating-point lanes.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[inline(always)]
pub unsafe fn fma(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_enh_fma(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_s390x_enh_vfa(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfs(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfms(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfnma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfnms(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfsq(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfce(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfch(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vfche(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vclz(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vctz(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_enh_vpopct(a: *const u8, out: *mut u8);
}

macro_rules! bin_enh {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancement facility.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! un_enh {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancement facility.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! tern_enh {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancement facility.
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

bin_enh!(
    /// `vfa`.
    vfa,
    rust_isa_s390x_enh_vfa
);
bin_enh!(
    /// `vfs`.
    vfs,
    rust_isa_s390x_enh_vfs
);
bin_enh!(
    /// `vfm`.
    vfm,
    rust_isa_s390x_enh_vfm
);
bin_enh!(
    /// `vfd`.
    vfd,
    rust_isa_s390x_enh_vfd
);
tern_enh!(
    /// `vfma`.
    vfma,
    rust_isa_s390x_enh_vfma
);
tern_enh!(
    /// `vfms`.
    vfms,
    rust_isa_s390x_enh_vfms
);
tern_enh!(
    /// `vfnma`.
    vfnma,
    rust_isa_s390x_enh_vfnma
);
tern_enh!(
    /// `vfnms`.
    vfnms,
    rust_isa_s390x_enh_vfnms
);
un_enh!(
    /// `vfsq`.
    vfsq,
    rust_isa_s390x_enh_vfsq
);
bin_enh!(
    /// `vfce`.
    vfce,
    rust_isa_s390x_enh_vfce
);
bin_enh!(
    /// `vfch`.
    vfch,
    rust_isa_s390x_enh_vfch
);
bin_enh!(
    /// `vfche`.
    vfche,
    rust_isa_s390x_enh_vfche
);
un_enh!(
    /// `vclz`.
    vclz,
    rust_isa_s390x_enh_vclz
);
un_enh!(
    /// `vctz`.
    vctz,
    rust_isa_s390x_enh_vctz
);
un_enh!(
    /// `vpopct`.
    vpopct,
    rust_isa_s390x_enh_vpopct
);

/// `vftci` on single-precision lanes.
///
/// Test each binary floating-point element against the 12-bit data class
/// mask `MASK` and set a true lane to all bits one. The condition code is
/// not returned.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[target_feature(enable = "vector-enhancements-1")]
#[inline]
pub unsafe fn vftci_f32<const MASK: u16>(a: V128) -> V128 {
    const { assert!(MASK <= 0x0fff, "vftci class mask must be in 0..=4095") };
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vftcisb {out}, {input}, {mask}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            mask = const MASK,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vfi` on single-precision lanes.
///
/// Round each binary floating-point element to an integral
/// floating-point value using rounding mode `M5`. `M4` must have bits 0-1
/// clear; bit 2 suppresses the inexact exception. The condition code is
/// not returned.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[target_feature(enable = "vector-enhancements-1")]
#[inline]
pub unsafe fn vfi_f32<const M4: u8, const M5: u8>(a: V128) -> V128 {
    const {
        assert!(M4 <= 0x0f, "vfi M4 must be in 0..=15");
        assert!(M4 & 0x03 == 0, "vfi M4 bits 0-1 must be zero");
        assert!(M5 <= 0x0f, "vfi M5 must be in 0..=15");
    }
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vfisb {out}, {input}, {m4}, {m5}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            m4 = const M4,
            m5 = const M5,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vfll` on single-precision lanes.
///
/// Convert the even-numbered single-precision elements to
/// double-precision elements. Element 0 and element 2 become element 0
/// and element 1 of the result.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[target_feature(enable = "vector-enhancements-1")]
#[inline]
pub unsafe fn vfll(a: V128) -> V128 {
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vflls {out}, {input}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vflr`.
///
/// Convert the double-precision elements to single-precision elements
/// using rounding mode `M5`. Element 0 and element 1 become element 0 and
/// element 2; the other elements are zero. `M4` must have bits 0-1 clear;
/// bit 2 suppresses the inexact exception. The condition code is not
/// returned.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[target_feature(enable = "vector-enhancements-1")]
#[inline]
pub unsafe fn vflr<const M4: u8, const M5: u8>(a: V128) -> V128 {
    const {
        assert!(M4 <= 0x0f, "vflr M4 must be in 0..=15");
        assert!(M4 & 0x03 == 0, "vflr M4 bits 0-1 must be zero");
        assert!(M5 <= 0x0f, "vflr M5 must be in 0..=15");
    }
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vflrd {out}, {input}, {m4}, {m5}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            m4 = const M4,
            m5 = const M5,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vnn`.
///
/// Bitwise NAND of `a` and `b`.
///
/// # Safety
///
/// The caller must enable the vector-enhancement facility.
#[target_feature(enable = "vector-enhancements-1")]
#[inline]
pub unsafe fn vnn(a: V128, b: V128) -> V128 {
    let a = u128::from_be_bytes(a.to_bytes());
    let b = u128::from_be_bytes(b.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vnn {out}, {a}, {b}",
            out = lateout(vreg) out,
            a = in(vreg) a,
            b = in(vreg) b,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}
