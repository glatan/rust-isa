//! SVE floating-point fused multiply-add instructions.

unsafe extern "C" {
    fn rust_isa_sve_mad_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `mad` on `f32` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mad_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mad_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_msb_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `msb` on `f32` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn msb_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_msb_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmad_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmad` on `f32` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmad_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmad_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmsb_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmsb` on `f32` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmsb_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmsb_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fnmad_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fnmad` on `f32` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmad_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmad_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fnmsb_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fnmsb` on `f32` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmsb_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmsb_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fnmla_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fnmla` on `f32` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmla_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmla_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fnmls_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fnmls` on `f32` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmls_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmls_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_mad_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `mad` on `f64` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn mad_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_mad_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_msb_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `msb` on `f64` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn msb_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_msb_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmad_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmad` on `f64` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmad_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmad_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmsb_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmsb` on `f64` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmsb_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmsb_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fnmad_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fnmad` on `f64` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmad_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmad_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fnmsb_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fnmsb` on `f64` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmsb_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmsb_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fnmla_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fnmla` on `f64` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmla_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmla_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fnmls_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fnmls` on `f64` slices with an accumulator.
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fnmls_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fnmls_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmla_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmla` on `f32` slices (alias of `fma`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmla_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmla_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmls_f32(
        a: *const f32,
        b: *const f32,
        acc: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fmls` on `f32` slices (alias of `mls`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmls_f32(a: &[f32], b: &[f32], acc: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmls_f32(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmla_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmla` on `f64` slices (alias of `fma`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmla_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmla_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmls_f64(
        a: *const f64,
        b: *const f64,
        acc: *const f64,
        out: *mut f64,
        len: usize,
    );
}
/// `fmls` on `f64` slices (alias of `mls`).
///
/// # Panics
///
/// Panics when the input slices do not satisfy the operation's length requirement.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmls_f64(a: &[f64], b: &[f64], acc: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), acc.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fmls_f64(
            a.as_ptr(),
            b.as_ptr(),
            acc.as_ptr(),
            out.as_mut_ptr(),
            a.len(),
        );
    }
}
