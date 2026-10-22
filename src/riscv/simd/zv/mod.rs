//! RISC-V vector cryptography extensions.
//!
//! This module covers Zvkn, Zvks, Zvkg, and Zvbc.

use super::rvv::V128;

unsafe extern "C" {
    fn rust_isa_rvv_vaesef(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rvv_vaesem(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rvv_vaesdf(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rvv_vaesdm(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rvv_vaesz(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rvv_vaeskf1(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rvv_vaeskf2(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rvv_vsm4k(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rvv_vsm4r(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsha2ms(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsha2ch(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsha2cl(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsm3me(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsm3c(state: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rvv_vclmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vclmulh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vghsh(acc: *const u8, value: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rvv_vgmul(acc: *const u8, key: *const u8, out: *mut u8);
}

macro_rules! crypto_binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the vector cryptography extensions
        /// enabled.
        #[inline(always)]
        pub unsafe fn $name(state: V128, key: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(state.as_ptr(), key.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

crypto_binary!(
    /// `vaesef.vv`.
    vaesef,
    rust_isa_rvv_vaesef
);
crypto_binary!(
    /// `vaesem.vv`.
    vaesem,
    rust_isa_rvv_vaesem
);
crypto_binary!(
    /// `vaesdf.vv`.
    vaesdf,
    rust_isa_rvv_vaesdf
);
crypto_binary!(
    /// `vaesdm.vv`.
    vaesdm,
    rust_isa_rvv_vaesdm
);
crypto_binary!(
    /// `vaesz.vs`.
    vaesz,
    rust_isa_rvv_vaesz
);

macro_rules! crypto_key_schedule {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// `round` must be at most the largest immediate accepted by the
        /// instruction.
        ///
        /// # Safety
        ///
        /// The caller must compile with the vector cryptography extensions
        /// enabled and pass a valid round number.
        #[inline(always)]
        pub unsafe fn $name(key: V128, round: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(key.as_ptr(), round, out.as_mut_ptr());
            }
            out
        }
    };
}

crypto_key_schedule!(
    /// `vaeskf1.vi`, with `round` in `1..=10`.
    vaeskf1,
    rust_isa_rvv_vaeskf1
);
crypto_key_schedule!(
    /// `vaeskf2.vi`, with `round` in `2..=10`.
    vaeskf2,
    rust_isa_rvv_vaeskf2
);
crypto_key_schedule!(
    /// `vsm4k.vi`, with `round` in `0..=7`.
    vsm4k,
    rust_isa_rvv_vsm4k
);

crypto_binary!(
    /// `vsm4r.vv`.
    vsm4r,
    rust_isa_rvv_vsm4r
);

macro_rules! crypto_ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the vector cryptography extensions
        /// enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

crypto_ternary!(
    /// `vsha2ms.vv`.
    vsha2ms,
    rust_isa_rvv_vsha2ms
);
crypto_ternary!(
    /// `vsha2ch.vv`.
    vsha2ch,
    rust_isa_rvv_vsha2ch
);
crypto_ternary!(
    /// `vsha2cl.vv`.
    vsha2cl,
    rust_isa_rvv_vsha2cl
);
crypto_ternary!(
    /// `vghsh.vv`.
    vghsh,
    rust_isa_rvv_vghsh
);

/// `vsm3me.vv` on a 256-bit element group.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled.
#[inline(always)]
pub unsafe fn vsm3me(a: [V128; 2], b: [V128; 2]) -> [V128; 2] {
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vsm3me(
            a.as_ptr().cast(),
            b.as_ptr().cast(),
            out.as_mut_ptr().cast(),
        );
    }
    out
}

/// `vsm3c.vi` on a 256-bit element group.
///
/// `round` must be at most 31.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled and
/// pass a valid round number.
#[inline(always)]
pub unsafe fn vsm3c(state: [V128; 2], round: u32) -> [V128; 2] {
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vsm3c(state.as_ptr().cast(), round, out.as_mut_ptr().cast());
    }
    out
}

crypto_binary!(
    /// `vclmul.vv` on 64-bit lanes.
    vclmul,
    rust_isa_rvv_vclmul
);
crypto_binary!(
    /// `vclmulh.vv` on 64-bit lanes.
    vclmulh,
    rust_isa_rvv_vclmulh
);

/// `vgmul.vv`.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled.
#[inline(always)]
pub unsafe fn vgmul(acc: V128, key: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vgmul(acc.as_ptr(), key.as_ptr(), out.as_mut_ptr());
    }
    out
}
