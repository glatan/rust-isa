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
            for ((b_elem, a_elem), out_elem) in b.iter().zip(a.iter()).zip(out.iter_mut()) {
                let shift = (*b_elem as u32) & 31;
                let wide = (*a_elem as i128) << shift;
                *out_elem = wide.clamp(i32::MIN as i128, i32::MAX as i128) as i32;}
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
            for ((b_elem, a_elem), out_elem) in b.iter().zip(a.iter()).zip(out.iter_mut()) {
                let shift = (*b_elem as u32) & 31;
                let wide = (*a_elem as u128) << shift;
                *out_elem = wide.min(u32::MAX as u128) as u32;}
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
            for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
                *out_elem = a_elem.wrapping_shl((*b_elem as u32) & 31);}
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
            for ((out_elem, b_elem), a_elem) in out.iter_mut().zip(b.iter()).zip(a.iter()) {
                *out_elem = b_elem.saturating_sub(*a_elem);}
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
            for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
                *out_elem = a_elem.saturating_add(*b_elem);}
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
