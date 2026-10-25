//! FMA.
//!
//! Every instruction wrapper is `unsafe` and the caller must enable AVX2 and FMA.
//! Packed operations use [`V256`]; scalar `*ss` and `*sd` operations use
//! [`V128`] and only define the low lane. The upper lanes of a scalar result
//! are copied from `a`.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::simd::fma::vfmadd132ps;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! assert_eq!(unsafe { vfmadd132ps(v, v, v) }.to_bytes(), [0; 32]);
//! ```
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::simd::fma::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//!
//! unsafe {
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd132pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd132ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd132sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd132ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd213pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd213ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd213sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd213ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd231pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd231ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd231sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmadd231ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsub132pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsub132ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsub213pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsub213ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsub231pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmaddsub231ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub132pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub132ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub132sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub132ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub213pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub213ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub213sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub213ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub231pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub231ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub231sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsub231ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubadd132pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubadd132ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubadd213pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubadd213ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubadd231pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfmsubadd231ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd132pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd132ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd132sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd132ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd213pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd213ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd213sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd213ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd231pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd231ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd231sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmadd231ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub132pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub132ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub132sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub132ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub213pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub213ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub213sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub213ss(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub231pd(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub231ps(v256, v256, v256) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub231sd(v128, v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { vfnmsub231ss(v128, v128, v128) };
//! }
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::fma::vfmadd132pd as _;
//! use rust_isa::x86::simd::fma::vfmadd132ps as _;
//! use rust_isa::x86::simd::fma::vfmadd132sd as _;
//! use rust_isa::x86::simd::fma::vfmadd132ss as _;
//! use rust_isa::x86::simd::fma::vfmadd213pd as _;
//! use rust_isa::x86::simd::fma::vfmadd213ps as _;
//! use rust_isa::x86::simd::fma::vfmadd213sd as _;
//! use rust_isa::x86::simd::fma::vfmadd213ss as _;
//! use rust_isa::x86::simd::fma::vfmadd231pd as _;
//! use rust_isa::x86::simd::fma::vfmadd231ps as _;
//! use rust_isa::x86::simd::fma::vfmadd231sd as _;
//! use rust_isa::x86::simd::fma::vfmadd231ss as _;
//! use rust_isa::x86::simd::fma::vfmaddsub132pd as _;
//! use rust_isa::x86::simd::fma::vfmaddsub132ps as _;
//! use rust_isa::x86::simd::fma::vfmaddsub213pd as _;
//! use rust_isa::x86::simd::fma::vfmaddsub213ps as _;
//! use rust_isa::x86::simd::fma::vfmaddsub231pd as _;
//! use rust_isa::x86::simd::fma::vfmaddsub231ps as _;
//! use rust_isa::x86::simd::fma::vfmsub132pd as _;
//! use rust_isa::x86::simd::fma::vfmsub132ps as _;
//! use rust_isa::x86::simd::fma::vfmsub132sd as _;
//! use rust_isa::x86::simd::fma::vfmsub132ss as _;
//! use rust_isa::x86::simd::fma::vfmsub213pd as _;
//! use rust_isa::x86::simd::fma::vfmsub213ps as _;
//! use rust_isa::x86::simd::fma::vfmsub213sd as _;
//! use rust_isa::x86::simd::fma::vfmsub213ss as _;
//! use rust_isa::x86::simd::fma::vfmsub231pd as _;
//! use rust_isa::x86::simd::fma::vfmsub231ps as _;
//! use rust_isa::x86::simd::fma::vfmsub231sd as _;
//! use rust_isa::x86::simd::fma::vfmsub231ss as _;
//! use rust_isa::x86::simd::fma::vfmsubadd132pd as _;
//! use rust_isa::x86::simd::fma::vfmsubadd132ps as _;
//! use rust_isa::x86::simd::fma::vfmsubadd213pd as _;
//! use rust_isa::x86::simd::fma::vfmsubadd213ps as _;
//! use rust_isa::x86::simd::fma::vfmsubadd231pd as _;
//! use rust_isa::x86::simd::fma::vfmsubadd231ps as _;
//! use rust_isa::x86::simd::fma::vfnmadd132pd as _;
//! use rust_isa::x86::simd::fma::vfnmadd132ps as _;
//! use rust_isa::x86::simd::fma::vfnmadd132sd as _;
//! use rust_isa::x86::simd::fma::vfnmadd132ss as _;
//! use rust_isa::x86::simd::fma::vfnmadd213pd as _;
//! use rust_isa::x86::simd::fma::vfnmadd213ps as _;
//! use rust_isa::x86::simd::fma::vfnmadd213sd as _;
//! use rust_isa::x86::simd::fma::vfnmadd213ss as _;
//! use rust_isa::x86::simd::fma::vfnmadd231pd as _;
//! use rust_isa::x86::simd::fma::vfnmadd231ps as _;
//! use rust_isa::x86::simd::fma::vfnmadd231sd as _;
//! use rust_isa::x86::simd::fma::vfnmadd231ss as _;
//! use rust_isa::x86::simd::fma::vfnmsub132pd as _;
//! use rust_isa::x86::simd::fma::vfnmsub132ps as _;
//! use rust_isa::x86::simd::fma::vfnmsub132sd as _;
//! use rust_isa::x86::simd::fma::vfnmsub132ss as _;
//! use rust_isa::x86::simd::fma::vfnmsub213pd as _;
//! use rust_isa::x86::simd::fma::vfnmsub213ps as _;
//! use rust_isa::x86::simd::fma::vfnmsub213sd as _;
//! use rust_isa::x86::simd::fma::vfnmsub213ss as _;
//! use rust_isa::x86::simd::fma::vfnmsub231pd as _;
//! use rust_isa::x86::simd::fma::vfnmsub231ps as _;
//! use rust_isa::x86::simd::fma::vfnmsub231sd as _;
//! use rust_isa::x86::simd::fma::vfnmsub231ss as _;
//! ```
//! <!-- doctest-inventory:end -->

use crate::x86::simd::avx2::V256;
use crate::x86::simd::sse2::V128;

unsafe extern "C" {
    fn rust_isa_fma_vfmadd132ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd132pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd213ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd213pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd231ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd231pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub132ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub132pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub213ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub213pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub231ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub231pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd132ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd132pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd213ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd213pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd231ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd231pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub132ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub132pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub213ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub213pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub231ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub231pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmaddsub132ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmaddsub132pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmaddsub213ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmaddsub213pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmaddsub231ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmaddsub231pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsubadd132ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsubadd132pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsubadd213ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsubadd213pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsubadd231ps(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsubadd231pd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd132ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd132sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd213ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd213sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd231ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmadd231sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub132ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub132sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub213ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub213sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub231ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfmsub231sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd132ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd132sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd213ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd213sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd231ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmadd231sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub132ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub132sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub213ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub213sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub231ss(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_fma_vfnmsub231sd(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

macro_rules! fma256 {
    ($name:ident, $symbol:ident, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must enable AVX2 and FMA.
        #[target_feature(enable = "avx2")]
        #[target_feature(enable = "fma")]
        #[inline]
        pub unsafe fn $name(a: V256, b: V256, c: V256) -> V256 {
            let mut out = V256::from_bytes([0; 32]);
            unsafe {
                $symbol(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! fma128 {
    ($name:ident, $symbol:ident, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must enable AVX2 and FMA.
        #[target_feature(enable = "avx2")]
        #[target_feature(enable = "fma")]
        #[inline]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $symbol(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

fma256!(
    vfmadd132ps,
    rust_isa_fma_vfmadd132ps,
    "Compute `a * b + c` for the eight 32-bit float lanes (132 form)."
);
fma256!(
    vfmadd132pd,
    rust_isa_fma_vfmadd132pd,
    "Compute `a * b + c` for the four 64-bit float lanes (132 form)."
);
fma256!(
    vfmadd213ps,
    rust_isa_fma_vfmadd213ps,
    "Compute `a * b + c` for the eight 32-bit float lanes (213 form)."
);
fma256!(
    vfmadd213pd,
    rust_isa_fma_vfmadd213pd,
    "Compute `a * b + c` for the four 64-bit float lanes (213 form)."
);
fma256!(
    vfmadd231ps,
    rust_isa_fma_vfmadd231ps,
    "Compute `b * c + a` for the eight 32-bit float lanes (231 form)."
);
fma256!(
    vfmadd231pd,
    rust_isa_fma_vfmadd231pd,
    "Compute `b * c + a` for the four 64-bit float lanes (231 form)."
);
fma256!(
    vfmsub132ps,
    rust_isa_fma_vfmsub132ps,
    "Compute `a * b - c` for the eight 32-bit float lanes (132 form)."
);
fma256!(
    vfmsub132pd,
    rust_isa_fma_vfmsub132pd,
    "Compute `a * b - c` for the four 64-bit float lanes (132 form)."
);
fma256!(
    vfmsub213ps,
    rust_isa_fma_vfmsub213ps,
    "Compute `a * b - c` for the eight 32-bit float lanes (213 form)."
);
fma256!(
    vfmsub213pd,
    rust_isa_fma_vfmsub213pd,
    "Compute `a * b - c` for the four 64-bit float lanes (213 form)."
);
fma256!(
    vfmsub231ps,
    rust_isa_fma_vfmsub231ps,
    "Compute `b * c - a` for the eight 32-bit float lanes (231 form)."
);
fma256!(
    vfmsub231pd,
    rust_isa_fma_vfmsub231pd,
    "Compute `b * c - a` for the four 64-bit float lanes (231 form)."
);
fma256!(
    vfnmadd132ps,
    rust_isa_fma_vfnmadd132ps,
    "Compute `c - a * b` for the eight 32-bit float lanes (132 form)."
);
fma256!(
    vfnmadd132pd,
    rust_isa_fma_vfnmadd132pd,
    "Compute `c - a * b` for the four 64-bit float lanes (132 form)."
);
fma256!(
    vfnmadd213ps,
    rust_isa_fma_vfnmadd213ps,
    "Compute `c - a * b` for the eight 32-bit float lanes (213 form)."
);
fma256!(
    vfnmadd213pd,
    rust_isa_fma_vfnmadd213pd,
    "Compute `c - a * b` for the four 64-bit float lanes (213 form)."
);
fma256!(
    vfnmadd231ps,
    rust_isa_fma_vfnmadd231ps,
    "Compute `a - b * c` for the eight 32-bit float lanes (231 form)."
);
fma256!(
    vfnmadd231pd,
    rust_isa_fma_vfnmadd231pd,
    "Compute `a - b * c` for the four 64-bit float lanes (231 form)."
);
fma256!(
    vfnmsub132ps,
    rust_isa_fma_vfnmsub132ps,
    "Compute `-(a * b) - c` for the eight 32-bit float lanes (132 form)."
);
fma256!(
    vfnmsub132pd,
    rust_isa_fma_vfnmsub132pd,
    "Compute `-(a * b) - c` for the four 64-bit float lanes (132 form)."
);
fma256!(
    vfnmsub213ps,
    rust_isa_fma_vfnmsub213ps,
    "Compute `-(a * b) - c` for the eight 32-bit float lanes (213 form)."
);
fma256!(
    vfnmsub213pd,
    rust_isa_fma_vfnmsub213pd,
    "Compute `-(a * b) - c` for the four 64-bit float lanes (213 form)."
);
fma256!(
    vfnmsub231ps,
    rust_isa_fma_vfnmsub231ps,
    "Compute `-(b * c) - a` for the eight 32-bit float lanes (231 form)."
);
fma256!(
    vfnmsub231pd,
    rust_isa_fma_vfnmsub231pd,
    "Compute `-(b * c) - a` for the four 64-bit float lanes (231 form)."
);
fma256!(
    vfmaddsub132ps,
    rust_isa_fma_vfmaddsub132ps,
    "Compute `a * b - c on even lanes, a * b + c on odd lanes` for the eight 32-bit float lanes (132 form)."
);
fma256!(
    vfmaddsub132pd,
    rust_isa_fma_vfmaddsub132pd,
    "Compute `a * b - c on even lanes, a * b + c on odd lanes` for the four 64-bit float lanes (132 form)."
);
fma256!(
    vfmaddsub213ps,
    rust_isa_fma_vfmaddsub213ps,
    "Compute `a * b - c on even lanes, a * b + c on odd lanes` for the eight 32-bit float lanes (213 form)."
);
fma256!(
    vfmaddsub213pd,
    rust_isa_fma_vfmaddsub213pd,
    "Compute `a * b - c on even lanes, a * b + c on odd lanes` for the four 64-bit float lanes (213 form)."
);
fma256!(
    vfmaddsub231ps,
    rust_isa_fma_vfmaddsub231ps,
    "Compute `b * c - a on even lanes, b * c + a on odd lanes` for the eight 32-bit float lanes (231 form)."
);
fma256!(
    vfmaddsub231pd,
    rust_isa_fma_vfmaddsub231pd,
    "Compute `b * c - a on even lanes, b * c + a on odd lanes` for the four 64-bit float lanes (231 form)."
);
fma256!(
    vfmsubadd132ps,
    rust_isa_fma_vfmsubadd132ps,
    "Compute `a * b + c on even lanes, a * b - c on odd lanes` for the eight 32-bit float lanes (132 form)."
);
fma256!(
    vfmsubadd132pd,
    rust_isa_fma_vfmsubadd132pd,
    "Compute `a * b + c on even lanes, a * b - c on odd lanes` for the four 64-bit float lanes (132 form)."
);
fma256!(
    vfmsubadd213ps,
    rust_isa_fma_vfmsubadd213ps,
    "Compute `a * b + c on even lanes, a * b - c on odd lanes` for the eight 32-bit float lanes (213 form)."
);
fma256!(
    vfmsubadd213pd,
    rust_isa_fma_vfmsubadd213pd,
    "Compute `a * b + c on even lanes, a * b - c on odd lanes` for the four 64-bit float lanes (213 form)."
);
fma256!(
    vfmsubadd231ps,
    rust_isa_fma_vfmsubadd231ps,
    "Compute `b * c + a on even lanes, b * c - a on odd lanes` for the eight 32-bit float lanes (231 form)."
);
fma256!(
    vfmsubadd231pd,
    rust_isa_fma_vfmsubadd231pd,
    "Compute `b * c + a on even lanes, b * c - a on odd lanes` for the four 64-bit float lanes (231 form)."
);
fma128!(
    vfmadd132ss,
    rust_isa_fma_vfmadd132ss,
    "Compute `a * b + c` in the low 32-bit float lane and preserve the upper lanes of `a` (132 form)."
);
fma128!(
    vfmadd132sd,
    rust_isa_fma_vfmadd132sd,
    "Compute `a * b + c` in the low 64-bit float lane and preserve the upper lanes of `a` (132 form)."
);
fma128!(
    vfmadd213ss,
    rust_isa_fma_vfmadd213ss,
    "Compute `a * b + c` in the low 32-bit float lane and preserve the upper lanes of `a` (213 form)."
);
fma128!(
    vfmadd213sd,
    rust_isa_fma_vfmadd213sd,
    "Compute `a * b + c` in the low 64-bit float lane and preserve the upper lanes of `a` (213 form)."
);
fma128!(
    vfmadd231ss,
    rust_isa_fma_vfmadd231ss,
    "Compute `b * c + a` in the low 32-bit float lane and preserve the upper lanes of `a` (231 form)."
);
fma128!(
    vfmadd231sd,
    rust_isa_fma_vfmadd231sd,
    "Compute `b * c + a` in the low 64-bit float lane and preserve the upper lanes of `a` (231 form)."
);
fma128!(
    vfmsub132ss,
    rust_isa_fma_vfmsub132ss,
    "Compute `a * b - c` in the low 32-bit float lane and preserve the upper lanes of `a` (132 form)."
);
fma128!(
    vfmsub132sd,
    rust_isa_fma_vfmsub132sd,
    "Compute `a * b - c` in the low 64-bit float lane and preserve the upper lanes of `a` (132 form)."
);
fma128!(
    vfmsub213ss,
    rust_isa_fma_vfmsub213ss,
    "Compute `a * b - c` in the low 32-bit float lane and preserve the upper lanes of `a` (213 form)."
);
fma128!(
    vfmsub213sd,
    rust_isa_fma_vfmsub213sd,
    "Compute `a * b - c` in the low 64-bit float lane and preserve the upper lanes of `a` (213 form)."
);
fma128!(
    vfmsub231ss,
    rust_isa_fma_vfmsub231ss,
    "Compute `b * c - a` in the low 32-bit float lane and preserve the upper lanes of `a` (231 form)."
);
fma128!(
    vfmsub231sd,
    rust_isa_fma_vfmsub231sd,
    "Compute `b * c - a` in the low 64-bit float lane and preserve the upper lanes of `a` (231 form)."
);
fma128!(
    vfnmadd132ss,
    rust_isa_fma_vfnmadd132ss,
    "Compute `c - a * b` in the low 32-bit float lane and preserve the upper lanes of `a` (132 form)."
);
fma128!(
    vfnmadd132sd,
    rust_isa_fma_vfnmadd132sd,
    "Compute `c - a * b` in the low 64-bit float lane and preserve the upper lanes of `a` (132 form)."
);
fma128!(
    vfnmadd213ss,
    rust_isa_fma_vfnmadd213ss,
    "Compute `c - a * b` in the low 32-bit float lane and preserve the upper lanes of `a` (213 form)."
);
fma128!(
    vfnmadd213sd,
    rust_isa_fma_vfnmadd213sd,
    "Compute `c - a * b` in the low 64-bit float lane and preserve the upper lanes of `a` (213 form)."
);
fma128!(
    vfnmadd231ss,
    rust_isa_fma_vfnmadd231ss,
    "Compute `a - b * c` in the low 32-bit float lane and preserve the upper lanes of `a` (231 form)."
);
fma128!(
    vfnmadd231sd,
    rust_isa_fma_vfnmadd231sd,
    "Compute `a - b * c` in the low 64-bit float lane and preserve the upper lanes of `a` (231 form)."
);
fma128!(
    vfnmsub132ss,
    rust_isa_fma_vfnmsub132ss,
    "Compute `-(a * b) - c` in the low 32-bit float lane and preserve the upper lanes of `a` (132 form)."
);
fma128!(
    vfnmsub132sd,
    rust_isa_fma_vfnmsub132sd,
    "Compute `-(a * b) - c` in the low 64-bit float lane and preserve the upper lanes of `a` (132 form)."
);
fma128!(
    vfnmsub213ss,
    rust_isa_fma_vfnmsub213ss,
    "Compute `-(a * b) - c` in the low 32-bit float lane and preserve the upper lanes of `a` (213 form)."
);
fma128!(
    vfnmsub213sd,
    rust_isa_fma_vfnmsub213sd,
    "Compute `-(a * b) - c` in the low 64-bit float lane and preserve the upper lanes of `a` (213 form)."
);
fma128!(
    vfnmsub231ss,
    rust_isa_fma_vfnmsub231ss,
    "Compute `-(b * c) - a` in the low 32-bit float lane and preserve the upper lanes of `a` (231 form)."
);
fma128!(
    vfnmsub231sd,
    rust_isa_fma_vfnmsub231sd,
    "Compute `-(b * c) - a` in the low 64-bit float lane and preserve the upper lanes of `a` (231 form)."
);
