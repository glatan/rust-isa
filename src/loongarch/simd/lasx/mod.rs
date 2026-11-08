//! LASX.
//!
//! LASX is not part of the LoongArch baseline. Every instruction wrapper is `unsafe` and
//! the caller must enable LASX with `#[target_feature(enable = "lasx")]` or a
//! global target feature.
//!
//! The values are [`V256`]. The C implementation uses GCC vector extensions and
//! ordinary loads and stores so that LTO can keep values in registers.
//! Immediate operands are const generics unless documented otherwise.
//! Comparison, shift, shuffle, and bit operations are written in Rust on the
//! byte representation. Float conversions, max, and min call the clang
//! intrinsics; zero cost is not claimed for those.
//!
//! Instruction behavior follows the [LoongArch Documentation](https://loongson.github.io/LoongArch-Documentation/).
//!
//! Instructions not listed in this module are not exposed.
//!
//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvseqi_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvseqi_b::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvseqi_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvseqi_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvseqi_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvseqi_w::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvseqi_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvseqi_d::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslei_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslei_b::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslei_bu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslei_bu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslei_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslei_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslei_hu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslei_hu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslei_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslei_w::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslei_wu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslei_wu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslei_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslei_d::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslei_du, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslei_du::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslti_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslti_b::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslti_bu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslti_bu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslti_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslti_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslti_hu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslti_hu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslti_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslti_w::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslti_wu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslti_wu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslti_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslti_d::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslti_du, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslti_du::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslli_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslli_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslli_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslli_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslli_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslli_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvslli_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvslli_d::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrli_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrli_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrli_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrli_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrli_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrli_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrli_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrli_d::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrai_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrai_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrai_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrai_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrai_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrai_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrai_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrai_d::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmaxi_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmaxi_b::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmaxi_bu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmaxi_bu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmaxi_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmaxi_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmaxi_hu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmaxi_hu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmaxi_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmaxi_w::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmaxi_wu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmaxi_wu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmaxi_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmaxi_d::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmaxi_du, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmaxi_du::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmini_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmini_b::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmini_bu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmini_bu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmini_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmini_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmini_hu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmini_hu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmini_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmini_w::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmini_wu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmini_wu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmini_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmini_d::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvmini_du, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvmini_du::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvrotri_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvrotri_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvrotri_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvrotri_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvrotri_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvrotri_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvrotri_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvrotri_d::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitclri_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitclri_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitclri_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitclri_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitclri_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitclri_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitclri_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitclri_d::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitseti_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitseti_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitseti_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitseti_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitseti_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitseti_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitseti_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitseti_d::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitrevi_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitrevi_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitrevi_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitrevi_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitrevi_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitrevi_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvbitrevi_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvbitrevi_d::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvaddi_bu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvaddi_bu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvaddi_hu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvaddi_hu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvaddi_wu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvaddi_wu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvaddi_du, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvaddi_du::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsubi_bu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsubi_bu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsubi_hu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsubi_hu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsubi_wu, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsubi_wu::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsubi_du, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsubi_du::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrlri_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrlri_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrlri_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrlri_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrlri_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrlri_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrlri_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrlri_d::<64>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrari_b, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrari_b::<8>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrari_h, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrari_h::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrari_w, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrari_w::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lasx::{xvsrari_d, V256};
//!
//! let v = V256::from_bytes([0; 32]);
//! let _ = unsafe { xvsrari_d::<64>(v) };
//! ```
//!

//! # Examples
//!
//! ```
//! use rust_isa::loongarch::simd::lasx::{splat_i32, xvadd, V256};
//!
//! let left = unsafe { splat_i32(1) };
//! let right = unsafe { splat_i32(2) };
//! let sum = unsafe { xvadd(left, right) };
//! assert_eq!(sum.to_bytes(), unsafe { splat_i32(3) }.to_bytes());
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::loongarch::simd::lasx::*;
//! use rust_isa::loongarch::simd::lsx::V128;
//! use rust_isa::loongarch::simd::lasx::V256;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { add(v256, v256) };
//! let _ = unsafe { add_i64(v256, v256) };
//! let _ = unsafe { and(v256, v256) };
//! let _ = unsafe { fadd(v256, v256) };
//! let _ = unsafe { fdiv(v256, v256) };
//! let _ = unsafe { fmul(v256, v256) };
//! let _ = unsafe { fsub(v256, v256) };
//! let _ = unsafe { mul(v256, v256) };
//! let _ = unsafe { mul_i32(v256, v256) };
//! let _ = unsafe { or(v256, v256) };
//! let _ = unsafe { splat_i32(0) };
//! let _ = unsafe { sub(v256, v256) };
//! let _ = unsafe { sub_i64(v256, v256) };
//! let _ = unsafe { v256_load(storage.0.as_ptr().cast()) };
//! let _ = unsafe { v256_store(storage.0.as_mut_ptr().cast(), v256) };
//! let _ = unsafe { vext2xv_h_b(v256) };
//! let _ = unsafe { xor(v256, v256) };
//! let _ = unsafe { xvabs(v256) };
//! let _ = unsafe { xvabs_b(v256) };
//! let _ = unsafe { xvabs_d(v256) };
//! let _ = unsafe { xvabs_h(v256) };
//! let _ = unsafe { xvadd(v256, v256) };
//! let _ = unsafe { xvadd_b(v256, v256) };
//! let _ = unsafe { xvadd_d(v256, v256) };
//! let _ = unsafe { xvadd_h(v256, v256) };
//! let _ = unsafe { xvadd_w(v256, v256) };
//! let _ = unsafe { xvand(v256, v256) };
//! let _ = unsafe { xvavgr(v256, v256) };
//! let _ = unsafe { xvbitsel(v256, v256, v256) };
//! let _ = unsafe { xvfadd(v256, v256) };
//! let _ = unsafe { xvfcmp::<0>(v256, v256) };
//! let _ = unsafe { xvfcmp_caf_s(v256, v256) };
//! let _ = unsafe { xvfmadd_s(v256, v256, v256) };
//! let _ = unsafe { xvfmadd_d(v256, v256, v256) };
//! let _ = unsafe { xvfmsub_s(v256, v256, v256) };
//! let _ = unsafe { xvfmsub_d(v256, v256, v256) };
//! let _ = unsafe { xvfnmadd_s(v256, v256, v256) };
//! let _ = unsafe { xvfnmadd_d(v256, v256, v256) };
//! let _ = unsafe { xvfnmsub_s(v256, v256, v256) };
//! let _ = unsafe { xvfnmsub_d(v256, v256, v256) };
//! let _ = unsafe { xvfcmp_saf_s(v256, v256) };
//! let _ = unsafe { xvfcmp_clt_s(v256, v256) };
//! let _ = unsafe { xvfcmp_slt_s(v256, v256) };
//! let _ = unsafe { xvfcmp_ceq_s(v256, v256) };
//! let _ = unsafe { xvfcmp_seq_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cle_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sle_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cun_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sun_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cult_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sult_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cueq_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sueq_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cule_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sule_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cne_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sne_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cor_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sor_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cune_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sune_s(v256, v256) };
//! let _ = unsafe { xvfcmp_caf_d(v256, v256) };
//! let _ = unsafe { xvfcmp_saf_d(v256, v256) };
//! let _ = unsafe { xvfcmp_clt_d(v256, v256) };
//! let _ = unsafe { xvfcmp_slt_d(v256, v256) };
//! let _ = unsafe { xvfcmp_ceq_d(v256, v256) };
//! let _ = unsafe { xvfcmp_seq_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cle_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sle_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cun_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sun_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cult_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sult_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cueq_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sueq_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cule_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sule_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cne_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sne_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cor_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sor_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cune_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sune_d(v256, v256) };
//! let _ = unsafe { xvfdiv(v256, v256) };
//! let _ = unsafe { xvffint(v256) };
//! let _ = unsafe { xvfmax(v256, v256) };
//! let _ = unsafe { xvfmin(v256, v256) };
//! let _ = unsafe { xvfmul(v256, v256) };
//! let _ = unsafe { xvfsub(v256, v256) };
//! let _ = unsafe { xvftint(v256) };
//! let _ = unsafe { xvld(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { xvldi::<0>() };
//! let _ = unsafe { xvmax(v256, v256) };
//! let _ = unsafe { xvmin(v256, v256) };
//! let _ = unsafe { xvmul(v256, v256) };
//! let _ = unsafe { xvmul_h(v256, v256) };
//! let _ = unsafe { xvmul_w(v256, v256) };
//! let _ = unsafe { xvneg(v256) };
//! let _ = unsafe { xvneg_b(v256) };
//! let _ = unsafe { xvneg_d(v256) };
//! let _ = unsafe { xvneg_h(v256) };
//! let _ = unsafe { xvnor(v256, v256) };
//! let _ = unsafe { xvor(v256, v256) };
//! let _ = unsafe { xvpackev(v256, v256) };
//! let _ = unsafe { xvpackod(v256, v256) };
//! let _ = unsafe { xvsat::<0>(v256) };
//! let _ = unsafe { xvseq(v256, v256) };
//! let _ = unsafe { xvseqi(v256, 0) };
//! let _ = unsafe { xvshuf(v256, v256, v256) };
//! let _ = unsafe { xvshuf4i::<0>(v256) };
//! let _ = unsafe { xvsle(v256, v256) };
//! let _ = unsafe { xvslei(v256, 0) };
//! let _ = unsafe { xvsll(v256, 0) };
//! let _ = unsafe { xvslli::<0>(v256) };
//! let _ = unsafe { xvslt(v256, v256) };
//! let _ = unsafe { xvslti(v256, 0) };
//! let _ = unsafe { xvsra(v256, 0) };
//! let _ = unsafe { xvsrai::<0>(v256) };
//! let _ = unsafe { xvsrl(v256, 0) };
//! let _ = unsafe { xvsrli::<0>(v256) };
//! let _ = unsafe { xvst(storage.0.as_mut_ptr().cast(), 0, v256) };
//! let _ = unsafe { xvsub(v256, v256) };
//! let _ = unsafe { xvsub_b(v256, v256) };
//! let _ = unsafe { xvsub_d(v256, v256) };
//! let _ = unsafe { xvsub_h(v256, v256) };
//! let _ = unsafe { xvsub_w(v256, v256) };
//! let _ = unsafe { xvxor(v256, v256) };

