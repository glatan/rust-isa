//! RISC-V Zbkb and Zbkx bit-manipulation instructions.
//!
//! These extensions are not part of the baseline. Every operation is `unsafe`
//! and the caller must enable the extension with `#[target_feature]` or a
//! global target feature.

unsafe extern "C" {
    fn rust_isa_riscv_zbkb_pack(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbkb_packh(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbkb_brev8(a: u64) -> u64;
    fn rust_isa_riscv_zbkb_xperm4(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbkb_xperm8(a: u64, b: u64) -> u64;
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[target_feature(enable = "zbkb")]
        #[inline]
        pub unsafe fn $name(a: u64, b: u64) -> u64 {
            unsafe { $c(a, b) }
        }
    };
}

binary!(
    /// `pack`.
    pack,
    rust_isa_riscv_zbkb_pack
);
binary!(
    /// `packh`.
    packh,
    rust_isa_riscv_zbkb_packh
);

/// `brev8`.
#[target_feature(enable = "zbkb")]
#[inline]
pub unsafe fn brev8(a: u64) -> u64 {
    unsafe { rust_isa_riscv_zbkb_brev8(a) }
}

/// `xperm4`.
#[target_feature(enable = "zbkx")]
#[inline]
pub unsafe fn xperm4(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbkb_xperm4(a, b) }
}

/// `xperm8`.
#[target_feature(enable = "zbkx")]
#[inline]
pub unsafe fn xperm8(a: u64, b: u64) -> u64 {
    unsafe { rust_isa_riscv_zbkb_xperm8(a, b) }
}
