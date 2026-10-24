//! AMX-AVX512 tile row moves.
//!
//! These instructions copy a row of a tile to a `zmm` register. The
//! conversion forms interpret the source row as `i32` or `f32` elements and
//! convert them to `f32`, `bf16`, or `f16`. AMX-AVX512 requires both AMX tile
//! state and AVX-512 state. rustc does not allow the feature on stable, so the
//! caller must compile the whole crate with AMX-AVX512 enabled and hold an
//! [`AmxGuard`].
//!
//! The runtime forms take a 32-bit row selector: the low 16 bits select the
//! row and the high 16 bits select the 64-byte row chunk. The immediate forms
//! use the instruction's 8-bit encoding: the low 6 bits select the row and the
//! high 2 bits select the chunk.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).

use core::arch::asm;

use super::amx_tile::AmxGuard;
use crate::x86::simd::avx512f::V512;

macro_rules! tile_row_to_zmm {
    ($name:ident, $instruction:literal, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must hold an [`AmxGuard`],
        /// enable AMX-AVX512 and AVX-512 state, and configure `TILE`. `TILE`
        /// must be in `0..=7`; a larger value is a compile-time error.
        #[inline]
        pub unsafe fn $name<const TILE: u8>(_guard: &AmxGuard, row: u32) -> V512 {
            const { assert!(TILE <= 7, "AMX tile number must be in 0..=7") };
            let mut out = V512::from_bytes([0; 64]);
            macro_rules! arm {
                        ($number:literal) => {
                            unsafe {
                                asm!(
                                    concat!($instruction, " zmm0, tmm", $number, ", {row:e}"),
                                    "vmovdqu64 [{out}], zmm0",
                                    row = in(reg) row,
                                    out = in(reg) out.as_mut_ptr(),
                                    out("zmm0") _,
                                    options(nostack, preserves_flags),
                                );
                            }
                        };
                    }
            match TILE {
                0 => arm!(0),
                1 => arm!(1),
                2 => arm!(2),
                3 => arm!(3),
                4 => arm!(4),
                5 => arm!(5),
                6 => arm!(6),
                7 => arm!(7),
                _ => unreachable!(),
            }
            out
        }
    };
}

macro_rules! tile_row_to_zmm_imm {
    ($name:ident, $instruction:literal, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must hold an [`AmxGuard`],
        /// enable AMX-AVX512 and AVX-512 state, and configure `TILE`. `TILE`
        /// must be in `0..=7`; a larger value is a compile-time error.
        #[inline]
        pub unsafe fn $name<const TILE: u8, const ROW: u8>(_guard: &AmxGuard) -> V512 {
            const { assert!(TILE <= 7, "AMX tile number must be in 0..=7") };
            let mut out = V512::from_bytes([0; 64]);
            macro_rules! arm {
                                ($number:literal) => {
                                    unsafe {
                                        asm!(
                                            concat!($instruction, " zmm0, tmm", $number, ", {row}"),
                                            "vmovdqu64 [{out}], zmm0",
                                            row = const ROW,
                                            out = in(reg) out.as_mut_ptr(),
                                            out("zmm0") _,
                                            options(nostack, preserves_flags),
                                        );
                                    }
                                };
                            }
            match TILE {
                0 => arm!(0),
                1 => arm!(1),
                2 => arm!(2),
                3 => arm!(3),
                4 => arm!(4),
                5 => arm!(5),
                6 => arm!(6),
                7 => arm!(7),
                _ => unreachable!(),
            }
            out
        }
    };
}

tile_row_to_zmm!(
    tilemovrow,
    "tilemovrow",
    "`tilemovrow::<TILE>`\n\nCopy the row selected by `row` from `TILE` without conversion."
);
tile_row_to_zmm_imm!(
    tilemovrow_imm,
    "tilemovrow",
    "`tilemovrow_imm::<TILE, ROW>`\n\nCopy the row selected by the immediate `ROW` from `TILE` without conversion."
);
tile_row_to_zmm!(
    tcvtrowd2ps,
    "tcvtrowd2ps",
    "`tcvtrowd2ps::<TILE>`\n\nCopy the row selected by `row` from `TILE`, converting `i32` elements to `f32`."
);
tile_row_to_zmm_imm!(
    tcvtrowd2ps_imm,
    "tcvtrowd2ps",
    "`tcvtrowd2ps_imm::<TILE, ROW>`\n\nCopy the row selected by the immediate `ROW` from `TILE`, converting `i32` elements to `f32`."
);
tile_row_to_zmm!(
    tcvtrowps2bf16h,
    "tcvtrowps2bf16h",
    "`tcvtrowps2bf16h::<TILE>`\n\nCopy the row selected by `row` from `TILE`, converting `f32` elements to `bf16` in the high half of each 32-bit lane."
);
tile_row_to_zmm_imm!(
    tcvtrowps2bf16h_imm,
    "tcvtrowps2bf16h",
    "`tcvtrowps2bf16h_imm::<TILE, ROW>`\n\nCopy the row selected by the immediate `ROW` from `TILE`, converting `f32` elements to `bf16` in the high half of each 32-bit lane."
);
tile_row_to_zmm!(
    tcvtrowps2bf16l,
    "tcvtrowps2bf16l",
    "`tcvtrowps2bf16l::<TILE>`\n\nCopy the row selected by `row` from `TILE`, converting `f32` elements to `bf16` in the low half of each 32-bit lane."
);
tile_row_to_zmm_imm!(
    tcvtrowps2bf16l_imm,
    "tcvtrowps2bf16l",
    "`tcvtrowps2bf16l_imm::<TILE, ROW>`\n\nCopy the row selected by the immediate `ROW` from `TILE`, converting `f32` elements to `bf16` in the low half of each 32-bit lane."
);
tile_row_to_zmm!(
    tcvtrowps2phh,
    "tcvtrowps2phh",
    "`tcvtrowps2phh::<TILE>`\n\nCopy the row selected by `row` from `TILE`, converting `f32` elements to `f16` in the high half of each 32-bit lane."
);
tile_row_to_zmm_imm!(
    tcvtrowps2phh_imm,
    "tcvtrowps2phh",
    "`tcvtrowps2phh_imm::<TILE, ROW>`\n\nCopy the row selected by the immediate `ROW` from `TILE`, converting `f32` elements to `f16` in the high half of each 32-bit lane."
);
tile_row_to_zmm!(
    tcvtrowps2phl,
    "tcvtrowps2phl",
    "`tcvtrowps2phl::<TILE>`\n\nCopy the row selected by `row` from `TILE`, converting `f32` elements to `f16` in the low half of each 32-bit lane."
);
tile_row_to_zmm_imm!(
    tcvtrowps2phl_imm,
    "tcvtrowps2phl",
    "`tcvtrowps2phl_imm::<TILE, ROW>`\n\nCopy the row selected by the immediate `ROW` from `TILE`, converting `f32` elements to `f16` in the low half of each 32-bit lane."
);
