//! SSE.
//!
//! SSE is part of the x86-64 baseline. The operations are safe.
//!
//! The values are [`V128`]. This module owns the SSE 32-bit float operations
//! and compiles its C backend with `-msse`.
//!
//! [`V128`]: super::sse2::V128
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The module has its own C backend; it does not re-export the SSE2 wrappers.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::simd::sse::addps;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(addps(v, v).to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::simd::sse::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! let _ = addps(v128, v128);
//! let _ = cmpeqps(v128, v128);
//! let _ = cmpleps(v128, v128);
//! let _ = cmpltps(v128, v128);
//! let _ = divps(v128, v128);
//! let _ = maxps(v128, v128);
//! let _ = minps(v128, v128);
//! let _ = mulps(v128, v128);
//! #[cfg(target_arch = "x86_64")] let _ = shufps::<0>(v128, v128);
//! let _ = sqrtps(v128);
//! let _ = subps(v128, v128);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::sse::addps as _;
//! use rust_isa::x86::simd::sse::cmpeqps as _;
//! use rust_isa::x86::simd::sse::cmpleps as _;
//! use rust_isa::x86::simd::sse::cmpltps as _;
//! use rust_isa::x86::simd::sse::divps as _;
//! use rust_isa::x86::simd::sse::maxps as _;
//! use rust_isa::x86::simd::sse::minps as _;
//! use rust_isa::x86::simd::sse::mulps as _;
//! use rust_isa::x86::simd::sse::shufps as _;
//! use rust_isa::x86::simd::sse::sqrtps as _;
//! use rust_isa::x86::simd::sse::subps as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use super::sse2::V128;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_sse_addps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse_subps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse_mulps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse_divps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse_minps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse_maxps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse_sqrtps(a: *const u8, out: *mut u8);
    fn rust_isa_sse_cmpeqps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse_cmpltps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse_cmpleps(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($name:ident, $symbol:ident, $doc:literal) => {
        #[doc = $doc]
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $symbol(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    addps,
    rust_isa_sse_addps,
    "`addps`\n\nAdd four 32-bit float lanes."
);
binary!(
    subps,
    rust_isa_sse_subps,
    "`subps`\n\nSubtract four 32-bit float lanes."
);
binary!(
    mulps,
    rust_isa_sse_mulps,
    "`mulps`\n\nMultiply four 32-bit float lanes."
);
binary!(
    divps,
    rust_isa_sse_divps,
    "`divps`\n\nDivide four 32-bit float lanes."
);
binary!(
    minps,
    rust_isa_sse_minps,
    "`minps`\n\nTake the minimum of each 32-bit float lane."
);
binary!(
    maxps,
    rust_isa_sse_maxps,
    "`maxps`\n\nTake the maximum of each 32-bit float lane."
);
binary!(
    cmpeqps,
    rust_isa_sse_cmpeqps,
    "`cmpeqps`\n\nCompare four float lanes for equality."
);
binary!(
    cmpltps,
    rust_isa_sse_cmpltps,
    "`cmpltps`\n\nCompare four float lanes for less-than."
);
binary!(
    cmpleps,
    rust_isa_sse_cmpleps,
    "`cmpleps`\n\nCompare four float lanes for less-than-or-equal."
);

/// `sqrtps`
#[inline(always)]
pub fn sqrtps(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse_sqrtps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `shufps`
///
/// Shuffle four 32-bit float lanes with `IMM`. The low two lanes come from `a`
/// and the high two lanes come from `b`.
#[inline(always)]
pub fn shufps<const IMM: u8>(a: V128, b: V128) -> V128 {
    let a = u128::from_le_bytes(a.to_bytes());
    let b = u128::from_le_bytes(b.to_bytes());
    let output: u128;
    unsafe {
        asm!(
            "shufps {output}, {b}, {imm}",
            output = inout(xmm_reg) a => output,
            b = in(xmm_reg) b,
            imm = const IMM,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(output.to_le_bytes())
}
