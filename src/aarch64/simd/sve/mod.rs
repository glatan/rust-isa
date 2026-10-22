//! AArch64 SVE slice API.
//!
//! SVE is not part of the AArch64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable SVE with `#[target_feature(enable = "sve")]` or a global
//! target feature.
//!
//! SVE vectors have a runtime vector length, so this module exposes slice
//! operations rather than per-register values. The slices use the element type
//! in each function name. The C loop chooses the vector length at run time.
//! Most functions that take multiple slices require them to have the same
//! length; a different length panics. Broadcast and replicate loads are the
//! exception: they read a fixed-size input and write an output slice whose
//! length is independent. The functions do not allocate.
//!
//! The per-register API is intentionally omitted because a fixed-length
//! [`V128`](crate::aarch64::simd::neon::V128) cannot represent an SVE register.
//! Each instruction is exposed as a slice operation instead.
//!
//! Instruction behavior follows the [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/).
//!
//! The build compiles the bundled C with clang.

unsafe extern "C" {
    fn rust_isa_sve_add_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_mul_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_add_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_mul_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_add_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_mul_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_sub_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_sub_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_sub_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_div_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_div_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_div_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_min_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_max_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_min_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_max_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_min_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_max_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_abs_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_neg_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_abs_i32(a: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_neg_i32(a: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_abs_f64(a: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_neg_f64(a: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_sqrt_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_sqrt_f64(a: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_and_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_or_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_xor_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_fma_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
    fn rust_isa_sve_fma_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}

/// Add two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Add two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Add two `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn add_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_add_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mul_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mul_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Subtract two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Subtract two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Subtract two `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sub_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sub_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Divide two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn div_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_div_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Divide two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn div_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_div_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Divide two `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn div_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_div_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Minimum of two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn min_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_min_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Maximum of two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn max_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_max_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Minimum of two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn min_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_min_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Maximum of two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn max_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_max_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Minimum of two `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn min_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_min_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Maximum of two `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn max_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_max_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// `abs` on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abs_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abs_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// `neg` on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn neg_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_neg_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// `abs` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abs_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abs_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// `neg` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn neg_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_neg_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// `abs` on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn abs_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_abs_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// `neg` on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn neg_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_neg_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Square root of an `f32` slice.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqrt_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrt_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Square root of an `f64` slice.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn sqrt_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_sqrt_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Bitwise AND of two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn and_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_and_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Bitwise OR of two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn or_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_or_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Bitwise XOR of two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn xor_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_xor_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Fused multiply-add on `f32` slices: `a * b + acc`.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fma_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fma_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
/// Fused multiply-add on `f64` slices: `a * b + acc`.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fma_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fma_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

/// Reciprocal estimate on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recip_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recip_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Reciprocal estimate on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn recip_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_recip_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_recip_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_recip_f64(a: *const f64, out: *mut f64, len: usize);
}

unsafe extern "C" {
    fn rust_isa_sve_rsqrt_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_rsqrt_f64(a: *const f64, out: *mut f64, len: usize);
}

/// Reciprocal square-root estimate on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrt_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrt_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Reciprocal square-root estimate on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rsqrt_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rsqrt_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_floor_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_ceil_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_floor_f64(a: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_ceil_f64(a: *const f64, out: *mut f64, len: usize);
}
/// Floor of `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn floor_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_floor_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Ceiling of `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ceil_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ceil_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Floor of `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn floor_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_floor_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Ceiling of `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ceil_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ceil_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_trunc_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_rint_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_trunc_f64(a: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_rint_f64(a: *const f64, out: *mut f64, len: usize);
}
/// Truncate `f32` slices toward zero.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn trunc_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_trunc_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Round `f32` slices to nearest integer.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rint_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rint_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Truncate `f64` slices toward zero.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn trunc_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_trunc_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Round `f64` slices to nearest integer.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rint_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rint_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_minnm_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_maxnm_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_minnm_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_maxnm_f64(a: *const f64, b: *const f64, out: *mut f64, len: usize);
}
/// IEEE minimumNumber on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn minnm_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_minnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// IEEE maximumNumber on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn maxnm_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_maxnm_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// IEEE minimumNumber on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn minnm_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_minnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// IEEE maximumNumber on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn maxnm_f64(a: &[f64], b: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_maxnm_f64(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_f32(a: *const f32, len: usize) -> f32;
    fn rust_isa_sve_addv_i32(a: *const i32, len: usize) -> i32;
    fn rust_isa_sve_addv_f64(a: *const f64, len: usize) -> f64;
}
/// Sum an `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_addv_f32(a.as_ptr(), a.len()) }
}
/// Sum an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_addv_i32(a.as_ptr(), a.len()) }
}
/// Sum an `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_addv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_f32(a: *const f32, len: usize) -> f32;
    fn rust_isa_sve_maxv_f32(a: *const f32, len: usize) -> f32;
    fn rust_isa_sve_minv_i32(a: *const i32, len: usize) -> i32;
    fn rust_isa_sve_maxv_i32(a: *const i32, len: usize) -> i32;
    fn rust_isa_sve_minv_f64(a: *const f64, len: usize) -> f64;
    fn rust_isa_sve_maxv_f64(a: *const f64, len: usize) -> f64;
}
/// Minimum of an `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_minv_f32(a.as_ptr(), a.len()) }
}
/// Maximum of an `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_maxv_f32(a.as_ptr(), a.len()) }
}
/// Minimum of an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_minv_i32(a.as_ptr(), a.len()) }
}
/// Maximum of an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_maxv_i32(a.as_ptr(), a.len()) }
}
/// Minimum of an `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_minv_f64(a.as_ptr(), a.len()) }
}
/// Maximum of an `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_maxv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_mls_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
    fn rust_isa_sve_mls_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// Fused multiply-subtract on `f32` slices: `acc - a * b`.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mls_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mls_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
