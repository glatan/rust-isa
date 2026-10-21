//! s390x vector facility.
//!
//! The lane order is big-endian.

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_s390x_vector_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// Vector integer add.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// Vector integer multiply.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_s390x_vector_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_permute(a: *const u8, b: *const u8, index: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[target_feature(enable = "vector")]
        #[inline]
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
    /// Byte-lane add (`vab`).
    add_i8,
    rust_isa_s390x_vector_add_i8
);
binary!(
    /// Halfword-lane add (`vah`).
    add_i16,
    rust_isa_s390x_vector_add_i16
);
binary!(
    /// Doubleword-lane add (`vag`).
    add_i64,
    rust_isa_s390x_vector_add_i64
);
binary!(
    /// Byte-lane subtract (`vsb`).
    sub_i8,
    rust_isa_s390x_vector_sub_i8
);
binary!(
    /// Halfword-lane subtract (`vsh`).
    sub_i16,
    rust_isa_s390x_vector_sub_i16
);
binary!(
    /// Word-lane subtract (`vsf`).
    sub_i32,
    rust_isa_s390x_vector_sub_i32
);
binary!(
    /// Doubleword-lane subtract (`vsg`).
    sub_i64,
    rust_isa_s390x_vector_sub_i64
);
binary!(
    /// Halfword-lane multiply, keeping the low 16 bits.
    mul_i16,
    rust_isa_s390x_vector_mul_i16
);
binary!(
    /// Word-lane multiply, keeping the low 32 bits.
    mul_i32,
    rust_isa_s390x_vector_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_s390x_vector_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_s390x_vector_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_s390x_vector_xor
);
binary!(
    /// Word-lane equality. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_s390x_vector_cmpeq_i32
);
binary!(
    /// Signed word-lane greater-than. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_s390x_vector_cmpgt_i32
);
binary!(
    /// Binary floating-point add (`vfa`).
    fadd,
    rust_isa_s390x_vector_fadd
);
binary!(
    /// Binary floating-point subtract (`vfs`).
    fsub,
    rust_isa_s390x_vector_fsub
);
binary!(
    /// Binary floating-point multiply (`vfm`).
    fmul,
    rust_isa_s390x_vector_fmul
);
binary!(
    /// Binary floating-point divide (`vfd`).
    fdiv,
    rust_isa_s390x_vector_fdiv
);

/// `vperm`
///
/// Permute bytes from the 32-byte concatenation of `a` and `b`. Each index
/// byte selects one of the 32 input bytes; index bit 7 selects `b`.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn permute(a: V128, b: V128, index: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_permute(a.as_ptr(), b.as_ptr(), index.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// Broadcast a 32-bit value to all four lanes.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_splat_i32(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_s390x_vector_vab(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vah(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vaf(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vag(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vsh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vsf(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vsg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vavg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vavgl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vceq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vch(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vcl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vsel(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vsl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vsr(a: *const u8, b: *const u8, out: *mut u8);
}

binary!(
    /// `vab`.
    vab,
    rust_isa_s390x_vector_vab
);
binary!(
    /// `vah`.
    vah,
    rust_isa_s390x_vector_vah
);
binary!(
    /// `vaf`.
    vaf,
    rust_isa_s390x_vector_vaf
);
binary!(
    /// `vag`.
    vag,
    rust_isa_s390x_vector_vag
);
binary!(
    /// `vsb`.
    vsb,
    rust_isa_s390x_vector_vsb
);
binary!(
    /// `vsh`.
    vsh,
    rust_isa_s390x_vector_vsh
);
binary!(
    /// `vsf`.
    vsf,
    rust_isa_s390x_vector_vsf
);
binary!(
    /// `vsg`.
    vsg,
    rust_isa_s390x_vector_vsg
);
binary!(
    /// `vmin`.
    vmin,
    rust_isa_s390x_vector_vmin
);
binary!(
    /// `vmax`.
    vmax,
    rust_isa_s390x_vector_vmax
);
binary!(
    /// `vavg`.
    vavg,
    rust_isa_s390x_vector_vavg
);
binary!(
    /// `vavgl`.
    vavgl,
    rust_isa_s390x_vector_vavgl
);
binary!(
    /// `vceq`.
    vceq,
    rust_isa_s390x_vector_vceq
);
binary!(
    /// `vch`.
    vch,
    rust_isa_s390x_vector_vch
);
binary!(
    /// `vcl`.
    vcl,
    rust_isa_s390x_vector_vcl
);
binary!(
    /// `vsl`.
    vsl,
    rust_isa_s390x_vector_vsl
);
binary!(
    /// `vsr`.
    vsr,
    rust_isa_s390x_vector_vsr
);

