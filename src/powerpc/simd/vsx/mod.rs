//! PowerPC VSX.
//!
//! VSX is not part of the PowerPC baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable VSX with a global target feature. The C file is compiled
//! with `-maltivec -mvsx`.
//!
//! The values are [`V128`]. The `f64` functions use two 64-bit float lanes and
//! the `f32` functions use four 32-bit float lanes. Immediate operands are const
//! generics unless documented otherwise.
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::powerpc::simd::altivec::V128;
//! use rust_isa::powerpc::simd::vsx::abs_f64;
//!
//! let value = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { abs_f64(value) }.to_bytes(), [0; 16]);
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::powerpc::simd::vsx::*;
//! use rust_isa::powerpc::simd::altivec::V128;
//! use rust_isa::powerpc::matrix::mma::MmaAccumulator;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! unsafe {
//!     let _ = unsafe { abs_f64(v128) };
//!     let _ = unsafe { add_f32(v128, v128) };
//!     let _ = unsafe { add_f64(v128, v128) };
//!     let _ = unsafe { div_f64(v128, v128) };
//!     let _ = unsafe { mul_f64(v128, v128) };
//!     let _ = unsafe { sqrt_f64(v128) };
//!     let _ = unsafe { sub_f64(v128, v128) };
//!     let _ = unsafe { vcmpgt(v128, v128) };
//!     let _ = unsafe { vsldoi::<0>(v128, v128) };
//!     let _ = unsafe { xscvdpsp(v128) };
//!     let _ = unsafe { xscvdpspn(v128) };
//!     let _ = unsafe { xscvspdp(v128) };
//!     let _ = unsafe { xscvspdpn(v128) };
//!     let _ = unsafe { xvabs(v128) };
//!     let _ = unsafe { xvadd(v128, v128) };
//!     let _ = unsafe { xvand(v128, v128) };
//!     let _ = unsafe { xvcmpeq(v128, v128) };
//!     let _ = unsafe { xvcpsgn(v128, v128) };
//!     let _ = unsafe { xvcvdpsxds(v128) };
//!     let _ = unsafe { xvcvdpsxws(v128) };
//!     let _ = unsafe { xvcvdpuxds(v128) };
//!     let _ = unsafe { xvcvdpuxws(v128) };
//!     let _ = unsafe { xvcvsxdsp(v128) };
//!     let _ = unsafe { xvcvsxwsp(v128) };
//!     let _ = unsafe { xvcvuxdsp(v128) };
//!     let _ = unsafe { xvcvuxwsp(v128) };
//!     let _ = unsafe { xvdiv(v128, v128) };
//!     let _ = unsafe { xvmax(v128, v128) };
//!     let _ = unsafe { xvmin(v128, v128) };
//!     let _ = unsafe { xvmul(v128, v128) };
//!     let _ = unsafe { xvneg(v128) };
//!     let _ = unsafe { xvor(v128, v128) };
//!     let _ = unsafe { xvperm(v128, v128, v128) };
//!     let _ = unsafe { xvrdpi(v128) };
//!     let _ = unsafe { xvrdpic(v128) };
//!     let _ = unsafe { xvrdpim(v128) };
//!     let _ = unsafe { xvrdpip(v128) };
//!     let _ = unsafe { xvrdpiz(v128) };
//!     let _ = unsafe { xvsel(v128, v128, v128) };
//!     let _ = unsafe { xvsl(v128, v128) };
//!     let _ = unsafe { xvsr(v128, v128) };
//!     let _ = unsafe { xvsub(v128, v128) };
//!     let _ = unsafe { xvxor(v128, v128) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::powerpc::simd::vsx::abs_f64 as _;
//! use rust_isa::powerpc::simd::vsx::add_f32 as _;
//! use rust_isa::powerpc::simd::vsx::add_f64 as _;
//! use rust_isa::powerpc::simd::vsx::div_f64 as _;
//! use rust_isa::powerpc::simd::vsx::mul_f64 as _;
//! use rust_isa::powerpc::simd::vsx::sqrt_f64 as _;
//! use rust_isa::powerpc::simd::vsx::sub_f64 as _;
//! use rust_isa::powerpc::simd::vsx::vcmpgt as _;
//! use rust_isa::powerpc::simd::vsx::vsldoi as _;
//! use rust_isa::powerpc::simd::vsx::xscvdpsp as _;
//! use rust_isa::powerpc::simd::vsx::xscvdpspn as _;
//! use rust_isa::powerpc::simd::vsx::xscvspdp as _;
//! use rust_isa::powerpc::simd::vsx::xscvspdpn as _;
//! use rust_isa::powerpc::simd::vsx::xvabs as _;
//! use rust_isa::powerpc::simd::vsx::xvadd as _;
//! use rust_isa::powerpc::simd::vsx::xvand as _;
//! use rust_isa::powerpc::simd::vsx::xvcmpeq as _;
//! use rust_isa::powerpc::simd::vsx::xvcpsgn as _;
//! use rust_isa::powerpc::simd::vsx::xvcvdpsxds as _;
//! use rust_isa::powerpc::simd::vsx::xvcvdpsxws as _;
//! use rust_isa::powerpc::simd::vsx::xvcvdpuxds as _;
//! use rust_isa::powerpc::simd::vsx::xvcvdpuxws as _;
//! use rust_isa::powerpc::simd::vsx::xvcvsxdsp as _;
//! use rust_isa::powerpc::simd::vsx::xvcvsxwsp as _;
//! use rust_isa::powerpc::simd::vsx::xvcvuxdsp as _;
//! use rust_isa::powerpc::simd::vsx::xvcvuxwsp as _;
//! use rust_isa::powerpc::simd::vsx::xvdiv as _;
//! use rust_isa::powerpc::simd::vsx::xvmax as _;
//! use rust_isa::powerpc::simd::vsx::xvmin as _;
//! use rust_isa::powerpc::simd::vsx::xvmul as _;
//! use rust_isa::powerpc::simd::vsx::xvneg as _;
//! use rust_isa::powerpc::simd::vsx::xvor as _;
//! use rust_isa::powerpc::simd::vsx::xvperm as _;
//! use rust_isa::powerpc::simd::vsx::xvrdpi as _;
//! use rust_isa::powerpc::simd::vsx::xvrdpic as _;
//! use rust_isa::powerpc::simd::vsx::xvrdpim as _;
//! use rust_isa::powerpc::simd::vsx::xvrdpip as _;
//! use rust_isa::powerpc::simd::vsx::xvrdpiz as _;
//! use rust_isa::powerpc::simd::vsx::xvsel as _;
//! use rust_isa::powerpc::simd::vsx::xvsl as _;
//! use rust_isa::powerpc::simd::vsx::xvsr as _;
//! use rust_isa::powerpc::simd::vsx::xvsub as _;
//! use rust_isa::powerpc::simd::vsx::xvxor as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_vsx_add_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_sub_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_mul_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_div_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_add_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_abs_f64(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_sqrt_f64(a: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable VSX.
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
    /// VSX 64-bit float add.
    add_f64,
    rust_isa_vsx_add_f64
);
binary!(
    /// VSX 64-bit float subtract.
    sub_f64,
    rust_isa_vsx_sub_f64
);
binary!(
    /// VSX 64-bit float multiply.
    mul_f64,
    rust_isa_vsx_mul_f64
);
binary!(
    /// VSX 64-bit float divide.
    div_f64,
    rust_isa_vsx_div_f64
);
binary!(
    /// VSX 32-bit float add.
    add_f32,
    rust_isa_vsx_add_f32
);

