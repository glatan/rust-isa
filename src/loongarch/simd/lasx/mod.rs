//! LASX.
//!
//! LASX is not part of the LoongArch baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable LASX with `#[target_feature(enable = "lasx")]` or a
//! global target feature.
//!
//! The values are [`V256`]. The C implementation uses GCC vector extensions and
//! ordinary loads and stores so that LTO can keep values in registers.
//! Immediate operands are const generics unless documented otherwise.
//! Comparison, shift, shuffle, and bit operations are written in Rust on the
//! byte representation. Float conversions, max, and min call the clang
//! intrinsics; zero cost is not claimed for those.
//!
//! Instruction behavior follows the [LoongArch Documentation](https://loongson.github.io/LoongArch-Documentation/).
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::loongarch::simd::lasx::{splat_i32, xvadd, V256};
//!
//! let left = unsafe { splat_i32(1) };
//! let right = unsafe { splat_i32(2) };
//! let sum = unsafe { xvadd(left, right) };
//! assert_eq!(sum.to_bytes(), unsafe { splat_i32(3) }.to_bytes());
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::loongarch::simd::lasx::*;
//! use rust_isa::loongarch::simd::lsx::V128;
//! use rust_isa::loongarch::simd::lasx::V256;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//!
//! unsafe {
//!     let _ = unsafe { add(v256, v256) };
//!     let _ = unsafe { add_i64(v256, v256) };
//!     let _ = unsafe { and(v256, v256) };
//!     let _ = unsafe { fadd(v256, v256) };
//!     let _ = unsafe { fdiv(v256, v256) };
//!     let _ = unsafe { fmul(v256, v256) };
//!     let _ = unsafe { fsub(v256, v256) };
//!     let _ = unsafe { mul(v256, v256) };
//!     let _ = unsafe { mul_i32(v256, v256) };
//!     let _ = unsafe { or(v256, v256) };
//!     let _ = unsafe { splat_i32(0) };
//!     let _ = unsafe { sub(v256, v256) };
//!     let _ = unsafe { sub_i64(v256, v256) };
//!     let _ = unsafe { v256_load(core::ptr::null()) };
//!     let _ = unsafe { v256_store(core::ptr::null_mut(), v256) };
//!     let _ = unsafe { vext2xv_h_b(v256) };
//!     let _ = unsafe { xor(v256, v256) };
//!     let _ = unsafe { xvabs(v256) };
//!     let _ = unsafe { xvabs_b(v256) };
//!     let _ = unsafe { xvabs_d(v256) };
//!     let _ = unsafe { xvabs_h(v256) };
//!     let _ = unsafe { xvadd(v256, v256) };
//!     let _ = unsafe { xvadd_b(v256, v256) };
//!     let _ = unsafe { xvadd_d(v256, v256) };
//!     let _ = unsafe { xvadd_h(v256, v256) };
//!     let _ = unsafe { xvadd_w(v256, v256) };
//!     let _ = unsafe { xvand(v256, v256) };
//!     let _ = unsafe { xvavgr(v256, v256) };
//!     let _ = unsafe { xvbitsel(v256, v256, v256) };
//!     let _ = unsafe { xvexth(v256) };
//!     let _ = unsafe { xvextl(v256) };
//!     let _ = unsafe { xvfadd(v256, v256) };
//!     let _ = unsafe { xvfcmp::<0>(v256, v256) };
//!     let _ = unsafe { xvfcvt(v256, v256) };
//!     let _ = unsafe { xvfdiv(v256, v256) };
//!     let _ = unsafe { xvffint(v256) };
//!     let _ = unsafe { xvfmax(v256, v256) };
//!     let _ = unsafe { xvfmin(v256, v256) };
//!     let _ = unsafe { xvfmul(v256, v256) };
//!     let _ = unsafe { xvfsub(v256, v256) };
//!     let _ = unsafe { xvftint(v256) };
//!     let _ = unsafe { xvld(core::ptr::null(), 0) };
//!     let _ = unsafe { xvldi::<0>() };
//!     let _ = unsafe { xvmax(v256, v256) };
//!     let _ = unsafe { xvmin(v256, v256) };
//!     let _ = unsafe { xvmul(v256, v256) };
//!     let _ = unsafe { xvmul_h(v256, v256) };
//!     let _ = unsafe { xvmul_w(v256, v256) };
//!     let _ = unsafe { xvneg(v256) };
//!     let _ = unsafe { xvneg_b(v256) };
//!     let _ = unsafe { xvneg_d(v256) };
//!     let _ = unsafe { xvneg_h(v256) };
//!     let _ = unsafe { xvnor(v256, v256) };
//!     let _ = unsafe { xvor(v256, v256) };
//!     let _ = unsafe { xvpackev(v256, v256) };
//!     let _ = unsafe { xvpackod(v256, v256) };
//!     let _ = unsafe { xvrepl(0) };
//!     let _ = unsafe { xvsat::<0>(v256) };
//!     let _ = unsafe { xvseq(v256, v256) };
//!     let _ = unsafe { xvseqi(v256, 0) };
//!     let _ = unsafe { xvshuf(v256, v256, v256) };
//!     let _ = unsafe { xvshuf4i::<0>(v256) };
//!     let _ = unsafe { xvsle(v256, v256) };
//!     let _ = unsafe { xvslei(v256, 0) };
//!     let _ = unsafe { xvsll(v256, 0) };
//!     let _ = unsafe { xvslli::<0>(v256) };
//!     let _ = unsafe { xvslt(v256, v256) };
//!     let _ = unsafe { xvslti(v256, 0) };
//!     let _ = unsafe { xvsra(v256, 0) };
//!     let _ = unsafe { xvsrai::<0>(v256) };
//!     let _ = unsafe { xvsrl(v256, 0) };
//!     let _ = unsafe { xvsrli::<0>(v256) };
//!     let _ = unsafe { xvssr(v256, v256) };
//!     let _ = unsafe { xvst(core::ptr::null_mut(), 0, v256) };
//!     let _ = unsafe { xvsub(v256, v256) };
//!     let _ = unsafe { xvsub_b(v256, v256) };
//!     let _ = unsafe { xvsub_d(v256, v256) };
//!     let _ = unsafe { xvsub_h(v256, v256) };
//!     let _ = unsafe { xvsub_w(v256, v256) };
//!     let _ = unsafe { xvxor(v256, v256) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::loongarch::simd::lasx::add as _;
//! use rust_isa::loongarch::simd::lasx::add_i64 as _;
//! use rust_isa::loongarch::simd::lasx::and as _;
//! use rust_isa::loongarch::simd::lasx::fadd as _;
//! use rust_isa::loongarch::simd::lasx::fdiv as _;
//! use rust_isa::loongarch::simd::lasx::fmul as _;
//! use rust_isa::loongarch::simd::lasx::fsub as _;
//! use rust_isa::loongarch::simd::lasx::mul as _;
//! use rust_isa::loongarch::simd::lasx::mul_i32 as _;
//! use rust_isa::loongarch::simd::lasx::or as _;
//! use rust_isa::loongarch::simd::lasx::splat_i32 as _;
//! use rust_isa::loongarch::simd::lasx::sub as _;
//! use rust_isa::loongarch::simd::lasx::sub_i64 as _;
//! use rust_isa::loongarch::simd::lasx::v256_load as _;
//! use rust_isa::loongarch::simd::lasx::v256_store as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xor as _;
//! use rust_isa::loongarch::simd::lasx::xvabs as _;
//! use rust_isa::loongarch::simd::lasx::xvabs_b as _;
//! use rust_isa::loongarch::simd::lasx::xvabs_d as _;
//! use rust_isa::loongarch::simd::lasx::xvabs_h as _;
//! use rust_isa::loongarch::simd::lasx::xvadd as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_b as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_d as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_h as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_w as _;
//! use rust_isa::loongarch::simd::lasx::xvand as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr as _;
//! use rust_isa::loongarch::simd::lasx::xvbitsel as _;
//! use rust_isa::loongarch::simd::lasx::xvexth as _;
//! use rust_isa::loongarch::simd::lasx::xvextl as _;
//! use rust_isa::loongarch::simd::lasx::xvfadd as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp as _;
//! use rust_isa::loongarch::simd::lasx::xvfcvt as _;
//! use rust_isa::loongarch::simd::lasx::xvfdiv as _;
//! use rust_isa::loongarch::simd::lasx::xvffint as _;
//! use rust_isa::loongarch::simd::lasx::xvfmax as _;
//! use rust_isa::loongarch::simd::lasx::xvfmin as _;
//! use rust_isa::loongarch::simd::lasx::xvfmul as _;
//! use rust_isa::loongarch::simd::lasx::xvfsub as _;
//! use rust_isa::loongarch::simd::lasx::xvftint as _;
//! use rust_isa::loongarch::simd::lasx::xvld as _;
//! use rust_isa::loongarch::simd::lasx::xvldi as _;
//! use rust_isa::loongarch::simd::lasx::xvmax as _;
//! use rust_isa::loongarch::simd::lasx::xvmin as _;
//! use rust_isa::loongarch::simd::lasx::xvmul as _;
//! use rust_isa::loongarch::simd::lasx::xvmul_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmul_w as _;
//! use rust_isa::loongarch::simd::lasx::xvneg as _;
//! use rust_isa::loongarch::simd::lasx::xvneg_b as _;
//! use rust_isa::loongarch::simd::lasx::xvneg_d as _;
//! use rust_isa::loongarch::simd::lasx::xvneg_h as _;
//! use rust_isa::loongarch::simd::lasx::xvnor as _;
//! use rust_isa::loongarch::simd::lasx::xvor as _;
//! use rust_isa::loongarch::simd::lasx::xvpackev as _;
//! use rust_isa::loongarch::simd::lasx::xvpackod as _;
//! use rust_isa::loongarch::simd::lasx::xvrepl as _;
//! use rust_isa::loongarch::simd::lasx::xvsat as _;
//! use rust_isa::loongarch::simd::lasx::xvseq as _;
//! use rust_isa::loongarch::simd::lasx::xvseqi as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf4i as _;
//! use rust_isa::loongarch::simd::lasx::xvsle as _;
//! use rust_isa::loongarch::simd::lasx::xvslei as _;
//! use rust_isa::loongarch::simd::lasx::xvsll as _;
//! use rust_isa::loongarch::simd::lasx::xvslli as _;
//! use rust_isa::loongarch::simd::lasx::xvslt as _;
//! use rust_isa::loongarch::simd::lasx::xvslti as _;
//! use rust_isa::loongarch::simd::lasx::xvsra as _;
//! use rust_isa::loongarch::simd::lasx::xvsrai as _;
//! use rust_isa::loongarch::simd::lasx::xvsrl as _;
//! use rust_isa::loongarch::simd::lasx::xvsrli as _;
//! use rust_isa::loongarch::simd::lasx::xvssr as _;
//! use rust_isa::loongarch::simd::lasx::xvst as _;
//! use rust_isa::loongarch::simd::lasx::xvsub as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_w as _;
//! use rust_isa::loongarch::simd::lasx::xvxor as _;
//! ```
//! <!-- doctest-inventory:end -->

