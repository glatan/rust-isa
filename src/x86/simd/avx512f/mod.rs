//! AVX-512 Foundation.
//!
//! AVX-512 is not part of the x86_64 baseline. Every instruction wrapper is `unsafe`
//! and the caller must enable the required target features. The values are
//! [`V512`], [`V256`](super::avx2::V256), and [`V128`](super::sse2::V128),
//! depending on the operation. Immediate operands are const generics unless
//! documented otherwise.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

mod float;
mod gather_scatter;
mod integer;
mod v512;

pub use float::*;
pub use gather_scatter::*;
pub use integer::*;
pub use v512::*;
