//! RISC-V vector cryptography extensions.
//!
//! The module is split into Zvkn, Zvks, Zvkg, and Zvbc.
//!
//! - `zvkn`: Zvkn (AES, SHA-2). `vaes*`, `vsha2*`.
//! - `zvks`: Zvks (SM3, SM4). `vsm3*`, `vsm4*`.
//! - `zvkg`: Zvkg (GCM). `vghsh`, `vgmul`.
//! - `zvbc`: Zvbc (carryless multiply). `vclmul`, `vclmulh`.
//!
//! Enable the target feature named by the submodule: `zvkn`, `zvks`, `zvkg`,
//! or `zvbc`. Key-schedule functions take a runtime round number; the valid
//! range is documented on each function.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::simd::zv::vaesdf as _;
//! use rust_isa::riscv::simd::zv::vaesdm as _;
//! use rust_isa::riscv::simd::zv::vaesef as _;
//! use rust_isa::riscv::simd::zv::vaesem as _;
//! use rust_isa::riscv::simd::zv::vaeskf1 as _;
//! use rust_isa::riscv::simd::zv::vaeskf2 as _;
//! use rust_isa::riscv::simd::zv::vaesz as _;
//! use rust_isa::riscv::simd::zv::vclmul as _;
//! use rust_isa::riscv::simd::zv::vclmulh as _;
//! use rust_isa::riscv::simd::zv::vghsh as _;
//! use rust_isa::riscv::simd::zv::vgmul as _;
//! use rust_isa::riscv::simd::zv::vsha2ch as _;
//! use rust_isa::riscv::simd::zv::vsha2cl as _;
//! use rust_isa::riscv::simd::zv::vsha2ms as _;
//! use rust_isa::riscv::simd::zv::vsm3c as _;
//! use rust_isa::riscv::simd::zv::vsm3me as _;
//! use rust_isa::riscv::simd::zv::vsm4k as _;
//! use rust_isa::riscv::simd::zv::vsm4r as _;
//! ```
//! <!-- doctest-inventory:end -->

macro_rules! crypto_binary {
    ($(#[$meta:meta])* $name:ident, $c:ident, $feature:literal) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        #[doc = concat!("The caller must compile with the `", $feature, "` target feature enabled.")]
        #[inline(always)]
        pub unsafe fn $name(state: V128, key: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(state.as_ptr(), key.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! crypto_key_schedule {
    ($(#[$meta:meta])* $name:ident, $c:ident, $feature:literal) => {
        $(#[$meta])*
        ///
        /// `round` must be at most the largest immediate accepted by the
        /// instruction.
        ///
        /// # Safety
        ///
        #[doc = concat!("The caller must compile with the `", $feature, "` target feature enabled and pass a valid round number.")]
        #[inline(always)]
        pub unsafe fn $name(key: V128, round: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(key.as_ptr(), round, out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! crypto_ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident, $feature:literal) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        #[doc = concat!("The caller must compile with the `", $feature, "` target feature enabled.")]
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

pub(crate) use crypto_binary;
pub(crate) use crypto_key_schedule;
pub(crate) use crypto_ternary;

mod zvbc;
mod zvkg;
mod zvkn;
mod zvks;

pub use zvbc::*;
pub use zvkg::*;
pub use zvkn::*;
pub use zvks::*;
