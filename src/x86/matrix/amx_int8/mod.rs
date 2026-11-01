//! AMX-INT8 matrix-multiply instructions.
//!
//! AMX-INT8 is not part of the x86-64 baseline. The caller must compile the
//! whole crate with `-C target-feature=+amx-int8` and the CPU and operating
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
//! use rust_isa::x86::matrix::amx_int8::{tdpbssd, tdpbsud, tdpbuud};
//! use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig};
//!
//! let config = TileConfig::builder().tile::<0>(16, 64).build();
//! let guard = unsafe { AmxGuard::enter(&config) };
//! let _ = unsafe { tdpbssd::<0, 0, 0>(&guard) };
//! let _ = unsafe { tdpbsud::<0, 0, 0>(&guard) };
//! let _ = unsafe { tdpbuud::<0, 0, 0>(&guard) };
//! ```

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::matrix::amx_int8::tdpbssd as _;
//! use rust_isa::x86::matrix::amx_int8::tdpbsud as _;
//! use rust_isa::x86::matrix::amx_int8::tdpbuud as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::amx_common::{define_mma, mma_ab, mma_one};
use super::amx_tile::AmxGuard;
use core::arch::asm;

define_mma!(
    tdpbssd,
    "tdpbssd",
    "`tdpbssd::<DST, A, B>`\n\nMultiply signed-byte tiles `A` and `B` and accumulate into `DST`."
);
define_mma!(
    tdpbsud,
    "tdpbsud",
    "`tdpbsud::<DST, A, B>`\n\nMultiply signed bytes by unsigned bytes and accumulate into `DST`."
);
define_mma!(
    tdpbuud,
    "tdpbuud",
    "`tdpbuud::<DST, A, B>`\n\nMultiply unsigned-byte tiles `A` and `B` and accumulate into `DST`."
);
