//! AMX-FP8 matrix-multiply instructions.
//!
//! AMX-FP8 is not part of the x86-64 baseline. The caller must compile the
//! whole crate with `-C target-feature=+amx-fp8` and the CPU and operating
//! system must allow AMX tile state. The guard is shared with
//! [`amx_tile::AmxGuard`](super::amx_tile::AmxGuard).
//!
//! These wrappers use inline assembly because rustc does not expose the AMX
//! tile number as a value type. Matrix-multiply results are accumulated into
//! the destination tile in the AMX tile state.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! Instructions not listed in this module are not exposed.

//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::matrix::amx_fp8::*;
//! use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig};
//!
//! let config = TileConfig::builder().tile::<0>(1, 64).build();
//! let guard = unsafe { AmxGuard::enter(&config) };
//! let _ = unsafe { tdpbf8ps::<0, 0, 0>(&guard) };
//! let _ = unsafe { tdpbhf8ps::<0, 0, 0>(&guard) };
//! let _ = unsafe { tdphbf8ps::<0, 0, 0>(&guard) };
//! let _ = unsafe { tdphf8ps::<0, 0, 0>(&guard) };
//! ```

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::matrix::amx_fp8::tdpbf8ps as _;
//! use rust_isa::x86::matrix::amx_fp8::tdpbhf8ps as _;
//! use rust_isa::x86::matrix::amx_fp8::tdphbf8ps as _;
//! use rust_isa::x86::matrix::amx_fp8::tdphf8ps as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::amx_common::{define_mma, mma_ab, mma_one};
use super::amx_tile::AmxGuard;
use core::arch::asm;

define_mma!(
    tdpbf8ps,
    "tdpbf8ps",
    "`tdpbf8ps::<DST, A, B>`\n\nMultiply BF8 tiles `A` and `B` and accumulate into `DST`. Requires AMX-FP8."
);
define_mma!(
    tdpbhf8ps,
    "tdpbhf8ps",
    "`tdpbhf8ps::<DST, A, B>`\n\nMultiply 4-bit BF8 by 4-bit HF8 tiles and accumulate into `DST`. Requires AMX-FP8."
);
define_mma!(
    tdphbf8ps,
    "tdphbf8ps",
    "`tdphbf8ps::<DST, A, B>`\n\nMultiply 4-bit HF8 by 4-bit BF8 tiles and accumulate into `DST`. Requires AMX-FP8."
);
define_mma!(
    tdphf8ps,
    "tdphf8ps",
    "`tdphf8ps::<DST, A, B>`\n\nMultiply HF8 tiles `A` and `B` and accumulate into `DST`. Requires AMX-FP8."
);
