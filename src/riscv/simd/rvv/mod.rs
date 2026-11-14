//! RISC-V V extension.
//!
//! The V extension is unstable. The caller must compile with
//! `-C target-feature=+v`; the wrappers do not carry `#[target_feature]`.
//! Every instruction wrapper is `unsafe`. The values are [`V128`].
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::simd::rvv::{add, splat_i32, V128};
//!
//! let left = unsafe { splat_i32(1) };
//! let right = unsafe { splat_i32(2) };
//! let sum = unsafe { add(left, right) };
//! assert_eq!(sum.to_bytes(), unsafe { splat_i32(3) }.to_bytes());
//! ```
//!
//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::{vadc_vim, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vadc_vim::<16>(v, 0) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::{vmadc_vim, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vmadc_vim::<16>(v, 0) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::{vmadc_vi, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vmadc_vi::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::{vrsub_vi, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vrsub_vi::<16>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::{vssra_vi, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vssra_vi::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::{vssrl_vi, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = unsafe { vssrl_vi::<32>(v) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::vsetivli;
//!
//! let _ = unsafe { vsetivli::<32, 0>() };
//! ```
//!
//! ```compile_fail
//! use rust_isa::riscv::simd::rvv::vsetvli;
//!
//! let _ = unsafe { vsetvli::<2048>(4) };
//! ```
//!
//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::riscv::simd::rvv::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { add(v128, v128) };
//! let _ = unsafe { add_i16(v128, v128) };
//! let _ = unsafe { add_i64(v128, v128) };
//! let _ = unsafe { add_i8(v128, v128) };
//! let _ = unsafe { and(v128, v128) };
//! let _ = unsafe { cmpeq_i32(v128, v128) };
//! let _ = unsafe { cmpgt_i32(v128, v128) };
//! let _ = unsafe { fadd(v128, v128) };
//! let _ = unsafe { fdiv(v128, v128) };
//! let _ = unsafe { fmul(v128, v128) };
//! let _ = unsafe { fsub(v128, v128) };
//! let _ = unsafe { mul(v128, v128) };
//! let _ = unsafe { mul_i16(v128, v128) };
//! let _ = unsafe { mul_i8(v128, v128) };
//! let _ = unsafe { or(v128, v128) };
//! let _ = unsafe { splat_i32(0) };
//! let _ = unsafe { sub_i16(v128, v128) };
//! let _ = unsafe { sub_i32(v128, v128) };
//! let _ = unsafe { sub_i64(v128, v128) };
//! let _ = unsafe { sub_i8(v128, v128) };
//! let _ = unsafe { v128_load(storage.0.as_ptr().cast()) };
//! let _ = unsafe { v128_store(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { vaadd(v128, v128) };
//! let _ = unsafe { vaaddu(v128, v128) };
//! let _ = unsafe { vadd(v128, v128) };
//! let _ = unsafe { vand(v128, v128) };
//! let _ = unsafe { vasub(v128, v128) };
//! let _ = unsafe { vasubu(v128, v128) };
//! let _ = unsafe { vcompress(v128, 0) };
//! let _ = unsafe { vcpop(0) };
//! let _ = unsafe { vdiv(v128, v128) };
//! let _ = unsafe { vdivu(v128, v128) };
//! let _ = unsafe { vfabs(v128) };
//! let _ = unsafe { vfadd(v128, v128) };
//! let _ = unsafe { vfclass(v128) };
//! let _ = unsafe { vfcvt_f_x(v128) };
//! let _ = unsafe { vfcvt_f_xu(v128) };
//! let _ = unsafe { vfcvt_rtz_x_f(v128) };
//! let _ = unsafe { vfcvt_rtz_xu_f(v128) };
//! let _ = unsafe { vfcvt_x_f(v128) };
//! let _ = unsafe { vfcvt_xu_f(v128) };
//! let _ = unsafe { vfwcvt_xu_f_v(v128) };
//! let _ = unsafe { vfwcvt_x_f_v(v128) };
//! let _ = unsafe { vfwcvt_f_xu_v(v128) };
//! let _ = unsafe { vfwcvt_f_x_v(v128) };
//! let _ = unsafe { vfwcvt_f_f_v(v128) };
//! let _ = unsafe { vfwcvt_rtz_xu_f_v(v128) };
//! let _ = unsafe { vfwcvt_rtz_x_f_v(v128) };
//! let _ = unsafe { vfncvt_xu_f_w(v128) };
//! let _ = unsafe { vfncvt_x_f_w(v128) };
//! let _ = unsafe { vfncvt_f_xu_w(v128) };
//! let _ = unsafe { vfncvt_f_x_w(v128) };
//! let _ = unsafe { vfncvt_f_f_w(v128) };
//! let _ = unsafe { vfncvt_rod_f_f_w(v128) };
//! let _ = unsafe { vfncvt_rtz_xu_f_w(v128) };
//! let _ = unsafe { vfncvt_rtz_x_f_w(v128) };
//! let _ = unsafe { vfdiv(v128, v128) };
//! let _ = unsafe { vfirst(0) };
//! let _ = unsafe { vfma(v128, v128, v128) };
//! let _ = unsafe { vfmax(v128, v128) };
//! let _ = unsafe { vfmerge(v128, 0.0f32, 0) };
//! let _ = unsafe { vfmin(v128, v128) };
//! let _ = unsafe { vfms(v128, v128, v128) };
//! let _ = unsafe { vfmadd_vv(v128, v128, v128) };
//! let _ = unsafe { vfmadd_vf(v128, v128, 0.0) };
//! let _ = unsafe { vfmsub_vv(v128, v128, v128) };
//! let _ = unsafe { vfmsub_vf(v128, v128, 0.0) };
//! let _ = unsafe { vfnmadd_vv(v128, v128, v128) };
//! let _ = unsafe { vfnmadd_vf(v128, v128, 0.0) };
//! let _ = unsafe { vfnmsub_vv(v128, v128, v128) };
//! let _ = unsafe { vfnmsub_vf(v128, v128, 0.0) };
//! let _ = unsafe { vfrsub_vf(v128, 0.0) };
//! let _ = unsafe { vfmacc_vv(v128, v128, v128) };
//! let _ = unsafe { vfmacc_vf(v128, v128, 0.0) };
//! let _ = unsafe { vfnmacc_vv(v128, v128, v128) };
//! let _ = unsafe { vfnmacc_vf(v128, v128, 0.0) };
//! let _ = unsafe { vfmsac_vv(v128, v128, v128) };
//! let _ = unsafe { vfmsac_vf(v128, v128, 0.0) };
//! let _ = unsafe { vfnmsac_vv(v128, v128, v128) };
//! let _ = unsafe { vfnmsac_vf(v128, v128, 0.0) };
//! let _ = unsafe { vmulhu_vv(v128, v128) };
//! let _ = unsafe { vmulhu_vx(v128, 0) };
//! let _ = unsafe { vmulhsu_vv(v128, v128) };
//! let _ = unsafe { vmulhsu_vx(v128, 0) };
//! let _ = unsafe { vmulh_vv(v128, v128) };
//! let _ = unsafe { vmulh_vx(v128, 0) };
//! let _ = unsafe { vmv1r(v128) };
//! let _ = unsafe { vmv2r([v128; 2]) };
//! let _ = unsafe { vmv4r([v128; 4]) };
//! let _ = unsafe { vmv8r([v128; 8]) };
//! let _ = unsafe { vfwredusum_vs(v128, v128) };
//! let _ = unsafe { vfwredosum_vs(v128, v128) };
//! let _ = unsafe { vrsub_vx(v128, 0) };
//! let _ = unsafe { vrsub_vi::<0>(v128) };
//! let _ = unsafe { vadc_vvm(v128, v128, 0) };
//! let _ = unsafe { vadc_vxm(v128, 0, 0) };
//! let _ = unsafe { vadc_vim::<0>(v128, 0) };
//! let _ = unsafe { vmadc_vvm(v128, v128, 0) };
//! let _ = unsafe { vmadc_vxm(v128, 0, 0) };
//! let _ = unsafe { vmadc_vv(v128, v128) };
//! let _ = unsafe { vmadc_vx(v128, 0) };
//! let _ = unsafe { vmadc_vim::<0>(v128, 0) };
//! let _ = unsafe { vmadc_vi::<0>(v128) };
//! let _ = unsafe { vsbc_vvm(v128, v128, 0) };
//! let _ = unsafe { vsbc_vxm(v128, 0, 0) };
//! let _ = unsafe { vmsbc_vvm(v128, v128, 0) };
//! let _ = unsafe { vmsbc_vxm(v128, 0, 0) };
//! let _ = unsafe { vmsbc_vv(v128, v128) };
//! let _ = unsafe { vmsbc_vx(v128, 0) };
//! let _ = unsafe { vsmul_vv(v128, v128) };
//! let _ = unsafe { vsmul_vx(v128, 0) };
//! let _ = unsafe { vssrl_vv(v128, v128) };
//! let _ = unsafe { vssrl_vx(v128, 0) };
//! let _ = unsafe { vssrl_vi::<0>(v128) };
//! let _ = unsafe { vssra_vv(v128, v128) };
//! let _ = unsafe { vssra_vx(v128, 0) };
//! let _ = unsafe { vssra_vi::<0>(v128) };
//! let _ = unsafe { vfmul(v128, v128) };
//! let _ = unsafe { vfmv(0.0f32) };
//! let _ = unsafe { vfmv_f_s(v128) };
//! let _ = unsafe { vfneg(v128) };
//! let _ = unsafe { vfnma(v128, v128, v128) };
//! let _ = unsafe { vfnms(v128, v128, v128) };
//! let _ = unsafe { vfrdiv(v128, 0.0f32) };
//! let _ = unsafe { vfrec7(v128) };
//! let _ = unsafe { vfredmax(v128) };
//! let _ = unsafe { vfredmin(v128) };
//! let _ = unsafe { vfredosum(v128) };
//! let _ = unsafe { vfredsum(v128) };
//! let _ = unsafe { vfredusum(v128) };
//! let _ = unsafe { vfrsqrt7(v128) };
//! let _ = unsafe { vfsgnj(v128, v128) };
//! let _ = unsafe { vfsgnjn(v128, v128) };
//! let _ = unsafe { vfsgnjx(v128, v128) };
//! let _ = unsafe { vfsqrt(v128) };
//! let _ = unsafe { vfsub(v128, v128) };
//! let _ = unsafe { vfslide1up(v128, 0.0) };
//! let _ = unsafe { vfslide1down(v128, 0.0) };
//! let _ = unsafe { vmfeq_vv(v128, v128) };
//! let _ = unsafe { vmfeq_vf(v128, 0.0) };
//! let _ = unsafe { vmfle_vv(v128, v128) };
//! let _ = unsafe { vmfle_vf(v128, 0.0) };
//! let _ = unsafe { vmflt_vv(v128, v128) };
//! let _ = unsafe { vmflt_vf(v128, 0.0) };
//! let _ = unsafe { vmfne_vv(v128, v128) };
//! let _ = unsafe { vmfne_vf(v128, 0.0) };
//! let _ = unsafe { vmfgt_vv(v128, v128) };
//! let _ = unsafe { vmfgt_vf(v128, 0.0) };
//! let _ = unsafe { vmfge_vv(v128, v128) };
//! let _ = unsafe { vmfge_vf(v128, 0.0) };
//! let _ = unsafe { vfwadd(v128, v128) };
//! let _ = unsafe { vfwmacc(v128, v128, v128) };
//! let _ = unsafe { vfwmsac(v128, v128, v128) };
//! let _ = unsafe { vfwmul(v128, v128) };
//! let _ = unsafe { vfwnmacc(v128, v128, v128) };
//! let _ = unsafe { vfwnmsac(v128, v128, v128) };
//! let _ = unsafe { vfwsub(v128, v128) };
//! let _ = unsafe { vid() };
//! let _ = unsafe { viota(0) };
//! let _ = unsafe { vle16(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vle32(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vle64(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vle8(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vle8ff(storage.0.as_ptr()) };
//! let _ = unsafe { vle16ff(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vle32ff(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vle64ff(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vl1re8(storage.0.as_ptr()) };
//! let _ = unsafe { vl2re8(storage.0.as_ptr()) };
//! let _ = unsafe { vl4re8(storage.0.as_ptr()) };
//! let _ = unsafe { vl8re8(storage.0.as_ptr()) };
//! let _ = unsafe { vl1re16(storage.0.as_ptr()) };
//! let _ = unsafe { vl2re16(storage.0.as_ptr()) };
//! let _ = unsafe { vl4re16(storage.0.as_ptr()) };
//! let _ = unsafe { vl8re16(storage.0.as_ptr()) };
//! let _ = unsafe { vl1re32(storage.0.as_ptr()) };
//! let _ = unsafe { vl2re32(storage.0.as_ptr()) };
//! let _ = unsafe { vl4re32(storage.0.as_ptr()) };
//! let _ = unsafe { vl8re32(storage.0.as_ptr()) };
//! let _ = unsafe { vl1re64(storage.0.as_ptr()) };
//! let _ = unsafe { vl2re64(storage.0.as_ptr()) };
//! let _ = unsafe { vl4re64(storage.0.as_ptr()) };
//! let _ = unsafe { vl8re64(storage.0.as_ptr()) };
//! let _ = unsafe { vs1r(storage.0.as_mut_ptr(), v128) };
//! let _ = unsafe { vs2r(storage.0.as_mut_ptr(), [v128; 2]) };
//! let _ = unsafe { vs4r(storage.0.as_mut_ptr(), [v128; 4]) };
//! let _ = unsafe { vs8r(storage.0.as_mut_ptr(), [v128; 8]) };
//! let _ = unsafe { vlm(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vloxei16(storage.0.as_ptr().cast(), v128) };
//! let _ = unsafe { vloxei32(storage.0.as_ptr().cast(), v128) };
//! let _ = unsafe { vloxei64(storage.0.as_ptr().cast(), v128) };
//! let _ = unsafe { vloxei8(storage.0.as_ptr().cast(), v128) };
//! let _ = unsafe { vlse16(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlse32(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlse64(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlse8(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlseg2(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vlseg3(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vlseg4(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vlseg5(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vlseg6(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vlseg7(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vlseg8(storage.0.as_ptr().cast()) };
//! let _ = unsafe { vlsseg2(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlsseg3(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlsseg4(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlsseg5(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlsseg6(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlsseg7(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vlsseg8(storage.0.as_ptr().cast(), 0) };
//! let _ = unsafe { vluxei16(storage.0.as_ptr().cast(), v128) };
//! let _ = unsafe { vluxei32(storage.0.as_ptr().cast(), v128) };
//! let _ = unsafe { vluxei64(storage.0.as_ptr().cast(), v128) };
//! let _ = unsafe { vluxei8(storage.0.as_ptr().cast(), v128) };
//! let _ = unsafe { vmacc(v128, v128, v128) };
//! let _ = unsafe { vmadd(v128, v128, v128) };
//! let _ = unsafe { vmand(0, 0) };
//! let _ = unsafe { vmandn(0, 0) };
//! let _ = unsafe { vmax(v128, v128) };
//! let _ = unsafe { vmaxu(v128, v128) };
//! let _ = unsafe { vmerge(v128, v128, 0) };
//! let _ = unsafe { vmin(v128, v128) };
//! let _ = unsafe { vminu(v128, v128) };
//! let _ = unsafe { vmnand(0, 0) };
//! let _ = unsafe { vmnor(0, 0) };
//! let _ = unsafe { vmor(0, 0) };
//! let _ = unsafe { vmorn(0, 0) };
//! let _ = unsafe { vmsbf(0) };
//! let _ = unsafe { vmseq(v128, v128) };
//! let _ = unsafe { vmseq_mask(v128, v128) };
//! let _ = unsafe { vmsge(v128, v128) };
//! let _ = unsafe { vmsge_mask(v128, v128) };
//! let _ = unsafe { vmsgeu(v128, v128) };
//! let _ = unsafe { vmsgeu_mask(v128, v128) };
//! let _ = unsafe { vmsgt(v128, v128) };
//! let _ = unsafe { vmsgt_mask(v128, v128) };
//! let _ = unsafe { vmsgtu(v128, v128) };
//! let _ = unsafe { vmsgtu_mask(v128, v128) };
//! let _ = unsafe { vmsif(0) };
//! let _ = unsafe { vmsle(v128, v128) };
//! let _ = unsafe { vmsle_mask(v128, v128) };
//! let _ = unsafe { vmsleu(v128, v128) };
//! let _ = unsafe { vmsleu_mask(v128, v128) };
//! let _ = unsafe { vmslt(v128, v128) };
//! let _ = unsafe { vmslt_mask(v128, v128) };
//! let _ = unsafe { vmsltu(v128, v128) };
//! let _ = unsafe { vmsltu_mask(v128, v128) };
//! let _ = unsafe { vmsne(v128, v128) };
//! let _ = unsafe { vmsne_mask(v128, v128) };
//! let _ = unsafe { vmsof(0) };
//! let _ = unsafe { vmul(v128, v128) };
//! let _ = unsafe { vmv(v128) };
//! let _ = unsafe { vmv_s_x(0) };
//! let _ = unsafe { vmv_v_x(0) };
//! let _ = unsafe { vmv_x_s(v128) };
//! let _ = unsafe { vmxnor(0, 0) };
//! let _ = unsafe { vmxor(0, 0) };
//! let _ = unsafe { vnclip(v128, 0) };
//! let _ = unsafe { vnclipu(v128, 0) };
//! let _ = unsafe { vnmsac(v128, v128, v128) };
//! let _ = unsafe { vnmsub(v128, v128, v128) };
//! let _ = unsafe { vnsra(v128, 0) };
//! let _ = unsafe { vnsrl(v128, 0) };
//! let _ = unsafe { vor(v128, v128) };
//! let _ = unsafe { vredand(v128) };
//! let _ = unsafe { vredmax(v128) };
//! let _ = unsafe { vredmaxu(v128) };
//! let _ = unsafe { vredmin(v128) };
//! let _ = unsafe { vredminu(v128) };
//! let _ = unsafe { vredor(v128) };
//! let _ = unsafe { vredsum(v128) };
//! let _ = unsafe { vredxor(v128) };
//! let _ = unsafe { vrem(v128, v128) };
//! let _ = unsafe { vremu(v128, v128) };
//! let _ = unsafe { vrgather(v128, v128) };
//! let _ = unsafe { vrgatherei16(v128, v128) };
//! let _ = unsafe { vsadd(v128, v128) };
//! let _ = unsafe { vsaddu(v128, v128) };
//! let _ = unsafe { vse16(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { vse32(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { vse64(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { vse8(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = unsafe { vsext(v128) };
//! let _ = unsafe { vslide1down(v128, 0) };
//! let _ = unsafe { vslide1up(v128, 0) };
//! let _ = unsafe { vslidedown(v128, 0) };
//! let _ = unsafe { vslideup(v128, v128, 0) };
//! let _ = unsafe { vsll(v128, v128) };
//! let _ = unsafe { vsm(storage.0.as_mut_ptr().cast(), 0) };
//! let _ = unsafe { vsoxei16(storage.0.as_mut_ptr().cast(), v128, v128) };
//! let _ = unsafe { vsoxei32(storage.0.as_mut_ptr().cast(), v128, v128) };
//! let _ = unsafe { vsoxei64(storage.0.as_mut_ptr().cast(), v128, v128) };
//! let _ = unsafe { vsoxei8(storage.0.as_mut_ptr().cast(), v128, v128) };
//! let _ = unsafe { vsra(v128, v128) };
//! let _ = unsafe { vsrl(v128, v128) };
//! let _ = unsafe { vsse16(storage.0.as_mut_ptr().cast(), 0, v128) };
//! let _ = unsafe { vsse32(storage.0.as_mut_ptr().cast(), 0, v128) };
//! let _ = unsafe { vsse64(storage.0.as_mut_ptr().cast(), 0, v128) };
//! let _ = unsafe { vsse8(storage.0.as_mut_ptr().cast(), 0, v128) };
//! let _ = unsafe { vsseg2(storage.0.as_mut_ptr().cast(), [v128; 2]) };
//! let _ = unsafe { vsseg3(storage.0.as_mut_ptr().cast(), [v128; 3]) };
//! let _ = unsafe { vsseg4(storage.0.as_mut_ptr().cast(), [v128; 4]) };
//! let _ = unsafe { vsseg5(storage.0.as_mut_ptr().cast(), [v128; 5]) };
//! let _ = unsafe { vsseg6(storage.0.as_mut_ptr().cast(), [v128; 6]) };
//! let _ = unsafe { vsseg7(storage.0.as_mut_ptr().cast(), [v128; 7]) };
//! let _ = unsafe { vsseg8(storage.0.as_mut_ptr().cast(), [v128; 8]) };
//! let _ = unsafe { vssseg2(storage.0.as_mut_ptr().cast(), 0, [v128; 2]) };
//! let _ = unsafe { vssseg3(storage.0.as_mut_ptr().cast(), 0, [v128; 3]) };
//! let _ = unsafe { vssseg4(storage.0.as_mut_ptr().cast(), 0, [v128; 4]) };
//! let _ = unsafe { vssseg5(storage.0.as_mut_ptr().cast(), 0, [v128; 5]) };
//! let _ = unsafe { vssseg6(storage.0.as_mut_ptr().cast(), 0, [v128; 6]) };
//! let _ = unsafe { vssseg7(storage.0.as_mut_ptr().cast(), 0, [v128; 7]) };
//! let _ = unsafe { vssseg8(storage.0.as_mut_ptr().cast(), 0, [v128; 8]) };
//! let _ = unsafe { vssub(v128, v128) };
//! let _ = unsafe { vssubu(v128, v128) };
//! let _ = unsafe { vsub(v128, v128) };
//! let _ = unsafe { vsuxei16(storage.0.as_mut_ptr().cast(), v128, v128) };
//! let _ = unsafe { vsuxei32(storage.0.as_mut_ptr().cast(), v128, v128) };
//! let _ = unsafe { vsuxei64(storage.0.as_mut_ptr().cast(), v128, v128) };
//! let _ = unsafe { vsuxei8(storage.0.as_mut_ptr().cast(), v128, v128) };
//! let _ = unsafe { vwadd(v128, v128) };
//! let _ = unsafe { vwaddu(v128, v128) };
//! let _ = unsafe { vwmacc(v128, v128, v128) };
//! let _ = unsafe { vwmaccsu(v128, v128, v128) };
//! let _ = unsafe { vwmaccu(v128, v128, v128) };
//! let _ = unsafe { vwmaccus(v128, 0, v128) };
//! let _ = unsafe { vwmul(v128, v128) };
//! let _ = unsafe { vwmulsu(v128, v128) };
//! let _ = unsafe { vwmulu(v128, v128) };
//! let _ = unsafe { vwredsum(v128) };
//! let _ = unsafe { vwredsumu(v128) };
//! let _ = unsafe { vwsub(v128, v128) };
//! let _ = unsafe { vwsubu(v128, v128) };
//! let _ = unsafe { vxor(v128, v128) };
//! let _ = unsafe { vzext(v128) };
//! let _ = unsafe { xor(v128, v128) };
//! ```
//!
//! let _ = unsafe { vsetivli::<4, 0>() };
//! let _ = unsafe { vsetvli::<0>(4) };
//! let _ = unsafe { vsetvl(4, 0) };
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::simd::rvv::add as _;
//! use rust_isa::riscv::simd::rvv::add_i16 as _;
//! use rust_isa::riscv::simd::rvv::add_i64 as _;
//! use rust_isa::riscv::simd::rvv::add_i8 as _;
//! use rust_isa::riscv::simd::rvv::and as _;
//! use rust_isa::riscv::simd::rvv::cmpeq_i32 as _;
//! use rust_isa::riscv::simd::rvv::cmpgt_i32 as _;
//! use rust_isa::riscv::simd::rvv::fadd as _;
//! use rust_isa::riscv::simd::rvv::fdiv as _;
//! use rust_isa::riscv::simd::rvv::fmul as _;
//! use rust_isa::riscv::simd::rvv::fsub as _;
//! use rust_isa::riscv::simd::rvv::mul as _;
//! use rust_isa::riscv::simd::rvv::mul_i16 as _;
//! use rust_isa::riscv::simd::rvv::mul_i8 as _;
//! use rust_isa::riscv::simd::rvv::or as _;
//! use rust_isa::riscv::simd::rvv::splat_i32 as _;
//! use rust_isa::riscv::simd::rvv::sub_i16 as _;
//! use rust_isa::riscv::simd::rvv::sub_i32 as _;
//! use rust_isa::riscv::simd::rvv::sub_i64 as _;
//! use rust_isa::riscv::simd::rvv::sub_i8 as _;
//! use rust_isa::riscv::simd::rvv::v128_load as _;
//! use rust_isa::riscv::simd::rvv::v128_store as _;
//! use rust_isa::riscv::simd::rvv::vaadd as _;
//! use rust_isa::riscv::simd::rvv::vaaddu as _;
//! use rust_isa::riscv::simd::rvv::vadc_vim as _;
//! use rust_isa::riscv::simd::rvv::vadc_vvm as _;
//! use rust_isa::riscv::simd::rvv::vadc_vxm as _;
//! use rust_isa::riscv::simd::rvv::vadd as _;
//! use rust_isa::riscv::simd::rvv::vand as _;
//! use rust_isa::riscv::simd::rvv::vasub as _;
//! use rust_isa::riscv::simd::rvv::vasubu as _;
//! use rust_isa::riscv::simd::rvv::vcompress as _;
//! use rust_isa::riscv::simd::rvv::vcpop as _;
//! use rust_isa::riscv::simd::rvv::vdiv as _;
//! use rust_isa::riscv::simd::rvv::vdivu as _;
//! use rust_isa::riscv::simd::rvv::vfabs as _;
//! use rust_isa::riscv::simd::rvv::vfadd as _;
//! use rust_isa::riscv::simd::rvv::vfclass as _;
//! use rust_isa::riscv::simd::rvv::vfcvt_f_x as _;
//! use rust_isa::riscv::simd::rvv::vfcvt_f_xu as _;
//! use rust_isa::riscv::simd::rvv::vfcvt_rtz_x_f as _;
//! use rust_isa::riscv::simd::rvv::vfcvt_rtz_xu_f as _;
//! use rust_isa::riscv::simd::rvv::vfcvt_x_f as _;
//! use rust_isa::riscv::simd::rvv::vfcvt_xu_f as _;
//! use rust_isa::riscv::simd::rvv::vfdiv as _;
//! use rust_isa::riscv::simd::rvv::vfirst as _;
//! use rust_isa::riscv::simd::rvv::vfma as _;
//! use rust_isa::riscv::simd::rvv::vfmacc_vf as _;
//! use rust_isa::riscv::simd::rvv::vfmacc_vv as _;
//! use rust_isa::riscv::simd::rvv::vfmadd_vf as _;
//! use rust_isa::riscv::simd::rvv::vfmadd_vv as _;
//! use rust_isa::riscv::simd::rvv::vfmax as _;
//! use rust_isa::riscv::simd::rvv::vfmerge as _;
//! use rust_isa::riscv::simd::rvv::vfmin as _;
//! use rust_isa::riscv::simd::rvv::vfms as _;
//! use rust_isa::riscv::simd::rvv::vfmsac_vf as _;
//! use rust_isa::riscv::simd::rvv::vfmsac_vv as _;
//! use rust_isa::riscv::simd::rvv::vfmsub_vf as _;
//! use rust_isa::riscv::simd::rvv::vfmsub_vv as _;
//! use rust_isa::riscv::simd::rvv::vfmul as _;
//! use rust_isa::riscv::simd::rvv::vfmv as _;
//! use rust_isa::riscv::simd::rvv::vfmv_f_s as _;
//! use rust_isa::riscv::simd::rvv::vfncvt_f_f_w as _;
//! use rust_isa::riscv::simd::rvv::vfncvt_f_x_w as _;
//! use rust_isa::riscv::simd::rvv::vfncvt_f_xu_w as _;
//! use rust_isa::riscv::simd::rvv::vfncvt_rod_f_f_w as _;
//! use rust_isa::riscv::simd::rvv::vfncvt_rtz_x_f_w as _;
//! use rust_isa::riscv::simd::rvv::vfncvt_rtz_xu_f_w as _;
//! use rust_isa::riscv::simd::rvv::vfncvt_x_f_w as _;
//! use rust_isa::riscv::simd::rvv::vfncvt_xu_f_w as _;
//! use rust_isa::riscv::simd::rvv::vfneg as _;
//! use rust_isa::riscv::simd::rvv::vfnma as _;
//! use rust_isa::riscv::simd::rvv::vfnmacc_vf as _;
//! use rust_isa::riscv::simd::rvv::vfnmacc_vv as _;
//! use rust_isa::riscv::simd::rvv::vfnmadd_vf as _;
//! use rust_isa::riscv::simd::rvv::vfnmadd_vv as _;
//! use rust_isa::riscv::simd::rvv::vfnms as _;
//! use rust_isa::riscv::simd::rvv::vfnmsac_vf as _;
//! use rust_isa::riscv::simd::rvv::vfnmsac_vv as _;
//! use rust_isa::riscv::simd::rvv::vfnmsub_vf as _;
//! use rust_isa::riscv::simd::rvv::vfnmsub_vv as _;
//! use rust_isa::riscv::simd::rvv::vfrdiv as _;
//! use rust_isa::riscv::simd::rvv::vfrec7 as _;
//! use rust_isa::riscv::simd::rvv::vfredmax as _;
//! use rust_isa::riscv::simd::rvv::vfredmin as _;
//! use rust_isa::riscv::simd::rvv::vfredosum as _;
//! use rust_isa::riscv::simd::rvv::vfredsum as _;
//! use rust_isa::riscv::simd::rvv::vfredusum as _;
//! use rust_isa::riscv::simd::rvv::vfrsqrt7 as _;
//! use rust_isa::riscv::simd::rvv::vfrsub_vf as _;
//! use rust_isa::riscv::simd::rvv::vfsgnj as _;
//! use rust_isa::riscv::simd::rvv::vfsgnjn as _;
//! use rust_isa::riscv::simd::rvv::vfsgnjx as _;
//! use rust_isa::riscv::simd::rvv::vfslide1down as _;
//! use rust_isa::riscv::simd::rvv::vfslide1up as _;
//! use rust_isa::riscv::simd::rvv::vfsqrt as _;
//! use rust_isa::riscv::simd::rvv::vfsub as _;
//! use rust_isa::riscv::simd::rvv::vfwadd as _;
//! use rust_isa::riscv::simd::rvv::vfwcvt_f_f_v as _;
//! use rust_isa::riscv::simd::rvv::vfwcvt_f_x_v as _;
//! use rust_isa::riscv::simd::rvv::vfwcvt_f_xu_v as _;
//! use rust_isa::riscv::simd::rvv::vfwcvt_rtz_x_f_v as _;
//! use rust_isa::riscv::simd::rvv::vfwcvt_rtz_xu_f_v as _;
//! use rust_isa::riscv::simd::rvv::vfwcvt_x_f_v as _;
//! use rust_isa::riscv::simd::rvv::vfwcvt_xu_f_v as _;
//! use rust_isa::riscv::simd::rvv::vfwmacc as _;
//! use rust_isa::riscv::simd::rvv::vfwmsac as _;
//! use rust_isa::riscv::simd::rvv::vfwmul as _;
//! use rust_isa::riscv::simd::rvv::vfwnmacc as _;
//! use rust_isa::riscv::simd::rvv::vfwnmsac as _;
//! use rust_isa::riscv::simd::rvv::vfwredosum_vs as _;
//! use rust_isa::riscv::simd::rvv::vfwredusum_vs as _;
//! use rust_isa::riscv::simd::rvv::vfwsub as _;
//! use rust_isa::riscv::simd::rvv::vid as _;
//! use rust_isa::riscv::simd::rvv::viota as _;
//! use rust_isa::riscv::simd::rvv::vl1re16 as _;
//! use rust_isa::riscv::simd::rvv::vl1re32 as _;
//! use rust_isa::riscv::simd::rvv::vl1re64 as _;
//! use rust_isa::riscv::simd::rvv::vl1re8 as _;
//! use rust_isa::riscv::simd::rvv::vl2re16 as _;
//! use rust_isa::riscv::simd::rvv::vl2re32 as _;
//! use rust_isa::riscv::simd::rvv::vl2re64 as _;
//! use rust_isa::riscv::simd::rvv::vl2re8 as _;
//! use rust_isa::riscv::simd::rvv::vl4re16 as _;
//! use rust_isa::riscv::simd::rvv::vl4re32 as _;
//! use rust_isa::riscv::simd::rvv::vl4re64 as _;
//! use rust_isa::riscv::simd::rvv::vl4re8 as _;
//! use rust_isa::riscv::simd::rvv::vl8re16 as _;
//! use rust_isa::riscv::simd::rvv::vl8re32 as _;
//! use rust_isa::riscv::simd::rvv::vl8re64 as _;
//! use rust_isa::riscv::simd::rvv::vl8re8 as _;
//! use rust_isa::riscv::simd::rvv::vle16 as _;
//! use rust_isa::riscv::simd::rvv::vle16ff as _;
//! use rust_isa::riscv::simd::rvv::vle32 as _;
//! use rust_isa::riscv::simd::rvv::vle32ff as _;
//! use rust_isa::riscv::simd::rvv::vle64 as _;
//! use rust_isa::riscv::simd::rvv::vle64ff as _;
//! use rust_isa::riscv::simd::rvv::vle8 as _;
//! use rust_isa::riscv::simd::rvv::vle8ff as _;
//! use rust_isa::riscv::simd::rvv::vlm as _;
//! use rust_isa::riscv::simd::rvv::vloxei16 as _;
//! use rust_isa::riscv::simd::rvv::vloxei32 as _;
//! use rust_isa::riscv::simd::rvv::vloxei64 as _;
//! use rust_isa::riscv::simd::rvv::vloxei8 as _;
//! use rust_isa::riscv::simd::rvv::vlse16 as _;
//! use rust_isa::riscv::simd::rvv::vlse32 as _;
//! use rust_isa::riscv::simd::rvv::vlse64 as _;
//! use rust_isa::riscv::simd::rvv::vlse8 as _;
//! use rust_isa::riscv::simd::rvv::vlseg2 as _;
//! use rust_isa::riscv::simd::rvv::vlseg3 as _;
//! use rust_isa::riscv::simd::rvv::vlseg4 as _;
//! use rust_isa::riscv::simd::rvv::vlseg5 as _;
//! use rust_isa::riscv::simd::rvv::vlseg6 as _;
//! use rust_isa::riscv::simd::rvv::vlseg7 as _;
//! use rust_isa::riscv::simd::rvv::vlseg8 as _;
//! use rust_isa::riscv::simd::rvv::vlsseg2 as _;
//! use rust_isa::riscv::simd::rvv::vlsseg3 as _;
//! use rust_isa::riscv::simd::rvv::vlsseg4 as _;
//! use rust_isa::riscv::simd::rvv::vlsseg5 as _;
//! use rust_isa::riscv::simd::rvv::vlsseg6 as _;
//! use rust_isa::riscv::simd::rvv::vlsseg7 as _;
//! use rust_isa::riscv::simd::rvv::vlsseg8 as _;
//! use rust_isa::riscv::simd::rvv::vluxei16 as _;
//! use rust_isa::riscv::simd::rvv::vluxei32 as _;
//! use rust_isa::riscv::simd::rvv::vluxei64 as _;
//! use rust_isa::riscv::simd::rvv::vluxei8 as _;
//! use rust_isa::riscv::simd::rvv::vmacc as _;
//! use rust_isa::riscv::simd::rvv::vmadc_vi as _;
//! use rust_isa::riscv::simd::rvv::vmadc_vim as _;
//! use rust_isa::riscv::simd::rvv::vmadc_vv as _;
//! use rust_isa::riscv::simd::rvv::vmadc_vvm as _;
//! use rust_isa::riscv::simd::rvv::vmadc_vx as _;
//! use rust_isa::riscv::simd::rvv::vmadc_vxm as _;
//! use rust_isa::riscv::simd::rvv::vmadd as _;
//! use rust_isa::riscv::simd::rvv::vmand as _;
//! use rust_isa::riscv::simd::rvv::vmandn as _;
//! use rust_isa::riscv::simd::rvv::vmax as _;
//! use rust_isa::riscv::simd::rvv::vmaxu as _;
//! use rust_isa::riscv::simd::rvv::vmerge as _;
//! use rust_isa::riscv::simd::rvv::vmfeq_vf as _;
//! use rust_isa::riscv::simd::rvv::vmfeq_vv as _;
//! use rust_isa::riscv::simd::rvv::vmfge_vf as _;
//! use rust_isa::riscv::simd::rvv::vmfge_vv as _;
//! use rust_isa::riscv::simd::rvv::vmfgt_vf as _;
//! use rust_isa::riscv::simd::rvv::vmfgt_vv as _;
//! use rust_isa::riscv::simd::rvv::vmfle_vf as _;
//! use rust_isa::riscv::simd::rvv::vmfle_vv as _;
//! use rust_isa::riscv::simd::rvv::vmflt_vf as _;
//! use rust_isa::riscv::simd::rvv::vmflt_vv as _;
//! use rust_isa::riscv::simd::rvv::vmfne_vf as _;
//! use rust_isa::riscv::simd::rvv::vmfne_vv as _;
//! use rust_isa::riscv::simd::rvv::vmin as _;
//! use rust_isa::riscv::simd::rvv::vminu as _;
//! use rust_isa::riscv::simd::rvv::vmnand as _;
//! use rust_isa::riscv::simd::rvv::vmnor as _;
//! use rust_isa::riscv::simd::rvv::vmor as _;
//! use rust_isa::riscv::simd::rvv::vmorn as _;
//! use rust_isa::riscv::simd::rvv::vmsbc_vv as _;
//! use rust_isa::riscv::simd::rvv::vmsbc_vvm as _;
//! use rust_isa::riscv::simd::rvv::vmsbc_vx as _;
//! use rust_isa::riscv::simd::rvv::vmsbc_vxm as _;
//! use rust_isa::riscv::simd::rvv::vmsbf as _;
//! use rust_isa::riscv::simd::rvv::vmseq as _;
//! use rust_isa::riscv::simd::rvv::vmseq_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsge as _;
//! use rust_isa::riscv::simd::rvv::vmsge_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsgeu as _;
//! use rust_isa::riscv::simd::rvv::vmsgeu_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsgt as _;
//! use rust_isa::riscv::simd::rvv::vmsgt_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsgtu as _;
//! use rust_isa::riscv::simd::rvv::vmsgtu_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsif as _;
//! use rust_isa::riscv::simd::rvv::vmsle as _;
//! use rust_isa::riscv::simd::rvv::vmsle_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsleu as _;
//! use rust_isa::riscv::simd::rvv::vmsleu_mask as _;
//! use rust_isa::riscv::simd::rvv::vmslt as _;
//! use rust_isa::riscv::simd::rvv::vmslt_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsltu as _;
//! use rust_isa::riscv::simd::rvv::vmsltu_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsne as _;
//! use rust_isa::riscv::simd::rvv::vmsne_mask as _;
//! use rust_isa::riscv::simd::rvv::vmsof as _;
//! use rust_isa::riscv::simd::rvv::vmul as _;
//! use rust_isa::riscv::simd::rvv::vmulh_vv as _;
//! use rust_isa::riscv::simd::rvv::vmulh_vx as _;
//! use rust_isa::riscv::simd::rvv::vmulhsu_vv as _;
//! use rust_isa::riscv::simd::rvv::vmulhsu_vx as _;
//! use rust_isa::riscv::simd::rvv::vmulhu_vv as _;
//! use rust_isa::riscv::simd::rvv::vmulhu_vx as _;
//! use rust_isa::riscv::simd::rvv::vmv as _;
//! use rust_isa::riscv::simd::rvv::vmv1r as _;
//! use rust_isa::riscv::simd::rvv::vmv2r as _;
//! use rust_isa::riscv::simd::rvv::vmv4r as _;
//! use rust_isa::riscv::simd::rvv::vmv8r as _;
//! use rust_isa::riscv::simd::rvv::vmv_s_x as _;
//! use rust_isa::riscv::simd::rvv::vmv_v_x as _;
//! use rust_isa::riscv::simd::rvv::vmv_x_s as _;
//! use rust_isa::riscv::simd::rvv::vmxnor as _;
//! use rust_isa::riscv::simd::rvv::vmxor as _;
//! use rust_isa::riscv::simd::rvv::vnclip as _;
//! use rust_isa::riscv::simd::rvv::vnclipu as _;
//! use rust_isa::riscv::simd::rvv::vnmsac as _;
//! use rust_isa::riscv::simd::rvv::vnmsub as _;
//! use rust_isa::riscv::simd::rvv::vnsra as _;
//! use rust_isa::riscv::simd::rvv::vnsrl as _;
//! use rust_isa::riscv::simd::rvv::vor as _;
//! use rust_isa::riscv::simd::rvv::vredand as _;
//! use rust_isa::riscv::simd::rvv::vredmax as _;
//! use rust_isa::riscv::simd::rvv::vredmaxu as _;
//! use rust_isa::riscv::simd::rvv::vredmin as _;
//! use rust_isa::riscv::simd::rvv::vredminu as _;
//! use rust_isa::riscv::simd::rvv::vredor as _;
//! use rust_isa::riscv::simd::rvv::vredsum as _;
//! use rust_isa::riscv::simd::rvv::vredxor as _;
//! use rust_isa::riscv::simd::rvv::vrem as _;
//! use rust_isa::riscv::simd::rvv::vremu as _;
//! use rust_isa::riscv::simd::rvv::vrgather as _;
//! use rust_isa::riscv::simd::rvv::vrgatherei16 as _;
//! use rust_isa::riscv::simd::rvv::vrsub_vi as _;
//! use rust_isa::riscv::simd::rvv::vrsub_vx as _;
//! use rust_isa::riscv::simd::rvv::vs1r as _;
//! use rust_isa::riscv::simd::rvv::vs2r as _;
//! use rust_isa::riscv::simd::rvv::vs4r as _;
//! use rust_isa::riscv::simd::rvv::vs8r as _;
//! use rust_isa::riscv::simd::rvv::vsadd as _;
//! use rust_isa::riscv::simd::rvv::vsaddu as _;
//! use rust_isa::riscv::simd::rvv::vsbc_vvm as _;
//! use rust_isa::riscv::simd::rvv::vsbc_vxm as _;
//! use rust_isa::riscv::simd::rvv::vse16 as _;
//! use rust_isa::riscv::simd::rvv::vse32 as _;
//! use rust_isa::riscv::simd::rvv::vse64 as _;
//! use rust_isa::riscv::simd::rvv::vse8 as _;
//! use rust_isa::riscv::simd::rvv::vsetivli as _;
//! use rust_isa::riscv::simd::rvv::vsetvl as _;
//! use rust_isa::riscv::simd::rvv::vsetvli as _;
//! use rust_isa::riscv::simd::rvv::vsext as _;
//! use rust_isa::riscv::simd::rvv::vslide1down as _;
//! use rust_isa::riscv::simd::rvv::vslide1up as _;
//! use rust_isa::riscv::simd::rvv::vslidedown as _;
//! use rust_isa::riscv::simd::rvv::vslideup as _;
//! use rust_isa::riscv::simd::rvv::vsll as _;
//! use rust_isa::riscv::simd::rvv::vsm as _;
//! use rust_isa::riscv::simd::rvv::vsmul_vv as _;
//! use rust_isa::riscv::simd::rvv::vsmul_vx as _;
//! use rust_isa::riscv::simd::rvv::vsoxei16 as _;
//! use rust_isa::riscv::simd::rvv::vsoxei32 as _;
//! use rust_isa::riscv::simd::rvv::vsoxei64 as _;
//! use rust_isa::riscv::simd::rvv::vsoxei8 as _;
//! use rust_isa::riscv::simd::rvv::vsra as _;
//! use rust_isa::riscv::simd::rvv::vsrl as _;
//! use rust_isa::riscv::simd::rvv::vsse16 as _;
//! use rust_isa::riscv::simd::rvv::vsse32 as _;
//! use rust_isa::riscv::simd::rvv::vsse64 as _;
//! use rust_isa::riscv::simd::rvv::vsse8 as _;
//! use rust_isa::riscv::simd::rvv::vsseg2 as _;
//! use rust_isa::riscv::simd::rvv::vsseg3 as _;
//! use rust_isa::riscv::simd::rvv::vsseg4 as _;
//! use rust_isa::riscv::simd::rvv::vsseg5 as _;
//! use rust_isa::riscv::simd::rvv::vsseg6 as _;
//! use rust_isa::riscv::simd::rvv::vsseg7 as _;
//! use rust_isa::riscv::simd::rvv::vsseg8 as _;
//! use rust_isa::riscv::simd::rvv::vssra_vi as _;
//! use rust_isa::riscv::simd::rvv::vssra_vv as _;
//! use rust_isa::riscv::simd::rvv::vssra_vx as _;
//! use rust_isa::riscv::simd::rvv::vssrl_vi as _;
//! use rust_isa::riscv::simd::rvv::vssrl_vv as _;
//! use rust_isa::riscv::simd::rvv::vssrl_vx as _;
//! use rust_isa::riscv::simd::rvv::vssseg2 as _;
//! use rust_isa::riscv::simd::rvv::vssseg3 as _;
//! use rust_isa::riscv::simd::rvv::vssseg4 as _;
//! use rust_isa::riscv::simd::rvv::vssseg5 as _;
//! use rust_isa::riscv::simd::rvv::vssseg6 as _;
//! use rust_isa::riscv::simd::rvv::vssseg7 as _;
//! use rust_isa::riscv::simd::rvv::vssseg8 as _;
//! use rust_isa::riscv::simd::rvv::vssub as _;
//! use rust_isa::riscv::simd::rvv::vssubu as _;
//! use rust_isa::riscv::simd::rvv::vsub as _;
//! use rust_isa::riscv::simd::rvv::vsuxei16 as _;
//! use rust_isa::riscv::simd::rvv::vsuxei32 as _;
//! use rust_isa::riscv::simd::rvv::vsuxei64 as _;
//! use rust_isa::riscv::simd::rvv::vsuxei8 as _;
//! use rust_isa::riscv::simd::rvv::vwadd as _;
//! use rust_isa::riscv::simd::rvv::vwaddu as _;
//! use rust_isa::riscv::simd::rvv::vwmacc as _;
//! use rust_isa::riscv::simd::rvv::vwmaccsu as _;
//! use rust_isa::riscv::simd::rvv::vwmaccu as _;
//! use rust_isa::riscv::simd::rvv::vwmaccus as _;
//! use rust_isa::riscv::simd::rvv::vwmul as _;
//! use rust_isa::riscv::simd::rvv::vwmulsu as _;
//! use rust_isa::riscv::simd::rvv::vwmulu as _;
//! use rust_isa::riscv::simd::rvv::vwredsum as _;
//! use rust_isa::riscv::simd::rvv::vwredsumu as _;
//! use rust_isa::riscv::simd::rvv::vwsub as _;
//! use rust_isa::riscv::simd::rvv::vwsubu as _;
//! use rust_isa::riscv::simd::rvv::vxor as _;
//! use rust_isa::riscv::simd::rvv::vzext as _;
//! use rust_isa::riscv::simd::rvv::xor as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

