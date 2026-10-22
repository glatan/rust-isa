//! SVE structure load and store instructions.

unsafe extern "C" {
    fn rust_isa_sve_ld2b_u8(a: *const u8, out0: *mut u8, out1: *mut u8, len: usize);
}
/// `ld2b` deinterleave from `u8` slices.
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
pub unsafe fn ld2b_u8(a: &[u8], out0: &mut [u8], out1: &mut [u8]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2b_u8(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld2h_u16(a: *const u16, out0: *mut u16, out1: *mut u16, len: usize);
}
/// `ld2h` deinterleave from `u16` slices.
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
pub unsafe fn ld2h_u16(a: &[u16], out0: &mut [u16], out1: &mut [u16]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2h_u16(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld2w_u32(a: *const u32, out0: *mut u32, out1: *mut u32, len: usize);
}
/// `ld2w` deinterleave from `u32` slices.
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
pub unsafe fn ld2w_u32(a: &[u32], out0: &mut [u32], out1: &mut [u32]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2w_u32(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld2d_u64(a: *const u64, out0: *mut u64, out1: *mut u64, len: usize);
}
/// `ld2d` deinterleave from `u64` slices.
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
pub unsafe fn ld2d_u64(a: &[u64], out0: &mut [u64], out1: &mut [u64]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2d_u64(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld3b_u8(a: *const u8, out0: *mut u8, out1: *mut u8, out2: *mut u8, len: usize);
}
/// `ld3b` deinterleave from `u8` slices.
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
pub unsafe fn ld3b_u8(a: &[u8], out0: &mut [u8], out1: &mut [u8], out2: &mut [u8]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3b_u8(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld3h_u16(
        a: *const u16,
        out0: *mut u16,
        out1: *mut u16,
        out2: *mut u16,
        len: usize,
    );
}
/// `ld3h` deinterleave from `u16` slices.
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
pub unsafe fn ld3h_u16(a: &[u16], out0: &mut [u16], out1: &mut [u16], out2: &mut [u16]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3h_u16(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld3w_u32(
        a: *const u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        len: usize,
    );
}
/// `ld3w` deinterleave from `u32` slices.
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
pub unsafe fn ld3w_u32(a: &[u32], out0: &mut [u32], out1: &mut [u32], out2: &mut [u32]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3w_u32(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld3d_u64(
        a: *const u64,
        out0: *mut u64,
        out1: *mut u64,
        out2: *mut u64,
        len: usize,
    );
}
/// `ld3d` deinterleave from `u64` slices.
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
pub unsafe fn ld3d_u64(a: &[u64], out0: &mut [u64], out1: &mut [u64], out2: &mut [u64]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3d_u64(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld4b_u8(
        a: *const u8,
        out0: *mut u8,
        out1: *mut u8,
        out2: *mut u8,
        out3: *mut u8,
        len: usize,
    );
}
/// `ld4b` deinterleave from `u8` slices.
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
pub unsafe fn ld4b_u8(
    a: &[u8],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4b_u8(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld4h_u16(
        a: *const u16,
        out0: *mut u16,
        out1: *mut u16,
        out2: *mut u16,
        out3: *mut u16,
        len: usize,
    );
}
/// `ld4h` deinterleave from `u16` slices.
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
pub unsafe fn ld4h_u16(
    a: &[u16],
    out0: &mut [u16],
    out1: &mut [u16],
    out2: &mut [u16],
    out3: &mut [u16],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4h_u16(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld4w_u32(
        a: *const u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        out3: *mut u32,
        len: usize,
    );
}
/// `ld4w` deinterleave from `u32` slices.
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
pub unsafe fn ld4w_u32(
    a: &[u32],
    out0: &mut [u32],
    out1: &mut [u32],
    out2: &mut [u32],
    out3: &mut [u32],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4w_u32(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld4d_u64(
        a: *const u64,
        out0: *mut u64,
        out1: *mut u64,
        out2: *mut u64,
        out3: *mut u64,
        len: usize,
    );
}
/// `ld4d` deinterleave from `u64` slices.
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
pub unsafe fn ld4d_u64(
    a: &[u64],
    out0: &mut [u64],
    out1: &mut [u64],
    out2: &mut [u64],
    out3: &mut [u64],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4d_u64(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st2b_u8(a0: *const u8, a1: *const u8, out: *mut u8, len: usize);
}
/// `st2b` interleave to `u8` slices.
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
pub unsafe fn st2b_u8(a0: &[u8], a1: &[u8], out: &mut [u8]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2b_u8(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st2h_u16(a0: *const u16, a1: *const u16, out: *mut u16, len: usize);
}
/// `st2h` interleave to `u16` slices.
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
pub unsafe fn st2h_u16(a0: &[u16], a1: &[u16], out: &mut [u16]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2h_u16(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st2w_u32(a0: *const u32, a1: *const u32, out: *mut u32, len: usize);
}
/// `st2w` interleave to `u32` slices.
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
pub unsafe fn st2w_u32(a0: &[u32], a1: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2w_u32(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st2d_u64(a0: *const u64, a1: *const u64, out: *mut u64, len: usize);
}
/// `st2d` interleave to `u64` slices.
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
pub unsafe fn st2d_u64(a0: &[u64], a1: &[u64], out: &mut [u64]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2d_u64(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st3b_u8(a0: *const u8, a1: *const u8, a2: *const u8, out: *mut u8, len: usize);
}
/// `st3b` interleave to `u8` slices.
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
pub unsafe fn st3b_u8(a0: &[u8], a1: &[u8], a2: &[u8], out: &mut [u8]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3b_u8(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st3h_u16(
        a0: *const u16,
        a1: *const u16,
        a2: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `st3h` interleave to `u16` slices.
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
pub unsafe fn st3h_u16(a0: &[u16], a1: &[u16], a2: &[u16], out: &mut [u16]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3h_u16(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st3w_u32(
        a0: *const u32,
        a1: *const u32,
        a2: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `st3w` interleave to `u32` slices.
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
pub unsafe fn st3w_u32(a0: &[u32], a1: &[u32], a2: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3w_u32(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st3d_u64(
        a0: *const u64,
        a1: *const u64,
        a2: *const u64,
        out: *mut u64,
        len: usize,
    );
}
/// `st3d` interleave to `u64` slices.
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
pub unsafe fn st3d_u64(a0: &[u64], a1: &[u64], a2: &[u64], out: &mut [u64]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3d_u64(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st4b_u8(
        a0: *const u8,
        a1: *const u8,
        a2: *const u8,
        a3: *const u8,
        out: *mut u8,
        len: usize,
    );
}
/// `st4b` interleave to `u8` slices.
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
pub unsafe fn st4b_u8(a0: &[u8], a1: &[u8], a2: &[u8], a3: &[u8], out: &mut [u8]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4b_u8(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st4h_u16(
        a0: *const u16,
        a1: *const u16,
        a2: *const u16,
        a3: *const u16,
        out: *mut u16,
        len: usize,
    );
}
/// `st4h` interleave to `u16` slices.
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
pub unsafe fn st4h_u16(a0: &[u16], a1: &[u16], a2: &[u16], a3: &[u16], out: &mut [u16]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4h_u16(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st4w_u32(
        a0: *const u32,
        a1: *const u32,
        a2: *const u32,
        a3: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `st4w` interleave to `u32` slices.
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
pub unsafe fn st4w_u32(a0: &[u32], a1: &[u32], a2: &[u32], a3: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4w_u32(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st4d_u64(
        a0: *const u64,
        a1: *const u64,
        a2: *const u64,
        a3: *const u64,
        out: *mut u64,
        len: usize,
    );
}
/// `st4d` interleave to `u64` slices.
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
pub unsafe fn st4d_u64(a0: &[u64], a1: &[u64], a2: &[u64], a3: &[u64], out: &mut [u64]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4d_u64(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld2_u32(a: *const u32, out0: *mut u32, out1: *mut u32, len: usize);
}
/// `ld2` deinterleave from `u32` slices.
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
pub unsafe fn ld2_u32(a: &[u32], out0: &mut [u32], out1: &mut [u32]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 2,
        "SVE input must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld2_u32(a.as_ptr(), out0.as_mut_ptr(), out1.as_mut_ptr(), out0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld3_u32(
        a: *const u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        len: usize,
    );
}
/// `ld3` deinterleave from `u32` slices.
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
pub unsafe fn ld3_u32(a: &[u32], out0: &mut [u32], out1: &mut [u32], out2: &mut [u32]) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 3,
        "SVE input must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld3_u32(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld4_u32(
        a: *const u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        out3: *mut u32,
        len: usize,
    );
}
/// `ld4` deinterleave from `u32` slices.
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
pub unsafe fn ld4_u32(
    a: &[u32],
    out0: &mut [u32],
    out1: &mut [u32],
    out2: &mut [u32],
    out3: &mut [u32],
) {
    assert_eq!(
        out0.len(),
        out1.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out2.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        out0.len(),
        out3.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        a.len(),
        out0.len() * 4,
        "SVE input must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_ld4_u32(
            a.as_ptr(),
            out0.as_mut_ptr(),
            out1.as_mut_ptr(),
            out2.as_mut_ptr(),
            out3.as_mut_ptr(),
            out0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st2_u32(a0: *const u32, a1: *const u32, out: *mut u32, len: usize);
}
/// `st2` interleave to `u32` slices.
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
pub unsafe fn st2_u32(a0: &[u32], a1: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 2,
        "SVE output must contain 2 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st2_u32(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr(), a0.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st3_u32(
        a0: *const u32,
        a1: *const u32,
        a2: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `st3` interleave to `u32` slices.
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
pub unsafe fn st3_u32(a0: &[u32], a1: &[u32], a2: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 3,
        "SVE output must contain 3 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st3_u32(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_st4_u32(
        a0: *const u32,
        a1: *const u32,
        a2: *const u32,
        a3: *const u32,
        out: *mut u32,
        len: usize,
    );
}
/// `st4` interleave to `u32` slices.
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
pub unsafe fn st4_u32(a0: &[u32], a1: &[u32], a2: &[u32], a3: &[u32], out: &mut [u32]) {
    assert_eq!(a0.len(), a1.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a2.len(), "SVE slices must have the same length");
    assert_eq!(a0.len(), a3.len(), "SVE slices must have the same length");
    assert_eq!(
        out.len(),
        a0.len() * 4,
        "SVE output must contain 4 interleaved slices"
    );
    unsafe {
        rust_isa_sve_st4_u32(
            a0.as_ptr(),
            a1.as_ptr(),
            a2.as_ptr(),
            a3.as_ptr(),
            out.as_mut_ptr(),
            a0.len(),
        );
    }
}