mod v256;

pub use v256::*;

unsafe extern "C" {
    fn rust_isa_lasx_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_mul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_sub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fmax_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fmin_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fcvt_s_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_ftint_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_ffint_s_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable LASX.
        #[target_feature(enable = "lasx")]
        #[inline]
        pub unsafe fn $name(a: V256, b: V256) -> V256 {
            let mut out = V256::from_bytes([0; 32]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `xvadd.w` on 32-bit lanes.
    add,
    rust_isa_lasx_add
);
binary!(
    /// `xvmul.w` on 32-bit lanes, keeping the low 32 bits.
    mul,
    rust_isa_lasx_mul
);
binary!(
    /// `xvsub.w` on 32-bit lanes.
    sub,
    rust_isa_lasx_sub
);
binary!(
    /// `xvadd.d` on 64-bit lanes.
    add_i64,
    rust_isa_lasx_add_i64
);
binary!(
    /// `xvsub.d` on 64-bit lanes.
    sub_i64,
    rust_isa_lasx_sub_i64
);
binary!(
    /// `xvmul.w` on 32-bit lanes.
    mul_i32,
    rust_isa_lasx_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_lasx_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_lasx_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_lasx_xor
);
binary!(
    /// `xvfadd.s` on 32-bit float lanes.
    fadd,
    rust_isa_lasx_fadd
);
binary!(
    /// `xvfsub.s` on 32-bit float lanes.
    fsub,
    rust_isa_lasx_fsub
);
binary!(
    /// `xvfmul.s` on 32-bit float lanes.
    fmul,
    rust_isa_lasx_fmul
);
binary!(
    /// `xvfdiv.s` on 32-bit float lanes.
    fdiv,
    rust_isa_lasx_fdiv
);

/// Broadcast a 32-bit value to all eight lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn splat_i32(value: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_splat_i32(value, out.as_mut_ptr());
    }
    out
}

