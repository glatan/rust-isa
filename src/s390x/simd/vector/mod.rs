//! s390x vector facility.
//!
//! The vector facility is not enabled by default. Every instruction wrapper is `unsafe`
//! and requires the `vector` target feature. The values are [`V128`]. Immediate
//! operands are const generics unless documented otherwise.
//!
//! The lane order is big-endian.
//!
//! # Examples
//!
//! ```
//! use rust_isa::s390x::simd::vector::{add, V128};
//!
//! let a = V128::from_bytes([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
//! let b = V128::from_bytes([2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0]);
//! let sum = unsafe { add(a, b) };
//! assert_eq!(
//!     sum.to_bytes(),
//!     [3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0]
//! );
//! ```
//!
//! Instruction behavior follows the [IBM z/Architecture Principles of Operation](https://www.ibm.com/docs/en/module_1678991624569/pdf/SA22-7832-14.pdf).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::s390x::simd::vector::*;
//! use rust_isa::s390x::simd::vector::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { add(v128, v128) };
//! let _ = unsafe { add_i16(v128, v128) };
//! let _ = unsafe { add_i64(v128, v128) };
//! let _ = unsafe { add_i8(v128, v128) };
//! let _ = unsafe { and(v128, v128) };
//! let _ = unsafe { cmpeq_i32(v128, v128) };
//! let _ = unsafe { cmpgt_i32(v128, v128) };
//! let _ = unsafe { cntlz_i32(v128) };
//! let _ = unsafe { fadd(v128, v128) };
//! let _ = unsafe { fdiv(v128, v128) };
//! let _ = unsafe { fmul(v128, v128) };
//! let _ = unsafe { fsub(v128, v128) };
//! let _ = unsafe { mul(v128, v128) };
//! let _ = unsafe { mul_i16(v128, v128) };
//! let _ = unsafe { mul_i32(v128, v128) };
//! let _ = unsafe { or(v128, v128) };
//! let _ = unsafe { permute(v128, v128, v128) };
//! let _ = unsafe { splat_i32(0) };
//! let _ = unsafe { sub_i16(v128, v128) };
//! let _ = unsafe { sub_i32(v128, v128) };
//! let _ = unsafe { sub_i64(v128, v128) };
//! let _ = unsafe { sub_i8(v128, v128) };
//! let _ = unsafe { vab(v128, v128) };
//! let _ = unsafe { vaf(v128, v128) };
//! let _ = unsafe { vag(v128, v128) };
//! let _ = unsafe { vah(v128, v128) };
//! let _ = unsafe { vavg(v128, v128) };
//! let _ = unsafe { vavgl(v128, v128) };
//! let _ = unsafe { vceq(v128, v128) };
//! let _ = unsafe { vch(v128, v128) };
//! let _ = unsafe { vchl(v128, v128) };
//! let _ = unsafe { vcksm(v128, v128) };
//! let _ = unsafe { vcl(v128, v128) };
//! let _ = unsafe { vclf(v128, v128) };
//! let _ = unsafe { vclg(v128, v128) };
//! let _ = unsafe { vclh(v128, v128) };
//! let _ = unsafe { vclzb(v128) };
//! let _ = unsafe { vclzg(v128) };
//! let _ = unsafe { vctzb(v128) };
//! let _ = unsafe { vctzg(v128) };
//! let _ = unsafe { vfa(v128, v128) };
//! let _ = unsafe { vfae::<0, 0>(v128, v128) };
//! let _ = unsafe { vfce(v128, v128) };
//! let _ = unsafe { vfch(v128, v128) };
//! let _ = unsafe { vfche(v128, v128) };
//! let _ = unsafe { vfd(v128, v128) };
//! let _ = unsafe { vfee::<0, 0>(v128, v128) };
//! let _ = unsafe { vfene::<0, 0>(v128, v128) };
//! let _ = unsafe { vfi_f32::<0, 0>(v128) };
//! let _ = unsafe { vfi_f64::<0, 0>(v128) };
//! let _ = unsafe { vfll(v128) };
//! let _ = unsafe { vflr::<0, 0>(v128) };
//! let _ = unsafe { vfm(v128, v128) };
//! let _ = unsafe { vfs(v128, v128) };
//! let _ = unsafe { vfsq(v128) };
//! let _ = unsafe { vftci_f32::<0>(v128) };
//! let _ = unsafe { vftci_f64::<0>(v128) };
//! let _ = unsafe { vgbm::<0>() };
//! let _ = unsafe { vistr::<0>(v128) };
//! let _ = unsafe { vlgv::<0, 0>(v128) };
//! let _ = unsafe { vmax(v128, v128) };
//! let _ = unsafe { vme(v128, v128, v128) };
//! let _ = unsafe { vmh(v128, v128, v128) };
//! let _ = unsafe { vmin(v128, v128) };
//! let _ = unsafe { vml(v128, v128, v128) };
//! let _ = unsafe { vmle::<0>(v128, v128) };
//! let _ = unsafe { vmlh::<0>(v128, v128) };
//! let _ = unsafe { vmlo::<0>(v128, v128) };
//! let _ = unsafe { vmo(v128, v128, v128) };
//! let _ = unsafe { vnn(v128, v128) };
//! let _ = unsafe { vperm(v128, v128, v128) };
//! let _ = unsafe { vpkls(v128, v128) };
//! let _ = unsafe { vpks(v128, v128) };
//! let _ = unsafe { vrep::<0, 0>(v128) };
//! let _ = unsafe { vrepb::<0>(v128) };
//! let _ = unsafe { vrepf::<0>(v128) };
//! let _ = unsafe { vrepg::<0>(v128) };
//! let _ = unsafe { vreph::<0>(v128) };
//! let _ = unsafe { vsb(v128, v128) };
//! let _ = unsafe { vscbi::<0>(v128, v128) };
//! let _ = unsafe { vseg::<0>(v128) };
//! let _ = unsafe { vsel(v128, v128, v128) };
//! let _ = unsafe { vsf(v128, v128) };
//! let _ = unsafe { vsg(v128, v128) };
//! let _ = unsafe { vsh(v128, v128) };
//! let _ = unsafe { vsl(v128, v128) };
//! let _ = unsafe { vsr(v128, v128) };
//! let _ = unsafe { vstrc::<0, 0>(v128, v128, v128) };
//! let _ = unsafe { vuph(v128) };
//! let _ = unsafe { vupl(v128) };
//! let _ = unsafe { vuplh(v128) };
//! let _ = unsafe { vupll(v128) };
//! let _ = unsafe { xor(v128, v128) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::s390x::simd::vector::*;
//! use rust_isa::s390x::simd::vector::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { v128_load(storage.0.as_ptr().cast()) };
//! let _ = unsafe { v128_store(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { vgef::<0>(v128, v128, storage.0.as_ptr().cast()) };
//! let _ = unsafe { vgeg::<0>(v128, v128, storage.0.as_ptr().cast()) };
//! let _ = unsafe { vle(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vlef(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vst(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { vstef(storage.0.as_mut_ptr().cast(), v128) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Instruction inventory</summary>
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::s390x::simd::vector::add as _;
//! use rust_isa::s390x::simd::vector::add_i16 as _;
//! use rust_isa::s390x::simd::vector::add_i64 as _;
//! use rust_isa::s390x::simd::vector::add_i8 as _;
//! use rust_isa::s390x::simd::vector::and as _;
//! use rust_isa::s390x::simd::vector::cmpeq_i32 as _;
//! use rust_isa::s390x::simd::vector::cmpgt_i32 as _;
//! use rust_isa::s390x::simd::vector::cntlz_i32 as _;
//! use rust_isa::s390x::simd::vector::fadd as _;
//! use rust_isa::s390x::simd::vector::fdiv as _;
//! use rust_isa::s390x::simd::vector::fmul as _;
//! use rust_isa::s390x::simd::vector::fsub as _;
//! use rust_isa::s390x::simd::vector::mul as _;
//! use rust_isa::s390x::simd::vector::mul_i16 as _;
//! use rust_isa::s390x::simd::vector::mul_i32 as _;
//! use rust_isa::s390x::simd::vector::or as _;
//! use rust_isa::s390x::simd::vector::permute as _;
//! use rust_isa::s390x::simd::vector::splat_i32 as _;
//! use rust_isa::s390x::simd::vector::sub_i16 as _;
//! use rust_isa::s390x::simd::vector::sub_i32 as _;
//! use rust_isa::s390x::simd::vector::sub_i64 as _;
//! use rust_isa::s390x::simd::vector::sub_i8 as _;
//! use rust_isa::s390x::simd::vector::v128_load as _;
//! use rust_isa::s390x::simd::vector::v128_store as _;
//! use rust_isa::s390x::simd::vector::vab as _;
//! use rust_isa::s390x::simd::vector::vaf as _;
//! use rust_isa::s390x::simd::vector::vag as _;
//! use rust_isa::s390x::simd::vector::vah as _;
//! use rust_isa::s390x::simd::vector::vavg as _;
//! use rust_isa::s390x::simd::vector::vavgl as _;
//! use rust_isa::s390x::simd::vector::vceq as _;
//! use rust_isa::s390x::simd::vector::vch as _;
//! use rust_isa::s390x::simd::vector::vchl as _;
//! use rust_isa::s390x::simd::vector::vcksm as _;
//! use rust_isa::s390x::simd::vector::vcl as _;
//! use rust_isa::s390x::simd::vector::vclf as _;
//! use rust_isa::s390x::simd::vector::vclg as _;
//! use rust_isa::s390x::simd::vector::vclh as _;
//! use rust_isa::s390x::simd::vector::vclzb as _;
//! use rust_isa::s390x::simd::vector::vclzg as _;
//! use rust_isa::s390x::simd::vector::vctzb as _;
//! use rust_isa::s390x::simd::vector::vctzg as _;
//! use rust_isa::s390x::simd::vector::vfa as _;
//! use rust_isa::s390x::simd::vector::vfae as _;
//! use rust_isa::s390x::simd::vector::vfce as _;
//! use rust_isa::s390x::simd::vector::vfch as _;
//! use rust_isa::s390x::simd::vector::vfche as _;
//! use rust_isa::s390x::simd::vector::vfd as _;
//! use rust_isa::s390x::simd::vector::vfee as _;
//! use rust_isa::s390x::simd::vector::vfene as _;
//! use rust_isa::s390x::simd::vector::vfi_f32 as _;
//! use rust_isa::s390x::simd::vector::vfi_f64 as _;
//! use rust_isa::s390x::simd::vector::vfll as _;
//! use rust_isa::s390x::simd::vector::vflr as _;
//! use rust_isa::s390x::simd::vector::vfm as _;
//! use rust_isa::s390x::simd::vector::vfs as _;
//! use rust_isa::s390x::simd::vector::vfsq as _;
//! use rust_isa::s390x::simd::vector::vftci_f32 as _;
//! use rust_isa::s390x::simd::vector::vftci_f64 as _;
//! use rust_isa::s390x::simd::vector::vgbm as _;
//! use rust_isa::s390x::simd::vector::vgef as _;
//! use rust_isa::s390x::simd::vector::vgeg as _;
//! use rust_isa::s390x::simd::vector::vistr as _;
//! use rust_isa::s390x::simd::vector::vle as _;
//! use rust_isa::s390x::simd::vector::vlef as _;
//! use rust_isa::s390x::simd::vector::vlgv as _;
//! use rust_isa::s390x::simd::vector::vmax as _;
//! use rust_isa::s390x::simd::vector::vme as _;
//! use rust_isa::s390x::simd::vector::vmh as _;
//! use rust_isa::s390x::simd::vector::vmin as _;
//! use rust_isa::s390x::simd::vector::vml as _;
//! use rust_isa::s390x::simd::vector::vmle as _;
//! use rust_isa::s390x::simd::vector::vmlh as _;
//! use rust_isa::s390x::simd::vector::vmlo as _;
//! use rust_isa::s390x::simd::vector::vmo as _;
//! use rust_isa::s390x::simd::vector::vnn as _;
//! use rust_isa::s390x::simd::vector::vperm as _;
//! use rust_isa::s390x::simd::vector::vpkls as _;
//! use rust_isa::s390x::simd::vector::vpks as _;
//! use rust_isa::s390x::simd::vector::vrep as _;
//! use rust_isa::s390x::simd::vector::vrepb as _;
//! use rust_isa::s390x::simd::vector::vrepf as _;
//! use rust_isa::s390x::simd::vector::vrepg as _;
//! use rust_isa::s390x::simd::vector::vreph as _;
//! use rust_isa::s390x::simd::vector::vsb as _;
//! use rust_isa::s390x::simd::vector::vscbi as _;
//! use rust_isa::s390x::simd::vector::vseg as _;
//! use rust_isa::s390x::simd::vector::vsel as _;
//! use rust_isa::s390x::simd::vector::vsf as _;
//! use rust_isa::s390x::simd::vector::vsg as _;
//! use rust_isa::s390x::simd::vector::vsh as _;
//! use rust_isa::s390x::simd::vector::vsl as _;
//! use rust_isa::s390x::simd::vector::vsr as _;
//! use rust_isa::s390x::simd::vector::vst as _;
//! use rust_isa::s390x::simd::vector::vstef as _;
//! use rust_isa::s390x::simd::vector::vstrc as _;
//! use rust_isa::s390x::simd::vector::vuph as _;
//! use rust_isa::s390x::simd::vector::vupl as _;
//! use rust_isa::s390x::simd::vector::vuplh as _;
//! use rust_isa::s390x::simd::vector::vupll as _;
//! use rust_isa::s390x::simd::vector::xor as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

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
    fn rust_isa_s390x_vector_vclzb(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_vctzb(a: *const u8, out: *mut u8);
    fn rust_isa_s390x_vector_cntlz_i32(a: *const u8, out: *mut u8);
}

