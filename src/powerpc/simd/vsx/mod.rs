//! PowerPC VSX.
//!
//! VSX is not part of the PowerPC baseline. Every operation is `unsafe` and the
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
    /// `xvadd`.
    xvadd,
    rust_isa_vsx_xvadd
);
vbin!(
    /// `xvsub`.
    xvsub,
    rust_isa_vsx_xvsub
);
vbin!(
    /// `xvmul`.
    xvmul,
    rust_isa_vsx_xvmul
);
vbin!(
    /// `xvdiv`.
    xvdiv,
    rust_isa_vsx_xvdiv
);
vbin!(
    /// `xvmax`.
    xvmax,
    rust_isa_vsx_xvmax
);
vbin!(
    /// `xvmin`.
    xvmin,
    rust_isa_vsx_xvmin
);
vbin!(
    /// `xvcmpeq`.
    xvcmpeq,
    rust_isa_vsx_xvcmpeq
);
vbin!(
    /// `vcmpgt`.
    vcmpgt,
    rust_isa_vsx_vcmpgt
);
vbin!(
    /// `xvand`.
    xvand,
    rust_isa_vsx_xvand
);
vbin!(
    /// `xvor`.
    xvor,
    rust_isa_vsx_xvor
);
vbin!(
    /// `xvxor`.
    xvxor,
    rust_isa_vsx_xvxor
);
vbin!(
    /// `xvsl`.
    xvsl,
    rust_isa_vsx_xvsl
);
vbin!(
    /// `xvsr`.
    xvsr,
    rust_isa_vsx_xvsr
);
vbin!(
    /// `xvcpsgn`.
    xvcpsgn,
    rust_isa_vsx_xvcpsgn
);
vun!(
    /// `xvabs`.
    xvabs,
    rust_isa_vsx_xvabs
);
vun!(
    /// `xvneg`.
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
    /// `xvsel`.
    xvsel,
    rust_isa_vsx_xvsel
);
vtern!(
    /// `xvperm`.
    xvperm,
    rust_isa_vsx_xvperm
);

unsafe extern "C" {
    fn rust_isa_vsx_xvsldoi(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
}

/// `xvsldoi` with an immediate shift count.
///
/// # Safety
///
/// The caller must enable VSX.
#[inline(always)]
pub unsafe fn xvsldoi<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 15, "xvsldoi immediate must be in 0..=15") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vsx_xvsldoi(a.as_ptr(), b.as_ptr(), u32::from(IMM), out.as_mut_ptr());
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
