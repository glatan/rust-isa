//! AMX tile load, store, and zero instructions.
//!
//! These wrappers use inline assembly because rustc does not expose the AMX
//! tile number as a value type.
//!
//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::x86::matrix::amx_tile::{tilezero, AmxGuard, TileConfig};
//!
//! let mut config = TileConfig::builder().tile::<0>(1, 64).build();
//! let guard = unsafe { AmxGuard::enter(&config) };
//! let _ = unsafe { tilezero::<8>(&guard) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::matrix::amx_tile::{tilestored, AmxGuard, TileConfig};
//!
//! let mut storage = [0u8; 64];
//! let mut config = TileConfig::builder().tile::<0>(1, 64).build();
//! let guard = unsafe { AmxGuard::enter(&config) };
//! let _ = unsafe { tilestored::<8>(&guard, storage.as_mut_ptr(), 64) };
//! ```

use core::arch::asm;

use super::AmxGuard;

unsafe extern "C" {
    fn rust_isa_amx_tile_zero_0();
    fn rust_isa_amx_tile_zero_1();
    fn rust_isa_amx_tile_zero_2();
    fn rust_isa_amx_tile_zero_3();
    fn rust_isa_amx_tile_zero_4();
    fn rust_isa_amx_tile_zero_5();
    fn rust_isa_amx_tile_zero_6();
    fn rust_isa_amx_tile_zero_7();
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
        pub unsafe fn $name<const TILE: u8>(
            _guard: &super::AmxGuard,
            base: $pointer,
            stride: usize,
        ) {
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
pub unsafe fn tilestored<const TILE: u8>(_guard: &super::AmxGuard, base: *mut u8, stride: usize) {
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

/// `tilezero::<TILE>`
///
/// Set all bytes of the selected tile to zero. `TILE` must be in `0..=7`; a
/// larger value is a compile-time error.
///
/// # Safety
///
/// The caller must hold an [`AmxGuard`] and enable AMX.
pub unsafe fn tilezero<const TILE: u8>(_guard: &AmxGuard) {
    const { assert!(TILE <= 7, "AMX tile number must be in 0..=7") };
    macro_rules! call {
        ($f:ident) => {
            unsafe { $f() }
        };
    }
    match TILE {
        0 => call!(rust_isa_amx_tile_zero_0),
        1 => call!(rust_isa_amx_tile_zero_1),
        2 => call!(rust_isa_amx_tile_zero_2),
        3 => call!(rust_isa_amx_tile_zero_3),
        4 => call!(rust_isa_amx_tile_zero_4),
        5 => call!(rust_isa_amx_tile_zero_5),
        6 => call!(rust_isa_amx_tile_zero_6),
        7 => call!(rust_isa_amx_tile_zero_7),
        _ => unreachable!(),
    }
}
