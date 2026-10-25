//! SSE2.
//!
//! SSE2 is part of the x86_64 baseline. The arithmetic operations are safe.
//! Loads and stores take raw pointers and are `unsafe`.
//!
//! The values are [`V128`]. The instruction decides what the lanes mean.
//! Immediate operands are const generics unless documented otherwise.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::simd::sse2::paddd;
//! use rust_isa::x86::simd::sse2::V128;
//!
//! let a = V128::from_bytes([1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
//! let b = V128::from_bytes([10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0]);
//! assert_eq!(
//!     paddd(a, b).to_bytes(),
//!     [11, 0, 0, 0, 22, 0, 0, 0, 33, 0, 0, 0, 44, 0, 0, 0]
//! );
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::sse2::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = addpd(v128, v128);
//! let _ = addps(v128, v128);
//! let _ = andnps(v128, v128);
//! let _ = andps(v128, v128);
//! let _ = cmpeqpd(v128, v128);
//! let _ = cmpeqps(v128, v128);
//! let _ = cmplepd(v128, v128);
//! let _ = cmpleps(v128, v128);
//! let _ = cmpltpd(v128, v128);
//! let _ = cmpltps(v128, v128);
//! let _ = comisd(v128, v128);
//! let _ = comiss(v128, v128);
//! let _ = cvtdq2pd(v128);
//! let _ = cvtdq2ps(v128);
//! let _ = cvtpd2dq(v128);
//! let _ = cvtpd2ps(v128);
//! let _ = cvtps2dq(v128);
//! let _ = cvtps2pd(v128);
//! let _ = cvtsd2si(v128);
//! let _ = cvtsi2sd(0);
//! let _ = cvtsi2ss(0);
//! let _ = cvtss2si(v128);
//! let _ = cvttpd2dq(v128);
//! let _ = cvttps2dq(v128);
//! let _ = cvttsd2si(v128);
//! let _ = cvttss2si(v128);
//! let _ = divpd(v128, v128);
//! let _ = divps(v128, v128);
//! let _ = unsafe { maskmovdqu(v128, v128, storage.0.as_mut_ptr().cast()) };
//! let _ = maxpd(v128, v128);
//! let _ = maxps(v128, v128);
//! let _ = minpd(v128, v128);
//! let _ = minps(v128, v128);
//! let _ = movd(v128);
//! let _ = movhlps(v128, v128);
//! let _ = movhps(v128, v128);
//! let _ = movlhps(v128, v128);
//! let _ = movlps(v128, v128);
//! let _ = movmskpd(v128);
//! let _ = movmskps(v128);
//! let _ = unsafe { movntdq(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { movnti(storage.0.as_mut_ptr().cast(), 0) };
//! let _ = unsafe { movntpd(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { movntps(storage.0.as_mut_ptr().cast(), v128) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { movntq(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = movq(v128);
//! let _ = movsd(v128, v128);
//! let _ = movss(v128, v128);
//! let _ = mulpd(v128, v128);
//! let _ = mulps(v128, v128);
//! let _ = orps(v128, v128);
//! let _ = packssdw(v128, v128);
//! let _ = packsswb(v128, v128);
//! let _ = packuswb(v128, v128);
//! let _ = paddb(v128, v128);
//! let _ = paddd(v128, v128);
//! let _ = paddq(v128, v128);
//! let _ = paddsb(v128, v128);
//! let _ = paddsw(v128, v128);
//! let _ = paddusb(v128, v128);
//! let _ = paddusw(v128, v128);
//! let _ = paddw(v128, v128);
//! let _ = pand(v128, v128);
//! let _ = pandn(v128, v128);
//! let _ = pavgb(v128, v128);
//! let _ = pavgw(v128, v128);
//! let _ = pcmpeqb(v128, v128);
//! let _ = pcmpeqd(v128, v128);
//! let _ = pcmpeqw(v128, v128);
//! let _ = pcmpgtb(v128, v128);
//! let _ = pcmpgtd(v128, v128);
//! let _ = pcmpgtw(v128, v128);
//! let _ = pmaddwd(v128, v128);
//! let _ = pmaxsw(v128, v128);
//! let _ = pmaxub(v128, v128);
//! let _ = pminsw(v128, v128);
//! let _ = pminub(v128, v128);
//! let _ = pmovmskb(v128);
//! let _ = pmulhuw(v128, v128);
//! let _ = pmulhw(v128, v128);
//! let _ = pmullw(v128, v128);
//! let _ = pmuludq(v128, v128);
//! let _ = por(v128, v128);
//! let _ = psadbw(v128, v128);
//! let _ = pshufd::<0>(v128);
//! let _ = pshufhw::<0>(v128);
//! let _ = pshuflw::<0>(v128);
//! let _ = pslld(v128, 0);
//! let _ = pslldq::<0>(v128);
//! let _ = psllq(v128, 0);
//! let _ = psllw(v128, 0);
//! let _ = psrad(v128, 0);
//! let _ = psraw(v128, 0);
//! let _ = psrld(v128, 0);
//! let _ = psrldq::<0>(v128);
//! let _ = psrlq(v128, 0);
//! let _ = psrlw(v128, 0);
//! let _ = psubb(v128, v128);
//! let _ = psubd(v128, v128);
//! let _ = psubq(v128, v128);
//! let _ = psubsb(v128, v128);
//! let _ = psubsw(v128, v128);
//! let _ = psubusb(v128, v128);
//! let _ = psubusw(v128, v128);
//! let _ = psubw(v128, v128);
//! let _ = punpckhbw(v128, v128);
//! let _ = punpckhdq(v128, v128);
//! let _ = punpckhqdq(v128, v128);
//! let _ = punpckhwd(v128, v128);
//! let _ = punpcklbw(v128, v128);
//! let _ = punpckldq(v128, v128);
//! let _ = punpcklqdq(v128, v128);
//! let _ = punpcklwd(v128, v128);
//! let _ = pxor(v128, v128);
//! let _ = shufpd::<0>(v128, v128);
//! #[cfg(target_arch = "x86_64")] let _ = shufps::<0>(v128, v128);
//! let _ = sqrtpd(v128);
//! let _ = sqrtps(v128);
//! let _ = subpd(v128, v128);
//! let _ = subps(v128, v128);
//! let _ = ucomisd(v128, v128);
//! let _ = ucomiss(v128, v128);
//! let _ = unpckhpd(v128, v128);
//! let _ = unpckhps(v128, v128);
//! let _ = unpcklpd(v128, v128);
//! let _ = unpcklps(v128, v128);
//! let _ = unsafe { v128_load(storage.0.as_ptr().cast()) };
//! let _ = unsafe { v128_store(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = xorps(v128, v128);
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Instruction inventory</summary>
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::sse2::addpd as _;
//! use rust_isa::x86::simd::sse2::addps as _;
//! use rust_isa::x86::simd::sse2::andnps as _;
//! use rust_isa::x86::simd::sse2::andps as _;
//! use rust_isa::x86::simd::sse2::cmpeqpd as _;
//! use rust_isa::x86::simd::sse2::cmpeqps as _;
//! use rust_isa::x86::simd::sse2::cmplepd as _;
//! use rust_isa::x86::simd::sse2::cmpleps as _;
//! use rust_isa::x86::simd::sse2::cmpltpd as _;
//! use rust_isa::x86::simd::sse2::cmpltps as _;
//! use rust_isa::x86::simd::sse2::comisd as _;
//! use rust_isa::x86::simd::sse2::comiss as _;
//! use rust_isa::x86::simd::sse2::cvtdq2pd as _;
//! use rust_isa::x86::simd::sse2::cvtdq2ps as _;
//! use rust_isa::x86::simd::sse2::cvtpd2dq as _;
//! use rust_isa::x86::simd::sse2::cvtpd2ps as _;
//! use rust_isa::x86::simd::sse2::cvtps2dq as _;
//! use rust_isa::x86::simd::sse2::cvtps2pd as _;
//! use rust_isa::x86::simd::sse2::cvtsd2si as _;
//! use rust_isa::x86::simd::sse2::cvtsi2sd as _;
//! use rust_isa::x86::simd::sse2::cvtsi2ss as _;
//! use rust_isa::x86::simd::sse2::cvtss2si as _;
//! use rust_isa::x86::simd::sse2::cvttpd2dq as _;
//! use rust_isa::x86::simd::sse2::cvttps2dq as _;
//! use rust_isa::x86::simd::sse2::cvttsd2si as _;
//! use rust_isa::x86::simd::sse2::cvttss2si as _;
//! use rust_isa::x86::simd::sse2::divpd as _;
//! use rust_isa::x86::simd::sse2::divps as _;
//! use rust_isa::x86::simd::sse2::maskmovdqu as _;
//! use rust_isa::x86::simd::sse2::maxpd as _;
//! use rust_isa::x86::simd::sse2::maxps as _;
//! use rust_isa::x86::simd::sse2::minpd as _;
//! use rust_isa::x86::simd::sse2::minps as _;
//! use rust_isa::x86::simd::sse2::movd as _;
//! use rust_isa::x86::simd::sse2::movhlps as _;
//! use rust_isa::x86::simd::sse2::movhps as _;
//! use rust_isa::x86::simd::sse2::movlhps as _;
//! use rust_isa::x86::simd::sse2::movlps as _;
//! use rust_isa::x86::simd::sse2::movmskpd as _;
//! use rust_isa::x86::simd::sse2::movmskps as _;
//! use rust_isa::x86::simd::sse2::movntdq as _;
//! use rust_isa::x86::simd::sse2::movnti as _;
//! use rust_isa::x86::simd::sse2::movntpd as _;
//! use rust_isa::x86::simd::sse2::movntps as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::simd::sse2::movntq as _;
//! use rust_isa::x86::simd::sse2::movq as _;
//! use rust_isa::x86::simd::sse2::movsd as _;
//! use rust_isa::x86::simd::sse2::movss as _;
//! use rust_isa::x86::simd::sse2::mulpd as _;
//! use rust_isa::x86::simd::sse2::mulps as _;
//! use rust_isa::x86::simd::sse2::orps as _;
//! use rust_isa::x86::simd::sse2::packssdw as _;
//! use rust_isa::x86::simd::sse2::packsswb as _;
//! use rust_isa::x86::simd::sse2::packuswb as _;
//! use rust_isa::x86::simd::sse2::paddb as _;
//! use rust_isa::x86::simd::sse2::paddd as _;
//! use rust_isa::x86::simd::sse2::paddq as _;
//! use rust_isa::x86::simd::sse2::paddsb as _;
//! use rust_isa::x86::simd::sse2::paddsw as _;
//! use rust_isa::x86::simd::sse2::paddusb as _;
//! use rust_isa::x86::simd::sse2::paddusw as _;
//! use rust_isa::x86::simd::sse2::paddw as _;
//! use rust_isa::x86::simd::sse2::pand as _;
//! use rust_isa::x86::simd::sse2::pandn as _;
//! use rust_isa::x86::simd::sse2::pavgb as _;
//! use rust_isa::x86::simd::sse2::pavgw as _;
//! use rust_isa::x86::simd::sse2::pcmpeqb as _;
//! use rust_isa::x86::simd::sse2::pcmpeqd as _;
//! use rust_isa::x86::simd::sse2::pcmpeqw as _;
//! use rust_isa::x86::simd::sse2::pcmpgtb as _;
//! use rust_isa::x86::simd::sse2::pcmpgtd as _;
//! use rust_isa::x86::simd::sse2::pcmpgtw as _;
//! use rust_isa::x86::simd::sse2::pmaddwd as _;
//! use rust_isa::x86::simd::sse2::pmaxsw as _;
//! use rust_isa::x86::simd::sse2::pmaxub as _;
//! use rust_isa::x86::simd::sse2::pminsw as _;
//! use rust_isa::x86::simd::sse2::pminub as _;
//! use rust_isa::x86::simd::sse2::pmovmskb as _;
//! use rust_isa::x86::simd::sse2::pmulhuw as _;
//! use rust_isa::x86::simd::sse2::pmulhw as _;
//! use rust_isa::x86::simd::sse2::pmullw as _;
//! use rust_isa::x86::simd::sse2::pmuludq as _;
//! use rust_isa::x86::simd::sse2::por as _;
//! use rust_isa::x86::simd::sse2::psadbw as _;
//! use rust_isa::x86::simd::sse2::pshufd as _;
//! use rust_isa::x86::simd::sse2::pshufhw as _;
//! use rust_isa::x86::simd::sse2::pshuflw as _;
//! use rust_isa::x86::simd::sse2::pslld as _;
//! use rust_isa::x86::simd::sse2::pslldq as _;
//! use rust_isa::x86::simd::sse2::psllq as _;
//! use rust_isa::x86::simd::sse2::psllw as _;
//! use rust_isa::x86::simd::sse2::psrad as _;
//! use rust_isa::x86::simd::sse2::psraw as _;
//! use rust_isa::x86::simd::sse2::psrld as _;
//! use rust_isa::x86::simd::sse2::psrldq as _;
//! use rust_isa::x86::simd::sse2::psrlq as _;
//! use rust_isa::x86::simd::sse2::psrlw as _;
//! use rust_isa::x86::simd::sse2::psubb as _;
//! use rust_isa::x86::simd::sse2::psubd as _;
//! use rust_isa::x86::simd::sse2::psubq as _;
//! use rust_isa::x86::simd::sse2::psubsb as _;
//! use rust_isa::x86::simd::sse2::psubsw as _;
//! use rust_isa::x86::simd::sse2::psubusb as _;
//! use rust_isa::x86::simd::sse2::psubusw as _;
//! use rust_isa::x86::simd::sse2::psubw as _;
//! use rust_isa::x86::simd::sse2::punpckhbw as _;
//! use rust_isa::x86::simd::sse2::punpckhdq as _;
//! use rust_isa::x86::simd::sse2::punpckhqdq as _;
//! use rust_isa::x86::simd::sse2::punpckhwd as _;
//! use rust_isa::x86::simd::sse2::punpcklbw as _;
//! use rust_isa::x86::simd::sse2::punpckldq as _;
//! use rust_isa::x86::simd::sse2::punpcklqdq as _;
//! use rust_isa::x86::simd::sse2::punpcklwd as _;
//! use rust_isa::x86::simd::sse2::pxor as _;
//! use rust_isa::x86::simd::sse2::shufpd as _;
//! use rust_isa::x86::simd::sse2::shufps as _;
//! use rust_isa::x86::simd::sse2::sqrtpd as _;
//! use rust_isa::x86::simd::sse2::sqrtps as _;
//! use rust_isa::x86::simd::sse2::subpd as _;
//! use rust_isa::x86::simd::sse2::subps as _;
//! use rust_isa::x86::simd::sse2::ucomisd as _;
//! use rust_isa::x86::simd::sse2::ucomiss as _;
//! use rust_isa::x86::simd::sse2::unpckhpd as _;
//! use rust_isa::x86::simd::sse2::unpckhps as _;
//! use rust_isa::x86::simd::sse2::unpcklpd as _;
//! use rust_isa::x86::simd::sse2::unpcklps as _;
//! use rust_isa::x86::simd::sse2::v128_load as _;
//! use rust_isa::x86::simd::sse2::v128_store as _;
//! use rust_isa::x86::simd::sse2::xorps as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

// Internal modules are grouped by operation.
mod arithmetic;
mod bitwise;
mod compare;
mod convert;
mod memory;
mod movemask;
mod moves;
mod nontemporal;
mod pack;
mod shift;
mod shuffle;
mod unpack;
mod v128;

pub use arithmetic::*;
pub use bitwise::*;
pub use compare::*;
pub use convert::*;
pub use memory::*;
pub use movemask::*;
pub use moves::*;
pub use nontemporal::*;
pub use pack::*;
pub use shift::*;
pub use shuffle::*;
pub use unpack::*;
pub use v128::*;
