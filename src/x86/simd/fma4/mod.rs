//! FMA4.
//!
//! FMA4 is an AMD extension and is not part of the x86-64 baseline. rustc does
//! not allow `#[target_feature(enable = "fma4")]` on stable, so the caller must
//! compile the whole crate with the feature enabled and the CPU must support
//! FMA4. The wrappers do not carry `#[target_feature]`. Every wrapper is
//! `unsafe`.
//!
//! The values are [`V128`]. The four-operand form is exposed as three operands
//! because the destination is also the first source in this API.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::fma4::vfmaddps;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vfmaddps(v, v, v) };
//! ```
//!
//! # Instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::fma4::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! unsafe {
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddpd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddps(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsubpd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsubps(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubaddpd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubaddps(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubpd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubps(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubsd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmaddpd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmaddps(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmaddsd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmaddss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsubpd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsubps(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsubsd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsubss(v128, v128, v128) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::fma4::vfmaddpd as _;
//! use rust_isa::x86::simd::fma4::vfmaddps as _;
//! use rust_isa::x86::simd::fma4::vfmaddsd as _;
//! use rust_isa::x86::simd::fma4::vfmaddss as _;
//! use rust_isa::x86::simd::fma4::vfmaddsubpd as _;
//! use rust_isa::x86::simd::fma4::vfmaddsubps as _;
//! use rust_isa::x86::simd::fma4::vfmsubaddpd as _;
//! use rust_isa::x86::simd::fma4::vfmsubaddps as _;
//! use rust_isa::x86::simd::fma4::vfmsubpd as _;
//! use rust_isa::x86::simd::fma4::vfmsubps as _;
//! use rust_isa::x86::simd::fma4::vfmsubsd as _;
//! use rust_isa::x86::simd::fma4::vfmsubss as _;
//! use rust_isa::x86::simd::fma4::vfnmaddpd as _;
//! use rust_isa::x86::simd::fma4::vfnmaddps as _;
//! use rust_isa::x86::simd::fma4::vfnmaddsd as _;
//! use rust_isa::x86::simd::fma4::vfnmaddss as _;
//! use rust_isa::x86::simd::fma4::vfnmsubpd as _;
//! use rust_isa::x86::simd::fma4::vfnmsubps as _;
//! use rust_isa::x86::simd::fma4::vfnmsubsd as _;
//! use rust_isa::x86::simd::fma4::vfnmsubss as _;
//! ```
//! <!-- doctest-inventory:end -->

use super::sse2::V128;

unsafe extern "C" {
    fn rust_isa_fma4_vfmaddps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmaddpd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmaddss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmaddsd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmsubps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmsubpd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmsubss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmsubsd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfnmaddps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfnmaddpd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfnmaddss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfnmaddsd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfnmsubps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfnmsubpd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfnmsubss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfnmsubsd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmaddsubps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmaddsubpd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmsubaddps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma4_vfmsubaddpd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

macro_rules! fma4 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with FMA4 enabled and the CPU must support it.
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

fma4!(
    /// `vfmaddps`
    ///
    /// Compute `a * b + c` with a single rounding on 32-bit float lanes.
    vfmaddps,
    rust_isa_fma4_vfmaddps
);
fma4!(
    /// `vfmaddpd`
    ///
    /// Compute `a * b + c` with a single rounding on 64-bit float lanes.
    vfmaddpd,
    rust_isa_fma4_vfmaddpd
);
fma4!(
    /// `vfmaddss`
    ///
    /// Compute `a * b + c` with a single rounding on the low 32-bit float lane.
    vfmaddss,
    rust_isa_fma4_vfmaddss
);
fma4!(
    /// `vfmaddsd`
    ///
    /// Compute `a * b + c` with a single rounding on the low 64-bit float lane.
    vfmaddsd,
    rust_isa_fma4_vfmaddsd
);
fma4!(
    /// `vfmsubps`
    ///
    /// Compute `a * b - c` with a single rounding on 32-bit float lanes.
    vfmsubps,
    rust_isa_fma4_vfmsubps
);
fma4!(
    /// `vfmsubpd`
    ///
    /// Compute `a * b - c` with a single rounding on 64-bit float lanes.
    vfmsubpd,
    rust_isa_fma4_vfmsubpd
);
fma4!(
    /// `vfmsubss`
    ///
    /// Compute `a * b - c` with a single rounding on the low 32-bit float lane.
    vfmsubss,
    rust_isa_fma4_vfmsubss
);
fma4!(
    /// `vfmsubsd`
    ///
    /// Compute `a * b - c` with a single rounding on the low 64-bit float lane.
    vfmsubsd,
    rust_isa_fma4_vfmsubsd
);
fma4!(
    /// `vfnmaddps`
    ///
    /// Compute `-(a * b) + c` with a single rounding on 32-bit float lanes.
    vfnmaddps,
    rust_isa_fma4_vfnmaddps
);
fma4!(
    /// `vfnmaddpd`
    ///
    /// Compute `-(a * b) + c` with a single rounding on 64-bit float lanes.
    vfnmaddpd,
    rust_isa_fma4_vfnmaddpd
);
fma4!(
    /// `vfnmaddss`
    ///
    /// Compute `-(a * b) + c` with a single rounding on the low 32-bit float lane.
    vfnmaddss,
    rust_isa_fma4_vfnmaddss
);
fma4!(
    /// `vfnmaddsd`
    ///
    /// Compute `-(a * b) + c` with a single rounding on the low 64-bit float lane.
    vfnmaddsd,
    rust_isa_fma4_vfnmaddsd
);
fma4!(
    /// `vfnmsubps`
    ///
    /// Compute `-(a * b) - c` with a single rounding on 32-bit float lanes.
    vfnmsubps,
    rust_isa_fma4_vfnmsubps
);
fma4!(
    /// `vfnmsubpd`
    ///
    /// Compute `-(a * b) - c` with a single rounding on 64-bit float lanes.
    vfnmsubpd,
    rust_isa_fma4_vfnmsubpd
);
fma4!(
    /// `vfnmsubss`
    ///
    /// Compute `-(a * b) - c` with a single rounding on the low 32-bit float lane.
    vfnmsubss,
    rust_isa_fma4_vfnmsubss
);
fma4!(
    /// `vfnmsubsd`
    ///
    /// Compute `-(a * b) - c` with a single rounding on the low 64-bit float lane.
    vfnmsubsd,
    rust_isa_fma4_vfnmsubsd
);
fma4!(
    /// `vfmaddsubps`
    ///
    /// Compute alternating add and subtract of `c` after multiplying 32-bit
    /// float lanes.
    vfmaddsubps,
    rust_isa_fma4_vfmaddsubps
);
fma4!(
    /// `vfmaddsubpd`
    ///
    /// Compute alternating add and subtract of `c` after multiplying 64-bit
    /// float lanes.
    vfmaddsubpd,
    rust_isa_fma4_vfmaddsubpd
);
fma4!(
    /// `vfmsubaddps`
    ///
    /// Compute alternating subtract and add of `c` after multiplying 32-bit
    /// float lanes.
    vfmsubaddps,
    rust_isa_fma4_vfmsubaddps
);
fma4!(
    /// `vfmsubaddpd`
    ///
    /// Compute alternating subtract and add of `c` after multiplying 64-bit
    /// float lanes.
    vfmsubaddpd,
    rust_isa_fma4_vfmsubaddpd
);
