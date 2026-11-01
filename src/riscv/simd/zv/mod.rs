//! RISC-V vector cryptography extensions.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The module is split into Zvkn, Zvks, Zvkg, and Zvbc.
//!
//! - `zvkn`: Zvkn (AES, SHA-2). `vaes*`, `vsha2*`.
//! - `zvks`: Zvks (SM3, SM4). `vsm3*`, `vsm4*`.
//! - `zvkg`: Zvkg (GCM). `vghsh`, `vgmul`.
//! - `zvbc`: Zvbc (carryless multiply). `vclmul`, `vclmulh`.
//! - `zvbb`: Zvbb (bit manipulation). `vandn`, `vbrev`, `vclz`, `vrot`, `vwsll`.
//!
//! Enable the target feature named by the submodule: `zvkn`, `zvks`, `zvkg`,
//! or `zvbc`. Key-schedule functions take a runtime round number; the valid
//! range is documented on each function.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::riscv::simd::rvv::V128;
//! use rust_isa::riscv::simd::zv::vgmul;
//!
//! let acc = V128::from_bytes([0; 16]);
//! let key = V128::from_bytes([0; 16]);
//! let _ = unsafe { vgmul(acc, key) };
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::riscv::simd::zv::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { vgmul(v, v) }.to_bytes(), [0; 16]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::riscv::simd::zv::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//!
//! let _ = unsafe { vaesdf(v128, v128) };
//! let _ = unsafe { vaesdm(v128, v128) };
//! let _ = unsafe { vaesef(v128, v128) };
//! let _ = unsafe { vaesem(v128, v128) };
//! let _ = unsafe { vaeskf1(v128, 0) };
//! let _ = unsafe { vaeskf2(v128, 0) };
//! let _ = unsafe { vaesz(v128, v128) };
//! let _ = unsafe { vclmul(v128, v128) };
//! let _ = unsafe { vclmulh(v128, v128) };
//! let _ = unsafe { vghsh(v128, v128, v128) };
//! let _ = unsafe { vgmul(v128, v128) };
//! let _ = unsafe { vsha2ch(v128, v128, v128) };
//! let _ = unsafe { vsha2cl(v128, v128, v128) };
//! let _ = unsafe { vsha2ms(v128, v128, v128) };
//! let _ = unsafe { vsm3c([v128; 2], 0) };
//! let _ = unsafe { vsm3me([v128; 2], [v128; 2]) };
//! let _ = unsafe { vsm4k(v128, 0) };
//! let _ = unsafe { vsm4r(v128, v128) };
//! let _ = unsafe { vandn_vv(v128, v128) };
//! let _ = unsafe { vandn_vx(v128, 1) };
//! let _ = unsafe { vbrev(v128) };
//! let _ = unsafe { vbrev8(v128) };
//! let _ = unsafe { vrev8(v128) };
//! let _ = unsafe { vclz(v128) };
//! let _ = unsafe { vctz(v128) };
//! let _ = unsafe { vcpop(v128) };
//! let _ = unsafe { vrol_vv(v128, v128) };
//! let _ = unsafe { vrol_vx(v128, 1) };
//! let _ = unsafe { vror_vv(v128, v128) };
//! let _ = unsafe { vror_vx(v128, 1) };
//! let _ = unsafe { vror_vi::<1>(v128) };
//! let _ = unsafe { vwsll_vv(v128, v128) };
//! let _ = unsafe { vwsll_vx(v128, 1) };
//! let _ = unsafe { vwsll_vi::<1>(v128) };
//! ```
//!
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
//! use rust_isa::riscv::simd::zv::vandn_vv as _;
//! use rust_isa::riscv::simd::zv::vandn_vx as _;
//! use rust_isa::riscv::simd::zv::vbrev as _;
//! use rust_isa::riscv::simd::zv::vbrev8 as _;
//! use rust_isa::riscv::simd::zv::vclmul as _;
//! use rust_isa::riscv::simd::zv::vclmulh as _;
//! use rust_isa::riscv::simd::zv::vclz as _;
//! use rust_isa::riscv::simd::zv::vcpop as _;
//! use rust_isa::riscv::simd::zv::vctz as _;
//! use rust_isa::riscv::simd::zv::vghsh as _;
//! use rust_isa::riscv::simd::zv::vgmul as _;
//! use rust_isa::riscv::simd::zv::vrev8 as _;
//! use rust_isa::riscv::simd::zv::vrol_vv as _;
//! use rust_isa::riscv::simd::zv::vrol_vx as _;
//! use rust_isa::riscv::simd::zv::vror_vi as _;
//! use rust_isa::riscv::simd::zv::vror_vv as _;
//! use rust_isa::riscv::simd::zv::vror_vx as _;
//! use rust_isa::riscv::simd::zv::vsha2ch as _;
//! use rust_isa::riscv::simd::zv::vsha2cl as _;
//! use rust_isa::riscv::simd::zv::vsha2ms as _;
//! use rust_isa::riscv::simd::zv::vsm3c as _;
//! use rust_isa::riscv::simd::zv::vsm3me as _;
//! use rust_isa::riscv::simd::zv::vsm4k as _;
//! use rust_isa::riscv::simd::zv::vsm4r as _;
//! use rust_isa::riscv::simd::zv::vwsll_vi as _;
//! use rust_isa::riscv::simd::zv::vwsll_vv as _;
//! use rust_isa::riscv::simd::zv::vwsll_vx as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

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

mod zvbb;
mod zvbc;
mod zvkg;
mod zvkn;
mod zvks;

pub use zvbb::*;
pub use zvbc::*;
pub use zvkg::*;
pub use zvkn::*;
pub use zvks::*;
