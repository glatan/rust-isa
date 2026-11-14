//! SVE duplicate and conditionally-extract instructions.

/// `dup` indexed for `i32` lanes.
///
/// # Panics
///
/// Panics when `out` is empty or `index` is out of range.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn dup_lane_i32(a: &[i32], index: usize, out: &mut [i32]) {
    assert!(!out.is_empty(), "SVE output must not be empty");
    assert!(index < a.len(), "SVE lane index is out of range");
    let value = a[index];
    for lane in out.iter_mut() {
        *lane = value;
    }
}

/// `dup` immediate for byte lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn dup_imm_u8(imm: u8, out: &mut [u8]) {
    for lane in out.iter_mut() {
        *lane = imm;
    }
}

/// `dupm` for `u64` lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn dupm_u64(mask: u64, out: &mut [u64]) {
    for lane in out.iter_mut() {
        *lane = mask;
    }
}

/// `clasta` scalar for `i32` lanes.
///
/// Returns the last active element, or `fallback` when no lane is active.
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
pub unsafe fn clasta_i32(mask: &[u32], fallback: i32, a: &[i32]) -> i32 {
    assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
    let mut result = fallback;
    for (mask_elem, a_elem) in mask.iter().zip(a.iter()) {
        if *mask_elem != 0 {
            result = *a_elem;
        }
    }
    result
}

/// `clastb` scalar for `i32` lanes.
///
/// Returns the first active element, or `fallback` when no lane is active.
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
pub unsafe fn clastb_i32(mask: &[u32], fallback: i32, a: &[i32]) -> i32 {
    assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
    for (mask_elem, a_elem) in mask.iter().zip(a.iter()) {
        if *mask_elem != 0 {
            return *a_elem;
        }
    }
    fallback
}

/// `index` with immediate start and step.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn index_imm_i32(start: i32, step: i32, out: &mut [i32]) {
    index_impl(start, step, out);
}

/// `index` with immediate start and scalar step.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn index_imm_scalar_i32(start: i32, step: i32, out: &mut [i32]) {
    index_impl(start, step, out);
}

/// `index` with scalar start and immediate step.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn index_scalar_imm_i32(start: i32, step: i32, out: &mut [i32]) {
    index_impl(start, step, out);
}

/// `index` with scalar start and step.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn index_scalar_i32(start: i32, step: i32, out: &mut [i32]) {
    index_impl(start, step, out);
}

#[inline(always)]
fn index_impl(start: i32, step: i32, out: &mut [i32]) {
    for (lane, value) in out.iter_mut().enumerate() {
        *value = start.wrapping_add((lane as i32).wrapping_mul(step));
    }
}

macro_rules! clast_simd {
    ($(#[$meta:meta])* $name:ident, $last:expr) => {
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
        pub unsafe fn $name(mask: &[u32], fallback: i32, a: &[i32]) -> i32 {
            assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
            let mut result = fallback;
            if $last {
                for (mask_elem, a_elem) in mask.iter().zip(a.iter()) {
                    if *mask_elem != 0 {
                        result = *a_elem;
                    }}
            } else {
                for (mask_elem, a_elem) in mask.iter().zip(a.iter()) {
                    if *mask_elem != 0 {
                        return *a_elem;
                    }}
            }
            result
        }
    };
}

clast_simd!(
    /// `clasta` SIMD and FP scalar form on `i32` lanes.
    clasta_simd_i32,
    true
);
clast_simd!(
    /// `clastb` SIMD and FP scalar form on `i32` lanes.
    clastb_simd_i32,
    false
);

macro_rules! clast_vec {
    ($(#[$meta:meta])* $name:ident, $last:expr) => {
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
        pub unsafe fn $name(mask: &[u32], fallback: &[i32], a: &[i32], out: &mut [i32]) {
            assert_eq!(mask.len(), a.len(), "SVE slices must have the same length");
            assert_eq!(mask.len(), fallback.len(), "SVE slices must have the same length");
            assert_eq!(mask.len(), out.len(), "SVE slices must have the same length");
            let mut selected = None;
            if $last {
                for (mask_elem, a_elem) in mask.iter().zip(a.iter()) {
                    if *mask_elem != 0 {
                        selected = Some(*a_elem);
                    }}
            } else {
                for (mask_elem, a_elem) in mask.iter().zip(a.iter()) {
                    if *mask_elem != 0 {
                        selected = Some(*a_elem);
                        break;
                    }}
            }
            for ((out_elem, mask_elem), fallback_elem) in out.iter_mut().zip(mask.iter()).zip(fallback.iter()) {
                *out_elem = if *mask_elem != 0 {
                    selected.unwrap_or(*fallback_elem)
                } else {
                    *fallback_elem
                };}
        }
    };
}

clast_vec!(
    /// `clasta` vector form on `i32` lanes.
    clasta_vec_i32,
    true
);
clast_vec!(
    /// `clastb` vector form on `i32` lanes.
    clastb_vec_i32,
    false
);
