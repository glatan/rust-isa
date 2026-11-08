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

//! # Examples
//!
//! ```
//! #[cfg(target_arch = "x86_64")]
//! if std::env::var_os("RUST_ISA_SDE_FLAGS").is_some() {
//! use rust_isa::x86::simd::avx512f::vpaddd;
//! use rust_isa::x86::simd::avx512f::V512;
//! use rust_isa::x86::simd::{Mask8, Mask16};
//!
//! let v = V512::from_bytes([0; 64]);
//! assert_eq!(unsafe { vpaddd(v, v) }.to_bytes(), [0; 64]);
//! }
//! ```
//!
//! # Compile-time predicate check
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx512f::vpcmpd;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vpcmpd::<8>(v, v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx512f::vpcmpq;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vpcmpq::<8>(v, v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx512f::vpcmpud;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vpcmpud::<8>(v, v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx512f::vpcmpuq;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vpcmpuq::<8>(v, v) };
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx512f::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//! use rust_isa::x86::simd::{Mask8, Mask16};
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { v512_load(storage.0.as_ptr().cast()) };
//! let _ = unsafe { v512_store(storage.0.as_mut_ptr().cast(), v512) };
//! let _ = unsafe { vblendmpd(v512, v512, Mask8::NONE) };
//! let _ = unsafe { vblendmps(v512, v512, Mask16::NONE) };
//! let _ = unsafe { vcompresspd(v512, Mask8::NONE) };
//! let _ = unsafe { vcompressps(v512, Mask16::NONE) };
//! let _ = unsafe { vcvtdq2ps(v512) };
//! let _ = unsafe { vcvtpd2dq(v512) };
//! let _ = unsafe { vcvtpd2ps(v512) };
//! let _ = unsafe { vcvtph2ps(v256) };
//! let _ = unsafe { vcvtps2dq(v512) };
//! let _ = unsafe { vcvtps2pd(v256) };
//! let _ = unsafe { vcvtps2ph(v512) };
//! let _ = unsafe { vcvtsd2ss(v128, v128) };
//! let _ = unsafe { vcvtss2sd(v128, v128) };
//! let _ = unsafe { vcvttpd2dq(v512) };
//! let _ = unsafe { vcvttps2dq(v512) };
//! let _ = unsafe { vexpandpd(v512, Mask8::NONE) };
//! let _ = unsafe { vexpandps(v512, Mask16::NONE) };
//! let _ = unsafe { vextractf32x4::<0>(v512) };
//! let _ = unsafe { vextractf64x4::<0>(v512) };
//! let _ = unsafe { vgatherdpd(v512, storage.0.as_ptr().cast(), v256, Mask8::NONE) };
//! let _ = unsafe { vgatherdps(v512, storage.0.as_ptr().cast(), v512, Mask16::NONE) };
//! let _ = unsafe { vgatherqpd(v512, storage.0.as_ptr().cast(), v512, Mask8::NONE) };
//! let _ = unsafe { vgatherqps(v256, storage.0.as_ptr().cast(), v512, Mask8::NONE) };
//! let _ = unsafe { vgetexppd(v512) };
//! let _ = unsafe { vgetexpps(v512) };
//! let _ = unsafe { vgetmantpd::<0>(v512) };
//! let _ = unsafe { vgetmantps::<0>(v512) };
//! let _ = unsafe { vinsertf32x4::<0>(v512, v128) };
//! let _ = unsafe { vinsertf64x4::<0>(v512, v256) };
//! let _ = unsafe { vinsertps::<0>(v128, v128) };
//! let _ = unsafe { vmovapd(v512, v512, Mask8::NONE) };
//! let _ = unsafe { vmovaps(v512, v512, Mask16::NONE) };
//! let _ = unsafe { vpaddd(v512, v512) };
//! let _ = unsafe { vpaddq(v512, v512) };
//! let _ = unsafe { vpandd(v512, v512) };
//! let _ = unsafe { vpandnd(v512, v512) };
//! let _ = unsafe { vpandnq(v512, v512) };
//! let _ = unsafe { vpandq(v512, v512) };
//! let _ = unsafe { vpbroadcastd(0) };
//! let _ = unsafe { vpbroadcastq(0) };
//! let _ = unsafe { vpcmpd::<0>(v512, v512) };
//! let _ = unsafe { vpcmpq::<0>(v512, v512) };
//! let _ = unsafe { vpcmpud::<0>(v512, v512) };
//! let _ = unsafe { vpcmpuq::<0>(v512, v512) };
//! let _ = unsafe { vpermd(v512, v512) };
//! let _ = unsafe { vpermilpd::<0>(v512) };
//! let _ = unsafe { vpermilps::<0>(v512) };
//! let _ = unsafe { vpermq(v512, v512) };
//! let _ = unsafe { vpmulld(v512, v512) };
//! let _ = unsafe { vpord(v512, v512) };
//! let _ = unsafe { vporq(v512, v512) };
//! let _ = unsafe { vpslld(v512, 0) };
//! let _ = unsafe { vpsrld(v512, 0) };
//! let _ = unsafe { vpsubd(v512, v512) };
//! let _ = unsafe { vpsubq(v512, v512) };
//! let _ = unsafe { vpternlogd::<0>(v512, v512, v512) };
//! let _ = unsafe { vpternlogq::<0>(v512, v512, v512) };
//! let _ = unsafe { vptestmd(v512, v512) };
//! let _ = unsafe { vptestmq(v512, v512) };
//! let _ = unsafe { vptestnmd(v512, v512) };
//! let _ = unsafe { vptestnmq(v512, v512) };
//! let _ = unsafe { vpxord(v512, v512) };
//! let _ = unsafe { vpxorq(v512, v512) };
//! let _ = unsafe { vrcpps(v512) };
//! let _ = unsafe { vrndscalepd::<0>(v512) };
//! let _ = unsafe { vrndscaleps::<0>(v512) };
//! let _ = unsafe { vrsqrtps(v512) };
//! let _ = unsafe { vscalefpd(v512, v512) };
//! let _ = unsafe { vscalefps(v512, v512) };
//! let _ = unsafe { vscatterdpd(storage.0.as_mut_ptr().cast(), v256, v512, Mask8::NONE) };
//! let _ = unsafe { vscatterdps(storage.0.as_mut_ptr().cast(), v512, v512, Mask16::NONE) };
//! let _ = unsafe { vscatterqpd(storage.0.as_mut_ptr().cast(), v512, v512, Mask8::NONE) };
//! let _ = unsafe { vscatterqps(storage.0.as_mut_ptr().cast(), v512, v256, Mask8::NONE) };
//! let _ = unsafe { vshufpd::<0>(v512, v512) };
//! let _ = unsafe { vshufps::<0>(v512, v512) };
//! let _ = unsafe { vsqrtpd(v512) };
//! let _ = unsafe { vsqrtps(v512) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx512f::v512_load as _;
//! use rust_isa::x86::simd::avx512f::v512_store as _;
//! use rust_isa::x86::simd::avx512f::vblendmpd as _;
//! use rust_isa::x86::simd::avx512f::vblendmps as _;
//! use rust_isa::x86::simd::avx512f::vcompresspd as _;
//! use rust_isa::x86::simd::avx512f::vcompressps as _;
//! use rust_isa::x86::simd::avx512f::vcvtdq2ps as _;
//! use rust_isa::x86::simd::avx512f::vcvtpd2dq as _;
//! use rust_isa::x86::simd::avx512f::vcvtpd2ps as _;
//! use rust_isa::x86::simd::avx512f::vcvtph2ps as _;
//! use rust_isa::x86::simd::avx512f::vcvtps2dq as _;
//! use rust_isa::x86::simd::avx512f::vcvtps2pd as _;
//! use rust_isa::x86::simd::avx512f::vcvtps2ph as _;
//! use rust_isa::x86::simd::avx512f::vcvtsd2ss as _;
//! use rust_isa::x86::simd::avx512f::vcvtss2sd as _;
//! use rust_isa::x86::simd::avx512f::vcvttpd2dq as _;
//! use rust_isa::x86::simd::avx512f::vcvttps2dq as _;
//! use rust_isa::x86::simd::avx512f::vexpandpd as _;
//! use rust_isa::x86::simd::avx512f::vexpandps as _;
//! use rust_isa::x86::simd::avx512f::vextractf32x4 as _;
//! use rust_isa::x86::simd::avx512f::vextractf64x4 as _;
//! use rust_isa::x86::simd::avx512f::vgatherdpd as _;
//! use rust_isa::x86::simd::avx512f::vgatherdps as _;
//! use rust_isa::x86::simd::avx512f::vgatherqpd as _;
//! use rust_isa::x86::simd::avx512f::vgatherqps as _;
//! use rust_isa::x86::simd::avx512f::vgetexppd as _;
//! use rust_isa::x86::simd::avx512f::vgetexpps as _;
//! use rust_isa::x86::simd::avx512f::vgetmantpd as _;
//! use rust_isa::x86::simd::avx512f::vgetmantps as _;
//! use rust_isa::x86::simd::avx512f::vinsertf32x4 as _;
//! use rust_isa::x86::simd::avx512f::vinsertf64x4 as _;
//! use rust_isa::x86::simd::avx512f::vinsertps as _;
//! use rust_isa::x86::simd::avx512f::vmovapd as _;
//! use rust_isa::x86::simd::avx512f::vmovaps as _;
//! use rust_isa::x86::simd::avx512f::vpaddd as _;
//! use rust_isa::x86::simd::avx512f::vpaddq as _;
//! use rust_isa::x86::simd::avx512f::vpandd as _;
//! use rust_isa::x86::simd::avx512f::vpandnd as _;
//! use rust_isa::x86::simd::avx512f::vpandnq as _;
//! use rust_isa::x86::simd::avx512f::vpandq as _;
//! use rust_isa::x86::simd::avx512f::vpbroadcastd as _;
//! use rust_isa::x86::simd::avx512f::vpbroadcastq as _;
//! use rust_isa::x86::simd::avx512f::vpcmpd as _;
//! use rust_isa::x86::simd::avx512f::vpcmpq as _;
//! use rust_isa::x86::simd::avx512f::vpcmpud as _;
//! use rust_isa::x86::simd::avx512f::vpcmpuq as _;
//! use rust_isa::x86::simd::avx512f::vpermd as _;
//! use rust_isa::x86::simd::avx512f::vpermilpd as _;
//! use rust_isa::x86::simd::avx512f::vpermilps as _;
//! use rust_isa::x86::simd::avx512f::vpermq as _;
//! use rust_isa::x86::simd::avx512f::vpmulld as _;
//! use rust_isa::x86::simd::avx512f::vpord as _;
//! use rust_isa::x86::simd::avx512f::vporq as _;
//! use rust_isa::x86::simd::avx512f::vpslld as _;
//! use rust_isa::x86::simd::avx512f::vpsrld as _;
//! use rust_isa::x86::simd::avx512f::vpsubd as _;
//! use rust_isa::x86::simd::avx512f::vpsubq as _;
//! use rust_isa::x86::simd::avx512f::vpternlogd as _;
//! use rust_isa::x86::simd::avx512f::vpternlogq as _;
//! use rust_isa::x86::simd::avx512f::vptestmd as _;
//! use rust_isa::x86::simd::avx512f::vptestmq as _;
//! use rust_isa::x86::simd::avx512f::vptestnmd as _;
//! use rust_isa::x86::simd::avx512f::vptestnmq as _;
//! use rust_isa::x86::simd::avx512f::vpxord as _;
//! use rust_isa::x86::simd::avx512f::vpxorq as _;
//! use rust_isa::x86::simd::avx512f::vrcpps as _;
//! use rust_isa::x86::simd::avx512f::vrndscalepd as _;
//! use rust_isa::x86::simd::avx512f::vrndscaleps as _;
//! use rust_isa::x86::simd::avx512f::vrsqrtps as _;
//! use rust_isa::x86::simd::avx512f::vscalefpd as _;
//! use rust_isa::x86::simd::avx512f::vscalefps as _;
//! use rust_isa::x86::simd::avx512f::vscatterdpd as _;
//! use rust_isa::x86::simd::avx512f::vscatterdps as _;
//! use rust_isa::x86::simd::avx512f::vscatterqpd as _;
//! use rust_isa::x86::simd::avx512f::vscatterqps as _;
//! use rust_isa::x86::simd::avx512f::vshufpd as _;
//! use rust_isa::x86::simd::avx512f::vshufps as _;
//! use rust_isa::x86::simd::avx512f::vsqrtpd as _;
//! use rust_isa::x86::simd::avx512f::vsqrtps as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

mod float;
mod gather_scatter;
mod integer;
mod v512;

pub use float::*;
pub use gather_scatter::*;
pub use integer::*;
pub use v512::*;
