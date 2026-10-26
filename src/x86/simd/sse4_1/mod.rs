//! SSE4.1.
//!
//! SSE4.1 is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable SSE4.1 with `#[target_feature(enable = "sse4.1")]` or
//! a global target feature.
//!
//! The values are [`V128`]. Comparison and blend masks use the most significant
//! bit of each lane. The module contains representative integer, conversion and
//! blend operations. The dot-product and insert/extract instructions that take
//! an immediate are intentionally omitted.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.

//! # Examples
//!
//! ```
//! use rust_isa::x86::simd::sse4_1::pmulld;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { pmulld(v, v) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::sse4_1::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { blendpd::<0>(v128, v128) };
//! let _ = unsafe { blendps::<0>(v128, v128) };
//! let _ = unsafe { blendvpd(v128, v128, v128) };
//! let _ = unsafe { blendvps(v128, v128, v128) };
//! let _ = unsafe { dppd::<0>(v128, v128) };
//! let _ = unsafe { dpps::<0>(v128, v128) };
//! let _ = unsafe { insertps::<0>(v128, v128) };
//! let _ = unsafe { movntdqa(storage.0.as_ptr().cast()) };
//! let _ = unsafe { mpsadbw::<0>(v128, v128) };
//! let _ = unsafe { packusdw(v128, v128) };
//! let _ = unsafe { pblendvb(v128, v128, v128) };
//! let _ = unsafe { pblendw::<0>(v128, v128) };
//! let _ = unsafe { pextrb::<0>(v128) };
//! let _ = unsafe { pextrd::<0>(v128) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { pextrq::<0>(v128) };
//! let _ = unsafe { pextrw::<0>(v128) };
//! let _ = unsafe { phminposuw(v128) };
//! let _ = unsafe { pinsrb::<0>(v128, 0) };
//! let _ = unsafe { pinsrd::<0>(v128, 0) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { pinsrq::<0>(v128, 0) };
//! let _ = unsafe { pinsrw::<0>(v128, 0) };
//! let _ = unsafe { pmaxsb(v128, v128) };
//! let _ = unsafe { pmaxsd(v128, v128) };
//! let _ = unsafe { pmaxud(v128, v128) };
//! let _ = unsafe { pmaxuw(v128, v128) };
//! let _ = unsafe { pminsb(v128, v128) };
//! let _ = unsafe { pminsd(v128, v128) };
//! let _ = unsafe { pminud(v128, v128) };
//! let _ = unsafe { pminuw(v128, v128) };
//! let _ = unsafe { pmovsxbd(v128) };
//! let _ = unsafe { pmovsxbq(v128) };
//! let _ = unsafe { pmovsxbw(v128) };
//! let _ = unsafe { pmovsxdq(v128) };
//! let _ = unsafe { pmovsxwd(v128) };
//! let _ = unsafe { pmovsxwq(v128) };
//! let _ = unsafe { pmovzxbd(v128) };
//! let _ = unsafe { pmovzxbq(v128) };
//! let _ = unsafe { pmovzxbw(v128) };
//! let _ = unsafe { pmovzxdq(v128) };
//! let _ = unsafe { pmovzxwd(v128) };
//! let _ = unsafe { pmovzxwq(v128) };
//! let _ = unsafe { pmuldq(v128, v128) };
//! let _ = unsafe { pmulld(v128, v128) };
//! let _ = unsafe { ptest(v128, v128) };
//! let _ = unsafe { roundpd::<0>(v128) };
//! let _ = unsafe { roundps::<0>(v128) };
//! let _ = unsafe { roundsd::<0>(v128, v128) };
//! let _ = unsafe { roundss::<0>(v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::sse4_1::blendpd as _;
//! use rust_isa::x86::simd::sse4_1::blendps as _;
//! use rust_isa::x86::simd::sse4_1::blendvpd as _;
//! use rust_isa::x86::simd::sse4_1::blendvps as _;
//! use rust_isa::x86::simd::sse4_1::dppd as _;
//! use rust_isa::x86::simd::sse4_1::dpps as _;
//! use rust_isa::x86::simd::sse4_1::insertps as _;
//! use rust_isa::x86::simd::sse4_1::movntdqa as _;
//! use rust_isa::x86::simd::sse4_1::mpsadbw as _;
//! use rust_isa::x86::simd::sse4_1::packusdw as _;
//! use rust_isa::x86::simd::sse4_1::pblendvb as _;
//! use rust_isa::x86::simd::sse4_1::pblendw as _;
//! use rust_isa::x86::simd::sse4_1::pextrb as _;
//! use rust_isa::x86::simd::sse4_1::pextrd as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::simd::sse4_1::pextrq as _;
//! use rust_isa::x86::simd::sse4_1::pextrw as _;
//! use rust_isa::x86::simd::sse4_1::phminposuw as _;
//! use rust_isa::x86::simd::sse4_1::pinsrb as _;
//! use rust_isa::x86::simd::sse4_1::pinsrd as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::simd::sse4_1::pinsrq as _;
//! use rust_isa::x86::simd::sse4_1::pinsrw as _;
//! use rust_isa::x86::simd::sse4_1::pmaxsb as _;
//! use rust_isa::x86::simd::sse4_1::pmaxsd as _;
//! use rust_isa::x86::simd::sse4_1::pmaxud as _;
//! use rust_isa::x86::simd::sse4_1::pmaxuw as _;
//! use rust_isa::x86::simd::sse4_1::pminsb as _;
//! use rust_isa::x86::simd::sse4_1::pminsd as _;
//! use rust_isa::x86::simd::sse4_1::pminud as _;
//! use rust_isa::x86::simd::sse4_1::pminuw as _;
//! use rust_isa::x86::simd::sse4_1::pmovsxbd as _;
//! use rust_isa::x86::simd::sse4_1::pmovsxbq as _;
//! use rust_isa::x86::simd::sse4_1::pmovsxbw as _;
//! use rust_isa::x86::simd::sse4_1::pmovsxdq as _;
//! use rust_isa::x86::simd::sse4_1::pmovsxwd as _;
//! use rust_isa::x86::simd::sse4_1::pmovsxwq as _;
//! use rust_isa::x86::simd::sse4_1::pmovzxbd as _;
//! use rust_isa::x86::simd::sse4_1::pmovzxbq as _;
//! use rust_isa::x86::simd::sse4_1::pmovzxbw as _;
//! use rust_isa::x86::simd::sse4_1::pmovzxdq as _;
//! use rust_isa::x86::simd::sse4_1::pmovzxwd as _;
//! use rust_isa::x86::simd::sse4_1::pmovzxwq as _;
//! use rust_isa::x86::simd::sse4_1::pmuldq as _;
//! use rust_isa::x86::simd::sse4_1::pmulld as _;
//! use rust_isa::x86::simd::sse4_1::ptest as _;
//! use rust_isa::x86::simd::sse4_1::roundpd as _;
//! use rust_isa::x86::simd::sse4_1::roundps as _;
//! use rust_isa::x86::simd::sse4_1::roundsd as _;
//! use rust_isa::x86::simd::sse4_1::roundss as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use super::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_sse41_pmulld(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pminud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmaxud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pminuw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmaxuw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_ptest(a: *const u8, b: *const u8) -> i32;
    fn rust_isa_sse41_blendvps(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_sse41_blendvpd(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_sse41_pblendvb(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_sse41_packusdw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_phminposuw(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovsxbd(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovsxbw(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovzxbw(a: *const u8, out: *mut u8);
}

/// `pmulld`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmulld(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmulld(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pminud`
///
/// Return the minimum of each unsigned 32-bit lane.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pminud(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pminud(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmaxud`
///
/// Return the maximum of each unsigned 32-bit lane.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmaxud(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmaxud(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pminuw`
///
/// Return the minimum of each unsigned 16-bit lane.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pminuw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pminuw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmaxuw`
///
/// Return the maximum of each unsigned 16-bit lane.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmaxuw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmaxuw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ptest`
///
/// Return `true` when the bitwise AND of the two values is zero.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn ptest(a: V128, b: V128) -> bool {
    unsafe { rust_isa_sse41_ptest(a.as_ptr(), b.as_ptr()) != 0 }
}

/// `blendvps`
///
/// Select 32-bit float lanes from `b` where the sign bit of the corresponding
/// lane in `mask` is set, otherwise from `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn blendvps(a: V128, b: V128, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_blendvps(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `blendvpd`
///
/// Select 64-bit float lanes from `b` where the sign bit of the corresponding
/// lane in `mask` is set, otherwise from `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn blendvpd(a: V128, b: V128, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_blendvpd(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pblendvb`
///
/// Select bytes from `b` where the high bit of the corresponding byte in `mask`
/// is set, otherwise from `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pblendvb(a: V128, b: V128, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pblendvb(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `packusdw`
///
/// Convert signed 32-bit lanes to unsigned 16-bit lanes with saturation.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn packusdw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_packusdw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `phminposuw`
///
/// Return the minimum unsigned 16-bit lane in the low lane and its index in the
/// next lane. The remaining lanes are zero.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn phminposuw(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_phminposuw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmovsxbd`
///
/// Sign-extend the low four bytes to four 32-bit lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmovsxbd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmovsxbd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmovsxbw`
///
/// Sign-extend the low eight bytes to eight 16-bit lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmovsxbw(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmovsxbw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmovzxbw`
///
/// Zero-extend the low eight bytes to eight 16-bit lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmovzxbw(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmovzxbw(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pblendw`
///
/// Select 16-bit lanes from `a` and `b` with the immediate mask. Bit `i` of
/// `IMM` selects `b` for lane `i`; a clear bit selects `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pblendw<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "pblendw {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `blendps`
///
/// Select 32-bit float lanes from `a` and `b` with the immediate mask. Bit `i`
/// of `IMM` selects `b` for lane `i`; a clear bit selects `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn blendps<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 0x0f, "blendps immediate must be in 0..=15") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "blendps {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `blendpd`
///
/// Select 64-bit float lanes from `a` and `b` with the immediate mask. Bit `i`
/// of `IMM` selects `b` for lane `i`; a clear bit selects `a`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn blendpd<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 0x03, "blendpd immediate must be in 0..=3") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "blendpd {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `dpps`
///
/// Compute the dot product of 32-bit float lanes. The immediate selects the
/// products and the destination lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn dpps<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "dpps {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `dppd`
///
/// Compute the dot product of 64-bit float lanes. The immediate selects the
/// products and the destination lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn dppd<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "dppd {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `insertps`
///
/// Insert one 32-bit float lane from `b` into `a`. The immediate controls the
/// source lane, destination lane, and zeroing.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn insertps<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "insertps {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `roundps`
///
/// Round four 32-bit float lanes. The low two bits of `IMM` select the
/// rounding mode.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn roundps<const IMM: u8>(a: V128) -> V128 {
    const { assert!(IMM <= 0x0f, "roundps immediate must be in 0..=15") };
    let a = u128::from_le_bytes(a.to_bytes());
    let mut out: u128 = 0;
    unsafe {
        asm!(
            "roundps {out}, {a}, {imm}",
            out = lateout(xmm_reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `roundpd`
///
/// Round two 64-bit float lanes. The low two bits of `IMM` select the rounding
/// mode.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn roundpd<const IMM: u8>(a: V128) -> V128 {
    const { assert!(IMM <= 0x03, "roundpd immediate must be in 0..=3") };
    let a = u128::from_le_bytes(a.to_bytes());
    let mut out: u128 = 0;
    unsafe {
        asm!(
            "roundpd {out}, {a}, {imm}",
            out = lateout(xmm_reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `roundss`
///
/// Round the low 32-bit float lane of `b` and keep the upper lanes of `a`. The
/// low two bits of `IMM` select the rounding mode.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn roundss<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 0x0f, "roundss immediate must be in 0..=15") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "roundss {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `roundsd`
///
/// Round the low 64-bit float lane of `b` and keep the upper lane of `a`. The
/// low two bits of `IMM` select the rounding mode.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn roundsd<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 0x0f, "roundsd immediate must be in 0..=15") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "roundsd {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `mpsadbw`
///
/// Compute multiple packed sums of absolute differences. The low three bits of
/// `IMM` select the source offsets.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn mpsadbw<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 0x07, "mpsadbw immediate must be in 0..=7") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    unsafe {
        asm!(
            "mpsadbw {out}, {b}, {imm}",
            out = inout(xmm_reg) out,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

unsafe extern "C" {
    fn rust_isa_sse41_pminsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pminsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmaxsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmaxsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovsxbq(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovsxwd(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovsxwq(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovsxdq(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovzxbq(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovzxwd(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovzxwq(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmovzxdq(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_movntdqa(p: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[target_feature(enable = "sse4.1")]
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
    /// `pminsb`
    ///
    /// Return the minimum of each signed 8-bit lane.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pminsb,
    rust_isa_sse41_pminsb
);
binary!(
    /// `pminsd`
    ///
    /// Return the minimum of each signed 32-bit lane.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pminsd,
    rust_isa_sse41_pminsd
);
binary!(
    /// `pmaxsb`
    ///
    /// Return the maximum of each signed 8-bit lane.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmaxsb,
    rust_isa_sse41_pmaxsb
);
binary!(
    /// `pmaxsd`
    ///
    /// Return the maximum of each signed 32-bit lane.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmaxsd,
    rust_isa_sse41_pmaxsd
);

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[target_feature(enable = "sse4.1")]
        #[inline]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

unary!(
    /// `pmovsxbq`
    ///
    /// Sign-extend the low two bytes to 64-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmovsxbq,
    rust_isa_sse41_pmovsxbq
);
unary!(
    /// `pmovsxwd`
    ///
    /// Sign-extend the low four words to 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmovsxwd,
    rust_isa_sse41_pmovsxwd
);
unary!(
    /// `pmovsxwq`
    ///
    /// Sign-extend the low two words to 64-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmovsxwq,
    rust_isa_sse41_pmovsxwq
);
unary!(
    /// `pmovsxdq`
    ///
    /// Sign-extend the low two dwords to 64-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmovsxdq,
    rust_isa_sse41_pmovsxdq
);
unary!(
    /// `pmovzxbq`
    ///
    /// Zero-extend the low two bytes to 64-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmovzxbq,
    rust_isa_sse41_pmovzxbq
);
unary!(
    /// `pmovzxwd`
    ///
    /// Zero-extend the low four words to 32-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmovzxwd,
    rust_isa_sse41_pmovzxwd
);
unary!(
    /// `pmovzxwq`
    ///
    /// Zero-extend the low two words to 64-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmovzxwq,
    rust_isa_sse41_pmovzxwq
);
unary!(
    /// `pmovzxdq`
    ///
    /// Zero-extend the low two dwords to 64-bit lanes.
    ///
    /// # Safety
    ///
    /// The caller must enable SSE4.1.
    pmovzxdq,
    rust_isa_sse41_pmovzxdq
);

/// `movntdqa`
///
/// Load 16 bytes with a non-temporal hint.
///
/// # Safety
///
/// The caller must enable SSE4.1. `p` must be valid for reading 16 bytes,
/// 16-byte aligned, and should point to write-combining memory for the hint to
/// take effect.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn movntdqa(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_movntdqa(p, out.as_mut_ptr());
    }
    out
}

/// `pinsrb`
///
/// Insert the low byte of `value` into lane `IMM`. `IMM` must be in `0..=15`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pinsrb<const IMM: u8>(a: V128, value: u8) -> V128 {
    const { assert!(IMM <= 15, "pinsrb lane must be in 0..=15") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    unsafe {
        asm!(
            "pinsrb {out}, {value:e}, {imm}",
            out = inout(xmm_reg) out,
            value = in(reg) value as u32,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `pinsrd`
///
/// Insert `value` into 32-bit lane `IMM`. `IMM` must be in `0..=3`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pinsrd<const IMM: u8>(a: V128, value: u32) -> V128 {
    const { assert!(IMM <= 3, "pinsrd lane must be in 0..=3") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    unsafe {
        asm!(
            "pinsrd {out}, {value:e}, {imm}",
            out = inout(xmm_reg) out,
            value = in(reg) value,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `pextrb`
///
/// Extract lane `IMM` as a byte. `IMM` must be in `0..=15`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pextrb<const IMM: u8>(a: V128) -> u8 {
    const { assert!(IMM <= 15, "pextrb lane must be in 0..=15") };
    let a = u128::from_le_bytes(a.to_bytes());
    let out: u32;
    unsafe {
        asm!(
            "pextrb {out:e}, {a}, {imm}",
            out = lateout(reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    out as u8
}

/// `pextrd`
///
/// Extract 32-bit lane `IMM`. `IMM` must be in `0..=3`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pextrd<const IMM: u8>(a: V128) -> u32 {
    const { assert!(IMM <= 3, "pextrd lane must be in 0..=3") };
    let a = u128::from_le_bytes(a.to_bytes());
    let out: u32;
    unsafe {
        asm!(
            "pextrd {out:e}, {a}, {imm}",
            out = lateout(reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    out
}

/// `pinsrq`
///
/// Insert `value` into 64-bit lane `IMM`. `IMM` must be in `0..=1`.
///
/// # Safety
///
/// The caller must enable SSE4.1 and target x86-64.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pinsrq<const IMM: u8>(a: V128, value: u64) -> V128 {
    const { assert!(IMM <= 1, "pinsrq lane must be in 0..=1") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    unsafe {
        asm!(
            "pinsrq {out}, {value}, {imm}",
            out = inout(xmm_reg) out,
            value = in(reg) value,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `pextrq`
///
/// Extract 64-bit lane `IMM`. `IMM` must be in `0..=1`.
///
/// # Safety
///
/// The caller must enable SSE4.1 and target x86-64.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pextrq<const IMM: u8>(a: V128) -> u64 {
    const { assert!(IMM <= 1, "pextrq lane must be in 0..=1") };
    let a = u128::from_le_bytes(a.to_bytes());
    let out: u64;
    unsafe {
        asm!(
            "pextrq {out}, {a}, {imm}",
            out = lateout(reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse41_pmovzxbd(a: *const u8, out: *mut u8);
    fn rust_isa_sse41_pmuldq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pmovzxbd`
///
/// Zero-extend the low four bytes to four 32-bit lanes.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmovzxbd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmovzxbd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmuldq`
///
/// Multiply the low signed 32-bit value of each 64-bit lane and return 64-bit
/// products.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pmuldq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse41_pmuldq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pinsrw`
///
/// Insert `value` into 16-bit lane `IMM`. `IMM` must be in `0..=7`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pinsrw<const IMM: u8>(a: V128, value: u16) -> V128 {
    const { assert!(IMM <= 7, "pinsrw lane must be in 0..=7") };
    let mut out = u128::from_le_bytes(a.to_bytes());
    unsafe {
        asm!(
            "pinsrw {out}, {value:e}, {imm}",
            out = inout(xmm_reg) out,
            value = in(reg) u32::from(value),
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    V128::from_bytes(out.to_le_bytes())
}

/// `pextrw`
///
/// Extract 16-bit lane `IMM`. `IMM` must be in `0..=7`.
///
/// # Safety
///
/// The caller must enable SSE4.1.
#[target_feature(enable = "sse4.1")]
#[inline]
pub unsafe fn pextrw<const IMM: u8>(a: V128) -> u16 {
    const { assert!(IMM <= 7, "pextrw lane must be in 0..=7") };
    let a = u128::from_le_bytes(a.to_bytes());
    let out: u32;
    unsafe {
        asm!(
            "pextrw {out:e}, {a}, {imm}",
            out = lateout(reg) out,
            a = in(xmm_reg) a,
            imm = const IMM,
            options(pure, nomem, nostack),
        );
    }
    out as u16
}
