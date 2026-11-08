//! AVX10.1.
//!
//! AVX10.1 is not part of the x86-64 baseline and rustc does not allow
//! `#[target_feature(enable = "avx10.1")]` on stable. The caller must compile
//! the whole crate with `-C target-feature=+avx10.1` and the CPU must support
//! AVX10.1. The wrappers do not carry `#[target_feature]`. Every wrapper is
//! `unsafe`. Immediate operands are const generics unless documented otherwise.
//!
//! The values are [`V128`], [`V256`], and [`V512`]. The module covers the
//! integer, mask, permute, compress/expand, gather/scatter, conversion, reciprocal,
//! square-root, FMA, VNNI, IFMA, population-count, leading-zero, and conflict
//! instructions. Embedded rounding control and the vector-length selection
//! bits are omitted.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The build compiles the bundled C with clang; some wrappers use `asm!`.

//! # Examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx10_1::vaddps_v256;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vaddps_v256(v, v) };
//! ```
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::x86::simd::avx10_1::*;
//! use rust_isa::x86::simd::{Mask4, Mask8, Mask16, Mask32, Mask64};
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! assert_eq!(unsafe { vaddps_v256(v, v) }.to_bytes(), [0; 32]);
//! ```
//!
//! # Compile-time predicate check
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx10_1::vcmpps_v256;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vcmpps_v256::<32>(v, v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx10_1::vcmpps_v512;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vcmpps_v512::<32>(v, v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx10_1::vcmppd_v256;
//! use rust_isa::x86::simd::avx::V256;
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { vcmppd_v256::<32>(v, v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::x86::simd::avx10_1::vcmppd_v512;
//! use rust_isa::x86::simd::avx512f::V512;
//!
//! let v = V512::from_bytes([0; 64]);
//! let _ = unsafe { vcmppd_v512::<32>(v, v) };
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::x86::simd::avx10_1::*;
//! use rust_isa::x86::simd::sse2::V128;
//! use rust_isa::x86::simd::avx::V256;
//! use rust_isa::x86::simd::avx512f::V512;
//! use rust_isa::x86::simd::{Mask4, Mask8, Mask16, Mask32, Mask64};
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! let v512 = V512::from_bytes([0; 64]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { vaddps_v256(v256, v256) };
//! let _ = unsafe { vaddps_v512(v512, v512) };
//! let _ = unsafe { vcmppd_v256::<0>(v256, v256) };
//! let _ = unsafe { vcmppd_v512::<0>(v512, v512) };
//! let _ = unsafe { vcmpps_v256::<0>(v256, v256) };
//! let _ = unsafe { vcmpps_v512::<0>(v512, v512) };
//! let _ = unsafe { vcompresspd_v256(v256, Mask4::from_bits(0)) };
//! let _ = unsafe { vcompresspd_v512(v512, Mask8::from_bits(0)) };
//! let _ = unsafe { vcompressps_v256(v256, Mask8::from_bits(0)) };
//! let _ = unsafe { vcompressps_v512(v512, Mask16::from_bits(0)) };
//! let _ = unsafe { vcvtdq2ps_v256(v256) };
//! let _ = unsafe { vcvtdq2ps_v512(v512) };
//! let _ = unsafe { vcvtpd2dq_v512(v512) };
//! let _ = unsafe { vcvtpd2ps_v512(v512) };
//! let _ = unsafe { vcvtpd2qq_v256(v256) };
//! let _ = unsafe { vcvtpd2qq_v512(v512) };
//! let _ = unsafe { vcvtpd2uqq_v256(v256) };
//! let _ = unsafe { vcvtpd2uqq_v512(v512) };
//! let _ = unsafe { vcvtps2dq_v256(v256) };
//! let _ = unsafe { vcvtps2dq_v512(v512) };
//! let _ = unsafe { vcvtps2pd_v256(v256) };
//! let _ = unsafe { vcvtps2qq_v256(v128) };
//! let _ = unsafe { vcvtps2qq_v512(v256) };
//! let _ = unsafe { vcvtps2uqq_v256(v128) };
//! let _ = unsafe { vcvtps2uqq_v512(v256) };
//! let _ = unsafe { vcvtqq2pd_v256(v256) };
//! let _ = unsafe { vcvtqq2pd_v512(v512) };
//! let _ = unsafe { vcvtqq2ps_v256(v256) };
//! let _ = unsafe { vcvtqq2ps_v512(v512) };
//! let _ = unsafe { vcvttpd2dq_v512(v512) };
//! let _ = unsafe { vcvttpd2qq_v256(v256) };
//! let _ = unsafe { vcvttpd2qq_v512(v512) };
//! let _ = unsafe { vcvttpd2uqq_v256(v256) };
//! let _ = unsafe { vcvttpd2uqq_v512(v512) };
//! let _ = unsafe { vcvttps2dq_v256(v256) };
//! let _ = unsafe { vcvttps2dq_v512(v512) };
//! let _ = unsafe { vcvttps2qq_v256(v128) };
//! let _ = unsafe { vcvttps2qq_v512(v256) };
//! let _ = unsafe { vcvttps2uqq_v256(v128) };
//! let _ = unsafe { vcvttps2uqq_v512(v256) };
//! let _ = unsafe { vcvtuqq2pd_v256(v256) };
//! let _ = unsafe { vcvtuqq2pd_v512(v512) };
//! let _ = unsafe { vcvtuqq2ps_v256(v256) };
//! let _ = unsafe { vcvtuqq2ps_v512(v512) };
//! let _ = unsafe { vexpandpd_v256(v256, Mask4::from_bits(0)) };
//! let _ = unsafe { vexpandpd_v512(v512, Mask8::from_bits(0)) };
//! let _ = unsafe { vexpandps_v256(v256, Mask8::from_bits(0)) };
//! let _ = unsafe { vexpandps_v512(v512, Mask16::from_bits(0)) };
//! let _ = unsafe { vfmadd132pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmadd132pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmadd132ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmadd132ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmadd213pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmadd213pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmadd213ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmadd213ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmadd231pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmadd231pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmadd231ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmadd231ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmaddsub132pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmaddsub132pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmaddsub132ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmaddsub132ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmaddsub213pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmaddsub213pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmaddsub213ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmaddsub213ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmaddsub231pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmaddsub231pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmaddsub231ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmaddsub231ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsub213pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsub213pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsub213ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsub213ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsub231pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsub231pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsub231ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsub231ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsubadd132pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsubadd132pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsubadd132ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsubadd132ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsubadd213pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsubadd213pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsubadd213ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsubadd213ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsubadd231pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsubadd231pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfmsubadd231ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfmsubadd231ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmadd132pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmadd132pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmadd132ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmadd132ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmadd213pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmadd213pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmadd213ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmadd213ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmadd231pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmadd231pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmadd231ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmadd231ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmsub132pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmsub132pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmsub132ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmsub132ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmsub213pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmsub213pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmsub213ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmsub213ps_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmsub231pd_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmsub231pd_v512(v512, v512, v512) };
//! let _ = unsafe { vfnmsub231ps_v256(v256, v256, v256) };
//! let _ = unsafe { vfnmsub231ps_v512(v512, v512, v512) };
//! let _ = unsafe { vgatherdpd_v256(v256, storage.0.as_ptr().cast(), v128, Mask4::from_bits(0)) };
//! let _ = unsafe { vgatherdpd_v512(v512, storage.0.as_ptr().cast(), v256, Mask8::from_bits(0)) };
//! let _ = unsafe { vgatherdps_v256(v256, storage.0.as_ptr().cast(), v256, Mask8::from_bits(0)) };
//! let _ = unsafe { vgatherdps_v512(v512, storage.0.as_ptr().cast(), v512, Mask16::from_bits(0)) };
//! let _ = unsafe { vgatherqpd_v256(v256, storage.0.as_ptr().cast(), v256, Mask4::from_bits(0)) };
//! let _ = unsafe { vgatherqpd_v512(v512, storage.0.as_ptr().cast(), v512, Mask8::from_bits(0)) };
//! let _ = unsafe { vgatherqps_v256(v128, storage.0.as_ptr().cast(), v256, Mask4::from_bits(0)) };
//! let _ = unsafe { vgatherqps_v512(v256, storage.0.as_ptr().cast(), v512, Mask8::from_bits(0)) };
//! let _ = unsafe { vmulps_v256(v256, v256) };
//! let _ = unsafe { vmulps_v512(v512, v512) };
//! let _ = unsafe { vpaddb_v256(v256, v256) };
//! let _ = unsafe { vpaddb_v512(v512, v512) };
//! let _ = unsafe { vpaddd_v256(v256, v256) };
//! let _ = unsafe { vpaddd_v512(v512, v512) };
//! let _ = unsafe { vpaddq_v256(v256, v256) };
//! let _ = unsafe { vpaddq_v512(v512, v512) };
//! let _ = unsafe { vpaddw_v256(v256, v256) };
//! let _ = unsafe { vpaddw_v512(v512, v512) };
//! let _ = unsafe { vpandd_v256(v256, v256) };
//! let _ = unsafe { vpandd_v512(v512, v512) };
//! let _ = unsafe { vpandnd_v256(v256, v256) };
//! let _ = unsafe { vpandnd_v512(v512, v512) };
//! let _ = unsafe { vpandnq_v256(v256, v256) };
//! let _ = unsafe { vpandnq_v512(v512, v512) };
//! let _ = unsafe { vpandq_v256(v256, v256) };
//! let _ = unsafe { vpandq_v512(v512, v512) };
//! let _ = unsafe { vpcmpb_v256::<0>(v256, v256) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { vpcmpb_v512::<0>(v512, v512) };
//! let _ = unsafe { vpcmpd_v256::<0>(v256, v256) };
//! let _ = unsafe { vpcmpd_v512::<0>(v512, v512) };
//! let _ = unsafe { vpcmpq_v256::<0>(v256, v256) };
//! let _ = unsafe { vpcmpq_v512::<0>(v512, v512) };
//! let _ = unsafe { vpcmpub_v256::<0>(v256, v256) };
//! #[cfg(target_arch = "x86_64")] let _ = unsafe { vpcmpub_v512::<0>(v512, v512) };
//! let _ = unsafe { vpcmpud_v256::<0>(v256, v256) };
//! let _ = unsafe { vpcmpud_v512::<0>(v512, v512) };
//! let _ = unsafe { vpcmpuq_v256::<0>(v256, v256) };
//! let _ = unsafe { vpcmpuq_v512::<0>(v512, v512) };
//! let _ = unsafe { vpcmpuw_v256::<0>(v256, v256) };
//! let _ = unsafe { vpcmpuw_v512::<0>(v512, v512) };
//! let _ = unsafe { vpcmpw_v256::<0>(v256, v256) };
//! let _ = unsafe { vpcmpw_v512::<0>(v512, v512) };
//! let _ = unsafe { vpcompressb_v256(v256, Mask32::from_bits(0)) };
//! let _ = unsafe { vpcompressb_v512(v512, Mask64::from_bits(0)) };
//! let _ = unsafe { vpcompressw_v256(v256, Mask16::from_bits(0)) };
//! let _ = unsafe { vpcompressw_v512(v512, Mask32::from_bits(0)) };
//! let _ = unsafe { vpconflictd_v256(v256) };
//! let _ = unsafe { vpconflictd_v512(v512) };
//! let _ = unsafe { vpconflictq_v256(v256) };
//! let _ = unsafe { vpconflictq_v512(v512) };
//! let _ = unsafe { vpdpbusd_v256(v256, v256, v256) };
//! let _ = unsafe { vpdpbusd_v512(v512, v512, v512) };
//! let _ = unsafe { vpdpbusds_v256(v256, v256, v256) };
//! let _ = unsafe { vpdpbusds_v512(v512, v512, v512) };
//! let _ = unsafe { vpdpwssd_v256(v256, v256, v256) };
//! let _ = unsafe { vpdpwssd_v512(v512, v512, v512) };
//! let _ = unsafe { vpdpwssds_v256(v256, v256, v256) };
//! let _ = unsafe { vpdpwssds_v512(v512, v512, v512) };
//! let _ = unsafe { vpermb_v256(v256, v256) };
//! let _ = unsafe { vpermb_v512(v512, v512) };
//! let _ = unsafe { vpermd_v256(v256, v256) };
//! let _ = unsafe { vpermd_v512(v512, v512) };
//! let _ = unsafe { vpermi2b_v256(v256, v256, v256) };
//! let _ = unsafe { vpermi2b_v512(v512, v512, v512) };
//! let _ = unsafe { vpermi2d_v256(v256, v256, v256) };
//! let _ = unsafe { vpermi2d_v512(v512, v512, v512) };
//! let _ = unsafe { vpermi2q_v256(v256, v256, v256) };
//! let _ = unsafe { vpermi2q_v512(v512, v512, v512) };
//! let _ = unsafe { vpermi2w_v256(v256, v256, v256) };
//! let _ = unsafe { vpermi2w_v512(v512, v512, v512) };
//! let _ = unsafe { vpermq_v256(v256, v256) };
//! let _ = unsafe { vpermq_v512(v512, v512) };
//! let _ = unsafe { vpermt2b_v256(v256, v256, v256) };
//! let _ = unsafe { vpermt2b_v512(v512, v512, v512) };
//! let _ = unsafe { vpermt2d_v256(v256, v256, v256) };
//! let _ = unsafe { vpermt2d_v512(v512, v512, v512) };
//! let _ = unsafe { vpermt2q_v256(v256, v256, v256) };
//! let _ = unsafe { vpermt2q_v512(v512, v512, v512) };
//! let _ = unsafe { vpermt2w_v256(v256, v256, v256) };
//! let _ = unsafe { vpermt2w_v512(v512, v512, v512) };
//! let _ = unsafe { vpexpandb_v256(v256, Mask32::from_bits(0)) };
//! let _ = unsafe { vpexpandb_v512(v512, Mask64::from_bits(0)) };
//! let _ = unsafe { vpexpandw_v256(v256, Mask16::from_bits(0)) };
//! let _ = unsafe { vpexpandw_v512(v512, Mask32::from_bits(0)) };
//! let _ = unsafe { vplzcntd_v256(v256) };
//! let _ = unsafe { vplzcntd_v512(v512) };
//! let _ = unsafe { vplzcntq_v256(v256) };
//! let _ = unsafe { vplzcntq_v512(v512) };
//! let _ = unsafe { vpmadd52huq_v256(v256, v256, v256) };
//! let _ = unsafe { vpmadd52huq_v512(v512, v512, v512) };
//! let _ = unsafe { vpmadd52luq_v256(v256, v256, v256) };
//! let _ = unsafe { vpmadd52luq_v512(v512, v512, v512) };
//! let _ = unsafe { vpmaddubsw_v256(v256, v256) };
//! let _ = unsafe { vpmaddubsw_v512(v512, v512) };
//! let _ = unsafe { vpmaddwd_v256(v256, v256) };
//! let _ = unsafe { vpmaddwd_v512(v512, v512) };
//! let _ = unsafe { vpmulhuw_v256(v256, v256) };
//! let _ = unsafe { vpmulhuw_v512(v512, v512) };
//! let _ = unsafe { vpmulhw_v256(v256, v256) };
//! let _ = unsafe { vpmulhw_v512(v512, v512) };
//! let _ = unsafe { vpmulld_v256(v256, v256) };
//! let _ = unsafe { vpmulld_v512(v512, v512) };
//! let _ = unsafe { vpmullq_v256(v256, v256) };
//! let _ = unsafe { vpmullq_v512(v512, v512) };
//! let _ = unsafe { vpmullw_v256(v256, v256) };
//! let _ = unsafe { vpmullw_v512(v512, v512) };
//! let _ = unsafe { vpopcntb_v256(v256) };
//! let _ = unsafe { vpopcntb_v512(v512) };
//! let _ = unsafe { vpopcntd_v256(v256) };
//! let _ = unsafe { vpopcntd_v512(v512) };
//! let _ = unsafe { vpopcntq_v256(v256) };
//! let _ = unsafe { vpopcntq_v512(v512) };
//! let _ = unsafe { vpopcntw_v256(v256) };
//! let _ = unsafe { vpopcntw_v512(v512) };
//! let _ = unsafe { vpord_v256(v256, v256) };
//! let _ = unsafe { vpord_v512(v512, v512) };
//! let _ = unsafe { vporq_v256(v256, v256) };
//! let _ = unsafe { vporq_v512(v512, v512) };
//! let _ = unsafe { vpshldd_v256::<0>(v256, v256) };
//! let _ = unsafe { vpshldd_v512::<0>(v512, v512) };
//! let _ = unsafe { vpshldq_v256::<0>(v256, v256) };
//! let _ = unsafe { vpshldq_v512::<0>(v512, v512) };
//! let _ = unsafe { vpshldw_v256::<0>(v256, v256) };
//! let _ = unsafe { vpshldw_v512::<0>(v512, v512) };
//! let _ = unsafe { vpshrdd_v256::<0>(v256, v256) };
//! let _ = unsafe { vpshrdd_v512::<0>(v512, v512) };
//! let _ = unsafe { vpshrdq_v256::<0>(v256, v256) };
//! let _ = unsafe { vpshrdq_v512::<0>(v512, v512) };
//! let _ = unsafe { vpshrdw_v256::<0>(v256, v256) };
//! let _ = unsafe { vpshrdw_v512::<0>(v512, v512) };
//! let _ = unsafe { vpsubb_v256(v256, v256) };
//! let _ = unsafe { vpsubb_v512(v512, v512) };
//! let _ = unsafe { vpsubd_v256(v256, v256) };
//! let _ = unsafe { vpsubd_v512(v512, v512) };
//! let _ = unsafe { vpsubq_v256(v256, v256) };
//! let _ = unsafe { vpsubq_v512(v512, v512) };
//! let _ = unsafe { vpsubw_v256(v256, v256) };
//! let _ = unsafe { vpsubw_v512(v512, v512) };
//! let _ = unsafe { vptestmb_v256(v256, v256) };
//! let _ = unsafe { vptestmb_v512(v512, v512) };
//! let _ = unsafe { vptestmd_v256(v256, v256) };
//! let _ = unsafe { vptestmd_v512(v512, v512) };
//! let _ = unsafe { vptestmq_v256(v256, v256) };
//! let _ = unsafe { vptestmq_v512(v512, v512) };
//! let _ = unsafe { vptestmw_v256(v256, v256) };
//! let _ = unsafe { vptestmw_v512(v512, v512) };
//! let _ = unsafe { vptestnmb_v256(v256, v256) };
//! let _ = unsafe { vptestnmb_v512(v512, v512) };
//! let _ = unsafe { vptestnmd_v256(v256, v256) };
//! let _ = unsafe { vptestnmd_v512(v512, v512) };
//! let _ = unsafe { vptestnmq_v256(v256, v256) };
//! let _ = unsafe { vptestnmq_v512(v512, v512) };
//! let _ = unsafe { vptestnmw_v256(v256, v256) };
//! let _ = unsafe { vptestnmw_v512(v512, v512) };
//! let _ = unsafe { vpxord_v256(v256, v256) };
//! let _ = unsafe { vpxord_v512(v512, v512) };
//! let _ = unsafe { vpxorq_v256(v256, v256) };
//! let _ = unsafe { vpxorq_v512(v512, v512) };
//! let _ = unsafe { vrcp14pd_v256(v256) };
//! let _ = unsafe { vrcp14pd_v512(v512) };
//! let _ = unsafe { vrcp14ps_v256(v256) };
//! let _ = unsafe { vrcp14ps_v512(v512) };
//! let _ = unsafe { vrsqrt14pd_v256(v256) };
//! let _ = unsafe { vrsqrt14pd_v512(v512) };
//! let _ = unsafe { vrsqrt14ps_v256(v256) };
//! let _ = unsafe { vrsqrt14ps_v512(v512) };
//! let _ = unsafe { vscatterdpd_v256(storage.0.as_mut_ptr().cast(), v128, v256, Mask4::from_bits(0)) };
//! let _ = unsafe { vscatterdpd_v512(storage.0.as_mut_ptr().cast(), v256, v512, Mask8::from_bits(0)) };
//! let _ = unsafe { vscatterdps_v256(storage.0.as_mut_ptr().cast(), v256, v256, Mask8::from_bits(0)) };
//! let _ = unsafe { vscatterdps_v512(storage.0.as_mut_ptr().cast(), v512, v512, Mask16::from_bits(0)) };
//! let _ = unsafe { vscatterqpd_v256(storage.0.as_mut_ptr().cast(), v256, v256, Mask4::from_bits(0)) };
//! let _ = unsafe { vscatterqpd_v512(storage.0.as_mut_ptr().cast(), v512, v512, Mask8::from_bits(0)) };
//! let _ = unsafe { vscatterqps_v256(storage.0.as_mut_ptr().cast(), v256, v128, Mask4::from_bits(0)) };
//! let _ = unsafe { vscatterqps_v512(storage.0.as_mut_ptr().cast(), v512, v256, Mask8::from_bits(0)) };
//! let _ = unsafe { vsqrtpd_v256(v256) };
//! let _ = unsafe { vsqrtpd_v512(v512) };
//! let _ = unsafe { vsqrtps_v256(v256) };
//! let _ = unsafe { vsqrtps_v512(v512) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::simd::avx10_1::vaddps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vaddps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcmppd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcmppd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcmpps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcmpps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcompresspd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcompresspd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcompressps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcompressps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtdq2ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtdq2ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtpd2dq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtpd2ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtpd2qq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtpd2qq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtpd2uqq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtpd2uqq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtps2dq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtps2dq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtps2pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtps2qq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtps2qq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtps2uqq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtps2uqq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtqq2pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtqq2pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtqq2ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtqq2ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttpd2dq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttpd2qq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttpd2qq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttpd2uqq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttpd2uqq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttps2dq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttps2dq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttps2qq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttps2qq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttps2uqq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvttps2uqq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtuqq2pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtuqq2pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtuqq2ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vcvtuqq2ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vexpandpd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vexpandpd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vexpandps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vexpandps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd132pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd132pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd132ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd132ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd213pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd213pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd213ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd213ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd231pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd231pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd231ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmadd231ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub132pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub132pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub132ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub132ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub213pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub213pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub213ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub213ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub231pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub231pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub231ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmaddsub231ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsub213pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsub213pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsub213ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsub213ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsub231pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsub231pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsub231ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsub231ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd132pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd132pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd132ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd132ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd213pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd213pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd213ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd213ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd231pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd231pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd231ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfmsubadd231ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd132pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd132pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd132ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd132ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd213pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd213pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd213ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd213ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd231pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd231pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd231ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmadd231ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub132pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub132pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub132ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub132ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub213pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub213pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub213ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub213ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub231pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub231pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub231ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vfnmsub231ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vgatherdpd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vgatherdpd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vgatherdps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vgatherdps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vgatherqpd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vgatherqpd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vgatherqps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vgatherqps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vmulps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vmulps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpaddb_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpaddb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpaddd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpaddd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpaddq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpaddq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpaddw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpaddw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpandd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpandd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpandnd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpandnd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpandnq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpandnq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpandq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpandq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpb_v256 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::simd::avx10_1::vpcmpb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpub_v256 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::simd::avx10_1::vpcmpub_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpud_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpud_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpuq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpuq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpuw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpuw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpcmpw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcompressb_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpcompressb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpcompressw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpcompressw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpconflictd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpconflictd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpconflictq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpconflictq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpdpbusd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpdpbusd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpdpbusds_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpdpbusds_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpdpwssd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpdpwssd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpdpwssds_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpdpwssds_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermb_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermi2b_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermi2b_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermi2d_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermi2d_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermi2q_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermi2q_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermi2w_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermi2w_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermt2b_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermt2b_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermt2d_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermt2d_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermt2q_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermt2q_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpermt2w_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpermt2w_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpexpandb_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpexpandb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpexpandw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpexpandw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vplzcntd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vplzcntd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vplzcntq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vplzcntq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmadd52huq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmadd52huq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmadd52luq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmadd52luq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmaddubsw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmaddubsw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmaddwd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmaddwd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmulhuw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmulhuw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmulhw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmulhw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmulld_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmulld_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmullq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmullq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpmullw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpmullw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpopcntb_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpopcntb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpopcntd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpopcntd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpopcntq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpopcntq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpopcntw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpopcntw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpord_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpord_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vporq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vporq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpshldd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpshldd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpshldq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpshldq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpshldw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpshldw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpshrdd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpshrdd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpshrdq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpshrdq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpshrdw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpshrdw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpsubb_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpsubb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpsubd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpsubd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpsubq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpsubq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpsubw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpsubw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vptestmb_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vptestmb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vptestmd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vptestmd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vptestmq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vptestmq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vptestmw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vptestmw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vptestnmb_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vptestnmb_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vptestnmd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vptestnmd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vptestnmq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vptestnmq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vptestnmw_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vptestnmw_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpxord_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpxord_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vpxorq_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vpxorq_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vrcp14pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vrcp14pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vrcp14ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vrcp14ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vrsqrt14pd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vrsqrt14pd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vrsqrt14ps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vrsqrt14ps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vscatterdpd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vscatterdpd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vscatterdps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vscatterdps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vscatterqpd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vscatterqpd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vscatterqps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vscatterqps_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vsqrtpd_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vsqrtpd_v512 as _;
//! use rust_isa::x86::simd::avx10_1::vsqrtps_v256 as _;
//! use rust_isa::x86::simd::avx10_1::vsqrtps_v512 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

use crate::x86::simd::avx2::V256;
use crate::x86::simd::avx512f::V512;
use crate::x86::simd::sse2::V128;
use crate::x86::simd::{Mask4, Mask8, Mask16, Mask32, Mask64};
use core::arch::asm;

unsafe extern "C" {
    fn rust_isa_avx10_1_vaddps_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaddps_v256`
/// Add the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vaddps_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vaddps_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vaddps_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vaddps_v512`
/// Add the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vaddps_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vaddps_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vmulps_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vmulps_v256`
/// Multiply the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vmulps_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vmulps_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vmulps_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vmulps_v512`
/// Multiply the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vmulps_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vmulps_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpaddb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddb_v256`
/// Add the 8-bit lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpaddb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpaddb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpaddb_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddb_v512`
/// Add the 8-bit lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpaddb_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpaddb_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpaddw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddw_v256`
/// Add the 16-bit lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpaddw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpaddw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpaddw_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddw_v512`
/// Add the 16-bit lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpaddw_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpaddw_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpaddd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddd_v256`
/// Add the 32-bit lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpaddd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpaddd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpaddd_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddd_v512`
/// Add the 32-bit lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpaddd_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpaddd_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpaddq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddq_v256`
/// Add the 64-bit lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpaddq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpaddq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpaddq_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpaddq_v512`
/// Add the 64-bit lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpaddq_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpaddq_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpsubb_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubb_v256`
/// Subtract the 8-bit lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpsubb_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpsubb_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpsubb_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubb_v512`
/// Subtract the 8-bit lanes, wrapping at 8 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpsubb_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpsubb_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpsubw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubw_v256`
/// Subtract the 16-bit lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpsubw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpsubw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpsubw_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubw_v512`
/// Subtract the 16-bit lanes, wrapping at 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpsubw_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpsubw_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpsubd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubd_v256`
/// Subtract the 32-bit lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpsubd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpsubd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpsubd_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubd_v512`
/// Subtract the 32-bit lanes, wrapping at 32 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpsubd_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpsubd_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpsubq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubq_v256`
/// Subtract the 64-bit lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpsubq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpsubq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpsubq_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpsubq_v512`
/// Subtract the 64-bit lanes, wrapping at 64 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpsubq_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpsubq_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmullw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullw_v256`
/// Multiply the 16-bit lanes, keeping the low 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmullw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmullw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmullw_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullw_v512`
/// Multiply the 16-bit lanes, keeping the low 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmullw_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmullw_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmulld_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulld_v256`
/// Multiply the 32-bit lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmulld_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmulld_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmulld_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulld_v512`
/// Multiply the 32-bit lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmulld_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmulld_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmullq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullq_v256`
/// Multiply the 64-bit lanes, keeping the low 64 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmullq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmullq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmullq_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmullq_v512`
/// Multiply the 64-bit lanes, keeping the low 64 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmullq_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmullq_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmulhw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhw_v256`
/// Multiply the signed 16-bit lanes and keep the high 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmulhw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmulhw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmulhw_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhw_v512`
/// Multiply the signed 16-bit lanes and keep the high 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmulhw_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmulhw_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmulhuw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhuw_v256`
/// Multiply the unsigned 16-bit lanes and keep the high 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmulhuw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmulhuw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmulhuw_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmulhuw_v512`
/// Multiply the unsigned 16-bit lanes and keep the high 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmulhuw_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmulhuw_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmaddwd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddwd_v256`
/// Multiply signed 16-bit lane pairs, add each pair, and store into 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmaddwd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmaddwd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmaddwd_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddwd_v512`
/// Multiply signed 16-bit lane pairs, add each pair, and store into 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmaddwd_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmaddwd_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmaddubsw_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddubsw_v256`
/// Multiply unsigned bytes by signed bytes, add adjacent products, and saturate to signed 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmaddubsw_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmaddubsw_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmaddubsw_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmaddubsw_v512`
/// Multiply unsigned bytes by signed bytes, add adjacent products, and saturate to signed 16 bits.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmaddubsw_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmaddubsw_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpandd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandd_v256`
/// Bitwise AND.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpandd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpandd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpandd_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandd_v512`
/// Bitwise AND.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpandd_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpandd_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpandq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandq_v256`
/// Bitwise AND.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpandq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpandq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpandq_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandq_v512`
/// Bitwise AND.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpandq_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpandq_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpandnd_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnd_v256`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpandnd_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpandnd_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpandnd_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnd_v512`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpandnd_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpandnd_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpandnq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnq_v256`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpandnq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpandnq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpandnq_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpandnq_v512`
/// Bitwise AND of `b` with the complement of `a`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpandnq_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpandnq_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpord_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpord_v256`
/// Bitwise OR.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpord_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpord_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpord_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpord_v512`
/// Bitwise OR.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpord_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpord_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vporq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vporq_v256`
/// Bitwise OR.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vporq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vporq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vporq_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vporq_v512`
/// Bitwise OR.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vporq_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vporq_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpxord_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxord_v256`
/// Bitwise XOR.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpxord_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpxord_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpxord_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxord_v512`
/// Bitwise XOR.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpxord_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpxord_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpxorq_v256(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxorq_v256`
/// Bitwise XOR.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpxorq_v256(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpxorq_v256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpxorq_v512(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpxorq_v512`
/// Bitwise XOR.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpxorq_v512(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpxorq_v512(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpcmpb_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpb_v256<const IMM: u8>(a: V256, b: V256) -> Mask32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpb k1, ymm0, ymm1, {imm}",
            "kmovd {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask32::from_bits(out as u32)
}

/// `vpcmpb_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
#[cfg(target_arch = "x86_64")]
pub unsafe fn vpcmpb_v512<const IMM: u8>(a: V512, b: V512) -> Mask64 {
    let out: u64;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpb k1, zmm0, zmm1, {imm}",
            "kmovq {out:r}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask64::from_bits(out)
}

/// `vpcmpub_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpub_v256<const IMM: u8>(a: V256, b: V256) -> Mask32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpub k1, ymm0, ymm1, {imm}",
            "kmovd {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask32::from_bits(out as u32)
}

/// `vpcmpub_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
#[cfg(target_arch = "x86_64")]
pub unsafe fn vpcmpub_v512<const IMM: u8>(a: V512, b: V512) -> Mask64 {
    let out: u64;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpub k1, zmm0, zmm1, {imm}",
            "kmovq {out:r}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask64::from_bits(out)
}

/// `vpcmpw_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpw_v256<const IMM: u8>(a: V256, b: V256) -> Mask16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpw k1, ymm0, ymm1, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask16::from_bits(out as u16)
}

/// `vpcmpw_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpw_v512<const IMM: u8>(a: V512, b: V512) -> Mask32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpw k1, zmm0, zmm1, {imm}",
            "kmovd {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask32::from_bits(out as u32)
}

/// `vpcmpuw_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpuw_v256<const IMM: u8>(a: V256, b: V256) -> Mask16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpuw k1, ymm0, ymm1, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask16::from_bits(out as u16)
}

/// `vpcmpuw_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpuw_v512<const IMM: u8>(a: V512, b: V512) -> Mask32 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpuw k1, zmm0, zmm1, {imm}",
            "kmovd {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask32::from_bits(out as u32)
}

