//! AVX2.
//!
//! AVX2 is not part of the x86_64 baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable AVX2 with `#[target_feature(enable = "avx2")]` or a
//! global target feature. The values are [`V256`]; insert, extract, and gather
//! operations also use [`V128`](super::sse2::V128). Immediate operands are
//! const generics unless documented otherwise.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::x86::simd::avx2::vpaddsb;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! assert_eq!(unsafe { vpaddsb(v, v) }.to_bytes(), [0; 32]);
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::x86::simd::avx2::*;
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
//! let _ = unsafe { v256_load(storage.0.as_ptr().cast()) };
//! let _ = unsafe { v256_store(storage.0.as_mut_ptr().cast(), v256) };
//! let _ = unsafe { vextracti128::<0>(v256) };
//! let _ = unsafe { vinserti128::<0>(v256, v128) };
//! let _ = unsafe { vpackssdw(v256, v256) };
//! let _ = unsafe { vpacksswb(v256, v256) };
//! let _ = unsafe { vpackusdw(v256, v256) };
//! let _ = unsafe { vpackuswb(v256, v256) };
//! let _ = unsafe { vpaddb(v256, v256) };
//! let _ = unsafe { vpaddd(v256, v256) };
//! let _ = unsafe { vpaddq(v256, v256) };
//! let _ = unsafe { vpaddsb(v256, v256) };
//! let _ = unsafe { vpaddsw(v256, v256) };
//! let _ = unsafe { vpaddusb(v256, v256) };
//! let _ = unsafe { vpaddusw(v256, v256) };
//! let _ = unsafe { vpaddw(v256, v256) };
//! let _ = unsafe { vpand(v256, v256) };
//! let _ = unsafe { vpblendd::<0>(v256, v256) };
//! let _ = unsafe { vpblendvb(v256, v256, v256) };
//! let _ = unsafe { vpblendw::<0>(v256, v256) };
//! let _ = unsafe { vpbroadcastb(0) };
//! let _ = unsafe { vpbroadcastd(0) };
//! let _ = unsafe { vpbroadcastq(0) };
//! let _ = unsafe { vpbroadcastw(0) };
//! let _ = unsafe { vpcmpeqb(v256, v256) };
//! let _ = unsafe { vpcmpeqd(v256, v256) };
//! let _ = unsafe { vpcmpeqw(v256, v256) };
//! let _ = unsafe { vperm2i128::<0>(v256, v256) };
//! let _ = unsafe { vpermd(v256, v256) };
//! let _ = unsafe { vpermps(v256, v256) };
//! let _ = unsafe { vpermq::<0>(v256) };
//! let _ = unsafe { vpgatherdd(v256, storage.0.as_ptr().cast(), v256, v256) };
//! let _ = unsafe { vpgatherdq(v256, storage.0.as_ptr().cast(), v128, v256) };
//! let _ = unsafe { vpgatherqd(v128, storage.0.as_ptr().cast(), v256, v128) };
//! let _ = unsafe { vpgatherqq(v256, storage.0.as_ptr().cast(), v256, v256) };
//! let _ = unsafe { vphaddd(v256, v256) };
//! let _ = unsafe { vphaddsw(v256, v256) };
//! let _ = unsafe { vphaddw(v256, v256) };
//! let _ = unsafe { vphsubd(v256, v256) };
//! let _ = unsafe { vphsubsw(v256, v256) };
//! let _ = unsafe { vphsubw(v256, v256) };
//! let _ = unsafe { vpmaddubsw(v256, v256) };
//! let _ = unsafe { vpmaddwd(v256, v256) };
//! let _ = unsafe { vpmaskmovd_load(storage.0.as_ptr().cast(), v256) };
//! let _ = unsafe { vpmaskmovd_store(storage.0.as_mut_ptr().cast(), v256, v256) };
//! let _ = unsafe { vpmaskmovq_load(storage.0.as_ptr().cast(), v256) };
//! let _ = unsafe { vpmaskmovq_store(storage.0.as_mut_ptr().cast(), v256, v256) };
//! let _ = unsafe { vpmaxsb(v256, v256) };
//! let _ = unsafe { vpmaxsd(v256, v256) };
//! let _ = unsafe { vpmaxsw(v256, v256) };
//! let _ = unsafe { vpmaxub(v256, v256) };
//! let _ = unsafe { vpmaxud(v256, v256) };
//! let _ = unsafe { vpminsb(v256, v256) };
//! let _ = unsafe { vpminsd(v256, v256) };
//! let _ = unsafe { vpminsw(v256, v256) };
//! let _ = unsafe { vpminub(v256, v256) };
//! let _ = unsafe { vpminud(v256, v256) };
//! let _ = unsafe { vpmovmskb(v256) };
//! let _ = unsafe { vpmulhrsw(v256, v256) };
//! let _ = unsafe { vpmulhuw(v256, v256) };
//! let _ = unsafe { vpmulhw(v256, v256) };
//! let _ = unsafe { vpmulld(v256, v256) };
//! let _ = unsafe { vpmullw(v256, v256) };
//! let _ = unsafe { vpor(v256, v256) };
//! let _ = unsafe { vpsadbw(v256, v256) };
//! let _ = unsafe { vpshufb(v256, v256) };
//! let _ = unsafe { vpshufd::<0>(v256) };
//! let _ = unsafe { vpshufhw::<0>(v256) };
//! let _ = unsafe { vpshuflw::<0>(v256) };
//! let _ = unsafe { vpsignb(v256, v256) };
//! let _ = unsafe { vpsignd(v256, v256) };
//! let _ = unsafe { vpsignw(v256, v256) };
//! let _ = unsafe { vpslld(v256, 0) };
//! let _ = unsafe { vpsllvd(v256, v256) };
//! let _ = unsafe { vpsllvq(v256, v256) };
//! let _ = unsafe { vpsravd(v256, v256) };
//! let _ = unsafe { vpsrld(v256, 0) };
//! let _ = unsafe { vpsrlvd(v256, v256) };
//! let _ = unsafe { vpsrlvq(v256, v256) };
//! let _ = unsafe { vpsubb(v256, v256) };
//! let _ = unsafe { vpsubd(v256, v256) };
//! let _ = unsafe { vpsubq(v256, v256) };
//! let _ = unsafe { vpsubsb(v256, v256) };
//! let _ = unsafe { vpsubsw(v256, v256) };
//! let _ = unsafe { vpsubusb(v256, v256) };
//! let _ = unsafe { vpsubusw(v256, v256) };
//! let _ = unsafe { vpsubw(v256, v256) };
//! let _ = unsafe { vpunpckhbw(v256, v256) };
//! let _ = unsafe { vpunpckhdq(v256, v256) };
//! let _ = unsafe { vpunpckhqdq(v256, v256) };
//! let _ = unsafe { vpunpckhwd(v256, v256) };
//! let _ = unsafe { vpunpcklbw(v256, v256) };
//! let _ = unsafe { vpunpckldq(v256, v256) };
//! let _ = unsafe { vpunpcklqdq(v256, v256) };
//! let _ = unsafe { vpunpcklwd(v256, v256) };
//! let _ = unsafe { vpxor(v256, v256) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx2::V256 as _;
//! use rust_isa::x86::simd::avx2::v256_load as _;
//! use rust_isa::x86::simd::avx2::v256_store as _;
//! use rust_isa::x86::simd::avx2::vextracti128 as _;
//! use rust_isa::x86::simd::avx2::vinserti128 as _;
//! use rust_isa::x86::simd::avx2::vpackssdw as _;
//! use rust_isa::x86::simd::avx2::vpacksswb as _;
//! use rust_isa::x86::simd::avx2::vpackusdw as _;
//! use rust_isa::x86::simd::avx2::vpackuswb as _;
//! use rust_isa::x86::simd::avx2::vpaddb as _;
//! use rust_isa::x86::simd::avx2::vpaddd as _;
//! use rust_isa::x86::simd::avx2::vpaddq as _;
//! use rust_isa::x86::simd::avx2::vpaddsb as _;
//! use rust_isa::x86::simd::avx2::vpaddsw as _;
//! use rust_isa::x86::simd::avx2::vpaddusb as _;
//! use rust_isa::x86::simd::avx2::vpaddusw as _;
//! use rust_isa::x86::simd::avx2::vpaddw as _;
//! use rust_isa::x86::simd::avx2::vpand as _;
//! use rust_isa::x86::simd::avx2::vpblendd as _;
//! use rust_isa::x86::simd::avx2::vpblendvb as _;
//! use rust_isa::x86::simd::avx2::vpblendw as _;
//! use rust_isa::x86::simd::avx2::vpbroadcastb as _;
//! use rust_isa::x86::simd::avx2::vpbroadcastd as _;
//! use rust_isa::x86::simd::avx2::vpbroadcastq as _;
//! use rust_isa::x86::simd::avx2::vpbroadcastw as _;
//! use rust_isa::x86::simd::avx2::vpcmpeqb as _;
//! use rust_isa::x86::simd::avx2::vpcmpeqd as _;
//! use rust_isa::x86::simd::avx2::vpcmpeqw as _;
//! use rust_isa::x86::simd::avx2::vperm2i128 as _;
//! use rust_isa::x86::simd::avx2::vpermd as _;
//! use rust_isa::x86::simd::avx2::vpermps as _;
//! use rust_isa::x86::simd::avx2::vpermq as _;
//! use rust_isa::x86::simd::avx2::vpgatherdd as _;
//! use rust_isa::x86::simd::avx2::vpgatherdq as _;
//! use rust_isa::x86::simd::avx2::vpgatherqd as _;
//! use rust_isa::x86::simd::avx2::vpgatherqq as _;
//! use rust_isa::x86::simd::avx2::vphaddd as _;
//! use rust_isa::x86::simd::avx2::vphaddsw as _;
//! use rust_isa::x86::simd::avx2::vphaddw as _;
//! use rust_isa::x86::simd::avx2::vphsubd as _;
//! use rust_isa::x86::simd::avx2::vphsubsw as _;
//! use rust_isa::x86::simd::avx2::vphsubw as _;
//! use rust_isa::x86::simd::avx2::vpmaddubsw as _;
//! use rust_isa::x86::simd::avx2::vpmaddwd as _;
//! use rust_isa::x86::simd::avx2::vpmaskmovd_load as _;
//! use rust_isa::x86::simd::avx2::vpmaskmovd_store as _;
//! use rust_isa::x86::simd::avx2::vpmaskmovq_load as _;
//! use rust_isa::x86::simd::avx2::vpmaskmovq_store as _;
//! use rust_isa::x86::simd::avx2::vpmaxsb as _;
//! use rust_isa::x86::simd::avx2::vpmaxsd as _;
//! use rust_isa::x86::simd::avx2::vpmaxsw as _;
//! use rust_isa::x86::simd::avx2::vpmaxub as _;
//! use rust_isa::x86::simd::avx2::vpmaxud as _;
//! use rust_isa::x86::simd::avx2::vpminsb as _;
//! use rust_isa::x86::simd::avx2::vpminsd as _;
//! use rust_isa::x86::simd::avx2::vpminsw as _;
//! use rust_isa::x86::simd::avx2::vpminub as _;
//! use rust_isa::x86::simd::avx2::vpminud as _;
//! use rust_isa::x86::simd::avx2::vpmovmskb as _;
//! use rust_isa::x86::simd::avx2::vpmulhrsw as _;
//! use rust_isa::x86::simd::avx2::vpmulhuw as _;
//! use rust_isa::x86::simd::avx2::vpmulhw as _;
//! use rust_isa::x86::simd::avx2::vpmulld as _;
//! use rust_isa::x86::simd::avx2::vpmullw as _;
//! use rust_isa::x86::simd::avx2::vpor as _;
//! use rust_isa::x86::simd::avx2::vpsadbw as _;
//! use rust_isa::x86::simd::avx2::vpshufb as _;
//! use rust_isa::x86::simd::avx2::vpshufd as _;
//! use rust_isa::x86::simd::avx2::vpshufhw as _;
//! use rust_isa::x86::simd::avx2::vpshuflw as _;
//! use rust_isa::x86::simd::avx2::vpsignb as _;
//! use rust_isa::x86::simd::avx2::vpsignd as _;
//! use rust_isa::x86::simd::avx2::vpsignw as _;
//! use rust_isa::x86::simd::avx2::vpslld as _;
//! use rust_isa::x86::simd::avx2::vpsllvd as _;
//! use rust_isa::x86::simd::avx2::vpsllvq as _;
//! use rust_isa::x86::simd::avx2::vpsravd as _;
//! use rust_isa::x86::simd::avx2::vpsrld as _;
//! use rust_isa::x86::simd::avx2::vpsrlvd as _;
//! use rust_isa::x86::simd::avx2::vpsrlvq as _;
//! use rust_isa::x86::simd::avx2::vpsubb as _;
//! use rust_isa::x86::simd::avx2::vpsubd as _;
//! use rust_isa::x86::simd::avx2::vpsubq as _;
//! use rust_isa::x86::simd::avx2::vpsubsb as _;
//! use rust_isa::x86::simd::avx2::vpsubsw as _;
//! use rust_isa::x86::simd::avx2::vpsubusb as _;
//! use rust_isa::x86::simd::avx2::vpsubusw as _;
//! use rust_isa::x86::simd::avx2::vpsubw as _;
//! use rust_isa::x86::simd::avx2::vpunpckhbw as _;
//! use rust_isa::x86::simd::avx2::vpunpckhdq as _;
//! use rust_isa::x86::simd::avx2::vpunpckhqdq as _;
//! use rust_isa::x86::simd::avx2::vpunpckhwd as _;
//! use rust_isa::x86::simd::avx2::vpunpcklbw as _;
//! use rust_isa::x86::simd::avx2::vpunpckldq as _;
//! use rust_isa::x86::simd::avx2::vpunpcklqdq as _;
//! use rust_isa::x86::simd::avx2::vpunpcklwd as _;
//! use rust_isa::x86::simd::avx2::vpxor as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

mod arithmetic;
mod bitwise;
mod blend_imm;
mod blend_variable;
mod broadcast;
mod compare;
mod gather;
mod horizontal;
mod insert_extract;
mod masked_memory;
mod memory;
mod movemask;
mod pack;
mod permute;
mod shift;
mod shuffle;
mod unpack;

pub use super::avx::V256;
pub use arithmetic::*;
pub use bitwise::*;
pub use blend_imm::*;
pub use blend_variable::*;
pub use broadcast::*;
pub use compare::*;
pub use gather::*;
pub use horizontal::*;
pub use insert_extract::*;
pub use masked_memory::*;
pub use memory::*;
pub use movemask::*;
pub use pack::*;
pub use permute::*;
pub use shift::*;
pub use shuffle::*;
pub use unpack::*;
