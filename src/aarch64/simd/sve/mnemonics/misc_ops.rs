//! SVE miscellaneous arithmetic and bitwise instructions.

macro_rules! mla_mls_lane {
    ($(#[$meta:meta])* $lane:ident, $op:tt) => {
        $(#[$meta])*
        ///
        /// Indexed form using lane 0 of `b`.
        ///
        /// # Panics
        ///
        /// Panics when the slice lengths differ or `b` is empty.
        ///
        /// # Safety
        ///
        /// The caller must enable SVE and the CPU must support it.
        #[target_feature(enable = "sve")]
        #[inline]
        pub unsafe fn $lane(acc: &[i32], a: &[i32], b: &[i32], out: &mut [i32]) {
            assert_eq!(acc.len(), a.len(), "SVE slices must have the same length");
            assert_eq!(acc.len(), out.len(), "SVE slices must have the same length");
            assert!(!b.is_empty(), "SVE indexed input must not be empty");
            let factor = b[0];
            for lane in 0..out.len() {
                out[lane] = acc[lane] $op a[lane].wrapping_mul(factor);
            }
        }
    };
}

mla_mls_lane!(
    /// `mla` indexed form on `i32` lanes.
    mla_lane_i32,
    +
);
mla_mls_lane!(
    /// `mls` indexed form on `i32` lanes.
    mls_lane_i32,
    -
);

/// `rdffr` predicated form.
///
/// The slice API treats every predicate lane as active, so the result is all ones.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rdffr_z(pg: u64) -> u64 {
    unsafe extern "C" {
        fn rust_isa_sve_rdffr() -> u64;
    }
    (unsafe { rust_isa_sve_rdffr() }) & pg
}

/// `bsl1n` on byte lanes.
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
pub unsafe fn bsl1n_u8(a: &[u8], b: &[u8], c: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..out.len() {
        out[lane] = (a[lane] & !b[lane]) | (c[lane] & b[lane]);
    }
}

/// `bsl2n` on byte lanes.
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
pub unsafe fn bsl2n_u8(a: &[u8], b: &[u8], c: &[u8], out: &mut [u8]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for lane in 0..out.len() {
        out[lane] = (a[lane] & b[lane]) | (c[lane] & !b[lane]);
    }
}

unsafe extern "C" {
    fn rust_isa_sve_addhnb_i16(a: *const i16, b: *const i16, out: *mut i8, len: usize);
    fn rust_isa_sve_addhnt_i16(a: *const i16, b: *const i16, out: *mut i8, len: usize);
}

/// `addhnb` on `i16` sources narrowing to `i8`.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addhnb_i16(a: &[i16], b: &[i16], out: &mut [i8]) {
    assert!(a.len() >= out.len() * 2, "SVE source is too short");
    assert!(b.len() >= out.len() * 2, "SVE source is too short");
    unsafe {
        rust_isa_sve_addhnb_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

/// `addhnt` on `i16` sources narrowing to `i8`.
///
/// # Panics
///
/// Panics when the slice lengths are inconsistent.
///
/// # Safety
///
/// The caller must enable SVE2 and the CPU must support it.
#[target_feature(enable = "sve2")]
#[inline]
pub unsafe fn addhnt_i16(a: &[i16], b: &[i16], out: &mut [i8]) {
    assert!(a.len() >= out.len() * 2, "SVE source is too short");
    assert!(b.len() >= out.len() * 2, "SVE source is too short");
    unsafe {
        rust_isa_sve_addhnt_i16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

/// `movs` unpredicated form on byte lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn movs_unpred(pg: u64, out: &mut [u8]) {
    for (lane, value) in out.iter_mut().enumerate() {
        *value = if pg & (1u64 << lane) != 0 { 0xff } else { 0 };
    }
}

/// `movs` predicated form on byte lanes.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn movs_pred(pg: u64, mask: u64, out: &mut [u8]) {
    let active = pg & mask;
    for (lane, value) in out.iter_mut().enumerate() {
        *value = if active & (1u64 << lane) != 0 {
            0xff
        } else {
            0
        };
    }
}