/// `vpcmpd_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpd_v256<const IMM: u8>(a: V256, b: V256) -> Mask8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpd k1, ymm0, ymm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask8::from_bits(out as u8)
}

/// `vpcmpd_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpd_v512<const IMM: u8>(a: V512, b: V512) -> Mask16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpd k1, zmm0, zmm1, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask16::from_bits(out as u16)
}

/// `vpcmpud_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpud_v256<const IMM: u8>(a: V256, b: V256) -> Mask8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpud k1, ymm0, ymm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask8::from_bits(out as u8)
}

/// `vpcmpud_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpud_v512<const IMM: u8>(a: V512, b: V512) -> Mask16 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpud k1, zmm0, zmm1, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask16::from_bits(out as u16)
}

/// `vpcmpq_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpq_v256<const IMM: u8>(a: V256, b: V256) -> Mask4 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpq k1, ymm0, ymm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask4::from_bits(out as u8)
}

/// `vpcmpq_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpq_v512<const IMM: u8>(a: V512, b: V512) -> Mask8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpq k1, zmm0, zmm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask8::from_bits(out as u8)
}

/// `vpcmpuq_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpuq_v256<const IMM: u8>(a: V256, b: V256) -> Mask4 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpcmpuq k1, ymm0, ymm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask4::from_bits(out as u8)
}

