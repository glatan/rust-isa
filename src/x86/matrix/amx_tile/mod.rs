//! AMX tile state.
//!
//! AMX is not part of the x86-64 baseline and rustc does not allow
//! `#[target_feature(enable = "amx-tile")]` on stable. The caller must compile
//! the whole crate with the AMX features enabled, and the CPU and operating
//! system must allow AMX tile state.
//!
//! [`TileConfig`] is the 64-byte tile configuration consumed by
//! [`AmxGuard::enter`]. [`TileConfig::builder`] configures palette 1 and its
//! eight tiles; the raw [`TileConfig::from_bytes`] constructor remains
//! available. The guard and instruction wrappers are `unsafe`. Tile numbers
//! are const generics, and the tile configuration is caller-provided state.
//! [`AmxGuard`] releases the tile state in its `Drop` implementation and is not
//! `Send` or `Sync`. Nested guards are not allowed.
//!
//! The module contains the guard, the tile configuration, and the tile
//! load/store and matrix-multiply wrappers. Instructions that read or write
//! tile state take `&AmxGuard`.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! Instructions not listed in this module are not exposed.
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.

//! # Examples
//!
//! ```
//! use rust_isa::x86::matrix::amx_tile::TileConfig;
//!
//! let config = TileConfig::builder().tile::<0>(1, 64).build();
//! assert_eq!(config.to_bytes()[48], 1);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::matrix::amx_tile::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//! use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig};
//! let mut config = TileConfig::builder().tile::<0>(1, 64).build();
//! let amx_guard = unsafe { AmxGuard::enter(&config) };
//!
//! let _ = unsafe { ldtilecfg(&config) };
//! let _ = unsafe { sttilecfg(&amx_guard, &mut config) };
//! let _ = unsafe { tileloadd::<0>(&amx_guard, storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { tileloaddt1::<0>(&amx_guard, storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { tilestored::<0>(&amx_guard, storage.0.as_mut_ptr().cast(), 0) };
//! let _ = unsafe { tilezero::<0>(&amx_guard) };
//! let _ = unsafe { AmxGuard::enter(&config) };
//! let _ = TileConfig::builder();
//! let _ = TileConfig::from_bytes([0; 64]);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::matrix::amx_tile::ldtilecfg as _;
//! use rust_isa::x86::matrix::amx_tile::sttilecfg as _;
//! use rust_isa::x86::matrix::amx_tile::tileloadd as _;
//! use rust_isa::x86::matrix::amx_tile::tileloaddt1 as _;
//! use rust_isa::x86::matrix::amx_tile::tilestored as _;
//! use rust_isa::x86::matrix::amx_tile::tilezero as _;
//! use rust_isa::x86::matrix::amx_tile::AmxGuard;
//! use rust_isa::x86::matrix::amx_tile::TileConfig;
//! let _ = AmxGuard::enter;
//! let _ = TileConfig::builder;
//! let _ = TileConfig::from_bytes;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

mod config;
mod tile;

pub use config::*;
pub use tile::*;

use core::marker::PhantomData;

/// The 64-byte tile configuration used by `ldtilecfg`.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TileConfig([u8; 64]);

impl TileConfig {
    /// Keep the 64 bytes unchanged.
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Return the 64 bytes.
    pub const fn to_bytes(self) -> [u8; 64] {
        self.0
    }

    /// Pointer to the first byte. The alignment is 64.
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Mutable pointer to the first byte. The alignment is 64.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    /// Start building a tile configuration with palette 1.
    pub const fn builder() -> TileConfigBuilder {
        let mut bytes = [0u8; 64];
        bytes[0] = 1;
        TileConfigBuilder {
            config: TileConfig(bytes),
        }
    }
}

/// A builder for the 64-byte AMX tile configuration.
///
/// Palette 1 provides eight tiles. A tile is configured with its number of
/// rows and bytes per row. Tiles that are not configured stay zero and are
/// inactive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TileConfigBuilder {
    config: TileConfig,
}

impl TileConfigBuilder {
    /// Set the restart row. Use 0 for a new configuration.
    pub const fn start_row(mut self, start_row: u8) -> Self {
        self.config.0[1] = start_row;
        self
    }

    /// Configure `TILE` with `rows` rows and `colsb` bytes per row.
    ///
    /// # Panics
    ///
    /// Panics if `TILE` is not in `0..8`, `rows` is not in `1..=16`, or
    /// `colsb` is not in `1..=64`.
    pub const fn tile<const TILE: usize>(mut self, rows: u8, colsb: u16) -> Self {
        assert!(TILE < 8, "AMX tile index must be in 0..8");
        assert!(rows >= 1 && rows <= 16, "AMX rows must be in 1..=16");
        assert!(colsb >= 1 && colsb <= 64, "AMX colsb must be in 1..=64");
        let offset = 16 + TILE * 2;
        self.config.0[offset] = colsb as u8;
        self.config.0[offset + 1] = (colsb >> 8) as u8;
        self.config.0[48 + TILE] = rows;
        self
    }

    /// Finish the configuration.
    pub const fn build(self) -> TileConfig {
        self.config
    }
}

unsafe extern "C" {
    fn rust_isa_amx_tile_load_config(config: *const u8);
    fn rust_isa_amx_tile_release();
}

/// An active AMX tile state.
///
/// The guard is created by [`AmxGuard::enter`] and releases the tile state when
/// dropped. It is neither `Send` nor `Sync`.
pub struct AmxGuard {
    _not_send_or_sync: PhantomData<*const ()>,
}

impl AmxGuard {
    /// Load `config` and enter the AMX tile state.
    ///
    /// # Safety
    ///
    /// The CPU must support AMX tile state, the operating system must allow it,
    /// and `config` must describe a valid tile configuration. The caller must
    /// not create a nested guard.
    pub unsafe fn enter(config: &TileConfig) -> Self {
        unsafe {
            rust_isa_amx_tile_load_config(config.0.as_ptr());
        }
        Self {
            _not_send_or_sync: PhantomData,
        }
    }
}

impl Drop for AmxGuard {
    fn drop(&mut self) {
        unsafe {
            rust_isa_amx_tile_release();
        }
    }
}