//! let _ = unsafe { xvfcmp_caf_s(v256, v256) };
//! let _ = unsafe { xvfcmp_saf_s(v256, v256) };
//! let _ = unsafe { xvfcmp_clt_s(v256, v256) };
//! let _ = unsafe { xvfcmp_slt_s(v256, v256) };
//! let _ = unsafe { xvfcmp_ceq_s(v256, v256) };
//! let _ = unsafe { xvfcmp_seq_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cle_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sle_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cun_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sun_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cult_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sult_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cueq_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sueq_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cule_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sule_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cne_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sne_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cor_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sor_s(v256, v256) };
//! let _ = unsafe { xvfcmp_cune_s(v256, v256) };
//! let _ = unsafe { xvfcmp_sune_s(v256, v256) };
//! let _ = unsafe { xvfcmp_caf_d(v256, v256) };
//! let _ = unsafe { xvfcmp_saf_d(v256, v256) };
//! let _ = unsafe { xvfcmp_clt_d(v256, v256) };
//! let _ = unsafe { xvfcmp_slt_d(v256, v256) };
//! let _ = unsafe { xvfcmp_ceq_d(v256, v256) };
//! let _ = unsafe { xvfcmp_seq_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cle_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sle_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cun_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sun_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cult_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sult_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cueq_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sueq_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cule_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sule_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cne_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sne_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cor_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sor_d(v256, v256) };
//! let _ = unsafe { xvfcmp_cune_d(v256, v256) };
//! let _ = unsafe { xvfcmp_sune_d(v256, v256) };
//! let _ = unsafe { xvbitsel_v(v256, v256, v256) };
//! let _ = unsafe { xvshuf_b(v256, v256, v256) };
//! let _ = unsafe { xvldx(storage.0.as_ptr(), 0) };
//! let _ = unsafe { xvstx(v256, storage.0.as_mut_ptr(), 0) };
//! let _ = unsafe { xvseq_b(v256, v256) };
//! let _ = unsafe { xvseq_h(v256, v256) };
//! let _ = unsafe { xvseq_w(v256, v256) };
//! let _ = unsafe { xvseq_d(v256, v256) };
//! let _ = unsafe { xvsle_b(v256, v256) };
//! let _ = unsafe { xvsle_h(v256, v256) };
//! let _ = unsafe { xvsle_w(v256, v256) };
//! let _ = unsafe { xvsle_d(v256, v256) };
//! let _ = unsafe { xvsle_bu(v256, v256) };
//! let _ = unsafe { xvsle_hu(v256, v256) };
//! let _ = unsafe { xvsle_wu(v256, v256) };
//! let _ = unsafe { xvsle_du(v256, v256) };
//! let _ = unsafe { xvslt_b(v256, v256) };
//! let _ = unsafe { xvslt_h(v256, v256) };
//! let _ = unsafe { xvslt_w(v256, v256) };
//! let _ = unsafe { xvslt_d(v256, v256) };
//! let _ = unsafe { xvslt_bu(v256, v256) };
//! let _ = unsafe { xvslt_hu(v256, v256) };
//! let _ = unsafe { xvslt_wu(v256, v256) };
//! let _ = unsafe { xvslt_du(v256, v256) };
//! let _ = unsafe { xvadd_b(v256, v256) };
//! let _ = unsafe { xvadd_h(v256, v256) };
//! let _ = unsafe { xvadd_w(v256, v256) };
//! let _ = unsafe { xvadd_d(v256, v256) };
//! let _ = unsafe { xvsub_b(v256, v256) };
//! let _ = unsafe { xvsub_h(v256, v256) };
//! let _ = unsafe { xvsub_w(v256, v256) };
//! let _ = unsafe { xvsub_d(v256, v256) };
//! let _ = unsafe { xvaddwev_h_b(v256, v256) };
//! let _ = unsafe { xvaddwev_w_h(v256, v256) };
//! let _ = unsafe { xvaddwev_d_w(v256, v256) };
//! let _ = unsafe { xvaddwev_q_d(v256, v256) };
//! let _ = unsafe { xvsubwev_h_b(v256, v256) };
//! let _ = unsafe { xvsubwev_w_h(v256, v256) };
//! let _ = unsafe { xvsubwev_d_w(v256, v256) };
//! let _ = unsafe { xvsubwev_q_d(v256, v256) };
//! let _ = unsafe { xvaddwod_h_b(v256, v256) };
//! let _ = unsafe { xvaddwod_w_h(v256, v256) };
//! let _ = unsafe { xvaddwod_d_w(v256, v256) };
//! let _ = unsafe { xvaddwod_q_d(v256, v256) };
//! let _ = unsafe { xvsubwod_h_b(v256, v256) };
//! let _ = unsafe { xvsubwod_w_h(v256, v256) };
//! let _ = unsafe { xvsubwod_d_w(v256, v256) };
//! let _ = unsafe { xvsubwod_q_d(v256, v256) };
//! let _ = unsafe { xvaddwev_h_bu(v256, v256) };
//! let _ = unsafe { xvaddwev_w_hu(v256, v256) };
//! let _ = unsafe { xvaddwev_d_wu(v256, v256) };
//! let _ = unsafe { xvaddwev_q_du(v256, v256) };
//! let _ = unsafe { xvsubwev_h_bu(v256, v256) };
//! let _ = unsafe { xvsubwev_w_hu(v256, v256) };
//! let _ = unsafe { xvsubwev_d_wu(v256, v256) };
//! let _ = unsafe { xvsubwev_q_du(v256, v256) };
//! let _ = unsafe { xvaddwod_h_bu(v256, v256) };
//! let _ = unsafe { xvaddwod_w_hu(v256, v256) };
//! let _ = unsafe { xvaddwod_d_wu(v256, v256) };
//! let _ = unsafe { xvaddwod_q_du(v256, v256) };
//! let _ = unsafe { xvsubwod_h_bu(v256, v256) };
//! let _ = unsafe { xvsubwod_w_hu(v256, v256) };
//! let _ = unsafe { xvsubwod_d_wu(v256, v256) };
//! let _ = unsafe { xvsubwod_q_du(v256, v256) };
//! let _ = unsafe { xvaddwev_h_bu_b(v256, v256) };
//! let _ = unsafe { xvaddwev_w_hu_h(v256, v256) };
//! let _ = unsafe { xvaddwev_d_wu_w(v256, v256) };
//! let _ = unsafe { xvaddwev_q_du_d(v256, v256) };
//! let _ = unsafe { xvaddwod_h_bu_b(v256, v256) };
//! let _ = unsafe { xvaddwod_w_hu_h(v256, v256) };
//! let _ = unsafe { xvaddwod_d_wu_w(v256, v256) };
//! let _ = unsafe { xvaddwod_q_du_d(v256, v256) };
//! let _ = unsafe { xvsadd_b(v256, v256) };
//! let _ = unsafe { xvsadd_h(v256, v256) };
//! let _ = unsafe { xvsadd_w(v256, v256) };
//! let _ = unsafe { xvsadd_d(v256, v256) };
//! let _ = unsafe { xvssub_b(v256, v256) };
//! let _ = unsafe { xvssub_h(v256, v256) };
//! let _ = unsafe { xvssub_w(v256, v256) };
//! let _ = unsafe { xvssub_d(v256, v256) };
//! let _ = unsafe { xvsadd_bu(v256, v256) };
//! let _ = unsafe { xvsadd_hu(v256, v256) };
//! let _ = unsafe { xvsadd_wu(v256, v256) };
//! let _ = unsafe { xvsadd_du(v256, v256) };
//! let _ = unsafe { xvssub_bu(v256, v256) };
//! let _ = unsafe { xvssub_hu(v256, v256) };
//! let _ = unsafe { xvssub_wu(v256, v256) };
//! let _ = unsafe { xvssub_du(v256, v256) };
//! let _ = unsafe { xvhaddw_h_b(v256, v256) };
//! let _ = unsafe { xvhaddw_w_h(v256, v256) };
//! let _ = unsafe { xvhaddw_d_w(v256, v256) };
//! let _ = unsafe { xvhaddw_q_d(v256, v256) };
//! let _ = unsafe { xvhsubw_h_b(v256, v256) };
//! let _ = unsafe { xvhsubw_w_h(v256, v256) };
//! let _ = unsafe { xvhsubw_d_w(v256, v256) };
//! let _ = unsafe { xvhsubw_q_d(v256, v256) };
//! let _ = unsafe { xvhaddw_hu_bu(v256, v256) };
//! let _ = unsafe { xvhaddw_wu_hu(v256, v256) };
//! let _ = unsafe { xvhaddw_du_wu(v256, v256) };
//! let _ = unsafe { xvhaddw_qu_du(v256, v256) };
//! let _ = unsafe { xvhsubw_hu_bu(v256, v256) };
//! let _ = unsafe { xvhsubw_wu_hu(v256, v256) };
//! let _ = unsafe { xvhsubw_du_wu(v256, v256) };
//! let _ = unsafe { xvhsubw_qu_du(v256, v256) };
//! let _ = unsafe { xvadda_b(v256, v256) };
//! let _ = unsafe { xvadda_h(v256, v256) };
//! let _ = unsafe { xvadda_w(v256, v256) };
//! let _ = unsafe { xvadda_d(v256, v256) };
//! let _ = unsafe { xvabsd_b(v256, v256) };
//! let _ = unsafe { xvabsd_h(v256, v256) };
//! let _ = unsafe { xvabsd_w(v256, v256) };
//! let _ = unsafe { xvabsd_d(v256, v256) };
//! let _ = unsafe { xvabsd_bu(v256, v256) };
//! let _ = unsafe { xvabsd_hu(v256, v256) };
//! let _ = unsafe { xvabsd_wu(v256, v256) };
//! let _ = unsafe { xvabsd_du(v256, v256) };
//! let _ = unsafe { xvavg_b(v256, v256) };
//! let _ = unsafe { xvavg_h(v256, v256) };
//! let _ = unsafe { xvavg_w(v256, v256) };
//! let _ = unsafe { xvavg_d(v256, v256) };
//! let _ = unsafe { xvavg_bu(v256, v256) };
//! let _ = unsafe { xvavg_hu(v256, v256) };
//! let _ = unsafe { xvavg_wu(v256, v256) };
//! let _ = unsafe { xvavg_du(v256, v256) };
//! let _ = unsafe { xvavgr_b(v256, v256) };
//! let _ = unsafe { xvavgr_h(v256, v256) };
//! let _ = unsafe { xvavgr_w(v256, v256) };
//! let _ = unsafe { xvavgr_d(v256, v256) };
//! let _ = unsafe { xvavgr_bu(v256, v256) };
//! let _ = unsafe { xvavgr_hu(v256, v256) };
//! let _ = unsafe { xvavgr_wu(v256, v256) };
//! let _ = unsafe { xvavgr_du(v256, v256) };
//! let _ = unsafe { xvmax_b(v256, v256) };
//! let _ = unsafe { xvmax_h(v256, v256) };
//! let _ = unsafe { xvmax_w(v256, v256) };
//! let _ = unsafe { xvmax_d(v256, v256) };
//! let _ = unsafe { xvmin_b(v256, v256) };
//! let _ = unsafe { xvmin_h(v256, v256) };
//! let _ = unsafe { xvmin_w(v256, v256) };
//! let _ = unsafe { xvmin_d(v256, v256) };
//! let _ = unsafe { xvmax_bu(v256, v256) };
//! let _ = unsafe { xvmax_hu(v256, v256) };
//! let _ = unsafe { xvmax_wu(v256, v256) };
//! let _ = unsafe { xvmax_du(v256, v256) };
//! let _ = unsafe { xvmin_bu(v256, v256) };
//! let _ = unsafe { xvmin_hu(v256, v256) };
//! let _ = unsafe { xvmin_wu(v256, v256) };
//! let _ = unsafe { xvmin_du(v256, v256) };
//! let _ = unsafe { xvmul_b(v256, v256) };
//! let _ = unsafe { xvmul_h(v256, v256) };
//! let _ = unsafe { xvmul_w(v256, v256) };
//! let _ = unsafe { xvmul_d(v256, v256) };
//! let _ = unsafe { xvmuh_b(v256, v256) };
//! let _ = unsafe { xvmuh_h(v256, v256) };
//! let _ = unsafe { xvmuh_w(v256, v256) };
//! let _ = unsafe { xvmuh_d(v256, v256) };
//! let _ = unsafe { xvmuh_bu(v256, v256) };
//! let _ = unsafe { xvmuh_hu(v256, v256) };
//! let _ = unsafe { xvmuh_wu(v256, v256) };
//! let _ = unsafe { xvmuh_du(v256, v256) };
//! let _ = unsafe { xvmulwev_h_b(v256, v256) };
//! let _ = unsafe { xvmulwev_w_h(v256, v256) };
//! let _ = unsafe { xvmulwev_d_w(v256, v256) };
//! let _ = unsafe { xvmulwev_q_d(v256, v256) };
//! let _ = unsafe { xvmulwod_h_b(v256, v256) };
//! let _ = unsafe { xvmulwod_w_h(v256, v256) };
//! let _ = unsafe { xvmulwod_d_w(v256, v256) };
//! let _ = unsafe { xvmulwod_q_d(v256, v256) };
//! let _ = unsafe { xvmulwev_h_bu(v256, v256) };
//! let _ = unsafe { xvmulwev_w_hu(v256, v256) };
//! let _ = unsafe { xvmulwev_d_wu(v256, v256) };
//! let _ = unsafe { xvmulwev_q_du(v256, v256) };
//! let _ = unsafe { xvmulwod_h_bu(v256, v256) };
//! let _ = unsafe { xvmulwod_w_hu(v256, v256) };
//! let _ = unsafe { xvmulwod_d_wu(v256, v256) };
//! let _ = unsafe { xvmulwod_q_du(v256, v256) };
//! let _ = unsafe { xvmulwev_h_bu_b(v256, v256) };
//! let _ = unsafe { xvmulwev_w_hu_h(v256, v256) };
//! let _ = unsafe { xvmulwev_d_wu_w(v256, v256) };
//! let _ = unsafe { xvmulwev_q_du_d(v256, v256) };
//! let _ = unsafe { xvmulwod_h_bu_b(v256, v256) };
//! let _ = unsafe { xvmulwod_w_hu_h(v256, v256) };
//! let _ = unsafe { xvmulwod_d_wu_w(v256, v256) };
//! let _ = unsafe { xvmulwod_q_du_d(v256, v256) };
//! let _ = unsafe { xvmadd_b(v256, v256, v256) };
//! let _ = unsafe { xvmadd_h(v256, v256, v256) };
//! let _ = unsafe { xvmadd_w(v256, v256, v256) };
//! let _ = unsafe { xvmadd_d(v256, v256, v256) };
//! let _ = unsafe { xvmsub_b(v256, v256, v256) };
//! let _ = unsafe { xvmsub_h(v256, v256, v256) };
//! let _ = unsafe { xvmsub_w(v256, v256, v256) };
//! let _ = unsafe { xvmsub_d(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_h_b(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_w_h(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_d_w(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_q_d(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_h_b(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_w_h(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_d_w(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_q_d(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_h_bu(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_w_hu(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_d_wu(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_q_du(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_h_bu(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_w_hu(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_d_wu(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_q_du(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_h_bu_b(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_w_hu_h(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_d_wu_w(v256, v256, v256) };
//! let _ = unsafe { xvmaddwev_q_du_d(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_h_bu_b(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_w_hu_h(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_d_wu_w(v256, v256, v256) };
//! let _ = unsafe { xvmaddwod_q_du_d(v256, v256, v256) };
//! let _ = unsafe { xvdiv_b(v256, v256) };
//! let _ = unsafe { xvdiv_h(v256, v256) };
//! let _ = unsafe { xvdiv_w(v256, v256) };
//! let _ = unsafe { xvdiv_d(v256, v256) };
//! let _ = unsafe { xvmod_b(v256, v256) };
//! let _ = unsafe { xvmod_h(v256, v256) };
//! let _ = unsafe { xvmod_w(v256, v256) };
//! let _ = unsafe { xvmod_d(v256, v256) };
//! let _ = unsafe { xvdiv_bu(v256, v256) };
//! let _ = unsafe { xvdiv_hu(v256, v256) };
//! let _ = unsafe { xvdiv_wu(v256, v256) };
//! let _ = unsafe { xvdiv_du(v256, v256) };
//! let _ = unsafe { xvmod_bu(v256, v256) };
//! let _ = unsafe { xvmod_hu(v256, v256) };
//! let _ = unsafe { xvmod_wu(v256, v256) };
//! let _ = unsafe { xvmod_du(v256, v256) };
//! let _ = unsafe { xvsll_b(v256, v256) };
//! let _ = unsafe { xvsll_h(v256, v256) };
//! let _ = unsafe { xvsll_w(v256, v256) };
//! let _ = unsafe { xvsll_d(v256, v256) };
//! let _ = unsafe { xvsrl_b(v256, v256) };
//! let _ = unsafe { xvsrl_h(v256, v256) };
//! let _ = unsafe { xvsrl_w(v256, v256) };
//! let _ = unsafe { xvsrl_d(v256, v256) };
//! let _ = unsafe { xvsra_b(v256, v256) };
//! let _ = unsafe { xvsra_h(v256, v256) };
//! let _ = unsafe { xvsra_w(v256, v256) };
//! let _ = unsafe { xvsra_d(v256, v256) };
//! let _ = unsafe { xvrotr_b(v256, v256) };
//! let _ = unsafe { xvrotr_h(v256, v256) };
//! let _ = unsafe { xvrotr_w(v256, v256) };
//! let _ = unsafe { xvrotr_d(v256, v256) };
//! let _ = unsafe { xvsrlr_b(v256, v256) };
//! let _ = unsafe { xvsrlr_h(v256, v256) };
//! let _ = unsafe { xvsrlr_w(v256, v256) };
//! let _ = unsafe { xvsrlr_d(v256, v256) };
//! let _ = unsafe { xvsrar_b(v256, v256) };
//! let _ = unsafe { xvsrar_h(v256, v256) };
//! let _ = unsafe { xvsrar_w(v256, v256) };
//! let _ = unsafe { xvsrar_d(v256, v256) };
//! let _ = unsafe { xvsrln_b_h(v256, v256) };
//! let _ = unsafe { xvsrln_h_w(v256, v256) };
//! let _ = unsafe { xvsrln_w_d(v256, v256) };
//! let _ = unsafe { xvsran_b_h(v256, v256) };
//! let _ = unsafe { xvsran_h_w(v256, v256) };
//! let _ = unsafe { xvsran_w_d(v256, v256) };
//! let _ = unsafe { xvsrlrn_b_h(v256, v256) };
//! let _ = unsafe { xvsrlrn_h_w(v256, v256) };
//! let _ = unsafe { xvsrlrn_w_d(v256, v256) };
//! let _ = unsafe { xvsrarn_b_h(v256, v256) };
//! let _ = unsafe { xvsrarn_h_w(v256, v256) };
//! let _ = unsafe { xvsrarn_w_d(v256, v256) };
//! let _ = unsafe { xvssrln_b_h(v256, v256) };
//! let _ = unsafe { xvssrln_h_w(v256, v256) };
//! let _ = unsafe { xvssrln_w_d(v256, v256) };
//! let _ = unsafe { xvssran_b_h(v256, v256) };
//! let _ = unsafe { xvssran_h_w(v256, v256) };
//! let _ = unsafe { xvssran_w_d(v256, v256) };
//! let _ = unsafe { xvssrlrn_b_h(v256, v256) };
//! let _ = unsafe { xvssrlrn_h_w(v256, v256) };
//! let _ = unsafe { xvssrlrn_w_d(v256, v256) };
//! let _ = unsafe { xvssrarn_b_h(v256, v256) };
//! let _ = unsafe { xvssrarn_h_w(v256, v256) };
//! let _ = unsafe { xvssrarn_w_d(v256, v256) };
//! let _ = unsafe { xvssrln_bu_h(v256, v256) };
//! let _ = unsafe { xvssrln_hu_w(v256, v256) };
//! let _ = unsafe { xvssrln_wu_d(v256, v256) };
//! let _ = unsafe { xvssran_bu_h(v256, v256) };
//! let _ = unsafe { xvssran_hu_w(v256, v256) };
//! let _ = unsafe { xvssran_wu_d(v256, v256) };
//! let _ = unsafe { xvssrlrn_bu_h(v256, v256) };
//! let _ = unsafe { xvssrlrn_hu_w(v256, v256) };
//! let _ = unsafe { xvssrlrn_wu_d(v256, v256) };
//! let _ = unsafe { xvssrarn_bu_h(v256, v256) };
//! let _ = unsafe { xvssrarn_hu_w(v256, v256) };
//! let _ = unsafe { xvssrarn_wu_d(v256, v256) };
//! let _ = unsafe { xvbitclr_b(v256, v256) };
//! let _ = unsafe { xvbitclr_h(v256, v256) };
//! let _ = unsafe { xvbitclr_w(v256, v256) };
//! let _ = unsafe { xvbitclr_d(v256, v256) };
//! let _ = unsafe { xvbitset_b(v256, v256) };
//! let _ = unsafe { xvbitset_h(v256, v256) };
//! let _ = unsafe { xvbitset_w(v256, v256) };
//! let _ = unsafe { xvbitset_d(v256, v256) };
//! let _ = unsafe { xvbitrev_b(v256, v256) };
//! let _ = unsafe { xvbitrev_h(v256, v256) };
//! let _ = unsafe { xvbitrev_w(v256, v256) };
//! let _ = unsafe { xvbitrev_d(v256, v256) };
//! let _ = unsafe { xvpackev_b(v256, v256) };
//! let _ = unsafe { xvpackev_h(v256, v256) };
//! let _ = unsafe { xvpackev_w(v256, v256) };
//! let _ = unsafe { xvpackev_d(v256, v256) };
//! let _ = unsafe { xvpackod_b(v256, v256) };
//! let _ = unsafe { xvpackod_h(v256, v256) };
//! let _ = unsafe { xvpackod_w(v256, v256) };
//! let _ = unsafe { xvpackod_d(v256, v256) };
//! let _ = unsafe { xvilvl_b(v256, v256) };
//! let _ = unsafe { xvilvl_h(v256, v256) };
//! let _ = unsafe { xvilvl_w(v256, v256) };
//! let _ = unsafe { xvilvl_d(v256, v256) };
//! let _ = unsafe { xvilvh_b(v256, v256) };
//! let _ = unsafe { xvilvh_h(v256, v256) };
//! let _ = unsafe { xvilvh_w(v256, v256) };
//! let _ = unsafe { xvilvh_d(v256, v256) };
//! let _ = unsafe { xvpickev_b(v256, v256) };
//! let _ = unsafe { xvpickev_h(v256, v256) };
//! let _ = unsafe { xvpickev_w(v256, v256) };
//! let _ = unsafe { xvpickev_d(v256, v256) };
//! let _ = unsafe { xvpickod_b(v256, v256) };
//! let _ = unsafe { xvpickod_h(v256, v256) };
//! let _ = unsafe { xvpickod_w(v256, v256) };
//! let _ = unsafe { xvpickod_d(v256, v256) };
//! let _ = unsafe { xvand_v(v256, v256) };
//! let _ = unsafe { xvor_v(v256, v256) };
//! let _ = unsafe { xvxor_v(v256, v256) };
//! let _ = unsafe { xvnor_v(v256, v256) };
//! let _ = unsafe { xvandn_v(v256, v256) };
//! let _ = unsafe { xvorn_v(v256, v256) };
//! let _ = unsafe { xvfrstp_b(v256, v256, v256) };
//! let _ = unsafe { xvfrstp_h(v256, v256, v256) };
//! let _ = unsafe { xvadd_q(v256, v256) };
//! let _ = unsafe { xvsub_q(v256, v256) };
//! let _ = unsafe { xvsigncov_b(v256, v256) };
//! let _ = unsafe { xvsigncov_h(v256, v256) };
//! let _ = unsafe { xvsigncov_w(v256, v256) };
//! let _ = unsafe { xvsigncov_d(v256, v256) };
//! let _ = unsafe { xvftint_w_d(v256, v256) };
//! let _ = unsafe { xvftintrm_w_d(v256, v256) };
//! let _ = unsafe { xvftintrp_w_d(v256, v256) };
//! let _ = unsafe { xvftintrz_w_d(v256, v256) };
//! let _ = unsafe { xvftintrne_w_d(v256, v256) };
//! let _ = unsafe { xvshuf_h(v256, v256, v256) };
//! let _ = unsafe { xvshuf_w(v256, v256, v256) };
//! let _ = unsafe { xvshuf_d(v256, v256, v256) };
//! let _ = unsafe { xvperm_w(v256, v256) };
//! let _ = unsafe { xvclo_b(v256) };
//! let _ = unsafe { xvclo_h(v256) };
//! let _ = unsafe { xvclo_w(v256) };
//! let _ = unsafe { xvclo_d(v256) };
//! let _ = unsafe { xvclz_b(v256) };
//! let _ = unsafe { xvclz_h(v256) };
//! let _ = unsafe { xvclz_w(v256) };
//! let _ = unsafe { xvclz_d(v256) };
//! let _ = unsafe { xvpcnt_b(v256) };
//! let _ = unsafe { xvpcnt_h(v256) };
//! let _ = unsafe { xvpcnt_w(v256) };
//! let _ = unsafe { xvpcnt_d(v256) };
//! let _ = unsafe { xvneg_b(v256) };
//! let _ = unsafe { xvneg_h(v256) };
//! let _ = unsafe { xvneg_w(v256) };
//! let _ = unsafe { xvneg_d(v256) };
//! let _ = unsafe { xvmskltz_b(v256) };
//! let _ = unsafe { xvmskltz_h(v256) };
//! let _ = unsafe { xvmskltz_w(v256) };
//! let _ = unsafe { xvmskltz_d(v256) };
//! let _ = unsafe { xvmskgez_b(v256) };
//! let _ = unsafe { xvmsknz_b(v256) };
//! let _ = unsafe { xvfclass_s(v256) };
//! let _ = unsafe { xvfclass_d(v256) };
//! let _ = unsafe { xvftint_w_s(v256) };
//! let _ = unsafe { xvftint_l_d(v256) };
//! let _ = unsafe { xvftintrm_w_s(v256) };
//! let _ = unsafe { xvftintrm_l_d(v256) };
//! let _ = unsafe { xvftintrp_w_s(v256) };
//! let _ = unsafe { xvftintrp_l_d(v256) };
//! let _ = unsafe { xvftintrz_w_s(v256) };
//! let _ = unsafe { xvftintrz_l_d(v256) };
//! let _ = unsafe { xvftintrne_w_s(v256) };
//! let _ = unsafe { xvftintrne_l_d(v256) };
//! let _ = unsafe { xvftint_wu_s(v256) };
//! let _ = unsafe { xvftint_lu_d(v256) };
//! let _ = unsafe { xvftintrz_wu_s(v256) };
//! let _ = unsafe { xvftintrz_lu_d(v256) };
//! let _ = unsafe { xvftintl_l_s(v256) };
//! let _ = unsafe { xvftinth_l_s(v256) };
//! let _ = unsafe { xvftintrml_l_s(v256) };
//! let _ = unsafe { xvftintrmh_l_s(v256) };
//! let _ = unsafe { xvftintrpl_l_s(v256) };
//! let _ = unsafe { xvftintrph_l_s(v256) };
//! let _ = unsafe { xvftintrzl_l_s(v256) };
//! let _ = unsafe { xvftintrzh_l_s(v256) };
//! let _ = unsafe { xvftintrnel_l_s(v256) };
//! let _ = unsafe { xvftintrneh_l_s(v256) };
//! let _ = unsafe { xvexth_h_b(v256) };
//! let _ = unsafe { xvexth_w_h(v256) };
//! let _ = unsafe { xvexth_d_w(v256) };
//! let _ = unsafe { xvexth_q_d(v256) };
//! let _ = unsafe { xvexth_hu_bu(v256) };
//! let _ = unsafe { xvexth_wu_hu(v256) };
//! let _ = unsafe { xvexth_du_wu(v256) };
//! let _ = unsafe { xvexth_qu_du(v256) };
//! let _ = unsafe { vext2xv_h_b(v256) };
//! let _ = unsafe { vext2xv_w_b(v256) };
//! let _ = unsafe { vext2xv_d_b(v256) };
//! let _ = unsafe { vext2xv_w_h(v256) };
//! let _ = unsafe { vext2xv_d_h(v256) };
//! let _ = unsafe { vext2xv_d_w(v256) };
//! let _ = unsafe { vext2xv_hu_bu(v256) };
//! let _ = unsafe { vext2xv_wu_bu(v256) };
//! let _ = unsafe { vext2xv_du_bu(v256) };
//! let _ = unsafe { vext2xv_wu_hu(v256) };
//! let _ = unsafe { vext2xv_du_hu(v256) };
//! let _ = unsafe { vext2xv_du_wu(v256) };
//! let _ = unsafe { xvextl_q_d(v256) };
//! let _ = unsafe { xvextl_qu_du(v256) };
//! let _ = unsafe { xvslei_bu::<1>(v256) };
//! let _ = unsafe { xvslti_bu::<1>(v256) };
//! let _ = unsafe { xvmaxi_bu::<1>(v256) };
//! let _ = unsafe { xvmini_bu::<1>(v256) };
//! let _ = unsafe { xvslei_hu::<1>(v256) };
//! let _ = unsafe { xvslti_hu::<1>(v256) };
//! let _ = unsafe { xvmaxi_hu::<1>(v256) };
//! let _ = unsafe { xvmini_hu::<1>(v256) };
//! let _ = unsafe { xvslei_wu::<1>(v256) };
//! let _ = unsafe { xvslti_wu::<1>(v256) };
//! let _ = unsafe { xvmaxi_wu::<1>(v256) };
//! let _ = unsafe { xvmini_wu::<1>(v256) };
//! let _ = unsafe { xvslei_du::<1>(v256) };
//! let _ = unsafe { xvslti_du::<1>(v256) };
//! let _ = unsafe { xvmaxi_du::<1>(v256) };
//! let _ = unsafe { xvmini_du::<1>(v256) };
//! let _ = unsafe { xvseqi_b::<1>(v256) };
//! let _ = unsafe { xvslei_b::<1>(v256) };
//! let _ = unsafe { xvslei_bu::<1>(v256) };
//! let _ = unsafe { xvslti_b::<1>(v256) };
//! let _ = unsafe { xvslti_bu::<1>(v256) };
//! let _ = unsafe { xvslli_b::<1>(v256) };
//! let _ = unsafe { xvsrli_b::<1>(v256) };
//! let _ = unsafe { xvsrai_b::<1>(v256) };
//! let _ = unsafe { xvmaxi_b::<1>(v256) };
//! let _ = unsafe { xvmaxi_bu::<1>(v256) };
//! let _ = unsafe { xvmini_b::<1>(v256) };
//! let _ = unsafe { xvmini_bu::<1>(v256) };
//! let _ = unsafe { xvseqi_h::<1>(v256) };
//! let _ = unsafe { xvslei_h::<1>(v256) };
//! let _ = unsafe { xvslei_hu::<1>(v256) };
//! let _ = unsafe { xvslti_h::<1>(v256) };
//! let _ = unsafe { xvslti_hu::<1>(v256) };
//! let _ = unsafe { xvslli_h::<1>(v256) };
//! let _ = unsafe { xvsrli_h::<1>(v256) };
//! let _ = unsafe { xvsrai_h::<1>(v256) };
//! let _ = unsafe { xvmaxi_h::<1>(v256) };
//! let _ = unsafe { xvmaxi_hu::<1>(v256) };
//! let _ = unsafe { xvmini_h::<1>(v256) };
//! let _ = unsafe { xvmini_hu::<1>(v256) };
//! let _ = unsafe { xvseqi_w::<1>(v256) };
//! let _ = unsafe { xvslei_w::<1>(v256) };
//! let _ = unsafe { xvslei_wu::<1>(v256) };
//! let _ = unsafe { xvslti_w::<1>(v256) };
//! let _ = unsafe { xvslti_wu::<1>(v256) };
//! let _ = unsafe { xvslli_w::<1>(v256) };
//! let _ = unsafe { xvsrli_w::<1>(v256) };
//! let _ = unsafe { xvsrai_w::<1>(v256) };
//! let _ = unsafe { xvmaxi_w::<1>(v256) };
//! let _ = unsafe { xvmaxi_wu::<1>(v256) };
//! let _ = unsafe { xvmini_w::<1>(v256) };
//! let _ = unsafe { xvmini_wu::<1>(v256) };
//! let _ = unsafe { xvseqi_d::<1>(v256) };
//! let _ = unsafe { xvslei_d::<1>(v256) };
//! let _ = unsafe { xvslei_du::<1>(v256) };
//! let _ = unsafe { xvslti_d::<1>(v256) };
//! let _ = unsafe { xvslti_du::<1>(v256) };
//! let _ = unsafe { xvslli_d::<1>(v256) };
//! let _ = unsafe { xvsrli_d::<1>(v256) };
//! let _ = unsafe { xvsrai_d::<1>(v256) };
//! let _ = unsafe { xvmaxi_d::<1>(v256) };
//! let _ = unsafe { xvmaxi_du::<1>(v256) };
//! let _ = unsafe { xvmini_d::<1>(v256) };
//! let _ = unsafe { xvmini_du::<1>(v256) };
//! let _ = unsafe { xvrotri_b::<1>(v256) };
//! let _ = unsafe { xvbitclri_b::<1>(v256) };
//! let _ = unsafe { xvbitseti_b::<1>(v256) };
//! let _ = unsafe { xvbitrevi_b::<1>(v256) };
//! let _ = unsafe { xvrotri_h::<1>(v256) };
//! let _ = unsafe { xvbitclri_h::<1>(v256) };
//! let _ = unsafe { xvbitseti_h::<1>(v256) };
//! let _ = unsafe { xvbitrevi_h::<1>(v256) };
//! let _ = unsafe { xvrotri_w::<1>(v256) };
//! let _ = unsafe { xvbitclri_w::<1>(v256) };
//! let _ = unsafe { xvbitseti_w::<1>(v256) };
//! let _ = unsafe { xvbitrevi_w::<1>(v256) };
//! let _ = unsafe { xvrotri_d::<1>(v256) };
//! let _ = unsafe { xvbitclri_d::<1>(v256) };
//! let _ = unsafe { xvbitseti_d::<1>(v256) };
//! let _ = unsafe { xvbitrevi_d::<1>(v256) };
//! let _ = unsafe { xvandi_b::<1>(v256) };
//! let _ = unsafe { xvori_b::<1>(v256) };
//! let _ = unsafe { xvxori_b::<1>(v256) };
//! let _ = unsafe { xvnori_b::<1>(v256) };
//! let _ = unsafe { xvaddi_bu::<1>(v256) };
//! let _ = unsafe { xvsubi_bu::<1>(v256) };
//! let _ = unsafe { xvsrlri_b::<1>(v256) };
//! let _ = unsafe { xvsrari_b::<1>(v256) };
//! let _ = unsafe { xvaddi_hu::<1>(v256) };
//! let _ = unsafe { xvsubi_hu::<1>(v256) };
//! let _ = unsafe { xvsrlri_h::<1>(v256) };
//! let _ = unsafe { xvsrari_h::<1>(v256) };
//! let _ = unsafe { xvaddi_wu::<1>(v256) };
//! let _ = unsafe { xvsubi_wu::<1>(v256) };
//! let _ = unsafe { xvsrlri_w::<1>(v256) };
//! let _ = unsafe { xvsrari_w::<1>(v256) };
//! let _ = unsafe { xvaddi_du::<1>(v256) };
//! let _ = unsafe { xvsubi_du::<1>(v256) };
//! let _ = unsafe { xvsrlri_d::<1>(v256) };
//! let _ = unsafe { xvsrari_d::<1>(v256) };
//! let _ = unsafe { xvfadd_s(v256, v256) };
//! let _ = unsafe { xvfadd_d(v256, v256) };
//! let _ = unsafe { xvfsub_s(v256, v256) };
//! let _ = unsafe { xvfsub_d(v256, v256) };
//! let _ = unsafe { xvfmul_s(v256, v256) };
//! let _ = unsafe { xvfmul_d(v256, v256) };
//! let _ = unsafe { xvfdiv_s(v256, v256) };
//! let _ = unsafe { xvfdiv_d(v256, v256) };
//! let _ = unsafe { xvfmax_s(v256, v256) };
//! let _ = unsafe { xvfmax_d(v256, v256) };
//! let _ = unsafe { xvfmin_s(v256, v256) };
//! let _ = unsafe { xvfmin_d(v256, v256) };
//! let _ = unsafe { xvfmaxa_s(v256, v256) };
//! let _ = unsafe { xvfmaxa_d(v256, v256) };
//! let _ = unsafe { xvfmina_s(v256, v256) };
//! let _ = unsafe { xvfmina_d(v256, v256) };
//! let _ = unsafe { xvffint_s_l(v256, v256) };
//! let _ = unsafe { xvflogb_s(v256) };
//! let _ = unsafe { xvflogb_d(v256) };
//! let _ = unsafe { xvfsqrt_s(v256) };
//! let _ = unsafe { xvfsqrt_d(v256) };
//! let _ = unsafe { xvfrecip_s(v256) };
//! let _ = unsafe { xvfrecip_d(v256) };
//! let _ = unsafe { xvfrsqrt_s(v256) };
//! let _ = unsafe { xvfrsqrt_d(v256) };
//! let _ = unsafe { xvfrint_s(v256) };
//! let _ = unsafe { xvfrint_d(v256) };
//! let _ = unsafe { xvfrintrm_s(v256) };
//! let _ = unsafe { xvfrintrm_d(v256) };
//! let _ = unsafe { xvfrintrp_s(v256) };
//! let _ = unsafe { xvfrintrp_d(v256) };
//! let _ = unsafe { xvfrintrz_s(v256) };
//! let _ = unsafe { xvfrintrz_d(v256) };
//! let _ = unsafe { xvfrintrne_s(v256) };
//! let _ = unsafe { xvfrintrne_d(v256) };
//! let _ = unsafe { xvffint_s_w(v256) };
//! let _ = unsafe { xvffint_s_wu(v256) };
//! let _ = unsafe { xvffint_d_l(v256) };
//! let _ = unsafe { xvffint_d_lu(v256) };
//! let _ = unsafe { xvffintl_d_w(v256) };
//! let _ = unsafe { xvffinth_d_w(v256) };
//! let _ = unsafe { xvldrepl_b(storage.0.as_ptr(), 0) };
//! let _ = unsafe { xvldrepl_h(storage.0.as_ptr(), 0) };
//! let _ = unsafe { xvldrepl_w(storage.0.as_ptr(), 0) };
//! let _ = unsafe { xvldrepl_d(storage.0.as_ptr(), 0) };
//! let _ = unsafe { xvstelm_b::<0>(v256, storage.0.as_mut_ptr(), 0) };
//! let _ = unsafe { xvstelm_h::<0>(v256, storage.0.as_mut_ptr(), 0) };
//! let _ = unsafe { xvstelm_w::<0>(v256, storage.0.as_mut_ptr(), 0) };
//! let _ = unsafe { xvstelm_d::<0>(v256, storage.0.as_mut_ptr(), 0) };
//! let _ = unsafe { xvinsgr2vr_w::<0>(v256, 1) };
//! let _ = unsafe { xvinsgr2vr_d::<0>(v256, 1) };
//! let _ = unsafe { xvinsve0_w::<0>(v256, v128) };
//! let _ = unsafe { xvinsve0_d::<0>(v256, v128) };
//! let _ = unsafe { xvpickve_w::<0>(v256) };
//! let _ = unsafe { xvpickve_d::<0>(v256) };
//! let _ = unsafe { xvpickve2gr_w::<0>(v256) };
//! let _ = unsafe { xvpickve2gr_d::<0>(v256) };
//! let _ = unsafe { xvpickve2gr_wu::<0>(v256) };
//! let _ = unsafe { xvpickve2gr_du::<0>(v256) };
//! let _ = unsafe { xvsllwil_h_b::<1>(v256) };
//! let _ = unsafe { xvsllwil_hu_bu::<1>(v256) };
//! let _ = unsafe { xvsrlni_b_h::<1>(v256, v256) };
//! let _ = unsafe { xvsrlrni_b_h::<1>(v256, v256) };
//! let _ = unsafe { xvssrlni_b_h::<1>(v256, v256) };
//! let _ = unsafe { xvssrlni_bu_h::<1>(v256, v256) };
//! let _ = unsafe { xvssrlrni_b_h::<1>(v256, v256) };
//! let _ = unsafe { xvssrlrni_bu_h::<1>(v256, v256) };
//! let _ = unsafe { xvsrani_b_h::<1>(v256, v256) };
//! let _ = unsafe { xvsrarni_b_h::<1>(v256, v256) };
//! let _ = unsafe { xvssrani_b_h::<1>(v256, v256) };
//! let _ = unsafe { xvssrani_bu_h::<1>(v256, v256) };
//! let _ = unsafe { xvssrarni_b_h::<1>(v256, v256) };
//! let _ = unsafe { xvssrarni_bu_h::<1>(v256, v256) };
//! let _ = unsafe { xvsllwil_w_h::<1>(v256) };
//! let _ = unsafe { xvsllwil_wu_hu::<1>(v256) };
//! let _ = unsafe { xvsrlni_h_w::<1>(v256, v256) };
//! let _ = unsafe { xvsrlrni_h_w::<1>(v256, v256) };
//! let _ = unsafe { xvssrlni_h_w::<1>(v256, v256) };
//! let _ = unsafe { xvssrlni_hu_w::<1>(v256, v256) };
//! let _ = unsafe { xvssrlrni_h_w::<1>(v256, v256) };
//! let _ = unsafe { xvssrlrni_hu_w::<1>(v256, v256) };
//! let _ = unsafe { xvsrani_h_w::<1>(v256, v256) };
//! let _ = unsafe { xvsrarni_h_w::<1>(v256, v256) };
//! let _ = unsafe { xvssrani_h_w::<1>(v256, v256) };
//! let _ = unsafe { xvssrani_hu_w::<1>(v256, v256) };
//! let _ = unsafe { xvssrarni_h_w::<1>(v256, v256) };
//! let _ = unsafe { xvssrarni_hu_w::<1>(v256, v256) };
//! let _ = unsafe { xvsllwil_d_w::<1>(v256) };
//! let _ = unsafe { xvsllwil_du_wu::<1>(v256) };
//! let _ = unsafe { xvsrlni_w_d::<1>(v256, v256) };
//! let _ = unsafe { xvsrlrni_w_d::<1>(v256, v256) };
//! let _ = unsafe { xvssrlni_w_d::<1>(v256, v256) };
//! let _ = unsafe { xvssrlni_wu_d::<1>(v256, v256) };
//! let _ = unsafe { xvssrlrni_w_d::<1>(v256, v256) };
//! let _ = unsafe { xvssrlrni_wu_d::<1>(v256, v256) };
//! let _ = unsafe { xvsrani_w_d::<1>(v256, v256) };
//! let _ = unsafe { xvsrarni_w_d::<1>(v256, v256) };
//! let _ = unsafe { xvssrani_w_d::<1>(v256, v256) };
//! let _ = unsafe { xvssrani_wu_d::<1>(v256, v256) };
//! let _ = unsafe { xvssrarni_w_d::<1>(v256, v256) };
//! let _ = unsafe { xvssrarni_wu_d::<1>(v256, v256) };
//! let _ = unsafe { xvsrlni_d_q::<1>(v256, v256) };
//! let _ = unsafe { xvsrlrni_d_q::<1>(v256, v256) };
//! let _ = unsafe { xvssrlni_d_q::<1>(v256, v256) };
//! let _ = unsafe { xvssrlni_du_q::<1>(v256, v256) };
//! let _ = unsafe { xvssrlrni_d_q::<1>(v256, v256) };
//! let _ = unsafe { xvssrlrni_du_q::<1>(v256, v256) };
//! let _ = unsafe { xvsrani_d_q::<1>(v256, v256) };
//! let _ = unsafe { xvsrarni_d_q::<1>(v256, v256) };
//! let _ = unsafe { xvssrani_d_q::<1>(v256, v256) };
//! let _ = unsafe { xvssrani_du_q::<1>(v256, v256) };
//! let _ = unsafe { xvssrarni_d_q::<1>(v256, v256) };
//! let _ = unsafe { xvssrarni_du_q::<1>(v256, v256) };
//! let _ = unsafe { xvbsll(v256, 1) };
//! let _ = unsafe { xvbsrl(v256, 1) };
//! let _ = unsafe { xvseteqz(v256) };
//! let _ = unsafe { xvsetnez(v256) };
//! let _ = unsafe { xvsetanyeqz_b(v256) };
//! let _ = unsafe { xvsetanyeqz_h(v256) };
//! let _ = unsafe { xvsetanyeqz_w(v256) };
//! let _ = unsafe { xvsetanyeqz_d(v256) };
//! let _ = unsafe { xvsetallnez_b(v256) };
//! let _ = unsafe { xvsetallnez_h(v256) };
//! let _ = unsafe { xvsetallnez_w(v256) };
//! let _ = unsafe { xvsetallnez_d(v256) };
//! let _ = unsafe { xvsat_b::<1>(v256) };
//! let _ = unsafe { xvsat_h::<1>(v256) };
//! let _ = unsafe { xvsat_w::<1>(v256) };
//! let _ = unsafe { xvsat_d::<1>(v256) };
//! let _ = unsafe { xvsat_bu::<1>(v256) };
//! let _ = unsafe { xvsat_hu::<1>(v256) };
//! let _ = unsafe { xvsat_wu::<1>(v256) };
//! let _ = unsafe { xvsat_du::<1>(v256) };
//! let _ = unsafe { xvbitseli_b::<0x0f>(v256, v256) };
//! let _ = unsafe { xvshuf4i_b::<0>(v256) };
//! let _ = unsafe { xvshuf4i_h::<0>(v256) };
//! let _ = unsafe { xvshuf4i_w::<0>(v256) };
//! let _ = unsafe { xvshuf4i_d::<0>(v256, v256) };
//! let _ = unsafe { xvpermi_w::<0>(v256, v256) };
//! let _ = unsafe { xvpermi_d::<0>(v256) };
//! let _ = unsafe { xvpermi_q::<0>(v256, v256) };
//! let _ = unsafe { xvfrecipe_s(v256) };
//! let _ = unsafe { xvfrecipe_d(v256) };
//! let _ = unsafe { xvfrsqrte_s(v256) };
//! let _ = unsafe { xvfrsqrte_d(v256) };
//! let _ = unsafe { xvextrins_b::<0>(v256, v256) };
//! let _ = unsafe { xvextrins_h::<0>(v256, v256) };
//! let _ = unsafe { xvextrins_w::<0>(v256, v256) };
//! let _ = unsafe { xvextrins_d::<0>(v256, v256) };
//! let _ = unsafe { xvfrstpi_b::<0>(v256, v256) };
//! let _ = unsafe { xvfrstpi_h::<0>(v256, v256) };
//! let _ = unsafe { xvfcvt_s_d(v256, v256) };
//! let _ = unsafe { xvfcvtl_s_h(v256) };
//! let _ = unsafe { xvfcvth_s_h(v256) };
//! let _ = unsafe { xvfcvtl_d_s(v256) };
//! let _ = unsafe { xvfcvth_d_s(v256) };
//! let _ = unsafe { xvfcvt_h_s(v256, v256) };
//! let _ = unsafe { xvreplgr2vr_b(0) };
//! let _ = unsafe { xvreplgr2vr_h(0) };
//! let _ = unsafe { xvreplgr2vr_w(0) };
//! let _ = unsafe { xvreplgr2vr_d(0) };
//! let _ = unsafe { xvreplve_b(v256, 0) };
//! let _ = unsafe { xvreplve_h(v256, 0) };
//! let _ = unsafe { xvreplve_w(v256, 0) };
//! let _ = unsafe { xvreplve_d(v256, 0) };
//! let _ = unsafe { xvreplve0_b(v256) };
//! let _ = unsafe { xvreplve0_h(v256) };
//! let _ = unsafe { xvreplve0_w(v256) };
//! let _ = unsafe { xvreplve0_d(v256) };
//! let _ = unsafe { xvreplve0_q(v256) };
//! let _ = unsafe { xvrepl128vei_b::<0>(v256) };
//! let _ = unsafe { xvrepl128vei_h::<0>(v256) };
//! let _ = unsafe { xvrepl128vei_w::<0>(v256) };
//! let _ = unsafe { xvrepl128vei_d::<0>(v256) };
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::loongarch::simd::lasx::add as _;
//! use rust_isa::loongarch::simd::lasx::add_i64 as _;
//! use rust_isa::loongarch::simd::lasx::and as _;
//! use rust_isa::loongarch::simd::lasx::fadd as _;
//! use rust_isa::loongarch::simd::lasx::fdiv as _;
//! use rust_isa::loongarch::simd::lasx::fmul as _;
//! use rust_isa::loongarch::simd::lasx::fsub as _;
//! use rust_isa::loongarch::simd::lasx::mul as _;
//! use rust_isa::loongarch::simd::lasx::mul_i32 as _;
//! use rust_isa::loongarch::simd::lasx::or as _;
//! use rust_isa::loongarch::simd::lasx::splat_i32 as _;
//! use rust_isa::loongarch::simd::lasx::sub as _;
//! use rust_isa::loongarch::simd::lasx::sub_i64 as _;
//! use rust_isa::loongarch::simd::lasx::v256_load as _;
//! use rust_isa::loongarch::simd::lasx::v256_store as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_d_b as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_d_h as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_d_w as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_du_bu as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_du_hu as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_du_wu as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_h_b as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_hu_bu as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_w_b as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_w_h as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_wu_bu as _;
//! use rust_isa::loongarch::simd::lasx::vext2xv_wu_hu as _;
//! use rust_isa::loongarch::simd::lasx::xor as _;
//! use rust_isa::loongarch::simd::lasx::xvabs as _;
//! use rust_isa::loongarch::simd::lasx::xvabs_b as _;
//! use rust_isa::loongarch::simd::lasx::xvabs_d as _;
//! use rust_isa::loongarch::simd::lasx::xvabs_h as _;
//! use rust_isa::loongarch::simd::lasx::xvabsd_b as _;
//! use rust_isa::loongarch::simd::lasx::xvabsd_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvabsd_d as _;
//! use rust_isa::loongarch::simd::lasx::xvabsd_du as _;
//! use rust_isa::loongarch::simd::lasx::xvabsd_h as _;
//! use rust_isa::loongarch::simd::lasx::xvabsd_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvabsd_w as _;
//! use rust_isa::loongarch::simd::lasx::xvabsd_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvadd as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_b as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_d as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_h as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_q as _;
//! use rust_isa::loongarch::simd::lasx::xvadd_w as _;
//! use rust_isa::loongarch::simd::lasx::xvadda_b as _;
//! use rust_isa::loongarch::simd::lasx::xvadda_d as _;
//! use rust_isa::loongarch::simd::lasx::xvadda_h as _;
//! use rust_isa::loongarch::simd::lasx::xvadda_w as _;
//! use rust_isa::loongarch::simd::lasx::xvaddi_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddi_du as _;
//! use rust_isa::loongarch::simd::lasx::xvaddi_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddi_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_d_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_d_wu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_h_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_h_bu_b as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_q_du as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_q_du_d as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_w_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwev_w_hu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_d_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_d_wu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_h_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_h_bu_b as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_q_du as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_q_du_d as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_w_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvaddwod_w_hu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvand as _;
//! use rust_isa::loongarch::simd::lasx::xvand_v as _;
//! use rust_isa::loongarch::simd::lasx::xvandi_b as _;
//! use rust_isa::loongarch::simd::lasx::xvandn_v as _;
//! use rust_isa::loongarch::simd::lasx::xvavg_b as _;
//! use rust_isa::loongarch::simd::lasx::xvavg_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvavg_d as _;
//! use rust_isa::loongarch::simd::lasx::xvavg_du as _;
//! use rust_isa::loongarch::simd::lasx::xvavg_h as _;
//! use rust_isa::loongarch::simd::lasx::xvavg_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvavg_w as _;
//! use rust_isa::loongarch::simd::lasx::xvavg_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr_b as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr_d as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr_du as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr_h as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr_w as _;
//! use rust_isa::loongarch::simd::lasx::xvavgr_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvbitclr_b as _;
//! use rust_isa::loongarch::simd::lasx::xvbitclr_d as _;
//! use rust_isa::loongarch::simd::lasx::xvbitclr_h as _;
//! use rust_isa::loongarch::simd::lasx::xvbitclr_w as _;
//! use rust_isa::loongarch::simd::lasx::xvbitclri_b as _;
//! use rust_isa::loongarch::simd::lasx::xvbitclri_d as _;
//! use rust_isa::loongarch::simd::lasx::xvbitclri_h as _;
//! use rust_isa::loongarch::simd::lasx::xvbitclri_w as _;
//! use rust_isa::loongarch::simd::lasx::xvbitrev_b as _;
//! use rust_isa::loongarch::simd::lasx::xvbitrev_d as _;
//! use rust_isa::loongarch::simd::lasx::xvbitrev_h as _;
//! use rust_isa::loongarch::simd::lasx::xvbitrev_w as _;
//! use rust_isa::loongarch::simd::lasx::xvbitrevi_b as _;
//! use rust_isa::loongarch::simd::lasx::xvbitrevi_d as _;
//! use rust_isa::loongarch::simd::lasx::xvbitrevi_h as _;
//! use rust_isa::loongarch::simd::lasx::xvbitrevi_w as _;
//! use rust_isa::loongarch::simd::lasx::xvbitsel as _;
//! use rust_isa::loongarch::simd::lasx::xvbitsel_v as _;
//! use rust_isa::loongarch::simd::lasx::xvbitseli_b as _;
//! use rust_isa::loongarch::simd::lasx::xvbitset_b as _;
//! use rust_isa::loongarch::simd::lasx::xvbitset_d as _;
//! use rust_isa::loongarch::simd::lasx::xvbitset_h as _;
//! use rust_isa::loongarch::simd::lasx::xvbitset_w as _;
//! use rust_isa::loongarch::simd::lasx::xvbitseti_b as _;
//! use rust_isa::loongarch::simd::lasx::xvbitseti_d as _;
//! use rust_isa::loongarch::simd::lasx::xvbitseti_h as _;
//! use rust_isa::loongarch::simd::lasx::xvbitseti_w as _;
//! use rust_isa::loongarch::simd::lasx::xvbsll as _;
//! use rust_isa::loongarch::simd::lasx::xvbsrl as _;
//! use rust_isa::loongarch::simd::lasx::xvclo_b as _;
//! use rust_isa::loongarch::simd::lasx::xvclo_d as _;
//! use rust_isa::loongarch::simd::lasx::xvclo_h as _;
//! use rust_isa::loongarch::simd::lasx::xvclo_w as _;
//! use rust_isa::loongarch::simd::lasx::xvclz_b as _;
//! use rust_isa::loongarch::simd::lasx::xvclz_d as _;
//! use rust_isa::loongarch::simd::lasx::xvclz_h as _;
//! use rust_isa::loongarch::simd::lasx::xvclz_w as _;
//! use rust_isa::loongarch::simd::lasx::xvdiv_b as _;
//! use rust_isa::loongarch::simd::lasx::xvdiv_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvdiv_d as _;
//! use rust_isa::loongarch::simd::lasx::xvdiv_du as _;
//! use rust_isa::loongarch::simd::lasx::xvdiv_h as _;
//! use rust_isa::loongarch::simd::lasx::xvdiv_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvdiv_w as _;
//! use rust_isa::loongarch::simd::lasx::xvdiv_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvexth_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvexth_du_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvexth_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvexth_hu_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvexth_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvexth_qu_du as _;
//! use rust_isa::loongarch::simd::lasx::xvexth_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvexth_wu_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvextl_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvextl_qu_du as _;
//! use rust_isa::loongarch::simd::lasx::xvextrins_b as _;
//! use rust_isa::loongarch::simd::lasx::xvextrins_d as _;
//! use rust_isa::loongarch::simd::lasx::xvextrins_h as _;
//! use rust_isa::loongarch::simd::lasx::xvextrins_w as _;
//! use rust_isa::loongarch::simd::lasx::xvfadd as _;
//! use rust_isa::loongarch::simd::lasx::xvfadd_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfadd_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfclass_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfclass_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_caf_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_caf_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_ceq_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_ceq_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cle_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cle_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_clt_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_clt_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cne_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cne_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cor_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cor_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cueq_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cueq_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cule_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cule_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cult_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cult_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cun_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cun_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cune_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_cune_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_saf_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_saf_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_seq_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_seq_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sle_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sle_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_slt_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_slt_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sne_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sne_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sor_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sor_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sueq_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sueq_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sule_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sule_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sult_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sult_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sun_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sun_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sune_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcmp_sune_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcvt_h_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcvt_s_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfcvth_d_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcvth_s_h as _;
//! use rust_isa::loongarch::simd::lasx::xvfcvtl_d_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfcvtl_s_h as _;
//! use rust_isa::loongarch::simd::lasx::xvfdiv as _;
//! use rust_isa::loongarch::simd::lasx::xvfdiv_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfdiv_s as _;
//! use rust_isa::loongarch::simd::lasx::xvffint as _;
//! use rust_isa::loongarch::simd::lasx::xvffint_d_l as _;
//! use rust_isa::loongarch::simd::lasx::xvffint_d_lu as _;
//! use rust_isa::loongarch::simd::lasx::xvffint_s_l as _;
//! use rust_isa::loongarch::simd::lasx::xvffint_s_w as _;
//! use rust_isa::loongarch::simd::lasx::xvffint_s_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvffinth_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvffintl_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvflogb_d as _;
//! use rust_isa::loongarch::simd::lasx::xvflogb_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfmadd_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfmadd_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfmax as _;
//! use rust_isa::loongarch::simd::lasx::xvfmax_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfmax_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfmaxa_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfmaxa_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfmin as _;
//! use rust_isa::loongarch::simd::lasx::xvfmin_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfmin_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfmina_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfmina_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfmsub_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfmsub_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfmul as _;
//! use rust_isa::loongarch::simd::lasx::xvfmul_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfmul_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfnmadd_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfnmadd_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfnmsub_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfnmsub_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrecip_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrecip_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrecipe_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrecipe_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrint_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrint_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrintrm_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrintrm_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrintrne_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrintrne_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrintrp_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrintrp_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrintrz_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrintrz_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrsqrt_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrsqrt_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrsqrte_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfrsqrte_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfrstp_b as _;
//! use rust_isa::loongarch::simd::lasx::xvfrstp_h as _;
//! use rust_isa::loongarch::simd::lasx::xvfrstpi_b as _;
//! use rust_isa::loongarch::simd::lasx::xvfrstpi_h as _;
//! use rust_isa::loongarch::simd::lasx::xvfsqrt_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfsqrt_s as _;
//! use rust_isa::loongarch::simd::lasx::xvfsub as _;
//! use rust_isa::loongarch::simd::lasx::xvfsub_d as _;
//! use rust_isa::loongarch::simd::lasx::xvfsub_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftint as _;
//! use rust_isa::loongarch::simd::lasx::xvftint_l_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftint_lu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftint_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftint_w_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftint_wu_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftinth_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintl_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrm_l_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrm_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrm_w_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrmh_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrml_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrne_l_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrne_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrne_w_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrneh_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrnel_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrp_l_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrp_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrp_w_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrph_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrpl_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrz_l_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrz_lu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrz_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrz_w_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrz_wu_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrzh_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvftintrzl_l_s as _;
//! use rust_isa::loongarch::simd::lasx::xvhaddw_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvhaddw_du_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvhaddw_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvhaddw_hu_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvhaddw_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvhaddw_qu_du as _;
//! use rust_isa::loongarch::simd::lasx::xvhaddw_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvhaddw_wu_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvhsubw_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvhsubw_du_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvhsubw_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvhsubw_hu_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvhsubw_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvhsubw_qu_du as _;
//! use rust_isa::loongarch::simd::lasx::xvhsubw_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvhsubw_wu_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvilvh_b as _;
//! use rust_isa::loongarch::simd::lasx::xvilvh_d as _;
//! use rust_isa::loongarch::simd::lasx::xvilvh_h as _;
//! use rust_isa::loongarch::simd::lasx::xvilvh_w as _;
//! use rust_isa::loongarch::simd::lasx::xvilvl_b as _;
//! use rust_isa::loongarch::simd::lasx::xvilvl_d as _;
//! use rust_isa::loongarch::simd::lasx::xvilvl_h as _;
//! use rust_isa::loongarch::simd::lasx::xvilvl_w as _;
//! use rust_isa::loongarch::simd::lasx::xvinsgr2vr_d as _;
//! use rust_isa::loongarch::simd::lasx::xvinsgr2vr_w as _;
//! use rust_isa::loongarch::simd::lasx::xvinsve0_d as _;
//! use rust_isa::loongarch::simd::lasx::xvinsve0_w as _;
//! use rust_isa::loongarch::simd::lasx::xvld as _;
//! use rust_isa::loongarch::simd::lasx::xvldi as _;
//! use rust_isa::loongarch::simd::lasx::xvldrepl_b as _;
//! use rust_isa::loongarch::simd::lasx::xvldrepl_d as _;
//! use rust_isa::loongarch::simd::lasx::xvldrepl_h as _;
//! use rust_isa::loongarch::simd::lasx::xvldrepl_w as _;
//! use rust_isa::loongarch::simd::lasx::xvldx as _;
//! use rust_isa::loongarch::simd::lasx::xvmadd_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmadd_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmadd_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmadd_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_d_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_d_wu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_h_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_h_bu_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_q_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_q_du_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_w_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwev_w_hu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_d_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_d_wu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_h_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_h_bu_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_q_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_q_du_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_w_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaddwod_w_hu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmax as _;
//! use rust_isa::loongarch::simd::lasx::xvmax_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmax_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmax_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmax_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmax_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmax_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmax_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmax_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaxi_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmaxi_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaxi_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmaxi_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmaxi_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmaxi_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmaxi_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmaxi_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmin as _;
//! use rust_isa::loongarch::simd::lasx::xvmin_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmin_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmin_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmin_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmin_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmin_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmin_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmin_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmini_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmini_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmini_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmini_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmini_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmini_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmini_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmini_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmod_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmod_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmod_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmod_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmod_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmod_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmod_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmod_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmskgez_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmskltz_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmskltz_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmskltz_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmskltz_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmsknz_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmsub_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmsub_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmsub_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmsub_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmuh_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmuh_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmuh_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmuh_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmuh_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmuh_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmuh_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmuh_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmul as _;
//! use rust_isa::loongarch::simd::lasx::xvmul_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmul_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmul_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmul_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_d_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_d_wu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_h_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_h_bu_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_q_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_q_du_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_w_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwev_w_hu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_d_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_d_wu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_h_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_h_bu_b as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_q_du as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_q_du_d as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_w_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvmulwod_w_hu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvneg as _;
//! use rust_isa::loongarch::simd::lasx::xvneg_b as _;
//! use rust_isa::loongarch::simd::lasx::xvneg_d as _;
//! use rust_isa::loongarch::simd::lasx::xvneg_h as _;
//! use rust_isa::loongarch::simd::lasx::xvneg_w as _;
//! use rust_isa::loongarch::simd::lasx::xvnor as _;
//! use rust_isa::loongarch::simd::lasx::xvnor_v as _;
//! use rust_isa::loongarch::simd::lasx::xvnori_b as _;
//! use rust_isa::loongarch::simd::lasx::xvor as _;
//! use rust_isa::loongarch::simd::lasx::xvor_v as _;
//! use rust_isa::loongarch::simd::lasx::xvori_b as _;
//! use rust_isa::loongarch::simd::lasx::xvorn_v as _;
//! use rust_isa::loongarch::simd::lasx::xvpackev as _;
//! use rust_isa::loongarch::simd::lasx::xvpackev_b as _;
//! use rust_isa::loongarch::simd::lasx::xvpackev_d as _;
//! use rust_isa::loongarch::simd::lasx::xvpackev_h as _;
//! use rust_isa::loongarch::simd::lasx::xvpackev_w as _;
//! use rust_isa::loongarch::simd::lasx::xvpackod as _;
//! use rust_isa::loongarch::simd::lasx::xvpackod_b as _;
//! use rust_isa::loongarch::simd::lasx::xvpackod_d as _;
//! use rust_isa::loongarch::simd::lasx::xvpackod_h as _;
//! use rust_isa::loongarch::simd::lasx::xvpackod_w as _;
//! use rust_isa::loongarch::simd::lasx::xvpcnt_b as _;
//! use rust_isa::loongarch::simd::lasx::xvpcnt_d as _;
//! use rust_isa::loongarch::simd::lasx::xvpcnt_h as _;
//! use rust_isa::loongarch::simd::lasx::xvpcnt_w as _;
//! use rust_isa::loongarch::simd::lasx::xvperm_w as _;
//! use rust_isa::loongarch::simd::lasx::xvpermi_d as _;
//! use rust_isa::loongarch::simd::lasx::xvpermi_q as _;
//! use rust_isa::loongarch::simd::lasx::xvpermi_w as _;
//! use rust_isa::loongarch::simd::lasx::xvpickev_b as _;
//! use rust_isa::loongarch::simd::lasx::xvpickev_d as _;
//! use rust_isa::loongarch::simd::lasx::xvpickev_h as _;
//! use rust_isa::loongarch::simd::lasx::xvpickev_w as _;
//! use rust_isa::loongarch::simd::lasx::xvpickod_b as _;
//! use rust_isa::loongarch::simd::lasx::xvpickod_d as _;
//! use rust_isa::loongarch::simd::lasx::xvpickod_h as _;
//! use rust_isa::loongarch::simd::lasx::xvpickod_w as _;
//! use rust_isa::loongarch::simd::lasx::xvpickve2gr_d as _;
//! use rust_isa::loongarch::simd::lasx::xvpickve2gr_du as _;
//! use rust_isa::loongarch::simd::lasx::xvpickve2gr_w as _;
//! use rust_isa::loongarch::simd::lasx::xvpickve2gr_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvpickve_d as _;
//! use rust_isa::loongarch::simd::lasx::xvpickve_w as _;
//! use rust_isa::loongarch::simd::lasx::xvrepl128vei_b as _;
//! use rust_isa::loongarch::simd::lasx::xvrepl128vei_d as _;
//! use rust_isa::loongarch::simd::lasx::xvrepl128vei_h as _;
//! use rust_isa::loongarch::simd::lasx::xvrepl128vei_w as _;
//! use rust_isa::loongarch::simd::lasx::xvreplgr2vr_b as _;
//! use rust_isa::loongarch::simd::lasx::xvreplgr2vr_d as _;
//! use rust_isa::loongarch::simd::lasx::xvreplgr2vr_h as _;
//! use rust_isa::loongarch::simd::lasx::xvreplgr2vr_w as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve0_b as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve0_d as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve0_h as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve0_q as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve0_w as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve_b as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve_d as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve_h as _;
//! use rust_isa::loongarch::simd::lasx::xvreplve_w as _;
//! use rust_isa::loongarch::simd::lasx::xvrotr_b as _;
//! use rust_isa::loongarch::simd::lasx::xvrotr_d as _;
//! use rust_isa::loongarch::simd::lasx::xvrotr_h as _;
//! use rust_isa::loongarch::simd::lasx::xvrotr_w as _;
//! use rust_isa::loongarch::simd::lasx::xvrotri_b as _;
//! use rust_isa::loongarch::simd::lasx::xvrotri_d as _;
//! use rust_isa::loongarch::simd::lasx::xvrotri_h as _;
//! use rust_isa::loongarch::simd::lasx::xvrotri_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsadd_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsadd_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvsadd_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsadd_du as _;
//! use rust_isa::loongarch::simd::lasx::xvsadd_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsadd_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvsadd_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsadd_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvsat as _;
//! use rust_isa::loongarch::simd::lasx::xvsat_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsat_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvsat_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsat_du as _;
//! use rust_isa::loongarch::simd::lasx::xvsat_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsat_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvsat_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsat_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvseq as _;
//! use rust_isa::loongarch::simd::lasx::xvseq_b as _;
//! use rust_isa::loongarch::simd::lasx::xvseq_d as _;
//! use rust_isa::loongarch::simd::lasx::xvseq_h as _;
//! use rust_isa::loongarch::simd::lasx::xvseq_w as _;
//! use rust_isa::loongarch::simd::lasx::xvseqi as _;
//! use rust_isa::loongarch::simd::lasx::xvseqi_b as _;
//! use rust_isa::loongarch::simd::lasx::xvseqi_d as _;
//! use rust_isa::loongarch::simd::lasx::xvseqi_h as _;
//! use rust_isa::loongarch::simd::lasx::xvseqi_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsetallnez_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsetallnez_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsetallnez_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsetallnez_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsetanyeqz_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsetanyeqz_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsetanyeqz_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsetanyeqz_w as _;
//! use rust_isa::loongarch::simd::lasx::xvseteqz as _;
//! use rust_isa::loongarch::simd::lasx::xvsetnez as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf4i as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf4i_b as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf4i_d as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf4i_h as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf4i_w as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf_b as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf_d as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf_h as _;
//! use rust_isa::loongarch::simd::lasx::xvshuf_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsigncov_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsigncov_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsigncov_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsigncov_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsle as _;
//! use rust_isa::loongarch::simd::lasx::xvsle_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsle_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvsle_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsle_du as _;
//! use rust_isa::loongarch::simd::lasx::xvsle_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsle_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvsle_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsle_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvslei as _;
//! use rust_isa::loongarch::simd::lasx::xvslei_b as _;
//! use rust_isa::loongarch::simd::lasx::xvslei_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvslei_d as _;
//! use rust_isa::loongarch::simd::lasx::xvslei_du as _;
//! use rust_isa::loongarch::simd::lasx::xvslei_h as _;
//! use rust_isa::loongarch::simd::lasx::xvslei_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvslei_w as _;
//! use rust_isa::loongarch::simd::lasx::xvslei_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvsll as _;
//! use rust_isa::loongarch::simd::lasx::xvsll_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsll_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsll_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsll_w as _;
//! use rust_isa::loongarch::simd::lasx::xvslli as _;
//! use rust_isa::loongarch::simd::lasx::xvslli_b as _;
//! use rust_isa::loongarch::simd::lasx::xvslli_d as _;
//! use rust_isa::loongarch::simd::lasx::xvslli_h as _;
//! use rust_isa::loongarch::simd::lasx::xvslli_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsllwil_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsllwil_du_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvsllwil_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsllwil_hu_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvsllwil_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsllwil_wu_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvslt as _;
//! use rust_isa::loongarch::simd::lasx::xvslt_b as _;
//! use rust_isa::loongarch::simd::lasx::xvslt_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvslt_d as _;
//! use rust_isa::loongarch::simd::lasx::xvslt_du as _;
//! use rust_isa::loongarch::simd::lasx::xvslt_h as _;
//! use rust_isa::loongarch::simd::lasx::xvslt_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvslt_w as _;
//! use rust_isa::loongarch::simd::lasx::xvslt_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvslti as _;
//! use rust_isa::loongarch::simd::lasx::xvslti_b as _;
//! use rust_isa::loongarch::simd::lasx::xvslti_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvslti_d as _;
//! use rust_isa::loongarch::simd::lasx::xvslti_du as _;
//! use rust_isa::loongarch::simd::lasx::xvslti_h as _;
//! use rust_isa::loongarch::simd::lasx::xvslti_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvslti_w as _;
//! use rust_isa::loongarch::simd::lasx::xvslti_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvsra as _;
//! use rust_isa::loongarch::simd::lasx::xvsra_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsra_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsra_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsra_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrai as _;
//! use rust_isa::loongarch::simd::lasx::xvsrai_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsrai_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrai_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrai_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsran_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsran_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsran_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrani_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrani_d_q as _;
//! use rust_isa::loongarch::simd::lasx::xvsrani_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrani_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrar_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsrar_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrar_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrar_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrari_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsrari_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrari_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrari_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrarn_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrarn_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrarn_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrarni_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrarni_d_q as _;
//! use rust_isa::loongarch::simd::lasx::xvsrarni_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrarni_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrl as _;
//! use rust_isa::loongarch::simd::lasx::xvsrl_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsrl_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrl_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrl_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrli as _;
//! use rust_isa::loongarch::simd::lasx::xvsrli_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsrli_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrli_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrli_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrln_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrln_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrln_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlni_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlni_d_q as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlni_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlni_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlr_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlr_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlr_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlr_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlri_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlri_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlri_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlri_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlrn_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlrn_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlrn_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlrni_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlrni_d_q as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlrni_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsrlrni_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssran_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssran_bu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssran_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssran_hu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssran_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssran_wu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrani_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrani_bu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrani_d_q as _;
//! use rust_isa::loongarch::simd::lasx::xvssrani_du_q as _;
//! use rust_isa::loongarch::simd::lasx::xvssrani_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrani_hu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrani_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrani_wu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarn_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarn_bu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarn_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarn_hu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarn_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarn_wu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarni_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarni_bu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarni_d_q as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarni_du_q as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarni_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarni_hu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarni_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrarni_wu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrln_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrln_bu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrln_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrln_hu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrln_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrln_wu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlni_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlni_bu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlni_d_q as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlni_du_q as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlni_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlni_hu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlni_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlni_wu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrn_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrn_bu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrn_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrn_hu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrn_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrn_wu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrni_b_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrni_bu_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrni_d_q as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrni_du_q as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrni_h_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrni_hu_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrni_w_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssrlrni_wu_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssub_b as _;
//! use rust_isa::loongarch::simd::lasx::xvssub_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvssub_d as _;
//! use rust_isa::loongarch::simd::lasx::xvssub_du as _;
//! use rust_isa::loongarch::simd::lasx::xvssub_h as _;
//! use rust_isa::loongarch::simd::lasx::xvssub_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvssub_w as _;
//! use rust_isa::loongarch::simd::lasx::xvssub_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvst as _;
//! use rust_isa::loongarch::simd::lasx::xvstelm_b as _;
//! use rust_isa::loongarch::simd::lasx::xvstelm_d as _;
//! use rust_isa::loongarch::simd::lasx::xvstelm_h as _;
//! use rust_isa::loongarch::simd::lasx::xvstelm_w as _;
//! use rust_isa::loongarch::simd::lasx::xvstx as _;
//! use rust_isa::loongarch::simd::lasx::xvsub as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_q as _;
//! use rust_isa::loongarch::simd::lasx::xvsub_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsubi_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvsubi_du as _;
//! use rust_isa::loongarch::simd::lasx::xvsubi_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvsubi_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwev_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwev_d_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwev_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwev_h_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwev_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwev_q_du as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwev_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwev_w_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwod_d_w as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwod_d_wu as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwod_h_b as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwod_h_bu as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwod_q_d as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwod_q_du as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwod_w_h as _;
//! use rust_isa::loongarch::simd::lasx::xvsubwod_w_hu as _;
//! use rust_isa::loongarch::simd::lasx::xvxor as _;
//! use rust_isa::loongarch::simd::lasx::xvxor_v as _;
//! use rust_isa::loongarch::simd::lasx::xvxori_b as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