/// `vpcmpuq_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcmpuq_v512<const IMM: u8>(a: V512, b: V512) -> Mask8 {
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpcmpuq k1, zmm0, zmm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask8::from_bits(out as u8)
}

/// `vcmpps_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask. `IMM` must be in `0..=31`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmpps_v256<const IMM: u8>(a: V256, b: V256) -> Mask8 {
    const {
        assert!(IMM <= 31, "vcmp predicate must be in 0..=31");
    }
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vcmpps k1, ymm0, ymm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask8::from_bits(out as u8)
}

/// `vcmpps_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask. `IMM` must be in `0..=31`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmpps_v512<const IMM: u8>(a: V512, b: V512) -> Mask16 {
    const {
        assert!(IMM <= 31, "vcmp predicate must be in 0..=31");
    }
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vcmpps k1, zmm0, zmm1, {imm}",
            "kmovw {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask16::from_bits(out as u16)
}

/// `vcmppd_v256`
/// Compare the lanes with the predicate in `IMM` and return the mask. `IMM` must be in `0..=31`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmppd_v256<const IMM: u8>(a: V256, b: V256) -> Mask4 {
    const {
        assert!(IMM <= 31, "vcmp predicate must be in 0..=31");
    }
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vcmppd k1, ymm0, ymm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("ymm0") _,
            out("ymm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask4::from_bits(out as u8)
}

