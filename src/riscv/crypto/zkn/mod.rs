//! RISC-V Zkn crypto.
//!
//! Zkn is a stable scalar cryptography extension. Every operation is `unsafe`
//! and the caller must enable the required sub-extension (`zkne`, `zknd`,
//! `zknh`) with `#[target_feature]` or a global target feature.
//!
//! The functions operate on scalar registers. The caller combines them with the
//! AES or SHA specification. The AES key-schedule instruction that takes a
//! round number immediate is intentionally omitted.

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
}

macro_rules! sha {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
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