/// `xvadd.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd(a: V256, b: V256) -> V256 {
    unsafe { add(a, b) }
}
/// `xvsub.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub(a: V256, b: V256) -> V256 {
    unsafe { sub(a, b) }
}
/// `xvmul.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul(a: V256, b: V256) -> V256 {
    unsafe { mul(a, b) }
}
/// `xvand.v`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvand(a: V256, b: V256) -> V256 {
    unsafe { and(a, b) }
}
/// `xvor.v`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvor(a: V256, b: V256) -> V256 {
    unsafe { or(a, b) }
}
/// `xvxor.v`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvxor(a: V256, b: V256) -> V256 {
    unsafe { xor(a, b) }
}

/// `xvabs.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabs(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let value = i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.wrapping_abs().to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvneg.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let value = i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.wrapping_neg().to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvadd.d`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_d(a: V256, b: V256) -> V256 {
    unsafe { add_i64(a, b) }
}
/// `xvsub.d`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_d(a: V256, b: V256) -> V256 {
    unsafe { sub_i64(a, b) }
}

/// `xvadd.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_b(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| a[i].wrapping_add(b[i])))
}
/// `xvadd.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_h(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for i in 0..16 {
        let value = u16::from_le_bytes([a[i * 2], a[i * 2 + 1]])
            .wrapping_add(u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]));
        out[i * 2..i * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}
