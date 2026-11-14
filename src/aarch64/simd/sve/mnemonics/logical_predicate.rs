//! SVE predicate logical and break instructions.

macro_rules! binary_mask {
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
        pub unsafe fn $name(a: &[u32], b: &[u32], out: &mut [u32]) {
            assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
                *out_elem = *a_elem $op *b_elem;}
        }
    };
}

binary_mask!(
    /// `ands` on lane masks.
    ands,
    &
);

macro_rules! binary_mask_not {
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
        pub unsafe fn $name(a: &[u32], b: &[u32], out: &mut [u32]) {
            assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
                *out_elem = *a_elem $op !*b_elem;}
        }
    };
}

binary_mask_not!(
    /// `bics` on lane masks.
    bics,
    &
);
binary_mask_not!(
    /// `eon` on lane masks.
    eon,
    ^
);

binary_mask!(
    /// `eors` on lane masks.
    eors,
    ^
);
binary_mask!(
    /// `orrs` on lane masks.
    orrs,
    |
);

macro_rules! break_mask {
    ($(#[$meta:meta])* $name:ident, $include_first:expr) => {
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
        pub unsafe fn $name(a: &[u32], out: &mut [u32]) {
            assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
            let mut stopped = false;
            for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
                if stopped {
                    *out_elem = 0;
                } else if $include_first {
                    *out_elem = u32::MAX;
                    stopped = *a_elem != 0;
                } else if *a_elem != 0 {
                    *out_elem = 0;
                    stopped = true;
                } else {
                    *out_elem = u32::MAX;
                }}
        }
    };
}

break_mask!(
    /// `brka` on lane masks.
    brka,
    true
);
break_mask!(
    /// `brkas` on lane masks.
    brkas,
    true
);
break_mask!(
    /// `brkb` on lane masks.
    brkb,
    false
);
break_mask!(
    /// `brkbs` on lane masks.
    brkbs,
    false
);

macro_rules! break_binary {
    ($(#[$meta:meta])* $name:ident, $kind:literal) => {
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
            let mut seen = false;
            for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
                match $kind {
                    "brkn" => {
                        *out_elem = if seen { u32::MAX } else { 0 };
                        seen |= *a_elem != 0;
                        if *b_elem != 0 {
                            seen = false;
                        }
                    }
                    "brkpa" => {
                        seen |= *a_elem != 0 || *b_elem != 0;
                        *out_elem = if seen { u32::MAX } else { 0 };
                    }
                    _ => {
                        seen |= *a_elem != 0 && *b_elem == 0;
                        *out_elem = if seen { u32::MAX } else { 0 };
                    }
                }}
        }
    };
}

break_binary!(
    /// `brkn` on lane masks.
    brkn,
    "brkn"
);
break_binary!(
    /// `brkns` on lane masks.
    brkns,
    "brkn"
);
break_binary!(
    /// `brkpa` on lane masks.
    brkpa,
    "brkpa"
);
break_binary!(
    /// `brkpas` on lane masks.
    brkpas,
    "brkpa"
);
break_binary!(
    /// `brkpb` on lane masks.
    brkpb,
    "brkpb"
);
break_binary!(
    /// `brkpbs` on lane masks.
    brkpbs,
    "brkpb"
);

/// `nands` on lane masks.
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
pub unsafe fn nands(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
        *out_elem = !(*a_elem & *b_elem);
    }
}

/// `nors` on lane masks.
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
pub unsafe fn nors(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
        *out_elem = !(*a_elem | *b_elem);
    }
}

/// `orns` on lane masks.
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
pub unsafe fn orns(a: &[u32], b: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for ((out_elem, a_elem), b_elem) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
        *out_elem = *a_elem | !*b_elem;
    }
}

/// `nots` on lane masks.
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
pub unsafe fn nots(a: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for (out_elem, a_elem) in out.iter_mut().zip(a.iter()) {
        *out_elem = !*a_elem;
    }
}

/// `nbsl` on lane masks.
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
pub unsafe fn nbsl(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    assert_eq!(a.len(), b.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), c.len(), "SVE slices must have the same length");
    assert_eq!(a.len(), out.len(), "SVE slices must have the same length");
    for (((out_elem, a_elem), b_elem), c_elem) in
        out.iter_mut().zip(a.iter()).zip(b.iter()).zip(c.iter())
    {
        *out_elem = !((*a_elem & !*b_elem) | (*c_elem & *b_elem));
    }
}
