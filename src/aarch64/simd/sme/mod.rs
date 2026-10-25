//! AArch64 SME streaming state.
//!
//! SME is not part of the AArch64 baseline. rustc does not allow
//! `#[target_feature(enable = "sme")]` on stable, so the caller must compile the
//! whole crate with SME enabled. The CPU must support SME.
//!
//! [`SmeGuard`] enters streaming mode and leaves it in its `Drop` implementation.
//! The guard is neither `Send` nor `Sync` and must not be nested. The slice
//! functions operate on `f32` values and take a guard so that the streaming
//! state is established before the C loop runs. They are `unsafe` and require
//! SME.
//!
//! ZA tile operations are intentionally omitted. They need an additional opaque
//! tile type and a separate lifetime model.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.

//! # Examples
//!
//! ```no_run
//! use rust_isa::aarch64::simd::sme::{add_f32, SmeGuard};
//!
//! let guard = unsafe { SmeGuard::enter() };
//! let a = [1.0f32, 2.0];
//! let b = [10.0f32, 20.0];
//! let mut out = [0.0f32; 2];
//! unsafe { add_f32(&guard, &a, &b, &mut out) };
//! assert_eq!(out, [11.0, 22.0]);
//! ```
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::aarch64::simd::sme::*;
//! use rust_isa::aarch64::simd::neon::V128;
//! use rust_isa::aarch64::simd::sme::SmeGuard;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let guard = unsafe { SmeGuard::enter() };
//!
//! let _ = unsafe { add_f32(&guard, &[0.0f32; 4], &[0.0f32; 4], &mut[0.0f32; 4]) };
//! let _ = unsafe { mul_f32(&guard, &[0.0f32; 4], &[0.0f32; 4], &mut[0.0f32; 4]) };
//! let _ = unsafe { SmeGuard::enter() };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::aarch64::simd::sme::add_f32 as _;
//! use rust_isa::aarch64::simd::sme::mul_f32 as _;
//! use rust_isa::aarch64::simd::sme::SmeGuard;
//! let _ = SmeGuard::enter;
//! ```
//! <!-- doctest-inventory:end -->

use core::marker::PhantomData;

unsafe extern "C" {
    fn rust_isa_sme_start();
    fn rust_isa_sme_stop();
    fn rust_isa_sme_add_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sme_mul_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
}

/// An active SME streaming state.
pub struct SmeGuard {
    _not_send_or_sync: PhantomData<*const ()>,
}

impl SmeGuard {
    /// Enter streaming mode.
    ///
    /// # Safety
    ///
    /// The CPU must support SME, and the caller must not create a nested guard.
    pub unsafe fn enter() -> Self {
        unsafe {
            rust_isa_sme_start();
        }
        Self {
            _not_send_or_sync: PhantomData,
        }
    }
}

impl Drop for SmeGuard {
    fn drop(&mut self) {
        unsafe {
            rust_isa_sme_stop();
        }
    }
}

/// Add two `f32` slices in streaming mode.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must hold an [`SmeGuard`] and the CPU must support SME.
pub unsafe fn add_f32(_guard: &SmeGuard, a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SME slices must have the same length");
    assert_eq!(a.len(), out.len(), "SME slices must have the same length");
    unsafe {
        rust_isa_sme_add_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `f32` slices in streaming mode.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must hold an [`SmeGuard`] and the CPU must support SME.
pub unsafe fn mul_f32(_guard: &SmeGuard, a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SME slices must have the same length");
    assert_eq!(a.len(), out.len(), "SME slices must have the same length");
    unsafe {
        rust_isa_sme_mul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