/// VSX 64-bit float absolute value.
///
/// # Safety
///
/// The caller must enable VSX.
#[inline(always)]
pub unsafe fn abs_f64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vsx_abs_f64(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// VSX 64-bit float square root.
///
/// # Safety
///
/// The caller must enable VSX.
#[inline(always)]
pub unsafe fn sqrt_f64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vsx_sqrt_f64(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vsx_xvadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcmpeq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_vcmpgt(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvand(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvxor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvsl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvsr(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcpsgn(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvabs(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvneg(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvrdpi(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvrdpic(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvrdpim(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvrdpip(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvrdpiz(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvsel(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvperm(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

macro_rules! vbin {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable VSX.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! vun {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable VSX.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! vtern {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable VSX.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
vbin!(
    /// `xvadddp`.
    xvadd,
    rust_isa_vsx_xvadd
);
vbin!(
    /// `xvsubdp`.
    xvsub,
    rust_isa_vsx_xvsub
);
vbin!(
    /// `xvmuldp`.
    xvmul,
    rust_isa_vsx_xvmul
);
vbin!(
    /// `xvdivdp`.
    xvdiv,
    rust_isa_vsx_xvdiv
);
vbin!(
    /// `xvmaxdp`.
    xvmax,
    rust_isa_vsx_xvmax
);
vbin!(
    /// `xvmindp`.
    xvmin,
    rust_isa_vsx_xvmin
);
vbin!(
    /// `xvcmpeqdp`.
    xvcmpeq,
    rust_isa_vsx_xvcmpeq
);
vbin!(
    /// `xvcmpgtdp`.
    vcmpgt,
    rust_isa_vsx_vcmpgt
);
vbin!(
    /// `xxland`.
    xvand,
    rust_isa_vsx_xvand
);
vbin!(
    /// `xxlor`.
    xvor,
    rust_isa_vsx_xvor
);
vbin!(
    /// `xxlxor`.
    xvxor,
    rust_isa_vsx_xvxor
);
vbin!(
    /// `vsld`.
    xvsl,
    rust_isa_vsx_xvsl
);
vbin!(
    /// `vsrd`.
    xvsr,
    rust_isa_vsx_xvsr
);
vbin!(
    /// `xvcpsgndp`.
    xvcpsgn,
    rust_isa_vsx_xvcpsgn
);
vun!(
    /// `xvabsdp`.
    xvabs,
    rust_isa_vsx_xvabs
);
vun!(
    /// `xvnegdp`.
    xvneg,
    rust_isa_vsx_xvneg
);
vun!(
    /// `xvrdpi`.
    xvrdpi,
    rust_isa_vsx_xvrdpi
);
vun!(
    /// `xvrdpic`.
    xvrdpic,
    rust_isa_vsx_xvrdpic
);
vun!(
    /// `xvrdpim`.
    xvrdpim,
    rust_isa_vsx_xvrdpim
);
vun!(
    /// `xvrdpip`.
    xvrdpip,
    rust_isa_vsx_xvrdpip
);
vun!(
    /// `xvrdpiz`.
    xvrdpiz,
    rust_isa_vsx_xvrdpiz
);
vtern!(
    /// `xxsel`.
    xvsel,
    rust_isa_vsx_xvsel
);
vtern!(
    /// `vperm`.
    xvperm,
    rust_isa_vsx_xvperm
);

unsafe extern "C" {
    fn rust_isa_vsx_vsldoi(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
}

/// `vsldoi` with an immediate shift count.
///
/// # Safety
///
/// The caller must enable VSX.
#[inline(always)]
pub unsafe fn vsldoi<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 15, "vsldoi immediate must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vsx_vsldoi(a.as_ptr(), b.as_ptr(), u32::from(IMM), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_vsx_xscvdpsp(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xscvdpspn(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xscvspdp(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xscvspdpn(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcvsxdsp(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcvuxdsp(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcvsxwsp(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcvuxwsp(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcvdpsxds(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcvdpsxws(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcvdpuxds(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_xvcvdpuxws(a: *const u8, out: *mut u8);
}

vun!(
    /// `xscvdpsp`.
    xscvdpsp,
    rust_isa_vsx_xscvdpsp
);
vun!(
    /// `xscvdpspn`.
    xscvdpspn,
    rust_isa_vsx_xscvdpspn
);
vun!(
    /// `xscvspdp`.
    xscvspdp,
    rust_isa_vsx_xscvspdp
);
vun!(
    /// `xscvspdpn`.
    xscvspdpn,
    rust_isa_vsx_xscvspdpn
);
vun!(
    /// `xvcvsxdsp`.
    xvcvsxdsp,
    rust_isa_vsx_xvcvsxdsp
);
vun!(
    /// `xvcvuxdsp`.
    xvcvuxdsp,
    rust_isa_vsx_xvcvuxdsp
);
vun!(
    /// `xvcvsxwsp`.
    xvcvsxwsp,
    rust_isa_vsx_xvcvsxwsp
);
vun!(
    /// `xvcvuxwsp`.
    xvcvuxwsp,
    rust_isa_vsx_xvcvuxwsp
);
vun!(
    /// `xvcvdpsxds`.
    xvcvdpsxds,
    rust_isa_vsx_xvcvdpsxds
);
vun!(
    /// `xvcvdpsxws`.
    xvcvdpsxws,
    rust_isa_vsx_xvcvdpsxws
);
vun!(
    /// `xvcvdpuxds`.
    xvcvdpuxds,
    rust_isa_vsx_xvcvdpuxds
);
vun!(
    /// `xvcvdpuxws`.
    xvcvdpuxws,
    rust_isa_vsx_xvcvdpuxws
);