mod float_extra;
mod frstpi;
mod immediate;
mod lane;
mod misc;
mod narrow;
mod v256;

pub use float_extra::*;
pub use frstpi::*;
pub use immediate::*;
pub use lane::*;
pub use misc::*;
pub use narrow::*;
pub use v256::*;

unsafe extern "C" {
    fn rust_isa_lasx_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_mul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_sub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fmax_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fmin_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fcvt_s_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_ftint_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_ffint_s_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable LASX.
        #[target_feature(enable = "lasx")]
        #[inline]
        pub unsafe fn $name(a: V256, b: V256) -> V256 {
            let mut out = V256::from_bytes([0; 32]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `xvadd.w` on 32-bit lanes.
    add,
    rust_isa_lasx_add
);
binary!(
    /// `xvmul.w` on 32-bit lanes, keeping the low 32 bits.
    mul,
    rust_isa_lasx_mul
);
binary!(
    /// `xvsub.w` on 32-bit lanes.
    sub,
    rust_isa_lasx_sub
);
binary!(
    /// `xvadd.d` on 64-bit lanes.
    add_i64,
    rust_isa_lasx_add_i64
);
binary!(
    /// `xvsub.d` on 64-bit lanes.
    sub_i64,
    rust_isa_lasx_sub_i64
);
binary!(
    /// `xvmul.w` on 32-bit lanes.
    mul_i32,
    rust_isa_lasx_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_lasx_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_lasx_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_lasx_xor
);
binary!(
    /// `xvfadd.s` on 32-bit float lanes.
    fadd,
    rust_isa_lasx_fadd
);
binary!(
    /// `xvfsub.s` on 32-bit float lanes.
    fsub,
    rust_isa_lasx_fsub
);
binary!(
    /// `xvfmul.s` on 32-bit float lanes.
    fmul,
    rust_isa_lasx_fmul
);
binary!(
    /// `xvfdiv.s` on 32-bit float lanes.
    fdiv,
    rust_isa_lasx_fdiv
);

/// Broadcast a 32-bit value to all eight lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn splat_i32(value: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_splat_i32(value, out.as_mut_ptr());
    }
    out
}

