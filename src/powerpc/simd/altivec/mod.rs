//! Altivec.
//!
//! Altivec and VSX are not part of the PowerPC baseline. The caller must
//! compile with `-C target-feature=+altivec,+vsx`. Every instruction wrapper is `unsafe`.
//! The values are [`V128`]. Immediate operands are const generics unless
//! documented otherwise.
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::powerpc::simd::altivec::add as _;
//! use rust_isa::powerpc::simd::altivec::add_i16 as _;
//! use rust_isa::powerpc::simd::altivec::add_i64 as _;
//! use rust_isa::powerpc::simd::altivec::add_i8 as _;
//! use rust_isa::powerpc::simd::altivec::and as _;
//! use rust_isa::powerpc::simd::altivec::cmpeq_i32 as _;
//! use rust_isa::powerpc::simd::altivec::cmpgt_i32 as _;
//! use rust_isa::powerpc::simd::altivec::fadd as _;
//! use rust_isa::powerpc::simd::altivec::fdiv as _;
//! use rust_isa::powerpc::simd::altivec::fmul as _;
//! use rust_isa::powerpc::simd::altivec::fsub as _;
//! use rust_isa::powerpc::simd::altivec::lvebx as _;
//! use rust_isa::powerpc::simd::altivec::lvehx as _;
//! use rust_isa::powerpc::simd::altivec::lvewx as _;
//! use rust_isa::powerpc::simd::altivec::lvx as _;
//! use rust_isa::powerpc::simd::altivec::mul as _;
//! use rust_isa::powerpc::simd::altivec::mul_i16 as _;
//! use rust_isa::powerpc::simd::altivec::mul_i32 as _;
//! use rust_isa::powerpc::simd::altivec::or as _;
//! use rust_isa::powerpc::simd::altivec::permute as _;
//! use rust_isa::powerpc::simd::altivec::splat_i32 as _;
//! use rust_isa::powerpc::simd::altivec::stvebx as _;
//! use rust_isa::powerpc::simd::altivec::stvehx as _;
//! use rust_isa::powerpc::simd::altivec::stvewx as _;
//! use rust_isa::powerpc::simd::altivec::stvx as _;
//! use rust_isa::powerpc::simd::altivec::sub_i16 as _;
//! use rust_isa::powerpc::simd::altivec::sub_i32 as _;
//! use rust_isa::powerpc::simd::altivec::sub_i64 as _;
//! use rust_isa::powerpc::simd::altivec::sub_i8 as _;
//! use rust_isa::powerpc::simd::altivec::v128_load as _;
//! use rust_isa::powerpc::simd::altivec::v128_store as _;
//! use rust_isa::powerpc::simd::altivec::vaddubm as _;
//! use rust_isa::powerpc::simd::altivec::vaddudm as _;
//! use rust_isa::powerpc::simd::altivec::vadduhm as _;
//! use rust_isa::powerpc::simd::altivec::vadduwm as _;
//! use rust_isa::powerpc::simd::altivec::vand as _;
//! use rust_isa::powerpc::simd::altivec::vandc as _;
//! use rust_isa::powerpc::simd::altivec::vavg as _;
//! use rust_isa::powerpc::simd::altivec::vavgl as _;
//! use rust_isa::powerpc::simd::altivec::vcmp as _;
//! use rust_isa::powerpc::simd::altivec::vcmpeq as _;
//! use rust_isa::powerpc::simd::altivec::vcmpgt as _;
//! use rust_isa::powerpc::simd::altivec::vmadd as _;
//! use rust_isa::powerpc::simd::altivec::vmax as _;
//! use rust_isa::powerpc::simd::altivec::vmin as _;
//! use rust_isa::powerpc::simd::altivec::vmla as _;
//! use rust_isa::powerpc::simd::altivec::vmls as _;
//! use rust_isa::powerpc::simd::altivec::vmsummbm as _;
//! use rust_isa::powerpc::simd::altivec::vmsumshm as _;
//! use rust_isa::powerpc::simd::altivec::vmsumshs as _;
//! use rust_isa::powerpc::simd::altivec::vmsumubm as _;
//! use rust_isa::powerpc::simd::altivec::vmsumuhm as _;
//! use rust_isa::powerpc::simd::altivec::vmsumuhs as _;
//! use rust_isa::powerpc::simd::altivec::vmul as _;
//! use rust_isa::powerpc::simd::altivec::vmuleub as _;
//! use rust_isa::powerpc::simd::altivec::vmuleuh as _;
//! use rust_isa::powerpc::simd::altivec::vmuleuw as _;
//! use rust_isa::powerpc::simd::altivec::vmuloub as _;
//! use rust_isa::powerpc::simd::altivec::vmulouh as _;
//! use rust_isa::powerpc::simd::altivec::vmulouw as _;
//! use rust_isa::powerpc::simd::altivec::vmuluwm as _;
//! use rust_isa::powerpc::simd::altivec::vnmsub as _;
//! use rust_isa::powerpc::simd::altivec::vnor as _;
//! use rust_isa::powerpc::simd::altivec::vnot as _;
//! use rust_isa::powerpc::simd::altivec::vor as _;
//! use rust_isa::powerpc::simd::altivec::vperm as _;
//! use rust_isa::powerpc::simd::altivec::vpk as _;
//! use rust_isa::powerpc::simd::altivec::vsel as _;
//! use rust_isa::powerpc::simd::altivec::vsl as _;
//! use rust_isa::powerpc::simd::altivec::vsldoi as _;
//! use rust_isa::powerpc::simd::altivec::vslo as _;
//! use rust_isa::powerpc::simd::altivec::vsr as _;
//! use rust_isa::powerpc::simd::altivec::vsro as _;
//! use rust_isa::powerpc::simd::altivec::vsububm as _;
//! use rust_isa::powerpc::simd::altivec::vsubudm as _;
//! use rust_isa::powerpc::simd::altivec::vsubuhm as _;
//! use rust_isa::powerpc::simd::altivec::vsubuwm as _;
//! use rust_isa::powerpc::simd::altivec::vsum2sws as _;
//! use rust_isa::powerpc::simd::altivec::vsum4sbs as _;
//! use rust_isa::powerpc::simd::altivec::vsum4shs as _;
//! use rust_isa::powerpc::simd::altivec::vsum4ubs as _;
//! use rust_isa::powerpc::simd::altivec::vsumsws as _;
//! use rust_isa::powerpc::simd::altivec::vupk as _;
//! use rust_isa::powerpc::simd::altivec::vxor as _;
//! use rust_isa::powerpc::simd::altivec::xor as _;
//! ```
//! <!-- doctest-inventory:end -->

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_altivec_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadduwm`
///
/// Add the four 32-bit integer lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::powerpc::simd::altivec::{add, V128};
///
/// let a = V128::from_bytes([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
/// let b = V128::from_bytes([2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0]);
/// let sum = unsafe { add(a, b) };
/// assert_eq!(
///     sum.to_bytes(),
///     [3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0]
/// );
/// ```
///
/// # Safety
///
/// The caller must compile with Altivec and VSX enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuluwm`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with Altivec and VSX enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_altivec_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_permute(a: *const u8, b: *const u8, index: *const u8, out: *mut u8);
    fn rust_isa_altivec_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with Altivec and VSX enabled.
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
    /// `vaddubm` on 8-bit lanes.
    add_i8,
    rust_isa_altivec_add_i8
);
binary!(
    /// `vadduhm` on 16-bit lanes.
    add_i16,
    rust_isa_altivec_add_i16
);
binary!(
    /// `vaddudm` (Power8) on 64-bit lanes.
    add_i64,
    rust_isa_altivec_add_i64
);
binary!(
    /// `vsububm` on 8-bit lanes.
    sub_i8,
    rust_isa_altivec_sub_i8
);
binary!(
    /// `vsubuhm` on 16-bit lanes.
    sub_i16,
    rust_isa_altivec_sub_i16
);
binary!(
    /// `vsubuwm` on 32-bit lanes.
    sub_i32,
    rust_isa_altivec_sub_i32
);
binary!(
    /// `vsubudm` (Power8) on 64-bit lanes.
    sub_i64,
    rust_isa_altivec_sub_i64
);
binary!(
    /// `vmladduhm` on 16-bit lanes.
    mul_i16,
    rust_isa_altivec_mul_i16
);
binary!(
    /// `vmuluwm` on 32-bit lanes.
    mul_i32,
    rust_isa_altivec_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_altivec_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_altivec_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_altivec_xor
);
binary!(
    /// `vcmpequw` on 32-bit lanes. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_altivec_cmpeq_i32
);
binary!(
    /// `vcmpgtsw` on signed 32-bit lanes. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_altivec_cmpgt_i32
);
binary!(
    /// Vector float add.
    fadd,
    rust_isa_altivec_fadd
);
binary!(
    /// Vector float subtract.
    fsub,
    rust_isa_altivec_fsub
);
binary!(
    /// Vector float multiply.
    fmul,
    rust_isa_altivec_fmul
);
binary!(
    /// Vector float divide.
    fdiv,
    rust_isa_altivec_fdiv
);