/// `vcmppd_v512`
/// Compare the lanes with the predicate in `IMM` and return the mask. `IMM` must be in `0..=31`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcmppd_v512<const IMM: u8>(a: V512, b: V512) -> Mask8 {
    const {
        assert!(IMM <= 31, "vcmp predicate must be in 0..=31");
    }
    let out: u32;
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vcmppd k1, zmm0, zmm1, {imm}",
            "kmovb {out:e}, k1",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = out(reg) out,
            out("zmm0") _,
            out("zmm1") _,
            out("k1") _,
            options(nostack, preserves_flags),
        );
    }
    Mask8::from_bits(out as u8)
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestmb_v256(a: *const u8, b: *const u8) -> u32;
}

/// `vptestmb_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestmb_v256(a: V256, b: V256) -> Mask32 {
    Mask32::from_bits(unsafe { rust_isa_avx10_1_vptestmb_v256(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestmb_v512(a: *const u8, b: *const u8) -> u64;
}

/// `vptestmb_v512`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestmb_v512(a: V512, b: V512) -> Mask64 {
    Mask64::from_bits(unsafe { rust_isa_avx10_1_vptestmb_v512(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestmw_v256(a: *const u8, b: *const u8) -> u16;
}

/// `vptestmw_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestmw_v256(a: V256, b: V256) -> Mask16 {
    Mask16::from_bits(unsafe { rust_isa_avx10_1_vptestmw_v256(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestmw_v512(a: *const u8, b: *const u8) -> u32;
}

/// `vptestmw_v512`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestmw_v512(a: V512, b: V512) -> Mask32 {
    Mask32::from_bits(unsafe { rust_isa_avx10_1_vptestmw_v512(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestmd_v256(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmd_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestmd_v256(a: V256, b: V256) -> Mask8 {
    Mask8::from_bits(unsafe { rust_isa_avx10_1_vptestmd_v256(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestmd_v512(a: *const u8, b: *const u8) -> u16;
}

/// `vptestmd_v512`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestmd_v512(a: V512, b: V512) -> Mask16 {
    Mask16::from_bits(unsafe { rust_isa_avx10_1_vptestmd_v512(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestmq_v256(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmq_v256`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestmq_v256(a: V256, b: V256) -> Mask4 {
    Mask4::from_bits(unsafe { rust_isa_avx10_1_vptestmq_v256(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestmq_v512(a: *const u8, b: *const u8) -> u8;
}

/// `vptestmq_v512`
/// Test the bitwise AND of the lanes and return the mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestmq_v512(a: V512, b: V512) -> Mask8 {
    Mask8::from_bits(unsafe { rust_isa_avx10_1_vptestmq_v512(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermd_v256(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermd_v256`
/// Permute the lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermd_v256(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermd_v256(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermd_v512(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermd_v512`
/// Permute the lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermd_v512(a: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermd_v512(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermq_v256(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermq_v256`
/// Permute the lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermq_v256(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermq_v256(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermq_v512(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermq_v512`
/// Permute the lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermq_v512(a: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermq_v512(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermb_v256(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermb_v256`
/// Permute the lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermb_v256(a: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermb_v256(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermb_v512(a: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermb_v512`
/// Permute the lanes with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermb_v512(a: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermb_v512(a.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermi2b_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2b_v256`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermi2b_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermi2b_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermi2b_v512(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2b_v512`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermi2b_v512(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermi2b_v512(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermt2b_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2b_v256`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermt2b_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermt2b_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermt2b_v512(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2b_v512`
/// Select bytes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermt2b_v512(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermt2b_v512(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcompressps_v256(a: *const u8, mask: u8, out: *mut u8);
}

/// `vcompressps_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcompressps_v256(a: V256, mask: Mask8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcompressps_v256(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcompressps_v512(a: *const u8, mask: u16, out: *mut u8);
}

/// `vcompressps_v512`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcompressps_v512(a: V512, mask: Mask16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcompressps_v512(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcompresspd_v256(a: *const u8, mask: u8, out: *mut u8);
}

/// `vcompresspd_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcompresspd_v256(a: V256, mask: Mask4) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcompresspd_v256(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcompresspd_v512(a: *const u8, mask: u8, out: *mut u8);
}

/// `vcompresspd_v512`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcompresspd_v512(a: V512, mask: Mask8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcompresspd_v512(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vexpandps_v256(a: *const u8, mask: u8, out: *mut u8);
}

/// `vexpandps_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vexpandps_v256(a: V256, mask: Mask8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vexpandps_v256(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vexpandps_v512(a: *const u8, mask: u16, out: *mut u8);
}

/// `vexpandps_v512`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vexpandps_v512(a: V512, mask: Mask16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vexpandps_v512(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vexpandpd_v256(a: *const u8, mask: u8, out: *mut u8);
}

/// `vexpandpd_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vexpandpd_v256(a: V256, mask: Mask4) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vexpandpd_v256(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vexpandpd_v512(a: *const u8, mask: u8, out: *mut u8);
}

/// `vexpandpd_v512`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vexpandpd_v512(a: V512, mask: Mask8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vexpandpd_v512(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtdq2ps_v256(a: *const u8, out: *mut u8);
}

/// `vcvtdq2ps_v256`
/// Convert signed 32-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtdq2ps_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtdq2ps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtps2dq_v256(a: *const u8, out: *mut u8);
}

/// `vcvtps2dq_v256`
/// Convert 32-bit float lanes to signed 32-bit integers.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtps2dq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtps2dq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttps2dq_v256(a: *const u8, out: *mut u8);
}

/// `vcvttps2dq_v256`
/// Convert 32-bit float lanes to signed 32-bit integers with truncation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttps2dq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvttps2dq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vrcp14ps_v256(a: *const u8, out: *mut u8);
}

/// `vrcp14ps_v256`
/// Compute a 14-bit approximate reciprocal of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14ps_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vrcp14ps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vrcp14pd_v256(a: *const u8, out: *mut u8);
}

/// `vrcp14pd_v256`
/// Compute a 14-bit approximate reciprocal of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14pd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vrcp14pd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vrsqrt14ps_v256(a: *const u8, out: *mut u8);
}

/// `vrsqrt14ps_v256`
/// Compute a 14-bit approximate reciprocal square root of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14ps_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vrsqrt14ps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vrsqrt14pd_v256(a: *const u8, out: *mut u8);
}

/// `vrsqrt14pd_v256`
/// Compute a 14-bit approximate reciprocal square root of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14pd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vrsqrt14pd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vsqrtps_v256(a: *const u8, out: *mut u8);
}

/// `vsqrtps_v256`
/// Compute the square root of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtps_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vsqrtps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vsqrtpd_v256(a: *const u8, out: *mut u8);
}

