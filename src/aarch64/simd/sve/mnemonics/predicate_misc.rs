//! SVE predicate generation and test instructions.

/// `pfalse`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn pfalse() -> u64 {
    0
}

/// `ptrue`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ptrue() -> u64 {
    u64::MAX
}

/// `ptrues`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ptrues() -> u64 {
    u64::MAX
}

/// `pfirst`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn pfirst(pg: u64) -> u64 {
    if pg == 0 {
        0
    } else {
        1u64 << pg.trailing_zeros()
    }
}

/// `pnext`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn pnext(pg: u64) -> u64 {
    if pg & pg.wrapping_neg() == 0 {
        0
    } else {
        pg & pg.wrapping_sub(1)
    }
}

/// `ptest`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ptest(pg: u64) -> bool {
    pg != 0
}

/// `cntp`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cntp(pg: u64) -> u64 {
    pg.count_ones() as u64
}

/// `ctermeq`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ctermeq(a: u64, b: u64) -> bool {
    a == b
}

/// `ctermne`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ctermne(a: u64, b: u64) -> bool {
    a != b
}

unsafe extern "C" {
    fn rust_isa_sve_setffr();
    fn rust_isa_sve_wrffr(pg: u64);
    fn rust_isa_sve_rdffrs(pg: u64) -> u64;
}

/// `setffr`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn setffr() {
    unsafe { rust_isa_sve_setffr() };
}

/// `wrffr`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn wrffr(pg: u64) {
    unsafe { rust_isa_sve_wrffr(pg) };
}

/// `rdffrs`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rdffrs(pg: u64) -> u64 {
    unsafe { rust_isa_sve_rdffrs(pg) }
}
