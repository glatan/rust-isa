//! AMX tile configuration instructions.
//!
//! These wrappers use inline assembly because rustc does not expose the AMX
//! tile number as a value type.

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
pub unsafe fn sttilecfg(_guard: &super::AmxGuard, config: &mut TileConfig) {
    unsafe {
        asm!(
            "sttilecfg [{config}]",
            config = in(reg) config.as_mut_ptr(),
            options(nostack),
        );
    }
}