/// `vclzb`.
///
/// Count the leading zero bits in each byte lane.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vclzb(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_vclzb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vctzb`.
///
/// Count the trailing zero bits in each byte lane.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vctzb(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_vctzb(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclzf` on 32-bit lanes.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn cntlz_i32(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_s390x_vector_cntlz_i32(a.as_ptr(), out.as_mut_ptr());
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
        ///
        /// # Safety
        ///
        /// The caller must enable the vector facility.
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
    /// `vmnf`.
    vmin,
    rust_isa_s390x_vector_vmin
);
binary!(
    /// `vmxf`.
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
    /// `vchf`.
    vcl,
    rust_isa_s390x_vector_vcl
);
binary!(
    /// `vsl`.
    vsl,
    rust_isa_s390x_vector_vsl
);
binary!(
    /// `vesrlvg`.
    vsr,
    rust_isa_s390x_vector_vsr
);

/// `vsel`.
///
/// Select each bit from `b` where the corresponding bit of `c` is one, and
/// from `a` where it is zero.
///
/// # Safety
///
/// The caller must enable the vector facility.
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
    /// Compare single-precision lanes with less-than.
    vclf,
    rust_isa_s390x_vector_vclf
);
binary!(
    /// Compare double-precision lanes with less-than.
    vclg,
    rust_isa_s390x_vector_vclg
);
binary!(
    /// `vchh`.
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
///
/// See [`permute`] for the index encoding.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vperm(a: V128, b: V128, index: V128) -> V128 {
    unsafe { permute(a, b, index) }
}