/// `xvadd.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd(a: V256, b: V256) -> V256 {
    unsafe { add(a, b) }
}
/// `xvsub.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub(a: V256, b: V256) -> V256 {
    unsafe { sub(a, b) }
}
/// `xvmul.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul(a: V256, b: V256) -> V256 {
    unsafe { mul(a, b) }
}
/// `xvand.v`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvand(a: V256, b: V256) -> V256 {
    unsafe { and(a, b) }
}
/// `xvor.v`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvor(a: V256, b: V256) -> V256 {
    unsafe { or(a, b) }
}
/// `xvxor.v`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvxor(a: V256, b: V256) -> V256 {
    unsafe { xor(a, b) }
}

/// `xvabs.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabs(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let value = i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.wrapping_abs().to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvneg.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let value = i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.wrapping_neg().to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvadd.d`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_d(a: V256, b: V256) -> V256 {
    unsafe { add_i64(a, b) }
}
/// `xvsub.d`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_d(a: V256, b: V256) -> V256 {
    unsafe { sub_i64(a, b) }
}

/// `xvadd.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_b(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| a[i].wrapping_add(b[i])))
}
/// `xvadd.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_h(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for i in 0..16 {
        let value = u16::from_le_bytes([a[i * 2], a[i * 2 + 1]])
            .wrapping_add(u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]));
        out[i * 2..i * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}
