//! AMX-MOVRS tile loads.
//!
//! AMX-MOVRS adds tile loads with a read-shared memory hint. The wrappers have
//! the same shape as [`tileloadd`](super::amx_tile::tileloadd) and
//! [`tileloaddt1`](super::amx_tile::tileloaddt1), and the caller must hold an
//! [`AmxGuard`].
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::matrix::amx_movrs::tileloaddrs as _;
//! use rust_isa::x86::matrix::amx_movrs::tileloaddrst1 as _;
//! ```
//! <!-- doctest-inventory:end -->

use core::arch::asm;

use super::amx_tile::AmxGuard;

macro_rules! tile_load_movrs {
    ($name:ident, $instruction:literal, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must hold an [`AmxGuard`],
        /// enable AMX-MOVRS, and pass a valid base pointer and row stride.
        /// `TILE` must be in `0..=7`; a larger value is a compile-time error.
        #[inline]
        pub unsafe fn $name<const TILE: u8>(_guard: &AmxGuard, base: *const u8, stride: usize) {
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

tile_load_movrs!(
    tileloaddrs,
    "tileloaddrs",
    "`tileloaddrs::<TILE>`\n\nLoad `TILE` from `base` with row stride `stride` using a read-shared memory hint."
);
tile_load_movrs!(
    tileloaddrst1,
    "tileloaddrst1",
    "`tileloaddrst1::<TILE>`\n\nLoad `TILE` from `base` with row stride `stride` using a read-shared memory hint and the temporal hint that the data is not reused."
);