/// Whole-register helpers in this module assume a 128-bit vector length.
#[inline(always)]
unsafe fn assert_vlen_128() {
    let vector_bytes: usize;
    unsafe {
        core::arch::asm!("csrr {}, vlenb", out(reg) vector_bytes, options(nomem, nostack));
    }
    assert_eq!(vector_bytes, 16, "whole-register helpers require VLEN=128");
}

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_rvv_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadd.vv`
///
/// Add the four 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.vv`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rvv_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_mul_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `vadd.vv` on 8-bit lanes.
    add_i8,
    rust_isa_rvv_add_i8
);
binary!(
    /// `vadd.vv` on 16-bit lanes.
    add_i16,
    rust_isa_rvv_add_i16
);
binary!(
    /// `vadd.vv` on 64-bit lanes.
    add_i64,
    rust_isa_rvv_add_i64
);
binary!(
    /// `vsub.vv` on 8-bit lanes.
    sub_i8,
    rust_isa_rvv_sub_i8
);
binary!(
    /// `vsub.vv` on 16-bit lanes.
    sub_i16,
    rust_isa_rvv_sub_i16
);
binary!(
    /// `vsub.vv` on 32-bit lanes.
    sub_i32,
    rust_isa_rvv_sub_i32
);
binary!(
    /// `vsub.vv` on 64-bit lanes.
    sub_i64,
    rust_isa_rvv_sub_i64
);
binary!(
    /// `vmul.vv` on 8-bit lanes, keeping the low 8 bits.
    mul_i8,
    rust_isa_rvv_mul_i8
);
binary!(
    /// `vmul.vv` on 16-bit lanes, keeping the low 16 bits.
    mul_i16,
    rust_isa_rvv_mul_i16
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_rvv_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_rvv_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_rvv_xor
);
binary!(
    /// `vmseq.vv` on 32-bit lanes. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_rvv_cmpeq_i32
);
binary!(
    /// `vmsgt.vv` on signed 32-bit lanes. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_rvv_cmpgt_i32
);
binary!(
    /// `vfadd.vv` on 32-bit float lanes.
    fadd,
    rust_isa_rvv_fadd
);
binary!(
    /// `vfsub.vv` on 32-bit float lanes.
    fsub,
    rust_isa_rvv_fsub
);
binary!(
    /// `vfmul.vv` on 32-bit float lanes.
    fmul,
    rust_isa_rvv_fmul
);
binary!(
    /// `vfdiv.vv` on 32-bit float lanes.
    fdiv,
    rust_isa_rvv_fdiv
);