/// `vle`.
///
/// Load 16 bytes.
///
/// # Safety
///
/// The caller must enable the vector facility. `p` must be valid for reading
/// 16 bytes.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vle(p: *const u8) -> V128 {
    unsafe { v128_load(p) }
}

/// `vst`.
///
/// Store 16 bytes.
///
/// # Safety
///
/// The caller must enable the vector facility. `p` must be valid for writing
/// 16 bytes.
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
        ///
        /// # Safety
        ///
        /// The caller must enable the vector facility.
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
///
/// Load 16 bytes into the four 32-bit lanes.
///
/// # Safety
///
/// The caller must enable the vector facility. `p` must be valid for reading
/// 16 bytes.
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
///
/// Store the four 32-bit lanes as 16 bytes.
///
/// # Safety
///
/// The caller must enable the vector facility. `p` must be valid for writing
/// 16 bytes.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vstef(p: *mut f32, a: V128) {
    unsafe {
        rust_isa_s390x_vector_vstef(p.cast(), a.as_ptr());
    }
}

pub use super::vector_enhancements_1::{
    vfa, vfce, vfch, vfche, vfd, vfi_f32, vfll, vflr, vfm, vfs, vfsq, vftci_f32, vnn,
};

use core::arch::asm;

/// `vgbm`.
///
/// Generate a vector by expanding each of the 16 immediate bits to a byte.
///
/// # Safety
///
/// The caller must enable the vector facility.
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
///
/// Broadcast byte `INDEX` to all 16 byte lanes. `INDEX` must be in `0..=15`.
///
/// # Safety
///
/// The caller must enable the vector facility.
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
///
/// Broadcast 16-bit lane `INDEX` to all eight 16-bit lanes. `INDEX` must be in
/// `0..=7`.
///
/// # Safety
///
/// The caller must enable the vector facility.
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
///
/// Broadcast 32-bit lane `INDEX` to all four 32-bit lanes. `INDEX` must be in
/// `0..=3`.
///
/// # Safety
///
/// The caller must enable the vector facility.
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
///
/// Broadcast 64-bit lane `INDEX` to both 64-bit lanes. `INDEX` must be in
/// `0..=1`.
///
/// # Safety
///
/// The caller must enable the vector facility.
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
        ///
        /// # Safety
        ///
        /// The caller must enable the vector facility.
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

