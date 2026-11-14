//! SVE miscellaneous remaining instructions.

/// `adr` on `u64` lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn adr_u64(base: u64, offsets: &[u64], out: &mut [u64]) {
    assert_eq!(
        offsets.len(),
        out.len(),
        "SVE slices must have the same length"
    );
    for (out_elem, offsets_elem) in out.iter_mut().zip(offsets.iter()) {
        *out_elem = base.wrapping_add(*offsets_elem);
    }
}

macro_rules! fac {
    ($(#[$meta:meta])* $name:ident, $op:tt) => {
        $(#[$meta])*
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
        pub unsafe fn $name(a: &[f32], b: &[f32], out: &mut [u32]) {
            assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
                *out_elem = if a_elem.abs() $op b_elem.abs() {
                    u32::MAX
                } else {
                    0
                };}
        }
    };
}

fac!(
    /// `facle`
    facle_f32,
    <=
);
fac!(
    /// `faclt`
    faclt_f32,
    <
);

/// `fsubr` immediate form on `f32` lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fsubr_imm_f32(a: &[f32], imm: f32, out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = imm - *a_elem;
    }
}

/// `insr` scalar form on `i32` lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn insr_scalar_i32(a: &[i32], value: i32, out: &mut [i32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    out.copy_from_slice(a);
    if !out.is_empty() {
        out[0] = value;
    }
}

/// `insr` SIMD and FP scalar form on `f32` lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn insr_simd_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert!(!b.is_empty(), "SVE source must not be empty");
    out.copy_from_slice(a);
    if !out.is_empty() {
        out[0] = b[0];
    }
}

/// `match` on byte lanes.
///
/// Each 128-bit segment is searched independently and a matching lane is
/// returned as `0xff`.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn match_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    for segment in 0..a.len().div_ceil(16) {
        let start = segment * 16;
        let end = (start + 16).min(a.len());
        for lane in start..end {
            out[lane] = if b[start..end].contains(&a[lane]) {
                0xff
            } else {
                0
            };
        }
    }
}

/// `nmatch` on byte lanes.
///
/// Each 128-bit segment is searched independently and a non-matching lane is
/// returned as `0xff`.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn nmatch_u8(a: &[u8], b: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    for segment in 0..a.len().div_ceil(16) {
        let start = segment * 16;
        let end = (start + 16).min(a.len());
        for lane in start..end {
            out[lane] = if b[start..end].contains(&a[lane]) {
                0
            } else {
                0xff
            };
        }
    }
}

/// `tbx` on byte lanes.
///
/// Out-of-range indices leave the destination lane unchanged.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn tbx_u8(table: &[u8], indices: &[u8], out: &mut [u8]) {
    assert_eq!(
        indices.len(),
        out.len(),
        "SVE slices must have the same length"
    );
    for (indices_elem, out_elem) in indices.iter().zip(out.iter_mut()) {
        if let Some(&value) = table.get(*indices_elem as usize) {
            *out_elem = value;
        }
    }
}

/// `ursqrte` on `u32` lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ursqrte_u32(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = if *a_elem == 0 { u32::MAX } else { 1 };
    }
}

fn range_mask(start: usize, end: usize) -> u64 {
    let count = end.saturating_sub(start).min(64);
    if count == 64 {
        u64::MAX
    } else if count == 0 {
        0
    } else {
        (1u64 << count) - 1
    }
}

/// `whilerw`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilerw_u32(start: usize, end: usize) -> u64 {
    range_mask(start, end)
}

/// `whilewr`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn whilewr_u32(start: usize, end: usize) -> u64 {
    range_mask(start, end)
}

/// `histcnt` on `u32` lanes.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn histcnt_u32(data: &[u32], counts: &[u32], out: &mut [u32]) {
    assert_eq!(
        data.len(),
        out.len(),
        "SVE slices must have the same length"
    );
    for (out_elem, data_elem) in out.iter_mut().zip(data.iter()) {
        *out_elem = counts.iter().filter(|&&v| v == *data_elem).count() as u32;
    }
}

/// `histseg` on byte lanes.
///
/// Each 128-bit segment is counted independently.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn histseg_u8(data: &[u8], counts: &[u8], out: &mut [u8]) {
    assert_eq!(
        data.len(),
        out.len(),
        "SVE slices must have the same length"
    );
    assert_eq!(
        data.len(),
        counts.len(),
        "SVE slices must have the same length"
    );
    for segment in 0..data.len().div_ceil(16) {
        let start = segment * 16;
        let end = (start + 16).min(data.len());
        for lane in start..end {
            out[lane] = counts[start..end]
                .iter()
                .filter(|&&v| v == data[lane])
                .count() as u8;
        }
    }
}

/// `sqrdcmlah` indexed form on `i32` lanes.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sqrdcmlah_indexed_i32(acc: &[i32], a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    for (((out_elem, acc_elem), a_elem), b_elem) in
        out.iter_mut().zip(acc.iter()).zip(a.iter()).zip(b.iter())
    {
        *out_elem = acc_elem.saturating_add(a_elem.wrapping_mul(*b_elem).wrapping_mul(2));
    }
}

/// `sqrdcmlah` vector form on `i32` lanes.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn sqrdcmlah_vec_i32(acc: &[i32], a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
    for (((out_elem, acc_elem), a_elem), b_elem) in
        out.iter_mut().zip(acc.iter()).zip(a.iter()).zip(b.iter())
    {
        *out_elem = acc_elem.saturating_add(a_elem.wrapping_mul(*b_elem).wrapping_mul(2));
    }
}