/// `xvsub.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_b(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| a[i].wrapping_sub(b[i])))
}
/// `xvsub.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_h(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for i in 0..16 {
        let value = u16::from_le_bytes([a[i * 2], a[i * 2 + 1]])
            .wrapping_sub(u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]));
        out[i * 2..i * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}
/// `xvmul.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul_h(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for i in 0..16 {
        let value = u16::from_le_bytes([a[i * 2], a[i * 2 + 1]])
            .wrapping_mul(u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]));
        out[i * 2..i * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvabs.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabs_b(a: V256) -> V256 {
    V256::from_bytes(core::array::from_fn(|i| {
        (a.to_bytes()[i] as i8).wrapping_abs() as u8
    }))
}

/// `xvabs.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabs_h(a: V256) -> V256 {
    let b = a.to_bytes();
    let mut o = [0u8; 32];
    for i in 0..16 {
        let v = i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]);
        o[i * 2..i * 2 + 2].copy_from_slice(&v.wrapping_abs().to_le_bytes());
    }
    V256::from_bytes(o)
}

/// `xvabs.d`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabs_d(a: V256) -> V256 {
    let b = a.to_bytes();
    let mut o = [0u8; 32];
    for i in 0..4 {
        let v = i64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        o[i * 8..i * 8 + 8].copy_from_slice(&v.wrapping_abs().to_le_bytes());
    }
    V256::from_bytes(o)
}