/// Broadcast a 32-bit value to all four lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_splat_i32(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rvv_vdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vdivu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrem(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vremu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsll(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsrl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsra(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vminu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmaxu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmseq_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsne_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmslt_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsle_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsgt_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsge_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsltu_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsleu_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsgtu_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsgeu_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmacc(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vnmsac(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmadd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vnmsub(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmv(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmv_x_s(a: *const u8) -> i32;
    fn rust_isa_rvv_vmv_s_x(value: i32, out: *mut u8);
}

/// Expand a four-lane mask into all-ones 32-bit lanes.
fn mask_to_v128_i32(mask: u16) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, chunk) in bytes.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        if mask & (1 << lane) != 0 {
            chunk.copy_from_slice(&(-1i32).to_le_bytes());
        }
    }
    V128::from_bytes(bytes)
}

/// `vadd.vv` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vadd(a: V128, b: V128) -> V128 {
    unsafe { add(a, b) }
}

/// `vsub.vv` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsub(a: V128, b: V128) -> V128 {
    unsafe { sub_i32(a, b) }
}

/// `vmul.vv` on 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmul(a: V128, b: V128) -> V128 {
    unsafe { mul(a, b) }
}

/// `vand.vv`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vand(a: V128, b: V128) -> V128 {
    unsafe { and(a, b) }
}

