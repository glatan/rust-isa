//! AMX tile state.
//!
//! AMX is not part of the x86-64 baseline and rustc does not allow
//! `#[target_feature(enable = "amx-tile")]` on stable. The caller must compile
//! the whole crate with the AMX features enabled, and the CPU and operating
//! system must allow AMX tile state.
//!
//! [`TileConfig`] is the 64-byte tile configuration consumed by
//! [`AmxGuard::enter`]. The caller builds it; this crate does not interpret it.
//! The guard and instruction wrappers are `unsafe`. Tile numbers are const
//! generics, and the tile configuration is caller-provided state.
//! [`AmxGuard`] releases the tile state in its `Drop` implementation and is not
//! `Send` or `Sync`. Nested guards are not allowed.
//!
//! The module contains the guard, the tile configuration, and the tile
//! load/store and matrix-multiply wrappers. Instructions that read or write
//! tile state take `&AmxGuard`.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.

mod config;
mod mma;
mod tile;

pub use config::*;
pub use mma::*;
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