/// Fused multiply-subtract on `f64` slices: `acc - a * b`.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mls_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mls_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_clz_i32(a: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_cnt_i32(a: *const i32, out: *mut i32, len: usize);
    fn rust_isa_sve_not_i32(a: *const i32, out: *mut i32, len: usize);
}
/// Count leading zeros of each `i32` lane.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn clz_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_clz_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Count set bits of each `i32` lane.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cnt_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cnt_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Bitwise NOT of each `i32` lane.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn not_i32(a: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_not_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_lsl_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
    fn rust_isa_sve_lsr_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
    fn rust_isa_sve_asr_i32(a: *const i32, shift: i32, out: *mut i32, len: usize);
}
/// Logical left shift of `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsl_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsl_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
/// Logical right shift of `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn lsr_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_lsr_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}
/// Arithmetic right shift of `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn asr_i32(a: &[i32], shift: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_asr_i32(a.as_ptr(), shift, out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_mla_i32(
        a: *const i32,
        b: *const i32,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
    fn rust_isa_sve_mls_i32(
        a: *const i32,
        b: *const i32,
        acc: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `acc + a * b` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mla_i32(a: &[i32], b: &[i32], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mla_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
/// `acc - a * b` on `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mls_i32(a: &[i32], b: &[i32], acc: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mls_i32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_rinta_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_rintx_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_rinti_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_rinta_f64(a: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_rintx_f64(a: *const f64, out: *mut f64, len: usize);
    fn rust_isa_sve_rinti_f64(a: *const f64, out: *mut f64, len: usize);
}
/// Round `f32` slices to nearest, ties away from zero.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rinta_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rinta_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Round `f32` slices to nearest, current mode.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintx_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintx_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Round `f32` slices using the current rounding mode.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rinti_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rinti_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Round `f64` slices to nearest, ties away from zero.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rinta_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rinta_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Round `f64` slices to nearest, current mode.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rintx_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rintx_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Round `f64` slices using the current rounding mode.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rinti_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_rinti_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_frecpx_f32(a: *const f32, out: *mut f32, len: usize);
    fn rust_isa_sve_frecpx_f64(a: *const f64, out: *mut f64, len: usize);
}
/// Reciprocal exponent on `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecpx_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecpx_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}
/// Reciprocal exponent on `f64` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn frecpx_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_frecpx_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

mod instructions;
pub use instructions::*;
