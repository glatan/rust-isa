//! AMX-BF16 matrix-multiply instructions.
//!
//! AMX-BF16 is not part of the x86-64 baseline. The caller must compile the
//! whole crate with `-C target-feature=+amx-bf16` and the CPU and operating
//! system must allow AMX tile state. The guard is shared with
//! [`amx_tile::AmxGuard`](super::amx_tile::AmxGuard).
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software
//! Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! Instructions not listed in this module are not exposed.

//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::matrix::amx_bf16::tdpbf16ps;
//! use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig};
//!
//! let config = TileConfig::builder().tile::<0>(16, 64).build();
//! let guard = unsafe { AmxGuard::enter(&config) };
//! let _ = unsafe { tdpbf16ps::<0, 0, 0>(&guard) };
//! ```

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::matrix::amx_bf16::tdpbf16ps as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::amx_common::{define_mma, mma_ab, mma_one};
use super::amx_tile::AmxGuard;
use core::arch::asm;

define_mma!(
    tdpbf16ps,
    "tdpbf16ps",
    "`tdpbf16ps::<DST, A, B>`\n\nMultiply bfloat16 tiles `A` and `B` and accumulate into `DST`."
);
