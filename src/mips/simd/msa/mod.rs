//! MSA.
//!
//! MSA is unstable. The caller must compile with
//! `-C target-feature=+msa,+fp64`. Every instruction wrapper is `unsafe`. The values are
//! [`V128`]. Immediate operands are const generics unless documented
//! otherwise.
//!
//! Instruction behavior follows the [MIPS Architecture](https://www.mips.com/products/architectures/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::mips::simd::msa::add as _;
//! use rust_isa::mips::simd::msa::add_a as _;
//! use rust_isa::mips::simd::msa::add_i16 as _;
//! use rust_isa::mips::simd::msa::add_i64 as _;
//! use rust_isa::mips::simd::msa::add_i8 as _;
//! use rust_isa::mips::simd::msa::adds_a as _;
//! use rust_isa::mips::simd::msa::adds_s as _;
//! use rust_isa::mips::simd::msa::adds_u as _;
//! use rust_isa::mips::simd::msa::and as _;
//! use rust_isa::mips::simd::msa::bclr as _;
//! use rust_isa::mips::simd::msa::binsl as _;
//! use rust_isa::mips::simd::msa::binsr as _;
//! use rust_isa::mips::simd::msa::bneg as _;
//! use rust_isa::mips::simd::msa::bset as _;
//! use rust_isa::mips::simd::msa::cmpeq_i32 as _;
//! use rust_isa::mips::simd::msa::cmpgt_i32 as _;
//! use rust_isa::mips::simd::msa::div_s as _;
//! use rust_isa::mips::simd::msa::div_u as _;
//! use rust_isa::mips::simd::msa::fadd as _;
//! use rust_isa::mips::simd::msa::fadd_f64 as _;
//! use rust_isa::mips::simd::msa::fclass as _;
//! use rust_isa::mips::simd::msa::fcmp as _;
//! use rust_isa::mips::simd::msa::fdiv as _;
//! use rust_isa::mips::simd::msa::fdiv_f64 as _;
//! use rust_isa::mips::simd::msa::fexp2 as _;
//! use rust_isa::mips::simd::msa::ffint as _;
//! use rust_isa::mips::simd::msa::ffint_s as _;
//! use rust_isa::mips::simd::msa::ffint_u as _;
//! use rust_isa::mips::simd::msa::flog2 as _;
//! use rust_isa::mips::simd::msa::fmax as _;
//! use rust_isa::mips::simd::msa::fmin as _;
//! use rust_isa::mips::simd::msa::fmul as _;
//! use rust_isa::mips::simd::msa::fmul_f64 as _;
//! use rust_isa::mips::simd::msa::frcp as _;
//! use rust_isa::mips::simd::msa::frsqrt as _;
//! use rust_isa::mips::simd::msa::fsqrt as _;
//! use rust_isa::mips::simd::msa::fsub as _;
//! use rust_isa::mips::simd::msa::fsub_f64 as _;
//! use rust_isa::mips::simd::msa::ftint as _;
//! use rust_isa::mips::simd::msa::ftint_s as _;
//! use rust_isa::mips::simd::msa::ftint_u as _;
//! use rust_isa::mips::simd::msa::max_s as _;
//! use rust_isa::mips::simd::msa::max_u as _;
//! use rust_isa::mips::simd::msa::min_s as _;
//! use rust_isa::mips::simd::msa::min_u as _;
//! use rust_isa::mips::simd::msa::mod_s as _;
//! use rust_isa::mips::simd::msa::mod_u as _;
//! use rust_isa::mips::simd::msa::mul as _;
//! use rust_isa::mips::simd::msa::mul_i16 as _;
//! use rust_isa::mips::simd::msa::mul_i32 as _;
//! use rust_isa::mips::simd::msa::nloc as _;
//! use rust_isa::mips::simd::msa::nlzc as _;
//! use rust_isa::mips::simd::msa::nor as _;
//! use rust_isa::mips::simd::msa::nor_v as _;
//! use rust_isa::mips::simd::msa::or as _;
//! use rust_isa::mips::simd::msa::pcnt as _;
//! use rust_isa::mips::simd::msa::sat_s as _;
//! use rust_isa::mips::simd::msa::sat_u as _;
//! use rust_isa::mips::simd::msa::seq as _;
//! use rust_isa::mips::simd::msa::shf as _;
//! use rust_isa::mips::simd::msa::sld as _;
//! use rust_isa::mips::simd::msa::sle_s as _;
//! use rust_isa::mips::simd::msa::sle_u as _;
//! use rust_isa::mips::simd::msa::sll as _;
//! use rust_isa::mips::simd::msa::slli as _;
//! use rust_isa::mips::simd::msa::slt_s as _;
//! use rust_isa::mips::simd::msa::slt_u as _;
//! use rust_isa::mips::simd::msa::splat_i32 as _;
//! use rust_isa::mips::simd::msa::sra as _;
//! use rust_isa::mips::simd::msa::srai as _;
//! use rust_isa::mips::simd::msa::srl as _;
//! use rust_isa::mips::simd::msa::srli as _;
//! use rust_isa::mips::simd::msa::sub_i16 as _;
//! use rust_isa::mips::simd::msa::sub_i32 as _;
//! use rust_isa::mips::simd::msa::sub_i64 as _;
//! use rust_isa::mips::simd::msa::sub_i8 as _;
//! use rust_isa::mips::simd::msa::subs_s as _;
//! use rust_isa::mips::simd::msa::subs_u as _;
//! use rust_isa::mips::simd::msa::v128_load as _;
//! use rust_isa::mips::simd::msa::v128_store as _;
//! use rust_isa::mips::simd::msa::vshf as _;
//! use rust_isa::mips::simd::msa::xor as _;
//! ```
//! <!-- doctest-inventory:end -->

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_msa_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `addv.w`
///
/// Add the four 32-bit integer lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::mips::simd::msa::{add, V128};
///
/// let a = V128::from_bytes([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
/// let b = V128::from_bytes([2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0]);
/// let sum = unsafe { add(a, b) };
/// assert_eq!(
///     sum.to_bytes(),
///     [3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0]
/// );
/// ```
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mulv.w`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_msa_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with MSA enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `addv.b`.
    add_i8,
    rust_isa_msa_add_i8
);
binary!(
    /// `addv.h`.
    add_i16,
    rust_isa_msa_add_i16
);
binary!(
    /// `addv.d`.
    add_i64,
    rust_isa_msa_add_i64
);
binary!(
    /// `subv.b`.
    sub_i8,
    rust_isa_msa_sub_i8
);
binary!(
    /// `subv.h`.
    sub_i16,
    rust_isa_msa_sub_i16
);
binary!(
    /// `subv.w`.
    sub_i32,
    rust_isa_msa_sub_i32
);
binary!(
    /// `subv.d`.
    sub_i64,
    rust_isa_msa_sub_i64
);
binary!(
    /// `mulv.h`, keeping the low 16 bits.
    mul_i16,
    rust_isa_msa_mul_i16
);
binary!(
    /// `mulv.w`, keeping the low 32 bits.
    mul_i32,
    rust_isa_msa_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_msa_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_msa_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_msa_xor
);
binary!(
    /// `ceq.w`. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_msa_cmpeq_i32
);
binary!(
    /// `clt.s.w`. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_msa_cmpgt_i32
);
binary!(
    /// `fadd.w`.
    fadd,
    rust_isa_msa_fadd
);
binary!(
    /// `fsub.w`.
    fsub,
    rust_isa_msa_fsub
);
binary!(
    /// `fmul.w`.
    fmul,
    rust_isa_msa_fmul
);
binary!(
    /// `fdiv.w`.
    fdiv,
    rust_isa_msa_fdiv
);

/// Broadcast a 32-bit value to all four lanes.
///
/// # Safety
///
/// The caller must enable MSA on the CPU and in the target features.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_splat_i32(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_msa_fadd_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fsub_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fmul_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fdiv_f64(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary_f64 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with MSA enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary_f64!(
    /// `fadd.d` on 64-bit float lanes.
    fadd_f64,
    rust_isa_msa_fadd_f64
);
binary_f64!(
    /// `fsub.d` on 64-bit float lanes.
    fsub_f64,
    rust_isa_msa_fsub_f64
);
binary_f64!(
    /// `fmul.d` on 64-bit float lanes.
    fmul_f64,
    rust_isa_msa_fmul_f64
);
binary_f64!(
    /// `fdiv.d` on 64-bit float lanes.
    fdiv_f64,
    rust_isa_msa_fdiv_f64
);

unsafe extern "C" {
    fn rust_isa_msa_div_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_div_u(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_mod_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_mod_u(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_add_a(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_adds_a(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_adds_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_adds_u(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_subs_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_subs_u(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_max_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_max_u(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_min_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_min_u(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_nor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_seq(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sle_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sle_u(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_slt_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_slt_u(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sll(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_srl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_sra(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_slli(a: *const u8, out: *mut u8, shift: u32);
    fn rust_isa_msa_srli(a: *const u8, out: *mut u8, shift: u32);
    fn rust_isa_msa_srai(a: *const u8, out: *mut u8, shift: u32);
    fn rust_isa_msa_bclr(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_bset(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_bneg(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_binsl(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_msa_binsr(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_msa_sat_s(a: *const u8, out: *mut u8, bits: u32);
    fn rust_isa_msa_sat_u(a: *const u8, out: *mut u8, bits: u32);
    fn rust_isa_msa_sld(dest: *const u8, src: *const u8, index: u32, out: *mut u8);
    fn rust_isa_msa_vshf(a: *const u8, b: *const u8, control: *const u8, out: *mut u8);
    fn rust_isa_msa_shf(a: *const u8, out: *mut u8, imm: u32);
    fn rust_isa_msa_pcnt(a: *const u8, out: *mut u8);
    fn rust_isa_msa_nlzc(a: *const u8, out: *mut u8);
    fn rust_isa_msa_nloc(a: *const u8, out: *mut u8);
    fn rust_isa_msa_fmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_fexp2(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_msa_flog2(a: *const u8, out: *mut u8);
    fn rust_isa_msa_frcp(a: *const u8, out: *mut u8);
    fn rust_isa_msa_frsqrt(a: *const u8, out: *mut u8);
    fn rust_isa_msa_fsqrt(a: *const u8, out: *mut u8);
    fn rust_isa_msa_ftint_s(a: *const u8, out: *mut u8);
    fn rust_isa_msa_ftint_u(a: *const u8, out: *mut u8);
    fn rust_isa_msa_ffint_s(a: *const u8, out: *mut u8);
    fn rust_isa_msa_ffint_u(a: *const u8, out: *mut u8);
    fn rust_isa_msa_fclass(a: *const u8, out: *mut u8);
    fn rust_isa_msa_fcmp(a: *const u8, b: *const u8, out: *mut u8, cond: u32);
}

binary!(
    /// `div_s.w`. Signed 32-bit lanes. A zero divisor produces `-1` for a
    /// non-negative dividend and `1` for a negative dividend.
    div_s,
    rust_isa_msa_div_s
);
binary!(
    /// `div_u.w`. Unsigned 32-bit lanes. A zero divisor produces all bits one.
    div_u,
    rust_isa_msa_div_u
);
binary!(
    /// `mod_s.w`. Signed 32-bit lanes. A zero divisor returns the dividend.
    mod_s,
    rust_isa_msa_mod_s
);
binary!(
    /// `mod_u.w`. Unsigned 32-bit lanes. A zero divisor returns the dividend.
    mod_u,
    rust_isa_msa_mod_u
);
binary!(
    /// `add_a.w`. Add the absolute values of signed 32-bit lanes, keeping the
    /// low 32 bits.
    add_a,
    rust_isa_msa_add_a
);
binary!(
    /// `adds_a.w`. Add the absolute values of signed 32-bit lanes with unsigned
    /// saturation.
    adds_a,
    rust_isa_msa_adds_a
);
binary!(
    /// `adds_s.w`. Signed saturating add on 32-bit lanes.
    adds_s,
    rust_isa_msa_adds_s
);
binary!(
    /// `adds_u.w`. Unsigned saturating add on 32-bit lanes.
    adds_u,
    rust_isa_msa_adds_u
);
binary!(
    /// `subs_s.w`. Signed saturating subtract on 32-bit lanes.
    subs_s,
    rust_isa_msa_subs_s
);
binary!(
    /// `subs_u.w`. Unsigned saturating subtract on 32-bit lanes.
    subs_u,
    rust_isa_msa_subs_u
);
binary!(
    /// `max_s.w`. Maximum of signed 32-bit lanes.
    max_s,
    rust_isa_msa_max_s
);
binary!(
    /// `max_u.w`. Maximum of unsigned 32-bit lanes.
    max_u,
    rust_isa_msa_max_u
);
binary!(
    /// `min_s.w`. Minimum of signed 32-bit lanes.
    min_s,
    rust_isa_msa_min_s
);
binary!(
    /// `min_u.w`. Minimum of unsigned 32-bit lanes.
    min_u,
    rust_isa_msa_min_u
);
binary!(
    /// `nor.v`. Bitwise NOR.
    nor,
    rust_isa_msa_nor
);
binary!(
    /// `ceq.w`. A true 32-bit lane is all bits one.
    seq,
    rust_isa_msa_seq
);
binary!(
    /// `cle_s.w`. Signed less-than-or-equal on 32-bit lanes. A true lane is
    /// all bits one.
    sle_s,
    rust_isa_msa_sle_s
);
binary!(
    /// `cle_u.w`. Unsigned less-than-or-equal on 32-bit lanes. A true lane is
    /// all bits one.
    sle_u,
    rust_isa_msa_sle_u
);
binary!(
    /// `clt_s.w`. Signed less-than on 32-bit lanes. A true lane is all bits
    /// one.
    slt_s,
    rust_isa_msa_slt_s
);
binary!(
    /// `clt_u.w`. Unsigned less-than on 32-bit lanes. A true lane is all bits
    /// one.
    slt_u,
    rust_isa_msa_slt_u
);
binary!(
    /// `sll.w`. Shift each 32-bit lane left by the low five bits of the
    /// corresponding lane of `b`.
    sll,
    rust_isa_msa_sll
);
binary!(
    /// `srl.w`. Shift each 32-bit lane right logically by the low five bits of
    /// the corresponding lane of `b`.
    srl,
    rust_isa_msa_srl
);
binary!(
    /// `sra.w`. Shift each 32-bit lane right arithmetically by the low five
    /// bits of the corresponding lane of `b`.
    sra,
    rust_isa_msa_sra
);
binary!(
    /// `bclr.w`. Clear the bit selected by the low five bits of each 32-bit
    /// lane of `b`.
    bclr,
    rust_isa_msa_bclr
);
binary!(
    /// `bset.w`. Set the bit selected by the low five bits of each 32-bit lane
    /// of `b`.
    bset,
    rust_isa_msa_bset
);
binary!(
    /// `bneg.w`. Toggle the bit selected by the low five bits of each 32-bit
    /// lane of `b`.
    bneg,
    rust_isa_msa_bneg
);
binary!(
    /// `fmax.w`. Maximum of 32-bit float lanes. A single NaN operand returns
    /// the other operand.
    fmax,
    rust_isa_msa_fmax
);
binary!(
    /// `fmin.w`. Minimum of 32-bit float lanes. A single NaN operand returns
    /// the other operand.
    fmin,
    rust_isa_msa_fmin
);

/// `nor.v`, an alias for [`nor`].
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn nor_v(a: V128, b: V128) -> V128 {
    unsafe { nor(a, b) }
}

/// `slli.w`. Shift each 32-bit lane left by `SHIFT`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn slli<const SHIFT: u32>(a: V128) -> V128 {
    const { assert!(SHIFT < 32, "slli shift must be in 0..=31") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_slli(a.as_ptr(), out.as_mut_ptr(), SHIFT);
    }
    out
}

/// `srli.w`. Shift each 32-bit lane right logically by `SHIFT`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn srli<const SHIFT: u32>(a: V128) -> V128 {
    const { assert!(SHIFT < 32, "srli shift must be in 0..=31") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_srli(a.as_ptr(), out.as_mut_ptr(), SHIFT);
    }
    out
}

/// `srai.w`. Shift each 32-bit lane right arithmetically by `SHIFT`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn srai<const SHIFT: u32>(a: V128) -> V128 {
    const { assert!(SHIFT < 32, "srai shift must be in 0..=31") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_srai(a.as_ptr(), out.as_mut_ptr(), SHIFT);
    }
    out
}

/// `binsl.w`. Insert bits from `src` into `dest` from the left, using the low
/// five bits of each lane of `control`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn binsl(dest: V128, src: V128, control: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_binsl(
            dest.as_ptr(),
            src.as_ptr(),
            control.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `binsr.w`. Insert bits from `src` into `dest` from the right, using the low
/// five bits of each lane of `control`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn binsr(dest: V128, src: V128, control: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_binsr(
            dest.as_ptr(),
            src.as_ptr(),
            control.as_ptr(),
            out.as_mut_ptr(),
        );
    }
    out
}

/// `sat_s.w`. Saturate signed 32-bit lanes to `BITS` bits, where `BITS` is in
/// `1..=32`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn sat_s<const BITS: u32>(a: V128) -> V128 {
    const { assert!(BITS >= 1 && BITS <= 32, "sat_s width must be in 1..=32") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_sat_s(a.as_ptr(), out.as_mut_ptr(), BITS);
    }
    out
}

/// `sat_u.w`. Saturate unsigned 32-bit lanes to `BITS` bits, where `BITS` is
/// in `1..=32`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn sat_u<const BITS: u32>(a: V128) -> V128 {
    const { assert!(BITS >= 1 && BITS <= 32, "sat_u width must be in 1..=32") };
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_sat_u(a.as_ptr(), out.as_mut_ptr(), BITS);
    }
    out
}

/// `sld.w`. Concatenate `src` and `dest`, then slide left by `INDEX` 32-bit
/// lanes. Only `INDEX % 4` is used.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn sld(dest: V128, src: V128, index: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_sld(dest.as_ptr(), src.as_ptr(), index, out.as_mut_ptr());
    }
    out
}

/// `vshf.w`. Shuffle 32-bit lanes. Each lane of `control` selects a source
/// lane: values `0..=3` select from `b`, values `4..=7` select from `a`, and a
/// control lane with any of bits 6 and 7 set produces zero.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn vshf(a: V128, b: V128, control: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_vshf(a.as_ptr(), b.as_ptr(), control.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `shf.w`. Permute four 32-bit lanes. Bits `2 * i..=2 * i + 1` of `IMM`
/// select the source lane for output lane `i`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn shf<const IMM: u8>(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_shf(a.as_ptr(), out.as_mut_ptr(), IMM as u32);
    }
    out
}

/// `pcnt.w`. Count the set bits in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn pcnt(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_pcnt(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `nlzc.w`. Count the leading zero bits in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn nlzc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_nlzc(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `nloc.w`. Count the leading one bits in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn nloc(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_nloc(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fexp2.w`. Compute `a * 2^b` for each 32-bit float lane, where `b` holds
/// the integer exponent bit patterns.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn fexp2(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_fexp2(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `flog2.w`. Base-2 logarithm of each 32-bit float lane.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn flog2(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_flog2(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `frcp.w`. Reciprocal of each 32-bit float lane.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn frcp(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_frcp(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `frsqrt.w`. Reciprocal square root of each 32-bit float lane.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn frsqrt(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_frsqrt(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `fsqrt.w`. Square root of each 32-bit float lane.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn fsqrt(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_fsqrt(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ftint_s.w`. Convert 32-bit float lanes to signed 32-bit integers using the
/// current rounding mode.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn ftint_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_ftint_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ftint_u.w`. Convert 32-bit float lanes to unsigned 32-bit integers using
/// the current rounding mode.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn ftint_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_ftint_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ftint_s.w` when `SIGNED` is true, otherwise `ftint_u.w`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn ftint<const SIGNED: bool>(a: V128) -> V128 {
    if SIGNED {
        unsafe { ftint_s(a) }
    } else {
        unsafe { ftint_u(a) }
    }
}

