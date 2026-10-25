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
//! # Instruction examples
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
//!
//! unsafe {
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { addpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { addps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { andnps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { andps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cmpeqpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cmpeqps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cmplepd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cmpleps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cmpltpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cmpltps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { comisd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { comiss(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtdq2pd(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtdq2ps(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtpd2dq(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtpd2ps(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtps2dq(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtps2pd(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtsd2si(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtsi2sd(0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtsi2ss(0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvtss2si(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvttpd2dq(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvttps2dq(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvttsd2si(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { cvttss2si(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { divpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { divps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { maskmovdqu(v128, v128, core::ptr::null_mut()) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { maxpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { maxps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { minpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { minps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movd(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movhlps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movhps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movlhps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movlps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movmskpd(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movmskps(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movntdq(core::ptr::null_mut(), v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movnti(core::ptr::null_mut(), 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movntpd(core::ptr::null_mut(), v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movntps(core::ptr::null_mut(), v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movntq(core::ptr::null_mut(), v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movq(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movsd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { movss(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { mulpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { mulps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { orps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { packssdw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { packsswb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { packuswb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { paddb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { paddd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { paddq(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { paddsb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { paddsw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { paddusb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { paddusw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { paddw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pand(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pandn(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pavgb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pavgw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pcmpeqb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pcmpeqd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pcmpeqw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pcmpgtb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pcmpgtd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pcmpgtw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pmaddwd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pmaxsw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pmaxub(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pminsw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pminub(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pmovmskb(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pmulhuw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pmulhw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pmullw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pmuludq(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { por(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psadbw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pshufd::<0>(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pshufhw::<0>(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pshuflw::<0>(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pslld(v128, 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pslldq::<0>(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psllq(v128, 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psllw(v128, 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psrad(v128, 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psraw(v128, 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psrld(v128, 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psrldq::<0>(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psrlq(v128, 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psrlw(v128, 0) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psubb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psubd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psubq(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psubsb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psubsw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psubusb(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psubusw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { psubw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { punpckhbw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { punpckhdq(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { punpckhqdq(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { punpckhwd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { punpcklbw(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { punpckldq(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { punpcklqdq(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { punpcklwd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { pxor(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { shufpd::<0>(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { shufps::<0>(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { sqrtpd(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { sqrtps(v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { subpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { subps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { ucomisd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { ucomiss(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { unpckhpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { unpckhps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { unpcklpd(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { unpcklps(v128, v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { v128_load(core::ptr::null()) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { v128_store(core::ptr::null_mut(), v128) };
//!     #[cfg(target_arch = "x86_64")] let _ = unsafe { xorps(v128, v128) };
//! }
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
