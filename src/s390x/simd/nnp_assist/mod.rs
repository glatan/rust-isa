//! s390x NNP assist facility.
//!
//! The NNP assist instructions convert between binary floating-point values
//! and the 16-bit NNP format. Every instruction wrapper is `unsafe` and the caller
//! must enable the NNP assist facility. The values are [`V128`]. The byte
//! order is big-endian.
//!
//! These instructions are not available in QEMU. The value tests in this
//! repository do not execute them.
//!
//! Instruction behavior follows the [IBM z/Architecture Principles of Operation](https://www.ibm.com/docs/en/module_1678991624569/pdf/SA22-7832-14.pdf).
//!
//! The wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::s390x::simd::nnp_assist::vcrnf;
//! use rust_isa::s390x::simd::vector::V128;
//!
//! let a = V128::from_bytes([0; 16]);
//! let b = V128::from_bytes([0; 16]);
//! let _ = unsafe { vcrnf(a, b) };
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::s390x::simd::nnp_assist::*;
//! use rust_isa::s390x::simd::vector::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { vcrnf(v, v) }.to_bytes().len(), 16);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::s390x::simd::nnp_assist::*;
//! use rust_isa::s390x::simd::vector::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { vcfn(v128) };
//! let _ = unsafe { vclfnh(v128) };
//! let _ = unsafe { vclfnl(v128) };
//! let _ = unsafe { vcnf(v128) };
//! let _ = unsafe { vcrnf(v128, v128) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::s390x::simd::nnp_assist::vcfn as _;
//! use rust_isa::s390x::simd::nnp_assist::vclfnh as _;
//! use rust_isa::s390x::simd::nnp_assist::vclfnl as _;
//! use rust_isa::s390x::simd::nnp_assist::vcnf as _;
//! use rust_isa::s390x::simd::nnp_assist::vcrnf as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use super::vector::V128;
use core::arch::asm;

macro_rules! nnp_unary {
    ($(#[$meta:meta])* $name:ident, $instruction:literal, $m3:literal, $m4:literal) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the NNP assist facility.
        #[target_feature(enable = "nnp-assist")]
        #[inline]
        pub unsafe fn $name(a: V128) -> V128 {
            let a = u128::from_be_bytes(a.to_bytes());
            let out: u128;
            unsafe {
                asm!(
                    concat!($instruction, " {out}, {a}, ", $m3, ", ", $m4),
                    out = lateout(vreg) out,
                    a = in(vreg) a,
                    options(nostack, nomem, pure)
                );
            }
            V128::from_bytes(out.to_be_bytes())
        }
    };
}

nnp_unary!(
    /// `vcnf`.
    ///
    /// Convert eight binary16 elements to the NNP format.
    vcnf,
    "vcnf",
    0,
    1
);
nnp_unary!(
    /// `vcfn`.
    ///
    /// Convert eight NNP elements to binary16.
    vcfn,
    "vcfn",
    1,
    0
);
nnp_unary!(
    /// `vclfnh`.
    ///
    /// Convert and lengthen the high NNP elements to binary32.
    vclfnh,
    "vclfnh",
    2,
    0
);
nnp_unary!(
    /// `vclfnl`.
    ///
    /// Convert and lengthen the low NNP elements to binary32.
    vclfnl,
    "vclfnl",
    2,
    0
);

/// `vcrnf`.
///
/// Convert and round two vectors of binary32 elements to the NNP format.
///
/// # Safety
///
/// The caller must enable the NNP assist facility.
#[target_feature(enable = "nnp-assist")]
#[inline]
pub unsafe fn vcrnf(a: V128, b: V128) -> V128 {
    let a = u128::from_be_bytes(a.to_bytes());
    let b = u128::from_be_bytes(b.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vcrnf {out}, {a}, {b}, 0, 2",
            out = lateout(vreg) out,
            a = in(vreg) a,
            b = in(vreg) b,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}
