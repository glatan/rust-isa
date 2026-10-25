//! s390x vector enhancements facility 2.
//!
//! These instructions are not part of the vector facility. Every operation
//! is `unsafe` and the caller must enable the vector enhancements facility
//! 2. The values are [`V128`]. Immediate operands are const generics unless
//! documented otherwise. The byte order is big-endian.
//!
//! Instruction behavior follows the [IBM z/Architecture Principles of Operation](https://www.ibm.com/docs/en/module_1678991624569/pdf/SA22-7832-14.pdf).
//!
//! The wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::s390x::simd::vector::V128;
//! use rust_isa::s390x::simd::vector_enhancements_2::vsld;
//!
//! let a = V128::from_bytes([1; 16]);
//! let b = V128::from_bytes([2; 16]);
//! let _ = unsafe { vsld::<1>(a, b) };
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::s390x::simd::vector_enhancements_2::*;
//! use rust_isa::s390x::simd::vector::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { vsld::<0>(v128, v128) };
//! let _ = unsafe { vsrd::<0>(v128, v128) };
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
//! use rust_isa::s390x::simd::vector_enhancements_2::vsld as _;
//! use rust_isa::s390x::simd::vector_enhancements_2::vsrd as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use super::vector::V128;
use core::arch::asm;

macro_rules! shift_double {
    ($(#[$meta:meta])* $name:ident, $instruction:literal, $side:literal) => {
        $(#[$meta])*
        ///
        /// Concatenate `a` and `b`, with `a` leftmost, shift the 256-bit
        #[doc = concat!("value ", $side, " by `BITS` bits, and return the ")]
        #[doc = concat!($side, "most 128 bits. `BITS` must be in `0..=7`.")]
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-enhancements-2 facility.
        #[target_feature(enable = "vector-enhancements-2")]
        #[inline]
        pub unsafe fn $name<const BITS: u8>(a: V128, b: V128) -> V128 {
            const { assert!(BITS <= 7, "shift amount must be in 0..=7") };
            let a = u128::from_be_bytes(a.to_bytes());
            let b = u128::from_be_bytes(b.to_bytes());
            let out: u128;
            unsafe {
                asm!(
                    concat!($instruction, " {out}, {a}, {b}, {bits}"),
                    out = lateout(vreg) out,
                    a = in(vreg) a,
                    b = in(vreg) b,
                    bits = const BITS,
                    options(nostack, nomem, pure)
                );
            }
            V128::from_bytes(out.to_be_bytes())
        }
    };
}

shift_double!(
    /// `vsld`.
    ///
    /// Vector shift left double by bit.
    vsld,
    "vsld",
    "left"
);
shift_double!(
    /// `vsrd`.
    ///
    /// Vector shift right double by bit.
    vsrd,
    "vsrd",
    "right"
);