/// `xvsub.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_b(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| a[i].wrapping_sub(b[i])))
}
/// `xvsub.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_h(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for i in 0..16 {
        let value = u16::from_le_bytes([a[i * 2], a[i * 2 + 1]])
            .wrapping_sub(u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]));
        out[i * 2..i * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}
/// `xvmul.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul_h(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for i in 0..16 {
        let value = u16::from_le_bytes([a[i * 2], a[i * 2 + 1]])
            .wrapping_mul(u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]));
        out[i * 2..i * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvabs.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabs_b(a: V256) -> V256 {
    V256::from_bytes(core::array::from_fn(|i| {
        (a.to_bytes()[i] as i8).wrapping_abs() as u8
    }))
}

/// `xvabs.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabs_h(a: V256) -> V256 {
    let b = a.to_bytes();
    let mut o = [0u8; 32];
    for i in 0..16 {
        let v = i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]);
        o[i * 2..i * 2 + 2].copy_from_slice(&v.wrapping_abs().to_le_bytes());
    }
    V256::from_bytes(o)
}

/// `xvabs.d`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabs_d(a: V256) -> V256 {
    let b = a.to_bytes();
    let mut o = [0u8; 32];
    for i in 0..4 {
        let v = i64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        o[i * 8..i * 8 + 8].copy_from_slice(&v.wrapping_abs().to_le_bytes());
    }
    V256::from_bytes(o)
}

/// `xvneg.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg_b(a: V256) -> V256 {
    V256::from_bytes(core::array::from_fn(|i| {
        (a.to_bytes()[i] as i8).wrapping_neg() as u8
    }))
}

/// `xvneg.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg_h(a: V256) -> V256 {
    let b = a.to_bytes();
    let mut o = [0u8; 32];
    for i in 0..16 {
        let v = i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]);
        o[i * 2..i * 2 + 2].copy_from_slice(&v.wrapping_neg().to_le_bytes());
    }
    V256::from_bytes(o)
}

/// `xvneg.d`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg_d(a: V256) -> V256 {
    let b = a.to_bytes();
    let mut o = [0u8; 32];
    for i in 0..4 {
        let v = i64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        o[i * 8..i * 8 + 8].copy_from_slice(&v.wrapping_neg().to_le_bytes());
    }
    V256::from_bytes(o)
}

