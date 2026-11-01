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
            for lane in 0..a.len() {
                out[lane] = a[lane] $op b[lane];
            }
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
            for lane in 0..a.len() {
                out[lane] = a[lane] $op !b[lane];
            }
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
            for lane in 0..a.len() {
                if stopped {
                    out[lane] = 0;
                } else if $include_first {
                    out[lane] = u32::MAX;
                    stopped = a[lane] != 0;
                } else if a[lane] != 0 {
                    out[lane] = 0;
                    stopped = true;
                } else {
                    out[lane] = u32::MAX;
                }
            }
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
            for lane in 0..a.len() {
                match $kind {
                    "brkn" => {
                        out[lane] = if seen { u32::MAX } else { 0 };
                        seen |= a[lane] != 0;
                        if b[lane] != 0 {
                            seen = false;
                        }
                    }
                    "brkpa" => {
                        seen |= a[lane] != 0 || b[lane] != 0;
                        out[lane] = if seen { u32::MAX } else { 0 };
                    }
                    _ => {
                        seen |= a[lane] != 0 && b[lane] == 0;
                        out[lane] = if seen { u32::MAX } else { 0 };
                    }
                }
            }
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
    for lane in 0..a.len() {
        out[lane] = !(a[lane] & b[lane]);
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
    for lane in 0..a.len() {
        out[lane] = !(a[lane] | b[lane]);
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
    for lane in 0..a.len() {
        out[lane] = a[lane] | !b[lane];
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
    for lane in 0..a.len() {
        out[lane] = !a[lane];
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
    for lane in 0..a.len() {
        out[lane] = !((a[lane] & !b[lane]) | (c[lane] & b[lane]));
    }
}