/// `vsqrtpd_v256`
/// Compute the square root of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtpd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vsqrtpd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtdq2ps_v512(a: *const u8, out: *mut u8);
}

/// `vcvtdq2ps_v512`
/// Convert signed 32-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtdq2ps_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtdq2ps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtps2dq_v512(a: *const u8, out: *mut u8);
}

/// `vcvtps2dq_v512`
/// Convert 32-bit float lanes to signed 32-bit integers.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtps2dq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtps2dq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttps2dq_v512(a: *const u8, out: *mut u8);
}

/// `vcvttps2dq_v512`
/// Convert 32-bit float lanes to signed 32-bit integers with truncation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttps2dq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvttps2dq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vrcp14ps_v512(a: *const u8, out: *mut u8);
}

/// `vrcp14ps_v512`
/// Compute a 14-bit approximate reciprocal of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14ps_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vrcp14ps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vrcp14pd_v512(a: *const u8, out: *mut u8);
}

/// `vrcp14pd_v512`
/// Compute a 14-bit approximate reciprocal of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrcp14pd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vrcp14pd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vrsqrt14ps_v512(a: *const u8, out: *mut u8);
}

/// `vrsqrt14ps_v512`
/// Compute a 14-bit approximate reciprocal square root of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14ps_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vrsqrt14ps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vrsqrt14pd_v512(a: *const u8, out: *mut u8);
}