/// `vor.vv`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vor(a: V128, b: V128) -> V128 {
    unsafe { or(a, b) }
}

/// `vxor.vv`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vxor(a: V128, b: V128) -> V128 {
    unsafe { xor(a, b) }
}

binary!(
    /// `vdiv.vv` on 32-bit integer lanes.
    vdiv,
    rust_isa_rvv_vdiv
);
binary!(
    /// `vdivu.vv` on 32-bit unsigned integer lanes.
    vdivu,
    rust_isa_rvv_vdivu
);
binary!(
    /// `vrem.vv` on 32-bit integer lanes.
    vrem,
    rust_isa_rvv_vrem
);
binary!(
    /// `vremu.vv` on 32-bit unsigned integer lanes.
    vremu,
    rust_isa_rvv_vremu
);
binary!(
    /// `vsll.vv` on 32-bit integer lanes.
    vsll,
    rust_isa_rvv_vsll
);
binary!(
    /// `vsrl.vv` on 32-bit integer lanes.
    vsrl,
    rust_isa_rvv_vsrl
);
binary!(
    /// `vsra.vv` on 32-bit integer lanes.
    vsra,
    rust_isa_rvv_vsra
);
binary!(
    /// `vmin.vv` on signed 32-bit integer lanes.
    vmin,
    rust_isa_rvv_vmin
);
binary!(
    /// `vmax.vv` on signed 32-bit integer lanes.
    vmax,
    rust_isa_rvv_vmax
);
binary!(
    /// `vminu.vv` on unsigned 32-bit integer lanes.
    vminu,
    rust_isa_rvv_vminu
);
binary!(
    /// `vmaxu.vv` on unsigned 32-bit integer lanes.
    vmaxu,
    rust_isa_rvv_vmaxu
);

macro_rules! compare_mask {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// Returns a `u16` with bit `i` set when lane `i` compares true.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> u16 {
            unsafe { $c(a.as_ptr(), b.as_ptr()) }
        }
    };
}

compare_mask!(
    /// `vmseq.vv` on 32-bit integer lanes.
    vmseq_mask,
    rust_isa_rvv_vmseq_mask
);
compare_mask!(
    /// `vmsne.vv` on 32-bit integer lanes.
    vmsne_mask,
    rust_isa_rvv_vmsne_mask
);
compare_mask!(
    /// `vmslt.vv` on signed 32-bit integer lanes.
    vmslt_mask,
    rust_isa_rvv_vmslt_mask
);
compare_mask!(
    /// `vmsle.vv` on signed 32-bit integer lanes.
    vmsle_mask,
    rust_isa_rvv_vmsle_mask
);
compare_mask!(
    /// `vmsgt.vv` on signed 32-bit integer lanes.
    vmsgt_mask,
    rust_isa_rvv_vmsgt_mask
);
compare_mask!(
    /// `vmsge.vv` on signed 32-bit integer lanes.
    vmsge_mask,
    rust_isa_rvv_vmsge_mask
);
compare_mask!(
    /// `vmsltu.vv` on unsigned 32-bit integer lanes.
    vmsltu_mask,
    rust_isa_rvv_vmsltu_mask
);
compare_mask!(
    /// `vmsleu.vv` on unsigned 32-bit integer lanes.
    vmsleu_mask,
    rust_isa_rvv_vmsleu_mask
);
compare_mask!(
    /// `vmsgtu.vv` on unsigned 32-bit integer lanes.
    vmsgtu_mask,
    rust_isa_rvv_vmsgtu_mask
);
compare_mask!(
    /// `vmsgeu.vv` on unsigned 32-bit integer lanes.
    vmsgeu_mask,
    rust_isa_rvv_vmsgeu_mask
);

macro_rules! compare_v128 {
    ($(#[$meta:meta])* $name:ident, $mask:ident) => {
        $(#[$meta])*
        ///
        /// A true lane is all bits one.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            mask_to_v128_i32(unsafe { $mask(a, b) })
        }
    };
}

compare_v128!(
    /// `vmseq.vv` on 32-bit integer lanes.
    vmseq,
    vmseq_mask
);
compare_v128!(
    /// `vmsne.vv` on 32-bit integer lanes.
    vmsne,
    vmsne_mask
);
compare_v128!(
    /// `vmslt.vv` on signed 32-bit integer lanes.
    vmslt,
    vmslt_mask
);
compare_v128!(
    /// `vmsle.vv` on signed 32-bit integer lanes.
    vmsle,
    vmsle_mask
);
compare_v128!(
    /// `vmsgt.vv` on signed 32-bit integer lanes.
    vmsgt,
    vmsgt_mask
);
compare_v128!(
    /// `vmsge.vv` on signed 32-bit integer lanes.
    vmsge,
    vmsge_mask
);
compare_v128!(
    /// `vmsltu.vv` on unsigned 32-bit integer lanes.
    vmsltu,
    vmsltu_mask
);
compare_v128!(
    /// `vmsleu.vv` on unsigned 32-bit integer lanes.
    vmsleu,
    vmsleu_mask
);
compare_v128!(
    /// `vmsgtu.vv` on unsigned 32-bit integer lanes.
    vmsgtu,
    vmsgtu_mask
);
compare_v128!(
    /// `vmsgeu.vv` on unsigned 32-bit integer lanes.
    vmsgeu,
    vmsgeu_mask
);

macro_rules! ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(
                    acc.as_ptr(),
                    a.as_ptr(),
                    b.as_ptr(),
                    out.as_mut_ptr(),
                );
            }
            out
        }
    };
}

ternary!(
    /// `vmacc.vv`: `acc + a * b` on 32-bit integer lanes.
    vmacc,
    rust_isa_rvv_vmacc
);
ternary!(
    /// `vnmsac.vv`: `acc - a * b` on 32-bit integer lanes.
    vnmsac,
    rust_isa_rvv_vnmsac
);
ternary!(
    /// `vmadd.vv`: `a * acc + b` on 32-bit integer lanes.
    vmadd,
    rust_isa_rvv_vmadd
);
ternary!(
    /// `vnmsub.vv`: `b - a * acc` on 32-bit integer lanes.
    vnmsub,
    rust_isa_rvv_vnmsub
);

/// `vmv.v.v`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmv(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmv.v.x`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv_v_x(value: i32) -> V128 {
    unsafe { splat_i32(value) }
}

/// `vmv.x.s`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv_x_s(a: V128) -> i32 {
    unsafe { rust_isa_rvv_vmv_x_s(a.as_ptr()) }
}

/// `vmv.s.x`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv_s_x(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmv_s_x(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rvv_vsadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsaddu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssubu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vaadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vaaddu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vasub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vasubu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwmacc(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwmaccu(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwmaccsu(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwmaccus(acc: *const u8, scalar: u32, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vzext(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsext(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwaddu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwsubu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwmulu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vwmulsu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vnsrl(a: *const u8, shift: u32, out: *mut u8);
    fn rust_isa_rvv_vnsra(a: *const u8, shift: u32, out: *mut u8);
    fn rust_isa_rvv_vnclip(a: *const u8, shift: u32, out: *mut u8);
    fn rust_isa_rvv_vnclipu(a: *const u8, shift: u32, out: *mut u8);
}

binary!(
    /// `vsadd.vv` on signed 32-bit integer lanes. Saturates to `i32::MIN..=i32::MAX`.
    vsadd,
    rust_isa_rvv_vsadd
);
binary!(
    /// `vssub.vv` on signed 32-bit integer lanes. Saturates to `i32::MIN..=i32::MAX`.
    vssub,
    rust_isa_rvv_vssub
);
binary!(
    /// `vsaddu.vv` on unsigned 32-bit integer lanes. Saturates to `u32::MAX`.
    vsaddu,
    rust_isa_rvv_vsaddu
);
binary!(
    /// `vssubu.vv` on unsigned 32-bit integer lanes. Saturates to zero.
    vssubu,
    rust_isa_rvv_vssubu
);
binary!(
    /// `vaadd.vv` on signed 32-bit integer lanes.
    ///
    /// Rounds to nearest, ties away from zero (`vxrm = 0`).
    vaadd,
    rust_isa_rvv_vaadd
);
binary!(
    /// `vaaddu.vv` on unsigned 32-bit integer lanes.
    ///
    /// Rounds to nearest, ties away from zero (`vxrm = 0`).
    vaaddu,
    rust_isa_rvv_vaaddu
);
binary!(
    /// `vasub.vv` on signed 32-bit integer lanes.
    ///
    /// Rounds to nearest, ties away from zero (`vxrm = 0`).
    vasub,
    rust_isa_rvv_vasub
);
binary!(
    /// `vasubu.vv` on unsigned 32-bit integer lanes.
    ///
    /// Rounds to nearest, ties away from zero (`vxrm = 0`).
    vasubu,
    rust_isa_rvv_vasubu
);

ternary!(
    /// `vwmacc.vv`: `acc + sign_extend(a) * sign_extend(b)`.
    ///
    /// The low four 16-bit lanes of `a` and `b` are widened to 32 bits.
    vwmacc,
    rust_isa_rvv_vwmacc
);
ternary!(
    /// `vwmaccu.vv`: `acc + zero_extend(a) * zero_extend(b)`.
    ///
    /// The low four 16-bit lanes of `a` and `b` are widened to 32 bits.
    vwmaccu,
    rust_isa_rvv_vwmaccu
);
ternary!(
    /// `vwmaccsu.vv`: `acc + sign_extend(a) * zero_extend(b)`.
    ///
    /// The low four 16-bit lanes of `a` and `b` are widened to 32 bits.
    vwmaccsu,
    rust_isa_rvv_vwmaccsu
);

/// `vwmaccus.vx`: `acc + zero_extend(scalar) * sign_extend(b)`.
///
/// The low four 16-bit lanes of `b` are widened to 32 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vwmaccus(acc: V128, scalar: u32, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vwmaccus(acc.as_ptr(), scalar, b.as_ptr(), out.as_mut_ptr());
    }
    out
}

