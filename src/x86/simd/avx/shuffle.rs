//! Shuffle and unpack instructions.

use super::V256;
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx_vunpcklps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vunpckhps(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vunpcklpd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_avx_vunpckhpd(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($name:ident, $symbol:ident, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must enable AVX.
        #[target_feature(enable = "avx")]
        #[inline]
        pub unsafe fn $name(a: V256, b: V256) -> V256 {
            let mut out = V256::from_bytes([0; 32]);
            unsafe {
                $symbol(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    vunpcklps,
    rust_isa_avx_vunpcklps,
    "Interleave the low 32-bit float lanes of `a` and `b`."
);
binary!(
    vunpckhps,
    rust_isa_avx_vunpckhps,
    "Interleave the high 32-bit float lanes of `a` and `b`."
);
binary!(
    vunpcklpd,
    rust_isa_avx_vunpcklpd,
    "Interleave the low 64-bit float lanes of `a` and `b`."
);
binary!(
    vunpckhpd,
    rust_isa_avx_vunpckhpd,
    "Interleave the high 64-bit float lanes of `a` and `b`."
);
macro_rules! shuffle {
    ($name:ident, $mnemonic:literal, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must enable AVX.
        #[target_feature(enable = "avx")]
        #[inline]
        pub unsafe fn $name<const IMM: u8>(a: V256, b: V256) -> V256 {
            let mut out = V256::from_bytes([0; 32]);
            unsafe {
                asm!(
                    "vmovdqu ymm0, [{a}]",
                    concat!($mnemonic, " ymm0, ymm0, [{b}], {imm}"),
                    "vmovdqu [{out}], ymm0",
                    a = in(reg) a.as_ptr(),
                    b = in(reg) b.as_ptr(),
                    out = in(reg) out.as_mut_ptr(),
                    imm = const IMM,
                    out("ymm0") _,
                    options(nostack, preserves_flags),
                );
            }
            out
        }
    };
}

shuffle!(
    vshufps,
    "vshufps",
    "Shuffle 32-bit float lanes from `a` and `b` with the immediate."
);
shuffle!(
    vshufpd,
    "vshufpd",
    "Shuffle 64-bit float lanes from `a` and `b` with the immediate."
);
