//! SVE floating-point conversion instructions.

#[inline(always)]
fn half_to_f32(bits: u16) -> f32 {
    let sign = ((bits >> 15) & 1) as u32;
    let exp = ((bits >> 10) & 0x1f) as u32;
    let mant = (bits & 0x3ff) as u32;
    let out = if exp == 0 {
        if mant == 0 {
            sign << 31
        } else {
            let mut e = 127 - 15 + 1;
            let mut m = mant;
            while m & 0x400 == 0 {
                m <<= 1;
                e -= 1;
            }
            m &= 0x3ff;
            (sign << 31) | ((e as u32) << 23) | (m << 13)
        }
    } else if exp == 0x1f {
        (sign << 31) | 0x7f80_0000 | (mant << 13)
    } else {
        (sign << 31) | ((exp + 112) << 23) | (mant << 13)
    };
    f32::from_bits(out)
}

#[inline(always)]
fn f32_to_half(value: f32) -> u16 {
    let bits = value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xff) as i32;
    let mant = bits & 0x7f_ffff;
    if exp == 0xff {
        return sign | 0x7c00 | if mant != 0 { 0x200 } else { 0 };
    }
    if exp > 142 {
        return sign | 0x7c00;
    }
    if exp < 113 {
        return sign;
    }
    let e = (exp - 127 + 15) as u16;
    sign | (e << 10) | ((mant >> 13) as u16)
}

/// `bfcvt` to bf16.
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
pub unsafe fn bfcvt_f32(a: &[f32], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        let bits = a[lane].to_bits();
        out[lane] = if a[lane].is_nan() {
            ((bits >> 16) | 0x0040) as u16
        } else {
            let lsb = (bits >> 16) & 1;
            (bits.wrapping_add(0x7fff + lsb) >> 16) as u16
        };
    }
}

/// `bfcvtnt` to bf16.
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
pub unsafe fn bfcvtnt_f32(a: &[f32], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        let bits = a[lane].to_bits();
        out[lane] = if a[lane].is_nan() {
            ((bits >> 16) | 0x0040) as u16
        } else {
            let lsb = (bits >> 16) & 1;
            (bits.wrapping_add(0x7fff + lsb) >> 16) as u16
        };
    }
}

/// `fcvt` from `f64` to `f32`.
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
pub unsafe fn fcvt_f64_to_f32(a: &[f64], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        out[lane] = a[lane] as f32;
    }
}

/// `fcvtlt` from f16 bits to `f32`.
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
pub unsafe fn fcvtlt_f16_to_f32(a: &[u16], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        out[lane] = half_to_f32(a[lane]);
    }
}

/// `fcvtnt` from `f32` to f16 bits.
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
pub unsafe fn fcvtnt_f32(a: &[f32], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        out[lane] = f32_to_half(a[lane]);
    }
}

/// `fcvtx` from `f32` to f16 bits.
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
pub unsafe fn fcvtx_f32(a: &[f32], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        out[lane] = f32_to_half(a[lane]);
    }
}

/// `fcvtxnt` from `f32` to f16 bits.
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
pub unsafe fn fcvtxnt_f32(a: &[f32], out: &mut [u16]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..a.len() {
        out[lane] = f32_to_half(a[lane]);
    }
}