/// `vrep`.
///
/// Replicate one element of `a` to every element. `SIZE` selects the
/// element size: 0 is byte, 1 is halfword, 2 is word, and 3 is doubleword.
/// `INDEX` selects the source element in that size.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vrep<const SIZE: u8, const INDEX: u8>(a: V128) -> V128 {
    const {
        assert!(SIZE <= 3, "vrep element size must be in 0..=3");
        assert!(
            (INDEX as u16) < (16u16 >> SIZE),
            "vrep element index is out of range"
        );
    }
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vrep {out}, {input}, {index}, {size}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            index = const INDEX,
            size = const SIZE,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vseg`.
///
/// Sign-extend the rightmost two elements of `a` to doublewords. `SIZE`
/// selects the source element size: 0 is byte, 1 is halfword, and 2 is
/// word.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vseg<const SIZE: u8>(a: V128) -> V128 {
    const { assert!(SIZE <= 2, "vseg element size must be in 0..=2") };
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vseg {out}, {input}, {size}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            size = const SIZE,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

macro_rules! vmul_lane {
    ($(#[$meta:meta])* $name:ident, $instruction:literal) => {
        $(#[$meta])*
        ///
        /// `SIZE` selects the source element size: 0 is byte, 1 is
        /// halfword, and 2 is word.
        ///
        /// # Safety
        ///
        /// The caller must enable the vector facility.
        #[target_feature(enable = "vector")]
        #[inline]
        pub unsafe fn $name<const SIZE: u8>(a: V128, b: V128) -> V128 {
            const { assert!(SIZE <= 2, "element size must be in 0..=2") };
            let a = u128::from_be_bytes(a.to_bytes());
            let b = u128::from_be_bytes(b.to_bytes());
            let out: u128;
            unsafe {
                asm!(
                    concat!($instruction, " {out}, {a}, {b}, {size}"),
                    out = lateout(vreg) out,
                    a = in(vreg) a,
                    b = in(vreg) b,
                    size = const SIZE,
                    options(nostack, nomem, pure)
                );
            }
            V128::from_bytes(out.to_be_bytes())
        }
    };
}

vmul_lane!(
    /// `vmle`.
    ///
    /// Multiply corresponding even-numbered elements as unsigned values
    /// and place each full product in the next larger lane.
    vmle,
    "vmle"
);
vmul_lane!(
    /// `vmlh`.
    ///
    /// Multiply corresponding elements as unsigned values and place the
    /// high half of each product in the same lane size.
    vmlh,
    "vmlh"
);
vmul_lane!(
    /// `vmlo`.
    ///
    /// Multiply corresponding odd-numbered elements as unsigned values
    /// and place each full product in the next larger lane.
    vmlo,
    "vmlo"
);

/// `vistr`.
///
/// Copy the elements of `a` up to the first zero element and set the
/// remaining elements to zero. `SIZE` selects the element size: 0 is byte,
/// 1 is halfword, and 2 is word. The condition code is not returned.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vistr<const SIZE: u8>(a: V128) -> V128 {
    const { assert!(SIZE <= 2, "vistr element size must be in 0..=2") };
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vistr {out}, {input}, {size}, 0",
            out = lateout(vreg) out,
            input = in(vreg) input,
            size = const SIZE,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vstrc`.