/// `xvadd.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_w(a: V256, b: V256) -> V256 {
    unsafe { add(a, b) }
}

/// `xvsub.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_w(a: V256, b: V256) -> V256 {
    unsafe { sub(a, b) }
}

/// `xvmul.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul_w(a: V256, b: V256) -> V256 {
    unsafe { mul(a, b) }
}

/// `xvmax.w` on signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&x.max(y).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmin.w` on signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&x.min(y).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvseq.w` on 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseq(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x == y { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsle.w` on signed 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x <= y { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslt.w` on signed 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x < y { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvseqi.w`. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseqi(a: V256, imm: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x == imm { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslei.w` on signed 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei(a: V256, imm: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x <= imm { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslti.w` on signed 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti(a: V256, imm: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x < imm { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvnor.v`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvnor(a: V256, b: V256) -> V256 {
    V256::from_bytes(core::array::from_fn(|i| {
        !(a.to_bytes()[i] | b.to_bytes()[i])
    }))
}

/// `xvbitsel.v`.
///
/// Where a mask bit is one the result takes the bit from `a`, otherwise from
/// `b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitsel(a: V256, b: V256, mask: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let m = mask.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| (a[i] & m[i]) | (b[i] & !m[i])))
}

/// `xvshuf.b`.
///
/// Each control byte selects a byte from the concatenation of `a` and `b`. The
/// low 6 bits are the index; indices 0 to 31 select `a` and indices 32 to 63
/// select `b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf(a: V256, b: V256, control: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let c = control.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        let index = (c[i] & 63) as usize;
        if index < 32 { a[index] } else { b[index - 32] }
    }))
}

/// `xvshuf4i.b`.
///
/// `IMM` holds four 2-bit selectors, one for each byte of a 4-byte group.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf4i<const IMM: u8>(a: V256) -> V256 {
    let a = a.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        a[(i / 4) * 4 + (((IMM >> ((i % 4) * 2)) & 3) as usize)]
    }))
}

/// `xvsll.w`.
///
/// Shift each 32-bit integer lane left by `shift & 31`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsll(a: V256, shift: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x << ((shift as u32) & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrl.w`.
///
/// Shift each 32-bit integer lane right by `shift & 31`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrl(a: V256, shift: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = u32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x >> ((shift as u32) & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsra.w`.
///
/// Shift each signed 32-bit integer lane right by `shift & 31`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsra(a: V256, shift: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x >> ((shift as u32) & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslli.w`. `SHIFT` is masked to 5 bits.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslli<const SHIFT: u32>(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x << (SHIFT & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrli.w`. `SHIFT` is masked to 5 bits.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrli<const SHIFT: u32>(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = u32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x >> (SHIFT & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrai.w`. `SHIFT` is masked to 5 bits.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrai<const SHIFT: u32>(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x >> (SHIFT & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvpackev.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackev(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        if i % 2 == 0 { a[i] } else { b[i - 1] }
    }))
}

/// `xvpackod.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackod(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        if i % 2 == 0 { a[i + 1] } else { b[i] }
    }))
}

/// `vext2xv.h.b`.
///
/// Sign-extend the low 16 bytes of `a` to 16-bit lanes. The high half of `a`
/// is ignored.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_h_b(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..16 {
        let value = (a[lane] as i8 as i16).to_le_bytes();
        out[lane * 2..lane * 2 + 2].copy_from_slice(&value);
    }
    V256::from_bytes(out)
}

/// `xvexth.h.b`.
///
/// Sign-extend the high 8 bytes of each 128-bit half to 16-bit lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for half in 0..2 {
        for lane in 0..8 {
            let value = (a[half * 16 + 8 + lane] as i8 as i16).to_le_bytes();
            out[half * 16 + lane * 2..half * 16 + lane * 2 + 2].copy_from_slice(&value);
        }
    }
    V256::from_bytes(out)
}