macro_rules! widen {
    ($(#[$meta:meta])* $name:ident, $c:ident, $doc:literal) => {
        $(#[$meta])*
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

widen!(
    /// `vzext.vf2`.
    vzext,
    rust_isa_rvv_vzext,
    "The low eight bytes are zero-extended to eight 16-bit lanes."
);
widen!(
    /// `vsext.vf2`.
    vsext,
    rust_isa_rvv_vsext,
    "The low eight bytes are sign-extended to eight 16-bit lanes."
);

binary!(
    /// `vwadd.vv`.
    ///
    /// The low four 16-bit lanes are sign-extended and added into 32-bit lanes.
    vwadd,
    rust_isa_rvv_vwadd
);
binary!(
    /// `vwaddu.vv`.
    ///
    /// The low four 16-bit lanes are zero-extended and added into 32-bit lanes.
    vwaddu,
    rust_isa_rvv_vwaddu
);
binary!(
    /// `vwsub.vv`.
    ///
    /// The low four 16-bit lanes are sign-extended and subtracted into 32-bit lanes.
    vwsub,
    rust_isa_rvv_vwsub
);
binary!(
    /// `vwsubu.vv`.
    ///
    /// The low four 16-bit lanes are zero-extended and subtracted into 32-bit lanes.
    vwsubu,
    rust_isa_rvv_vwsubu
);
binary!(
    /// `vwmul.vv`.
    ///
    /// The low four 16-bit lanes are sign-extended and multiplied into 32-bit lanes.
    vwmul,
    rust_isa_rvv_vwmul
);
binary!(
    /// `vwmulu.vv`.
    ///
    /// The low four 16-bit lanes are zero-extended and multiplied into 32-bit lanes.
    vwmulu,
    rust_isa_rvv_vwmulu
);
binary!(
    /// `vwmulsu.vv`.
    ///
    /// `a` is sign-extended and `b` is zero-extended before multiplying.
    vwmulsu,
    rust_isa_rvv_vwmulsu
);

macro_rules! narrow {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// The low four 32-bit lanes are narrowed to four 16-bit lanes. The
        /// remaining bytes are zero.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, shift: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), shift, out.as_mut_ptr());
            }
            out
        }
    };
}

narrow!(
    /// `vnsrl.wx`.
    vnsrl,
    rust_isa_rvv_vnsrl
);
narrow!(
    /// `vnsra.wx`.
    vnsra,
    rust_isa_rvv_vnsra
);
narrow!(
    /// `vnclip.wx`, rounding to nearest, ties away from zero (`vxrm = 0`).
    vnclip,
    rust_isa_rvv_vnclip
);
narrow!(
    /// `vnclipu.wx`, rounding to nearest, ties away from zero (`vxrm = 0`).
    vnclipu,
    rust_isa_rvv_vnclipu
);

unsafe extern "C" {
    fn rust_isa_rvv_vmerge(a: *const u8, b: *const u8, mask: u16, out: *mut u8);
    fn rust_isa_rvv_vslideup(dest: *const u8, src: *const u8, amount: u32, out: *mut u8);
    fn rust_isa_rvv_vslidedown(src: *const u8, amount: u32, out: *mut u8);
    fn rust_isa_rvv_vslide1up(src: *const u8, value: i32, out: *mut u8);
    fn rust_isa_rvv_vslide1down(src: *const u8, value: i32, out: *mut u8);
    fn rust_isa_rvv_vrgather(a: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrgatherei16(a: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vcompress(a: *const u8, mask: u16, out: *mut u8);
    fn rust_isa_rvv_vmand(a: u16, b: u16) -> u16;
    fn rust_isa_rvv_vmnand(a: u16, b: u16) -> u16;
    fn rust_isa_rvv_vmandn(a: u16, b: u16) -> u16;
    fn rust_isa_rvv_vmxor(a: u16, b: u16) -> u16;
    fn rust_isa_rvv_vmor(a: u16, b: u16) -> u16;
    fn rust_isa_rvv_vmnor(a: u16, b: u16) -> u16;
    fn rust_isa_rvv_vmorn(a: u16, b: u16) -> u16;
    fn rust_isa_rvv_vmxnor(a: u16, b: u16) -> u16;
    fn rust_isa_rvv_vcpop(mask: u16) -> u64;
    fn rust_isa_rvv_vfirst(mask: u16) -> i64;
    fn rust_isa_rvv_vmsbf(mask: u16) -> u16;
    fn rust_isa_rvv_vmsof(mask: u16) -> u16;
    fn rust_isa_rvv_vmsif(mask: u16) -> u16;
    fn rust_isa_rvv_viota(mask: u16, out: *mut u8);
    fn rust_isa_rvv_vid(out: *mut u8);
    fn rust_isa_rvv_vredsum(a: *const u8) -> u32;
    fn rust_isa_rvv_vredand(a: *const u8) -> i32;
    fn rust_isa_rvv_vredor(a: *const u8) -> i32;
    fn rust_isa_rvv_vredxor(a: *const u8) -> i32;
    fn rust_isa_rvv_vredmin(a: *const u8) -> i32;
    fn rust_isa_rvv_vredmax(a: *const u8) -> i32;
    fn rust_isa_rvv_vredminu(a: *const u8) -> u32;
    fn rust_isa_rvv_vredmaxu(a: *const u8) -> u32;
    fn rust_isa_rvv_vwredsum(a: *const u8) -> i64;
    fn rust_isa_rvv_vwredsumu(a: *const u8) -> u64;
}

/// `vmerge.vvm` on 32-bit integer lanes.
///
/// Lane `i` comes from `b` when bit `i` of `mask` is set, otherwise from `a`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmerge(a: V128, b: V128, mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmerge(a.as_ptr(), b.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

/// `vslideup.vx` on 32-bit integer lanes.
///
/// Slides `src` up by `amount`; the low lanes come from `dest`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vslideup(dest: V128, src: V128, amount: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vslideup(dest.as_ptr(), src.as_ptr(), amount, out.as_mut_ptr());
    }
    out
}

/// `vslidedown.vx` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vslidedown(src: V128, amount: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vslidedown(src.as_ptr(), amount, out.as_mut_ptr());
    }
    out
}

/// `vslide1up.vx` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vslide1up(src: V128, value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vslide1up(src.as_ptr(), value, out.as_mut_ptr());
    }
    out
}

/// `vslide1down.vx` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vslide1down(src: V128, value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vslide1down(src.as_ptr(), value, out.as_mut_ptr());
    }
    out
}

/// `vrgather.vv` on 32-bit integer lanes.
///
/// Out-of-range indices produce zero.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vrgather(a: V128, indices: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vrgather(a.as_ptr(), indices.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrgatherei16.vv` on 32-bit integer lanes.
///
/// The low four 16-bit lanes of `indices` select the source lanes.
/// Out-of-range indices produce zero.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vrgatherei16(a: V128, indices: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vrgatherei16(a.as_ptr(), indices.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcompress.vm` on 32-bit integer lanes.
///
/// Active lanes are packed to the low end; the remaining lanes are zero.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vcompress(a: V128, mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vcompress(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

macro_rules! mask_binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: u16, b: u16) -> u16 {
            unsafe { $c(a, b) }
        }
    };
}

mask_binary!(
    /// `vmand.mm`.
    vmand,
    rust_isa_rvv_vmand
);
mask_binary!(
    /// `vmnand.mm`.
    vmnand,
    rust_isa_rvv_vmnand
);
mask_binary!(
    /// `vmandn.mm`.
    vmandn,
    rust_isa_rvv_vmandn
);
mask_binary!(
    /// `vmxor.mm`.
    vmxor,
    rust_isa_rvv_vmxor
);
mask_binary!(
    /// `vmor.mm`.
    vmor,
    rust_isa_rvv_vmor
);
mask_binary!(
    /// `vmnor.mm`.
    vmnor,
    rust_isa_rvv_vmnor
);
mask_binary!(
    /// `vmorn.mm`.
    vmorn,
    rust_isa_rvv_vmorn
);
mask_binary!(
    /// `vmxnor.mm`.
    vmxnor,
    rust_isa_rvv_vmxnor
);

/// `vcpop.m`.
///
/// Returns the number of set bits in the 16-bit mask.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vcpop(mask: u16) -> u64 {
    unsafe { rust_isa_rvv_vcpop(mask) }
}

/// `vfirst.m`.
///
/// Returns the index of the first set bit, or `-1` when the mask is zero.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfirst(mask: u16) -> i64 {
    unsafe { rust_isa_rvv_vfirst(mask) }
}

macro_rules! mask_unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(mask: u16) -> u16 {
            unsafe { $c(mask) }
        }
    };
}

mask_unary!(
    /// `vmsbf.m`.
    vmsbf,
    rust_isa_rvv_vmsbf
);
mask_unary!(
    /// `vmsof.m`.
    vmsof,
    rust_isa_rvv_vmsof
);
mask_unary!(
    /// `vmsif.m`.
    vmsif,
    rust_isa_rvv_vmsif
);

/// `viota.m` on 8-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn viota(mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_viota(mask, out.as_mut_ptr());
    }
    out
}

/// `vid.v` on 8-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vid() -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vid(out.as_mut_ptr());
    }
    out
}

macro_rules! reduce {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> $ty {
            unsafe { $c(a.as_ptr()) }
        }
    };
}

reduce!(
    /// `vredsum.vs` on 32-bit integer lanes.
    vredsum,
    rust_isa_rvv_vredsum,
    u32
);
reduce!(
    /// `vredand.vs` on 32-bit integer lanes.
    vredand,
    rust_isa_rvv_vredand,
    i32
);
reduce!(
    /// `vredor.vs` on 32-bit integer lanes.
    vredor,
    rust_isa_rvv_vredor,
    i32
);
reduce!(
    /// `vredxor.vs` on 32-bit integer lanes.
    vredxor,
    rust_isa_rvv_vredxor,
    i32
);
reduce!(
    /// `vredmin.vs` on signed 32-bit integer lanes.
    vredmin,
    rust_isa_rvv_vredmin,
    i32
);
reduce!(
    /// `vredmax.vs` on signed 32-bit integer lanes.
    vredmax,
    rust_isa_rvv_vredmax,
    i32
);
reduce!(
    /// `vredminu.vs` on unsigned 32-bit integer lanes.
    vredminu,
    rust_isa_rvv_vredminu,
    u32
);
reduce!(
    /// `vredmaxu.vs` on unsigned 32-bit integer lanes.
    vredmaxu,
    rust_isa_rvv_vredmaxu,
    u32
);
reduce!(
    /// `vwredsum.vs` from signed 32-bit to 64-bit lanes.
    vwredsum,
    rust_isa_rvv_vwredsum,
    i64
);
reduce!(
    /// `vwredsumu.vs` from unsigned 32-bit to 64-bit lanes.
    vwredsumu,
    rust_isa_rvv_vwredsumu,
    u64
);