/// `xvneg.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg_b(a: V256) -> V256 {
    V256::from_bytes(core::array::from_fn(|i| {
        (a.to_bytes()[i] as i8).wrapping_neg() as u8
    }))
}

/// `xvneg.h`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg_h(a: V256) -> V256 {
    let b = a.to_bytes();
    let mut o = [0u8; 32];
    for i in 0..16 {
        let v = i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]);
        o[i * 2..i * 2 + 2].copy_from_slice(&v.wrapping_neg().to_le_bytes());
    }
    V256::from_bytes(o)
}

/// `xvneg.d`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg_d(a: V256) -> V256 {
    let b = a.to_bytes();
    let mut o = [0u8; 32];
    for i in 0..4 {
        let v = i64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        o[i * 8..i * 8 + 8].copy_from_slice(&v.wrapping_neg().to_le_bytes());
    }
    V256::from_bytes(o)
}

/// `xvadd.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_w(a: V256, b: V256) -> V256 {
    unsafe { add(a, b) }
}

/// `xvsub.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_w(a: V256, b: V256) -> V256 {
    unsafe { sub(a, b) }
}

/// `xvmul.w`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul_w(a: V256, b: V256) -> V256 {
    unsafe { mul(a, b) }
}

/// `xvmax.w` on signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&x.max(y).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmin.w` on signed 32-bit lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&x.min(y).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvseq.w` on 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseq(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x == y { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsle.w` on signed 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x <= y { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslt.w` on signed 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x < y { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvseqi.w`. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseqi(a: V256, imm: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x == imm { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslei.w` on signed 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei(a: V256, imm: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x <= imm { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslti.w` on signed 32-bit lanes. A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti(a: V256, imm: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = if x < imm { -1i32 } else { 0 };
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvnor.v`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvnor(a: V256, b: V256) -> V256 {
    V256::from_bytes(core::array::from_fn(|i| {
        !(a.to_bytes()[i] | b.to_bytes()[i])
    }))
}

/// `xvbitsel.v`.
///
/// Where a mask bit is one the result takes the bit from `a`, otherwise from
/// `b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitsel(a: V256, b: V256, mask: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let m = mask.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| (a[i] & m[i]) | (b[i] & !m[i])))
}

/// `xvshuf.b`.
///
/// Each control byte selects a byte from the concatenation of `a` and `b`. The
/// low 6 bits are the index; indices 0 to 31 select `a` and indices 32 to 63
/// select `b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf(a: V256, b: V256, control: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let c = control.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        let index = (c[i] & 63) as usize;
        if index < 32 { a[index] } else { b[index - 32] }
    }))
}

/// `xvshuf4i.b`.
///
/// `IMM` holds four 2-bit selectors, one for each byte of a 4-byte group.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf4i<const IMM: u8>(a: V256) -> V256 {
    let a = a.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        a[(i / 4) * 4 + (((IMM >> ((i % 4) * 2)) & 3) as usize)]
    }))
}

/// `xvsll.w`.
///
/// Shift each 32-bit integer lane left by `shift & 31`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsll(a: V256, shift: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x << ((shift as u32) & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrl.w`.
///
/// Shift each 32-bit integer lane right by `shift & 31`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrl(a: V256, shift: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = u32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x >> ((shift as u32) & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsra.w`.
///
/// Shift each signed 32-bit integer lane right by `shift & 31`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsra(a: V256, shift: i32) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x >> ((shift as u32) & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslli.w`. `SHIFT` is masked to 5 bits.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslli<const SHIFT: u32>(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x << (SHIFT & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrli.w`. `SHIFT` is masked to 5 bits.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrli<const SHIFT: u32>(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = u32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x >> (SHIFT & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrai.w`. `SHIFT` is masked to 5 bits.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrai<const SHIFT: u32>(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&(x >> (SHIFT & 31)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvpackev.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackev(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        if i % 2 == 0 { a[i] } else { b[i - 1] }
    }))
}

/// `xvpackod.b`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackod(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        if i % 2 == 0 { a[i + 1] } else { b[i] }
    }))
}

/// `vext2xv.h.b`.
///
/// Sign-extend the low 16 bytes of `a` to 16-bit lanes. The high half of `a`
/// is ignored.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_h_b(a: V256) -> V256 {
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..16 {
        let value = (a[lane] as i8 as i16).to_le_bytes();
        out[lane * 2..lane * 2 + 2].copy_from_slice(&value);
    }
    V256::from_bytes(out)
}

