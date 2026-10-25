//! s390x vector packed decimal facility.
//!
//! The packed decimal instructions operate on packed decimal values in
//! vector registers. Every instruction wrapper is `unsafe` and the caller must enable
//! the vector packed decimal facility. The values are [`V128`], except for
//! the conversions between packed decimal and binary integers, which take
//! or return a general-purpose register. Immediate operands are const generics
//! unless documented otherwise. The byte order is big-endian.
//!
//! The condition code is not returned. Overflow and the decimal data
//! exception are left to the caller.
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
//! use rust_isa::s390x::simd::vector_packed_decimal::vsdp;
//!
//! let a = V128::from_bytes([0; 16]);
//! let b = V128::from_bytes([0; 16]);
//! let _ = unsafe { vsdp::<0, 0>(a, b) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::s390x::simd::vector_packed_decimal::vap as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vcvb as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vcvbg as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vcvd as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vpkz as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vsdp as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vsp as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vsrp as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vstrl as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vupkz as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vupkzh as _;
//! use rust_isa::s390x::simd::vector_packed_decimal::vupkzl as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::vector::V128;
use core::arch::asm;

macro_rules! decimal_binary {
    ($(#[$meta:meta])* $name:ident, $instruction:literal) => {
        $(#[$meta])*
        ///
        /// `I4` holds the result digit count in bits 0-4 and the
        /// instruction-overflow mask in bit 7. `M5` holds the
        /// force-positive controls in bits 1-3 and the condition-code-set
        /// control in bit 0. The condition code is not returned.
        ///
        /// # Safety
        ///
        /// The caller must enable the vector packed decimal facility.
        #[target_feature(enable = "vector-packed-decimal")]
        #[inline]
        pub unsafe fn $name<const I4: u8, const M5: u8>(a: V128, b: V128) -> V128 {
            const { assert!(I4 & 0x60 == 0, "I4 reserved bits must be zero") };
            const { assert!(I4 & 0x1f != 0, "I4 result digit count must not be zero") };
            const { assert!(M5 <= 0x0f, "M5 must be in 0..=15") };
            let a = u128::from_be_bytes(a.to_bytes());
            let b = u128::from_be_bytes(b.to_bytes());
            let out: u128;
            unsafe {
                asm!(
                    concat!($instruction, " {out}, {a}, {b}, {i4}, {m5}"),
                    out = lateout(vreg) out,
                    a = in(vreg) a,
                    b = in(vreg) b,
                    i4 = const I4,
                    m5 = const M5,
                    options(nostack, nomem, pure)
                );
            }
            V128::from_bytes(out.to_be_bytes())
        }
    };
}

decimal_binary!(
    /// `vap`.
    ///
    /// Add packed decimal values.
    vap,
    "vap"
);
decimal_binary!(
    /// `vsp`.
    ///
    /// Subtract packed decimal values.
    vsp,
    "vsp"
);
/// `vsdp`.
///
/// Shift left and divide packed decimal values. `I4` holds the shift
/// amount in bits 3-7; bits 0-2 are reserved and must be zero. `M5`
/// holds the force-positive controls in bits 1-3 and the
/// condition-code-set control in bit 0. The condition code is not
/// returned.
///
/// # Safety
///
/// The caller must enable the vector packed decimal facility.
#[target_feature(enable = "vector-packed-decimal")]
#[inline]
pub unsafe fn vsdp<const I4: u8, const M5: u8>(a: V128, b: V128) -> V128 {
    const { assert!(I4 & 0x07 == 0, "I4 reserved bits must be zero") };
    const { assert!(M5 <= 0x0f, "M5 must be in 0..=15") };
    let a = u128::from_be_bytes(a.to_bytes());
    let b = u128::from_be_bytes(b.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vsdp {out}, {a}, {b}, {i4}, {m5}",
            out = lateout(vreg) out,
            a = in(vreg) a,
            b = in(vreg) b,
            i4 = const I4,
            m5 = const M5,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vsrp`.
///
/// Shift right and round a packed decimal value. `I3` holds the shift
/// amount and the decimal rounding digit. `M4` holds the force-positive
/// controls and the condition-code-set control. `I5` holds the result
/// digit count in bits 0-4 and the instruction-overflow mask in bit 7.
/// The condition code is not returned.
///
/// # Safety
///
/// The caller must enable the vector packed decimal facility.
#[target_feature(enable = "vector-packed-decimal")]
#[inline]
pub unsafe fn vsrp<const I3: u8, const M4: u8, const I5: u8>(a: V128) -> V128 {
    const { assert!(I5 & 0x60 == 0, "I5 reserved bits must be zero") };
    const { assert!(I5 & 0x1f != 0, "I5 result digit count must not be zero") };
    const { assert!(M4 <= 0x0f, "M4 must be in 0..=15") };
    let a = u128::from_be_bytes(a.to_bytes());
    let out: u128;
    unsafe {
        asm!(
            "vsrp {out}, {a}, {i3}, {m4}, {i5}",
            out = lateout(vreg) out,
            a = in(vreg) a,
            i3 = const I3,
            m4 = const M4,
            i5 = const I5,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vpkz`.
///
/// Convert `LENGTH` zoned decimal bytes at `p` to packed decimal.
/// `LENGTH` must be in `1..=31`; the instruction encodes `LENGTH - 1`.
///
/// # Safety
///
/// The caller must enable the vector packed decimal facility. `p` must be
/// valid for reading `LENGTH` bytes.
#[target_feature(enable = "vector-packed-decimal")]
#[inline]
pub unsafe fn vpkz<const LENGTH: u8>(p: *const u8) -> V128 {
    const {
        assert!(LENGTH >= 1 && LENGTH <= 31, "length must be in 1..=31");
        assert!(LENGTH - 1 <= 30, "length code must be in 0..=30");
    }
    let out: u128;
    unsafe {
        asm!(
            "vpkz {out}, 0(%r1), {length}",
            out = lateout(vreg) out,
            in("r1") p,
            length = const { LENGTH - 1 },
            options(nostack)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

/// `vupkz`.
///
/// Convert the packed decimal value in `a` to `LENGTH` zoned decimal
/// bytes at `p`. `LENGTH` must be in `1..=31`; the instruction encodes
/// `LENGTH - 1`.
///
/// # Safety
///
/// The caller must enable the vector packed decimal facility. `p` must be
/// valid for writing `LENGTH` bytes.
#[target_feature(enable = "vector-packed-decimal")]
#[inline]
pub unsafe fn vupkz<const LENGTH: u8>(a: V128, p: *mut u8) {
    const {
        assert!(LENGTH >= 1 && LENGTH <= 31, "length must be in 1..=31");
        assert!(LENGTH - 1 <= 30, "length code must be in 0..=30");
    }
    let a = u128::from_be_bytes(a.to_bytes());
    unsafe {
        asm!(
            "vupkz {a}, 0(%r1), {length}",
            a = in(vreg) a,
            in("r1") p,
            length = const { LENGTH - 1 },
            options(nostack)
        );
    }
}

macro_rules! unpack_zoned {
    ($(#[$meta:meta])* $name:ident, $instruction:literal) => {
        $(#[$meta])*
        ///
        /// `M3` holds the no-sign-validation control in bit 3 and the
        /// no-validation control in bit 2.
        ///
        /// # Safety
        ///
        /// The caller must enable the vector-packed-decimal-enhancement-2
        /// facility.
        #[target_feature(enable = "vector-packed-decimal-enhancement-2")]
        #[inline]
        pub unsafe fn $name<const M3: u8>(a: V128) -> V128 {
            const { assert!(M3 <= 0x0f, "M3 must be in 0..=15") };
            let a = u128::from_be_bytes(a.to_bytes());
            let out: u128;
            unsafe {
                asm!(
                    concat!($instruction, " {out}, {a}, {m3}"),
                    out = lateout(vreg) out,
                    a = in(vreg) a,
                    m3 = const M3,
                    options(nostack, nomem, pure)
                );
            }
            V128::from_bytes(out.to_be_bytes())
        }
    };
}

unpack_zoned!(
    /// `vupkzh`.
    ///
    /// Unpack the high half of a packed decimal value to zoned decimal.
    vupkzh,
    "vupkzh"
);
unpack_zoned!(
    /// `vupkzl`.
    ///
    /// Unpack the low half of a packed decimal value to zoned decimal.
    vupkzl,
    "vupkzl"
);

/// `vstrl`.
///
/// Store the `LENGTH` rightmost bytes of `a` at `p`. `LENGTH` must be in
/// `1..=16`; the instruction encodes `LENGTH - 1`.
///
/// # Safety
///
/// The caller must enable the vector packed decimal facility. `p` must be
/// valid for writing `LENGTH` bytes.
#[target_feature(enable = "vector-packed-decimal")]
#[inline]
pub unsafe fn vstrl<const LENGTH: u8>(a: V128, p: *mut u8) {
    const {
        assert!(LENGTH >= 1 && LENGTH <= 16, "length must be in 1..=16");
    }
    let a = u128::from_be_bytes(a.to_bytes());
    unsafe {
        asm!(
            "vstrl {a}, 0(%r1), {length}",
            a = in(vreg) a,
            in("r1") p,
            length = const { LENGTH - 1 },
            options(nostack)
        );
    }
}

/// `vcvd`.
///
/// Convert the 32-bit binary integer `value` to packed decimal. `I3`
/// holds the result digit count in bits 0-4 and the instruction-overflow
/// mask in bit 7. `M4` holds the logical-binary control in bit 3, the
/// force-positive control in bit 1, and the condition-code-set control in
/// bit 0. The condition code is not returned.
///
/// # Safety
///
/// The caller must enable the vector packed decimal facility.
#[target_feature(enable = "vector-packed-decimal")]
#[inline]
pub unsafe fn vcvd<const I3: u8, const M4: u8>(value: u32) -> V128 {
    const { assert!(I3 & 0x60 == 0, "I3 reserved bits must be zero") };
    const { assert!(I3 & 0x1f != 0, "I3 result digit count must not be zero") };
    const { assert!(M4 <= 0x0f, "M4 must be in 0..=15") };
    let out: u128;
    unsafe {
        asm!(
            "vcvd {out}, {value}, {i3}, {m4}",
            out = lateout(vreg) out,
            value = in(reg) value,
            i3 = const I3,
            m4 = const M4,
            options(nostack, nomem, pure)
        );
    }
    V128::from_bytes(out.to_be_bytes())
}

macro_rules! convert_to_binary {
    ($(#[$meta:meta])* $name:ident, $instruction:literal, $int:ty) => {
        $(#[$meta])*
        ///
        /// `M3` holds the force-positive control in bit 3, the
        /// logical-binary control in bit 1, and the condition-code-set
        /// control in bit 0. The condition code is not returned.
        ///
        /// # Safety
        ///
        /// The caller must enable the vector packed decimal facility.
        #[target_feature(enable = "vector-packed-decimal")]
        #[inline]
        pub unsafe fn $name<const M3: u8>(a: V128) -> $int {
            const { assert!(M3 <= 0x0f, "M3 must be in 0..=15") };
            let a = u128::from_be_bytes(a.to_bytes());
            let out: $int;
            unsafe {
                asm!(
                    concat!($instruction, " {out}, {a}, {m3}"),
                    out = lateout(reg) out,
                    a = in(vreg) a,
                    m3 = const M3,
                    options(nostack, nomem, pure)
                );
            }
            out
        }
    };
}

convert_to_binary!(
    /// `vcvb`.
    ///
    /// Convert a packed decimal value to a 32-bit binary integer.
    vcvb,
    "vcvb",
    u32
);
convert_to_binary!(
    /// `vcvbg`.
    ///
    /// Convert a packed decimal value to a 64-bit binary integer.
    vcvbg,
    "vcvbg",
    u64
);