/// `vsel`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vsel(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_vsel(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_s390x_vector_vclf(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vclg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vclh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vchl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vpks(a: *const u8, b: *const u8, out: *mut u8);
}

binary!(
    /// `vclf`.
    vclf,
    rust_isa_s390x_vector_vclf
);
binary!(
    /// `vclg`.
    vclg,
    rust_isa_s390x_vector_vclg
);
binary!(
    /// `vclh`.
    vclh,
    rust_isa_s390x_vector_vclh
);
binary!(
    /// `vchl`.
    vchl,
    rust_isa_s390x_vector_vchl
);
binary!(
    /// `vpks`.
    vpks,
    rust_isa_s390x_vector_vpks
);

/// `vperm`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vperm(a: V128, b: V128, index: V128) -> V128 {
    unsafe { permute(a, b, index) }
}

/// `vle`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vle(p: *const u8) -> V128 {
    unsafe { v128_load(p) }
}

/// `vst`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vst(p: *mut u8, value: V128) {
    unsafe { v128_store(p, value) }
}

unsafe extern "C" {
    fn rust_isa_s390x_vector_vcksm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vme(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vmh(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vml(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vmo(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vpkls(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vlef(p: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vstef(p: *mut u8, a: *const u8);
}

binary!(
    /// `vcksm`.
    vcksm,
    rust_isa_s390x_vector_vcksm
);
binary!(
    /// `vpkls`.
    vpkls,
    rust_isa_s390x_vector_vpkls
);
macro_rules! tern_vec {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[target_feature(enable = "vector")]
        #[inline]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

tern_vec!(
    /// `vme`.
    vme,
    rust_isa_s390x_vector_vme
);
tern_vec!(
    /// `vmh`.
    vmh,
    rust_isa_s390x_vector_vmh
);
tern_vec!(
    /// `vml`.
    vml,
    rust_isa_s390x_vector_vml
);
tern_vec!(
    /// `vmo`.
    vmo,
    rust_isa_s390x_vector_vmo
);

/// `vlef`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vlef(p: *const f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_vlef(p.cast(), out.as_mut_ptr());
    }
    out
}

/// `vstef`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vstef(p: *mut f32, a: V128) {
    unsafe {
        rust_isa_s390x_vector_vstef(p.cast(), a.as_ptr());
    }
}

pub use super::vector_enhancements::{vfa, vfce, vfch, vfche, vfd, vfm, vfs, vfsq};

use core::arch::asm;

/// `vgbm`.
///
/// Generate a vector by expanding each of the 16 immediate bits to a byte.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vgbm<const IMM: u16>() -> V128 {
    let out: u128;
    unsafe {
        asm!(
            "vgbm {out}, {imm}",
            out = lateout(vreg) out,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vrepb`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vrepb<const INDEX: u8>(a: V128) -> V128 {
    const { assert!(INDEX <= 15, "vrepb index must be in 0..=15") };
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vrepb {out}, {input}, {index}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            index = const INDEX,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vreph`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vreph<const INDEX: u8>(a: V128) -> V128 {
    const { assert!(INDEX <= 7, "vreph index must be in 0..=7") };
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vreph {out}, {input}, {index}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            index = const INDEX,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vrepf`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vrepf<const INDEX: u8>(a: V128) -> V128 {
    const { assert!(INDEX <= 3, "vrepf index must be in 0..=3") };
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vrepf {out}, {input}, {index}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            index = const INDEX,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vrepg`.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vrepg<const INDEX: u8>(a: V128) -> V128 {
    const { assert!(INDEX <= 1, "vrepg index must be in 0..=1") };
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vrepg {out}, {input}, {index}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            index = const INDEX,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

unsafe extern "C" {
    fn rust_isa_s390x_vector_vuph(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vupl(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vuplh(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vupll(a: *const u8, out: *mut u8);
}
macro_rules! un_vec {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[target_feature(enable = "vector")]
        #[inline]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
un_vec!(
    /// `vuph`.
    vuph,
    rust_isa_s390x_vector_vuph
);
un_vec!(
    /// `vupl`.
    vupl,
    rust_isa_s390x_vector_vupl
);
un_vec!(
    /// `vuplh`.
    vuplh,
    rust_isa_s390x_vector_vuplh
);
un_vec!(
    /// `vupll`.
    vupll,
    rust_isa_s390x_vector_vupll
);
