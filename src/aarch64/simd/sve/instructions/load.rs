//! SVE load instructions.

unsafe extern "C" {
    fn rust_isa_sve_ld1_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ld1` on `u8` slices.
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
pub unsafe fn ld1_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ld1` on `u16` slices.
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
pub unsafe fn ld1_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ld1` on `u32` slices.
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
pub unsafe fn ld1_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ld1` on `u64` slices.
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
pub unsafe fn ld1_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1_f32(a: *const f32, out: *mut f32, len: usize);
}
/// `ld1` on `f32` slices.
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
pub unsafe fn ld1_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1_f64(a: *const f64, out: *mut f64, len: usize);
}
/// `ld1` on `f64` slices.
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
pub unsafe fn ld1_f64(a: &[f64], out: &mut [f64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1_f64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1b_u8(a: *const u8, out: *mut u32, len: usize);
}
/// `ld1b` from `u8` to `u32` slices.
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
pub unsafe fn ld1b_u8(a: &[u8], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1sb_i8(a: *const i8, out: *mut i32, len: usize);
}
/// `ld1sb` from `i8` to `i32` slices.
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
pub unsafe fn ld1sb_i8(a: &[i8], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1sb_i8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1h_u16(a: *const u16, out: *mut u32, len: usize);
}
/// `ld1h` from `u16` to `u32` slices.
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
pub unsafe fn ld1h_u16(a: &[u16], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1sh_i16(a: *const i16, out: *mut i32, len: usize);
}
/// `ld1sh` from `i16` to `i32` slices.
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
pub unsafe fn ld1sh_i16(a: &[i16], out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1sh_i16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1w_u32(a: *const u32, out: *mut u64, len: usize);
}
/// `ld1w` from `u32` to `u64` slices.
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
pub unsafe fn ld1w_u32(a: &[u32], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1sw_i32(a: *const i32, out: *mut i64, len: usize);
}
/// `ld1sw` from `i32` to `i64` slices.
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
pub unsafe fn ld1sw_i32(a: &[i32], out: &mut [i64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1sw_i32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ld1d` on `u64` slices.
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
pub unsafe fn ld1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ld1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldnt1b_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ldnt1b` on `u8` slices.
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
pub unsafe fn ldnt1b_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnt1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldnt1h_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ldnt1h` on `u16` slices.
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
pub unsafe fn ldnt1h_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnt1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldnt1w_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ldnt1w` on `u32` slices.
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
pub unsafe fn ldnt1w_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnt1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldnt1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ldnt1d` on `u64` slices.
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
pub unsafe fn ldnt1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnt1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldff1b_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ldff1b` on `u8` slices.
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
pub unsafe fn ldff1b_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldff1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldff1h_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ldff1h` on `u16` slices.
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
pub unsafe fn ldff1h_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldff1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldff1w_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ldff1w` on `u32` slices.
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
pub unsafe fn ldff1w_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldff1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldff1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ldff1d` on `u64` slices.
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
pub unsafe fn ldff1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldff1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldnf1b_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ldnf1b` on `u8` slices.
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
pub unsafe fn ldnf1b_u8(a: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnf1b_u8(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldnf1h_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ldnf1h` on `u16` slices.
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
pub unsafe fn ldnf1h_u16(a: &[u16], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnf1h_u16(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldnf1w_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ldnf1w` on `u32` slices.
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
pub unsafe fn ldnf1w_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnf1w_u32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ldnf1d_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ldnf1d` on `u64` slices.
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
pub unsafe fn ldnf1d_u64(a: &[u64], out: &mut [u64]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_ldnf1d_u64(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rqb_u8(a: *const u8, out: *mut u8, len: usize);
}
/// `ld1rqb` replicate from `u8` slices.
///
/// Read the 16-byte quadword at the start of `a` and replicate it across `out`.
///
/// # Panics
///
/// Panics when `a` has fewer than 16 bytes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rqb_u8(a: &[u8], out: &mut [u8]) {
    assert!(a.len() >= 16, "SVE replicate load needs a full quadword");
    unsafe {
        rust_isa_sve_ld1rqb_u8(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rqh_u16(a: *const u16, out: *mut u16, len: usize);
}
/// `ld1rqh` replicate from `u16` slices.
///
/// Read the 16-byte quadword at the start of `a` and replicate it across `out`.
///
/// # Panics
///
/// Panics when `a` has fewer than 8 elements.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rqh_u16(a: &[u16], out: &mut [u16]) {
    assert!(a.len() >= 8, "SVE replicate load needs a full quadword");
    unsafe {
        rust_isa_sve_ld1rqh_u16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rqw_u32(a: *const u32, out: *mut u32, len: usize);
}
/// `ld1rqw` replicate from `u32` slices.
///
/// Read the 16-byte quadword at the start of `a` and replicate it across `out`.
///
/// # Panics
///
/// Panics when `a` has fewer than 4 elements.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rqw_u32(a: &[u32], out: &mut [u32]) {
    assert!(a.len() >= 4, "SVE replicate load needs a full quadword");
    unsafe {
        rust_isa_sve_ld1rqw_u32(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rqd_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ld1rqd` replicate from `u64` slices.
///
/// Read the 16-byte quadword at the start of `a` and replicate it across `out`.
///
/// # Panics
///
/// Panics when `a` has fewer than 2 elements.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rqd_u64(a: &[u64], out: &mut [u64]) {
    assert!(a.len() >= 2, "SVE replicate load needs a full quadword");
    unsafe {
        rust_isa_sve_ld1rqd_u64(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rb_u8(a: *const u8, out: *mut u32, len: usize);
}
/// `ld1rb` broadcast from `u8` to `u32` slices.
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
pub unsafe fn ld1rb_u8(a: &[u8], out: &mut [u32]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rb_u8(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rsb_i8(a: *const i8, out: *mut i32, len: usize);
}
/// `ld1rsb` broadcast from `i8` to `i32` slices.
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
pub unsafe fn ld1rsb_i8(a: &[i8], out: &mut [i32]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rsb_i8(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rh_u16(a: *const u16, out: *mut u32, len: usize);
}
/// `ld1rh` broadcast from `u16` to `u32` slices.
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
pub unsafe fn ld1rh_u16(a: &[u16], out: &mut [u32]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rh_u16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rsh_i16(a: *const i16, out: *mut i32, len: usize);
}
/// `ld1rsh` broadcast from `i16` to `i32` slices.
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
pub unsafe fn ld1rsh_i16(a: &[i16], out: &mut [i32]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rsh_i16(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rw_u32(a: *const u32, out: *mut u64, len: usize);
}
/// `ld1rw` broadcast from `u32` to `u64` slices.
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
pub unsafe fn ld1rw_u32(a: &[u32], out: &mut [u64]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rw_u32(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rsw_i32(a: *const i32, out: *mut i64, len: usize);
}
/// `ld1rsw` broadcast from `i32` to `i64` slices.
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
pub unsafe fn ld1rsw_i32(a: &[i32], out: &mut [i64]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rsw_i32(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_ld1rd_u64(a: *const u64, out: *mut u64, len: usize);
}
/// `ld1rd` broadcast from `u64` to `u64` slices.
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
pub unsafe fn ld1rd_u64(a: &[u64], out: &mut [u64]) {
    assert!(!a.is_empty(), "SVE broadcast load needs an input element");
    // SVE broadcast: input is a scalar, output length is independent
    unsafe {
        rust_isa_sve_ld1rd_u64(a.as_ptr(), out.as_mut_ptr(), out.len());
    }
}
