//! RISC-V Zbb.
//!
//! Zbb is a stable extension. Every operation is `unsafe` and the caller must
//! enable Zbb with `#[target_feature(enable = "zbb")]` or a global target
//! feature.
//!
//! The functions operate on 64-bit values. `clz` and `ctz` return 64 for a zero
//! input. `rol` and `ror` use the low six bits of the shift amount.

unsafe extern "C" {
    fn rust_isa_riscv_zbb_andn(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_orn(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_xnor(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_clz(value: u64) -> u64;
    fn rust_isa_riscv_zbb_ctz(value: u64) -> u64;
    fn rust_isa_riscv_zbb_cpop(value: u64) -> u64;
    fn rust_isa_riscv_zbb_min(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_max(a: u64, b: u64) -> u64;
    fn rust_isa_riscv_zbb_sext_b(value: u64) -> u64;
    fn rust_isa_riscv_zbb_sext_h(value: u64) -> u64;
    fn rust_isa_riscv_zbb_zext_h(value: u64) -> u64;
    fn rust_isa_riscv_zbb_rol(value: u64, shift: u64) -> u64;
    fn rust_isa_riscv_zbb_ror(value: u64, shift: u64) -> u64;
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zbb.
        #[target_feature(enable = "zbb")]
        #[inline]
        pub unsafe fn $name(a: u64, b: u64) -> u64 {
            unsafe { $c(a, b) }
        }
    };
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable Zbb.
        #[target_feature(enable = "zbb")]
        #[inline]
        pub unsafe fn $name(value: u64) -> u64 {
            unsafe { $c(value) }
        }
    };
}

binary!(
    /// Bitwise AND with the complement of `b`.
    andn,
    rust_isa_riscv_zbb_andn
);
binary!(
    /// Bitwise OR with the complement of `b`.
    orn,
    rust_isa_riscv_zbb_orn
);
binary!(
    /// Bitwise XNOR.
    xnor,
    rust_isa_riscv_zbb_xnor
);
unary!(
    /// Count leading zero bits.
    clz,
    rust_isa_riscv_zbb_clz
);
unary!(
    /// Count trailing zero bits.
    ctz,
    rust_isa_riscv_zbb_ctz
);
unary!(
    /// Count set bits.
    cpop,
    rust_isa_riscv_zbb_cpop
);
binary!(
    /// Unsigned minimum.
    min,
    rust_isa_riscv_zbb_min
);
binary!(
    /// Unsigned maximum.
    max,
    rust_isa_riscv_zbb_max
);
unary!(
    /// Sign-extend the low byte.
    sext_b,
    rust_isa_riscv_zbb_sext_b
);
unary!(
    /// Sign-extend the low 16 bits.
    sext_h,
    rust_isa_riscv_zbb_sext_h
);
unary!(
    /// Zero-extend the low 16 bits.
    zext_h,
    rust_isa_riscv_zbb_zext_h
);
binary!(
    /// Rotate left by the low six bits of `b`.
    rol,
    rust_isa_riscv_zbb_rol
);
binary!(
    /// Rotate right by the low six bits of `b`.
    ror,
    rust_isa_riscv_zbb_ror
);