/// `ffint_s.w`. Convert signed 32-bit integer lanes to 32-bit floats.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn ffint_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_ffint_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ffint_u.w`. Convert unsigned 32-bit integer lanes to 32-bit floats.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn ffint_u(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_ffint_u(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ffint_s.w` when `SIGNED` is true, otherwise `ffint_u.w`.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn ffint<const SIGNED: bool>(a: V128) -> V128 {
    if SIGNED {
        unsafe { ffint_s(a) }
    } else {
        unsafe { ffint_u(a) }
    }
}

/// `fclass.w`. Classify each 32-bit float lane. The result bits are, from bit
/// 0: signaling NaN, quiet NaN, negative infinity, negative normal, negative
/// subnormal, negative zero, positive infinity, positive normal, positive
/// subnormal, and positive zero.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn fclass(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_fclass(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// A floating-point comparison condition for [`fcmp`].
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Fcmp {
    /// False.
    Af = 0,
    /// Unordered.
    Un = 1,
    /// Equal.
    Eq = 2,
    /// Unordered or equal.
    Ueq = 3,
    /// Less than.
    Lt = 4,
    /// Unordered or less than.
    Ult = 5,
    /// Less than or equal.
    Le = 6,
    /// Unordered or less than or equal.
    Ule = 7,
    /// Signaling false.
    Saf = 8,
    /// Signaling unordered.
    Sun = 9,
    /// Signaling equal.
    Seq = 10,
    /// Signaling unordered or equal.
    Suneq = 11,
    /// Signaling less than.
    Slt = 12,
    /// Signaling unordered or less than.
    Sult = 13,
    /// Signaling less than or equal.
    Sle = 14,
    /// Signaling unordered or less than or equal.
    Sule = 15,
    /// Ordered.
    Or = 16,
    /// Unordered or not equal.
    Une = 17,
    /// Not equal.
    Ne = 18,
    /// Signaling ordered.
    Sor = 19,
    /// Signaling unordered or not equal.
    Sune = 20,
    /// Signaling not equal.
    Sne = 21,
}

/// `fcmp.w` with the condition selected by `cond`. A true 32-bit lane is all
/// bits one. Signaling conditions update the MSA floating-point cause bits.
///
/// # Safety
///
/// The caller must compile with MSA enabled.
#[inline(always)]
pub unsafe fn fcmp(a: V128, b: V128, cond: Fcmp) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_msa_fcmp(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), cond as u32);
    }
    out
}
