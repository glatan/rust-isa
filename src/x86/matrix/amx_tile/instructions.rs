//! AMX tile load, store, and matrix-multiply instructions.
//!
//! These wrappers use inline assembly because rustc does not expose the AMX
//! tile number as a value type. Matrix-multiply results are accumulated into
//! the destination tile in the AMX tile state.

use core::arch::asm;

use super::TileConfig;

/// `ldtilecfg`
///
/// Load `config` as the current tile configuration.
///
/// # Safety
///
/// The CPU and operating system must allow AMX tile state, `config` must
/// describe a valid tile configuration, and the caller must not create a
/// nested [`super::AmxGuard`].
#[inline]
pub unsafe fn ldtilecfg(config: &TileConfig) {
    unsafe {
        asm!(
            "ldtilecfg [{config}]",
            config = in(reg) config.as_ptr(),
            options(nostack, readonly),
        );
    }
}

/// `sttilecfg`
///
/// Store the current tile configuration to `config`.
///
/// # Safety
///
/// The CPU and operating system must allow AMX tile state and the caller must
/// hold an [`super::AmxGuard`] or an equivalent active tile state.
#[inline]
pub unsafe fn sttilecfg(config: &mut TileConfig) {
    unsafe {
        asm!(
            "sttilecfg [{config}]",
            config = in(reg) config.as_mut_ptr(),
            options(nostack),
        );
    }
}

