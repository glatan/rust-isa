//! AVX.
//!
//! AVX is not part of the x86-64 baseline. Every operation is `unsafe` and the
//! caller must enable AVX with `#[target_feature(enable = "avx")]` or a global
//! target feature.
//!
//! The 256-bit values are [`V256`]. The scalar `*ss` and `*sd` operations take
//! and return a [`V128`]; only the low lane is defined by the instruction and
//! the upper lanes are copied from the first operand.
//!
//! Immediate forms use const-generic inline assembly with memory operands.

pub use super::avx2::V256;
pub use super::sse2::V128;

mod blend;
mod mask;
mod moves;
mod permute;
mod round;
mod shuffle;

pub use blend::*;
pub use mask::*;
pub use moves::*;
pub use permute::*;
pub use round::*;
pub use shuffle::*;

unsafe extern "C" {
    fn rust_isa_avx_vaddps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vaddpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vsubps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vsubpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vmulps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vmulpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vdivps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vdivpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vandps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vorps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vxorps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vaddss(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vaddsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vbroadcastss(value: f32, out: *mut u8);
    fn rust_isa_avx_vbroadcastsd(value: f64, out: *mut u8);
    fn rust_isa_avx_vmovmskps(a: *const u8) -> u32;
    fn rust_isa_avx_vzeroupper();
}

/// `vaddps`
///
/// Add the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vaddps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddpd`
///
/// Add the four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vaddpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubps`
///
/// Subtract the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vsubps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vsubps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubpd`
///
/// Subtract the four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vsubpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vsubpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulps`
///
/// Multiply the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmulps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmulps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulpd`
///
/// Multiply the four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmulpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vmulpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdivps`
///
/// Divide the eight 32-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vdivps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vdivps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdivpd`
///
/// Divide the four 64-bit float lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vdivpd(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vdivpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vandps`
///
/// Bitwise AND of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vandps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vandps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vorps`
///
/// Bitwise OR of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vorps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vorps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vxorps`
///
/// Bitwise XOR of the 256-bit values.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vxorps(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vxorps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddss`
///
/// Add the low 32-bit float lane. The upper lanes are copied from `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddss(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vaddss(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddsd`
///
/// Add the low 64-bit float lane. The upper lane is copied from `a`.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vaddsd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx_vaddsd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbroadcastss`
///
/// Broadcast a 32-bit float to all eight lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vbroadcastss(value: f32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vbroadcastss(value, out.as_mut_ptr());
    }
    out
}

/// `vbroadcastsd`
///
/// Broadcast a 64-bit float to all four lanes.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vbroadcastsd(value: f64) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx_vbroadcastsd(value, out.as_mut_ptr());
    }
    out
}

/// `vmovmskps`
///
/// Return the sign bits of the eight 32-bit float lanes as a mask.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vmovmskps(a: V256) -> u32 {
    unsafe { rust_isa_avx_vmovmskps(a.as_ptr()) }
}

/// `vzeroupper`
///
/// Clear the upper halves of the vector registers.
///
/// # Safety
///
/// The caller must enable AVX.
#[target_feature(enable = "avx")]
#[inline]
pub unsafe fn vzeroupper() {
    unsafe {
        rust_isa_avx_vzeroupper();
    }
}