/// `xvsat.w`.
///
/// Saturate each signed 32-bit lane to the signed `SAT`-bit range. `SAT` must
/// be in `0..=31`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat<const SAT: u32>(a: V256) -> V256 {
    const { assert!(SAT <= 31) };
    let max = (1i64 << SAT) - 1;
    let min = -(1i64 << SAT);
    let a = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let x = i32::from_le_bytes(a[lane * 4..lane * 4 + 4].try_into().unwrap());
        let value = (x as i64).clamp(min, max) as i32;
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvavgr.bu`.
///
/// Rounding average of the unsigned 8-bit lanes: `(a + b + 1) >> 1`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr(a: V256, b: V256) -> V256 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V256::from_bytes(core::array::from_fn(|i| {
        ((a[i] as u16 + b[i] as u16 + 1) >> 1) as u8
    }))
}

/// `xvfadd.s`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfadd(a: V256, b: V256) -> V256 {
    unsafe { fadd(a, b) }
}

/// `xvfsub.s`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfsub(a: V256, b: V256) -> V256 {
    unsafe { fsub(a, b) }
}

/// `xvfmul.s`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmul(a: V256, b: V256) -> V256 {
    unsafe { fmul(a, b) }
}

/// `xvfdiv.s`.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfdiv(a: V256, b: V256) -> V256 {
    unsafe { fdiv(a, b) }
}

/// `xvfmax.s`.
///
/// A NaN operand selects the other operand. The result for two NaNs is the
/// instruction's NaN.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmax(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_fmax_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmin.s`.
///
/// A NaN operand selects the other operand. The result for two NaNs is the
/// instruction's NaN.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmin(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_fmin_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfcmp.<COND>.s`.
///
/// `COND` is the 5-bit condition field. The low bit selects the signaling
/// form; only the result mask is exposed, so the quiet and signaling forms
/// return the same lanes. Valid values and mnemonics:
///
/// | `COND` | quiet  | signaling |
/// | ------ | ------ | --------- |
/// | 0      | `caf`  | `saf`     |
/// | 2      | `clt`  | `slt`     |
/// | 4      | `ceq`  | `seq`     |
/// | 6      | `cle`  | `sle`     |
/// | 8      | `cun`  | `sun`     |
/// | 10     | `cult` | `sult`    |
/// | 12     | `cueq` | `sueq`    |
/// | 14     | `cule` | `sule`    |
/// | 16     | `cne`  | `sne`     |
/// | 20     | `cor`  | `sor`     |
/// | 24     | `cune` | `sune`    |
///
/// A true lane is all bits one. Unordered comparisons are true when either
/// lane is NaN.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp<const COND: u32>(a: V256, b: V256) -> V256 {
    const { assert!(matches!(COND, 0..=17 | 20 | 21 | 24 | 25)) };
    let less = (COND >> 1) & 1 != 0;
    let equal = (COND >> 1) & 2 != 0;
    let unordered = (COND >> 1) & 4 != 0;
    let ordered_ne = (COND >> 1) & 8 != 0;
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..8 {
        let offset = lane * 4;
        let x = f32::from_le_bytes(a[offset..offset + 4].try_into().unwrap());
        let y = f32::from_le_bytes(b[offset..offset + 4].try_into().unwrap());
        let hit = (less && x < y)
            || (equal && x == y)
            || (unordered && (x.is_nan() || y.is_nan()))
            || (ordered_ne && (x < y || x > y));
        let value = if hit { -1i32 } else { 0 };
        out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvftint.w.s`.
///
/// Convert f32 lanes to signed 32-bit lanes using the current floating-point
/// rounding mode from FCSR.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftint(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_ftint_w_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffint.s.w`.
///
/// Convert signed 32-bit lanes to f32 lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffint(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_ffint_s_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvld`.
///
/// Load 32 bytes from `p + offset`.
///
/// # Safety
///
/// The caller must enable LASX. `offset` must be in `-2048..=2047` and the
/// range must be valid for reading 32 bytes. The load does not require
/// alignment.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvld(p: *const u8, offset: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), out.as_mut_ptr(), 32);
    }
    out
}

/// `xvst`.
///
/// Store 32 bytes to `p + offset`.
///
/// # Safety
///
/// The caller must enable LASX. `offset` must be in `-2048..=2047` and the
/// range must be valid for writing 32 bytes. The store does not require
/// alignment.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvst(p: *mut u8, offset: i32, value: V256) {
    unsafe {
        core::ptr::copy_nonoverlapping(value.as_ptr(), p.offset(offset as isize), 32);
    }
}

/// `xvldi`.
///
/// Build the immediate value of the instruction. `IMM` is the 13-bit encoding,
/// not the loaded scalar.
///
/// When bit 12 is clear, bits 9 to 0 are sign-extended and replicated as lanes
/// of the width selected by bits 11 and 10 (`0` is 8-bit, `1` is 16-bit, `2` is
/// 32-bit, `3` is 64-bit).
///
/// When bit 12 is set, bits 11 to 8 select one of the 13 constant patterns and
/// bits 7 to 0 supply the pattern data. The 64-bit pattern is replicated four
/// times.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvldi<const IMM: i32>() -> V256 {
    const { V256::from_bytes(vldi_bytes(IMM as u32)) }
}

const fn vldi_bytes(imm: u32) -> [u8; 32] {
    let mut out = [0u8; 32];
    if imm & 0x1000 == 0 {
        let lane = 1usize << ((imm >> 10) & 3);
        let value = (((imm & 0x3ff) << 22) as i32 >> 22) as i64 as u64;
        let bytes = value.to_le_bytes();
        let mut i = 0;
        while i < 32 {
            out[i] = bytes[i % lane];
            i += 1;
        }
    } else {
        let data = vldi_pattern(imm).to_le_bytes();
        let mut i = 0;
        while i < 32 {
            out[i] = data[i % 8];
            i += 1;
        }
    }
    out
}