/// `vrsqrt14pd_v512`
/// Compute a 14-bit approximate reciprocal square root of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vrsqrt14pd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vrsqrt14pd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vsqrtps_v512(a: *const u8, out: *mut u8);
}

/// `vsqrtps_v512`
/// Compute the square root of the 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtps_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vsqrtps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vsqrtpd_v512(a: *const u8, out: *mut u8);
}

/// `vsqrtpd_v512`
/// Compute the square root of the 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vsqrtpd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vsqrtpd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtps2pd_v256(a: *const u8, out: *mut u8);
}

/// `vcvtps2pd_v256`
/// Convert eight 32-bit float lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtps2pd_v256(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtps2pd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtpd2ps_v512(a: *const u8, out: *mut u8);
}

/// `vcvtpd2ps_v512`
/// Convert eight 64-bit float lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtpd2ps_v512(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtpd2ps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtpd2dq_v512(a: *const u8, out: *mut u8);
}

/// `vcvtpd2dq_v512`
/// Convert eight 64-bit float lanes to signed 32-bit integers.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtpd2dq_v512(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtpd2dq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttpd2dq_v512(a: *const u8, out: *mut u8);
}

/// `vcvttpd2dq_v512`
/// Convert eight 64-bit float lanes to signed 32-bit integers with truncation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttpd2dq_v512(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvttpd2dq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd132ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd132ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd132ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmadd132ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd132ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd132ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd132ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmadd132ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd132pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd132pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd132pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmadd132pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd132pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd132pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd132pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmadd132pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd132ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd132ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd132ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmadd132ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd132ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd132ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd132ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmadd132ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd132pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd132pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd132pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmadd132pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd132pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd132pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd132pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmadd132pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpdpbusd_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusd_v256`
/// Multiply lane pairs and accumulate into `acc`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpdpbusd_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpdpbusd_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpdpbusd_v512(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusd_v512`
/// Multiply lane pairs and accumulate into `acc`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpdpbusd_v512(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpdpbusd_v512(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpdpwssd_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssd_v256`
/// Multiply lane pairs and accumulate into `acc`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpdpwssd_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpdpwssd_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpdpwssd_v512(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssd_v512`
/// Multiply lane pairs and accumulate into `acc`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpdpwssd_v512(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpdpwssd_v512(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmadd52luq_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52luq_v256`
/// Add the low or high 52 bits of the unsigned products to `acc`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmadd52luq_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmadd52luq_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmadd52luq_v512(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52luq_v512`
/// Add the low or high 52 bits of the unsigned products to `acc`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmadd52luq_v512(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmadd52luq_v512(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmadd52huq_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52huq_v256`
/// Add the low or high 52 bits of the unsigned products to `acc`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmadd52huq_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpmadd52huq_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpmadd52huq_v512(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpmadd52huq_v512`
/// Add the low or high 52 bits of the unsigned products to `acc`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpmadd52huq_v512(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpmadd52huq_v512(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpopcntb_v256(a: *const u8, out: *mut u8);
}

/// `vpopcntb_v256`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpopcntb_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpopcntb_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpopcntb_v512(a: *const u8, out: *mut u8);
}

/// `vpopcntb_v512`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpopcntb_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpopcntb_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpopcntw_v256(a: *const u8, out: *mut u8);
}

/// `vpopcntw_v256`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpopcntw_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpopcntw_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpopcntw_v512(a: *const u8, out: *mut u8);
}

/// `vpopcntw_v512`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpopcntw_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpopcntw_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpopcntd_v256(a: *const u8, out: *mut u8);
}

/// `vpopcntd_v256`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpopcntd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpopcntd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpopcntd_v512(a: *const u8, out: *mut u8);
}

/// `vpopcntd_v512`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpopcntd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpopcntd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpopcntq_v256(a: *const u8, out: *mut u8);
}

/// `vpopcntq_v256`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpopcntq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpopcntq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpopcntq_v512(a: *const u8, out: *mut u8);
}

/// `vpopcntq_v512`
/// Count the set bits in each lane.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpopcntq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpopcntq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vplzcntd_v256(a: *const u8, out: *mut u8);
}

/// `vplzcntd_v256`
/// Apply the lane-wise operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vplzcntd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vplzcntd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vplzcntd_v512(a: *const u8, out: *mut u8);
}

/// `vplzcntd_v512`
/// Apply the lane-wise operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vplzcntd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vplzcntd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vplzcntq_v256(a: *const u8, out: *mut u8);
}

/// `vplzcntq_v256`
/// Apply the lane-wise operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vplzcntq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vplzcntq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vplzcntq_v512(a: *const u8, out: *mut u8);
}

/// `vplzcntq_v512`
/// Apply the lane-wise operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vplzcntq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vplzcntq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpconflictd_v256(a: *const u8, out: *mut u8);
}

/// `vpconflictd_v256`
/// Apply the lane-wise operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpconflictd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpconflictd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpconflictd_v512(a: *const u8, out: *mut u8);
}

/// `vpconflictd_v512`
/// Apply the lane-wise operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpconflictd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpconflictd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpconflictq_v256(a: *const u8, out: *mut u8);
}

/// `vpconflictq_v256`
/// Apply the lane-wise operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpconflictq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpconflictq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpconflictq_v512(a: *const u8, out: *mut u8);
}

