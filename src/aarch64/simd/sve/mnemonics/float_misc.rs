//! SVE miscellaneous floating-point instructions.

/// `fcpy` on `f32` lanes.
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
pub unsafe fn fcpy_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    out.copy_from_slice(a);
}

/// `fcmuo` on `f32` lanes.
///
/// A true lane is all bits one when either input is NaN.
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
pub unsafe fn fcmuo_f32(a: &[f32], b: &[f32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        out[lane] = if a[lane].is_nan() || b[lane].is_nan() {
            u32::MAX
        } else {
            0
        };
    }
}

/// `flogb` on `f32` lanes.
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
pub unsafe fn flogb_f32(a: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        let value = a[lane];
        let bits = value.abs().to_bits();
        let exponent = ((bits >> 23) & 0xff) as i32;
        let mantissa = bits & 0x7f_ffff;
        out[lane] = if exponent == 0xff {
            if mantissa == 0 {
                f32::INFINITY
            } else {
                f32::NAN
            }
        } else if exponent == 0 {
            if mantissa == 0 {
                f32::NEG_INFINITY
            } else {
                (31 - mantissa.leading_zeros() as i32 - 149) as f32
            }
        } else {
            (exponent - 127) as f32
        };
    }
}
