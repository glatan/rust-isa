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
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(unsafe { paddd(v, v) }.to_bytes(), [0; 16]);
//! ```
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