/// `vpconflictq_v512`
/// Apply the lane-wise operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpconflictq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpconflictq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vgatherdps_v256(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherdps_v256`
/// Gather the 32-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for reading 4 bytes.
#[inline]
pub unsafe fn vgatherdps_v256(src: V256, base: *const u8, indices: V256, mask: Mask8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vgatherdps_v256(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask.bits(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vgatherdps_v512(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u16,
        out: *mut u8,
    );
}

/// `vgatherdps_v512`
/// Gather the 32-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for reading 4 bytes.
#[inline]
pub unsafe fn vgatherdps_v512(src: V512, base: *const u8, indices: V512, mask: Mask16) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vgatherdps_v512(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask.bits(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vgatherdpd_v256(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherdpd_v256`
/// Gather the 64-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for reading 8 bytes.
#[inline]
pub unsafe fn vgatherdpd_v256(src: V256, base: *const u8, indices: V128, mask: Mask4) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vgatherdpd_v256(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask.bits(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vgatherdpd_v512(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherdpd_v512`
/// Gather the 64-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for reading 8 bytes.
#[inline]
pub unsafe fn vgatherdpd_v512(src: V512, base: *const u8, indices: V256, mask: Mask8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vgatherdpd_v512(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask.bits(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vgatherqpd_v256(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqpd_v256`
/// Gather the 64-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for reading 8 bytes.
#[inline]
pub unsafe fn vgatherqpd_v256(src: V256, base: *const u8, indices: V256, mask: Mask4) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vgatherqpd_v256(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask.bits(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vgatherqpd_v512(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqpd_v512`
/// Gather the 64-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for reading 8 bytes.
#[inline]
pub unsafe fn vgatherqpd_v512(src: V512, base: *const u8, indices: V512, mask: Mask8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vgatherqpd_v512(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask.bits(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vgatherqps_v256(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqps_v256`
/// Gather four 32-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for reading 4 bytes.
#[inline]
pub unsafe fn vgatherqps_v256(src: V128, base: *const u8, indices: V256, mask: Mask4) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_1_vgatherqps_v256(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask.bits(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vgatherqps_v512(
        src: *const u8,
        base: *const u8,
        indices: *const u8,
        mask: u8,
        out: *mut u8,
    );
}

/// `vgatherqps_v512`
/// Gather eight 32-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for reading 4 bytes.
#[inline]
pub unsafe fn vgatherqps_v512(src: V256, base: *const u8, indices: V512, mask: Mask8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vgatherqps_v512(
            src.as_ptr(),
            base,
            indices.as_ptr(),
            mask.bits(),
            out.as_mut_ptr(),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vscatterdps_v256(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterdps_v256`
/// Scatter the 32-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for writing 4 bytes.
#[inline]
pub unsafe fn vscatterdps_v256(base: *mut u8, indices: V256, a: V256, mask: Mask8) {
    unsafe { rust_isa_avx10_1_vscatterdps_v256(base, indices.as_ptr(), a.as_ptr(), mask.bits()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vscatterdps_v512(
        base: *mut u8,
        indices: *const u8,
        a: *const u8,
        mask: u16,
    );
}

/// `vscatterdps_v512`
/// Scatter the 32-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for writing 4 bytes.
#[inline]
pub unsafe fn vscatterdps_v512(base: *mut u8, indices: V512, a: V512, mask: Mask16) {
    unsafe { rust_isa_avx10_1_vscatterdps_v512(base, indices.as_ptr(), a.as_ptr(), mask.bits()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vscatterdpd_v256(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterdpd_v256`
/// Scatter the 64-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for writing 8 bytes.
#[inline]
pub unsafe fn vscatterdpd_v256(base: *mut u8, indices: V128, a: V256, mask: Mask4) {
    unsafe { rust_isa_avx10_1_vscatterdpd_v256(base, indices.as_ptr(), a.as_ptr(), mask.bits()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vscatterdpd_v512(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterdpd_v512`
/// Scatter the 64-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for writing 8 bytes.
#[inline]
pub unsafe fn vscatterdpd_v512(base: *mut u8, indices: V256, a: V512, mask: Mask8) {
    unsafe { rust_isa_avx10_1_vscatterdpd_v512(base, indices.as_ptr(), a.as_ptr(), mask.bits()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vscatterqpd_v256(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterqpd_v256`
/// Scatter the 64-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for writing 8 bytes.
#[inline]
pub unsafe fn vscatterqpd_v256(base: *mut u8, indices: V256, a: V256, mask: Mask4) {
    unsafe { rust_isa_avx10_1_vscatterqpd_v256(base, indices.as_ptr(), a.as_ptr(), mask.bits()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vscatterqpd_v512(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterqpd_v512`
/// Scatter the 64-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for writing 8 bytes.
#[inline]
pub unsafe fn vscatterqpd_v512(base: *mut u8, indices: V512, a: V512, mask: Mask8) {
    unsafe { rust_isa_avx10_1_vscatterqpd_v512(base, indices.as_ptr(), a.as_ptr(), mask.bits()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vscatterqps_v256(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterqps_v256`
/// Scatter four 32-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for writing 4 bytes.
#[inline]
pub unsafe fn vscatterqps_v256(base: *mut u8, indices: V256, a: V128, mask: Mask4) {
    unsafe { rust_isa_avx10_1_vscatterqps_v256(base, indices.as_ptr(), a.as_ptr(), mask.bits()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vscatterqps_v512(base: *mut u8, indices: *const u8, a: *const u8, mask: u8);
}

/// `vscatterqps_v512`
/// Scatter eight 32-bit float lanes selected by `mask`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
///
/// Every selected address must be valid for writing 4 bytes.
#[inline]
pub unsafe fn vscatterqps_v512(base: *mut u8, indices: V512, a: V256, mask: Mask8) {
    unsafe { rust_isa_avx10_1_vscatterqps_v512(base, indices.as_ptr(), a.as_ptr(), mask.bits()) }
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpcompressb_v256(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpcompressb_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcompressb_v256(a: V256, mask: Mask32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpcompressb_v256(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpcompressb_v512(a: *const u8, mask: u64, out: *mut u8);
}

/// `vpcompressb_v512`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcompressb_v512(a: V512, mask: Mask64) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpcompressb_v512(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpcompressw_v256(a: *const u8, mask: u16, out: *mut u8);
}

/// `vpcompressw_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcompressw_v256(a: V256, mask: Mask16) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpcompressw_v256(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpcompressw_v512(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpcompressw_v512`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpcompressw_v512(a: V512, mask: Mask32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpcompressw_v512(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpexpandb_v256(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpexpandb_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpexpandb_v256(a: V256, mask: Mask32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpexpandb_v256(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpexpandb_v512(a: *const u8, mask: u64, out: *mut u8);
}

/// `vpexpandb_v512`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpexpandb_v512(a: V512, mask: Mask64) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpexpandb_v512(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpexpandw_v256(a: *const u8, mask: u16, out: *mut u8);
}

/// `vpexpandw_v256`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpexpandw_v256(a: V256, mask: Mask16) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpexpandw_v256(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpexpandw_v512(a: *const u8, mask: u32, out: *mut u8);
}

/// `vpexpandw_v512`
/// Pack or expand the selected lanes and zero the rest.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpexpandw_v512(a: V512, mask: Mask32) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpexpandw_v512(a.as_ptr(), mask.bits(), out.as_mut_ptr());
    }
    out
}

/// `vpshldw_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshldw_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshldw ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshldw_v512`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshldw_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshldw zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshldd_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshldd_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshldd ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshldd_v512`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshldd_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshldd zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshldq_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshldq_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshldq ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshldq_v512`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshldq_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshldq zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdw_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshrdw_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshrdw ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdw_v512`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshrdw_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshrdw zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdd_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshrdd_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshrdd ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdd_v512`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshrdd_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshrdd zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdq_v256`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshrdq_v256<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        asm!(
            "vmovdqu ymm0, [{a}]",
            "vmovdqu ymm1, [{b}]",
            "vpshrdq ymm0, ymm0, ymm1, {imm}",
            "vmovdqu [{out}], ymm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

/// `vpshrdq_v512`
/// Shift and merge the lanes by the count in `IMM`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpshrdq_v512<const IMM: u8>(a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        asm!(
            "vmovdqu64 zmm0, [{a}]",
            "vmovdqu64 zmm1, [{b}]",
            "vpshrdq zmm0, zmm0, zmm1, {imm}",
            "vmovdqu64 [{out}], zmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            imm = const IMM,
            out = in(reg) out.as_mut_ptr(),
            out("zmm0") _,
            out("zmm1") _,
            options(nostack, preserves_flags),
        );
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermi2w_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2w_v256`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermi2w_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermi2w_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermi2w_v512(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2w_v512`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermi2w_v512(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermi2w_v512(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermt2w_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2w_v256`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermt2w_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermt2w_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermt2w_v512(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2w_v512`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermt2w_v512(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermt2w_v512(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermi2d_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2d_v256`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermi2d_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermi2d_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermi2d_v512(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2d_v512`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermi2d_v512(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermi2d_v512(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermt2d_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2d_v256`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermt2d_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermt2d_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermt2d_v512(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2d_v512`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermt2d_v512(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermt2d_v512(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermi2q_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2q_v256`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermi2q_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermi2q_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermi2q_v512(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermi2q_v512`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermi2q_v512(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermi2q_v512(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermt2q_v256(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2q_v256`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermt2q_v256(a: V256, b: V256, idx: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpermt2q_v256(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpermt2q_v512(a: *const u8, b: *const u8, idx: *const u8, out: *mut u8);
}

/// `vpermt2q_v512`
/// Select lanes from `a` and `b` with the indices in `idx`.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpermt2q_v512(a: V512, b: V512, idx: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpermt2q_v512(a.as_ptr(), b.as_ptr(), idx.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestnmb_v256(a: *const u8, b: *const u8) -> u32;
}

/// `vptestnmb_v256`
/// Test the bitwise AND of the lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestnmb_v256(a: V256, b: V256) -> Mask32 {
    Mask32::from_bits(unsafe { rust_isa_avx10_1_vptestnmb_v256(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestnmb_v512(a: *const u8, b: *const u8) -> u64;
}

/// `vptestnmb_v512`
/// Test the bitwise AND of the lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestnmb_v512(a: V512, b: V512) -> Mask64 {
    Mask64::from_bits(unsafe { rust_isa_avx10_1_vptestnmb_v512(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestnmw_v256(a: *const u8, b: *const u8) -> u16;
}

/// `vptestnmw_v256`
/// Test the bitwise AND of the lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestnmw_v256(a: V256, b: V256) -> Mask16 {
    Mask16::from_bits(unsafe { rust_isa_avx10_1_vptestnmw_v256(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestnmw_v512(a: *const u8, b: *const u8) -> u32;
}

/// `vptestnmw_v512`
/// Test the bitwise AND of the lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestnmw_v512(a: V512, b: V512) -> Mask32 {
    Mask32::from_bits(unsafe { rust_isa_avx10_1_vptestnmw_v512(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestnmd_v256(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmd_v256`
/// Test the bitwise AND of the lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestnmd_v256(a: V256, b: V256) -> Mask8 {
    Mask8::from_bits(unsafe { rust_isa_avx10_1_vptestnmd_v256(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestnmd_v512(a: *const u8, b: *const u8) -> u16;
}

/// `vptestnmd_v512`
/// Test the bitwise AND of the lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestnmd_v512(a: V512, b: V512) -> Mask16 {
    Mask16::from_bits(unsafe { rust_isa_avx10_1_vptestnmd_v512(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestnmq_v256(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmq_v256`
/// Test the bitwise AND of the lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestnmq_v256(a: V256, b: V256) -> Mask4 {
    Mask4::from_bits(unsafe { rust_isa_avx10_1_vptestnmq_v256(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vptestnmq_v512(a: *const u8, b: *const u8) -> u8;
}

/// `vptestnmq_v512`
/// Test the bitwise AND of the lanes and return the zero-lane mask.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vptestnmq_v512(a: V512, b: V512) -> Mask8 {
    Mask8::from_bits(unsafe { rust_isa_avx10_1_vptestnmq_v512(a.as_ptr(), b.as_ptr()) })
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd213ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd213ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd213ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmadd213ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd213ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd213ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd213ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmadd213ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd213pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd213pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd213pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmadd213pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd213pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd213pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd213pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmadd213pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd231ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd231ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd231ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmadd231ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd231ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd231ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd231ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmadd231ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd231pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd231pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd231pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmadd231pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmadd231pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmadd231pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmadd231pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmadd231pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsub213ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub213ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsub213ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsub213ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsub213ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub213ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsub213ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsub213ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsub213pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub213pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsub213pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsub213pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsub213pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub213pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsub213pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsub213pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsub231ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub231ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsub231ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsub231ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsub231ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub231ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsub231ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsub231ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsub231pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub231pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsub231pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsub231pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsub231pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsub231pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsub231pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsub231pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd213ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd213ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd213ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmadd213ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd213ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd213ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd213ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmadd213ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd213pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd213pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd213pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmadd213pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd213pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd213pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd213pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmadd213pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd231ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd231ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd231ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmadd231ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd231ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd231ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd231ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmadd231ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd231pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd231pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd231pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmadd231pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmadd231pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmadd231pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmadd231pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmadd231pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub132ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub132ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub132ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmsub132ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub132ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub132ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub132ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmsub132ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub132pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub132pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub132pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmsub132pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub132pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub132pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub132pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmsub132pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub213ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub213ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub213ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmsub213ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub213ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub213ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub213ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmsub213ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub213pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub213pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub213pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmsub213pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub213pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub213pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub213pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmsub213pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub231ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub231ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub231ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmsub231ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub231ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub231ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub231ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmsub231ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub231pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub231pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub231pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfnmsub231pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfnmsub231pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfnmsub231pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfnmsub231pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfnmsub231pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub132ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub132ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub132ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub132ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub132ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub132ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub132ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub132ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub132pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub132pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub132pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub132pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub132pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub132pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub132pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub132pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub213ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub213ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub213ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub213ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub213ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub213ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub213ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub213ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub213pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub213pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub213pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub213pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub213pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub213pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub213pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub213pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub231ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub231ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub231ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub231ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub231ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub231ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub231ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub231ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub231pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub231pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub231pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub231pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmaddsub231pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmaddsub231pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmaddsub231pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmaddsub231pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd132ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd132ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd132ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd132ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd132ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd132ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd132ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd132ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd132pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd132pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd132pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd132pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd132pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd132pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd132pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd132pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd213ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd213ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd213ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd213ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd213ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd213ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd213ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd213ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd213pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd213pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd213pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd213pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd213pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd213pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd213pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd213pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd231ps_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd231ps_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd231ps_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd231ps_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd231ps_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd231ps_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd231ps_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd231ps_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd231pd_v256(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd231pd_v256`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd231pd_v256(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd231pd_v256(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vfmsubadd231pd_v512(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `vfmsubadd231pd_v512`
/// Multiply `a` and `b`, then add or subtract `c` according to the operation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vfmsubadd231pd_v512(a: V512, b: V512, c: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vfmsubadd231pd_v512(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpdpbusds_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusds_v256`
/// Multiply lane pairs and accumulate into `acc` with saturation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpdpbusds_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpdpbusds_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpdpbusds_v512(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpbusds_v512`
/// Multiply lane pairs and accumulate into `acc` with saturation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpdpbusds_v512(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpdpbusds_v512(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpdpwssds_v256(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssds_v256`
/// Multiply lane pairs and accumulate into `acc` with saturation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpdpwssds_v256(acc: V256, a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vpdpwssds_v256(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vpdpwssds_v512(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
}

/// `vpdpwssds_v512`
/// Multiply lane pairs and accumulate into `acc` with saturation.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vpdpwssds_v512(acc: V512, a: V512, b: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vpdpwssds_v512(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtpd2qq_v512(a: *const u8, out: *mut u8);
}

/// `vcvtpd2qq_v512`
/// Convert 64-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtpd2qq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtpd2qq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtpd2uqq_v512(a: *const u8, out: *mut u8);
}

/// `vcvtpd2uqq_v512`
/// Convert 64-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtpd2uqq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtpd2uqq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtpd2qq_v256(a: *const u8, out: *mut u8);
}

/// `vcvtpd2qq_v256`
/// Convert four 64-bit float lanes to signed 64-bit integers.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtpd2qq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtpd2qq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtpd2uqq_v256(a: *const u8, out: *mut u8);
}

/// `vcvtpd2uqq_v256`
/// Convert four 64-bit float lanes to unsigned 64-bit integers.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtpd2uqq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtpd2uqq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtps2qq_v256(a: *const u8, out: *mut u8);
}

/// `vcvtps2qq_v256`
/// Convert 32-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtps2qq_v256(a: V128) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtps2qq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtps2qq_v512(a: *const u8, out: *mut u8);
}

/// `vcvtps2qq_v512`
/// Convert 32-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtps2qq_v512(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtps2qq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtps2uqq_v256(a: *const u8, out: *mut u8);
}

/// `vcvtps2uqq_v256`
/// Convert 32-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtps2uqq_v256(a: V128) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtps2uqq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtps2uqq_v512(a: *const u8, out: *mut u8);
}

/// `vcvtps2uqq_v512`
/// Convert 32-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtps2uqq_v512(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtps2uqq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttps2qq_v256(a: *const u8, out: *mut u8);
}

/// `vcvttps2qq_v256`
/// Convert 32-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttps2qq_v256(a: V128) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvttps2qq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttps2qq_v512(a: *const u8, out: *mut u8);
}

/// `vcvttps2qq_v512`
/// Convert 32-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttps2qq_v512(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvttps2qq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttps2uqq_v256(a: *const u8, out: *mut u8);
}

/// `vcvttps2uqq_v256`
/// Convert 32-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttps2uqq_v256(a: V128) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvttps2uqq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttps2uqq_v512(a: *const u8, out: *mut u8);
}

/// `vcvttps2uqq_v512`
/// Convert 32-bit float lanes to 64-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttps2uqq_v512(a: V256) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvttps2uqq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtqq2pd_v256(a: *const u8, out: *mut u8);
}

/// `vcvtqq2pd_v256`
/// Convert 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtqq2pd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtqq2pd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtqq2pd_v512(a: *const u8, out: *mut u8);
}

/// `vcvtqq2pd_v512`
/// Convert 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtqq2pd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtqq2pd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtuqq2pd_v256(a: *const u8, out: *mut u8);
}

/// `vcvtuqq2pd_v256`
/// Convert 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtuqq2pd_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtuqq2pd_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtuqq2pd_v512(a: *const u8, out: *mut u8);
}

/// `vcvtuqq2pd_v512`
/// Convert 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtuqq2pd_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvtuqq2pd_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttpd2qq_v256(a: *const u8, out: *mut u8);
}

/// `vcvttpd2qq_v256`
/// Convert 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttpd2qq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvttpd2qq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttpd2qq_v512(a: *const u8, out: *mut u8);
}

/// `vcvttpd2qq_v512`
/// Convert 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttpd2qq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvttpd2qq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttpd2uqq_v256(a: *const u8, out: *mut u8);
}

/// `vcvttpd2uqq_v256`
/// Convert 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttpd2uqq_v256(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvttpd2uqq_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvttpd2uqq_v512(a: *const u8, out: *mut u8);
}

/// `vcvttpd2uqq_v512`
/// Convert 64-bit integer lanes to 64-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvttpd2uqq_v512(a: V512) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx10_1_vcvttpd2uqq_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtqq2ps_v256(a: *const u8, out: *mut u8);
}

/// `vcvtqq2ps_v256`
/// Convert signed 64-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtqq2ps_v256(a: V256) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_1_vcvtqq2ps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtqq2ps_v512(a: *const u8, out: *mut u8);
}

/// `vcvtqq2ps_v512`
/// Convert signed 64-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtqq2ps_v512(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtqq2ps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtuqq2ps_v256(a: *const u8, out: *mut u8);
}

/// `vcvtuqq2ps_v256`
/// Convert unsigned 64-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtuqq2ps_v256(a: V256) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_avx10_1_vcvtuqq2ps_v256(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_avx10_1_vcvtuqq2ps_v512(a: *const u8, out: *mut u8);
}

/// `vcvtuqq2ps_v512`
/// Convert unsigned 64-bit integer lanes to 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with AVX10.1 enabled and the CPU must support it.
#[inline]
pub unsafe fn vcvtuqq2ps_v512(a: V512) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx10_1_vcvtuqq2ps_v512(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