///
/// Compare the string in `a` with the ranges in `b` and the per-range
/// comparison controls in `c`. `CONTROL` selects the result form and the
/// zero-search and inversion controls; bit 2 selects a byte mask, and
/// without it the byte index of the first match is placed in doubleword 0.
/// `SIZE` selects the element size: 0 is byte, 1 is halfword, and 2 is
/// word. The condition code is not returned.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vstrc<const SIZE: u8, const CONTROL: u8>(a: V128, b: V128, c: V128) -> V128 {
    const {
        assert!(SIZE <= 2, "vstrc element size must be in 0..=2");
        assert!(CONTROL <= 0x0f, "vstrc control must be in 0..=15");
    }
    let a = u128::from_be_bytes(a.to_bytes());
    let b = u128::from_be_bytes(b.to_bytes());
    let c = u128::from_be_bytes(c.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vstrc {out}, {a}, {b}, {c}, {size}, {control}",
            out = lateout(vreg) out,
            a = in(vreg) a,
            b = in(vreg) b,
            c = in(vreg) c,
            size = const SIZE,
            control = const CONTROL,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

macro_rules! find_element {
    ($(#[$meta:meta])* $name:ident, $instruction:literal) => {
        $(#[$meta])*
        ///
        /// `SIZE` selects the element size: 0 is byte, 1 is halfword,
        /// and 2 is word. For `vfae`, bit 2 of `CONTROL` selects a byte
        /// mask; otherwise the byte index of the first match is placed in
        /// doubleword 0. `vfee` and `vfene` always return the index, and
        /// `CONTROL` selects the zero-search and condition-code controls.
        /// The condition code is not returned.
        ///
        /// # Safety
        ///
        /// The caller must enable the vector facility.
        #[target_feature(enable = "vector")]
        #[inline]
        pub unsafe fn $name<const SIZE: u8, const CONTROL: u8>(a: V128, b: V128) -> V128 {
            const {
                assert!(SIZE <= 2, "element size must be in 0..=2");
                assert!(CONTROL <= 0x0f, "control must be in 0..=15");
            }
            let a = u128::from_be_bytes(a.to_bytes());
            let b = u128::from_be_bytes(b.to_bytes());
            let out: u128;
            unsafe {
                asm!(
                    concat!($instruction, " {out}, {a}, {b}, {size}, {control}"),
                    out = lateout(vreg) out,
                    a = in(vreg) a,
                    b = in(vreg) b,
                    size = const SIZE,
                    control = const CONTROL,
                    options(nostack, nomem, pure)
                );
            }
            V128::from_bytes(out.to_be_bytes())
        }
    };
}

find_element!(
    /// `vfae`.
    ///
    /// Find any element of `a` that equals an element of `b`.
    vfae,
    "vfae"
);
find_element!(
    /// `vfee`.
    ///
    /// Find the first element of `a` that equals the corresponding
    /// element of `b`.
    vfee,
    "vfee"
);
find_element!(
    /// `vfene`.
    ///
    /// Find the first element of `a` that is not equal to the
    /// corresponding element of `b`.
    vfene,
    "vfene"
);

/// `vscbi`.
///
/// Subtract `b` from `a` and return the borrow indication. Each lane is 0
/// when a borrow occurred and 1 otherwise. `SIZE` selects the element size:
/// 0 is byte, 1 is halfword, 2 is word, and 3 is doubleword.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vscbi<const SIZE: u8>(a: V128, b: V128) -> V128 {
    const { assert!(SIZE <= 3, "vscbi element size must be in 0..=3") };
    let a = u128::from_be_bytes(a.to_bytes());
    let b = u128::from_be_bytes(b.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vscbi {out}, {a}, {b}, {size}",
            out = lateout(vreg) out,
            a = in(vreg) a,
            b = in(vreg) b,
            size = const SIZE,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vlgv`.
///
/// Load one element of `a` into a general-purpose register. `SIZE` selects
/// the element size: 0 is byte, 1 is halfword, 2 is word, and 3 is
/// doubleword. The result is zero-extended to 64 bits.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vlgv<const SIZE: u8, const INDEX: u8>(a: V128) -> u64 {
    const {
        assert!(SIZE <= 3, "vlgv element size must be in 0..=3");
        assert!(
            (INDEX as u16) < (16u16 >> SIZE),
            "vlgv element index is out of range"
        );
    }
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u64;
    unsafe {
        asm!(
            "vlgv {out}, {input}, {index}, {size}",
            out = lateout(reg) out,
            input = in(vreg) input,
            index = const INDEX,
            size = const SIZE,
            options(nostack, nomem, pure)
        );
    }
    out
}

macro_rules! gather_element {
    ($(#[$meta:meta])* $name:ident, $instruction:literal, $max:literal) => {
        $(#[$meta])*
        ///
        /// `INDEX` selects the element to replace and must be in
        /// `0..=$max`. `base` must be valid for reading one element at
        /// `base` plus the selected offset.
        ///
        /// # Safety
        ///
        /// The caller must enable the vector facility. `base` must be
        /// valid for the gathered access.
        #[target_feature(enable = "vector")]
        #[inline]
        pub unsafe fn $name<const INDEX: u8>(a: V128, offsets: V128, base: *const u8) -> V128 {
            const { assert!(INDEX <= $max, "gather element index is out of range") };
            let mut out = u128::from_be_bytes(a.to_bytes());
            let offsets = u128::from_be_bytes(offsets.to_bytes());
            unsafe {
                asm!(
                    concat!($instruction, " {out}, 0({offsets}, %r1), {index}"),
                    out = inout(vreg) out,
                    offsets = in(vreg) offsets,
                    in("r1") base,
                    index = const INDEX,
                    options(nostack)
                );
            }
            V128::from_bytes(out.to_be_bytes())
        }
    };
}

gather_element!(
    /// `vgef`.
    ///
    /// Gather one 32-bit element from `base` plus the offset in the
    /// selected element of `offsets` and replace the same element of `a`.
    vgef,
    "vgef",
    3
);
gather_element!(
    /// `vgeg`.
    ///
    /// Gather one 64-bit element from `base` plus the offset in the
    /// selected element of `offsets` and replace the same element of `a`.
    vgeg,
    "vgeg",
    1
);

/// `vftci` on double-precision lanes.
///
/// Test each binary floating-point element against the 12-bit data class
/// mask `MASK` and set a true lane to all bits one. The condition code is
/// not returned.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vftci_f64<const MASK: u16>(a: V128) -> V128 {
    const { assert!(MASK <= 0x0fff, "vftci class mask must be in 0..=4095") };
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vftcidb {out}, {input}, {mask}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            mask = const MASK,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vfi` on double-precision lanes.
///
/// Round each binary floating-point element to an integral
/// floating-point value using rounding mode `M5`. `M4` must have bits 0-1
/// clear; bit 2 suppresses the inexact exception. The condition code is
/// not returned.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vfi_f64<const M4: u8, const M5: u8>(a: V128) -> V128 {
    const {
        assert!(M4 <= 0x0f, "vfi M4 must be in 0..=15");
        assert!(M4 & 0x03 == 0, "vfi M4 bits 0-1 must be zero");
        assert!(M5 <= 0x0f, "vfi M5 must be in 0..=15");
    }
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vfidb {out}, {input}, {m4}, {m5}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            m4 = const M4,
            m5 = const M5,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vclzg`.
///
/// Count the leading zero bits in each 64-bit lane.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vclzg(a: V128) -> V128 {
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vclzg {out}, {input}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vctzg`.
///
/// Count the trailing zero bits in each 64-bit lane.
///
/// # Safety
///
/// The caller must enable the vector facility.
#[target_feature(enable = "vector")]
#[inline]
pub unsafe fn vctzg(a: V128) -> V128 {
    let input = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vctzg {out}, {input}",
            out = lateout(vreg) out,
            input = in(vreg) input,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}