/// `xvextl.q.d`.
///
/// Sign-extend the even 64-bit lanes to 128-bit lanes, one per 128-bit half.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvextl(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for half in 0..2 {
        let value = i64::from_le_bytes(a[half * 16..half * 16 + 8].try_into().unwrap());
        out[half * 16..half * 16 + 16].copy_from_slice(&(value as i128).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsat.w`.
///
/// Saturate each signed 32-bit lane to the signed `SAT`-bit range. `SAT` must
/// be in `0..=31`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat<const SAT: u32>(a: V256) -> V256 {
    const { assert!(SAT <= 31) };
    let max = (1i64 << SAT) - 1;
    let min = -(1i64 << SAT);
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = (x as i64).clamp(min, max) as i32;
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvssrln.b.h`.
///
/// Shift the 16-bit lanes of `a` right by the low 4 bits of the corresponding
/// lanes of `b`, saturate to unsigned 8 bits, and write the result to the low
/// 8 bytes of each 128-bit half. The high 8 bytes of each half are zero.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssr(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for half in 0..2 {
        for lane in 0..8 {
            let offset = half * 16 + lane * 2;
            let source = u16::from_le_bytes([a[offset], a[offset + 1]]);
            let shift = u16::from_le_bytes([b[offset], b[offset + 1]]) % 16;
            let value = (source >> shift).min(0xff) as u8;
            out[half * 16 + lane] = value;
        }
    }
    V256::from_bytes(out)
}

/// `xvavgr.bu`.
///
/// Rounding average of the unsigned 8-bit lanes: `(a + b + 1) >> 1`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        ((a[i] as u16 + b[i] as u16 + 1) >> 1) as u8
    }))
}

/// `xvfadd.s`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfadd(a: V256, b: V256) -> V256 {
    unsafe { fadd(a, b) }
}

/// `xvfsub.s`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfsub(a: V256, b: V256) -> V256 {
    unsafe { fsub(a, b) }
}

/// `xvfmul.s`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmul(a: V256, b: V256) -> V256 {
    unsafe { fmul(a, b) }
}

/// `xvfdiv.s`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfdiv(a: V256, b: V256) -> V256 {
    unsafe { fdiv(a, b) }
}

/// `xvfmax.s`.
///
/// A NaN operand selects the other operand. The result for two NaNs is the
/// instruction's NaN.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmax(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_fmax_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmin.s`.
///
/// A NaN operand selects the other operand. The result for two NaNs is the
/// instruction's NaN.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmin(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_fmin_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfcmp.<COND>.s`.
///
/// `COND` is the 5-bit condition field. The low bit selects the signaling
/// form; only the result mask is exposed, so the quiet and signaling forms
/// return the same lanes. Valid values and mnemonics:
///
/// | `COND` | quiet  | signaling |
/// | ------ | ------ | --------- |
/// | 0      | `caf`  | `saf`     |
/// | 2      | `clt`  | `slt`     |
/// | 4      | `ceq`  | `seq`     |
/// | 6      | `cle`  | `sle`     |
/// | 8      | `cun`  | `sun`     |
/// | 10     | `cult` | `sult`    |
/// | 12     | `cueq` | `sueq`    |
/// | 14     | `cule` | `sule`    |
/// | 16     | `cne`  | `sne`     |
/// | 20     | `cor`  | `sor`     |
/// | 24     | `cune` | `sune`    |
///
/// A true lane is all bits one. Unordered comparisons are true when either
/// lane is NaN.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp<const COND: u32>(a: V256, b: V256) -> V256 {
    const { assert!(matches!(COND, 0..=17 | 20 | 21 | 24 | 25)) };
    let less = (COND >> 1) & 1 != 0;
    let equal = (COND >> 1) & 2 != 0;
    let unordered = (COND >> 1) & 4 != 0;
    let ordered_ne = (COND >> 1) & 8 != 0;
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let offset = lane * 4;
        let x = f32::from_le_bytes(a[offset..offset + 4].try_into().unwrap());
        let y = f32::from_le_bytes(b[offset..offset + 4].try_into().unwrap());
        let hit = (less && x < y)
            || (equal && x == y)
            || (unordered && (x.is_nan() || y.is_nan()))
            || (ordered_ne && (x < y || x > y));
        let value = if hit { -1i32 } else { 0 };
        out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvfcvt.s.d`.
///
/// Convert the f64 lanes to f32 lanes. In each 128-bit half the first two
/// 32-bit result lanes come from `b` and the last two come from `a`, matching
/// the instruction's two-source layout.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcvt(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_fcvt_s_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftint.w.s`.
///
/// Convert f32 lanes to signed 32-bit lanes using the current floating-point
/// rounding mode from FCSR.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftint(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_ftint_w_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffint.s.w`.
///
/// Convert signed 32-bit lanes to f32 lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffint(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_ffint_s_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvld`.
///
/// Load 32 bytes from `p + offset`.
///
/// # Safety
///
/// The caller must enable LASX. `offset` must be in `-2048..=2047` and the
/// range must be valid for reading 32 bytes. The load does not require
/// alignment.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvld(p: *const u8, offset: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), out.as_mut_ptr(), 32);
    }
    out
}