const fn vldi_pattern(imm: u32) -> u64 {
    let mode = (imm >> 8) & 0xf;
    let t = (imm & 0xff) as u64;
    match mode {
        0 => (t << 32) | t,
        1 => (t << 40) | (t << 8),
        2 => (t << 48) | (t << 16),
        3 => (t << 56) | (t << 24),
        4 => (t << 48) | (t << 32) | (t << 16) | t,
        5 => (t << 56) | (t << 40) | (t << 24) | (t << 8),
        6 => (t << 40) | (0xff << 32) | (t << 8) | 0xff,
        7 => (t << 48) | (0xffff << 32) | (t << 16) | 0xffff,
        8 => {
            let mut data = 0u64;
            let mut i = 0;
            while i < 8 {
                data |= t << (i * 8);
                i += 1;
            }
            data
        }
        9 => {
            let mut data = 0u64;
            let mut i = 0;
            while i < 8 {
                if (t >> i) & 1 != 0 {
                    data |= 0xffu64 << (i * 8);
                }
                i += 1;
            }
            data
        }
        10 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 6) | (((1 - b6) as u64) << 5) | if b6 != 0 { 0x1f } else { 0 };
            (high << 57) | (low << 51) | (high << 25) | (low << 19)
        }
        11 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 6) | (((1 - b6) as u64) << 5) | if b6 != 0 { 0x1f } else { 0 };
            (high << 25) | (low << 19)
        }
        12 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 9) | (((1 - b6) as u64) << 8) | if b6 != 0 { 0xff } else { 0 };
            (high << 54) | (low << 48)
        }
        _ => panic!("invalid xvldi immediate"),
    }
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_caf_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.caf.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_caf_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_caf_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_saf_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.saf.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_saf_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_saf_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_clt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.clt.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_clt_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_clt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_slt_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.slt.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_slt_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_slt_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_ceq_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.ceq.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_ceq_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_ceq_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_seq_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.seq.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_seq_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_seq_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cle_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cle.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cle_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cle_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sle_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sle.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sle_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sle_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cun_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cun.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cun_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cun_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sun_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sun.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sun_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sun_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cult_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cult.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cult_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cult_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sult_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sult.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sult_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sult_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cueq_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cueq.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cueq_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cueq_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sueq_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sueq.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sueq_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sueq_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cule_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cule.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cule_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cule_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sule_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sule.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sule_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sule_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cne_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cne.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cne_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cne_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sne_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sne.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sne_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sne_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cor_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cor.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cor_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cor_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sor_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sor.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sor_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sor_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cune_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cune.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cune_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cune_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sune_s(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sune.s`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sune_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sune_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_caf_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.caf.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_caf_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_caf_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_saf_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.saf.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_saf_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_saf_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_clt_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.clt.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_clt_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_clt_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_slt_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.slt.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_slt_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_slt_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_ceq_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.ceq.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_ceq_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_ceq_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_seq_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.seq.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_seq_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_seq_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cle_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cle.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cle_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cle_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sle_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sle.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sle_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sle_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cun_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cun.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cun_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cun_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sun_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sun.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sun_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sun_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cult_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cult.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cult_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cult_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sult_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sult.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sult_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sult_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cueq_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cueq.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cueq_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cueq_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sueq_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sueq.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sueq_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sueq_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cule_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cule.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cule_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cule_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sule_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sule.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sule_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sule_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cne_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cne.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cne_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cne_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sne_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sne.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sne_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sne_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cor_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cor.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cor_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cor_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sor_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sor.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sor_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sor_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_cune_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.cune.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_cune_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_cune_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfcmp_sune_d(a: *const u8, b: *const u8, out: *mut u8);
}
/// `xvfcmp.sune.d`
///
/// A true lane is all bits one.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcmp_sune_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcmp_sune_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfmadd_s(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `xvfmadd.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmadd_s(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmadd_s(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfmadd_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `xvfmadd.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmadd_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmadd_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfmsub_s(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `xvfmsub.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmsub_s(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmsub_s(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfmsub_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `xvfmsub.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmsub_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmsub_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfnmadd_s(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `xvfnmadd.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfnmadd_s(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfnmadd_s(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfnmadd_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `xvfnmadd.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfnmadd_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfnmadd_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfnmsub_s(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `xvfnmsub.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfnmsub_s(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfnmsub_s(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfnmsub_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `xvfnmsub.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfnmsub_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfnmsub_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvbitsel_v(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvshuf_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvldx(p: *const u8, offset: i64, out: *mut u8);
    fn rust_isa_lasx_xvstx(value: *const u8, p: *mut u8, offset: i64);
    fn rust_isa_lasx_xvseq_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvseq_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvseq_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvseq_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsle_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsle_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsle_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsle_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsle_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsle_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsle_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsle_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvslt_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvslt_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvslt_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvslt_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvslt_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvslt_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvslt_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvslt_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwev_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwev_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwev_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwev_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwod_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwod_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwod_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwod_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwev_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwev_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwev_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwev_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwod_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwod_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwod_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsubwod_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_h_bu_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_w_hu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_d_wu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwev_q_du_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_h_bu_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_w_hu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_d_wu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvaddwod_q_du_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsadd_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsadd_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsadd_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsadd_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssub_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssub_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssub_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssub_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsadd_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsadd_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsadd_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsadd_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssub_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssub_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssub_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssub_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhaddw_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhaddw_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhaddw_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhaddw_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhsubw_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhsubw_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhsubw_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhsubw_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhaddw_hu_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhaddw_wu_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhaddw_du_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhaddw_qu_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhsubw_hu_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhsubw_wu_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhsubw_du_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvhsubw_qu_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvadda_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvadda_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvadda_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvadda_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvabsd_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvabsd_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvabsd_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvabsd_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvabsd_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvabsd_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvabsd_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvabsd_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavg_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavg_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavg_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavg_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavg_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavg_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavg_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavg_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavgr_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavgr_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavgr_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavgr_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavgr_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavgr_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavgr_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvavgr_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmax_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmax_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmax_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmax_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmin_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmin_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmin_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmin_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmax_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmax_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmax_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmax_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmin_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmin_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmin_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmin_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmul_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmul_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmuh_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmuh_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmuh_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmuh_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmuh_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmuh_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmuh_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmuh_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_h_bu_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_w_hu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_d_wu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwev_q_du_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_h_bu_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_w_hu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_d_wu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmulwod_q_du_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmadd_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmadd_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmadd_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmadd_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmsub_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmsub_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmsub_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmsub_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_h_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_w_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_d_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_q_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_h_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_w_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_d_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_q_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_h_bu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_w_hu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_d_wu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_q_du(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_h_bu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_w_hu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_d_wu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_q_du(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_h_bu_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_w_hu_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_d_wu_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwev_q_du_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_h_bu_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_w_hu_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_d_wu_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmaddwod_q_du_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvdiv_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvdiv_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvdiv_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvdiv_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmod_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmod_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmod_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmod_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvdiv_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvdiv_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvdiv_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvdiv_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmod_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmod_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmod_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmod_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsll_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsll_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsll_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsll_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrl_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrl_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrl_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrl_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsra_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsra_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsra_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsra_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvrotr_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvrotr_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvrotr_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvrotr_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrlr_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrlr_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrlr_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrlr_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrar_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrar_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrar_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrar_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrln_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrln_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrln_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsran_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsran_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsran_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrlrn_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrlrn_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrlrn_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrarn_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrarn_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsrarn_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrln_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrln_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrln_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssran_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssran_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssran_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrlrn_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrlrn_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrlrn_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrarn_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrarn_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrarn_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrln_bu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrln_hu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrln_wu_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssran_bu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssran_hu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssran_wu_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrlrn_bu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrlrn_hu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrlrn_wu_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrarn_bu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrarn_hu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvssrarn_wu_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitclr_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitclr_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitclr_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitclr_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitset_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitset_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitset_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitset_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitrev_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitrev_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitrev_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvbitrev_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpackev_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpackev_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpackev_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpackev_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpackod_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpackod_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpackod_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpackod_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvilvl_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvilvl_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvilvl_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvilvl_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvilvh_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvilvh_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvilvh_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvilvh_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpickev_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpickev_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpickev_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpickev_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpickod_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpickod_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpickod_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpickod_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvreplve_b(a: *const u8, index: i32, out: *mut u8);
    fn rust_isa_lasx_xvreplve_h(a: *const u8, index: i32, out: *mut u8);
    fn rust_isa_lasx_xvreplve_w(a: *const u8, index: i32, out: *mut u8);
    fn rust_isa_lasx_xvreplve_d(a: *const u8, index: i32, out: *mut u8);
    fn rust_isa_lasx_xvand_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvor_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvxor_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvnor_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvandn_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvorn_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstp_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstp_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvadd_q(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsub_q(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsigncov_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsigncov_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsigncov_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvsigncov_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfcvt_h_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftint_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrm_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrp_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrz_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrne_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvshuf_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvshuf_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvshuf_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvperm_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvclo_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvclo_h(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvclo_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvclo_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvclz_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvclz_h(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvclz_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvclz_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpcnt_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpcnt_h(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpcnt_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvpcnt_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvneg_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmskltz_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmskltz_h(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmskltz_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmskltz_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmskgez_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvmsknz_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfclass_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfclass_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftint_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftint_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrm_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrm_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrp_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrp_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrz_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrz_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrne_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrne_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftint_wu_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftint_lu_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrz_wu_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrz_lu_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintl_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftinth_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrml_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrmh_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrpl_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrph_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrzl_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrzh_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrnel_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvftintrneh_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvexth_h_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvexth_w_h(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvexth_d_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvexth_q_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvexth_hu_bu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvexth_wu_hu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvexth_du_wu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvexth_qu_du(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvreplgr2vr_b(value: i32, out: *mut u8);
    fn rust_isa_lasx_xvreplgr2vr_h(value: i32, out: *mut u8);
    fn rust_isa_lasx_xvreplgr2vr_w(value: i32, out: *mut u8);
    fn rust_isa_lasx_xvreplgr2vr_d(value: i64, out: *mut u8);
    fn rust_isa_lasx_vext2xv_w_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_d_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_w_h(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_d_h(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_d_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_hu_bu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_wu_bu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_du_bu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_wu_hu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_du_hu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_vext2xv_du_wu(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvreplve0_b(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvreplve0_h(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvreplve0_w(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvreplve0_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvreplve0_q(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvextl_q_d(a: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvextl_qu_du(a: *const u8, out: *mut u8);
}

/// `xvbitsel.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitsel_v(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitsel_v(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvshuf.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf_b(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvshuf_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvldx`
///
/// Loads 32 bytes from `p + offset`.
///
/// # Safety
///
/// The caller must ensure the range is valid for reading.
#[inline(always)]
pub unsafe fn xvldx(p: *const u8, offset: i64) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvldx(p, offset, out.as_mut_ptr());
    }
    out
}

/// `xvstx`
///
/// Stores 32 bytes to `p + offset`.
///
/// # Safety
///
/// The caller must ensure the range is valid for writing.
#[inline(always)]
pub unsafe fn xvstx(value: V256, p: *mut u8, offset: i64) {
    unsafe {
        rust_isa_lasx_xvstx(value.as_ptr(), p, offset);
    }
}

/// `xvseq.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseq_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvseq_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvseq.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseq_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvseq_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvseq.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseq_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvseq_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvseq.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseq_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvseq_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsle.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsle_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsle.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsle_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsle.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsle_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsle.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsle_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsle.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsle_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsle.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsle_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsle.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsle_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsle.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsle_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsle_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvslt.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvslt_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvslt.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvslt_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvslt.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvslt_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvslt.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvslt_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvslt.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvslt_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvslt.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvslt_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvslt.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvslt_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvslt.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslt_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvslt_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_h_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_w_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_d_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_q_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwev.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwev_h_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwev_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwev.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwev_w_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwev_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwev.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwev_d_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwev_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwev.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwev_q_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwev_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_h_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_w_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_d_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_q_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwod.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwod_h_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwod_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwod.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwod_w_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwod_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwod.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwod_d_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwod_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwod.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwod_q_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwod_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.h.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_h_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.w.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_w_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.d.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_d_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.q.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_q_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwev.h.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwev_h_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwev_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwev.w.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwev_w_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwev_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwev.d.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwev_d_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwev_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwev.q.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwev_q_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwev_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.h.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_h_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.w.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_w_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.d.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_d_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.q.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_q_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwod.h.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwod_h_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwod_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwod.w.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwod_w_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwod_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwod.d.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwod_d_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwod_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsubwod.q.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubwod_q_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsubwod_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.h.bu.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_h_bu_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_h_bu_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.w.hu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_w_hu_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_w_hu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.d.wu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_d_wu_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_d_wu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwev.q.du.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwev_q_du_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwev_q_du_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.h.bu.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_h_bu_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_h_bu_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.w.hu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_w_hu_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_w_hu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.d.wu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_d_wu_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_d_wu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvaddwod.q.du.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddwod_q_du_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvaddwod_q_du_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsadd.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsadd_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsadd_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsadd.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsadd_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsadd_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsadd.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsadd_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsadd_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsadd.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsadd_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsadd_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssub.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssub_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssub_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssub.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssub_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssub_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssub.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssub_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssub_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssub.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssub_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssub_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsadd.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsadd_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsadd_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsadd.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsadd_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsadd_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsadd.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsadd_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsadd_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsadd.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsadd_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsadd_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssub.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssub_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssub_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssub.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssub_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssub_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssub.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssub_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssub_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssub.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssub_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssub_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhaddw.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhaddw_h_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhaddw_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhaddw.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhaddw_w_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhaddw_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhaddw.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhaddw_d_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhaddw_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhaddw.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhaddw_q_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhaddw_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhsubw.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhsubw_h_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhsubw_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhsubw.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhsubw_w_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhsubw_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhsubw.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhsubw_d_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhsubw_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhsubw.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhsubw_q_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhsubw_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhaddw.hu.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhaddw_hu_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhaddw_hu_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhaddw.wu.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhaddw_wu_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhaddw_wu_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhaddw.du.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhaddw_du_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhaddw_du_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhaddw.qu.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhaddw_qu_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhaddw_qu_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhsubw.hu.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhsubw_hu_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhsubw_hu_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhsubw.wu.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhsubw_wu_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhsubw_wu_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhsubw.du.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhsubw_du_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhsubw_du_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvhsubw.qu.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvhsubw_qu_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvhsubw_qu_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvadda.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadda_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvadda_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvadda.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadda_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvadda_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvadda.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadda_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvadda_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvadda.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadda_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvadda_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvabsd.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabsd_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvabsd_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvabsd.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabsd_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvabsd_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvabsd.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabsd_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvabsd_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvabsd.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabsd_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvabsd_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvabsd.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabsd_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvabsd_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvabsd.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabsd_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvabsd_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvabsd.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabsd_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvabsd_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvabsd.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvabsd_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvabsd_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavg.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavg_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavg_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavg.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavg_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavg_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavg.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavg_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavg_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavg.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavg_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavg_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavg.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavg_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavg_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavg.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavg_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavg_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavg.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavg_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavg_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavg.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavg_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavg_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavgr.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavgr_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavgr.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavgr_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavgr.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavgr_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavgr.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavgr_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavgr.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavgr_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavgr.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavgr_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavgr.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavgr_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvavgr.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvavgr_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvavgr_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmax.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmax_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmax.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmax_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmax.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmax_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmax.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmax_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmin.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmin_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmin.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmin_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmin.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmin_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmin.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmin_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmax.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmax_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmax.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmax_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmax.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmax_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmax.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmax_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmax_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmin.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmin_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmin.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmin_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmin.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmin_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmin.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmin_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmin_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmul.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmul_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmul.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmul_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmuh.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmuh_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmuh_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmuh.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmuh_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmuh_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmuh.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmuh_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmuh_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmuh.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmuh_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmuh_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmuh.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmuh_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmuh_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmuh.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmuh_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmuh_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmuh.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmuh_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmuh_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmuh.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmuh_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmuh_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_h_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_w_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_d_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_q_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_h_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_w_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_d_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_q_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.h.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_h_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.w.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_w_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.d.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_d_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.q.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_q_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.h.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_h_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.w.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_w_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.d.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_d_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.q.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_q_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.h.bu.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_h_bu_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_h_bu_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.w.hu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_w_hu_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_w_hu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.d.wu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_d_wu_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_d_wu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwev.q.du.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwev_q_du_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwev_q_du_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.h.bu.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_h_bu_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_h_bu_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.w.hu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_w_hu_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_w_hu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.d.wu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_d_wu_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_d_wu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmulwod.q.du.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmulwod_q_du_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmulwod_q_du_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmadd.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmadd_b(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmadd_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmadd.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmadd_h(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmadd_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmadd.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmadd_w(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmadd_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmadd.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmadd_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmadd_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmsub.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmsub_b(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmsub_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmsub.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmsub_h(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmsub_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmsub.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmsub_w(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmsub_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmsub.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmsub_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmsub_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_h_b(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_h_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_w_h(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_w_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_d_w(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_d_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_q_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_q_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_h_b(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_h_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_w_h(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_w_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_d_w(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_d_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_q_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_q_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.h.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_h_bu(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_h_bu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.w.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_w_hu(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_w_hu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.d.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_d_wu(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_d_wu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.q.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_q_du(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_q_du(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.h.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_h_bu(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_h_bu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.w.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_w_hu(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_w_hu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.d.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_d_wu(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_d_wu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.q.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_q_du(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_q_du(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.h.bu.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_h_bu_b(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_h_bu_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.w.hu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_w_hu_h(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_w_hu_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.d.wu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_d_wu_w(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_d_wu_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwev.q.du.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwev_q_du_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwev_q_du_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.h.bu.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_h_bu_b(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_h_bu_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.w.hu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_w_hu_h(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_w_hu_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.d.wu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_d_wu_w(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_d_wu_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmaddwod.q.du.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaddwod_q_du_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmaddwod_q_du_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvdiv.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvdiv_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvdiv_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvdiv.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvdiv_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvdiv_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvdiv.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvdiv_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvdiv_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvdiv.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvdiv_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvdiv_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmod.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmod_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmod_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmod.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmod_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmod_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmod.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmod_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmod_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmod.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmod_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmod_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvdiv.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvdiv_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvdiv_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvdiv.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvdiv_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvdiv_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvdiv.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvdiv_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvdiv_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvdiv.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvdiv_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvdiv_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmod.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmod_bu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmod_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmod.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmod_hu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmod_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmod.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmod_wu(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmod_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmod.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmod_du(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmod_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsll.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsll_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsll_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsll.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsll_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsll_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsll.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsll_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsll_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsll.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsll_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsll_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrl.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrl_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrl_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrl.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrl_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrl_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrl.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrl_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrl_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrl.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrl_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrl_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsra.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsra_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsra_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsra.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsra_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsra_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsra.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsra_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsra_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsra.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsra_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsra_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvrotr.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrotr_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvrotr_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvrotr.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrotr_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvrotr_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvrotr.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrotr_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvrotr_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvrotr.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrotr_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvrotr_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrlr.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlr_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrlr_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrlr.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlr_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrlr_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrlr.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlr_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrlr_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrlr.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlr_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrlr_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrar.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrar_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrar_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrar.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrar_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrar_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrar.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrar_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrar_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrar.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrar_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrar_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrln.b.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrln_b_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrln_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrln.h.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrln_h_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrln_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrln.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrln_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrln_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsran.b.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsran_b_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsran_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsran.h.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsran_h_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsran_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsran.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsran_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsran_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrlrn.b.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlrn_b_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrlrn_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrlrn.h.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlrn_h_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrlrn_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrlrn.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlrn_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrlrn_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrarn.b.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrarn_b_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrarn_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrarn.h.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrarn_h_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrarn_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsrarn.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrarn_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsrarn_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrln.b.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrln_b_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrln_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrln.h.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrln_h_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrln_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrln.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrln_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrln_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssran.b.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssran_b_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssran_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssran.h.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssran_h_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssran_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssran.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssran_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssran_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrlrn.b.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrlrn_b_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrlrn_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrlrn.h.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrlrn_h_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrlrn_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrlrn.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrlrn_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrlrn_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrarn.b.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrarn_b_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrarn_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrarn.h.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrarn_h_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrarn_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrarn.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrarn_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrarn_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrln.bu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrln_bu_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrln_bu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrln.hu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrln_hu_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrln_hu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrln.wu.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrln_wu_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrln_wu_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssran.bu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssran_bu_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssran_bu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssran.hu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssran_hu_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssran_hu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssran.wu.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssran_wu_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssran_wu_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrlrn.bu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrlrn_bu_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrlrn_bu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrlrn.hu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrlrn_hu_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrlrn_hu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrlrn.wu.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrlrn_wu_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrlrn_wu_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrarn.bu.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrarn_bu_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrarn_bu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrarn.hu.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrarn_hu_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrarn_hu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvssrarn.wu.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvssrarn_wu_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvssrarn_wu_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitclr.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitclr_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitclr_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitclr.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitclr_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitclr_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitclr.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitclr_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitclr_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitclr.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitclr_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitclr_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitset.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitset_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitset_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitset.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitset_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitset_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitset.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitset_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitset_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitset.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitset_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitset_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitrev.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitrev_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitrev_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitrev.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitrev_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitrev_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitrev.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitrev_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitrev_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvbitrev.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitrev_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvbitrev_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpackev.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackev_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpackev_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpackev.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackev_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpackev_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpackev.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackev_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpackev_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpackev.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackev_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpackev_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpackod.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackod_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpackod_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpackod.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackod_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpackod_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpackod.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackod_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpackod_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpackod.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpackod_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpackod_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvilvl.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvilvl_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvilvl_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvilvl.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvilvl_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvilvl_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvilvl.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvilvl_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvilvl_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvilvl.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvilvl_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvilvl_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvilvh.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvilvh_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvilvh_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvilvh.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvilvh_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvilvh_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvilvh.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvilvh_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvilvh_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvilvh.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvilvh_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvilvh_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpickev.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickev_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpickev_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpickev.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickev_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpickev_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpickev.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickev_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpickev_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpickev.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickev_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpickev_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpickod.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickod_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpickod_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpickod.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickod_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpickod_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpickod.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickod_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpickod_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpickod.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickod_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpickod_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvreplve.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve_b(a: V256, index: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve_b(a.as_ptr(), index, out.as_mut_ptr());
    }
    out
}

/// `xvreplve.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve_h(a: V256, index: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve_h(a.as_ptr(), index, out.as_mut_ptr());
    }
    out
}

/// `xvreplve.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve_w(a: V256, index: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve_w(a.as_ptr(), index, out.as_mut_ptr());
    }
    out
}

/// `xvreplve.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve_d(a: V256, index: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve_d(a.as_ptr(), index, out.as_mut_ptr());
    }
    out
}

/// `xvand.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvand_v(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvand_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvor.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvor_v(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvor_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvxor.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvxor_v(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvxor_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvnor.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvnor_v(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvnor_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvandn.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvandn_v(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvandn_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvorn.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvorn_v(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvorn_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrstp.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrstp_b(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrstp_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrstp.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrstp_h(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrstp_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvadd.q`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd_q(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvadd_q(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsub.q`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub_q(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsub_q(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsigncov.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsigncov_b(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsigncov_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsigncov.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsigncov_h(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsigncov_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsigncov.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsigncov_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsigncov_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvsigncov.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsigncov_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvsigncov_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfcvt.h.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcvt_h_s(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcvt_h_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftint.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftint_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftint_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrm.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrm_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrm_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrp.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrp_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrp_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrz.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrz_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrz_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrne.w.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrne_w_d(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrne_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvshuf.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf_h(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvshuf_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvshuf.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf_w(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvshuf_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvshuf.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf_d(a: V256, b: V256, c: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvshuf_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvperm.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvperm_w(a: V256, b: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvperm_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvclo.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvclo_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvclo_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvclo.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvclo_h(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvclo_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvclo.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvclo_w(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvclo_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvclo.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvclo_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvclo_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvclz.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvclz_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvclz_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvclz.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvclz_h(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvclz_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvclz.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvclz_w(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvclz_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvclz.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvclz_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvclz_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpcnt.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpcnt_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpcnt_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpcnt.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpcnt_h(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpcnt_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpcnt.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpcnt_w(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpcnt_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvpcnt.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpcnt_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvpcnt_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvneg.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvneg_w(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvneg_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmskltz.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmskltz_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmskltz_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmskltz.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmskltz_h(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmskltz_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmskltz.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmskltz_w(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmskltz_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmskltz.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmskltz_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmskltz_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmskgez.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmskgez_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmskgez_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvmsknz.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmsknz_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvmsknz_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfclass.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfclass_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfclass_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfclass.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfclass_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfclass_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftint.w.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftint_w_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftint_w_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftint.l.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftint_l_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftint_l_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrm.w.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrm_w_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrm_w_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrm.l.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrm_l_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrm_l_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrp.w.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrp_w_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrp_w_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrp.l.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrp_l_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrp_l_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrz.w.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrz_w_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrz_w_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrz.l.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrz_l_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrz_l_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrne.w.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrne_w_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrne_w_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrne.l.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrne_l_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrne_l_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftint.wu.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftint_wu_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftint_wu_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftint.lu.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftint_lu_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftint_lu_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrz.wu.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrz_wu_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrz_wu_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrz.lu.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrz_lu_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrz_lu_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintl.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintl_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintl_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftinth.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftinth_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftinth_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrml.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrml_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrml_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrmh.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrmh_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrmh_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrpl.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrpl_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrpl_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrph.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrph_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrph_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrzl.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrzl_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrzl_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrzh.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrzh_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrzh_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrnel.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrnel_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrnel_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvftintrneh.l.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvftintrneh_l_s(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvftintrneh_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvexth.h.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth_h_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvexth_h_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvexth.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth_w_h(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvexth_w_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvexth.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth_d_w(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvexth_d_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvexth.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth_q_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvexth_q_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvexth.hu.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth_hu_bu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvexth_hu_bu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvexth.wu.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth_wu_hu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvexth_wu_hu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvexth.du.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth_du_wu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvexth_du_wu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvexth.qu.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvexth_qu_du(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvexth_qu_du(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvreplgr2vr.b`
#[inline(always)]
pub fn xvreplgr2vr_b(value: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplgr2vr_b(value, out.as_mut_ptr());
    }
    out
}

/// `xvreplgr2vr.h`
#[inline(always)]
pub fn xvreplgr2vr_h(value: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplgr2vr_h(value, out.as_mut_ptr());
    }
    out
}

/// `xvreplgr2vr.w`
#[inline(always)]
pub fn xvreplgr2vr_w(value: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplgr2vr_w(value, out.as_mut_ptr());
    }
    out
}

/// `xvreplgr2vr.d`
#[inline(always)]
pub fn xvreplgr2vr_d(value: i64) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplgr2vr_d(value, out.as_mut_ptr());
    }
    out
}

/// `vext2xv.w.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_w_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_w_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.d.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_d_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_d_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.w.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_w_h(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_w_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.d.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_d_h(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_d_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_d_w(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_d_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.hu.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_hu_bu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_hu_bu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.wu.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_wu_bu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_wu_bu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.du.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_du_bu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_du_bu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.wu.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_wu_hu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_wu_hu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.du.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_du_hu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_du_hu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vext2xv.du.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn vext2xv_du_wu(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_vext2xv_du_wu(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvreplve0.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve0_b(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve0_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvreplve0.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve0_h(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve0_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvreplve0.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve0_w(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve0_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvreplve0.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve0_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve0_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvreplve0.q`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvreplve0_q(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvreplve0_q(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvextl.q.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvextl_q_d(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvextl_q_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvextl.qu.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvextl_qu_du(a: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvextl_qu_du(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