/// `vperm`
///
/// Permute bytes from the 32-byte concatenation of `a` and `b`. Each index
/// byte selects one of the 32 input bytes.
///
/// # Safety
///
/// The caller must enable Altivec.
#[inline(always)]
pub unsafe fn permute(a: V128, b: V128, index: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_permute(a.as_ptr(), b.as_ptr(), index.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// Broadcast a 32-bit value to all four lanes.
///
/// # Safety
///
/// The caller must enable Altivec.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_splat_i32(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_altivec_vaddubm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vadduhm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vadduwm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vaddudm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsububm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsubuhm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsubuwm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsubudm(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vand(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vxor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vandc(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vnor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmuleub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmuleuh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmuleuw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmuloub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmulouh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmulouw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vavg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vavgl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vcmpeq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vcmpgt(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vcmp(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsr(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vslo(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsro(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vnot(a: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsel(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vperm(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsldoi(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
}

macro_rules! abin {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with Altivec and VSX enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! aun {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with Altivec and VSX enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! atern {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with Altivec and VSX enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
abin!(
    /// `vaddubm`.
    vaddubm,
    rust_isa_altivec_vaddubm
);
abin!(
    /// `vadduhm`.
    vadduhm,
    rust_isa_altivec_vadduhm
);
abin!(
    /// `vadduwm`.
    vadduwm,
    rust_isa_altivec_vadduwm
);
abin!(
    /// `vaddudm`.
    vaddudm,
    rust_isa_altivec_vaddudm
);
abin!(
    /// `vsububm`.
    vsububm,
    rust_isa_altivec_vsububm
);
abin!(
    /// `vsubuhm`.
    vsubuhm,
    rust_isa_altivec_vsubuhm
);
abin!(
    /// `vsubuwm`.
    vsubuwm,
    rust_isa_altivec_vsubuwm
);
abin!(
    /// `vsubudm`.
    vsubudm,
    rust_isa_altivec_vsubudm
);
abin!(
    /// `vand`.
    vand,
    rust_isa_altivec_vand
);
abin!(
    /// `vor`.
    vor,
    rust_isa_altivec_vor
);
abin!(
    /// `vxor`.
    vxor,
    rust_isa_altivec_vxor
);
abin!(
    /// `vandc`.
    vandc,
    rust_isa_altivec_vandc
);
abin!(
    /// `vnor`.
    vnor,
    rust_isa_altivec_vnor
);
abin!(
    /// `vmuleub`.
    vmuleub,
    rust_isa_altivec_vmuleub
);
abin!(
    /// `vmuleuh`.
    vmuleuh,
    rust_isa_altivec_vmuleuh
);
abin!(
    /// `vmuleuw`.
    vmuleuw,
    rust_isa_altivec_vmuleuw
);
abin!(
    /// `vmuloub`.
    vmuloub,
    rust_isa_altivec_vmuloub
);
abin!(
    /// `vmulouh`.
    vmulouh,
    rust_isa_altivec_vmulouh
);
abin!(
    /// `vmulouw`.
    vmulouw,
    rust_isa_altivec_vmulouw
);
abin!(
    /// `vminuw`.
    vmin,
    rust_isa_altivec_vmin
);
abin!(
    /// `vmaxuw`.
    vmax,
    rust_isa_altivec_vmax
);
abin!(
    /// `vavgub`.
    vavg,
    rust_isa_altivec_vavg
);
abin!(
    /// `vavguh`.
    vavgl,
    rust_isa_altivec_vavgl
);
abin!(
    /// `vcmpequw`.
    vcmpeq,
    rust_isa_altivec_vcmpeq
);
abin!(
    /// `vcmpgtsw`.
    vcmpgt,
    rust_isa_altivec_vcmpgt
);
abin!(
    /// `vcmpgtsw`.
    vcmp,
    rust_isa_altivec_vcmp
);
abin!(
    /// `vsl`.
    vsl,
    rust_isa_altivec_vsl
);
abin!(
    /// `vsr`.
    vsr,
    rust_isa_altivec_vsr
);
abin!(
    /// `vslo`.
    vslo,
    rust_isa_altivec_vslo
);
abin!(
    /// `vsro`.
    vsro,
    rust_isa_altivec_vsro
);
aun!(
    /// `vnot`.
    vnot,
    rust_isa_altivec_vnot
);
atern!(
    /// `vsel`.
    vsel,
    rust_isa_altivec_vsel
);
atern!(
    /// `vperm`.
    vperm,
    rust_isa_altivec_vperm
);

/// `vsldoi` with an immediate shift count.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[inline(always)]
pub unsafe fn vsldoi<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 15, "vsldoi immediate must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_vsldoi(a.as_ptr(), b.as_ptr(), u32::from(IMM), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_altivec_lvx(p: *const u8, out: *mut u8);
    fn rust_isa_altivec_stvx(p: *mut u8, a: *const u8);
    fn rust_isa_altivec_vmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmla(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmls(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmadd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vnmsub(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vpk(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_altivec_vupk(a: *const u8, out: *mut u8);
}

/// `lvx` load.
///
/// # Safety
///
/// `p` must be valid for reading 16 bytes.
#[inline(always)]
pub unsafe fn lvx(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_altivec_lvx(p, out.as_mut_ptr());
    }
    out
}

/// `stvx` store.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes.
#[inline(always)]
pub unsafe fn stvx(p: *mut u8, a: V128) {
    unsafe {
        rust_isa_altivec_stvx(p, a.as_ptr());
    }
}

abin!(
    /// `vmladduhm`.
    vmul,
    rust_isa_altivec_vmul
);
abin!(
    /// `vpkuwum`.
    vpk,
    rust_isa_altivec_vpk
);
aun!(
    /// `vupklsh`.
    vupk,
    rust_isa_altivec_vupk
);

macro_rules! afma {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with Altivec and VSX enabled.
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

afma!(
    /// `xvmaddasp`.
    vmla,
    rust_isa_altivec_vmla
);
afma!(
    /// `xvmsubasp`.
    vmls,
    rust_isa_altivec_vmls
);
afma!(
    /// `xvmaddasp`.
    vmadd,
    rust_isa_altivec_vmadd
);
afma!(
    /// `xvnmsubasp`.
    vnmsub,
    rust_isa_altivec_vnmsub
);

/// `vmuluwm`.
///
/// # Safety
///
/// The caller must compile with Altivec and VSX enabled.
#[inline(always)]
pub unsafe fn vmuluwm(a: V128, b: V128) -> V128 {
    unsafe { mul_i32(a, b) }
}

unsafe extern "C" {
    fn rust_isa_altivec_lvebx(p: *const u8, index: u32, out: *mut u8);
    fn rust_isa_altivec_lvehx(p: *const u8, index: u32, out: *mut u8);
    fn rust_isa_altivec_lvewx(p: *const u8, index: u32, out: *mut u8);
    fn rust_isa_altivec_stvebx(p: *mut u8, index: u32, a: *const u8);
    fn rust_isa_altivec_stvehx(p: *mut u8, index: u32, a: *const u8);
    fn rust_isa_altivec_stvewx(p: *mut u8, index: u32, a: *const u8);
}

macro_rules! lve {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// `p` must be valid for reading the selected element.
        #[inline(always)]
        pub unsafe fn $name(p: *const u8, index: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(p, index, out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! stve {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// `p` must be valid for writing the selected element.
        #[inline(always)]
        pub unsafe fn $name(p: *mut u8, index: u32, a: V128) {
            unsafe { $c(p, index, a.as_ptr()); }
        }
    };
}
lve!(
    /// `lvebx`.
    lvebx,
    rust_isa_altivec_lvebx
);
lve!(
    /// `lvehx`.
    lvehx,
    rust_isa_altivec_lvehx
);
lve!(
    /// `lvewx`.
    lvewx,
    rust_isa_altivec_lvewx
);
stve!(
    /// `stvebx`.
    stvebx,
    rust_isa_altivec_stvebx
);
stve!(
    /// `stvehx`.
    stvehx,
    rust_isa_altivec_stvehx
);
stve!(
    /// `stvewx`.
    stvewx,
    rust_isa_altivec_stvewx
);

unsafe extern "C" {
    fn rust_isa_altivec_vmsumubm(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmsummbm(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmsumuhm(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmsumshm(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmsumuhs(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vmsumshs(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsum4ubs(a: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsum4sbs(a: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsum4shs(a: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsum2sws(a: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_altivec_vsumsws(a: *const u8, c: *const u8, out: *mut u8);
}

macro_rules! asum3 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with Altivec and VSX enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! asum2 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with Altivec and VSX enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), c.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
asum3!(
    /// `vmsumubm`.
    vmsumubm,
    rust_isa_altivec_vmsumubm
);
asum3!(
    /// `vmsummbm`.
    vmsummbm,
    rust_isa_altivec_vmsummbm
);
asum3!(
    /// `vmsumuhm`.
    vmsumuhm,
    rust_isa_altivec_vmsumuhm
);
asum3!(
    /// `vmsumshm`.
    vmsumshm,
    rust_isa_altivec_vmsumshm
);
asum3!(
    /// `vmsumuhs`.
    vmsumuhs,
    rust_isa_altivec_vmsumuhs
);
asum3!(
    /// `vmsumshs`.
    vmsumshs,
    rust_isa_altivec_vmsumshs
);
asum2!(
    /// `vsum4ubs`.
    vsum4ubs,
    rust_isa_altivec_vsum4ubs
);
asum2!(
    /// `vsum4sbs`.
    vsum4sbs,
    rust_isa_altivec_vsum4sbs
);
asum2!(
    /// `vsum4shs`.
    vsum4shs,
    rust_isa_altivec_vsum4shs
);
asum2!(
    /// `vsum2sws`.
    vsum2sws,
    rust_isa_altivec_vsum2sws
);
asum2!(
    /// `vsumsws`.
    vsumsws,
    rust_isa_altivec_vsumsws
);
