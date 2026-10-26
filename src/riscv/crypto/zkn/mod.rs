//! RISC-V Zkn crypto.
//!
//! Zkn is a stable scalar cryptography extension. Every instruction wrapper is `unsafe`
//! and the caller must enable the required sub-extension (`zkne`, `zknd`,
//! `zknh`) with `#[target_feature]` or a global target feature.
//!
//! The functions operate on scalar registers. The caller combines them with the
//! AES or SHA specification. `aes64ks1i` accepts the round number as a runtime
//! argument.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::crypto::zkn::sha256sum0;
//!
//! assert_ne!(unsafe { sha256sum0(1) }, 1);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::riscv::crypto::zkn::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { aes64ds(0, 0) };
//! let _ = unsafe { aes64dsm(0, 0) };
//! let _ = unsafe { aes64es(0, 0) };
//! let _ = unsafe { aes64esm(0, 0) };
//! let _ = unsafe { aes64im(0) };
//! let _ = unsafe { aes64ks1i(0, 0) };
//! let _ = unsafe { aes64ks2(0, 0) };
//! let _ = unsafe { sha256sig0(0) };
//! let _ = unsafe { sha256sig1(0) };
//! let _ = unsafe { sha256sum0(0) };
//! let _ = unsafe { sha256sum1(0) };
//! let _ = unsafe { sha512sig0(0) };
//! let _ = unsafe { sha512sig1(0) };
//! let _ = unsafe { sha512sum0(0) };
//! let _ = unsafe { sha512sum1(0) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::crypto::zkn::aes64ds as _;
//! use rust_isa::riscv::crypto::zkn::aes64dsm as _;
//! use rust_isa::riscv::crypto::zkn::aes64es as _;
//! use rust_isa::riscv::crypto::zkn::aes64esm as _;
//! use rust_isa::riscv::crypto::zkn::aes64im as _;
//! use rust_isa::riscv::crypto::zkn::aes64ks1i as _;
//! use rust_isa::riscv::crypto::zkn::aes64ks2 as _;
//! use rust_isa::riscv::crypto::zkn::sha256sig0 as _;
//! use rust_isa::riscv::crypto::zkn::sha256sig1 as _;
//! use rust_isa::riscv::crypto::zkn::sha256sum0 as _;
//! use rust_isa::riscv::crypto::zkn::sha256sum1 as _;
//! use rust_isa::riscv::crypto::zkn::sha512sig0 as _;
//! use rust_isa::riscv::crypto::zkn::sha512sig1 as _;
//! use rust_isa::riscv::crypto::zkn::sha512sum0 as _;
//! use rust_isa::riscv::crypto::zkn::sha512sum1 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_riscv_zkn_sha256sig0(value: u32) -> u32;
    fn rust_isa_riscv_zkn_sha256sig1(value: u32) -> u32;
    fn rust_isa_riscv_zkn_sha256sum0(value: u32) -> u32;
    fn rust_isa_riscv_zkn_sha256sum1(value: u32) -> u32;
    fn rust_isa_riscv_zkn_sha512sig0(value: u64) -> u64;
    fn rust_isa_riscv_zkn_sha512sig1(value: u64) -> u64;
    fn rust_isa_riscv_zkn_sha512sum0(value: u64) -> u64;
    fn rust_isa_riscv_zkn_sha512sum1(value: u64) -> u64;
    fn rust_isa_riscv_zkn_aes64es(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zkn_aes64ds(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zkn_aes64esm(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zkn_aes64dsm(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zkn_aes64im(value: u64) -> u64;
    fn rust_isa_riscv_zkn_aes64ks2(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zkn_aes64ks1i(value: u64, round: u32) -> u64;
}

macro_rules! sha {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zkn.
        #[target_feature(enable = "zkn")]
        #[inline]
        pub unsafe fn $name(value: $ty) -> $ty {
            unsafe { $c(value) }
        }
    };
}

macro_rules! aes {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zkn.
        #[target_feature(enable = "zkn")]
        #[inline]
        pub unsafe fn $name(a: u64, b: u64) -> u64 {
            unsafe { $c(a, b) }
        }
    };
}

sha!(
    /// SHA-256 `Σ0`.
    sha256sig0,
    rust_isa_riscv_zkn_sha256sig0,
    u32
);
sha!(
    /// SHA-256 `Σ1`.
    sha256sig1,
    rust_isa_riscv_zkn_sha256sig1,
    u32
);
sha!(
    /// SHA-256 `σ0`.
    sha256sum0,
    rust_isa_riscv_zkn_sha256sum0,
    u32
);
sha!(
    /// SHA-256 `σ1`.
    sha256sum1,
    rust_isa_riscv_zkn_sha256sum1,
    u32
);
sha!(
    /// SHA-512 `Σ0`.
    sha512sig0,
    rust_isa_riscv_zkn_sha512sig0,
    u64
);
sha!(
    /// SHA-512 `Σ1`.
    sha512sig1,
    rust_isa_riscv_zkn_sha512sig1,
    u64
);
sha!(
    /// SHA-512 `σ0`.
    sha512sum0,
    rust_isa_riscv_zkn_sha512sum0,
    u64
);
sha!(
    /// SHA-512 `σ1`.
    sha512sum1,
    rust_isa_riscv_zkn_sha512sum1,
    u64
);
aes!(
    /// AES encryption round.
    aes64es,
    rust_isa_riscv_zkn_aes64es
);
aes!(
    /// AES decryption round.
    aes64ds,
    rust_isa_riscv_zkn_aes64ds
);
aes!(
    /// AES encryption round with MixColumns.
    aes64esm,
    rust_isa_riscv_zkn_aes64esm
);
aes!(
    /// AES decryption round with inverse MixColumns.
    aes64dsm,
    rust_isa_riscv_zkn_aes64dsm
);
sha!(
    /// AES inverse MixColumns.
    aes64im,
    rust_isa_riscv_zkn_aes64im,
    u64
);
aes!(
    /// AES key-schedule step 2.
    aes64ks2,
    rust_isa_riscv_zkn_aes64ks2
);

/// `aes64ks1i`
///
/// Perform one AES key-schedule round with the round number `round`.
///
/// # Safety
///
/// The caller must enable ZKN.
#[target_feature(enable = "zkn")]
#[inline]
pub unsafe fn aes64ks1i(value: u64, round: u8) -> u64 {
    unsafe { rust_isa_riscv_zkn_aes64ks1i(value, u32::from(round)) }
}
