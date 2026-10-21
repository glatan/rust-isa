//! s390x vector enhancements.
//!
//! The vector-enhancement facilities are not part of the s390x baseline. Every
//! operation is `unsafe` and the caller must compile with the corresponding
//! enhancement enabled. The C file is compiled with `-march=z14`.
//!
//! The values are [`V128`]. `cntlz` and `popcnt` operate on the lane type in
//! the function name. The byte order is big-endian.

use super::vector::V128;

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
