//! SVE vector length instructions.

unsafe extern "C" {
    fn rust_isa_sve_cntb() -> u64;
}
/// Number of bytes in an SVE vector for `cntb`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cntb() -> usize {
    unsafe { rust_isa_sve_cntb() as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_cnth() -> u64;
}
/// Number of 16-bit lanes in an SVE vector for `cnth`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cnth() -> usize {
    unsafe { rust_isa_sve_cnth() as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_cntw() -> u64;
}
/// Number of 32-bit lanes in an SVE vector for `cntw`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cntw() -> usize {
    unsafe { rust_isa_sve_cntw() as usize }
}

unsafe extern "C" {
    fn rust_isa_sve_cntd() -> u64;
}
/// Number of 64-bit lanes in an SVE vector for `cntd`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn cntd() -> usize {
    unsafe { rust_isa_sve_cntd() as usize }
}

/// `addpl` adds `IMM` predicate lengths to `x`.
///
/// `IMM` must be in `-32..=31`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn addpl<const IMM: i8>(x: i64) -> i64 {
    const {
        assert!(
            IMM >= -32 && IMM <= 31,
            "addpl immediate must be in -32..=31"
        )
    };
    let out: i64;
    unsafe {
        core::arch::asm!(
            "addpl {out}, {x}, #{imm}",
            out = out(reg) out,
            x = in(reg) x,
            imm = const IMM,
            options(nomem, nostack, pure),
        );
    }
    out
}

/// `addvl` adds `IMM` vector lengths to `x`.
///
/// `IMM` must be in `-32..=31`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn addvl<const IMM: i8>(x: i64) -> i64 {
    const {
        assert!(
            IMM >= -32 && IMM <= 31,
            "addvl immediate must be in -32..=31"
        )
    };
    let out: i64;
    unsafe {
        core::arch::asm!(
            "addvl {out}, {x}, #{imm}",
            out = out(reg) out,
            x = in(reg) x,
            imm = const IMM,
            options(nomem, nostack, pure),
        );
    }
    out
}

/// `rdvl` returns `IMM` vector lengths in bytes.
///
/// `IMM` must be in `-32..=31`.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn rdvl<const IMM: i8>() -> i64 {
    const {
        assert!(
            IMM >= -32 && IMM <= 31,
            "rdvl immediate must be in -32..=31"
        )
    };
    let out: i64;
    unsafe {
        core::arch::asm!(
            "rdvl {out}, #{imm}",
            out = out(reg) out,
            imm = const IMM,
            options(nomem, nostack, pure),
        );
    }
    out
}
