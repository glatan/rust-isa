//! SVE complex and long multiply instructions.

unsafe extern "C" {
    fn rust_isa_sve_fcmne_f32(a: *const f32, b: *const f32, out: *mut u32, len: usize);
}
/// `fcmne` vector form on `f32` slices.
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
pub unsafe fn fcmne_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmne_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fcmne_zero_f32(a: *const f32, out: *mut u32, len: usize);
}
/// `fcmne` zero form on `f32` slices.
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
pub unsafe fn fcmne_zero_f32(a: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmne_zero_f32(a.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmlalb_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `fmlalb` vector form on `f32` accumulators and `f16` inputs.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmlalb_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE f16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE f16 input is too short");
    unsafe {
        rust_isa_sve_fmlalb_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmlalt_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `fmlalt` vector form on `f32` accumulators and `f16` inputs.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmlalt_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE f16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE f16 input is too short");
    unsafe {
        rust_isa_sve_fmlalt_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmlslb_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `fmlslb` vector form on `f32` accumulators and `f16` inputs.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmlslb_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE f16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE f16 input is too short");
    unsafe {
        rust_isa_sve_fmlslb_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}

unsafe extern "C" {
    fn rust_isa_sve_fmlslt_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `fmlslt` vector form on `f32` accumulators and `f16` inputs.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmlslt_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE f16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE f16 input is too short");
    unsafe {
        rust_isa_sve_fmlslt_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cdot_i32(
        acc: *const i32,
        a: *const i8,
        b: *const i8,
        out: *mut i32,
        len: usize,
    );
}
/// `cdot` complex operation.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn cdot_i32(acc: &[i32], a: &[i8], b: &[i8], out: &mut [i32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 4, "SVE int8 input is too short");
    assert!(b.len() >= acc.len() * 4, "SVE int8 input is too short");
    unsafe {
        rust_isa_sve_cdot_i32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cdot_lane_i32(
        acc: *const i32,
        a: *const i8,
        b: *const i8,
        out: *mut i32,
        len: usize,
    );
}
/// `cdot` complex operation.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn cdot_lane_i32(acc: &[i32], a: &[i8], b: &[i8], out: &mut [i32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 4, "SVE int8 input is too short");
    assert!(b.len() >= acc.len() * 4, "SVE int8 input is too short");
    unsafe {
        rust_isa_sve_cdot_lane_i32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cmla_i32(
        acc: *const i32,
        a: *const i32,
        b: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `cmla` complex operation.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn cmla_i32(acc: &[i32], a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), b.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cmla_i32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_cmla_lane_i32(
        acc: *const i32,
        a: *const i32,
        b: *const i32,
        out: *mut i32,
        len: usize,
    );
}
/// `cmla` complex operation.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn cmla_lane_i32(acc: &[i32], a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), b.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_cmla_lane_i32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fcmla_lane_f32(
        acc: *const f32,
        a: *const f32,
        b: *const f32,
        out: *mut f32,
        len: usize,
    );
}
/// `fcmla` complex operation.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fcmla_lane_f32(acc: &[f32], a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), b.len(), "SVE slices must have the same length");
    unsafe {
        rust_isa_sve_fcmla_lane_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmlalb_lane_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `fmlalb` indexed form with lane 0.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fmlalb_lane_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE f16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE f16 input is too short");
    unsafe {
        rust_isa_sve_fmlalb_lane_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmlalt_lane_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `fmlalt` indexed form with lane 0.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fmlalt_lane_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE f16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE f16 input is too short");
    unsafe {
        rust_isa_sve_fmlalt_lane_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmlslb_lane_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `fmlslb` indexed form with lane 0.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fmlslb_lane_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE f16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE f16 input is too short");
    unsafe {
        rust_isa_sve_fmlslb_lane_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
unsafe extern "C" {
    fn rust_isa_sve_fmlslt_lane_f32(
        acc: *const f32,
        a: *const u16,
        b: *const u16,
        out: *mut f32,
        len: usize,
    );
}
/// `fmlslt` indexed form with lane 0.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn fmlslt_lane_f32(acc: &[f32], a: &[u16], b: &[u16], out: &mut [f32]) {
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    assert!(a.len() >= acc.len() * 2, "SVE f16 input is too short");
    assert!(b.len() >= acc.len() * 2, "SVE f16 input is too short");
    unsafe {
        rust_isa_sve_fmlslt_lane_f32(
            acc.as_ptr(),
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            acc.len(),
        );
    }
}
