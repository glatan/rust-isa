//! XOP.
//!
//! XOP is an AMD extension and is not part of the x86-64 baseline. rustc does
//! not allow `#[target_feature(enable = "xop")]` on stable, so the caller must
//! compile the whole crate with the feature enabled and the CPU must support
//! XOP. The wrappers do not carry `#[target_feature]`.
//!
//! Every instruction wrapper is `unsafe`. The values are [`V128`]. The module exposes the
//! 128-bit XOP permute, compare, rotate, shift, multiply-accumulate,
//! fractional, and horizontal add/subtract instructions.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_xop_vpperm(a: *const u8, b: *const u8, control: *const u8, out: *mut u8);
}

/// `vpperm`
///
/// Permute bytes from `a` and `b` using `control`.
///
/// # Safety
///
/// The caller must compile with XOP enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn vpperm(a: V128, b: V128, control: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_xop_vpperm(a.as_ptr(), b.as_ptr(), control.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_xop_vpcmov(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpcomb(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_xop_vpcomw(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_xop_vpcomd(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_xop_vpcomq(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_xop_vpcomub(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_xop_vpcomuw(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_xop_vpcomud(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_xop_vpcomuq(a: *const u8, b: *const u8, imm: u32, out: *mut u8);
    fn rust_isa_xop_vprotb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vprotw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vprotd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vprotq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpshab(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpshaw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpshad(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpshaq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpshlb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpshlw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpshld(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpshlq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacssww(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacsww(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacsswd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacswd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacssdql(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacsdql(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacssdqh(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacsdqh(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacssdd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacsdd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmadcsswd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmadcswd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacssdu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpmacsdu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_xop_vpermil2ps(
        a: *const u8,
        b: *const u8,
        control: *const u8,
        imm: u32,
        out: *mut u8,
    );
    fn rust_isa_xop_vpermil2pd(
        a: *const u8,
        b: *const u8,
        control: *const u8,
        imm: u32,
        out: *mut u8,
    );
    fn rust_isa_xop_vfrczps(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vfrczpd(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vfrczss(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vfrczsd(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddbw(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddbd(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddbq(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddwd(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddwq(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphadddq(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddubw(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddubd(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddubq(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphadduwd(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphadduwq(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphaddudq(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphsubbw(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphsubwd(a: *const u8, out: *mut u8);
    fn rust_isa_xop_vphsubdq(a: *const u8, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with XOP enabled and the CPU must support it.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with XOP enabled and the CPU must support it.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with XOP enabled and the CPU must support it.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! compare {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// `imm` selects the predicate in `0..=7`.
        ///
        /// # Safety
        ///
        /// The caller must compile with XOP enabled and the CPU must support it.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128, imm: u8) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), u32::from(imm), out.as_mut_ptr()); }
            out
        }
    };
}
unary!(
    /// `vfrczps`
    ///
    /// Extract the fractional part of each 32-bit float lane.
    vfrczps,
    rust_isa_xop_vfrczps
);
unary!(
    /// `vfrczpd`
    ///
    /// Extract the fractional part of each 64-bit float lane.
    vfrczpd,
    rust_isa_xop_vfrczpd
);
unary!(
    /// `vfrczss`
    ///
    /// Extract the fractional part of the low 32-bit float lane.
    vfrczss,
    rust_isa_xop_vfrczss
);
unary!(
    /// `vfrczsd`
    ///
    /// Extract the fractional part of the low 64-bit float lane.
    vfrczsd,
    rust_isa_xop_vfrczsd
);
unary!(
    /// `vphaddbw`
    ///
    /// Add adjacent byte pairs and widen to 16-bit lanes.
    vphaddbw,
    rust_isa_xop_vphaddbw
);
unary!(
    /// `vphaddbd`
    ///
    /// Add adjacent byte quads and widen to 32-bit lanes.
    vphaddbd,
    rust_isa_xop_vphaddbd
);
unary!(
    /// `vphaddbq`
    ///
    /// Add adjacent byte groups of eight and widen to 64-bit lanes.
    vphaddbq,
    rust_isa_xop_vphaddbq
);
unary!(
    /// `vphaddwd`
    ///
    /// Add adjacent 16-bit pairs and widen to 32-bit lanes.
    vphaddwd,
    rust_isa_xop_vphaddwd
);
unary!(
    /// `vphaddwq`
    ///
    /// Add adjacent 16-bit quads and widen to 64-bit lanes.
    vphaddwq,
    rust_isa_xop_vphaddwq
);
unary!(
    /// `vphadddq`
    ///
    /// Add adjacent 32-bit pairs and widen to 64-bit lanes.
    vphadddq,
    rust_isa_xop_vphadddq
);
unary!(
    /// `vphaddubw`
    ///
    /// Add adjacent unsigned byte pairs and widen to 16-bit lanes.
    vphaddubw,
    rust_isa_xop_vphaddubw
);
unary!(
    /// `vphaddubd`
    ///
    /// Add adjacent unsigned byte quads and widen to 32-bit lanes.
    vphaddubd,
    rust_isa_xop_vphaddubd
);
unary!(
    /// `vphaddubq`
    ///
    /// Add adjacent unsigned byte groups of eight and widen to 64-bit lanes.
    vphaddubq,
    rust_isa_xop_vphaddubq
);
unary!(
    /// `vphadduwd`
    ///
    /// Add adjacent unsigned 16-bit pairs and widen to 32-bit lanes.
    vphadduwd,
    rust_isa_xop_vphadduwd
);
unary!(
    /// `vphadduwq`
    ///
    /// Add adjacent unsigned 16-bit quads and widen to 64-bit lanes.
    vphadduwq,
    rust_isa_xop_vphadduwq
);
unary!(
    /// `vphaddudq`
    ///
    /// Add adjacent unsigned 32-bit pairs and widen to 64-bit lanes.
    vphaddudq,
    rust_isa_xop_vphaddudq
);
unary!(
    /// `vphsubbw`
    ///
    /// Subtract adjacent byte pairs and widen to 16-bit lanes.
    vphsubbw,
    rust_isa_xop_vphsubbw
);
unary!(
    /// `vphsubwd`
    ///
    /// Subtract adjacent 16-bit pairs and widen to 32-bit lanes.
    vphsubwd,
    rust_isa_xop_vphsubwd
);
unary!(
    /// `vphsubdq`
    ///
    /// Subtract adjacent 32-bit pairs and widen to 64-bit lanes.
    vphsubdq,
    rust_isa_xop_vphsubdq
);
binary!(
    /// `vprotb`
    ///
    /// Rotate each byte by the count in the corresponding byte of `b`.
    vprotb,
    rust_isa_xop_vprotb
);
binary!(
    /// `vprotw`
    ///
    /// Rotate each 16-bit lane by the count in the corresponding lane of `b`.
    vprotw,
    rust_isa_xop_vprotw
);
binary!(
    /// `vprotd`
    ///
    /// Rotate each 32-bit lane by the count in the corresponding lane of `b`.
    vprotd,
    rust_isa_xop_vprotd
);
binary!(
    /// `vprotq`
    ///
    /// Rotate each 64-bit lane by the count in the corresponding lane of `b`.
    vprotq,
    rust_isa_xop_vprotq
);
binary!(
    /// `vpshab`
    ///
    /// Arithmetic-shift each byte by the signed count in `b`.
    vpshab,
    rust_isa_xop_vpshab
);
binary!(
    /// `vpshaw`
    ///
    /// Arithmetic-shift each 16-bit lane by the signed count in `b`.
    vpshaw,
    rust_isa_xop_vpshaw
);
binary!(
    /// `vpshad`
    ///
    /// Arithmetic-shift each 32-bit lane by the signed count in `b`.
    vpshad,
    rust_isa_xop_vpshad
);
binary!(
    /// `vpshaq`
    ///
    /// Arithmetic-shift each 64-bit lane by the signed count in `b`.
    vpshaq,
    rust_isa_xop_vpshaq
);
binary!(
    /// `vpshlb`
    ///
    /// Logical-shift each byte by the signed count in `b`.
    vpshlb,
    rust_isa_xop_vpshlb
);
binary!(
    /// `vpshlw`
    ///
    /// Logical-shift each 16-bit lane by the signed count in `b`.
    vpshlw,
    rust_isa_xop_vpshlw
);
binary!(
    /// `vpshld`
    ///
    /// Logical-shift each 32-bit lane by the signed count in `b`.
    vpshld,
    rust_isa_xop_vpshld
);
binary!(
    /// `vpshlq`
    ///
    /// Logical-shift each 64-bit lane by the signed count in `b`.
    vpshlq,
    rust_isa_xop_vpshlq
);
ternary!(
    /// `vpcmov`
    ///
    /// Select bits from `a` and `b` using `c` as the mask.
    vpcmov,
    rust_isa_xop_vpcmov
);
ternary!(
    /// `vpmacssww`
    ///
    /// Signed saturating word multiply-accumulate into `c`.
    vpmacssww,
    rust_isa_xop_vpmacssww
);
ternary!(
    /// `vpmacsww`
    ///
    /// Signed word multiply-accumulate into `c`.
    vpmacsww,
    rust_isa_xop_vpmacsww
);
ternary!(
    /// `vpmacsswd`
    ///
    /// Signed saturating word multiply-accumulate into 32-bit lanes of `c`.
    vpmacsswd,
    rust_isa_xop_vpmacsswd
);
ternary!(
    /// `vpmacswd`
    ///
    /// Signed word multiply-accumulate into 32-bit lanes of `c`.
    vpmacswd,
    rust_isa_xop_vpmacswd
);
ternary!(
    /// `vpmacssdql`
    ///
    /// Signed saturating dword multiply-accumulate into the low qword of `c`.
    vpmacssdql,
    rust_isa_xop_vpmacssdql
);
ternary!(
    /// `vpmacsdql`
    ///
    /// Signed dword multiply-accumulate into the low qword of `c`.
    vpmacsdql,
    rust_isa_xop_vpmacsdql
);
ternary!(
    /// `vpmacssdqh`
    ///
    /// Signed saturating dword multiply-accumulate into the high qword of `c`.
    vpmacssdqh,
    rust_isa_xop_vpmacssdqh
);
ternary!(
    /// `vpmacsdqh`
    ///
    /// Signed dword multiply-accumulate into the high qword of `c`.
    vpmacsdqh,
    rust_isa_xop_vpmacsdqh
);
ternary!(
    /// `vpmacssdd`
    ///
    /// Signed saturating dword multiply-accumulate into `c`.
    vpmacssdd,
    rust_isa_xop_vpmacssdd
);
ternary!(
    /// `vpmacsdd`
    ///
    /// Signed dword multiply-accumulate into `c`.
    vpmacsdd,
    rust_isa_xop_vpmacsdd
);
ternary!(
    /// `vpmadcsswd`
    ///
    /// Signed saturating word multiply-add into 32-bit lanes of `c`.
    vpmadcsswd,
    rust_isa_xop_vpmadcsswd
);
ternary!(
    /// `vpmadcswd`
    ///
    /// Signed word multiply-add into 32-bit lanes of `c`.
    vpmadcswd,
    rust_isa_xop_vpmadcswd
);
ternary!(
    /// `vpmacssdu`
    ///
    /// Signed saturating multiply-accumulate of signed dwords in `a` by
    /// unsigned dwords in `b` into `c`.
    ///
    /// LLVM has no encoding for this instruction. The wrapper computes the low
    /// 32 bits of each product, adds the corresponding lane of `c`, and
    /// saturates the result.
    vpmacssdu,
    rust_isa_xop_vpmacssdu
);
ternary!(
    /// `vpmacsdu`
    ///
    /// Multiply-accumulate of signed dwords in `a` by unsigned dwords in `b`
    /// into `c`.
    ///
    /// LLVM has no encoding for this instruction. The wrapper computes the low
    /// 32 bits of each product and adds the corresponding lane of `c`.
    vpmacsdu,
    rust_isa_xop_vpmacsdu
);
/// `vpermil2ps`
///
/// Permute four 32-bit float lanes from `a` and `b` using `control`. The low
/// two bits of `imm` select the mask and zeroing mode.
///
/// # Safety
///
/// The caller must compile with XOP enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn vpermil2ps(a: V128, b: V128, control: V128, imm: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_xop_vpermil2ps(
            a.as_ptr(),
            b.as_ptr(),
            control.as_ptr(),
            u32::from(imm & 0x03),
            out.as_mut_ptr(),
        );
    }
    out
}
/// `vpermil2pd`
///
/// Permute two 64-bit float lanes from `a` and `b` using `control`. The low
/// two bits of `imm` select the mask and zeroing mode.
///
/// # Safety
///
/// The caller must compile with XOP enabled and the CPU must support it.
#[inline(always)]
pub unsafe fn vpermil2pd(a: V128, b: V128, control: V128, imm: u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_xop_vpermil2pd(
            a.as_ptr(),
            b.as_ptr(),
            control.as_ptr(),
            u32::from(imm & 0x03),
            out.as_mut_ptr(),
        );
    }
    out
}
compare!(
    /// `vpcomb`
    ///
    /// Compare signed bytes.
    vpcomb,
    rust_isa_xop_vpcomb
);
compare!(
    /// `vpcomw`
    ///
    /// Compare signed 16-bit lanes.
    vpcomw,
    rust_isa_xop_vpcomw
);
compare!(
    /// `vpcomd`
    ///
    /// Compare signed 32-bit lanes.
    vpcomd,
    rust_isa_xop_vpcomd
);
compare!(
    /// `vpcomq`
    ///
    /// Compare signed 64-bit lanes.
    vpcomq,
    rust_isa_xop_vpcomq
);
compare!(
    /// `vpcomub`
    ///
    /// Compare unsigned bytes.
    vpcomub,
    rust_isa_xop_vpcomub
);
compare!(
    /// `vpcomuw`
    ///
    /// Compare unsigned 16-bit lanes.
    vpcomuw,
    rust_isa_xop_vpcomuw
);
compare!(
    /// `vpcomud`
    ///
    /// Compare unsigned 32-bit lanes.
    vpcomud,
    rust_isa_xop_vpcomud
);
compare!(
    /// `vpcomuq`
    ///
    /// Compare unsigned 64-bit lanes.
    vpcomuq,
    rust_isa_xop_vpcomuq
);
