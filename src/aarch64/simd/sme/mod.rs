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