unsafe extern "C" {
    fn rust_isa_rvv_vle8(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vle16(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vle32(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vle64(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vle8ff(p: *const u8, out: *mut u8, vl_out: *mut usize);
    fn rust_isa_rvv_vle16ff(p: *const u8, out: *mut u8, vl_out: *mut usize);
    fn rust_isa_rvv_vle32ff(p: *const u8, out: *mut u8, vl_out: *mut usize);
    fn rust_isa_rvv_vle64ff(p: *const u8, out: *mut u8, vl_out: *mut usize);
    fn rust_isa_rvv_vse8(p: *mut u8, value: *const u8);
    fn rust_isa_rvv_vse16(p: *mut u8, value: *const u8);
    fn rust_isa_rvv_vse32(p: *mut u8, value: *const u8);
    fn rust_isa_rvv_vse64(p: *mut u8, value: *const u8);
    fn rust_isa_rvv_vlse8(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vlse16(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vlse32(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vlse64(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vsse8(p: *mut u8, stride: isize, value: *const u8);
    fn rust_isa_rvv_vsse16(p: *mut u8, stride: isize, value: *const u8);
    fn rust_isa_rvv_vsse32(p: *mut u8, stride: isize, value: *const u8);
    fn rust_isa_rvv_vsse64(p: *mut u8, stride: isize, value: *const u8);
    fn rust_isa_rvv_vloxei8(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vloxei16(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vloxei32(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vloxei64(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vluxei8(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vluxei16(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vluxei32(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vluxei64(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsoxei8(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rvv_vsoxei16(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rvv_vsoxei32(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rvv_vsoxei64(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rvv_vsuxei8(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rvv_vsuxei16(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rvv_vsuxei32(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rvv_vsuxei64(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rvv_vlm(p: *const u8) -> u16;
    fn rust_isa_rvv_vsm(p: *mut u8, mask: u16);
}

macro_rules! load {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled. `p` must be
        /// valid for reading 16 bytes.
        #[inline(always)]
        pub unsafe fn $name(p: *const $ty) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(p.cast(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! store {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled. `p` must be
        /// valid for writing 16 bytes.
        #[inline(always)]
        pub unsafe fn $name(p: *mut $ty, value: V128) {
            unsafe {
                $c(p.cast(), value.as_ptr());
            }
        }
    };
}

load!(
    /// `vle8.v`.
    vle8,
    rust_isa_rvv_vle8,
    u8
);
load!(
    /// `vle16.v`.
    vle16,
    rust_isa_rvv_vle16,
    u16
);
load!(
    /// `vle32.v`.
    vle32,
    rust_isa_rvv_vle32,
    i32
);
load!(
    /// `vle64.v`.
    vle64,
    rust_isa_rvv_vle64,
    u64
);
store!(
    /// `vse8.v`.
    vse8,
    rust_isa_rvv_vse8,
    u8
);
store!(
    /// `vse16.v`.
    vse16,
    rust_isa_rvv_vse16,
    u16
);
store!(
    /// `vse32.v`.
    vse32,
    rust_isa_rvv_vse32,
    i32
);
store!(
    /// `vse64.v`.
    vse64,
    rust_isa_rvv_vse64,
    u64
);

macro_rules! strided_load {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// `stride` is in bytes.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled. `p + i * stride`
        /// must be valid for reading one element for each loaded lane.
        #[inline(always)]
        pub unsafe fn $name(p: *const $ty, stride: isize) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(p.cast(), stride, out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! strided_store {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// `stride` is in bytes.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled. `p + i * stride`
        /// must be valid for writing one element for each stored lane.
        #[inline(always)]
        pub unsafe fn $name(p: *mut $ty, stride: isize, value: V128) {
            unsafe {
                $c(p.cast(), stride, value.as_ptr());
            }
        }
    };
}

strided_load!(
    /// `vlse8.v`.
    vlse8,
    rust_isa_rvv_vlse8,
    u8
);
strided_load!(
    /// `vlse16.v`.
    vlse16,
    rust_isa_rvv_vlse16,
    u16
);
strided_load!(
    /// `vlse32.v`.
    vlse32,
    rust_isa_rvv_vlse32,
    i32
);
strided_load!(
    /// `vlse64.v`.
    vlse64,
    rust_isa_rvv_vlse64,
    u64
);
strided_store!(
    /// `vsse8.v`.
    vsse8,
    rust_isa_rvv_vsse8,
    u8
);
strided_store!(
    /// `vsse16.v`.
    vsse16,
    rust_isa_rvv_vsse16,
    u16
);
strided_store!(
    /// `vsse32.v`.
    vsse32,
    rust_isa_rvv_vsse32,
    i32
);
strided_store!(
    /// `vsse64.v`.
    vsse64,
    rust_isa_rvv_vsse64,
    u64
);

macro_rules! indexed_load {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// The low bytes of `indices` hold byte offsets for the 32-bit data
        /// elements.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and the base
        /// pointer must be valid for every selected element.
        #[inline(always)]
        pub unsafe fn $name(base: *const i32, indices: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(base.cast(), indices.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! indexed_store {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// The low bytes of `indices` hold byte offsets for the 32-bit data
        /// elements.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and the base
        /// pointer must be valid for every selected element.
        #[inline(always)]
        pub unsafe fn $name(base: *mut i32, indices: V128, value: V128) {
            unsafe {
                $c(base.cast(), indices.as_ptr(), value.as_ptr());
            }
        }
    };
}

indexed_load!(
    /// `vloxei8.v`.
    vloxei8,
    rust_isa_rvv_vloxei8
);
indexed_load!(
    /// `vloxei16.v`.
    vloxei16,
    rust_isa_rvv_vloxei16
);
indexed_load!(
    /// `vloxei32.v`.
    vloxei32,
    rust_isa_rvv_vloxei32
);
indexed_load!(
    /// `vloxei64.v`. Only the first two lanes are written.
    vloxei64,
    rust_isa_rvv_vloxei64
);
indexed_load!(
    /// `vluxei8.v`.
    vluxei8,
    rust_isa_rvv_vluxei8
);
indexed_load!(
    /// `vluxei16.v`.
    vluxei16,
    rust_isa_rvv_vluxei16
);
indexed_load!(
    /// `vluxei32.v`.
    vluxei32,
    rust_isa_rvv_vluxei32
);
indexed_load!(
    /// `vluxei64.v`. Only the first two lanes are written.
    vluxei64,
    rust_isa_rvv_vluxei64
);
indexed_store!(
    /// `vsoxei8.v`.
    vsoxei8,
    rust_isa_rvv_vsoxei8
);
indexed_store!(
    /// `vsoxei16.v`.
    vsoxei16,
    rust_isa_rvv_vsoxei16
);
indexed_store!(
    /// `vsoxei32.v`.
    vsoxei32,
    rust_isa_rvv_vsoxei32
);
indexed_store!(
    /// `vsoxei64.v`. Only the first two lanes are written.
    vsoxei64,
    rust_isa_rvv_vsoxei64
);
indexed_store!(
    /// `vsuxei8.v`.
    vsuxei8,
    rust_isa_rvv_vsuxei8
);
indexed_store!(
    /// `vsuxei16.v`.
    vsuxei16,
    rust_isa_rvv_vsuxei16
);
indexed_store!(
    /// `vsuxei32.v`.
    vsuxei32,
    rust_isa_rvv_vsuxei32
);
indexed_store!(
    /// `vsuxei64.v`. Only the first two lanes are written.
    vsuxei64,
    rust_isa_rvv_vsuxei64
);

unsafe extern "C" {
    fn rust_isa_rvv_vlseg2(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsseg2(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vlsseg2(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vssseg2(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rvv_vlseg3(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsseg3(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vlsseg3(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vssseg3(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rvv_vlseg4(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsseg4(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vlsseg4(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vssseg4(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rvv_vlseg5(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsseg5(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vlsseg5(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vssseg5(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rvv_vlseg6(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsseg6(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vlsseg6(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vssseg6(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rvv_vlseg7(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsseg7(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vlsseg7(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vssseg7(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rvv_vlseg8(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsseg8(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vlsseg8(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rvv_vssseg8(p: *mut u8, stride: isize, values: *const u8);
}

macro_rules! segment {
    ($(#[$meta:meta])* $load:ident, $store:ident, $sload:ident, $sstore:ident,
     $cload:ident, $cstore:ident, $csload:ident, $csstore:ident, $n:expr) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and the base
        /// pointer must be valid for the requested number of interleaved
        /// elements.
        #[inline(always)]
        pub unsafe fn $load(p: *const i32) -> [V128; $n] {
            let mut out = [V128::from_bytes([0; 16]); $n];
            unsafe {
                $cload(p.cast(), out.as_mut_ptr().cast());
            }
            out
        }

        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and the base
        /// pointer must be valid for the requested number of interleaved
        /// elements.
        #[inline(always)]
        pub unsafe fn $store(p: *mut i32, values: [V128; $n]) {
            unsafe {
                $cstore(p.cast(), values.as_ptr().cast());
            }
        }

        $(#[$meta])*
        ///
        /// `stride` is in bytes.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and every
        /// selected element must be valid.
        #[inline(always)]
        pub unsafe fn $sload(p: *const i32, stride: isize) -> [V128; $n] {
            let mut out = [V128::from_bytes([0; 16]); $n];
            unsafe {
                $csload(p.cast(), stride, out.as_mut_ptr().cast());
            }
            out
        }

        $(#[$meta])*
        ///
        /// `stride` is in bytes.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and every
        /// selected element must be valid.
        #[inline(always)]
        pub unsafe fn $sstore(p: *mut i32, stride: isize, values: [V128; $n]) {
            unsafe {
                $csstore(p.cast(), stride, values.as_ptr().cast());
            }
        }
    };
}

segment!(
    /// `vlseg2e32.v`, `vsseg2e32.v`, `vlsseg2e32.v`, `vssseg2e32.v`.
    vlseg2,
    vsseg2,
    vlsseg2,
    vssseg2,
    rust_isa_rvv_vlseg2,
    rust_isa_rvv_vsseg2,
    rust_isa_rvv_vlsseg2,
    rust_isa_rvv_vssseg2,
    2
);
segment!(
    /// `vlseg3e32.v`, `vsseg3e32.v`, `vlsseg3e32.v`, `vssseg3e32.v`.
    vlseg3,
    vsseg3,
    vlsseg3,
    vssseg3,
    rust_isa_rvv_vlseg3,
    rust_isa_rvv_vsseg3,
    rust_isa_rvv_vlsseg3,
    rust_isa_rvv_vssseg3,
    3
);
segment!(
    /// `vlseg4e32.v`, `vsseg4e32.v`, `vlsseg4e32.v`, `vssseg4e32.v`.
    vlseg4,
    vsseg4,
    vlsseg4,
    vssseg4,
    rust_isa_rvv_vlseg4,
    rust_isa_rvv_vsseg4,
    rust_isa_rvv_vlsseg4,
    rust_isa_rvv_vssseg4,
    4
);
segment!(
    /// `vlseg5e32.v`, `vsseg5e32.v`, `vlsseg5e32.v`, `vssseg5e32.v`.
    vlseg5,
    vsseg5,
    vlsseg5,
    vssseg5,
    rust_isa_rvv_vlseg5,
    rust_isa_rvv_vsseg5,
    rust_isa_rvv_vlsseg5,
    rust_isa_rvv_vssseg5,
    5
);
segment!(
    /// `vlseg6e32.v`, `vsseg6e32.v`, `vlsseg6e32.v`, `vssseg6e32.v`.
    vlseg6,
    vsseg6,
    vlsseg6,
    vssseg6,
    rust_isa_rvv_vlseg6,
    rust_isa_rvv_vsseg6,
    rust_isa_rvv_vlsseg6,
    rust_isa_rvv_vssseg6,
    6
);
segment!(
    /// `vlseg7e32.v`, `vsseg7e32.v`, `vlsseg7e32.v`, `vssseg7e32.v`.
    vlseg7,
    vsseg7,
    vlsseg7,
    vssseg7,
    rust_isa_rvv_vlseg7,
    rust_isa_rvv_vsseg7,
    rust_isa_rvv_vlsseg7,
    rust_isa_rvv_vssseg7,
    7
);
segment!(
    /// `vlseg8e32.v`, `vsseg8e32.v`, `vlsseg8e32.v`, `vssseg8e32.v`.
    vlseg8,
    vsseg8,
    vlsseg8,
    vssseg8,
    rust_isa_rvv_vlseg8,
    rust_isa_rvv_vsseg8,
    rust_isa_rvv_vlsseg8,
    rust_isa_rvv_vssseg8,
    8
);

/// `vlm.v`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled. `p` must be valid for
/// reading two bytes.
#[inline(always)]
pub unsafe fn vlm(p: *const u8) -> u16 {
    unsafe { rust_isa_rvv_vlm(p) }
}

/// `vsm.v`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled. `p` must be valid for
/// writing two bytes.
#[inline(always)]
pub unsafe fn vsm(p: *mut u8, mask: u16) {
    unsafe {
        rust_isa_rvv_vsm(p, mask);
    }
}

unsafe extern "C" {
    fn rust_isa_rvv_vfrdiv(a: *const u8, scalar: f32, out: *mut u8);
    fn rust_isa_rvv_vfmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfsgnj(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfsgnjn(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfsgnjx(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfabs(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfneg(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfsqrt(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfrsqrt7(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfrec7(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfmerge(a: *const u8, scalar: f32, mask: u16, out: *mut u8);
    fn rust_isa_rvv_vfmv(scalar: f32, out: *mut u8);
    fn rust_isa_rvv_vfmv_f_s(a: *const u8) -> f32;
    fn rust_isa_rvv_vfma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfms(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfnma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfnms(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfredsum(a: *const u8) -> f32;
    fn rust_isa_rvv_vfredosum(a: *const u8) -> f32;
    fn rust_isa_rvv_vfredusum(a: *const u8) -> f32;
    fn rust_isa_rvv_vfredmax(a: *const u8) -> f32;
    fn rust_isa_rvv_vfredmin(a: *const u8) -> f32;
    fn rust_isa_rvv_vfwadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwmacc(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwnmacc(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwmsac(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwnmsac(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfclass(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfcvt_xu_f(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfcvt_x_f(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfcvt_rtz_xu_f(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfcvt_rtz_x_f(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfcvt_f_xu(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfcvt_f_x(a: *const u8, out: *mut u8);
}

/// `vfadd.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfadd(a: V128, b: V128) -> V128 {
    unsafe { fadd(a, b) }
}

/// `vfsub.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfsub(a: V128, b: V128) -> V128 {
    unsafe { fsub(a, b) }
}

/// `vfmul.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmul(a: V128, b: V128) -> V128 {
    unsafe { fmul(a, b) }
}

/// `vfdiv.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfdiv(a: V128, b: V128) -> V128 {
    unsafe { fdiv(a, b) }
}

/// `vfrdiv.vf` on 32-bit float lanes.
///
/// Computes `scalar / a`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfrdiv(a: V128, scalar: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfrdiv(a.as_ptr(), scalar, out.as_mut_ptr());
    }
    out
}

binary!(
    /// `vfmin.vv` on 32-bit float lanes.
    vfmin,
    rust_isa_rvv_vfmin
);
binary!(
    /// `vfmax.vv` on 32-bit float lanes.
    vfmax,
    rust_isa_rvv_vfmax
);
binary!(
    /// `vfsgnj.vv` on 32-bit float lanes.
    vfsgnj,
    rust_isa_rvv_vfsgnj
);
binary!(
    /// `vfsgnjn.vv` on 32-bit float lanes.
    vfsgnjn,
    rust_isa_rvv_vfsgnjn
);
binary!(
    /// `vfsgnjx.vv` on 32-bit float lanes.
    vfsgnjx,
    rust_isa_rvv_vfsgnjx
);

macro_rules! float_unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

float_unary!(
    /// `vfabs.v`.
    vfabs,
    rust_isa_rvv_vfabs
);
float_unary!(
    /// `vfneg.v`.
    vfneg,
    rust_isa_rvv_vfneg
);
float_unary!(
    /// `vfsqrt.v`.
    vfsqrt,
    rust_isa_rvv_vfsqrt
);
float_unary!(
    /// `vfrsqrt7.v`.
    vfrsqrt7,
    rust_isa_rvv_vfrsqrt7
);
float_unary!(
    /// `vfrec7.v`.
    vfrec7,
    rust_isa_rvv_vfrec7
);

/// `vfmerge.vfm` on 32-bit float lanes.
///
/// Lane `i` becomes `scalar` when bit `i` of `mask` is set.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmerge(a: V128, scalar: f32, mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfmerge(a.as_ptr(), scalar, mask, out.as_mut_ptr());
    }
    out
}

/// `vfmv.v.f`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmv(scalar: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfmv(scalar, out.as_mut_ptr());
    }
    out
}

/// `vfmv.f.s`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmv_f_s(a: V128) -> f32 {
    unsafe { rust_isa_rvv_vfmv_f_s(a.as_ptr()) }
}

ternary!(
    /// `vfmadd.vv`: `acc * a + b`.
    vfma,
    rust_isa_rvv_vfma
);
ternary!(
    /// `vfmsub.vv`: `acc * a - b`.
    vfms,
    rust_isa_rvv_vfms
);
ternary!(
    /// `vfnmadd.vv`: `-(acc * a) - b`.
    vfnma,
    rust_isa_rvv_vfnma
);
ternary!(
    /// `vfnmsub.vv`: `-(acc * a) + b`.
    vfnms,
    rust_isa_rvv_vfnms
);

macro_rules! float_reduce {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> f32 {
            unsafe { $c(a.as_ptr()) }
        }
    };
}

float_reduce!(
    /// `vfredsum.vs`, the unordered reduction.
    vfredsum,
    rust_isa_rvv_vfredsum
);
float_reduce!(
    /// `vfredusum.vs`, the unordered reduction.
    vfredusum,
    rust_isa_rvv_vfredusum
);
float_reduce!(
    /// `vfredosum.vs`, the ordered reduction.
    vfredosum,
    rust_isa_rvv_vfredosum
);
float_reduce!(
    /// `vfredmax.vs`.
    vfredmax,
    rust_isa_rvv_vfredmax
);
float_reduce!(
    /// `vfredmin.vs`.
    vfredmin,
    rust_isa_rvv_vfredmin
);

binary!(
    /// `vfwadd.vv`, widening the low two 32-bit lanes to 64-bit lanes.
    vfwadd,
    rust_isa_rvv_vfwadd
);
binary!(
    /// `vfwsub.vv`, widening the low two 32-bit lanes to 64-bit lanes.
    vfwsub,
    rust_isa_rvv_vfwsub
);
binary!(
    /// `vfwmul.vv`, widening the low two 32-bit lanes to 64-bit lanes.
    vfwmul,
    rust_isa_rvv_vfwmul
);
ternary!(
    /// `vfwmacc.vv`: `acc + a * b` on two 64-bit accumulator lanes.
    vfwmacc,
    rust_isa_rvv_vfwmacc
);
ternary!(
    /// `vfwnmacc.vv`: `-(a * b) - acc` on two 64-bit accumulator lanes.
    vfwnmacc,
    rust_isa_rvv_vfwnmacc
);
ternary!(
    /// `vfwmsac.vv`: `a * b - acc` on two 64-bit accumulator lanes.
    vfwmsac,
    rust_isa_rvv_vfwmsac
);
ternary!(
    /// `vfwnmsac.vv`: `-(a * b) + acc` on two 64-bit accumulator lanes.
    vfwnmsac,
    rust_isa_rvv_vfwnmsac
);

float_unary!(
    /// `vfclass.v`, producing a 10-bit class mask in each 32-bit lane.
    vfclass,
    rust_isa_rvv_vfclass
);

float_unary!(
    /// `vfcvt.xu.f.v` on 32-bit lanes.
    vfcvt_xu_f,
    rust_isa_rvv_vfcvt_xu_f
);
float_unary!(
    /// `vfcvt.x.f.v` on 32-bit lanes.
    vfcvt_x_f,
    rust_isa_rvv_vfcvt_x_f
);
float_unary!(
    /// `vfcvt.rtz.xu.f.v` on 32-bit lanes.
    vfcvt_rtz_xu_f,
    rust_isa_rvv_vfcvt_rtz_xu_f
);
float_unary!(
    /// `vfcvt.rtz.x.f.v` on 32-bit lanes.
    vfcvt_rtz_x_f,
    rust_isa_rvv_vfcvt_rtz_x_f
);
float_unary!(
    /// `vfcvt.f.xu.v` on 32-bit lanes.
    vfcvt_f_xu,
    rust_isa_rvv_vfcvt_f_xu
);
float_unary!(
    /// `vfcvt.f.x.v` on 32-bit lanes.
    vfcvt_f_x,
    rust_isa_rvv_vfcvt_f_x
);

/// `vsetivli` sets the vector length from an immediate AVL and a vtype immediate.
///
/// `AVL` must be in `0..=31`; `VTYPE` must fit in 11 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsetivli<const AVL: u64, const VTYPE: u64>() -> usize {
    const {
        assert!(AVL <= 31, "vsetivli AVL must be in 0..=31");
        assert!(VTYPE < (1 << 11), "vsetivli VTYPE must fit in 11 bits");
    }
    let result: usize;
    unsafe {
        core::arch::asm!(
            "vsetivli {result}, {avl}, {vtype}",
            result = out(reg) result,
            avl = const AVL,
            vtype = const VTYPE,
            options(nostack),
        );
    }
    result
}

/// `vsetvli` sets the vector length from a register AVL and a vtype immediate.
///
/// `VTYPE` must fit in 11 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsetvli<const VTYPE: u64>(avl: usize) -> usize {
    const { assert!(VTYPE < (1 << 11), "vsetvli VTYPE must fit in 11 bits") };
    let result: usize;
    unsafe {
        core::arch::asm!(
            "vsetvli {result}, {avl}, {vtype}",
            result = out(reg) result,
            avl = in(reg) avl,
            vtype = const VTYPE,
            options(nostack),
        );
    }
    result
}

/// `vsetvl` sets the vector length from register AVL and vtype values.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsetvl(avl: usize, vtype: usize) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "vsetvl {result}, {avl}, {vtype}",
            result = out(reg) result,
            avl = in(reg) avl,
            vtype = in(reg) vtype,
            options(nostack),
        );
    }
    result
}

/// `vle8ff.v` fault-only-first load.
///
/// Returns the vector and the number of elements loaded before the first
/// fault. `p` must be valid for reading up to 16 bytes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vle8ff(p: *const u8) -> (V128, usize) {
    let mut out = V128::from_bytes([0; 16]);
    let mut vl = 0usize;
    unsafe {
        rust_isa_rvv_vle8ff(p, out.as_mut_ptr(), &mut vl);
    }
    (out, vl)
}

/// `vle16ff.v` fault-only-first load.
///
/// Returns the vector and the number of elements loaded before the first
/// fault. `p` must be valid for reading up to 16 bytes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vle16ff(p: *const u16) -> (V128, usize) {
    let mut out = V128::from_bytes([0; 16]);
    let mut vl = 0usize;
    unsafe {
        rust_isa_rvv_vle16ff(p.cast(), out.as_mut_ptr(), &mut vl);
    }
    (out, vl)
}

/// `vle32ff.v` fault-only-first load.
///
/// Returns the vector and the number of elements loaded before the first
/// fault. `p` must be valid for reading up to 16 bytes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vle32ff(p: *const i32) -> (V128, usize) {
    let mut out = V128::from_bytes([0; 16]);
    let mut vl = 0usize;
    unsafe {
        rust_isa_rvv_vle32ff(p.cast(), out.as_mut_ptr(), &mut vl);
    }
    (out, vl)
}

/// `vle64ff.v` fault-only-first load.
///
/// Returns the vector and the number of elements loaded before the first
/// fault. `p` must be valid for reading up to 16 bytes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vle64ff(p: *const u64) -> (V128, usize) {
    let mut out = V128::from_bytes([0; 16]);
    let mut vl = 0usize;
    unsafe {
        rust_isa_rvv_vle64ff(p.cast(), out.as_mut_ptr(), &mut vl);
    }
    (out, vl)
}

unsafe extern "C" {
    fn rust_isa_rvv_vl1re8(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl2re8(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl4re8(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl8re8(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl1re16(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl2re16(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl4re16(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl8re16(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl1re32(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl2re32(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl4re32(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl8re32(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl1re64(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl2re64(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl4re64(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vl8re64(p: *const u8, out: *mut u8);
    fn rust_isa_rvv_vs1r(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vs2r(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vs4r(p: *mut u8, values: *const u8);
    fn rust_isa_rvv_vs8r(p: *mut u8, values: *const u8);
}

/// `vl1re8.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 16 bytes.
#[inline(always)]
pub unsafe fn vl1re8(p: *const u8) -> V128 {
    assert_vlen_128();
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vl1re8(p, out.as_mut_ptr());
    }
    out
}

/// `vl2re8.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 32 bytes.
#[inline(always)]
pub unsafe fn vl2re8(p: *const u8) -> [V128; 2] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vl2re8(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl4re8.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 64 bytes.
#[inline(always)]
pub unsafe fn vl4re8(p: *const u8) -> [V128; 4] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 4];
    unsafe {
        rust_isa_rvv_vl4re8(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl8re8.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 128 bytes.
#[inline(always)]
pub unsafe fn vl8re8(p: *const u8) -> [V128; 8] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 8];
    unsafe {
        rust_isa_rvv_vl8re8(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl1re16.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 16 bytes.
#[inline(always)]
pub unsafe fn vl1re16(p: *const u8) -> V128 {
    assert_vlen_128();
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vl1re16(p, out.as_mut_ptr());
    }
    out
}

/// `vl2re16.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 32 bytes.
#[inline(always)]
pub unsafe fn vl2re16(p: *const u8) -> [V128; 2] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vl2re16(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl4re16.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 64 bytes.
#[inline(always)]
pub unsafe fn vl4re16(p: *const u8) -> [V128; 4] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 4];
    unsafe {
        rust_isa_rvv_vl4re16(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl8re16.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 128 bytes.
#[inline(always)]
pub unsafe fn vl8re16(p: *const u8) -> [V128; 8] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 8];
    unsafe {
        rust_isa_rvv_vl8re16(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl1re32.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 16 bytes.
#[inline(always)]
pub unsafe fn vl1re32(p: *const u8) -> V128 {
    assert_vlen_128();
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vl1re32(p, out.as_mut_ptr());
    }
    out
}

/// `vl2re32.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 32 bytes.
#[inline(always)]
pub unsafe fn vl2re32(p: *const u8) -> [V128; 2] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vl2re32(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl4re32.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 64 bytes.
#[inline(always)]
pub unsafe fn vl4re32(p: *const u8) -> [V128; 4] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 4];
    unsafe {
        rust_isa_rvv_vl4re32(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl8re32.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 128 bytes.
#[inline(always)]
pub unsafe fn vl8re32(p: *const u8) -> [V128; 8] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 8];
    unsafe {
        rust_isa_rvv_vl8re32(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl1re64.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 16 bytes.
#[inline(always)]
pub unsafe fn vl1re64(p: *const u8) -> V128 {
    assert_vlen_128();
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vl1re64(p, out.as_mut_ptr());
    }
    out
}

/// `vl2re64.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 32 bytes.
#[inline(always)]
pub unsafe fn vl2re64(p: *const u8) -> [V128; 2] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vl2re64(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl4re64.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 64 bytes.
#[inline(always)]
pub unsafe fn vl4re64(p: *const u8) -> [V128; 4] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 4];
    unsafe {
        rust_isa_rvv_vl4re64(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vl8re64.v` whole-register load.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for reading 128 bytes.
#[inline(always)]
pub unsafe fn vl8re64(p: *const u8) -> [V128; 8] {
    assert_vlen_128();
    let mut out = [V128::from_bytes([0; 16]); 8];
    unsafe {
        rust_isa_rvv_vl8re64(p, out.as_mut_ptr().cast());
    }
    out
}

/// `vs1r.v` whole-register store.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for writing 16 bytes.
#[inline(always)]
pub unsafe fn vs1r(p: *mut u8, values: V128) {
    assert_vlen_128();
    unsafe {
        rust_isa_rvv_vs1r(p, values.as_ptr().cast());
    }
}

/// `vs2r.v` whole-register store.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for writing 32 bytes.
#[inline(always)]
pub unsafe fn vs2r(p: *mut u8, values: [V128; 2]) {
    assert_vlen_128();
    unsafe {
        rust_isa_rvv_vs2r(p, values.as_ptr().cast());
    }
}

/// `vs4r.v` whole-register store.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for writing 64 bytes.
#[inline(always)]
pub unsafe fn vs4r(p: *mut u8, values: [V128; 4]) {
    assert_vlen_128();
    unsafe {
        rust_isa_rvv_vs4r(p, values.as_ptr().cast());
    }
}

/// `vs8r.v` whole-register store.
///
/// # Safety
///
/// The caller must compile with the V extension enabled and `p` must be valid
/// for writing 128 bytes.
#[inline(always)]
pub unsafe fn vs8r(p: *mut u8, values: [V128; 8]) {
    assert_vlen_128();
    unsafe {
        rust_isa_rvv_vs8r(p, values.as_ptr().cast());
    }
}

unsafe extern "C" {
    fn rust_isa_rvv_vfslide1up(a: *const u8, value: f32, out: *mut u8);
    fn rust_isa_rvv_vfslide1down(a: *const u8, value: f32, out: *mut u8);
    fn rust_isa_rvv_vmfeq_vv(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmfeq_vf(a: *const u8, b: f32) -> u16;
    fn rust_isa_rvv_vmfle_vv(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmfle_vf(a: *const u8, b: f32) -> u16;
    fn rust_isa_rvv_vmflt_vv(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmflt_vf(a: *const u8, b: f32) -> u16;
    fn rust_isa_rvv_vmfne_vv(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmfne_vf(a: *const u8, b: f32) -> u16;
    fn rust_isa_rvv_vmfgt_vv(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmfgt_vf(a: *const u8, b: f32) -> u16;
    fn rust_isa_rvv_vmfge_vv(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmfge_vf(a: *const u8, b: f32) -> u16;
}

/// `vfslide1up.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfslide1up(a: V128, value: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfslide1up(a.as_ptr(), value, out.as_mut_ptr());
    }
    out
}

/// `vfslide1down.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfslide1down(a: V128, value: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfslide1down(a.as_ptr(), value, out.as_mut_ptr());
    }
    out
}

/// `vmfeq.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfeq_vv(a: V128, b: V128) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfeq_vv(a.as_ptr(), b.as_ptr()) })
}

/// `vmfeq.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfeq_vf(a: V128, b: f32) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfeq_vf(a.as_ptr(), b) })
}

/// `vmfle.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfle_vv(a: V128, b: V128) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfle_vv(a.as_ptr(), b.as_ptr()) })
}

/// `vmfle.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfle_vf(a: V128, b: f32) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfle_vf(a.as_ptr(), b) })
}

/// `vmflt.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmflt_vv(a: V128, b: V128) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmflt_vv(a.as_ptr(), b.as_ptr()) })
}

/// `vmflt.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmflt_vf(a: V128, b: f32) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmflt_vf(a.as_ptr(), b) })
}

/// `vmfne.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfne_vv(a: V128, b: V128) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfne_vv(a.as_ptr(), b.as_ptr()) })
}

/// `vmfne.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfne_vf(a: V128, b: f32) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfne_vf(a.as_ptr(), b) })
}

/// `vmfgt.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfgt_vv(a: V128, b: V128) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfgt_vv(a.as_ptr(), b.as_ptr()) })
}

/// `vmfgt.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfgt_vf(a: V128, b: f32) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfgt_vf(a.as_ptr(), b) })
}

/// `vmfge.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfge_vv(a: V128, b: V128) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfge_vv(a.as_ptr(), b.as_ptr()) })
}

/// `vmfge.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmfge_vf(a: V128, b: f32) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmfge_vf(a.as_ptr(), b) })
}

unsafe extern "C" {
    fn rust_isa_rvv_vfmadd_vf(acc: *const u8, a: *const u8, b: f32, out: *mut u8);
    fn rust_isa_rvv_vfmsub_vf(acc: *const u8, a: *const u8, b: f32, out: *mut u8);
    fn rust_isa_rvv_vfnmadd_vf(acc: *const u8, a: *const u8, b: f32, out: *mut u8);
    fn rust_isa_rvv_vfnmsub_vf(acc: *const u8, a: *const u8, b: f32, out: *mut u8);
    fn rust_isa_rvv_vfrsub_vf(a: *const u8, b: f32, out: *mut u8);
}

/// `vfmadd.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmadd_vv(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfma(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmadd.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmadd_vf(acc: V128, a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfmadd_vf(acc.as_ptr(), a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vfmsub.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmsub_vv(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfms(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmsub.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmsub_vf(acc: V128, a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfmsub_vf(acc.as_ptr(), a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vfnmadd.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfnmadd_vv(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfnma(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfnmadd.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfnmadd_vf(acc: V128, a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfnmadd_vf(acc.as_ptr(), a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vfnmsub.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfnmsub_vv(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfnms(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfnmsub.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfnmsub_vf(acc: V128, a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfnmsub_vf(acc.as_ptr(), a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vfrsub.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfrsub_vf(a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfrsub_vf(a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rvv_vfwcvt_xu_f_v(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwcvt_x_f_v(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwcvt_f_xu_v(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwcvt_f_x_v(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwcvt_f_f_v(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwcvt_rtz_xu_f_v(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwcvt_rtz_x_f_v(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfncvt_xu_f_w(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfncvt_x_f_w(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfncvt_f_xu_w(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfncvt_f_x_w(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfncvt_f_f_w(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfncvt_rod_f_f_w(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfncvt_rtz_xu_f_w(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfncvt_rtz_x_f_w(a: *const u8, out: *mut u8);
}

/// `vfwcvt.xu.f.v`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwcvt_xu_f_v(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwcvt_xu_f_v(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfwcvt.x.f.v`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwcvt_x_f_v(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwcvt_x_f_v(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfwcvt.f.xu.v`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwcvt_f_xu_v(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwcvt_f_xu_v(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfwcvt.f.x.v`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwcvt_f_x_v(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwcvt_f_x_v(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfwcvt.f.f.v`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwcvt_f_f_v(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwcvt_f_f_v(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfwcvt.rtz.xu.f.v`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwcvt_rtz_xu_f_v(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwcvt_rtz_xu_f_v(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfwcvt.rtz.x.f.v`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwcvt_rtz_x_f_v(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwcvt_rtz_x_f_v(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfncvt.xu.f.w`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfncvt_xu_f_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfncvt_xu_f_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfncvt.x.f.w`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfncvt_x_f_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfncvt_x_f_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfncvt.f.xu.w`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfncvt_f_xu_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfncvt_f_xu_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfncvt.f.x.w`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfncvt_f_x_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfncvt_f_x_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfncvt.f.f.w`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfncvt_f_f_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfncvt_f_f_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfncvt.rod.f.f.w`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfncvt_rod_f_f_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfncvt_rod_f_f_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfncvt.rtz.xu.f.w`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfncvt_rtz_xu_f_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfncvt_rtz_xu_f_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfncvt.rtz.x.f.w`
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfncvt_rtz_x_f_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfncvt_rtz_x_f_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rvv_vfmacc_vv(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfmacc_vf(acc: *const u8, a: *const u8, b: f32, out: *mut u8);
    fn rust_isa_rvv_vfnmacc_vv(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfnmacc_vf(acc: *const u8, a: *const u8, b: f32, out: *mut u8);
    fn rust_isa_rvv_vfmsac_vv(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfmsac_vf(acc: *const u8, a: *const u8, b: f32, out: *mut u8);
    fn rust_isa_rvv_vfnmsac_vv(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfnmsac_vf(acc: *const u8, a: *const u8, b: f32, out: *mut u8);
    fn rust_isa_rvv_vmulhu_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmulhu_vx(a: *const u8, b: u32, out: *mut u8);
    fn rust_isa_rvv_vmulhsu_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmulhsu_vx(a: *const u8, b: i32, out: *mut u8);
    fn rust_isa_rvv_vmulh_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmulh_vx(a: *const u8, b: i32, out: *mut u8);
    fn rust_isa_rvv_vmv1r(values: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmv2r(values: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmv4r(values: *const u8, out: *mut u8);
    fn rust_isa_rvv_vmv8r(values: *const u8, out: *mut u8);
}

/// `vfmacc.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmacc_vv(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfmacc_vv(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmacc.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmacc_vf(acc: V128, a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfmacc_vf(acc.as_ptr(), a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vfnmacc.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfnmacc_vv(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfnmacc_vv(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfnmacc.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfnmacc_vf(acc: V128, a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfnmacc_vf(acc.as_ptr(), a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vfmsac.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmsac_vv(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfmsac_vv(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmsac.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmsac_vf(acc: V128, a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfmsac_vf(acc.as_ptr(), a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vfnmsac.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfnmsac_vv(acc: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfnmsac_vv(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfnmsac.vf` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfnmsac_vf(acc: V128, a: V128, b: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfnmsac_vf(acc.as_ptr(), a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vmulhu.vv` on 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmulhu_vv(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmulhu_vv(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulhu.vx` on 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmulhu_vx(a: V128, b: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmulhu_vx(a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vmulhsu.vv` on 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmulhsu_vv(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmulhsu_vv(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulhsu.vx` on 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmulhsu_vx(a: V128, b: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmulhsu_vx(a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vmulh.vv` on 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmulh_vv(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmulh_vv(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulh.vx` on 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmulh_vx(a: V128, b: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmulh_vx(a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vmv1r.v` whole-register move.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv1r(values: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vmv1r(values.as_ptr().cast(), out.as_mut_ptr().cast());
    }
    out
}

/// `vmv2r.v` whole-register move.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv2r(values: [V128; 2]) -> [V128; 2] {
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rvv_vmv2r(values.as_ptr().cast(), out.as_mut_ptr().cast());
    }
    out
}

/// `vmv4r.v` whole-register move.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv4r(values: [V128; 4]) -> [V128; 4] {
    let mut out = [V128::from_bytes([0; 16]); 4];
    unsafe {
        rust_isa_rvv_vmv4r(values.as_ptr().cast(), out.as_mut_ptr().cast());
    }
    out
}

/// `vmv8r.v` whole-register move.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv8r(values: [V128; 8]) -> [V128; 8] {
    let mut out = [V128::from_bytes([0; 16]); 8];
    unsafe {
        rust_isa_rvv_vmv8r(values.as_ptr().cast(), out.as_mut_ptr().cast());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rvv_vfwredusum_vs(a: *const u8, acc: *const u8, out: *mut u8);
    fn rust_isa_rvv_vfwredosum_vs(a: *const u8, acc: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vx(a: *const u8, b: i32, out: *mut u8);
    fn rust_isa_rvv_vadc_vvm(a: *const u8, b: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vadc_vxm(a: *const u8, b: i32, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vvm(a: *const u8, b: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vxm(a: *const u8, b: i32, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vv(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmadc_vx(a: *const u8, b: i32) -> u16;
    fn rust_isa_rvv_vsbc_vvm(a: *const u8, b: *const u8, borrow: u16, out: *mut u8);
    fn rust_isa_rvv_vsbc_vxm(a: *const u8, b: i32, borrow: u16, out: *mut u8);
    fn rust_isa_rvv_vmsbc_vvm(a: *const u8, b: *const u8, borrow: u16) -> u16;
    fn rust_isa_rvv_vmsbc_vxm(a: *const u8, b: i32, borrow: u16) -> u16;
    fn rust_isa_rvv_vmsbc_vv(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rvv_vmsbc_vx(a: *const u8, b: i32) -> u16;
    fn rust_isa_rvv_vsmul_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vsmul_vx(a: *const u8, b: i32, out: *mut u8);
    fn rust_isa_rvv_vssrl_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssrl_vx(a: *const u8, b: u32, out: *mut u8);
    fn rust_isa_rvv_vssra_vv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vx(a: *const u8, b: u32, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_0(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_0(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_0(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_0(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_0(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_0(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_1(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_1(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_1(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_1(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_1(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_1(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_2(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_2(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_2(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_2(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_2(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_2(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_3(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_3(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_3(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_3(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_3(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_3(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_4(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_4(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_4(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_4(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_4(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_4(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_5(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_5(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_5(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_5(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_5(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_5(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_6(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_6(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_6(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_6(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_6(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_6(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_7(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_7(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_7(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_7(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_7(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_7(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_8(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_8(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_8(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_8(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_8(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_8(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_9(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_9(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_9(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_9(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_9(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_9(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_10(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_10(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_10(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_10(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_10(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_10(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_11(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_11(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_11(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_11(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_11(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_11(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_12(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_12(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_12(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_12(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_12(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_12(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_13(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_13(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_13(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_13(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_13(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_13(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_14(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_14(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_14(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_14(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_14(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_14(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_15(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_15(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_15(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_15(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_15(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_15(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_16(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_16(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_16(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_16(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_16(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_16(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_17(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_17(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_17(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_17(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_17(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_17(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_18(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_18(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_18(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_18(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_18(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_18(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_19(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_19(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_19(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_19(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_19(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_19(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_20(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_20(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_20(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_20(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_20(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_20(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_21(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_21(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_21(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_21(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_21(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_21(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_22(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_22(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_22(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_22(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_22(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_22(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_23(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_23(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_23(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_23(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_23(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_23(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_24(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_24(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_24(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_24(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_24(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_24(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_25(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_25(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_25(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_25(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_25(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_25(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_26(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_26(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_26(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_26(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_26(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_26(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_27(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_27(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_27(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_27(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_27(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_27(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_28(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_28(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_28(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_28(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_28(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_28(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_29(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_29(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_29(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_29(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_29(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_29(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_30(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_30(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_30(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_30(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_30(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_30(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vrsub_vi_31(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vadc_vim_31(a: *const u8, carry: u16, out: *mut u8);
    fn rust_isa_rvv_vmadc_vim_31(a: *const u8, carry: u16) -> u16;
    fn rust_isa_rvv_vmadc_vi_31(a: *const u8) -> u16;
    fn rust_isa_rvv_vssrl_vi_31(a: *const u8, out: *mut u8);
    fn rust_isa_rvv_vssra_vi_31(a: *const u8, out: *mut u8);
}

/// `vfwredusum.vs` widening reduction on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwredusum_vs(a: V128, acc: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwredusum_vs(a.as_ptr(), acc.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfwredosum.vs` widening reduction on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfwredosum_vs(a: V128, acc: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vfwredosum_vs(a.as_ptr(), acc.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrsub.vx` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vrsub_vx(a: V128, b: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vrsub_vx(a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vadc.vvm` on 32-bit integer lanes.
///
/// `carry` holds one carry bit per lane in the low four bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vadc_vvm(a: V128, b: V128, carry: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vadc_vvm(a.as_ptr(), b.as_ptr(), carry, out.as_mut_ptr());
    }
    out
}

/// `vadc.vxm` on 32-bit integer lanes.
///
/// `carry` holds one carry bit per lane in the low four bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vadc_vxm(a: V128, b: i32, carry: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vadc_vxm(a.as_ptr(), b, carry, out.as_mut_ptr());
    }
    out
}

/// `vmadc.vvm` on 32-bit integer lanes.
///
/// Returns a lane mask as all-ones or zero in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmadc_vvm(a: V128, b: V128, carry: u16) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vvm(a.as_ptr(), b.as_ptr(), carry) })
}

/// `vmadc.vxm` on 32-bit integer lanes.
///
/// Returns a lane mask as all-ones or zero in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmadc_vxm(a: V128, b: i32, carry: u16) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vxm(a.as_ptr(), b, carry) })
}

/// `vmadc.vv` on 32-bit integer lanes.
///
/// Returns a lane mask as all-ones or zero in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmadc_vv(a: V128, b: V128) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vv(a.as_ptr(), b.as_ptr()) })
}

/// `vmadc.vx` on 32-bit integer lanes.
///
/// Returns a lane mask as all-ones or zero in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmadc_vx(a: V128, b: i32) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vx(a.as_ptr(), b) })
}

/// `vsbc.vvm` on 32-bit integer lanes.
///
/// `borrow` holds one borrow bit per lane in the low four bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsbc_vvm(a: V128, b: V128, borrow: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vsbc_vvm(a.as_ptr(), b.as_ptr(), borrow, out.as_mut_ptr());
    }
    out
}

/// `vsbc.vxm` on 32-bit integer lanes.
///
/// `borrow` holds one borrow bit per lane in the low four bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsbc_vxm(a: V128, b: i32, borrow: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vsbc_vxm(a.as_ptr(), b, borrow, out.as_mut_ptr());
    }
    out
}

/// `vmsbc.vvm` on 32-bit integer lanes.
///
/// Returns a lane mask as all-ones or zero in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmsbc_vvm(a: V128, b: V128, borrow: u16) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmsbc_vvm(a.as_ptr(), b.as_ptr(), borrow) })
}

/// `vmsbc.vxm` on 32-bit integer lanes.
///
/// Returns a lane mask as all-ones or zero in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmsbc_vxm(a: V128, b: i32, borrow: u16) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmsbc_vxm(a.as_ptr(), b, borrow) })
}

/// `vmsbc.vv` on 32-bit integer lanes.
///
/// Returns a lane mask as all-ones or zero in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmsbc_vv(a: V128, b: V128) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmsbc_vv(a.as_ptr(), b.as_ptr()) })
}

/// `vmsbc.vx` on 32-bit integer lanes.
///
/// Returns a lane mask as all-ones or zero in each 32-bit lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmsbc_vx(a: V128, b: i32) -> V128 {
    mask_to_v128_i32(unsafe { rust_isa_rvv_vmsbc_vx(a.as_ptr(), b) })
}

/// `vsmul.vv` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsmul_vv(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vsmul_vv(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsmul.vx` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsmul_vx(a: V128, b: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vsmul_vx(a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vssrl.vv` on 32-bit lanes.
///
/// The shift amount is taken from the low bits of each lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vssrl_vv(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vssrl_vv(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrl.vx` on 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vssrl_vx(a: V128, b: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vssrl_vx(a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vssra.vv` on 32-bit lanes.
///
/// The shift amount is taken from the low bits of each lane.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vssra_vv(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vssra_vv(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssra.vx` on 32-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vssra_vx(a: V128, b: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rvv_vssra_vx(a.as_ptr(), b, out.as_mut_ptr());
    }
    out
}

/// `vrsub.vi` on 32-bit lanes.
///
/// `IMM` must be in `-16..=15`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vrsub_vi<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    match IMM + 16 {
        0 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_0(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        1 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_1(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        2 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_2(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        3 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_3(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        4 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_4(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        5 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_5(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        6 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_6(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        7 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_7(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        8 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_8(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        9 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_9(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        10 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_10(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        11 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_11(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        12 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_12(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        13 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_13(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        14 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_14(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        15 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_15(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        16 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_16(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        17 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_17(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        18 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_18(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        19 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_19(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        20 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_20(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        21 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_21(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        22 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_22(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        23 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_23(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        24 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_24(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        25 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_25(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        26 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_26(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        27 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_27(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        28 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_28(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        29 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_29(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        30 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_30(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        31 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vrsub_vi_31(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        _ => unreachable!(),
    }
}

/// `vadc.vim` on 32-bit lanes.
///
/// `IMM` must be in `-16..=15`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vadc_vim<const IMM: i32>(a: V128, carry: u16) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    match IMM + 16 {
        0 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_0(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        1 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_1(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        2 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_2(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        3 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_3(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        4 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_4(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        5 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_5(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        6 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_6(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        7 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_7(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        8 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_8(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        9 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_9(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        10 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_10(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        11 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_11(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        12 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_12(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        13 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_13(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        14 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_14(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        15 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_15(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        16 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_16(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        17 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_17(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        18 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_18(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        19 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_19(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        20 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_20(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        21 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_21(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        22 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_22(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        23 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_23(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        24 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_24(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        25 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_25(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        26 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_26(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        27 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_27(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        28 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_28(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        29 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_29(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        30 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_30(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        31 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vadc_vim_31(a.as_ptr(), carry, out.as_mut_ptr());
            }
            out
        }
        _ => unreachable!(),
    }
}

/// `vmadc.vim` on 32-bit lanes.
///
/// `IMM` must be in `-16..=15`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmadc_vim<const IMM: i32>(a: V128, carry: u16) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    match IMM + 16 {
        0 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_0(a.as_ptr(), carry) }),
        1 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_1(a.as_ptr(), carry) }),
        2 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_2(a.as_ptr(), carry) }),
        3 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_3(a.as_ptr(), carry) }),
        4 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_4(a.as_ptr(), carry) }),
        5 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_5(a.as_ptr(), carry) }),
        6 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_6(a.as_ptr(), carry) }),
        7 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_7(a.as_ptr(), carry) }),
        8 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_8(a.as_ptr(), carry) }),
        9 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_9(a.as_ptr(), carry) }),
        10 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_10(a.as_ptr(), carry) }),
        11 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_11(a.as_ptr(), carry) }),
        12 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_12(a.as_ptr(), carry) }),
        13 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_13(a.as_ptr(), carry) }),
        14 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_14(a.as_ptr(), carry) }),
        15 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_15(a.as_ptr(), carry) }),
        16 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_16(a.as_ptr(), carry) }),
        17 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_17(a.as_ptr(), carry) }),
        18 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_18(a.as_ptr(), carry) }),
        19 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_19(a.as_ptr(), carry) }),
        20 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_20(a.as_ptr(), carry) }),
        21 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_21(a.as_ptr(), carry) }),
        22 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_22(a.as_ptr(), carry) }),
        23 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_23(a.as_ptr(), carry) }),
        24 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_24(a.as_ptr(), carry) }),
        25 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_25(a.as_ptr(), carry) }),
        26 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_26(a.as_ptr(), carry) }),
        27 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_27(a.as_ptr(), carry) }),
        28 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_28(a.as_ptr(), carry) }),
        29 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_29(a.as_ptr(), carry) }),
        30 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_30(a.as_ptr(), carry) }),
        31 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vim_31(a.as_ptr(), carry) }),
        _ => unreachable!(),
    }
}

/// `vmadc.vi` on 32-bit lanes.
///
/// `IMM` must be in `-16..=15`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmadc_vi<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    match IMM + 16 {
        0 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_0(a.as_ptr()) }),
        1 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_1(a.as_ptr()) }),
        2 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_2(a.as_ptr()) }),
        3 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_3(a.as_ptr()) }),
        4 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_4(a.as_ptr()) }),
        5 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_5(a.as_ptr()) }),
        6 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_6(a.as_ptr()) }),
        7 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_7(a.as_ptr()) }),
        8 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_8(a.as_ptr()) }),
        9 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_9(a.as_ptr()) }),
        10 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_10(a.as_ptr()) }),
        11 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_11(a.as_ptr()) }),
        12 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_12(a.as_ptr()) }),
        13 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_13(a.as_ptr()) }),
        14 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_14(a.as_ptr()) }),
        15 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_15(a.as_ptr()) }),
        16 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_16(a.as_ptr()) }),
        17 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_17(a.as_ptr()) }),
        18 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_18(a.as_ptr()) }),
        19 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_19(a.as_ptr()) }),
        20 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_20(a.as_ptr()) }),
        21 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_21(a.as_ptr()) }),
        22 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_22(a.as_ptr()) }),
        23 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_23(a.as_ptr()) }),
        24 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_24(a.as_ptr()) }),
        25 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_25(a.as_ptr()) }),
        26 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_26(a.as_ptr()) }),
        27 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_27(a.as_ptr()) }),
        28 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_28(a.as_ptr()) }),
        29 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_29(a.as_ptr()) }),
        30 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_30(a.as_ptr()) }),
        31 => mask_to_v128_i32(unsafe { rust_isa_rvv_vmadc_vi_31(a.as_ptr()) }),
        _ => unreachable!(),
    }
}

/// `vssrl.vi` on 32-bit lanes.
///
/// `IMM` must be in `0..=31`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vssrl_vi<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    match IMM {
        0 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_0(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        1 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_1(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        2 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_2(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        3 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_3(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        4 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_4(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        5 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_5(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        6 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_6(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        7 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_7(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        8 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_8(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        9 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_9(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        10 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_10(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        11 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_11(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        12 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_12(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        13 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_13(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        14 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_14(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        15 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_15(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        16 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_16(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        17 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_17(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        18 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_18(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        19 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_19(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        20 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_20(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        21 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_21(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        22 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_22(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        23 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_23(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        24 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_24(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        25 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_25(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        26 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_26(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        27 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_27(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        28 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_28(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        29 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_29(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        30 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_30(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        31 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssrl_vi_31(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        _ => unreachable!(),
    }
}

/// `vssra.vi` on 32-bit lanes.
///
/// `IMM` must be in `0..=31`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vssra_vi<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    match IMM {
        0 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_0(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        1 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_1(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        2 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_2(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        3 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_3(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        4 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_4(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        5 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_5(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        6 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_6(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        7 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_7(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        8 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_8(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        9 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_9(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        10 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_10(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        11 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_11(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        12 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_12(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        13 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_13(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        14 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_14(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        15 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_15(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        16 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_16(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        17 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_17(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        18 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_18(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        19 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_19(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        20 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_20(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        21 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_21(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        22 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_22(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        23 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_23(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        24 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_24(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        25 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_25(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        26 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_26(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        27 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_27(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        28 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_28(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        29 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_29(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        30 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_30(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        31 => {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                rust_isa_rvv_vssra_vi_31(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
        _ => unreachable!(),
    }
}
