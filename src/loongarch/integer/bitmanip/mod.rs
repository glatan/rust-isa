//! LoongArch bit-manipulation semantic helpers.
//!
//! These functions implement value semantics in Rust. They are safe and do not
//! require the LBT target feature. They are semantic helpers, not LBT
//! instruction wrappers. Immediate operands (bit ranges) are const generics;
//! invalid ranges do not compile.
//!
//! Instruction behavior follows the [LoongArch Documentation](https://loongson.github.io/LoongArch-Documentation/).
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::loongarch::integer::bitmanip::revb;
//!
//! assert_eq!(revb(0x0102_0304_0506_0708), 0x0807_0605_0403_0201);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::loongarch::integer::bitmanip::*;
//! use rust_isa::loongarch::simd::lsx::V128;
//! use rust_isa::loongarch::simd::lasx::V256;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//!
//! let _ = bitrev(0);
//! let _ = bstrins::<0, 0>(0, 0);
//! let _ = bstrpick::<0, 0>(0);
//! let _ = revb(0);
//! let _ = revh(0);
//! let _ = revw(0);
//! ```
//!

//! </details>

/// `revb.d`: reverse the byte order of a 64-bit value.
#[inline(always)]
pub fn revb(value: u64) -> u64 {
    value.swap_bytes()
}

/// `revh.d`: reverse the order of the four 16-bit halfwords.
#[inline(always)]
pub fn revh(value: u64) -> u64 {
    ((value & 0x0000_0000_0000_ffff) << 48)
        | ((value & 0x0000_0000_ffff_0000) << 16)
        | ((value & 0x0000_ffff_0000_0000) >> 16)
        | ((value & 0xffff_0000_0000_0000) >> 48)
}

/// Reverse the order of the two 32-bit words.
///
/// This is a semantic helper equivalent to `rotri.d` with 32; LBT has no
/// separate `revw.d` mnemonic.
#[inline(always)]
pub fn revw(value: u64) -> u64 {
    value.rotate_left(32)
}

/// `bitrev.d`: reverse all 64 bits.
#[inline(always)]
pub fn bitrev(value: u64) -> u64 {
    value.reverse_bits()
}

/// `bstrins.d`: insert the low bits of `src` into `dst` at `LSB`.
///
/// `MSB` and `LSB` are inclusive and must satisfy `LSB <= MSB <= 63`.
#[inline(always)]
pub fn bstrins<const MSB: u32, const LSB: u32>(dst: u64, src: u64) -> u64 {
    const { assert!(LSB <= MSB && MSB <= 63, "invalid bit range") };
    let width = MSB - LSB + 1;
    let mask = if width == 64 {
        u64::MAX
    } else {
        ((1u64 << width) - 1) << LSB
    };
    (dst & !mask) | ((src << LSB) & mask)
}

/// `bstrpick.d`: extract bits `LSB..=MSB` from `src`.
#[inline(always)]
pub fn bstrpick<const MSB: u32, const LSB: u32>(src: u64) -> u64 {
    const { assert!(LSB <= MSB && MSB <= 63, "invalid bit range") };
    let width = MSB - LSB + 1;
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };
    (src >> LSB) & mask
}
