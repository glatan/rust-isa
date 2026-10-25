//! NEON.
//!
//! NEON is unstable on this target. The caller must compile with
//! `-C target-feature=+neon`. Every instruction wrapper is `unsafe`. The values are
//! [`V128`]. Immediate operands are const generics unless documented
//! otherwise.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::arm::simd::neon::add as _;
//! use rust_isa::arm::simd::neon::add_i16 as _;
//! use rust_isa::arm::simd::neon::add_i64 as _;
//! use rust_isa::arm::simd::neon::add_i8 as _;
//! use rust_isa::arm::simd::neon::and as _;
//! use rust_isa::arm::simd::neon::cmpeq_i16 as _;
//! use rust_isa::arm::simd::neon::cmpeq_i32 as _;
//! use rust_isa::arm::simd::neon::cmpeq_i8 as _;
//! use rust_isa::arm::simd::neon::cmpgt_i16 as _;
//! use rust_isa::arm::simd::neon::cmpgt_i32 as _;
//! use rust_isa::arm::simd::neon::cmpgt_i8 as _;
//! use rust_isa::arm::simd::neon::eor as _;
//! use rust_isa::arm::simd::neon::fadd as _;
//! use rust_isa::arm::simd::neon::fmul as _;
//! use rust_isa::arm::simd::neon::fsub as _;
//! use rust_isa::arm::simd::neon::mul as _;
//! use rust_isa::arm::simd::neon::mul_i16 as _;
//! use rust_isa::arm::simd::neon::mul_i32 as _;
//! use rust_isa::arm::simd::neon::orr as _;
//! use rust_isa::arm::simd::neon::splat_i16 as _;
//! use rust_isa::arm::simd::neon::splat_i32 as _;
//! use rust_isa::arm::simd::neon::splat_i8 as _;
//! use rust_isa::arm::simd::neon::sub_i16 as _;
//! use rust_isa::arm::simd::neon::sub_i32 as _;
//! use rust_isa::arm::simd::neon::sub_i64 as _;
//! use rust_isa::arm::simd::neon::sub_i8 as _;
//! use rust_isa::arm::simd::neon::v128_load as _;
//! use rust_isa::arm::simd::neon::v128_store as _;
//! use rust_isa::arm::simd::neon::vaba as _;
//! use rust_isa::arm::simd::neon::vabal as _;
//! use rust_isa::arm::simd::neon::vabd as _;
//! use rust_isa::arm::simd::neon::vabdl as _;
//! use rust_isa::arm::simd::neon::vabs as _;
//! use rust_isa::arm::simd::neon::vaddhn as _;
//! use rust_isa::arm::simd::neon::vaddl as _;
//! use rust_isa::arm::simd::neon::vaddw as _;
//! use rust_isa::arm::simd::neon::vbic as _;
//! use rust_isa::arm::simd::neon::vbsl as _;
//! use rust_isa::arm::simd::neon::vceq as _;
//! use rust_isa::arm::simd::neon::vcge as _;
//! use rust_isa::arm::simd::neon::vcgt as _;
//! use rust_isa::arm::simd::neon::vcle as _;
//! use rust_isa::arm::simd::neon::vcls as _;
//! use rust_isa::arm::simd::neon::vclt as _;
//! use rust_isa::arm::simd::neon::vclz as _;
//! use rust_isa::arm::simd::neon::vcnt as _;
//! use rust_isa::arm::simd::neon::vcvt as _;
//! use rust_isa::arm::simd::neon::vext as _;
//! use rust_isa::arm::simd::neon::vfma as _;
//! use rust_isa::arm::simd::neon::vfms as _;
//! use rust_isa::arm::simd::neon::vfnma as _;
//! use rust_isa::arm::simd::neon::vfnms as _;
//! use rust_isa::arm::simd::neon::vhadd as _;
//! use rust_isa::arm::simd::neon::vhsub as _;
//! use rust_isa::arm::simd::neon::vld1 as _;
//! use rust_isa::arm::simd::neon::vld2 as _;
//! use rust_isa::arm::simd::neon::vld3 as _;
//! use rust_isa::arm::simd::neon::vld4 as _;
//! use rust_isa::arm::simd::neon::vmax as _;
//! use rust_isa::arm::simd::neon::vmin as _;
//! use rust_isa::arm::simd::neon::vmla as _;
//! use rust_isa::arm::simd::neon::vmlal as _;
//! use rust_isa::arm::simd::neon::vmls as _;
//! use rust_isa::arm::simd::neon::vmlsl as _;
//! use rust_isa::arm::simd::neon::vmov as _;
//! use rust_isa::arm::simd::neon::vmovn as _;
//! use rust_isa::arm::simd::neon::vmull as _;
//! use rust_isa::arm::simd::neon::vmvn as _;
//! use rust_isa::arm::simd::neon::vneg as _;
//! use rust_isa::arm::simd::neon::vorn as _;
//! use rust_isa::arm::simd::neon::vpadal as _;
//! use rust_isa::arm::simd::neon::vpadd as _;
//! use rust_isa::arm::simd::neon::vpmax as _;
//! use rust_isa::arm::simd::neon::vpmin as _;
//! use rust_isa::arm::simd::neon::vqabs as _;
//! use rust_isa::arm::simd::neon::vqadd as _;
//! use rust_isa::arm::simd::neon::vqdmlal as _;
//! use rust_isa::arm::simd::neon::vqdmlsl as _;
//! use rust_isa::arm::simd::neon::vqdmulh as _;
//! use rust_isa::arm::simd::neon::vqdmull as _;
//! use rust_isa::arm::simd::neon::vqmovn as _;
//! use rust_isa::arm::simd::neon::vqmovun as _;
//! use rust_isa::arm::simd::neon::vqneg as _;
//! use rust_isa::arm::simd::neon::vqrdmulh as _;
//! use rust_isa::arm::simd::neon::vqrshl as _;
//! use rust_isa::arm::simd::neon::vqrshrn as _;
//! use rust_isa::arm::simd::neon::vqrshrun as _;
//! use rust_isa::arm::simd::neon::vqshl as _;
//! use rust_isa::arm::simd::neon::vqshlu as _;
//! use rust_isa::arm::simd::neon::vqshrn as _;
//! use rust_isa::arm::simd::neon::vqshrun as _;
//! use rust_isa::arm::simd::neon::vqsub as _;
//! use rust_isa::arm::simd::neon::vraddhn as _;
//! use rust_isa::arm::simd::neon::vrecpe as _;
//! use rust_isa::arm::simd::neon::vrecps as _;
//! use rust_isa::arm::simd::neon::vrev16 as _;
//! use rust_isa::arm::simd::neon::vrev32 as _;
//! use rust_isa::arm::simd::neon::vrev64 as _;
//! use rust_isa::arm::simd::neon::vrshl as _;
//! use rust_isa::arm::simd::neon::vrshr as _;
//! use rust_isa::arm::simd::neon::vrshrn as _;
//! use rust_isa::arm::simd::neon::vrsqrte as _;
//! use rust_isa::arm::simd::neon::vrsqrts as _;
//! use rust_isa::arm::simd::neon::vrsra as _;
//! use rust_isa::arm::simd::neon::vrsubhn as _;
//! use rust_isa::arm::simd::neon::vshl as _;
//! use rust_isa::arm::simd::neon::vshll as _;
//! use rust_isa::arm::simd::neon::vshr as _;
//! use rust_isa::arm::simd::neon::vshrn as _;
//! use rust_isa::arm::simd::neon::vsli as _;
//! use rust_isa::arm::simd::neon::vsra as _;
//! use rust_isa::arm::simd::neon::vsri as _;
//! use rust_isa::arm::simd::neon::vsubhn as _;
//! use rust_isa::arm::simd::neon::vsubl as _;
//! use rust_isa::arm::simd::neon::vsubw as _;
//! use rust_isa::arm::simd::neon::vswp as _;
//! use rust_isa::arm::simd::neon::vtbl as _;
//! use rust_isa::arm::simd::neon::vtbx as _;
//! use rust_isa::arm::simd::neon::vtrn as _;
//! use rust_isa::arm::simd::neon::vtst as _;
//! use rust_isa::arm::simd::neon::vuzp as _;
//! use rust_isa::arm::simd::neon::vzip as _;
//! ```
//! <!-- doctest-inventory:end -->

mod arith;
mod bitwise;
mod compare;
mod float;
mod lane;
mod memory;
mod structure;
mod v128;
mod widen;

pub use arith::*;
pub use bitwise::*;
pub use compare::*;
pub use float::*;
pub use lane::*;
pub use memory::*;
pub use structure::*;
pub use v128::*;
pub use widen::*;

unsafe extern "C" {
    fn rust_isa_arm_neon_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadd.i32`
///
/// Add the four 32-bit integer lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::arm::simd::neon::{add, V128};
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
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.i32`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with NEON enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