macro_rules! tile_memory {
    ($name:ident, $instruction:literal, $doc:literal, $pointer:ty) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must hold an [`super::AmxGuard`], enable the AMX tile
        /// state, and pass a valid base pointer and row stride. `TILE` must be
        /// in `0..=7`; a larger value is a compile-time error.
        #[inline]
        pub unsafe fn $name<const TILE: u8>(base: $pointer, stride: usize) {
            const { assert!(TILE <= 7, "AMX tile number must be in 0..=7") };
            macro_rules! arm {
                        ($number:literal) => {
                            unsafe {
                                asm!(
                                    concat!($instruction, " tmm", $number, ", [{base} + {stride}]"),
                                    base = in(reg) base,
                                    stride = in(reg) stride,
                                    options(nostack, readonly),
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
        }
    };
}

tile_memory!(
    tileloadd,
    "tileloadd",
    "`tileloadd::<TILE>`\n\nLoad the tile from `base` with row stride `stride`.",
    *const u8
);

tile_memory!(
    tileloaddt1,
    "tileloaddt1",
    "`tileloaddt1::<TILE>`\n\nLoad the tile with a temporal hint that the data is not reused.",
    *const u8
);

/// `tilestored::<TILE>`
///
/// Store the tile to `base` with row stride `stride`.
///
/// # Safety
///
/// The caller must hold an [`super::AmxGuard`], enable the AMX tile state, and
/// pass a valid base pointer and row stride. `TILE` must be in `0..=7`; a
/// larger value is a compile-time error.
#[inline]
pub unsafe fn tilestored<const TILE: u8>(base: *mut u8, stride: usize) {
    const { assert!(TILE <= 7, "AMX tile number must be in 0..=7") };
    macro_rules! arm {
        ($number:literal) => {
            unsafe {
                asm!(
                    concat!("tilestored [{base} + {stride}], tmm", $number),
                    base = in(reg) base,
                    stride = in(reg) stride,
                    options(nostack),
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
}

macro_rules! mma_one {
    ($instruction:literal, $dst:literal, $a:literal, $b:literal) => {
        unsafe {
            asm!(
                concat!($instruction, " tmm", $dst, ", tmm", $a, ", tmm", $b),
                options(nostack),
            );
        }
    };
}

macro_rules! mma_ab {
    ($instruction:literal, $dst:literal) => {
        match (A, B) {
            (0, 0) => mma_one!($instruction, $dst, 0, 0),
            (0, 1) => mma_one!($instruction, $dst, 0, 1),
            (0, 2) => mma_one!($instruction, $dst, 0, 2),
            (0, 3) => mma_one!($instruction, $dst, 0, 3),
            (0, 4) => mma_one!($instruction, $dst, 0, 4),
            (0, 5) => mma_one!($instruction, $dst, 0, 5),
            (0, 6) => mma_one!($instruction, $dst, 0, 6),
            (0, 7) => mma_one!($instruction, $dst, 0, 7),
            (1, 0) => mma_one!($instruction, $dst, 1, 0),
            (1, 1) => mma_one!($instruction, $dst, 1, 1),
            (1, 2) => mma_one!($instruction, $dst, 1, 2),
            (1, 3) => mma_one!($instruction, $dst, 1, 3),
            (1, 4) => mma_one!($instruction, $dst, 1, 4),
            (1, 5) => mma_one!($instruction, $dst, 1, 5),
            (1, 6) => mma_one!($instruction, $dst, 1, 6),
            (1, 7) => mma_one!($instruction, $dst, 1, 7),
            (2, 0) => mma_one!($instruction, $dst, 2, 0),
            (2, 1) => mma_one!($instruction, $dst, 2, 1),
            (2, 2) => mma_one!($instruction, $dst, 2, 2),
            (2, 3) => mma_one!($instruction, $dst, 2, 3),
            (2, 4) => mma_one!($instruction, $dst, 2, 4),
            (2, 5) => mma_one!($instruction, $dst, 2, 5),
            (2, 6) => mma_one!($instruction, $dst, 2, 6),
            (2, 7) => mma_one!($instruction, $dst, 2, 7),
            (3, 0) => mma_one!($instruction, $dst, 3, 0),
            (3, 1) => mma_one!($instruction, $dst, 3, 1),
            (3, 2) => mma_one!($instruction, $dst, 3, 2),
            (3, 3) => mma_one!($instruction, $dst, 3, 3),
            (3, 4) => mma_one!($instruction, $dst, 3, 4),
            (3, 5) => mma_one!($instruction, $dst, 3, 5),
            (3, 6) => mma_one!($instruction, $dst, 3, 6),
            (3, 7) => mma_one!($instruction, $dst, 3, 7),
            (4, 0) => mma_one!($instruction, $dst, 4, 0),
            (4, 1) => mma_one!($instruction, $dst, 4, 1),
            (4, 2) => mma_one!($instruction, $dst, 4, 2),
            (4, 3) => mma_one!($instruction, $dst, 4, 3),
            (4, 4) => mma_one!($instruction, $dst, 4, 4),
            (4, 5) => mma_one!($instruction, $dst, 4, 5),
            (4, 6) => mma_one!($instruction, $dst, 4, 6),
            (4, 7) => mma_one!($instruction, $dst, 4, 7),
            (5, 0) => mma_one!($instruction, $dst, 5, 0),
            (5, 1) => mma_one!($instruction, $dst, 5, 1),
            (5, 2) => mma_one!($instruction, $dst, 5, 2),
            (5, 3) => mma_one!($instruction, $dst, 5, 3),
            (5, 4) => mma_one!($instruction, $dst, 5, 4),
            (5, 5) => mma_one!($instruction, $dst, 5, 5),
            (5, 6) => mma_one!($instruction, $dst, 5, 6),
            (5, 7) => mma_one!($instruction, $dst, 5, 7),
            (6, 0) => mma_one!($instruction, $dst, 6, 0),
            (6, 1) => mma_one!($instruction, $dst, 6, 1),
            (6, 2) => mma_one!($instruction, $dst, 6, 2),
            (6, 3) => mma_one!($instruction, $dst, 6, 3),
            (6, 4) => mma_one!($instruction, $dst, 6, 4),
            (6, 5) => mma_one!($instruction, $dst, 6, 5),
            (6, 6) => mma_one!($instruction, $dst, 6, 6),
            (6, 7) => mma_one!($instruction, $dst, 6, 7),
            (7, 0) => mma_one!($instruction, $dst, 7, 0),
            (7, 1) => mma_one!($instruction, $dst, 7, 1),
            (7, 2) => mma_one!($instruction, $dst, 7, 2),
            (7, 3) => mma_one!($instruction, $dst, 7, 3),
            (7, 4) => mma_one!($instruction, $dst, 7, 4),
            (7, 5) => mma_one!($instruction, $dst, 7, 5),
            (7, 6) => mma_one!($instruction, $dst, 7, 6),
            (7, 7) => mma_one!($instruction, $dst, 7, 7),
            _ => unreachable!(),
        }
    };
}

macro_rules! mma {
    ($name:ident, $instruction:literal, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must hold an [`super::AmxGuard`], enable the AMX extension
        /// that defines this instruction, and configure compatible tile shapes.
        /// `DST`, `A`, and `B` must be in `0..=7`; larger values are compile-time
        /// errors.
        #[inline]
        pub unsafe fn $name<const DST: u8, const A: u8, const B: u8>() {
            const {
                assert!(DST <= 7, "AMX tile number must be in 0..=7");
                assert!(A <= 7, "AMX tile number must be in 0..=7");
                assert!(B <= 7, "AMX tile number must be in 0..=7");
            }
            match DST {
                0 => mma_ab!($instruction, 0),
                1 => mma_ab!($instruction, 1),
                2 => mma_ab!($instruction, 2),
                3 => mma_ab!($instruction, 3),
                4 => mma_ab!($instruction, 4),
                5 => mma_ab!($instruction, 5),
                6 => mma_ab!($instruction, 6),
                7 => mma_ab!($instruction, 7),
                _ => unreachable!(),
            }
        }
    };
}

mma!(
    tdpbf16ps,
    "tdpbf16ps",
    "`tdpbf16ps::<DST, A, B>`\n\nMultiply bfloat16 tiles `A` and `B` and accumulate into `DST`. Requires AMX-BF16."
);
mma!(
    tdpbssd,
    "tdpbssd",
    "`tdpbssd::<DST, A, B>`\n\nMultiply signed-byte tiles `A` and `B` and accumulate into `DST`. Requires AMX-INT8."
);
mma!(
    tdpbsud,
    "tdpbsud",
    "`tdpbsud::<DST, A, B>`\n\nMultiply signed bytes by unsigned bytes and accumulate into `DST`. Requires AMX-INT8."
);
mma!(
    tdpbuud,
    "tdpbuud",
    "`tdpbuud::<DST, A, B>`\n\nMultiply unsigned-byte tiles `A` and `B` and accumulate into `DST`. Requires AMX-INT8."
);
mma!(
    tdpfp16ps,
    "tdpfp16ps",
    "`tdpfp16ps::<DST, A, B>`\n\nMultiply binary16 tiles `A` and `B` and accumulate into `DST`. Requires AMX-FP16."
);
mma!(
    tcmmimfp16ps,
    "tcmmimfp16ps",
    "`tcmmimfp16ps::<DST, A, B>`\n\nComplex binary16 matrix multiply with imaginary parts. Requires AMX-COMPLEX."
);
mma!(
    tcmmrlfp16ps,
    "tcmmrlfp16ps",
    "`tcmmrlfp16ps::<DST, A, B>`\n\nComplex binary16 matrix multiply with real parts. Requires AMX-COMPLEX."
);
