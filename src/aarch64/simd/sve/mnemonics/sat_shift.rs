//! SVE saturating shift, subtract, and accumulate instructions.

macro_rules! shift_sat_signed {
    ($(#[$meta:meta])* $name:ident $(,)?) => {
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
        pub unsafe fn $name(a: &[i32], b: &[i32], out: &mut [i32]) {
            assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            for lane in 0..a.len() {
                let shift = (b[lane] as u32) & 31;
                let wide = (a[lane] as i128) << shift;
                out[lane] = wide.clamp(i32::MIN as i128, i32::MAX as i128) as i32;
            }
        }
    };
}

macro_rules! shift_sat_unsigned {
    ($(#[$meta:meta])* $name:ident $(,)?) => {
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
        pub unsafe fn $name(a: &[u32], b: &[u32], out: &mut [u32]) {
            assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            for lane in 0..a.len() {
                let shift = (b[lane] as u32) & 31;
                let wide = (a[lane] as u128) << shift;
                out[lane] = wide.min(u32::MAX as u128) as u32;
            }
        }
    };
}

shift_sat_signed!(
    /// `sqshlr` on `i32` lanes.
    sqshlr_i32,
);
shift_sat_signed!(
    /// `sqrshlr` on `i32` lanes.
    sqrshlr_i32,
);
shift_sat_unsigned!(
    /// `uqshlr` on `u32` lanes.
    uqshlr_u32,
);
shift_sat_unsigned!(
    /// `uqrshlr` on `u32` lanes.
    uqrshlr_u32,
);

macro_rules! shift_wrap {
    ($(#[$meta:meta])* $name:ident, $ty:ty) => {
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
        pub unsafe fn $name(a: &[$ty], b: &[$ty], out: &mut [$ty]) {
            assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            for lane in 0..a.len() {
                out[lane] = a[lane].wrapping_shl((b[lane] as u32) & 31);
            }
        }
    };
}

shift_wrap!(
    /// `srshlr` on `i32` lanes.
    srshlr_i32,
    i32
);
shift_wrap!(
    /// `urshlr` on `u32` lanes.
    urshlr_u32,
    u32
);

macro_rules! subr {
    ($(#[$meta:meta])* $name:ident, $ty:ty) => {
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
        pub unsafe fn $name(a: &[$ty], b: &[$ty], out: &mut [$ty]) {
            assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            for lane in 0..a.len() {
                out[lane] = b[lane].saturating_sub(a[lane]);
            }
        }
    };
}

subr!(
    /// `sqsubr` on `i32` lanes.
    sqsubr_i32,
    i32
);
subr!(
    /// `uqsubr` on `u32` lanes.
    uqsubr_u32,
    u32
);

macro_rules! uqadd {
    ($(#[$meta:meta])* $name:ident, $ty:ty) => {
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
        pub unsafe fn $name(a: &[$ty], b: &[$ty], out: &mut [$ty]) {
            assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            for lane in 0..a.len() {
                out[lane] = a[lane].saturating_add(b[lane]);
            }
        }
    };
}

uqadd!(
    /// `suqadd` on `i32` lanes.
    suqadd_i32,
    i32
);
uqadd!(
    /// `usqadd` on `u32` lanes.
    usqadd_u32,
    u32
);