/// `xvst`.
///
/// Store 32 bytes to `p + offset`.
///
/// # Safety
///
/// The caller must enable LASX. `offset` must be in `-2048..=2047` and the
/// range must be valid for writing 32 bytes. The store does not require
/// alignment.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvst(p: *mut u8, offset: i32, value: V256) {
    unsafe {
        core::ptr::copy_nonoverlapping(value.as_ptr(), p.offset(offset as isize), 32);
    }
}

/// `xvldi`.
///
/// Build the immediate value of the instruction. `IMM` is the 13-bit encoding,
/// not the loaded scalar.
///
/// When bit 12 is clear, bits 9 to 0 are sign-extended and replicated as lanes
/// of the width selected by bits 11 and 10 (`0` is 8-bit, `1` is 16-bit, `2` is
/// 32-bit, `3` is 64-bit).
///
/// When bit 12 is set, bits 11 to 8 select one of the 13 constant patterns and
/// bits 7 to 0 supply the pattern data. The 64-bit pattern is replicated four
/// times.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvldi<const IMM: i32>() -> V256 {
    const { V256::from_bytes(vldi_bytes(IMM as u32)) }
}

/// `xvreplgr2vr.w`.
///
/// Broadcast a 32-bit value to all eight lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrepl(value: i32) -> V256 {
    unsafe { splat_i32(value) }
}

const fn vldi_bytes(imm: u32) -> [u8; 32] {
    let mut out = [0u8; 32];
    if imm & 0x1000 == 0 {
        let lane = 1usize << ((imm >> 10) & 3);
        let value = (((imm & 0x3ff) << 22) as i32 >> 22) as i64 as u64;
        let bytes = value.to_le_bytes();
        let mut i = 0;
        while i < 32 {
            out[i] = bytes[i % lane];
            i += 1;
        }
    } else {
        let data = vldi_pattern(imm).to_le_bytes();
        let mut i = 0;
        while i < 32 {
            out[i] = data[i % 8];
            i += 1;
        }
    }
    out
}

const fn vldi_pattern(imm: u32) -> u64 {
    let mode = (imm >> 8) & 0xf;
    let t = (imm & 0xff) as u64;
    match mode {
        0 => (t << 32) | t,
        1 => (t << 40) | (t << 8),
        2 => (t << 48) | (t << 16),
        3 => (t << 56) | (t << 24),
        4 => (t << 48) | (t << 32) | (t << 16) | t,
        5 => (t << 56) | (t << 40) | (t << 24) | (t << 8),
        6 => (t << 40) | (0xff << 32) | (t << 8) | 0xff,
        7 => (t << 48) | (0xffff << 32) | (t << 16) | 0xffff,
        8 => {
            let mut data = 0u64;
            let mut i = 0;
            while i < 8 {
                data |= t << (i * 8);
                i += 1;
            }
            data
        }
        9 => {
            let mut data = 0u64;
            let mut i = 0;
            while i < 8 {
                if (t >> i) & 1 != 0 {
                    data |= 0xffu64 << (i * 8);
                }
                i += 1;
            }
            data
        }
        10 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 6) | (((1 - b6) as u64) << 5) | if b6 != 0 { 0x1f } else { 0 };
            (high << 57) | (low << 51) | (high << 25) | (low << 19)
        }
        11 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 6) | (((1 - b6) as u64) << 5) | if b6 != 0 { 0x1f } else { 0 };
            (high << 25) | (low << 19)
        }
        12 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 9) | (((1 - b6) as u64) << 8) | if b6 != 0 { 0xff } else { 0 };
            (high << 54) | (low << 48)
        }
        _ => panic!("invalid xvldi immediate"),
    }
}
