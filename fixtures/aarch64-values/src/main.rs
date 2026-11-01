#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::aarch64::crypto::aes::{aesd, aese, aesimc, aesmc};
use rust_isa::aarch64::crypto::crc::{crc32b, crc32h, crc32w, crc32x};
use rust_isa::aarch64::crypto::sha2::{sha256h, sha256h2, sha256su0, sha256su1};
use rust_isa::aarch64::crypto::sha3::xar;
use rust_isa::aarch64::crypto::sha3::{bcax, eor3, rax1};
use rust_isa::aarch64::crypto::sm4::{sm4e, sm4ekey};
use rust_isa::aarch64::simd::bf16::bfdot;
use rust_isa::aarch64::simd::bf16::bfmmla;
use rust_isa::aarch64::simd::dotprod::{sdot, udot};
use rust_isa::aarch64::simd::f32mm::fmmla_f32;
use rust_isa::aarch64::simd::f64mm::fmmla_f64;
use rust_isa::aarch64::simd::fcma::fcmla_rot90;
use rust_isa::aarch64::simd::fcma::{fcmla, fcmla_rot180, fcmla_rot270};
use rust_isa::aarch64::simd::fhm::{fmlal_f16, fmlal2_f16, fmlsl_f16, fmlsl2_f16};
use rust_isa::aarch64::simd::fp16::{fadd_f16, fmla_f16, fmls_f16, fmul_f16};
use rust_isa::aarch64::simd::fp16::{
    fcvt_f16_f32, fcvt_f16_f64, fcvt_f32_f16, fcvt_f64_f16, fcvtl_f32_f16, fcvtl2_f32_f16,
    fcvtn_f16_f32, fcvtn2_f16_f32, fcvtxn_f16_f64, fdiv_f16, frecpx_f16, fsub_f16,
};
use rust_isa::aarch64::simd::i8mm::{smmla, ummla, usmmla};
use rust_isa::aarch64::simd::jsconv::{fjcvtzs, fjcvtzs_v128};
use rust_isa::aarch64::simd::neon::{
    V128, abs_i32, add, add_i8, add_i16, add_i32, add_i64, addp_s8, addp_s16, addp_s32, addv_s8,
    addv_s16, addv_s32, and, bic, bif, bit, bsl, cls_i8, cls_i16, cls_i32, clz_i8, clz_i16,
    clz_i32, cmge_i8, cmge_i16, cmge_i32, cmle_i8, cmle_i16, cmle_i32, cmlt_i8, cmlt_i16, cmlt_i32,
    cmpeq_i8, cmpeq_i16, cmpeq_i32, cmpeq_i64, cmpgt_i8, cmpgt_i16, cmpgt_i32, cmpgt_i64, cmtst_i8,
    cmtst_i16, cmtst_i32, cnt_i8, eor, extract_i32, fabd_f32, fabd_f64, fabs, facge_f32, facge_f64,
    facgt_f32, facgt_f64, fadd, fadd_f64, faddp_f32, faddp_f64, faddv_f32, faddv_f64, fcmeq_f32,
    fcmeq_f64, fcmge_f32, fcmge_f64, fcmgt_f32, fcmgt_f64, fcmle_f32, fcmle_f64, fcmlt_f32,
    fcmlt_f64, fdiv, fmax, fmaxnm_f32, fmaxnm_f64, fmaxnmv_f32, fmaxnmv_f64, fmaxv_f32, fmaxv_f64,
    fmin, fminnm_f32, fminnm_f64, fminnmv_f32, fminnmv_f64, fminv_f32, fminv_f64, fmla_f32,
    fmla_f64, fmls_f32, fmls_f64, fmul, fmul_f64, fmulx_f32, fmulx_f64, fneg, fsqrt, fsub, mla_s8,
    mla_s16, mla_s32, mls_s8, mls_s16, mls_s32, mul, mul_i8, mul_i16, mul_i32, neg_i32, not, orn,
    orr, rbit, replace_i32, rev16, rev32, rev64, saddl_s8, saddl_s16, saddl_s32, saddw_s8,
    saddw_s16, saddw_s32, sar_i8, sar_i16, sar_i32, shl_i32, shr_i16, smax_s8, smax_s16, smax_s32,
    smaxp_s8, smaxp_s16, smaxp_s32, smaxv_s8, smaxv_s16, smaxv_s32, smin_s8, smin_s16, smin_s32,
    sminp_s8, sminp_s16, sminp_s32, sminv_s8, sminv_s16, sminv_s32, splat_i32, ssubl_s8, ssubl_s16,
    ssubl_s32, ssubw_s8, ssubw_s16, ssubw_s32, sub_i8, sub_i16, sub_i32, sub_i64, tbl, trn1, trn2,
    uaddl_u8, uaddl_u16, uaddl_u32, uaddw_u8, uaddw_u16, uaddw_u32, umax_u8, umax_u16, umax_u32,
    umaxp_u8, umaxp_u16, umaxp_u32, umaxv_u8, umaxv_u16, umaxv_u32, umin_u8, umin_u16, umin_u32,
    uminp_u8, uminp_u16, uminp_u32, uminv_u8, uminv_u16, uminv_u32, usubl_u8, usubl_u16, usubl_u32,
    usubw_u8, usubw_u16, usubw_u32, uzp1, uzp2, v128_load, v128_store, zip1, zip2,
};
use rust_isa::aarch64::simd::neon::{
    cls_u64, clz_u64, cnt_u64, ext, ldnp, ldp, pmull_p64, splat_f32, splat_f64, splat_i8,
    splat_i16, splat_i64, stnp, stp,
};
use rust_isa::aarch64::simd::neon::{
    dup_i8 as dup_lane_i8, dup_i16 as dup_lane_i16, dup_i64 as dup_lane_i64, ins_i8 as ins_lane_i8,
    ins_i16 as ins_lane_i16, ins_i32 as ins_lane_i32, ins_i64 as ins_lane_i64,
};
use rust_isa::aarch64::simd::neon::{ld1, ld2, ld3, ld4, st1, st2, st3, st4};
use rust_isa::aarch64::simd::rdm::{sqrdmlah, sqrdmlsh};
use rust_isa::aarch64::simd::sve;
use rust_isa::aarch64::simd::sve2;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "mov x8, #93",
            "svc #0",
            in("x0") code as u64,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "mov x8, #93",
            "svc #0",
            in("x0") 255u64,
            options(noreturn, nostack)
        );
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_add(a: &V128, b: &V128) -> V128 {
    add(add(*a, *b), *b)
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_mul(a: &V128, b: &V128) -> V128 {
    mul(mul(*a, *b), *b)
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_tbl(a: &V128, idx: &V128) -> V128 {
    tbl(tbl(*a, *idx), *idx)
}

#[inline(never)]
#[target_feature(enable = "dotprod")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_dot(a: &V128, b: &V128, acc: &V128) -> V128 {
    let first = unsafe { sdot(*a, *b, *acc) };
    unsafe { sdot(*a, *b, first) }
}

#[inline(never)]
#[target_feature(enable = "aes")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_aes(a: &V128, b: &V128) -> V128 {
    unsafe { aese(aese(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "sha2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sha2(a: &V128, b: &V128, c: &V128) -> V128 {
    unsafe { sha256h(sha256h(*a, *b, *c), *b, *c) }
}

#[inline(never)]
#[target_feature(enable = "crc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_crc(crc: u32, value: u8) -> u32 {
    unsafe { crc32b(crc32b(crc, value), value) }
}

#[inline(never)]
#[target_feature(enable = "fp16,fhm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_fp16(a: &V128, b: &V128) -> V128 {
    unsafe { fadd_f16(fadd_f16(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "sha3")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sha3(a: &V128, b: &V128, c: &V128) -> V128 {
    unsafe { eor3(eor3(*a, *b, *c), *b, *c) }
}

#[inline(never)]
#[target_feature(enable = "sm4")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sm4(a: &V128, b: &V128) -> V128 {
    unsafe { sm4e(sm4e(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "i8mm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_i8mm(acc: &V128, a: &V128, b: &V128) -> V128 {
    unsafe { smmla(smmla(*acc, *a, *b), *a, *b) }
}

#[inline(never)]
#[target_feature(enable = "bf16")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_bf16(acc: &V128, a: &V128, b: &V128) -> V128 {
    unsafe { bfdot(bfdot(*acc, *a, *b), *a, *b) }
}

#[inline(never)]
#[target_feature(enable = "fp16,fhm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_fhm(acc: &V128, a: &V128, b: &V128) -> V128 {
    unsafe { fmla_f16(fmla_f16(*acc, *a, *b), *a, *b) }
}

#[inline(never)]
#[target_feature(enable = "jsconv")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_jsconv(a: f64, b: f64) -> i32 {
    unsafe { fjcvtzs(a) ^ fjcvtzs(b) }
}

#[inline(never)]
#[target_feature(enable = "sve")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_f32mm(a: &[f32; 4], b: &[f32; 4], acc: &mut [f32; 4]) -> f32 {
    unsafe {
        fmmla_f32(a, b, acc);
        fmmla_f32(a, b, acc);
    }
    acc[0]
}

#[inline(never)]
#[target_feature(enable = "sve")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_f64mm(a: &[f64; 2], b: &[f64; 2], acc: &mut [f64; 2]) -> f64 {
    unsafe {
        fmmla_f64(a, b, acc);
        fmmla_f64(a, b, acc);
    }
    acc[0]
}

#[inline(never)]
#[target_feature(enable = "fcma")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_fcma(acc: &V128, a: &V128, b: &V128) -> V128 {
    unsafe { fcmla(fcmla(*acc, *a, *b), *a, *b) }
}

#[inline(never)]
#[target_feature(enable = "sve")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sve_add(a: &[f32; 4], b: &[f32; 4], out: &mut [f32; 4]) -> f32 {
    unsafe { sve::add_f32(a, b, out) };
    out[0]
}

#[inline(never)]
#[target_feature(enable = "sve")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sve_mul(a: &[f32; 4], b: &[f32; 4], out: &mut [f32; 4]) -> f32 {
    unsafe { sve::mul_f32(a, b, out) };
    out[0]
}

#[inline(never)]
#[target_feature(enable = "sve")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sve_sub(a: &[f32; 4], b: &[f32; 4], out: &mut [f32; 4]) -> f32 {
    unsafe { sve::sub_f32(a, b, out) };
    out[0]
}

#[target_feature(enable = "sve2")]
unsafe fn sve2_ops() -> bool {
    let a_i8 = [0; 4];
    let b_i8 = [0; 4];
    let mut o_i8 = [0; 4];
    let a_i16 = [0; 4];
    let b_i16 = [0; 4];
    let mut o_i16 = [0; 4];
    let a_i32 = [0; 4];
    let b_i32 = [0; 4];
    let mut o_i32 = [0; 4];
    let a_i64 = [0; 4];
    let b_i64 = [0; 4];
    let mut o_i64 = [0; 4];
    let a_u8 = [0; 4];
    let b_u8 = [0; 4];
    let mut o_u8 = [0; 4];
    let a_u16 = [0; 4];
    let b_u16 = [0; 4];
    let mut o_u16 = [0; 4];
    let a_u32 = [0; 4];
    let b_u32 = [0; 4];
    let mut o_u32 = [0; 4];
    let a_u64 = [0; 4];
    let b_u64 = [0; 4];
    let mut o_u64 = [0; 4];
    let a_f32 = [0.0; 4];
    let b_f32 = [0.0; 4];
    let mut o_f32 = [0.0; 4];
    let a_f64 = [0.0; 4];
    let b_f64 = [0.0; 4];
    let mut o_f64 = [0.0; 4];
    unsafe { sve2::addp_i8(&a_i8, &b_i8, &mut o_i8) };
    core::hint::black_box(&o_i8);
    unsafe { sve2::addp_i16(&a_i16, &b_i16, &mut o_i16) };
    core::hint::black_box(&o_i16);
    unsafe { sve2::addp_i32(&a_i32, &b_i32, &mut o_i32) };
    core::hint::black_box(&o_i32);
    unsafe { sve2::addp_i64(&a_i64, &b_i64, &mut o_i64) };
    core::hint::black_box(&o_i64);
    unsafe { sve2::addp_u8(&a_u8, &b_u8, &mut o_u8) };
    core::hint::black_box(&o_u8);
    unsafe { sve2::addp_u16(&a_u16, &b_u16, &mut o_u16) };
    core::hint::black_box(&o_u16);
    unsafe { sve2::addp_u32(&a_u32, &b_u32, &mut o_u32) };
    core::hint::black_box(&o_u32);
    unsafe { sve2::addp_u64(&a_u64, &b_u64, &mut o_u64) };
    core::hint::black_box(&o_u64);
    unsafe { sve2::addp_f32(&a_f32, &b_f32, &mut o_f32) };
    core::hint::black_box(&o_f32);
    unsafe { sve2::addp_f64(&a_f64, &b_f64, &mut o_f64) };
    core::hint::black_box(&o_f64);
    unsafe { sve2::shadd_i8(&a_i8, &b_i8, &mut o_i8) };
    core::hint::black_box(&o_i8);
    unsafe { sve2::shadd_i16(&a_i16, &b_i16, &mut o_i16) };
    core::hint::black_box(&o_i16);
    unsafe { sve2::shadd_i32(&a_i32, &b_i32, &mut o_i32) };
    core::hint::black_box(&o_i32);
    unsafe { sve2::shadd_i64(&a_i64, &b_i64, &mut o_i64) };
    core::hint::black_box(&o_i64);
    unsafe { sve2::shadd_u8(&a_u8, &b_u8, &mut o_u8) };
    core::hint::black_box(&o_u8);
    unsafe { sve2::shadd_u16(&a_u16, &b_u16, &mut o_u16) };
    core::hint::black_box(&o_u16);
    unsafe { sve2::shadd_u32(&a_u32, &b_u32, &mut o_u32) };
    core::hint::black_box(&o_u32);
    unsafe { sve2::shadd_u64(&a_u64, &b_u64, &mut o_u64) };
    core::hint::black_box(&o_u64);
    unsafe { sve2::shsub_i8(&a_i8, &b_i8, &mut o_i8) };
    core::hint::black_box(&o_i8);
    unsafe { sve2::shsub_i16(&a_i16, &b_i16, &mut o_i16) };
    core::hint::black_box(&o_i16);
    unsafe { sve2::shsub_i32(&a_i32, &b_i32, &mut o_i32) };
    core::hint::black_box(&o_i32);
    unsafe { sve2::shsub_i64(&a_i64, &b_i64, &mut o_i64) };
    core::hint::black_box(&o_i64);
    unsafe { sve2::shsub_u8(&a_u8, &b_u8, &mut o_u8) };
    core::hint::black_box(&o_u8);
    unsafe { sve2::shsub_u16(&a_u16, &b_u16, &mut o_u16) };
    core::hint::black_box(&o_u16);
    unsafe { sve2::shsub_u32(&a_u32, &b_u32, &mut o_u32) };
    core::hint::black_box(&o_u32);
    unsafe { sve2::shsub_u64(&a_u64, &b_u64, &mut o_u64) };
    core::hint::black_box(&o_u64);
    unsafe { sve2::uhadd_i8(&a_i8, &b_i8, &mut o_i8) };
    core::hint::black_box(&o_i8);
    unsafe { sve2::uhadd_i16(&a_i16, &b_i16, &mut o_i16) };
    core::hint::black_box(&o_i16);
    unsafe { sve2::uhadd_i32(&a_i32, &b_i32, &mut o_i32) };
    core::hint::black_box(&o_i32);
    unsafe { sve2::uhadd_i64(&a_i64, &b_i64, &mut o_i64) };
    core::hint::black_box(&o_i64);
    unsafe { sve2::uhadd_u8(&a_u8, &b_u8, &mut o_u8) };
    core::hint::black_box(&o_u8);
    unsafe { sve2::uhadd_u16(&a_u16, &b_u16, &mut o_u16) };
    core::hint::black_box(&o_u16);
    unsafe { sve2::uhadd_u32(&a_u32, &b_u32, &mut o_u32) };
    core::hint::black_box(&o_u32);
    unsafe { sve2::uhadd_u64(&a_u64, &b_u64, &mut o_u64) };
    core::hint::black_box(&o_u64);
    unsafe { sve2::uhsub_i8(&a_i8, &b_i8, &mut o_i8) };
    core::hint::black_box(&o_i8);
    unsafe { sve2::uhsub_i16(&a_i16, &b_i16, &mut o_i16) };
    core::hint::black_box(&o_i16);
    unsafe { sve2::uhsub_i32(&a_i32, &b_i32, &mut o_i32) };
    core::hint::black_box(&o_i32);
    unsafe { sve2::uhsub_i64(&a_i64, &b_i64, &mut o_i64) };
    core::hint::black_box(&o_i64);
    unsafe { sve2::uhsub_u8(&a_u8, &b_u8, &mut o_u8) };
    core::hint::black_box(&o_u8);
    unsafe { sve2::uhsub_u16(&a_u16, &b_u16, &mut o_u16) };
    core::hint::black_box(&o_u16);
    unsafe { sve2::uhsub_u32(&a_u32, &b_u32, &mut o_u32) };
    core::hint::black_box(&o_u32);
    unsafe { sve2::uhsub_u64(&a_u64, &b_u64, &mut o_u64) };
    core::hint::black_box(&o_u64);
    unsafe { sve2::smaxp_i8(&a_i8, &b_i8, &mut o_i8) };
    core::hint::black_box(&o_i8);
    unsafe { sve2::smaxp_i16(&a_i16, &b_i16, &mut o_i16) };
    core::hint::black_box(&o_i16);
    unsafe { sve2::smaxp_i32(&a_i32, &b_i32, &mut o_i32) };
    core::hint::black_box(&o_i32);
    unsafe { sve2::smaxp_i64(&a_i64, &b_i64, &mut o_i64) };
    core::hint::black_box(&o_i64);
    unsafe { sve2::sminp_i8(&a_i8, &b_i8, &mut o_i8) };
    core::hint::black_box(&o_i8);
    unsafe { sve2::sminp_i16(&a_i16, &b_i16, &mut o_i16) };
    core::hint::black_box(&o_i16);
    unsafe { sve2::sminp_i32(&a_i32, &b_i32, &mut o_i32) };
    core::hint::black_box(&o_i32);
    unsafe { sve2::sminp_i64(&a_i64, &b_i64, &mut o_i64) };
    core::hint::black_box(&o_i64);
    unsafe { sve2::umaxp_u8(&a_u8, &b_u8, &mut o_u8) };
    core::hint::black_box(&o_u8);
    unsafe { sve2::umaxp_u16(&a_u16, &b_u16, &mut o_u16) };
    core::hint::black_box(&o_u16);
    unsafe { sve2::umaxp_u32(&a_u32, &b_u32, &mut o_u32) };
    core::hint::black_box(&o_u32);
    unsafe { sve2::umaxp_u64(&a_u64, &b_u64, &mut o_u64) };
    core::hint::black_box(&o_u64);
    unsafe { sve2::uminp_u8(&a_u8, &b_u8, &mut o_u8) };
    core::hint::black_box(&o_u8);
    unsafe { sve2::uminp_u16(&a_u16, &b_u16, &mut o_u16) };
    core::hint::black_box(&o_u16);
    unsafe { sve2::uminp_u32(&a_u32, &b_u32, &mut o_u32) };
    core::hint::black_box(&o_u32);
    unsafe { sve2::uminp_u64(&a_u64, &b_u64, &mut o_u64) };
    core::hint::black_box(&o_u64);
    unsafe { sve2::faddp_f32(&a_f32, &b_f32, &mut o_f32) };
    core::hint::black_box(&o_f32);
    unsafe { sve2::faddp_f64(&a_f64, &b_f64, &mut o_f64) };
    core::hint::black_box(&o_f64);
    unsafe { sve2::fmaxp_f32(&a_f32, &b_f32, &mut o_f32) };
    core::hint::black_box(&o_f32);
    unsafe { sve2::fmaxp_f64(&a_f64, &b_f64, &mut o_f64) };
    core::hint::black_box(&o_f64);
    unsafe { sve2::fminp_f32(&a_f32, &b_f32, &mut o_f32) };
    core::hint::black_box(&o_f32);
    unsafe { sve2::fminp_f64(&a_f64, &b_f64, &mut o_f64) };
    core::hint::black_box(&o_f64);
    unsafe { sve2::fmaxnmp_f32(&a_f32, &b_f32, &mut o_f32) };
    core::hint::black_box(&o_f32);
    unsafe { sve2::fmaxnmp_f64(&a_f64, &b_f64, &mut o_f64) };
    core::hint::black_box(&o_f64);
    unsafe { sve2::fminnmp_f32(&a_f32, &b_f32, &mut o_f32) };
    core::hint::black_box(&o_f32);
    unsafe { sve2::fminnmp_f64(&a_f64, &b_f64, &mut o_f64) };
    core::hint::black_box(&o_f64);
    true
}
fn run() -> u32 {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    let c = V128::from_bytes([3; 16]);
    core::hint::black_box(unsafe { kernel_aes(&a, &b) });
    core::hint::black_box(unsafe { kernel_sha2(&a, &b, &c) });
    core::hint::black_box(unsafe { kernel_crc(0, 1) });
    core::hint::black_box(unsafe { kernel_fp16(&a, &b) });
    core::hint::black_box(unsafe { kernel_sha3(&a, &b, &c) });
    core::hint::black_box(unsafe { kernel_sm4(&a, &b) });
    core::hint::black_box(unsafe { kernel_i8mm(&a, &b, &c) });
    core::hint::black_box(unsafe { kernel_bf16(&a, &b, &c) });
    core::hint::black_box(unsafe { kernel_fhm(&a, &b, &c) });
    core::hint::black_box(unsafe { kernel_jsconv(1.0, 2.0) });
    let mut f32_out = [0.0f32; 4];
    core::hint::black_box(unsafe { kernel_sve_add(&[1.0; 4], &[2.0; 4], &mut f32_out) });
    core::hint::black_box(unsafe { kernel_sve_mul(&[1.0; 4], &[2.0; 4], &mut f32_out) });
    core::hint::black_box(unsafe { kernel_sve_sub(&[1.0; 4], &[2.0; 4], &mut f32_out) });
    core::hint::black_box(unsafe { kernel_f32mm(&[1.0; 4], &[2.0; 4], &mut f32_out) });
    let mut f64_out = [0.0f64; 2];
    core::hint::black_box(unsafe { kernel_f64mm(&[1.0; 2], &[2.0; 2], &mut f64_out) });
    core::hint::black_box(unsafe { kernel_fcma(&a, &b, &c) });
    if !load_store() {
        return 1;
    }
    if !integer() {
        return 2;
    }
    if !float() {
        return 3;
    }
    if !table() {
        return 4;
    }
    if !unsafe { dot() } {
        return 5;
    }
    if !integer_ops() {
        return 6;
    }
    if !float_ops() {
        return 7;
    }
    if !shuffle_ops() {
        return 8;
    }
    if !unsafe { fp16_ops() } {
        return 9;
    }
    if !unsafe { dotprod_ops() } {
        return 10;
    }
    if !unsafe { i8mm_ops() } {
        return 12;
    }
    if !unsafe { fcma_ops() } {
        return 14;
    }
    if !unsafe { fhm_ops() } {
        return 15;
    }
    if !unsafe { crypto_aes_ops() } {
        return 16;
    }
    if !unsafe { crypto_sha2_ops() } {
        return 17;
    }
    if !unsafe { crypto_sha3_ops() } {
        return 18;
    }
    if !unsafe { crypto_sm4_ops() } {
        return 19;
    }
    if !unsafe { crypto_crc_ops() } {
        return 20;
    }
    if !neon_bitwise_ops() {
        return 21;
    }
    if !unsafe { jsconv_ops() } {
        return 22;
    }
    if !unsafe { f32mm_ops() } {
        return 23;
    }
    if !unsafe { f64mm_ops() } {
        return 24;
    }
    if !unsafe { sve_ops() } {
        return 11;
    }
    if !unsafe { sve2_ops() } {
        return 34;
    }
    if !unsafe { sve_ext_ops() } {
        return 25;
    }
    if !unsafe { bf16_ops() } {
        return 26;
    }
    if !structure_ops() {
        return 27;
    }
    if !unsafe { rdm_ops() } {
        return 28;
    }
    if !lane_ops() {
        return 29;
    }
    if !unsafe { sve_mnemonics_smoke() } {
        return 30;
    }
    if !remaining_ops() {
        return 31;
    }
    if !unsafe { sve_mnemonic_values() } {
        return 32;
    }
    if !unsafe { sve_mnemonic_values2() } {
        return 33;
    }
    let gap = gap_ops();
    if gap != 0 {
        return 40 + gap;
    }
    0
}

fn gap_ops() -> u32 {
    let minus_one = V128::from_bytes([0xff; 16]);
    let zero = V128::from_bytes([0; 16]);
    let one16 = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    if smax_s8(minus_one, zero) != zero {
        return 1;
    }
    if smin_s16(minus_one, zero) != minus_one {
        return 2;
    }
    if add_i16(one16, one16).to_bytes()[0] != 2 {
        return 3;
    }
    if sub_i8(V128::from_bytes([3; 16]), V128::from_bytes([1; 16])).to_bytes() != [2; 16] {
        return 4;
    }
    if clz_i16(one16).to_bytes()[0] != 15 {
        return 5;
    }
    if unsafe { rax1(zero, zero) }.to_bytes() != [0; 16] {
        return 6;
    }
    0
}

// SME streaming mode is not entered by the value fixture because the QEMU
// configuration used by the test does not expose it. Keep the call in a
// compiled function so the reference inventory still sees it.
#[allow(dead_code)]
fn sme_enter_reference() {
    let _ = unsafe { rust_isa::aarch64::simd::sme::SmeGuard::enter() };
}

#[target_feature(enable = "dotprod")]
unsafe fn dot() -> bool {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    let acc = V128::from_bytes([0; 16]);
    let out = unsafe { sdot(a, b, acc) }.to_bytes();
    for lane in 0..4 {
        let value = i32::from_le_bytes(out[lane * 4..lane * 4 + 4].try_into().unwrap());
        if value != 8 {
            return false;
        }
    }
    let negative = V128::from_bytes([0xff; 16]);
    let out = unsafe { sdot(negative, b, acc) }.to_bytes();
    for lane in 0..4 {
        let value = i32::from_le_bytes(out[lane * 4..lane * 4 + 4].try_into().unwrap());
        if value != -8 {
            return false;
        }
    }
    let out = unsafe { kernel_dot(&a, &b, &acc) }.to_bytes();
    for lane in 0..4 {
        let value = i32::from_le_bytes(out[lane * 4..lane * 4 + 4].try_into().unwrap());
        if value != 16 {
            return false;
        }
    }
    if kernel_mul(&a, &b).to_bytes() != mul(mul(a, b), b).to_bytes() {
        return false;
    }
    true
}

fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn lanes(value: V128) -> [u32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|index| {
        u32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
    })
}

fn load_store() -> bool {
    let source = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let value = unsafe { v128_load(source.as_ptr()) };
    if value.to_bytes() != source {
        return false;
    }
    let mut output = [0u8; 16];
    unsafe {
        v128_store(output.as_mut_ptr(), value);
    }
    output == source
}

fn integer() -> bool {
    let a = i32x4([1, 2, 3, 4]);
    let b = i32x4([10, 20, 30, 40]);
    lanes(kernel_add(&a, &b)) == [21, 42, 63, 84] && lanes(mul(a, b)) == [10, 40, 90, 160]
}

fn float() -> bool {
    let mut a = [0u8; 16];
    let mut b = [0u8; 16];
    for lane in 0..4 {
        a[lane * 4..lane * 4 + 4].copy_from_slice(&(lane as f32 + 1.0).to_le_bytes());
        b[lane * 4..lane * 4 + 4].copy_from_slice(&10.0f32.to_le_bytes());
    }
    let a = V128::from_bytes(a);
    let b = V128::from_bytes(b);
    let sum = fadd(a, b).to_bytes();
    let product = fmul(a, b).to_bytes();
    for lane in 0..4 {
        let sum = f32::from_le_bytes(sum[lane * 4..lane * 4 + 4].try_into().unwrap());
        let product = f32::from_le_bytes(product[lane * 4..lane * 4 + 4].try_into().unwrap());
        if sum != lane as f32 + 11.0 || product != (lane as f32 + 1.0) * 10.0 {
            return false;
        }
    }
    true
}

#[target_feature(enable = "rdm")]
unsafe fn rdm_ops() -> bool {
    let acc = i32x4([0; 4]);
    let a = i32x4([1 << 16; 4]);
    let b = i32x4([1 << 16; 4]);
    let sum = unsafe { sqrdmlah(acc, a, b) };
    let diff = unsafe { sqrdmlsh(i32x4([200_000; 4]), a, b) };
    let max = unsafe { sqrdmlah(acc, i32x4([i32::MAX; 4]), i32x4([i32::MAX; 4])) };
    let min = unsafe { sqrdmlsh(acc, i32x4([i32::MAX; 4]), i32x4([i32::MAX; 4])) };
    lanes(sum) == [2; 4]
        && lanes(diff) == [199_998; 4]
        && lanes(max) == [i32::MAX as u32 - 1; 4]
        && lanes(min) == [i32::MIN as u32 + 2; 4]
}

fn lane_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let dup8 = dup_lane_i8::<5>(a).to_bytes();
    if dup8 != [5; 16] {
        return false;
    }
    let dup16 = dup_lane_i16::<7>(i16x8([10, 20, 30, 40, 50, 60, 70, 80]));
    if i16_lanes(dup16) != [80; 8] {
        return false;
    }
    let lanes64 = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    if !i64_ok(dup_lane_i64::<1>(lanes64), [2, 2]) {
        return false;
    }

    let mut expected = a.to_bytes();
    expected[3] = 0x7f;
    if ins_lane_i8::<3>(a, 0x7f).to_bytes() != expected {
        return false;
    }
    let inserted16 = ins_lane_i16::<3>(i16x8([10, 20, 30, 40, 50, 60, 70, 80]), 999);
    if i16_lanes(inserted16) != [10, 20, 30, 999, 50, 60, 70, 80] {
        return false;
    }
    let inserted32 = ins_lane_i32::<1>(i32x4([1, 2, 3, 4]), -7);
    if lanes(inserted32) != [1, -7i32 as u32, 3, 4] {
        return false;
    }
    let inserted64 = ins_lane_i64::<0>(lanes64, 123);
    i64_ok(inserted64, [123, 2])
}

fn structure_ops() -> bool {
    let bytes = core::array::from_fn::<u8, 16, _>(|index| index as u8);
    let mut out = [0u8; 16];
    unsafe {
        st1(out.as_mut_ptr(), ld1(bytes.as_ptr()));
    }
    if out != bytes {
        return false;
    }

    let data2 = core::array::from_fn::<u8, 32, _>(|index| index as u8);
    let (a2, b2) = unsafe { ld2(data2.as_ptr()) };
    if a2.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 2) as u8)
        || b2.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 2 + 1) as u8)
    {
        return false;
    }
    let mut out2 = [0u8; 32];
    unsafe { st2(out2.as_mut_ptr(), a2, b2) };
    if out2 != data2 {
        return false;
    }

    let data3 = core::array::from_fn::<u8, 48, _>(|index| index as u8);
    let (a3, b3, c3) = unsafe { ld3(data3.as_ptr()) };
    if a3.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 3) as u8)
        || b3.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 3 + 1) as u8)
        || c3.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 3 + 2) as u8)
    {
        return false;
    }
    let mut out3 = [0u8; 48];
    unsafe { st3(out3.as_mut_ptr(), a3, b3, c3) };
    if out3 != data3 {
        return false;
    }

    let data4 = core::array::from_fn::<u8, 64, _>(|index| index as u8);
    let (a4, b4, c4, d4) = unsafe { ld4(data4.as_ptr()) };
    if a4.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 4) as u8)
        || b4.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 4 + 1) as u8)
        || c4.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 4 + 2) as u8)
        || d4.to_bytes() != core::array::from_fn::<u8, 16, _>(|index| (index * 4 + 3) as u8)
    {
        return false;
    }
    let mut out4 = [0u8; 64];
    unsafe { st4(out4.as_mut_ptr(), a4, b4, c4, d4) };
    out4 == data4
}

#[target_feature(enable = "sve")]
unsafe fn sve_mnemonics_smoke() -> bool {
    let in_a_f32_4 = [1.0 as f32; 4];
    let in_a_f64_4 = [1.0 as f64; 4];
    let in_a_i16_4 = [1 as i16; 4];
    let in_a_i16_8 = [1 as i16; 8];
    let in_a_i32_4 = [1 as i32; 4];
    let in_a_i64_4 = [1 as i64; 4];
    let in_a_i8_4 = [1 as i8; 4];
    let in_a_u16_1 = [1 as u16; 1];
    let in_a_u16_4 = [1 as u16; 4];
    let in_a_u16_8 = [1 as u16; 8];
    let in_a_u16_12 = [1 as u16; 12];
    let in_a_u16_16 = [1 as u16; 16];
    let in_a_u32_1 = [1 as u32; 1];
    let in_a_u32_4 = [1 as u32; 4];
    let in_a_u32_8 = [1 as u32; 8];
    let in_a_u32_12 = [1 as u32; 12];
    let in_a_u32_16 = [1 as u32; 16];
    let in_a_u64_1 = [1 as u64; 1];
    let in_a_u64_2 = [1 as u64; 2];
    let in_a_u64_4 = [1 as u64; 4];
    let in_a_u64_8 = [1 as u64; 8];
    let in_a_u64_12 = [1 as u64; 12];
    let in_a_u64_16 = [1 as u64; 16];
    let in_a_u8_4 = [1 as u8; 4];
    let in_a_u8_16 = [1 as u8; 16];
    let in_a0_u16_4 = [1 as u16; 4];
    let in_a0_u32_4 = [1 as u32; 4];
    let in_a0_u64_4 = [1 as u64; 4];
    let in_a1_u16_4 = [1 as u16; 4];
    let in_a1_u32_4 = [1 as u32; 4];
    let in_a1_u64_4 = [1 as u64; 4];
    let in_a2_u16_4 = [1 as u16; 4];
    let in_a2_u32_4 = [1 as u32; 4];
    let in_a2_u64_4 = [1 as u64; 4];
    let in_a3_u16_4 = [1 as u16; 4];
    let in_a3_u32_4 = [1 as u32; 4];
    let in_a3_u64_4 = [1 as u64; 4];
    let in_acc_f32_4 = [1.0 as f32; 4];
    let in_acc_f64_4 = [1.0 as f64; 4];
    let in_acc_i16_4 = [1 as i16; 4];
    let in_acc_i32_4 = [1 as i32; 4];
    let in_acc_i8_4 = [1 as i8; 4];
    let in_acc_u16_4 = [1 as u16; 4];
    let in_acc_u32_4 = [1 as u32; 4];
    let in_acc_u8_4 = [1 as u8; 4];
    let in_b_f32_4 = [1.0 as f32; 4];
    let in_b_f64_4 = [1.0 as f64; 4];
    let in_b_i16_4 = [1 as i16; 4];
    let in_b_i32_4 = [1 as i32; 4];
    let in_b_i64_4 = [1 as i64; 4];
    let in_b_i8_4 = [1 as i8; 4];
    let in_b_i8_16 = [1 as i8; 16];
    let in_b_u16_4 = [1 as u16; 4];
    let in_b_u32_4 = [1 as u32; 4];
    let in_b_u64_4 = [1 as u64; 4];
    let in_b_u8_4 = [1 as u8; 4];
    let in_b_u8_16 = [1 as u8; 16];
    let mut out_out_f32_4 = [123.0 as f32; 4];
    let mut out_out_f64_4 = [123.0 as f64; 4];
    let mut out_out_i16_4 = [0x55 as i16; 4];
    let mut out_out_i32_4 = [0x55 as i32; 4];
    let mut out_out_i64_4 = [0x55 as i64; 4];
    let mut out_out_i8_4 = [0x55 as i8; 4];
    let mut out_out_u16_4 = [0x55 as u16; 4];
    let mut out_out_u16_8 = [0x55 as u16; 8];
    let mut out_out_u16_12 = [0x55 as u16; 12];
    let mut out_out_u16_16 = [0x55 as u16; 16];
    let mut out_out_u32_4 = [0x55 as u32; 4];
    let mut out_out_u32_8 = [0x55 as u32; 8];
    let mut out_out_u32_12 = [0x55 as u32; 12];
    let mut out_out_u32_16 = [0x55 as u32; 16];
    let mut out_out_u64_2 = [0x55 as u64; 2];
    let mut out_out_u64_4 = [0x55 as u64; 4];
    let mut out_out_u64_8 = [0x55 as u64; 8];
    let mut out_out_u64_12 = [0x55 as u64; 12];
    let mut out_out_u64_16 = [0x55 as u64; 16];
    let mut out_out_u8_4 = [0x55 as u8; 4];
    let mut out_out0_u16_4 = [0x55 as u16; 4];
    let mut out_out0_u32_4 = [0x55 as u32; 4];
    let mut out_out0_u64_4 = [0x55 as u64; 4];
    let mut out_out1_u16_4 = [0x55 as u16; 4];
    let mut out_out1_u32_4 = [0x55 as u32; 4];
    let mut out_out1_u64_4 = [0x55 as u64; 4];
    let mut out_out2_u16_4 = [0x55 as u16; 4];
    let mut out_out2_u32_4 = [0x55 as u32; 4];
    let mut out_out2_u64_4 = [0x55 as u64; 4];
    let mut out_out3_u16_4 = [0x55 as u16; 4];
    let mut out_out3_u32_4 = [0x55 as u32; 4];
    let mut out_out3_u64_4 = [0x55 as u64; 4];
    unsafe { sve::abs_i16(&in_a_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::add_u16(&in_a_u16_4, &in_b_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::add_u32(&in_a_u32_4, &in_b_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::add_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::and_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::and_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::asr_i16(&in_a_i16_4, 0 as i32, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::asr_i8(&in_a_i8_4, 0 as i32, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::clz_i64(&in_a_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::cnt_i64(&in_a_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::div_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::div_u32(&in_a_u32_4, &in_b_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::div_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::eor_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::eor_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::fabs_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fabs_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fadd_f32(&in_a_f32_4, &in_b_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fadd_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fcadd_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fcmla_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fcvtzs_i32_f64(&in_a_f64_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::fcvtzs_i64_f32(&in_a_f32_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::fcvtzs_i64_f64(&in_a_f64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::fcvtzu_u32_f32(&in_a_f32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::fcvtzu_u64_f32(&in_a_f32_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::fcvtzu_u64_f64(&in_a_f64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::fdiv_f32(&in_a_f32_4, &in_b_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fdiv_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fmad_f32(&in_a_f32_4, &in_b_f32_4, &in_acc_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fmad_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fmax_f32(&in_a_f32_4, &in_b_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fmax_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fmin_f32(&in_a_f32_4, &in_b_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fmin_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fmov_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fmsb_f32(&in_a_f32_4, &in_b_f32_4, &in_acc_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fmsb_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fmul_f32(&in_a_f32_4, &in_b_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fmul_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fneg_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fneg_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fnmad_f32(&in_a_f32_4, &in_b_f32_4, &in_acc_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fnmad_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fnmla_f32(&in_a_f32_4, &in_b_f32_4, &in_acc_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fnmla_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fnmls_f32(&in_a_f32_4, &in_b_f32_4, &in_acc_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fnmls_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fnmsb_f32(&in_a_f32_4, &in_b_f32_4, &in_acc_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fnmsb_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frecpe_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frecps_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frinta_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::frinta_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frinti_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::frinti_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frintm_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::frintm_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frintn_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::frintn_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frintp_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::frintp_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frintx_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::frintx_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frintz_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::frintz_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frsqrte_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::frsqrts_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fscale_f32(&in_a_f32_4, &in_b_i32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fscale_f64(&in_a_f64_4, &in_b_i64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fsqrt_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fsqrt_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::fsub_f32(&in_a_f32_4, &in_b_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::fsub_f64(&in_a_f64_4, &in_b_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::ftmad_f32(&in_a_f32_4, &in_b_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::ftssel_f32(&in_a_f32_4, &in_b_u32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::insr_i32(0 as i32, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::ld1_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::ld1_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::ld1_u16(&in_a_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::ld1_u32(&in_a_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::ld1_u64(&in_a_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::ld1rd_u64(&in_a_u64_1, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::ld1rh_u16(&in_a_u16_1, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::ld1rqd_u64(&in_a_u64_2, &mut out_out_u64_2) };
    core::hint::black_box(&out_out_u64_2);
    unsafe { sve::ld1rqh_u16(&in_a_u16_8, &mut out_out_u16_8) };
    core::hint::black_box(&out_out_u16_8);
    unsafe { sve::ld1rqw_u32(&in_a_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::ld1rsh_i16(&in_a_i16_8, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::ld1rsw_i32(&in_a_i32_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::ld1rw_u32(&in_a_u32_1, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::ld1sh_i16(&in_a_i16_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::ld1sw_i32(&in_a_i32_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::ld2_u32(&in_a_u32_8, &mut out_out0_u32_4, &mut out_out1_u32_4) };
    core::hint::black_box(&out_out0_u32_4);
    core::hint::black_box(&out_out1_u32_4);
    unsafe { sve::ld2d_u64(&in_a_u64_8, &mut out_out0_u64_4, &mut out_out1_u64_4) };
    core::hint::black_box(&out_out0_u64_4);
    core::hint::black_box(&out_out1_u64_4);
    unsafe { sve::ld2h_u16(&in_a_u16_8, &mut out_out0_u16_4, &mut out_out1_u16_4) };
    core::hint::black_box(&out_out0_u16_4);
    core::hint::black_box(&out_out1_u16_4);
    unsafe { sve::ld2w_u32(&in_a_u32_8, &mut out_out0_u32_4, &mut out_out1_u32_4) };
    core::hint::black_box(&out_out0_u32_4);
    core::hint::black_box(&out_out1_u32_4);
    unsafe {
        sve::ld3_u32(
            &in_a_u32_12,
            &mut out_out0_u32_4,
            &mut out_out1_u32_4,
            &mut out_out2_u32_4,
        )
    };
    core::hint::black_box(&out_out0_u32_4);
    core::hint::black_box(&out_out1_u32_4);
    core::hint::black_box(&out_out2_u32_4);
    unsafe {
        sve::ld3d_u64(
            &in_a_u64_12,
            &mut out_out0_u64_4,
            &mut out_out1_u64_4,
            &mut out_out2_u64_4,
        )
    };
    core::hint::black_box(&out_out0_u64_4);
    core::hint::black_box(&out_out1_u64_4);
    core::hint::black_box(&out_out2_u64_4);
    unsafe {
        sve::ld3h_u16(
            &in_a_u16_12,
            &mut out_out0_u16_4,
            &mut out_out1_u16_4,
            &mut out_out2_u16_4,
        )
    };
    core::hint::black_box(&out_out0_u16_4);
    core::hint::black_box(&out_out1_u16_4);
    core::hint::black_box(&out_out2_u16_4);
    unsafe {
        sve::ld3w_u32(
            &in_a_u32_12,
            &mut out_out0_u32_4,
            &mut out_out1_u32_4,
            &mut out_out2_u32_4,
        )
    };
    core::hint::black_box(&out_out0_u32_4);
    core::hint::black_box(&out_out1_u32_4);
    core::hint::black_box(&out_out2_u32_4);
    unsafe {
        sve::ld4_u32(
            &in_a_u32_16,
            &mut out_out0_u32_4,
            &mut out_out1_u32_4,
            &mut out_out2_u32_4,
            &mut out_out3_u32_4,
        )
    };
    core::hint::black_box(&out_out0_u32_4);
    core::hint::black_box(&out_out1_u32_4);
    core::hint::black_box(&out_out2_u32_4);
    core::hint::black_box(&out_out3_u32_4);
    unsafe {
        sve::ld4d_u64(
            &in_a_u64_16,
            &mut out_out0_u64_4,
            &mut out_out1_u64_4,
            &mut out_out2_u64_4,
            &mut out_out3_u64_4,
        )
    };
    core::hint::black_box(&out_out0_u64_4);
    core::hint::black_box(&out_out1_u64_4);
    core::hint::black_box(&out_out2_u64_4);
    core::hint::black_box(&out_out3_u64_4);
    unsafe {
        sve::ld4h_u16(
            &in_a_u16_16,
            &mut out_out0_u16_4,
            &mut out_out1_u16_4,
            &mut out_out2_u16_4,
            &mut out_out3_u16_4,
        )
    };
    core::hint::black_box(&out_out0_u16_4);
    core::hint::black_box(&out_out1_u16_4);
    core::hint::black_box(&out_out2_u16_4);
    core::hint::black_box(&out_out3_u16_4);
    unsafe {
        sve::ld4w_u32(
            &in_a_u32_16,
            &mut out_out0_u32_4,
            &mut out_out1_u32_4,
            &mut out_out2_u32_4,
            &mut out_out3_u32_4,
        )
    };
    core::hint::black_box(&out_out0_u32_4);
    core::hint::black_box(&out_out1_u32_4);
    core::hint::black_box(&out_out2_u32_4);
    core::hint::black_box(&out_out3_u32_4);
    unsafe { sve::ldff1d_u64(&in_a_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::ldff1h_u16(&in_a_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::ldff1w_u32(&in_a_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::ldnf1d_u64(&in_a_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::ldnf1h_u16(&in_a_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::ldnf1w_u32(&in_a_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::ldnt1d_u64(&in_a_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::ldnt1h_u16(&in_a_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::ldnt1w_u32(&in_a_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::lsl_i16(&in_a_i16_4, 0 as i32, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::lsl_i64(&in_a_i64_4, 0 as i32, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::lsr_i64(&in_a_i64_4, 0 as i32, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::lsr_i8(&in_a_i8_4, 0 as i32, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::mad_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::msb_f64(&in_a_f64_4, &in_b_f64_4, &in_acc_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::mul_u16(&in_a_u16_4, &in_b_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::mul_u32(&in_a_u32_4, &in_b_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::mul_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::mul_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u8_4) };
    core::hint::black_box(&out_out_u8_4);
    unsafe { sve::neg_i64(&in_a_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::neg_i8(&in_a_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::not_i16(&in_a_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::not_i8(&in_a_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::orr_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::orr_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::punpkhi_u8(&in_a_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::punpklo_u8(&in_a_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::raddhnb_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::raddhnt_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::reduce_add_i16(&in_a_i16_4) };
    unsafe { sve::reduce_add_i64(&in_a_i64_4) };
    unsafe { sve::reduce_add_u16(&in_a_u16_4) };
    unsafe { sve::reduce_add_u64(&in_a_u64_4) };
    unsafe { sve::reduce_add_u8(&in_a_u8_4) };
    unsafe { sve::reduce_max_i64(&in_a_i64_4) };
    unsafe { sve::reduce_max_i8(&in_a_i8_4) };
    unsafe { sve::reduce_max_u16(&in_a_u16_4) };
    unsafe { sve::reduce_max_u32(&in_a_u32_4) };
    unsafe { sve::reduce_max_u64(&in_a_u64_4) };
    unsafe { sve::reduce_max_u8(&in_a_u8_4) };
    unsafe { sve::reduce_maxnm_f64(&in_a_f64_4) };
    unsafe { sve::reduce_min_i16(&in_a_i16_4) };
    unsafe { sve::reduce_min_i64(&in_a_i64_4) };
    unsafe { sve::reduce_min_i8(&in_a_i8_4) };
    unsafe { sve::reduce_min_u16(&in_a_u16_4) };
    unsafe { sve::reduce_min_u64(&in_a_u64_4) };
    unsafe { sve::reduce_min_u8(&in_a_u8_4) };
    unsafe { sve::reduce_minnm_f32(&in_a_f32_4) };
    unsafe { sve::rev_i32(&in_a_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::revw_i64(&in_a_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::rsubhnb_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::rsubhnt_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::saba_i16(&in_a_i16_4, &in_b_i16_4, &in_acc_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::saba_i32(&in_a_i32_4, &in_b_i32_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::saba_i8(&in_a_i8_4, &in_b_i8_4, &in_acc_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::sabalb_i8(&in_a_i8_4, &in_b_i8_4, &in_acc_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sabalt_i8(&in_a_i8_4, &in_b_i8_4, &in_acc_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sabd_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sabd_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sabd_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::sabdlb_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sabdlt_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sadalp_i16(&in_a_i16_8, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::saddlbt_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::saddlt_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::saddwb_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::saddwt_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::scale_f64(&in_a_f64_4, &in_b_i64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::scvtf_f32_i64(&in_a_i64_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::scvtf_f64_i32(&in_a_i32_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::scvtf_f64_i64(&in_a_i64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::smax_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::smax_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::smin_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::smin_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::smin_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::smlalt_i8(&in_a_i8_4, &in_b_i8_4, &in_acc_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::smlslb_i8(&in_a_i8_4, &in_b_i8_4, &in_acc_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::smlslt_i8(&in_a_i8_4, &in_b_i8_4, &in_acc_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::smulh_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::smullt_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::splice_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqabs_i16(&in_a_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sqabs_i32(&in_a_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqabs_i64(&in_a_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::sqadd_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sqadd_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqadd_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::sqcadd_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmlalb_i16(&in_a_i16_4, &in_b_i16_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmlalbt_i16(&in_a_i16_4, &in_b_i16_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmlalt_i16(&in_a_i16_4, &in_b_i16_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmlslb_i16(&in_a_i16_4, &in_b_i16_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmlslbt_i16(&in_a_i16_4, &in_b_i16_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmlslt_i16(&in_a_i16_4, &in_b_i16_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmulh_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmullb_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqdmullt_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqneg_i32(&in_a_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqneg_i64(&in_a_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::sqneg_i8(&in_a_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::sqrdmlah_i32(&in_a_i32_4, &in_b_i32_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqrdmlsh_i32(&in_a_i32_4, &in_b_i32_4, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqrdmulh_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sqrdmulh_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqrshrnb_i32(&in_a_i32_4, 0 as i32, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sqrshrnt_i32(&in_acc_i16_4, &in_a_i32_4, 0 as i32, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sqrshrunb_i32(&in_a_i32_4, 0 as i32, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::sqrshrunt_i32(&in_acc_u16_4, &in_a_i32_4, 0 as i32, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::sqshrnb_i32(&in_a_i32_4, 0 as i32, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sqshrnt_i32(&in_acc_i16_4, &in_a_i32_4, 0 as i32, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sqshrunb_i32(&in_a_i32_4, 0 as i32, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::sqshrunt_i32(&in_acc_u16_4, &in_a_i32_4, 0 as i32, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::sqsub_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sqsub_i64(&in_a_i64_4, &in_b_i64_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::sqsub_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::sqxtnt_i32(&in_acc_i16_4, &in_a_i32_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sqxtunt_i32(&in_acc_u16_4, &in_a_i32_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::srhadd_i16(&in_a_i16_4, &in_b_i16_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::srhadd_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::srhadd_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i8_4) };
    core::hint::black_box(&out_out_i8_4);
    unsafe { sve::ssublb_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::ssublbt_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::ssublt_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::ssubltb_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::ssubwb_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::ssubwt_i8(&in_a_i8_4, &in_b_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::st1_f32(&in_a_f32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::st1_f64(&in_a_f64_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::st1_u16(&in_a_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::st1_u32(&in_a_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::st1_u64(&in_a_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::st2_u32(&in_a0_u32_4, &in_a1_u32_4, &mut out_out_u32_8) };
    core::hint::black_box(&out_out_u32_8);
    unsafe { sve::st2d_u64(&in_a0_u64_4, &in_a1_u64_4, &mut out_out_u64_8) };
    core::hint::black_box(&out_out_u64_8);
    unsafe { sve::st2h_u16(&in_a0_u16_4, &in_a1_u16_4, &mut out_out_u16_8) };
    core::hint::black_box(&out_out_u16_8);
    unsafe { sve::st2w_u32(&in_a0_u32_4, &in_a1_u32_4, &mut out_out_u32_8) };
    core::hint::black_box(&out_out_u32_8);
    unsafe {
        sve::st3_u32(
            &in_a0_u32_4,
            &in_a1_u32_4,
            &in_a2_u32_4,
            &mut out_out_u32_12,
        )
    };
    core::hint::black_box(&out_out_u32_12);
    unsafe {
        sve::st3d_u64(
            &in_a0_u64_4,
            &in_a1_u64_4,
            &in_a2_u64_4,
            &mut out_out_u64_12,
        )
    };
    core::hint::black_box(&out_out_u64_12);
    unsafe {
        sve::st3h_u16(
            &in_a0_u16_4,
            &in_a1_u16_4,
            &in_a2_u16_4,
            &mut out_out_u16_12,
        )
    };
    core::hint::black_box(&out_out_u16_12);
    unsafe {
        sve::st3w_u32(
            &in_a0_u32_4,
            &in_a1_u32_4,
            &in_a2_u32_4,
            &mut out_out_u32_12,
        )
    };
    core::hint::black_box(&out_out_u32_12);
    unsafe {
        sve::st4_u32(
            &in_a0_u32_4,
            &in_a1_u32_4,
            &in_a2_u32_4,
            &in_a3_u32_4,
            &mut out_out_u32_16,
        )
    };
    core::hint::black_box(&out_out_u32_16);
    unsafe {
        sve::st4d_u64(
            &in_a0_u64_4,
            &in_a1_u64_4,
            &in_a2_u64_4,
            &in_a3_u64_4,
            &mut out_out_u64_16,
        )
    };
    core::hint::black_box(&out_out_u64_16);
    unsafe {
        sve::st4h_u16(
            &in_a0_u16_4,
            &in_a1_u16_4,
            &in_a2_u16_4,
            &in_a3_u16_4,
            &mut out_out_u16_16,
        )
    };
    core::hint::black_box(&out_out_u16_16);
    unsafe {
        sve::st4w_u32(
            &in_a0_u32_4,
            &in_a1_u32_4,
            &in_a2_u32_4,
            &in_a3_u32_4,
            &mut out_out_u32_16,
        )
    };
    core::hint::black_box(&out_out_u32_16);
    unsafe { sve::stnt1b_u8(&in_a_u8_4, &mut out_out_u8_4) };
    core::hint::black_box(&out_out_u8_4);
    unsafe { sve::stnt1d_u64(&in_a_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::stnt1h_u16(&in_a_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::stnt1w_u32(&in_a_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::sub_u16(&in_a_u16_4, &in_b_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::sub_u32(&in_a_u32_4, &in_b_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::sub_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::sub_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u8_4) };
    core::hint::black_box(&out_out_u8_4);
    unsafe { sve::subr_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sunpklo_i8(&in_a_i8_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::sxth_i16(&in_a_i16_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::sxtw_i32(&in_a_i32_4, &mut out_out_i64_4) };
    core::hint::black_box(&out_out_i64_4);
    unsafe { sve::trn1_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::trn2_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::uaba_u16(&in_a_u16_4, &in_b_u16_4, &in_acc_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uaba_u32(&in_a_u32_4, &in_b_u32_4, &in_acc_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::uaba_u8(&in_a_u8_4, &in_b_u8_4, &in_acc_u8_4, &mut out_out_u8_4) };
    core::hint::black_box(&out_out_u8_4);
    unsafe { sve::uabalb_u8(&in_a_u8_4, &in_b_u8_4, &in_acc_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uabalt_u8(&in_a_u8_4, &in_b_u8_4, &in_acc_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uabd_u16(&in_a_u16_4, &in_b_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uabd_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::uabd_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u8_4) };
    core::hint::black_box(&out_out_u8_4);
    unsafe { sve::uabdlb_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uabdlt_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uadalp_u16(&in_a_u16_8, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::uaddlb_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uaddlt_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uaddwb_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uaddwt_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::ucvtf_f32_u32(&in_a_u32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::ucvtf_f32_u64(&in_a_u64_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::ucvtf_f64_u32(&in_a_u32_4, &mut out_out_f64_4) };
    core::hint::black_box(&out_out_f64_4);
    unsafe { sve::umax_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::umin_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::umlalb_u8(&in_a_u8_4, &in_b_u8_4, &in_acc_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::umlalt_u8(&in_a_u8_4, &in_b_u8_4, &in_acc_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::umlslb_u8(&in_a_u8_4, &in_b_u8_4, &in_acc_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::umlslt_u8(&in_a_u8_4, &in_b_u8_4, &in_acc_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::ummla_u32(&in_a_u8_16, &in_b_u8_16, &in_acc_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::umulh_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::umullb_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::umullt_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uqadd_u16(&in_a_u16_4, &in_b_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uqadd_u32(&in_a_u32_4, &in_b_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::uqadd_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::uqrshrnb_u32(&in_a_u32_4, 0 as i32, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uqrshrnt_u32(&in_acc_u16_4, &in_a_u32_4, 0 as i32, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uqshrnb_u32(&in_a_u32_4, 0 as i32, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uqshrnt_u32(&in_acc_u16_4, &in_a_u32_4, 0 as i32, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uqsub_u32(&in_a_u32_4, &in_b_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::uqsub_u64(&in_a_u64_4, &in_b_u64_4, &mut out_out_u64_4) };
    core::hint::black_box(&out_out_u64_4);
    unsafe { sve::uqsub_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u8_4) };
    core::hint::black_box(&out_out_u8_4);
    unsafe { sve::uqxtnt_u32(&in_acc_u16_4, &in_a_u32_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::urecpe_f32_u32(&in_a_u32_4, &mut out_out_f32_4) };
    core::hint::black_box(&out_out_f32_4);
    unsafe { sve::urhadd_u16(&in_a_u16_4, &in_b_u16_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::urhadd_u32(&in_a_u32_4, &in_b_u32_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::urhadd_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u8_4) };
    core::hint::black_box(&out_out_u8_4);
    unsafe { sve::usdot_i32(&in_a_u8_16, &in_b_i8_16, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::usmmla_i32(&in_a_u8_16, &in_b_i8_16, &in_acc_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::usublb_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::usublt_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::usubwb_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::usubwt_u8(&in_a_u8_4, &in_b_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uunpkhi_u8(&in_a_u8_4, &mut out_out_u16_4) };
    core::hint::black_box(&out_out_u16_4);
    unsafe { sve::uxtb_u8(&in_a_u8_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::uxth_u16(&in_a_u16_4, &mut out_out_u32_4) };
    core::hint::black_box(&out_out_u32_4);
    unsafe { sve::uzp1_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::uzp2_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::whilege_i32(0, 0) };
    unsafe { sve::whilegt_u32(0, 0) };
    unsafe { sve::whilehi_i32(0, 0) };
    unsafe { sve::whilehi_u32(0, 0) };
    unsafe { sve::whilehs_i32(0, 0) };
    unsafe { sve::whilehs_u32(0, 0) };
    unsafe { sve::whilele_i32(0, 0) };
    unsafe { sve::whilelo_i32(0, 0) };
    unsafe { sve::whilelo_u32(0, 0) };
    unsafe { sve::whilels_i32(0, 0) };
    unsafe { sve::whilels_u32(0, 0) };
    unsafe { sve::whilelt_u32(0, 0) };
    unsafe { sve::xtn_i32(&in_a_i32_4, &mut out_out_i16_4) };
    core::hint::black_box(&out_out_i16_4);
    unsafe { sve::zip1_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    unsafe { sve::zip2_i32(&in_a_i32_4, &in_b_i32_4, &mut out_out_i32_4) };
    core::hint::black_box(&out_out_i32_4);
    true
}

fn remaining_ops() -> bool {
    if cnt_u64(0xff) != 8 || clz_u64(1) != 63 || cls_u64(1) != 62 {
        return false;
    }
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    if ext::<4>(a, b).to_bytes() != core::array::from_fn::<u8, 16, _>(|index| index as u8 + 4) {
        return false;
    }
    if splat_i8(0x7f).to_bytes() != [0x7f; 16]
        || i16_lanes(splat_i16(0x1234)) != [0x1234; 8]
        || !i64_ok(splat_i64(5), [5, 5])
        || f32_lanes(splat_f32(1.5)) != [1.5; 4]
        || f64_lanes(splat_f64(2.5)) != [2.5; 2]
    {
        return false;
    }
    let low = |value: u64| {
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&value.to_le_bytes());
        V128::from_bytes(bytes)
    };
    if u64::from_le_bytes(
        unsafe { pmull_p64(low(1), low(1)) }.to_bytes()[..8]
            .try_into()
            .unwrap(),
    ) != 1
        || u64::from_le_bytes(
            unsafe { pmull_p64(low(2), low(3)) }.to_bytes()[..8]
                .try_into()
                .unwrap(),
        ) != 6
    {
        return false;
    }

    let pair_a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let pair_b = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    let pair = [pair_a, pair_b];
    let mut pair_bytes = [0u8; 32];
    unsafe { stp(pair_bytes.as_mut_ptr(), pair_a, pair_b) };
    let (load_a, load_b) = unsafe { ldp(pair_bytes.as_ptr()) };
    if load_a.to_bytes() != pair_a.to_bytes() || load_b.to_bytes() != pair_b.to_bytes() {
        return false;
    }
    let mut np_bytes = [0u8; 32];
    unsafe { stnp(np_bytes.as_mut_ptr(), pair_a, pair_b) };
    let (np_a, np_b) = unsafe { ldnp(np_bytes.as_ptr()) };
    if np_a.to_bytes() != pair_a.to_bytes() || np_b.to_bytes() != pair_b.to_bytes() {
        return false;
    }
    let _ = pair;

    let f16 = |values: [u16; 8]| {
        let mut bytes = [0u8; 16];
        for (lane, value) in values.iter().enumerate() {
            bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
        }
        V128::from_bytes(bytes)
    };
    let one = f16([0x3c00; 8]);
    let two = f16([0x4000; 8]);
    let three = f16([0x4200; 8]);
    let four = f16([0x4400; 8]);
    if unsafe { fsub_f16(four, one) } != three || unsafe { fdiv_f16(four, two) } != two {
        return false;
    }
    let low_half = f16([0x3c00, 0x4000, 0x4200, 0x4400, 0, 0, 0, 0]);
    if f32_lanes(unsafe { fcvtl_f32_f16(low_half) }) != [1.0, 2.0, 3.0, 4.0] {
        return false;
    }
    let full = f16([
        0x3c00, 0x4000, 0x4200, 0x4400, 0x4500, 0x4600, 0x4700, 0x4800,
    ]);
    if f32_lanes(unsafe { fcvtl2_f32_f16(full) }) != [5.0, 6.0, 7.0, 8.0] {
        return false;
    }
    if unsafe { fcvtn_f16_f32(f32x4([1.0, 2.0, 3.0, 4.0])) } != low_half {
        return false;
    }
    if unsafe { fcvtn2_f16_f32(f32x4([5.0, 6.0, 7.0, 8.0]), low_half) } != full {
        return false;
    }
    if unsafe { fcvt_f16_f32(1.0) } != 0x3c00
        || unsafe { fcvt_f32_f16(0x3c00) } != 1.0
        || unsafe { fcvt_f16_f64(1.0) } != 0x3c00
        || unsafe { fcvt_f64_f16(0x3c00) } != 1.0
    {
        return false;
    }
    let js = V128::from_bytes({
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&1.9f64.to_le_bytes());
        bytes
    });
    if unsafe { fjcvtzs_v128(js) } != 1 {
        return false;
    }
    let _: unsafe fn() -> rust_isa::aarch64::simd::sme::SmeGuard =
        rust_isa::aarch64::simd::sme::SmeGuard::enter;
    core::hint::black_box(unsafe { bfmmla(one, one, one) });
    core::hint::black_box(unsafe { fcmla_rot90(one, one, one) });
    core::hint::black_box(unsafe { fcvtxn_f16_f64(f64x2([1.0, 2.0])) });
    core::hint::black_box(unsafe { frecpx_f16(0x3c00) });
    core::hint::black_box(unsafe { xar::<4>(a, b) });
    true
}

#[target_feature(enable = "sve")]
unsafe fn sve_mnemonic_values() -> bool {
    let mut bytes = [0u8; 8];
    let mut out8 = [0u8; 8];
    let mut out_i8 = [0i8; 8];
    let mut out16 = [0i16; 4];
    let mut out32 = [0i32; 4];
    let mut out_f32 = [0.0f32; 4];
    unsafe {
        sve::add_i8(&[1, 2, 3], &[4, 5, 6], &mut out_i8[..3]);
        if out_i8[..3] != [5, 7, 9] {
            return false;
        }
        sve::mul_i32(&[2, 3], &[4, 5], &mut out32[..2]);
        if out32[..2] != [8, 15] {
            return false;
        }
        sve::saddlb_i8(&[1, 2], &[3, 4], &mut out16[..2]);
        if out16[..2] != [4, 6] {
            return false;
        }
        sve::sqadd_i8(&[100, -100], &[50, -50], &mut out_i8[..2]);
        if out_i8[..2] != [127, -128] {
            return false;
        }
        let mut narrow = [0i16; 4];
        sve::sqxtnb_i32(&[1, -1, 40_000, 70_000], &mut narrow);
        if narrow != [1, -1, i16::MAX, i16::MAX] {
            return false;
        }
        sve::fadd_f32(&[1.0, 2.0], &[3.0, 4.0], &mut out_f32[..2]);
        if out_f32[..2] != [4.0, 6.0] {
            return false;
        }
        sve::fdiv_f32(&[4.0], &[2.0], &mut out_f32[..1]);
        if out_f32[0] != 2.0 {
            return false;
        }
        sve::fcvtzs_i32_f32(&[1.0, -2.0], &mut out32[..2]);
        if out32[..2] != [1, -2] {
            return false;
        }
        sve::scvtf_f32_i32(&[1, 2], &mut out_f32[..2]);
        if out_f32[..2] != [1.0, 2.0] {
            return false;
        }
        if sve::reduce_add_i8(&[1, 2, 3]) != 6 {
            return false;
        }
        sve::ext_i32(&[1, 2, 3, 4], &[5, 6, 7, 8], 2, &mut out32);
        if out32 != [3, 4, 5, 6] {
            return false;
        }
        sve::trn1_i32(&[1, 2], &[3, 4], &mut out32[..2]);
        if out32[..2] != [1, 3] {
            return false;
        }
        sve::zip1_i32(&[1, 2], &[3, 4], &mut out32[..2]);
        if out32[..2] != [1, 3] {
            return false;
        }
        sve::uzp1_i32(&[1, 2], &[3, 4], &mut out32[..2]);
        if out32[..2] != [1, 0] {
            return false;
        }
        sve::ld1_u8(&[1, 2, 3, 4], &mut out8[..4]);
        if out8[..4] != [1, 2, 3, 4] {
            return false;
        }
        sve::st1_u8(&[1, 2, 3, 4], &mut bytes[..4]);
        if bytes[..4] != [1, 2, 3, 4] {
            return false;
        }
        let mut even = [0u8; 4];
        let mut odd = [0u8; 4];
        sve::ld2b_u8(&[1, 2, 3, 4, 5, 6, 7, 8], &mut even, &mut odd);
        if even != [1, 3, 5, 7] || odd != [2, 4, 6, 8] {
            return false;
        }
        let mut dot = [0i32; 1];
        sve::sdot_i32(&[1, 2, 3, 4], &[1, 1, 1, 1], &[0], &mut dot);
        if dot != [10] {
            return false;
        }
        if sve::whilelt_i32(0, 4) == 0 || sve::whilelt_i32(4, 4) != 0 {
            return false;
        }
    }
    true
}

#[target_feature(enable = "sve")]
unsafe fn sve_mnemonic_values2() -> bool {
    let mut out8 = [0i8; 4];
    let mut out_u8 = [0u8; 4];
    let mut out_u16 = [0u16; 4];
    let mut out_i16 = [0i16; 4];
    let mut out32 = [0i32; 4];
    let mut out_u32 = [0u32; 4];
    let mut out_f32 = [0.0f32; 4];
    unsafe {
        sve::sqsub_i8(&[-128, 127], &[1, -1], &mut out8[..2]);
        if out8[..2] != [-128, 127] {
            return false;
        }
        sve::uqadd_u8(&[250, 1], &[10, 2], &mut out_u8[..2]);
        if out_u8[..2] != [255, 3] {
            return false;
        }
        sve::sqxtunb_i32(&[1, -1, 70_000], &mut out_u16[..3]);
        if out_u16[..3] != [1, 0, u16::MAX] {
            return false;
        }
        sve::uaddlb_u8(&[1, 2], &[3, 4], &mut out_u16[..2]);
        if out_u16[..2] != [4, 6] {
            return false;
        }
        sve::saddlt_i8(&[1, 2], &[3, 4], &mut out_i16[..2]);
        if out_i16[..2] != [4, 6] {
            return false;
        }
        sve::fmax_f32(&[1.0, 4.0], &[3.0, 2.0], &mut out_f32[..2]);
        if out_f32[..2] != [3.0, 4.0] {
            return false;
        }
        sve::fmin_f32(&[1.0, 4.0], &[3.0, 2.0], &mut out_f32[..2]);
        if out_f32[..2] != [1.0, 2.0] {
            return false;
        }
        sve::fcmeq_f32(&[1.0, 2.0], &[1.0, 3.0], &mut out_u32[..2]);
        if out_u32[..2] != [u32::MAX, 0] {
            return false;
        }
        sve::fcvtzs_i32_f64(&[1.0, -2.0], &mut out32[..2]);
        if out32[..2] != [1, -2] {
            return false;
        }
        if sve::reduce_max_i8(&[1, 5, 3]) != 5 || sve::reduce_min_u16(&[4, 2, 9]) != 2 {
            return false;
        }
        sve::trn2_i32(&[1, 2, 3, 4], &[5, 6, 7, 8], &mut out32);
        if out32 != [2, 6, 4, 8] {
            return false;
        }
        sve::zip1_i32(&[1, 2, 3, 4], &[5, 6, 7, 8], &mut out32);
        if out32 != [1, 5, 2, 6] {
            return false;
        }
    }
    true
}

fn table() -> bool {
    let source = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let reverse = V128::from_bytes(core::array::from_fn(|index| 15 - index as u8));
    let reversed = V128::from_bytes(core::array::from_fn(|index| 15 - index as u8));
    if tbl(source, reverse).to_bytes() != reversed.to_bytes() {
        return false;
    }
    kernel_tbl(&source, &reverse).to_bytes() == source.to_bytes()
}

fn i16x8(values: [i16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i16_lanes(value: V128) -> [i16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f32_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    [
        f64::from_le_bytes(bytes[..8].try_into().unwrap()),
        f64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

fn integer_ops() -> bool {
    if add_i8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [3; 16] {
        return false;
    }
    if i16_lanes(sub_i16(i16x8([5; 8]), i16x8([2; 8]))) != [3; 8] {
        return false;
    }
    if lanes(sub_i32(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4]))) != [9, 18, 27, 36] {
        return false;
    }
    if !i64_ok(
        add_i64(
            V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]),
            V128::from_bytes([2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]),
        ),
        [3, 5],
    ) {
        return false;
    }
    if !i64_ok(
        sub_i64(
            V128::from_bytes([5, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]),
            V128::from_bytes([2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]),
        ),
        [3, 4],
    ) {
        return false;
    }
    if mul_i8(V128::from_bytes([0x10; 16]), V128::from_bytes([0x10; 16])).to_bytes() != [0; 16] {
        return false;
    }
    if i16_lanes(mul_i16(i16x8([3; 8]), i16x8([4; 8]))) != [12; 8] {
        return false;
    }
    if lanes(mul_i32(i32x4([2, 3, 4, 5]), i32x4([6, 7, 8, 9]))) != [12, 21, 32, 45] {
        return false;
    }
    let a = V128::from_bytes([0b1010_1010; 16]);
    let b = V128::from_bytes([0b1100_1100; 16]);
    if and(a, b).to_bytes() != [0b1000_1000; 16]
        || orr(a, b).to_bytes() != [0b1110_1110; 16]
        || eor(a, b).to_bytes() != [0b0110_0110; 16]
        || bic(a, b).to_bytes() != [0b0010_0010; 16]
        || orn(a, b).to_bytes() != [0b1011_1011; 16]
        || not(V128::from_bytes([0; 16])).to_bytes() != [0xff; 16]
    {
        return false;
    }
    if !i8_signed(
        smin_s8(
            V128::from_bytes([1, 0xff, 3, 0x80, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            V128::from_bytes([0; 16]),
        ),
        0xff,
    ) {
        return false;
    }
    if i16_lanes(smax_s16(i16x8([1, -2, 3, -4, 5, -6, 7, -8]), i16x8([0; 8])))
        != [1, 0, 3, 0, 5, 0, 7, 0]
    {
        return false;
    }
    if lanes(abs_i32(i32x4([-1, 2, -3, 4]))) != [1u32, 2, 3, 4]
        || lanes(neg_i32(i32x4([1, -2, 3, -4]))) != [u32::MAX, 2, u32::MAX - 2, 4]
    {
        return false;
    }
    if cmpeq_i8(V128::from_bytes([1; 16]), V128::from_bytes([1; 16])).to_bytes() != [0xff; 16]
        || cmpeq_i16(i16x8([1; 8]), i16x8([2; 8])).to_bytes() != [0; 16]
        || cmpeq_i32(i32x4([1; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
        || cmpgt_i8(V128::from_bytes([2; 16]), V128::from_bytes([1; 16])).to_bytes() != [0xff; 16]
        || cmpgt_i16(i16x8([1; 8]), i16x8([2; 8])).to_bytes() != [0; 16]
        || cmpgt_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
    {
        return false;
    }
    if i16_lanes(saddl_s8(
        V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]),
        V128::from_bytes([10, 20, 30, 40, 50, 60, 70, 80, 0, 0, 0, 0, 0, 0, 0, 0]),
    )) != [11, 22, 33, 44, 55, 66, 77, 88]
        || i16_lanes(uaddl_u8(
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]),
            V128::from_bytes([10, 20, 30, 40, 50, 60, 70, 80, 0, 0, 0, 0, 0, 0, 0, 0]),
        )) != [11, 22, 33, 44, 55, 66, 77, 88]
        || i16_lanes(ssubl_s8(
            V128::from_bytes([10, 20, 30, 40, 50, 60, 70, 80, 0, 0, 0, 0, 0, 0, 0, 0]),
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]),
        )) != [9, 18, 27, 36, 45, 54, 63, 72]
        || i16_lanes(usubl_u8(
            V128::from_bytes([10, 20, 30, 40, 50, 60, 70, 80, 0, 0, 0, 0, 0, 0, 0, 0]),
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]),
        )) != [9, 18, 27, 36, 45, 54, 63, 72]
        || lanes(saddl_s16(
            i16x8([1, 2, 3, 4, 0, 0, 0, 0]),
            i16x8([10, 20, 30, 40, 0, 0, 0, 0]),
        )) != [11, 22, 33, 44]
        || lanes(uaddl_u16(
            i16x8([1, 2, 3, 4, 0, 0, 0, 0]),
            i16x8([10, 20, 30, 40, 0, 0, 0, 0]),
        )) != [11, 22, 33, 44]
        || lanes(ssubl_s16(
            i16x8([10, 20, 30, 40, 0, 0, 0, 0]),
            i16x8([1, 2, 3, 4, 0, 0, 0, 0]),
        )) != [9, 18, 27, 36]
        || lanes(usubl_u16(
            i16x8([10, 20, 30, 40, 0, 0, 0, 0]),
            i16x8([1, 2, 3, 4, 0, 0, 0, 0]),
        )) != [9, 18, 27, 36]
        || !i64_ok(
            saddl_s32(i32x4([1, 2, 3, 4]), i32x4([10, 20, 30, 40])),
            [11, 22],
        )
        || !i64_ok(
            uaddl_u32(i32x4([1, 2, 3, 4]), i32x4([10, 20, 30, 40])),
            [11, 22],
        )
        || !i64_ok(
            ssubl_s32(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4])),
            [9, 18],
        )
        || !i64_ok(
            usubl_u32(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4])),
            [9, 18],
        )
    {
        return false;
    }
    if i16_lanes(saddw_s8(
        i16x8([1, 2, 3, 4, 5, 6, 7, 8]),
        V128::from_bytes([10, 20, 30, 40, 50, 60, 70, 80, 0, 0, 0, 0, 0, 0, 0, 0]),
    )) != [11, 22, 33, 44, 55, 66, 77, 88]
        || i16_lanes(uaddw_u8(
            i16x8([1, 2, 3, 4, 5, 6, 7, 8]),
            V128::from_bytes([10, 20, 30, 40, 50, 60, 70, 80, 0, 0, 0, 0, 0, 0, 0, 0]),
        )) != [11, 22, 33, 44, 55, 66, 77, 88]
        || i16_lanes(ssubw_s8(
            i16x8([100, 200, 300, 400, 500, 600, 700, 800]),
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]),
        )) != [99, 198, 297, 396, 495, 594, 693, 792]
        || i16_lanes(usubw_u8(
            i16x8([100, 200, 300, 400, 500, 600, 700, 800]),
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]),
        )) != [99, 198, 297, 396, 495, 594, 693, 792]
        || lanes(saddw_s16(
            i32x4([1, 2, 3, 4]),
            i16x8([10, 20, 30, 40, 0, 0, 0, 0]),
        )) != [11, 22, 33, 44]
        || lanes(uaddw_u16(
            i32x4([1, 2, 3, 4]),
            i16x8([10, 20, 30, 40, 0, 0, 0, 0]),
        )) != [11, 22, 33, 44]
        || lanes(ssubw_s16(
            i32x4([100, 200, 300, 400]),
            i16x8([1, 2, 3, 4, 0, 0, 0, 0]),
        )) != [99, 198, 297, 396]
        || lanes(usubw_u16(
            i32x4([100, 200, 300, 400]),
            i16x8([1, 2, 3, 4, 0, 0, 0, 0]),
        )) != [99, 198, 297, 396]
        || !i64_ok(saddw_s32(i64x2([1, 2]), i32x4([10, 20, 30, 40])), [11, 22])
        || !i64_ok(uaddw_u32(i64x2([1, 2]), i32x4([10, 20, 30, 40])), [11, 22])
        || !i64_ok(ssubw_s32(i64x2([100, 200]), i32x4([1, 2, 3, 4])), [99, 198])
        || !i64_ok(usubw_u32(i64x2([100, 200]), i32x4([1, 2, 3, 4])), [99, 198])
    {
        return false;
    }
    if cmge_i8(V128::from_bytes([2; 16]), V128::from_bytes([1; 16])).to_bytes() != [0xff; 16]
        || cmge_i8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0; 16]
        || cmge_i16(i16x8([2; 8]), i16x8([1; 8])).to_bytes() != [0xff; 16]
        || cmge_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
        || cmle_i8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0xff; 16]
        || cmle_i16(i16x8([1; 8]), i16x8([2; 8])).to_bytes() != [0xff; 16]
        || cmle_i32(i32x4([1; 4]), i32x4([2; 4])).to_bytes() != [0xff; 16]
        || cmlt_i8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0xff; 16]
        || cmlt_i16(i16x8([1; 8]), i16x8([2; 8])).to_bytes() != [0xff; 16]
        || cmlt_i32(i32x4([1; 4]), i32x4([2; 4])).to_bytes() != [0xff; 16]
        || cmtst_i8(V128::from_bytes([0xff; 16]), V128::from_bytes([1; 16])).to_bytes()
            != [0xff; 16]
        || cmtst_i8(V128::from_bytes([0x0f; 16]), V128::from_bytes([0xf0; 16])).to_bytes()
            != [0; 16]
        || cmtst_i16(i16x8([1; 8]), i16x8([3; 8])).to_bytes() != [0xff; 16]
        || cmtst_i32(i32x4([1; 4]), i32x4([3; 4])).to_bytes() != [0xff; 16]
    {
        return false;
    }
    if addp_s8(
        V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
        V128::from_bytes([0; 16]),
    )
    .to_bytes()
        != [3, 7, 11, 15, 19, 23, 27, 31, 0, 0, 0, 0, 0, 0, 0, 0]
        || i16_lanes(addp_s16(i16x8([1, 2, 3, 4, 5, 6, 7, 8]), i16x8([0; 8])))
            != [3, 7, 11, 15, 0, 0, 0, 0]
        || lanes(addp_s32(i32x4([1, 2, 3, 4]), i32x4([0; 4]))) != [3, 7, 0, 0]
        || addv_s8(V128::from_bytes([1; 16])) != 16
        || addv_s16(i16x8([1; 8])) != 8
        || addv_s32(i32x4([1; 4])) != 4
        || smaxv_s8(V128::from_bytes([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ])) != 16
        || sminv_s8(V128::from_bytes([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ])) != 1
        || smaxv_s16(i16x8([1, 2, 3, 4, 5, 6, 7, 8])) != 8
        || sminv_s16(i16x8([1, 2, 3, 4, 5, 6, 7, 8])) != 1
        || smaxv_s32(i32x4([1, 2, 3, 4])) != 4
        || sminv_s32(i32x4([1, 2, 3, 4])) != 1
        || umaxv_u8(V128::from_bytes([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ])) != 16
        || uminv_u8(V128::from_bytes([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ])) != 1
        || umaxv_u16(i16x8([1, 2, 3, 4, 5, 6, 7, 8])) != 8
        || uminv_u16(i16x8([1, 2, 3, 4, 5, 6, 7, 8])) != 1
        || umaxv_u32(i32x4([1, 2, 3, 4])) != 4
        || uminv_u32(i32x4([1, 2, 3, 4])) != 1
    {
        return false;
    }
    if smax_s8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [2; 16]
        || smin_s8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [1; 16]
        || i16_lanes(smax_s16(i16x8([1; 8]), i16x8([2; 8]))) != [2; 8]
        || i16_lanes(smin_s16(i16x8([1; 8]), i16x8([2; 8]))) != [1; 8]
        || lanes(smax_s32(i32x4([1; 4]), i32x4([2; 4]))) != [2; 4]
        || lanes(smin_s32(i32x4([1; 4]), i32x4([2; 4]))) != [1; 4]
        || umax_u8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [2; 16]
        || umin_u8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [1; 16]
        || i16_lanes(umax_u16(i16x8([1; 8]), i16x8([2; 8]))) != [2; 8]
        || i16_lanes(umin_u16(i16x8([1; 8]), i16x8([2; 8]))) != [1; 8]
        || lanes(umax_u32(i32x4([1; 4]), i32x4([2; 4]))) != [2; 4]
        || lanes(umin_u32(i32x4([1; 4]), i32x4([2; 4]))) != [1; 4]
        || smaxp_s8(
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            V128::from_bytes([0; 16]),
        )
        .to_bytes()
            != [2, 4, 6, 8, 10, 12, 14, 16, 0, 0, 0, 0, 0, 0, 0, 0]
        || sminp_s8(
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            V128::from_bytes([0; 16]),
        )
        .to_bytes()
            != [1, 3, 5, 7, 9, 11, 13, 15, 0, 0, 0, 0, 0, 0, 0, 0]
        || i16_lanes(smaxp_s16(i16x8([1, 2, 3, 4, 5, 6, 7, 8]), i16x8([0; 8])))
            != [2, 4, 6, 8, 0, 0, 0, 0]
        || i16_lanes(sminp_s16(i16x8([1, 2, 3, 4, 5, 6, 7, 8]), i16x8([0; 8])))
            != [1, 3, 5, 7, 0, 0, 0, 0]
        || lanes(smaxp_s32(i32x4([1, 2, 3, 4]), i32x4([0; 4]))) != [2, 4, 0, 0]
        || lanes(sminp_s32(i32x4([1, 2, 3, 4]), i32x4([0; 4]))) != [1, 3, 0, 0]
        || umaxp_u8(
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            V128::from_bytes([0; 16]),
        )
        .to_bytes()
            != [2, 4, 6, 8, 10, 12, 14, 16, 0, 0, 0, 0, 0, 0, 0, 0]
        || uminp_u8(
            V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            V128::from_bytes([0; 16]),
        )
        .to_bytes()
            != [1, 3, 5, 7, 9, 11, 13, 15, 0, 0, 0, 0, 0, 0, 0, 0]
        || i16_lanes(umaxp_u16(i16x8([1, 2, 3, 4, 5, 6, 7, 8]), i16x8([0; 8])))
            != [2, 4, 6, 8, 0, 0, 0, 0]
        || i16_lanes(uminp_u16(i16x8([1, 2, 3, 4, 5, 6, 7, 8]), i16x8([0; 8])))
            != [1, 3, 5, 7, 0, 0, 0, 0]
        || lanes(umaxp_u32(i32x4([1, 2, 3, 4]), i32x4([0; 4]))) != [2, 4, 0, 0]
        || lanes(uminp_u32(i32x4([1, 2, 3, 4]), i32x4([0; 4]))) != [1, 3, 0, 0]
        || mla_s8(
            V128::from_bytes([1; 16]),
            V128::from_bytes([2; 16]),
            V128::from_bytes([3; 16]),
        )
        .to_bytes()
            != [7; 16]
        || mls_s8(
            V128::from_bytes([1; 16]),
            V128::from_bytes([2; 16]),
            V128::from_bytes([3; 16]),
        )
        .to_bytes()
            != [0xfb; 16]
        || i16_lanes(mla_s16(i16x8([1; 8]), i16x8([2; 8]), i16x8([3; 8]))) != [7; 8]
        || i16_lanes(mls_s16(i16x8([1; 8]), i16x8([2; 8]), i16x8([3; 8]))) != [-5; 8]
        || lanes(mla_s32(i32x4([1; 4]), i32x4([2; 4]), i32x4([3; 4]))) != [7; 4]
        || lanes(mls_s32(i32x4([1; 4]), i32x4([2; 4]), i32x4([3; 4]))) != [u32::MAX - 4; 4]
    {
        return false;
    }
    if lanes(shl_i32(i32x4([1, 2, 3, 4]), 2)) != [4, 8, 12, 16]
        || i16_lanes(shr_i16(i16x8([2, 4, 6, 8, 10, 12, 14, 16]), 1)) != [1, 2, 3, 4, 5, 6, 7, 8]
    {
        return false;
    }
    if i8_all(
        sar_i8(
            V128::from_bytes([
                0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
                0x80, 0x80,
            ]),
            1,
        ),
        0xc0,
    ) == false
    {
        return false;
    }
    let splat = splat_i32(7);
    if lanes(splat) != [7; 4] || unsafe { extract_i32::<2>(splat) } != 7 {
        return false;
    }
    if lanes(replace_i32::<1>(splat, 9)) != [7, 9, 7, 7] {
        return false;
    }
    true
}

fn i64_ok(value: V128, expected: [i64; 2]) -> bool {
    let bytes = value.to_bytes();
    let lanes = [
        i64::from_le_bytes(bytes[..8].try_into().unwrap()),
        i64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ];
    lanes == expected
}

fn i8_signed(value: V128, expected: u8) -> bool {
    value.to_bytes()[1] == expected
}

fn i8_all(value: V128, expected: u8) -> bool {
    value.to_bytes().iter().all(|byte| *byte == expected)
}

fn float_ops() -> bool {
    let a = f32x4([8.0, 6.0, 4.0, 2.0]);
    let b = f32x4([2.0, 2.0, 2.0, 2.0]);
    if f32_lanes(sub_ps(a, b)) != [6.0, 4.0, 2.0, 0.0]
        || f32_lanes(div_ps(a, b)) != [4.0, 3.0, 2.0, 1.0]
        || f32_lanes(min_ps(a, b)) != [2.0, 2.0, 2.0, 2.0]
        || f32_lanes(max_ps(a, b)) != [8.0, 6.0, 4.0, 2.0]
        || f32_lanes(fabs(f32x4([-1.0, 2.0, -3.0, 4.0]))) != [1.0, 2.0, 3.0, 4.0]
        || f32_lanes(fneg(f32x4([1.0, -2.0, 3.0, -4.0]))) != [-1.0, 2.0, -3.0, 4.0]
        || f32_lanes(fsqrt(f32x4([4.0, 9.0, 16.0, 25.0]))) != [2.0, 3.0, 4.0, 5.0]
    {
        return false;
    }
    if f64_lanes(fadd_f64(f64x2([1.0, 2.0]), f64x2([3.0, 4.0]))) != [4.0, 6.0]
        || f64_lanes(fmul_f64(f64x2([2.0, 3.0]), f64x2([4.0, 5.0]))) != [8.0, 15.0]
    {
        return false;
    }
    if fcmeq_f32(f32x4([1.0, 2.0, 3.0, 4.0]), f32x4([1.0, 0.0, 3.0, 0.0])).to_bytes()
        != [
            0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
        ]
        || fcmge_f32(f32x4([2.0, 1.0, 3.0, 0.0]), f32x4([1.0, 1.0, 4.0, 1.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || fcmgt_f32(f32x4([2.0, 1.0, 3.0, 0.0]), f32x4([1.0, 1.0, 2.0, 1.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
            ]
        || fcmle_f32(f32x4([1.0, 1.0, 3.0, 0.0]), f32x4([1.0, 2.0, 2.0, 1.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff,
            ]
        || fcmlt_f32(f32x4([1.0, 2.0, 1.0, 0.0]), f32x4([2.0, 2.0, 0.0, 1.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff,
            ]
        || facge_f32(f32x4([-2.0, 1.0, -3.0, 0.0]), f32x4([1.0, 1.0, 4.0, 1.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || facgt_f32(f32x4([-2.0, 1.0, -3.0, 0.0]), f32x4([1.0, 1.0, 2.0, 1.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
            ]
    {
        return false;
    }
    if fcmeq_f64(f64x2([1.0, 2.0]), f64x2([1.0, 0.0])).to_bytes()
        != [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
        || fcmge_f64(f64x2([2.0, 1.0]), f64x2([1.0, 2.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || fcmgt_f64(f64x2([2.0, 1.0]), f64x2([1.0, 2.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || fcmle_f64(f64x2([1.0, 2.0]), f64x2([1.0, 1.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || fcmlt_f64(f64x2([1.0, 2.0]), f64x2([2.0, 2.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || facge_f64(f64x2([-2.0, 1.0]), f64x2([1.0, 2.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || facgt_f64(f64x2([-2.0, 1.0]), f64x2([1.0, 2.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
    {
        return false;
    }
    if f32_lanes(fabd_f32(
        f32x4([1.0, 2.0, 3.0, 4.0]),
        f32x4([2.0, 4.0, 6.0, 8.0]),
    )) != [1.0, 2.0, 3.0, 4.0]
        || f64_lanes(fabd_f64(f64x2([1.0, 2.0]), f64x2([3.0, 5.0]))) != [2.0, 3.0]
        || f32_lanes(faddp_f32(
            f32x4([1.0, 2.0, 3.0, 4.0]),
            f32x4([5.0, 6.0, 7.0, 8.0]),
        )) != [3.0, 7.0, 11.0, 15.0]
        || f64_lanes(faddp_f64(f64x2([1.0, 2.0]), f64x2([3.0, 4.0]))) != [3.0, 7.0]
        || faddv_f32(f32x4([1.0, 2.0, 3.0, 4.0])) != 10.0
        || faddv_f64(f64x2([1.0, 2.0])) != 3.0
        || f32_lanes(fmaxnm_f32(
            f32x4([1.0, 5.0, 3.0, 4.0]),
            f32x4([2.0, 4.0, 6.0, 3.0]),
        )) != [2.0, 5.0, 6.0, 4.0]
        || f64_lanes(fmaxnm_f64(f64x2([1.0, 5.0]), f64x2([2.0, 4.0]))) != [2.0, 5.0]
        || f32_lanes(fminnm_f32(
            f32x4([1.0, 5.0, 3.0, 4.0]),
            f32x4([2.0, 4.0, 6.0, 3.0]),
        )) != [1.0, 4.0, 3.0, 3.0]
        || f64_lanes(fminnm_f64(f64x2([1.0, 5.0]), f64x2([2.0, 4.0]))) != [1.0, 4.0]
        || fmaxnmv_f32(f32x4([1.0, 4.0, 2.0, 3.0])) != 4.0
        || fmaxnmv_f64(f64x2([1.0, 4.0])) != 4.0
        || fmaxv_f32(f32x4([1.0, 4.0, 2.0, 3.0])) != 4.0
        || fmaxv_f64(f64x2([1.0, 4.0])) != 4.0
        || fminnmv_f32(f32x4([2.0, 4.0, 1.0, 3.0])) != 1.0
        || fminnmv_f64(f64x2([2.0, 1.0])) != 1.0
        || fminv_f32(f32x4([2.0, 4.0, 1.0, 3.0])) != 1.0
        || fminv_f64(f64x2([2.0, 1.0])) != 1.0
    {
        return false;
    }
    if f32_lanes(fmla_f32(f32x4([2.0; 4]), f32x4([3.0; 4]), f32x4([1.0; 4]))) != [5.0; 4]
        || f64_lanes(fmla_f64(f64x2([2.0; 2]), f64x2([3.0; 2]), f64x2([1.0; 2]))) != [5.0; 2]
        || f32_lanes(fmls_f32(f32x4([2.0; 4]), f32x4([3.0; 4]), f32x4([1.0; 4]))) != [-1.0; 4]
        || f64_lanes(fmls_f64(f64x2([2.0; 2]), f64x2([3.0; 2]), f64x2([1.0; 2]))) != [-1.0; 2]
        || f32_lanes(fmulx_f32(
            f32x4([2.0, 3.0, 4.0, 5.0]),
            f32x4([6.0, 7.0, 8.0, 9.0]),
        )) != [12.0, 21.0, 32.0, 45.0]
        || f64_lanes(fmulx_f64(f64x2([2.0, 3.0]), f64x2([4.0, 5.0]))) != [8.0, 15.0]
    {
        return false;
    }
    true
}

fn shuffle_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    if zip1(a, b).to_bytes() != [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23] {
        return false;
    }
    if uzp1(a, b).to_bytes() != [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30] {
        return false;
    }
    if trn1(a, b).to_bytes() != [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30] {
        return false;
    }
    if zip2(a, b).to_bytes() != [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31] {
        return false;
    }
    if uzp2(a, b).to_bytes() != [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31] {
        return false;
    }
    if trn2(a, b).to_bytes() != [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31] {
        return false;
    }
    let idx = V128::from_bytes(core::array::from_fn(|index| index as u8));
    if tbl(a, idx).to_bytes() != a.to_bytes() {
        return false;
    }
    let out_of_range = V128::from_bytes([16; 16]);
    if rust_isa::aarch64::simd::neon::tbx(a, out_of_range).to_bytes() != [0; 16] {
        return false;
    }
    true
}

// Aliases keep the float test readable without importing every name twice.
fn sub_ps(a: V128, b: V128) -> V128 {
    fsub(a, b)
}
fn div_ps(a: V128, b: V128) -> V128 {
    fdiv(a, b)
}
fn min_ps(a: V128, b: V128) -> V128 {
    fmin(a, b)
}
fn max_ps(a: V128, b: V128) -> V128 {
    fmax(a, b)
}

#[target_feature(enable = "fp16")]
unsafe fn fp16_ops() -> bool {
    let one = V128::from_bytes({
        let mut bytes = [0u8; 16];
        for lane in 0..8 {
            bytes[lane * 2..lane * 2 + 2].copy_from_slice(&0x3c00u16.to_le_bytes());
        }
        bytes
    });
    let two = V128::from_bytes({
        let mut bytes = [0u8; 16];
        for lane in 0..8 {
            bytes[lane * 2..lane * 2 + 2].copy_from_slice(&0x4000u16.to_le_bytes());
        }
        bytes
    });
    let sum = unsafe { fadd_f16(one, two) }.to_bytes();
    let product = unsafe { fmul_f16(one, two) }.to_bytes();
    let fused = unsafe { fmla_f16(V128::from_bytes([0; 16]), one, two) }.to_bytes();
    let fused_sub = unsafe { fmls_f16(V128::from_bytes([0; 16]), one, two) }.to_bytes();
    u16::from_le_bytes(sum[0..2].try_into().unwrap()) == 0x4200
        && u16::from_le_bytes(product[0..2].try_into().unwrap()) == 0x4000
        && u16::from_le_bytes(fused[0..2].try_into().unwrap()) == 0x4000
        && u16::from_le_bytes(fused_sub[0..2].try_into().unwrap()) == 0xc000
}

#[target_feature(enable = "dotprod")]
unsafe fn dotprod_ops() -> bool {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    let zero = V128::from_bytes([0; 16]);
    let s = unsafe { sdot(a, b, zero) }.to_bytes();
    let u = unsafe { udot(a, b, zero) }.to_bytes();
    lanes(V128::from_bytes(s)) == [8; 4] && lanes(V128::from_bytes(u)) == [8; 4]
}

#[target_feature(enable = "bf16")]
unsafe fn bf16_ops() -> bool {
    let one = V128::from_bytes(core::array::from_fn(|index| {
        if index % 2 == 0 { 0x80 } else { 0x3f }
    }));
    let zero = V128::from_bytes([0; 16]);
    f32_lanes(unsafe { bfdot(zero, one, one) }) == [2.0; 4]
}

#[target_feature(enable = "i8mm")]
unsafe fn i8mm_ops() -> bool {
    let a = V128::from_bytes([1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2]);
    let b = V128::from_bytes([3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4]);
    let zero = V128::from_bytes([0; 16]);
    lanes(unsafe { smmla(zero, a, b) }) == [24, 32, 48, 64]
        && lanes(unsafe { usmmla(zero, a, b) }) == [24, 32, 48, 64]
        && lanes(unsafe { ummla(zero, a, b) }) == [24, 32, 48, 64]
}

#[target_feature(enable = "fcma")]
unsafe fn fcma_ops() -> bool {
    let acc = f32x4([1.0, 2.0, 3.0, 4.0]);
    let a = f32x4([2.0, 3.0, 4.0, 5.0]);
    let b = f32x4([6.0, 7.0, 8.0, 9.0]);
    let out = unsafe { fcmla(acc, a, b) };
    let rot180 = unsafe { fcmla_rot180(acc, a, b) };
    let rot270 = unsafe { fcmla_rot270(acc, a, b) };
    f32_lanes(out) == [13.0, 16.0, 35.0, 40.0]
        && f32_lanes(rot180) == [-11.0, -12.0, -29.0, -32.0]
        && f32_lanes(rot270) == [22.0, -16.0, 48.0, -36.0]
}

#[target_feature(enable = "fhm")]
unsafe fn fhm_ops() -> bool {
    let one = V128::from_bytes({
        let mut bytes = [0u8; 16];
        for lane in 0..8 {
            bytes[lane * 2..lane * 2 + 2].copy_from_slice(&0x3c00u16.to_le_bytes());
        }
        bytes
    });
    let two = V128::from_bytes({
        let mut bytes = [0u8; 16];
        for lane in 0..8 {
            bytes[lane * 2..lane * 2 + 2].copy_from_slice(&0x4000u16.to_le_bytes());
        }
        bytes
    });
    let zero = V128::from_bytes([0; 16]);
    f32_lanes(unsafe { fmlal_f16(zero, one, two) })[..2] == [2.0, 2.0]
        && f32_lanes(unsafe { fmlal2_f16(zero, one, two) })[..2] == [2.0, 2.0]
        && f32_lanes(unsafe { fmlsl_f16(zero, one, two) })[..2] == [-2.0, -2.0]
        && f32_lanes(unsafe { fmlsl2_f16(zero, one, two) })[..2] == [-2.0, -2.0]
}

#[target_feature(enable = "aes")]
unsafe fn crypto_aes_ops() -> bool {
    let one = V128::from_bytes([1; 16]);
    let zero = V128::from_bytes([0; 16]);
    unsafe { aese(one, one) }.to_bytes() != zero.to_bytes()
        && unsafe { aesd(one, one) }.to_bytes() != zero.to_bytes()
        && unsafe { aesmc(one) }.to_bytes() != zero.to_bytes()
        && unsafe { aesimc(one) }.to_bytes() != zero.to_bytes()
}

#[target_feature(enable = "sha2")]
unsafe fn crypto_sha2_ops() -> bool {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    let c = V128::from_bytes([3; 16]);
    let zero = V128::from_bytes([0; 16]);
    let h = unsafe { sha256h(a, b, c) };
    let h_swapped = unsafe { sha256h(a, c, b) };
    let h2 = unsafe { sha256h2(a, b, c) };
    h.to_bytes() != zero.to_bytes()
        && h2.to_bytes() != zero.to_bytes()
        && h.to_bytes() != h_swapped.to_bytes()
        && unsafe { sha256su0(a, b) }.to_bytes() != zero.to_bytes()
        && unsafe { sha256su1(a, b, c) }.to_bytes() != zero.to_bytes()
}

#[target_feature(enable = "sha3")]
unsafe fn crypto_sha3_ops() -> bool {
    let ones = V128::from_bytes([0xff; 16]);
    let zero = V128::from_bytes([0; 16]);
    if unsafe { eor3(ones, ones, ones) }.to_bytes() != ones.to_bytes()
        || unsafe { bcax(ones, ones, ones) }.to_bytes() != ones.to_bytes()
    {
        return false;
    }
    true
}

#[target_feature(enable = "sm4")]
unsafe fn crypto_sm4_ops() -> bool {
    let one = V128::from_bytes([1; 16]);
    let zero = V128::from_bytes([0; 16]);
    unsafe { sm4e(one, one) }.to_bytes() != zero.to_bytes()
        && unsafe { sm4ekey(one, one) }.to_bytes() != zero.to_bytes()
}

#[target_feature(enable = "crc")]
unsafe fn crypto_crc_ops() -> bool {
    let mut crc = 0xffff_ffffu32;
    for byte in b"123456789" {
        crc = unsafe { crc32b(crc, *byte) };
    }
    let value16 = 0x1234u16;
    let value32 = 0x1234_5678u32;
    let value64 = 0x0123_4567_89ab_cdefu64;
    crc == 0x340b_c6d9
        && unsafe { crc32h(0xffff_ffff, value16) } == crc32_ref(0xffff_ffff, &value16.to_le_bytes())
        && unsafe { crc32w(0xffff_ffff, value32) } == crc32_ref(0xffff_ffff, &value32.to_le_bytes())
        && unsafe { crc32x(0xffff_ffff, value64) } == crc32_ref(0xffff_ffff, &value64.to_le_bytes())
}

fn crc32_ref(mut crc: u32, bytes: &[u8]) -> u32 {
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    crc
}

fn neon_bitwise_ops() -> bool {
    let ones = V128::from_bytes([0xff; 16]);
    let zeros = V128::from_bytes([0; 16]);
    let a = V128::from_bytes([0xaa; 16]);
    let b = V128::from_bytes([0x55; 16]);
    if bsl(ones, a, b) != a || bsl(zeros, a, b) != b {
        return false;
    }
    if bit(zeros, ones, V128::from_bytes([0x0f; 16])) != V128::from_bytes([0x0f; 16]) {
        return false;
    }
    if bif(a, b, V128::from_bytes([0x0f; 16])) != V128::from_bytes([0x5a; 16]) {
        return false;
    }
    if cls_i8(zeros).to_bytes() != [7; 16]
        || cls_i16(zeros).to_bytes() != [15, 0, 15, 0, 15, 0, 15, 0, 15, 0, 15, 0, 15, 0, 15, 0]
        || cls_i32(zeros).to_bytes() != [31, 0, 0, 0, 31, 0, 0, 0, 31, 0, 0, 0, 31, 0, 0, 0]
    {
        return false;
    }
    if clz_i8(V128::from_bytes([1; 16])).to_bytes() != [7; 16] {
        return false;
    }
    if cnt_i8(ones).to_bytes() != [8; 16] {
        return false;
    }
    let mut single = [0u8; 16];
    single[0] = 1;
    if rbit(V128::from_bytes(single)).to_bytes()[0] != 0x80 {
        return false;
    }
    let source = V128::from_bytes(core::array::from_fn(|index| index as u8));
    if rev16(source).to_bytes() != [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14] {
        return false;
    }
    if rev32(source).to_bytes() != [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12] {
        return false;
    }
    if rev64(source).to_bytes() != [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8] {
        return false;
    }
    if cmpeq_i64(ones, ones) != ones {
        return false;
    }
    let mut low = [0u8; 16];
    low[0] = 1;
    if cmpgt_i64(V128::from_bytes(low), zeros)
        != V128::from_bytes([
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    {
        return false;
    }
    true
}

#[target_feature(enable = "sve")]
unsafe fn sve_ops() -> bool {
    let a = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let b = [10.0f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0];
    let mut out = [0.0f32; 7];
    unsafe { sve::add_f32(&a, &b, &mut out) };
    if out != [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0] {
        return false;
    }
    unsafe { sve::sub_f32(&b, &a, &mut out) };
    if out != [9.0, 18.0, 27.0, 36.0, 45.0, 54.0, 63.0] {
        return false;
    }
    let ai = [1i32, 2, 3, 4, 5];
    let bi = [10i32, 20, 30, 40, 50];
    let mut oi = [0i32; 5];
    unsafe { sve::sub_i32(&bi, &ai, &mut oi) };
    if oi != [9, 18, 27, 36, 45] {
        return false;
    }
    let ad = [1.0f64, 2.0, 3.0];
    let bd = [10.0f64, 20.0, 30.0];
    let mut od = [0.0f64; 3];
    unsafe { sve::sub_f64(&bd, &ad, &mut od) };
    if od != [9.0, 18.0, 27.0] {
        return false;
    }
    unsafe { sve::add_f64(&ad, &bd, &mut od) };
    if od != [11.0, 22.0, 33.0] {
        return false;
    }
    unsafe { sve::mul_f64(&ad, &bd, &mut od) };
    if od != [10.0, 40.0, 90.0] {
        return false;
    }
    unsafe { sve::div_f32(&b, &a, &mut out[..7]) };
    if out != [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0] {
        return false;
    }
    let mut oi = [0i32; 5];
    unsafe { sve::div_i32(&bi, &ai, &mut oi) };
    if oi != [10, 10, 10, 10, 10] {
        return false;
    }
    let mut od = [0.0f64; 3];
    unsafe { sve::div_f64(&bd, &ad, &mut od) };
    if od != [10.0, 10.0, 10.0] {
        return false;
    }
    unsafe { sve::min_f32(&a, &b, &mut out) };
    if out != [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0] {
        return false;
    }
    unsafe { sve::max_f32(&a, &b, &mut out) };
    if out != [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0] {
        return false;
    }
    unsafe { sve::min_i32(&ai, &bi, &mut oi) };
    if oi != [1, 2, 3, 4, 5] {
        return false;
    }
    unsafe { sve::max_i32(&ai, &bi, &mut oi) };
    if oi != [10, 20, 30, 40, 50] {
        return false;
    }
    unsafe { sve::min_f64(&ad, &bd, &mut od) };
    if od != [1.0, 2.0, 3.0] {
        return false;
    }
    unsafe { sve::max_f64(&ad, &bd, &mut od) };
    if od != [10.0, 20.0, 30.0] {
        return false;
    }
    let neg = [-1.0f32, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0];
    unsafe { sve::abs_f32(&neg, &mut out) };
    if out != [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0] {
        return false;
    }
    unsafe { sve::neg_f32(&a, &mut out) };
    if out != [-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0] {
        return false;
    }
    unsafe { sve::abs_i32(&[-1, -2, -3, -4, -5], &mut oi) };
    if oi != [1, 2, 3, 4, 5] {
        return false;
    }
    unsafe { sve::neg_i32(&ai, &mut oi) };
    if oi != [-1, -2, -3, -4, -5] {
        return false;
    }
    unsafe { sve::abs_f64(&[-1.0, -2.0, -3.0], &mut od) };
    if od != [1.0, 2.0, 3.0] {
        return false;
    }
    unsafe { sve::neg_f64(&ad, &mut od) };
    if od != [-1.0, -2.0, -3.0] {
        return false;
    }
    unsafe { sve::sqrt_f32(&[4.0, 9.0, 16.0], &mut out[..3]) };
    if out[..3] != [2.0, 3.0, 4.0] {
        return false;
    }
    unsafe { sve::sqrt_f64(&[4.0, 9.0, 16.0], &mut od) };
    if od != [2.0, 3.0, 4.0] {
        return false;
    }
    unsafe { sve::and_i32(&[0b1010, 0b1100], &[0b1100, 0b1010], &mut oi[..2]) };
    if oi[..2] != [0b1000, 0b1000] {
        return false;
    }
    unsafe { sve::or_i32(&[0b1010, 0b1100], &[0b1100, 0b1010], &mut oi[..2]) };
    if oi[..2] != [0b1110, 0b1110] {
        return false;
    }
    unsafe { sve::xor_i32(&[0b1010, 0b1100], &[0b1100, 0b1010], &mut oi[..2]) };
    if oi[..2] != [0b0110, 0b0110] {
        return false;
    }
    unsafe { sve::fma_f32(&[2.0, 3.0], &[4.0, 5.0], &[1.0, 2.0], &mut out[..2]) };
    if out[..2] != [9.0, 17.0] {
        return false;
    }
    unsafe { sve::fma_f64(&[2.0, 3.0], &[4.0, 5.0], &[1.0, 2.0], &mut od[..2]) };
    if od[..2] != [9.0, 17.0] {
        return false;
    }
    unsafe { sve::recip_f32(&[1.0, 2.0, 4.0], &mut out[..3]) };
    if out[..3]
        .iter()
        .any(|value| *value == 0.0 || !value.is_finite())
    {
        return false;
    }
    unsafe { sve::recip_f64(&[1.0, 2.0, 4.0], &mut od) };
    if od.iter().any(|value| *value == 0.0 || !value.is_finite()) {
        return false;
    }
    unsafe { sve::rsqrt_f32(&[1.0, 4.0, 16.0], &mut out[..3]) };
    if out[..3]
        .iter()
        .any(|value| *value == 0.0 || !value.is_finite())
    {
        return false;
    }
    unsafe { sve::rsqrt_f64(&[1.0, 4.0, 16.0], &mut od) };
    if od.iter().any(|value| *value == 0.0 || !value.is_finite()) {
        return false;
    }
    unsafe { sve::floor_f32(&[1.5, -1.5, 2.9], &mut out[..3]) };
    if out[..3] != [1.0, -2.0, 2.0] {
        return false;
    }
    unsafe { sve::ceil_f32(&[1.5, -1.5, 2.9], &mut out[..3]) };
    if out[..3] != [2.0, -1.0, 3.0] {
        return false;
    }
    unsafe { sve::floor_f64(&[1.5, -1.5, 2.9], &mut od) };
    if od != [1.0, -2.0, 2.0] {
        return false;
    }
    unsafe { sve::ceil_f64(&[1.5, -1.5, 2.9], &mut od) };
    if od != [2.0, -1.0, 3.0] {
        return false;
    }
    unsafe { sve::trunc_f32(&[1.9, -1.9, 2.1], &mut out[..3]) };
    if out[..3] != [1.0, -1.0, 2.0] {
        return false;
    }
    unsafe { sve::rint_f32(&[1.5, 2.5, -1.5], &mut out[..3]) };
    if out[..3] != [2.0, 2.0, -2.0] {
        return false;
    }
    unsafe { sve::trunc_f64(&[1.9, -1.9, 2.1], &mut od) };
    if od != [1.0, -1.0, 2.0] {
        return false;
    }
    unsafe { sve::rint_f64(&[1.5, 2.5, -1.5], &mut od) };
    if od != [2.0, 2.0, -2.0] {
        return false;
    }
    unsafe { sve::minnm_f32(&a, &b, &mut out) };
    if out != [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0] {
        return false;
    }
    unsafe { sve::maxnm_f32(&a, &b, &mut out) };
    if out != [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0] {
        return false;
    }
    unsafe { sve::minnm_f64(&ad, &bd, &mut od) };
    if od != [1.0, 2.0, 3.0] {
        return false;
    }
    unsafe { sve::maxnm_f64(&ad, &bd, &mut od) };
    if od != [10.0, 20.0, 30.0] {
        return false;
    }
    if unsafe { sve::reduce_add_f32(&a) } != 28.0
        || unsafe { sve::reduce_add_i32(&ai) } != 15
        || unsafe { sve::reduce_add_f64(&ad) } != 6.0
        || unsafe { sve::reduce_min_f32(&a) } != 1.0
        || unsafe { sve::reduce_max_f32(&a) } != 7.0
        || unsafe { sve::reduce_min_i32(&ai) } != 1
        || unsafe { sve::reduce_max_i32(&ai) } != 5
        || unsafe { sve::reduce_min_f64(&ad) } != 1.0
        || unsafe { sve::reduce_max_f64(&ad) } != 3.0
    {
        return false;
    }
    unsafe { sve::mls_f32(&[2.0, 3.0], &[4.0, 5.0], &[10.0, 20.0], &mut out[..2]) };
    if out[..2] != [2.0, 5.0] {
        return false;
    }
    unsafe { sve::mls_f64(&[2.0, 3.0], &[4.0, 5.0], &[10.0, 20.0], &mut od[..2]) };
    if od[..2] != [2.0, 5.0] {
        return false;
    }
    unsafe { sve::clz_i32(&[1, 2, 4], &mut oi[..3]) };
    if oi[..3] != [31, 30, 29] {
        return false;
    }
    unsafe { sve::cnt_i32(&[0, 1, 3], &mut oi[..3]) };
    if oi[..3] != [0, 1, 2] {
        return false;
    }
    unsafe { sve::not_i32(&[0, -1], &mut oi[..2]) };
    if oi[..2] != [-1, 0] {
        return false;
    }
    unsafe { sve::lsl_i32(&[1, 2, 3], 1, &mut oi[..3]) };
    if oi[..3] != [2, 4, 6] {
        return false;
    }
    unsafe { sve::lsr_i32(&[2, 4, 6], 1, &mut oi[..3]) };
    if oi[..3] != [1, 2, 3] {
        return false;
    }
    unsafe { sve::asr_i32(&[-2, -4, 4], 1, &mut oi[..3]) };
    if oi[..3] != [-1, -2, 2] {
        return false;
    }
    unsafe { sve::mla_i32(&[2, 3], &[4, 5], &[1, 2], &mut oi[..2]) };
    if oi[..2] != [9, 17] {
        return false;
    }
    unsafe { sve::mls_i32(&[2, 3], &[4, 5], &[10, 20], &mut oi[..2]) };
    if oi[..2] != [2, 5] {
        return false;
    }
    unsafe { sve::rinta_f32(&[1.5, 2.5, -1.5], &mut out[..3]) };
    if out[..3] != [2.0, 3.0, -2.0] {
        return false;
    }
    unsafe { sve::rinta_f64(&[1.5, 2.5, -1.5], &mut od) };
    if od != [2.0, 3.0, -2.0] {
        return false;
    }
    unsafe { sve::rintx_f32(&[1.5, 2.5, -1.5], &mut out[..3]) };
    if out[..3] != [2.0, 2.0, -2.0] {
        return false;
    }
    unsafe { sve::rintx_f64(&[1.5, 2.5, -1.5], &mut od) };
    if od != [2.0, 2.0, -2.0] {
        return false;
    }
    unsafe { sve::rinti_f32(&[1.5, 2.5, -1.5], &mut out[..3]) };
    if out[..3] != [2.0, 2.0, -2.0] {
        return false;
    }
    unsafe { sve::rinti_f64(&[1.5, 2.5, -1.5], &mut od) };
    if od != [2.0, 2.0, -2.0] {
        return false;
    }
    unsafe { sve::frecpx_f32(&[1.0, 2.0, 4.0], &mut out[..3]) };
    if out[..3]
        .iter()
        .any(|value| *value == 0.0 || !value.is_finite())
    {
        return false;
    }
    unsafe { sve::frecpx_f64(&[1.0, 2.0, 4.0], &mut od) };
    if od.iter().any(|value| *value == 0.0 || !value.is_finite()) {
        return false;
    }
    unsafe { sve::mul_f32(&a, &b, &mut out) };
    if out != [10.0, 40.0, 90.0, 160.0, 250.0, 360.0, 490.0] {
        return false;
    }
    let a = [1i32, 2, 3, 4, 5];
    let b = [10i32, 20, 30, 40, 50];
    let mut out = [0i32; 5];
    unsafe { sve::add_i32(&a, &b, &mut out) };
    if out != [11, 22, 33, 44, 55] {
        return false;
    }

    let ones_i8 = [1i8; 4];
    let ones_i16 = [1i16; 4];
    let ones_i32 = [1i32; 4];
    let ones_i64 = [1i64; 4];
    let mut cls_i8 = [0i8; 4];
    let mut cls_i16 = [0i16; 4];
    let mut cls_i32 = [0i32; 4];
    let mut cls_i64 = [0i64; 4];
    unsafe {
        sve::cls_i8(&ones_i8, &mut cls_i8);
        sve::cls_i16(&ones_i16, &mut cls_i16);
        sve::cls_i32(&ones_i32, &mut cls_i32);
        sve::cls_i64(&ones_i64, &mut cls_i64);
    }
    core::hint::black_box(&cls_i8);
    core::hint::black_box(&cls_i16);
    core::hint::black_box(&cls_i32);
    core::hint::black_box(&cls_i64);
    true
}

#[target_feature(enable = "sve")]
unsafe fn sve_ext_ops() -> bool {
    if unsafe { sve::cntb() } < 16
        || unsafe { sve::cnth() } < 8
        || unsafe { sve::cntw() } < 4
        || unsafe { sve::cntd() } < 2
    {
        return false;
    }
    let vl = unsafe { sve::rdvl::<1>() };
    if vl < 16
        || unsafe { sve::rdvl::<2>() } != vl * 2
        || unsafe { sve::addvl::<1>(0) } != vl
        || unsafe { sve::addpl::<1>(0) } != vl / 8
    {
        return false;
    }
    if unsafe { sve::ptest_i32(&[1, 2]) } != true || unsafe { sve::ptest_i32(&[]) } != false {
        return false;
    }
    if unsafe { sve::whilelt_i32(0, 3) } != 3 {
        return false;
    }
    if unsafe { sve::whilele_u32(0, 3) } != 4 {
        return false;
    }
    if unsafe { sve::whilegt_i32(5, 2) } != 3 {
        return false;
    }
    if unsafe { sve::whilege_u32(5, 2) } != 4 {
        return false;
    }
    if unsafe { sve::whilerw_i32(0, 1) } == 0 {
        return false;
    }
    if unsafe { sve::whilewr_i32(0, 1) } == 0 {
        return false;
    }
    let mut o8 = [0i8; 32];
    let mut o16 = [0i16; 32];
    let mut o32 = [0i32; 32];
    let mut o64 = [0i64; 32];
    let mut ou8 = [0u8; 32];
    let mut ou16 = [0u16; 32];
    let mut ou32 = [0u32; 32];
    let mut ou64 = [0u64; 32];
    let mut of32 = [0.0f32; 32];
    let mut of64 = [0.0f64; 32];

    unsafe { sve::abs_i8(&[-1, -2, 3], &mut o8[..3]) };
    if o8[..3] != [1, 2, 3] {
        return false;
    }
    unsafe { sve::neg_i16(&[1, -2, 3], &mut o16[..3]) };
    if o16[..3] != [-1, 2, -3] {
        return false;
    }
    unsafe { sve::abs_i64(&[-4, 5], &mut o64[..2]) };
    if o64[..2] != [4, 5] {
        return false;
    }
    unsafe { sve::clz_i8(&[1, 2, 4], &mut o8[..3]) };
    if o8[..3] != [7, 6, 5] {
        return false;
    }
    unsafe { sve::cnt_i16(&[0, 1, 3], &mut o16[..3]) };
    if o16[..3] != [0, 1, 2] {
        return false;
    }
    unsafe { sve::not_i64(&[0, -1], &mut o64[..2]) };
    if o64[..2] != [-1, 0] {
        return false;
    }
    unsafe { sve::sqabs_i8(&[-128, -1, 1], &mut o8[..3]) };
    if o8[..3] != [127, 1, 1] {
        return false;
    }
    unsafe { sve::sqneg_i16(&[-32768, 2], &mut o16[..2]) };
    if o16[..2] != [32767, -2] {
        return false;
    }
    unsafe { sve::rbit_i32(&[1], &mut o32[..1]) };
    if o32[0] != i32::MIN {
        return false;
    }
    unsafe { sve::revb_i32(&[0x11223344], &mut o32[..1]) };
    if o32[0] != 0x44332211 {
        return false;
    }
    unsafe { sve::revh_i32(&[0x11223344], &mut o32[..1]) };
    if o32[0] != 0x33441122 {
        return false;
    }
    unsafe { sve::cnot_i32(&[0, -1], &mut o32[..2]) };
    if o32[..2] != [-1, 0] {
        return false;
    }
    unsafe { sve::sunpkhi_i8(&[-1, 2], &mut o16[..2]) };
    if o16[..2] != [-1, 2] {
        return false;
    }
    unsafe { sve::uunpklo_u8(&[1, 255], &mut ou16[..2]) };
    if ou16[..2] != [1, 255] {
        return false;
    }
    unsafe { sve::sxtb_i8(&[-1, 2], &mut o32[..2]) };
    if o32[..2] != [-1, 2] {
        return false;
    }
    unsafe { sve::uxtw_u32(&[1, 0xffff_ffff], &mut ou64[..2]) };
    if ou64[..2] != [1, 0xffff_ffff] {
        return false;
    }

    unsafe { sve::add_i8(&[1, 2], &[3, 4], &mut o8[..2]) };
    if o8[..2] != [4, 6] {
        return false;
    }
    unsafe { sve::sub_i16(&[5, 6], &[1, 2], &mut o16[..2]) };
    if o16[..2] != [4, 4] {
        return false;
    }
    unsafe { sve::mul_i64(&[2, 3], &[4, 5], &mut o64[..2]) };
    if o64[..2] != [8, 15] {
        return false;
    }
    unsafe { sve::add_u8(&[250, 1], &[5, 2], &mut ou8[..2]) };
    if ou8[..2] != [255, 3] {
        return false;
    }
    unsafe { sve::smax_i8(&[-1, 2], &[0, 1], &mut o8[..2]) };
    if o8[..2] != [0, 2] {
        return false;
    }
    unsafe { sve::umin_u16(&[1, 9], &[5, 3], &mut ou16[..2]) };
    if ou16[..2] != [1, 3] {
        return false;
    }
    unsafe { sve::sabd_i8(&[-5, 3], &[2, -1], &mut o8[..2]) };
    if o8[..2] != [7, 4] {
        return false;
    }
    unsafe { sve::uabd_u32(&[1, 9], &[5, 3], &mut ou32[..2]) };
    if ou32[..2] != [4, 6] {
        return false;
    }
    unsafe { sve::smulh_i32(&[0x4000_0000, -2], &[4, 3], &mut o32[..2]) };
    if o32[..2] != [1, -1] {
        return false;
    }
    unsafe { sve::umulh_u32(&[0x8000_0000, 2], &[2, 3], &mut ou32[..2]) };
    if ou32[..2] != [1, 0] {
        return false;
    }
    unsafe { sve::and_i8(&[0b1010, 0b1100], &[0b1100, 0b1010], &mut o8[..2]) };
    if o8[..2] != [0b1000, 0b1000] {
        return false;
    }
    unsafe { sve::orr_i16(&[0b1010, 0b1100], &[0b1100, 0b1010], &mut o16[..2]) };
    if o16[..2] != [0b1110, 0b1110] {
        return false;
    }
    unsafe { sve::eor_i64(&[1, 3], &[3, 1], &mut o64[..2]) };
    if o64[..2] != [2, 2] {
        return false;
    }
    unsafe { sve::bic_i32(&[0b1010], &[0b1100], &mut o32[..1]) };
    if o32[0] != 0b0010 {
        return false;
    }
    unsafe { sve::nand_i32(&[0b1010], &[0b1100], &mut o32[..1]) };
    if o32[0] != -9 {
        return false;
    }
    unsafe { sve::nor_i32(&[0b1010], &[0b1100], &mut o32[..1]) };
    if o32[0] != -15 {
        return false;
    }
    unsafe { sve::orn_i32(&[0b1010], &[0b1100], &mut o32[..1]) };
    if o32[0] != -5 {
        return false;
    }
    unsafe { sve::bsl_i32(&[0b1010], &[0b0101], &[0b1100], &mut o32[..1]) };
    if o32[0] != 0b1001 {
        return false;
    }
    unsafe { sve::sqadd_i8(&[120, -120], &[20, -20], &mut o8[..2]) };
    if o8[..2] != [127, -128] {
        return false;
    }
    unsafe { sve::uqadd_u8(&[250, 1], &[10, 2], &mut ou8[..2]) };
    if ou8[..2] != [255, 3] {
        return false;
    }
    unsafe { sve::sqsub_i16(&[-32768, 10], &[1, 20], &mut o16[..2]) };
    if o16[..2] != [-32768, -10] {
        return false;
    }
    unsafe { sve::uqsub_u16(&[1, 10], &[2, 20], &mut ou16[..2]) };
    if ou16[..2] != [0, 0] {
        return false;
    }
    unsafe { sve::sqdmulh_i16(&[16384, -16384], &[16384, 16384], &mut o16[..2]) };
    if o16[..2] != [8192, -8192] {
        return false;
    }
    unsafe { sve::sqxtnb_i32(&[1, -1, 40000, 70000], &mut o16[..4]) };
    if o16[..4] != [1, -1, 32767, 32767] {
        return false;
    }
    unsafe { sve::sqxtunb_i32(&[1, -1, 70000], &mut ou16[..3]) };
    if ou16[..3] != [1, 0, 65535] {
        return false;
    }
    unsafe { sve::uqxtnb_u32(&[1, 70000], &mut ou16[..2]) };
    if ou16[..2] != [1, 65535] {
        return false;
    }
    unsafe { sve::cadd_i32(&[1, 2], &[3, 4], &mut o32[..2]) };
    if o32[..2] != [-3, 5] {
        return false;
    }
    unsafe { sve::sclamp_i32(&[-5, 0, 5], -2, 2, &mut o32[..3]) };
    if o32[..3] != [-2, 0, 2] {
        return false;
    }

    if unsafe { sve::reduce_add_i8(&[1, 2, 3]) } != 6 {
        return false;
    }
    if unsafe { sve::reduce_add_u32(&[1, 2, 3]) } != 6 {
        return false;
    }
    if unsafe { sve::reduce_max_i16(&[-1, 5, 2]) } != 5 {
        return false;
    }
    if unsafe { sve::reduce_min_u32(&[4, 2, 9]) } != 2 {
        return false;
    }
    if unsafe { sve::reduce_and_i32(&[0b1100, 0b1010]) } != 0b1000 {
        return false;
    }
    if unsafe { sve::reduce_eor_i32(&[1, 3]) } != 2 {
        return false;
    }
    if unsafe { sve::orv_i8(&[1, 2, 4]) } != 7
        || unsafe { sve::orv_i16(&[1, 2, 4]) } != 7
        || unsafe { sve::orv_i32(&[1, 2, 4]) } != 7
        || unsafe { sve::orv_i64(&[1, 2, 4]) } != 7
        || unsafe { sve::orv_u8(&[1, 2, 4]) } != 7
        || unsafe { sve::orv_u16(&[1, 2, 4]) } != 7
        || unsafe { sve::orv_u32(&[1, 2, 4]) } != 7
        || unsafe { sve::orv_u64(&[1, 2, 4]) } != 7
    {
        return false;
    }
    if unsafe { sve::saddv_i8(&[100, 100]) } != 200
        || unsafe { sve::saddv_i16(&[30000, 30000]) } != 60000
        || unsafe { sve::saddv_i32(&[1_000_000_000, 1_000_000_000]) } != 2_000_000_000
        || unsafe { sve::saddv_i64(&[1 << 40, 1 << 40]) } != 2 << 40
        || unsafe { sve::uaddv_u8(&[200, 200]) } != 400
        || unsafe { sve::uaddv_u16(&[60000, 60000]) } != 120000
        || unsafe { sve::uaddv_u32(&[3_000_000_000, 3_000_000_000]) } != 6_000_000_000
        || unsafe { sve::uaddv_u64(&[1 << 40, 1 << 40]) } != 2 << 40
    {
        return false;
    }
    unsafe {
        if sve::pfalse() != 0 {
            return false;
        }
        if sve::ptrue() != u64::MAX || sve::ptrues() != u64::MAX {
            return false;
        }
        if sve::pfirst(0b1000) != 0b1000 {
            return false;
        }
        if sve::pnext(0b1010) != 0b1000 {
            return false;
        }
        if !sve::ptest(1) || sve::ptest(0) {
            return false;
        }
        if sve::cntp(0x0f) != 4 {
            return false;
        }
        if !sve::ctermeq(7, 7) || sve::ctermne(7, 7) {
            return false;
        }
        sve::wrffr(0);
        if sve::rdffr() != 0 {
            return false;
        }
        sve::setffr();
        if sve::rdffr() == 0 {
            return false;
        }
        if sve::rdffrs(0) != 0 {
            return false;
        }
        if sve::rdffr() == 0 {
            return false;
        }
    }
    let mut hist_out = [0u32; 2];
    let mut sqrd_out = [0i32; 2];
    unsafe {
        sve::histcnt_u32(&[1, 2], &[1, 1, 2, 3], &mut hist_out);
        if hist_out != [2, 1] {
            return false;
        }
        let mut histseg = [0u8; 2];
        sve::histseg_u8(&[1, 9], &[1, 1], &mut histseg);
        if histseg != [2, 0] {
            return false;
        }
        // SQRDCMLAH is excluded: the rotation immediate is not represented by
        // the slice API, so no value assertion is made here.
        sve::sqrdcmlah_indexed_i32(&[1, 1], &[2, 2], &[3, 3], &mut sqrd_out);
        sve::sqrdcmlah_vec_i32(&[1, 1], &[2, 2], &[3, 3], &mut sqrd_out);
        let mut fml_out = [0.0f32; 4];
        sve::fmlalb_lane_f32(&[1.0; 4], &[0u16; 8], &[0u16; 8], &mut fml_out);
        sve::fmlalt_lane_f32(&[1.0; 4], &[0u16; 8], &[0u16; 8], &mut fml_out);
        sve::fmlslb_lane_f32(&[1.0; 4], &[0u16; 8], &[0u16; 8], &mut fml_out);
        sve::fmlslt_lane_f32(&[1.0; 4], &[0u16; 8], &[0u16; 8], &mut fml_out);
    }
    let mut misc64 = [0u64; 2];
    let mut misc32 = [0u32; 2];
    let mut miscf = [0.0f32; 2];
    unsafe {
        sve::adr_u64(10, &[1, 2], &mut misc64);
        if misc64 != [11, 12] {
            return false;
        }
        sve::facle_f32(&[1.0, 5.0], &[2.0, 4.0], &mut misc32);
        if misc32 != [u32::MAX, 0] {
            return false;
        }
        sve::faclt_f32(&[1.0, 5.0], &[2.0, 4.0], &mut misc32);
        if misc32 != [u32::MAX, 0] {
            return false;
        }
        sve::fsubr_imm_f32(&[1.0, 2.0], 5.0, &mut miscf);
        if miscf != [4.0, 3.0] {
            return false;
        }
        let mut matched = [0u8; 2];
        sve::match_u8(&[2, 9], &[1, 2], &mut matched);
        if matched != [0xff, 0] {
            return false;
        }
        sve::nmatch_u8(&[2, 9], &[1, 2], &mut matched);
        if matched != [0, 0xff] {
            return false;
        }
        let mut table = [7u8, 8, 9];
        sve::tbx_u8(&[10, 20, 30], &[0, 2, 9], &mut table);
        if table != [10, 30, 9] {
            return false;
        }
        // URSQRTE and WHILERW/WHILEWR are excluded: their exact results
        // depend on the architecture lookup table and address operands.
        sve::ursqrte_u32(&[0, 4], &mut misc32);
        let _ = sve::whilerw_u32(1, 5);
        let _ = sve::whilewr_u32(1, 5);
        sve::insr_scalar_i32(&[1, 2], 9, &mut [0i32; 2]);
        sve::insr_simd_f32(&[1.0, 2.0], &[9.0, 2.0], &mut miscf);
    }
    let mut sat_shift_out = [0i32; 2];
    unsafe {
        sve::sqsubr_i32(&[3, 4], &[10, 15], &mut sat_shift_out);
        if sat_shift_out != [7, 11] {
            return false;
        }
        sve::uqsubr_u32(&[3u32, 4], &[10u32, 15], &mut [0u32; 2]);
        sve::suqadd_i32(&[i32::MAX - 1, 1], &[5, 5], &mut sat_shift_out);
        if sat_shift_out[0] != i32::MAX {
            return false;
        }
        sve::usqadd_u32(&[u32::MAX - 1, 1], &[5, 5], &mut [0u32; 2]);
        sve::sqshlr_i32(&[1, 1], &[3, 4], &mut sat_shift_out);
        if sat_shift_out != [8, 16] {
            return false;
        }
        sve::sqshlr_i32(&[i32::MAX, i32::MIN], &[1, 1], &mut sat_shift_out);
        if sat_shift_out != [i32::MAX, i32::MIN] {
            return false;
        }
        sve::sqrshlr_i32(&[1; 2], &[3; 2], &mut sat_shift_out);
        sve::uqshlr_u32(&[1u32; 2], &[3u32; 2], &mut [0u32; 2]);
        sve::uqrshlr_u32(&[1u32; 2], &[3u32; 2], &mut [0u32; 2]);
        sve::srshlr_i32(&[1; 2], &[3; 2], &mut sat_shift_out);
        sve::urshlr_u32(&[1u32; 2], &[3u32; 2], &mut [0u32; 2]);
    }
    let mut subhn_out = [0i8; 8];
    let mut subr_out = [0i32; 2];
    unsafe {
        sve::subhnb_i16(&[0; 8], &[0; 8], &mut subhn_out);
        sve::subhnt_i16(&[0; 8], &[0; 8], &mut subhn_out);
        sve::shsubr_i32(&[1, 2], &[7, 9], &mut subr_out);
        if subr_out != [3, 3] {
            return false;
        }
        sve::uhsubr_u32(&[1u32, 2], &[7u32, 9], &mut [0u32; 2]);
    }
    let mut half_out = [0u16; 4];
    let mut float_out2 = [0.0f32; 4];
    unsafe {
        sve::bfcvt_f32(&[1.0], &mut half_out[..1]);
        if half_out[0] != 0x3f80 {
            return false;
        }
        sve::bfcvtnt_f32(&[1.0], &mut half_out[..1]);
        if half_out[0] != 0x3f80 {
            return false;
        }
        let bf16_round = [f32::from_bits(0x3f80_8000), f32::from_bits(0x3f81_8000)];
        sve::bfcvt_f32(&bf16_round, &mut half_out[..2]);
        if half_out[..2] != [0x3f80, 0x3f82] {
            return false;
        }
        sve::fcvt_f64_to_f32(&[1.5], &mut float_out2[..1]);
        if float_out2[0] != 1.5 {
            return false;
        }
        sve::fcvtnt_f32(&[1.0], &mut half_out[..1]);
        if half_out[0] != 0x3c00 {
            return false;
        }
        sve::fcvtx_f32(&[1.0], &mut half_out[..1]);
        if half_out[0] != 0x3c00 {
            return false;
        }
        sve::fcvtxnt_f32(&[1.0], &mut half_out[..1]);
        if half_out[0] != 0x3c00 {
            return false;
        }
        sve::fcvtlt_f16_to_f32(&[0x3c00], &mut float_out2[..1]);
        if float_out2[0] != 1.0 {
            return false;
        }
    }
    let mut float_out = [0.0f32; 4];
    let mut mask32_out = [0u32; 4];
    unsafe {
        sve::fdup_f32(1.5, &mut float_out);
        if float_out != [1.5; 4] {
            return false;
        }
        sve::fcpy_f32(&[1.0, 2.0, 3.0, 4.0], &mut float_out);
        if float_out != [1.0, 2.0, 3.0, 4.0] {
            return false;
        }
        sve::fcmuo_f32(&[f32::NAN, 1.0, 2.0, 3.0], &[1.0; 4], &mut mask32_out);
        if mask32_out != [u32::MAX, 0, 0, 0] {
            return false;
        }
        sve::flogb_f32(&[8.0, 1.0], &mut float_out[..2]);
        if float_out[..2] != [3.0, 0.0] {
            return false;
        }
        sve::flogb_f32(
            &[0.0, f32::INFINITY, f32::from_bits(1), f32::NAN],
            &mut float_out,
        );
        if !float_out[0].is_infinite()
            || float_out[0] > 0.0
            || float_out[1] != f32::INFINITY
            || float_out[2] != -149.0
            || !float_out[3].is_nan()
        {
            return false;
        }
        sve::fcadd_f32(&[1.0; 4], &[2.0; 4], &mut float_out);
    }
    let mut load32 = [0i32; 4];
    let sb = [1i8, -2, 3, -4];
    let sh = [100i16, -200, 300, -400];
    let sw = [1000i32, -2000, 3000, -4000];
    unsafe {
        sve::ldnf1sb_i32(sb.as_ptr(), &mut load32);
        if load32 != [1, -2, 3, -4] {
            return false;
        }
        sve::ldnf1sh_i32(sh.as_ptr(), &mut load32);
        if load32 != [100, -200, 300, -400] {
            return false;
        }
        sve::ldnf1sw_i32(sw.as_ptr(), &mut load32);
        if load32 != [1000, -2000, 3000, -4000] {
            return false;
        }
        sve::ldnt1sb_i32(sb.as_ptr(), &mut load32);
        if load32 != [1, -2, 3, -4] {
            return false;
        }
        sve::ldnt1sh_i32(sh.as_ptr(), &mut load32);
        if load32 != [100, -200, 300, -400] {
            return false;
        }
        sve::ldnt1sw_i32(sw.as_ptr(), &mut load32);
        if load32 != [1000, -2000, 3000, -4000] {
            return false;
        }
    }
    let mut logic_out = [0u32; 2];
    unsafe {
        sve::nots(&[0, u32::MAX], &mut logic_out);
        if logic_out != [u32::MAX, 0] {
            return false;
        }
        sve::nands(&[u32::MAX, u32::MAX], &[u32::MAX, 0], &mut logic_out);
        if logic_out != [0, u32::MAX] {
            return false;
        }
        sve::nors(&[0, 0], &[0, u32::MAX], &mut logic_out);
        if logic_out != [u32::MAX, 0] {
            return false;
        }
        sve::orns(&[0, 0], &[u32::MAX, 0], &mut logic_out);
        if logic_out != [0, u32::MAX] {
            return false;
        }
        sve::nbsl(&[0; 2], &[0; 2], &[0; 2], &mut logic_out);
    }
    let mut byte_out = [0i8; 4];
    let mut ubyte_out = [0u8; 4];
    unsafe {
        if sve::incb_scalar(1, 1) != 1 + (sve::cntb() as i8) {
            return false;
        }
        if sve::sqincb_scalar(i8::MAX, 1) != i8::MAX {
            return false;
        }
        if sve::uqdecb_scalar(0, 1) != 0 {
            return false;
        }
        sve::incb_vector(&[1; 4], 1, &mut byte_out);
        sve::decb_vector(&[1; 4], 1, &mut byte_out);
        sve::sqincb_vector(&[1; 4], 1, &mut byte_out);
        sve::sqdecb_vector(&[1; 4], 1, &mut byte_out);
        sve::uqincb_vector(&[1u8; 4], 1, &mut ubyte_out);
        sve::uqdecb_vector(&[1u8; 4], 1, &mut ubyte_out);
        core::hint::black_box(sve::decb_scalar(0, 1));
        core::hint::black_box(sve::sqdecb_scalar(0, 1));
        core::hint::black_box(sve::uqincb_scalar(0, 1));
    }
    let mut bit_out = [0u32; 4];
    unsafe {
        sve::bdep_u32(&[0; 4], &[0; 4], &mut bit_out);
        sve::bext_u32(&[0; 4], &[0; 4], &mut bit_out);
        sve::bgrp_u32(&[0; 4], &[0; 4], &mut bit_out);
        sve::eorbt_u32(&[1; 4], &[2; 4], &[4; 4], &mut bit_out);
        sve::eortb_u32(&[1; 4], &[2; 4], &[4; 4], &mut bit_out);
    }
    let mut pmul_out = [0u8; 2];
    unsafe {
        sve::pmul_u8(&[2, 3], &[3, 5], &mut pmul_out);
        if pmul_out != [6, 15] {
            return false;
        }
        sve::pmullb_u16(&[0; 2], &[0; 2], &mut [0u16; 2]);
        sve::pmullt_u16(&[0; 2], &[0; 2], &mut [0u16; 2]);
        sve::smaxp_i32(&[1; 4], &[2; 4], &mut [0i32; 4]);
        sve::sminp_i32(&[1; 4], &[2; 4], &mut [0i32; 4]);
        sve::umaxp_u32(&[1; 4], &[2; 4], &mut [0u32; 4]);
        sve::uminp_u32(&[1; 4], &[2; 4], &mut [0u32; 4]);
    }
    let mut wide_out = [0i16; 4];
    unsafe {
        sve::sshllb_i16(&[1, 2, 3, 4, 5, 6, 7, 8], 1, &mut wide_out);
        if wide_out != [2, 6, 10, 14] {
            return false;
        }
        sve::sshllt_i16(&[1, 2, 3, 4, 5, 6, 7, 8], 1, &mut wide_out);
        if wide_out != [4, 8, 12, 16] {
            return false;
        }
        sve::ushllb_u16(&[1, 2, 3, 4, 5, 6, 7, 8], 1, &mut [0u16; 4]);
        sve::ushllt_u16(&[1, 2, 3, 4, 5, 6, 7, 8], 1, &mut [0u16; 4]);
    }
    let mut shrn_out = [0i8; 8];
    unsafe {
        sve::shrnb_i16(&[2, 4, 6, 8, 10, 12, 14, 16], &mut shrn_out);
        sve::rshrnb_i16(&[3, 5, 7, 9, 11, 13, 15, 17], &mut shrn_out);
        sve::shrnt_i16(&[2, 4, 6, 8, 10, 12, 14, 16], &mut shrn_out);
        sve::rshrnt_i16(&[3, 5, 7, 9, 11, 13, 15, 17], &mut shrn_out);
    }
    let mut bf_out = [0.0f32; 4];
    unsafe {
        sve::bfdot_f32(&[1.0; 4], &[0u16; 8], &[0u16; 8], &mut bf_out);
        if bf_out != [1.0; 4] {
            return false;
        }
        sve::bfmlalb_f32(&[1.0; 4], &[0u16; 8], &[0u16; 8], &mut bf_out);
        if bf_out != [1.0; 4] {
            return false;
        }
        sve::bfmlalt_f32(&[1.0; 4], &[0u16; 8], &[0u16; 8], &mut bf_out);
        if bf_out != [1.0; 4] {
            return false;
        }
        sve::bfmmla_f32(&[1.0; 4], &[0u16; 8], &[0u16; 8], &mut bf_out);
        if bf_out != [1.0; 4] {
            return false;
        }
    }
    let mut shift_out = [0i32; 2];
    let mut carry_out = [0u32; 4];
    unsafe {
        sve::asrd_i32(&[8, -8], 1, &mut shift_out);
        if shift_out != [4, -4] {
            return false;
        }
        sve::asrr_i32(&[3, -3], 1, &mut shift_out);
        if shift_out != [2, -1] {
            return false;
        }
        sve::lslr_i32(&[1, 2], 2, &mut shift_out);
        if shift_out != [4, 8] {
            return false;
        }
        sve::lsrr_i32(&[8, 9], 1, &mut shift_out);
        if shift_out != [4, 5] {
            return false;
        }
    }
    unsafe {
        sve::adclb_u32(&[1; 4], &[2; 4], &[3; 4], &mut carry_out);
        sve::adclt_u32(&[1; 4], &[2; 4], &[3; 4], &mut carry_out);
        sve::sbclb_u32(&[1; 4], &[2; 4], &[3; 4], &mut carry_out);
        sve::sbclt_u32(&[1; 4], &[2; 4], &[3; 4], &mut carry_out);
    }
    let mut crypto_out = [0u64; 2];
    unsafe {
        sve::eor3_u64(&[1, 2], &[4, 8], &[16, 32], &mut crypto_out);
        if crypto_out != [21, 42] {
            return false;
        }
    }
    unsafe {
        let mut aes_out = [0u8; 16];
        let mut sm4_out = [0u32; 4];
        let mut word_out = [0u64; 2];
        sve::aese_u8(&[0; 16], &[0; 16], &mut aes_out);
        sve::aesd_u8(&[0; 16], &[0; 16], &mut aes_out);
        sve::aesimc_u8(&[0; 16], &mut aes_out);
        sve::aesmc_u8(&[0; 16], &mut aes_out);
        sve::sm4e_u32(&[0; 4], &[0; 4], &mut sm4_out);
        sve::sm4ekey_u32(&[0; 4], &[0; 4], &mut sm4_out);
        sve::bcax_u64(&[1; 2], &[2; 2], &[3; 2], &mut word_out);
        sve::rax1_u64(&[1; 2], &[2; 2], &mut word_out);
        sve::xar_u64(&[1; 2], &[2; 2], &mut word_out);
    }
    let mut addhn_out = [0i8; 8];
    let mut movs_out = [0u8; 4];
    unsafe {
        sve::addhnb_i16(&[0i16; 16], &[0i16; 16], &mut addhn_out);
        if addhn_out != [0; 8] {
            return false;
        }
        sve::addhnt_i16(&[0i16; 16], &[0i16; 16], &mut addhn_out);
        if addhn_out != [0; 8] {
            return false;
        }
        sve::movs_unpred(0b1010, &mut movs_out);
        if movs_out != [0, 0xff, 0, 0xff] {
            return false;
        }
        sve::movs_pred(0b1111, 0b1010, &mut movs_out);
        if movs_out != [0, 0xff, 0, 0xff] {
            return false;
        }
    }
    let mut brk_out = [0u32; 4];
    unsafe {
        sve::brka(&[0, u32::MAX, 0, 0], &mut brk_out);
        if brk_out != [u32::MAX, u32::MAX, 0, 0] {
            return false;
        }
        sve::brkb(&[0, u32::MAX, 0, 0], &mut brk_out);
        if brk_out != [u32::MAX, 0, 0, 0] {
            return false;
        }
    }
    let mut mla_out = [0i32; 4];
    let mut bsl_out = [0u8; 4];
    unsafe {
        sve::mla_i32(&[2; 4], &[3; 4], &[1; 4], &mut mla_out);
        if mla_out != [7; 4] {
            return false;
        }
        sve::mla_lane_i32(&[1; 4], &[2; 4], &[3; 4], &mut mla_out);
        if mla_out != [7; 4] {
            return false;
        }
        sve::mls_i32(&[2; 4], &[3; 4], &[1; 4], &mut mla_out);
        if mla_out != [-5; 4] {
            return false;
        }
        sve::mls_lane_i32(&[1; 4], &[2; 4], &[3; 4], &mut mla_out);
        if mla_out != [-5; 4] {
            return false;
        }
        sve::bsl1n_u8(&[0xf0; 4], &[0x0f; 4], &[0xaa; 4], &mut bsl_out);
        if bsl_out != [0xfa; 4] {
            return false;
        }
        sve::bsl2n_u8(&[0xf0; 4], &[0x0f; 4], &[0xaa; 4], &mut bsl_out);
        if bsl_out != [0xa0; 4] {
            return false;
        }
        if sve::rdffr_z(0) != 0 {
            return false;
        }
    }
    let ro_b = [1u8, 2, 3, 4];
    let ro_h = [100u16, 200, 300];
    let mut ro_out8 = [0u8; 4];
    let mut ro_out16 = [0u16; 4];
    let mut pred_mem = [0u8; 8];
    let mut vec_mem = [0u8; 16];
    unsafe {
        sve::ld1rob_imm_u8(ro_b.as_ptr(), 1, &mut ro_out8[..3]);
        if ro_out8[..3] != [2, 3, 4] {
            return false;
        }
        sve::ld1roh_scalar_u16(ro_h.as_ptr(), 0, &mut ro_out16[..3]);
        if ro_out16[..3] != [100, 200, 300] {
            return false;
        }
        sve::str_pred(pred_mem.as_mut_ptr(), 0x0123_4567_89ab_cdef);
        if sve::ldr_pred(pred_mem.as_ptr()) != 0x0123_4567_89ab_cdef {
            return false;
        }
        sve::str_vector(vec_mem.as_mut_ptr(), &[0xab; 16]);
        let mut vec_out = [0u8; 16];
        sve::ldr_vector(vec_mem.as_ptr(), &mut vec_out);
        if vec_out != [0xab; 16] {
            return false;
        }
    }

    unsafe {
        core::hint::black_box(sve::ld1rob_imm_u8([0u8; 4].as_ptr(), 0, &mut [0u8; 4]));
        core::hint::black_box(sve::ld1rob_scalar_u8([0u8; 4].as_ptr(), 0, &mut [0u8; 4]));
        core::hint::black_box(sve::ld1roh_imm_u16([0u16; 4].as_ptr(), 0, &mut [0u16; 4]));
        core::hint::black_box(sve::ld1roh_scalar_u16(
            [0u16; 4].as_ptr(),
            0,
            &mut [0u16; 4],
        ));
        core::hint::black_box(sve::ld1row_imm_u32([0u32; 4].as_ptr(), 0, &mut [0u32; 4]));
        core::hint::black_box(sve::ld1row_scalar_u32(
            [0u32; 4].as_ptr(),
            0,
            &mut [0u32; 4],
        ));
        core::hint::black_box(sve::ld1rod_imm_u64([0u64; 4].as_ptr(), 0, &mut [0u64; 4]));
        core::hint::black_box(sve::ld1rod_scalar_u64(
            [0u64; 4].as_ptr(),
            0,
            &mut [0u64; 4],
        ));
        core::hint::black_box(sve::ldr_pred([0u8; 8].as_ptr()));
        core::hint::black_box(sve::ldr_vector([0u8; 16].as_ptr(), &mut [0u8; 16]));
        core::hint::black_box(sve::str_pred([0u8; 8].as_mut_ptr(), 0));
        core::hint::black_box(sve::str_vector([0u8; 16].as_mut_ptr(), &[0u8; 16]));
    }
    let mut sat_out = [0i64; 4];
    unsafe {
        if sve::sqincw_scalar(i32::MAX - 1, 1) != i32::MAX {
            return false;
        }
        if sve::uqincw_scalar(u32::MAX - 1, 1) != u32::MAX {
            return false;
        }
        if sve::sqincp_scalar(&[u32::MAX, 0, u32::MAX, 0], 10, 1) != 12 {
            return false;
        }
        if sve::uqdecp_scalar(&[u32::MAX, 0, u32::MAX, 0], 10, 1) != 8 {
            return false;
        }
        sve::sqincd_vector(&[i64::MAX - 1, 1, 2, 3], 1, &mut sat_out);
        if sat_out[0] != i64::MAX {
            return false;
        }
    }

    unsafe {
        core::hint::black_box(sve::sqdecd_scalar(0, 1));
        core::hint::black_box(sve::sqdecd_vector(&[0i64; 4], 1, &mut [0i64; 4]));
        core::hint::black_box(sve::sqdech_scalar(0, 1));
        core::hint::black_box(sve::sqdech_vector(&[0i16; 4], 1, &mut [0i16; 4]));
        core::hint::black_box(sve::sqdecp_scalar(&[0u32; 4], 0, 1));
        core::hint::black_box(sve::sqdecp_vector(
            &[0u32; 4],
            &[0i64; 4],
            1,
            &mut [0i64; 4],
        ));
        core::hint::black_box(sve::sqdecw_scalar(0, 1));
        core::hint::black_box(sve::sqdecw_vector(&[0i32; 4], 1, &mut [0i32; 4]));
        core::hint::black_box(sve::sqincd_scalar(0, 1));
        core::hint::black_box(sve::sqincd_vector(&[0i64; 4], 1, &mut [0i64; 4]));
        core::hint::black_box(sve::sqinch_scalar(0, 1));
        core::hint::black_box(sve::sqinch_vector(&[0i16; 4], 1, &mut [0i16; 4]));
        core::hint::black_box(sve::sqincp_scalar(&[0u32; 4], 0, 1));
        core::hint::black_box(sve::sqincp_vector(
            &[0u32; 4],
            &[0i64; 4],
            1,
            &mut [0i64; 4],
        ));
        core::hint::black_box(sve::sqincw_scalar(0, 1));
        core::hint::black_box(sve::sqincw_vector(&[0i32; 4], 1, &mut [0i32; 4]));
        core::hint::black_box(sve::uqdecd_scalar(0, 1));
        core::hint::black_box(sve::uqdecd_vector(&[0u64; 4], 1, &mut [0u64; 4]));
        core::hint::black_box(sve::uqdech_scalar(0, 1));
        core::hint::black_box(sve::uqdech_vector(&[0u16; 4], 1, &mut [0u16; 4]));
        core::hint::black_box(sve::uqdecp_scalar(&[0u32; 4], 0, 1));
        core::hint::black_box(sve::uqdecp_vector(
            &[0u32; 4],
            &[0u64; 4],
            1,
            &mut [0u64; 4],
        ));
        core::hint::black_box(sve::uqdecw_scalar(0, 1));
        core::hint::black_box(sve::uqdecw_vector(&[0u32; 4], 1, &mut [0u32; 4]));
        core::hint::black_box(sve::uqincd_scalar(0, 1));
        core::hint::black_box(sve::uqincd_vector(&[0u64; 4], 1, &mut [0u64; 4]));
        core::hint::black_box(sve::uqinch_scalar(0, 1));
        core::hint::black_box(sve::uqinch_vector(&[0u16; 4], 1, &mut [0u16; 4]));
        core::hint::black_box(sve::uqincp_scalar(&[0u32; 4], 0, 1));
        core::hint::black_box(sve::uqincp_vector(
            &[0u32; 4],
            &[0u64; 4],
            1,
            &mut [0u64; 4],
        ));
        core::hint::black_box(sve::uqincw_scalar(0, 1));
        core::hint::black_box(sve::uqincw_vector(&[0u32; 4], 1, &mut [0u32; 4]));
    }
    let cntw = unsafe { sve::cntw() } as i32;
    let cntd = unsafe { sve::cntd() } as i64;
    let mut inc_out = [0i64; 4];
    unsafe {
        if sve::incw_scalar(5, 1) != 5 + cntw {
            return false;
        }
        if sve::decw_scalar(5, 1) != 5 - cntw {
            return false;
        }
        sve::incd_vector(&[1, 2, 3, 4], 1, &mut inc_out);
        if inc_out != [1 + cntd, 2 + cntd, 3 + cntd, 4 + cntd] {
            return false;
        }
        if sve::incp_scalar(&[u32::MAX, 0, u32::MAX, 0], 7, 1) != 9 {
            return false;
        }
        if sve::decp_scalar(&[u32::MAX, 0, u32::MAX, 0], 7, 1) != 5 {
            return false;
        }
        sve::incp_vector(&[u32::MAX, 0, u32::MAX, 0], &[1, 2, 3, 4], 1, &mut inc_out);
        if inc_out != [3, 4, 5, 6] {
            return false;
        }
    }
    unsafe {
        core::hint::black_box(sve::decd_scalar(0, 1));
        core::hint::black_box(sve::dech_scalar(0, 1));
        core::hint::black_box(sve::decw_scalar(0, 1));
        core::hint::black_box(sve::decd_vector(&[0i64; 4], 1, &mut inc_out));
        core::hint::black_box(sve::dech_vector(&[0i16; 4], 1, &mut [0i16; 4]));
        core::hint::black_box(sve::decw_vector(&[0i32; 4], 1, &mut [0i32; 4]));
        core::hint::black_box(sve::incd_scalar(0, 1));
        core::hint::black_box(sve::inch_scalar(0, 1));
        core::hint::black_box(sve::incw_scalar(0, 1));
        core::hint::black_box(sve::incd_vector(&[0i64; 4], 1, &mut inc_out));
        core::hint::black_box(sve::inch_vector(&[0i16; 4], 1, &mut [0i16; 4]));
        core::hint::black_box(sve::incw_vector(&[0i32; 4], 1, &mut [0i32; 4]));
        core::hint::black_box(sve::decp_vector(&[0u32; 4], &[0i64; 4], 1, &mut inc_out));
    }
    let mut cdot_acc = [1i32, 2, 3, 4];
    let mut cdot_out = [0i32; 4];
    let zeros8 = [0i8; 16];
    let zeros32 = [0i32; 4];
    let zerosf32 = [0.0f32; 4];
    unsafe {
        sve::cdot_i32(&cdot_acc, &zeros8, &zeros8, &mut cdot_out);
        if cdot_out != cdot_acc {
            return false;
        }
        sve::cdot_lane_i32(&cdot_acc, &zeros8, &zeros8, &mut cdot_out);
        if cdot_out != cdot_acc {
            return false;
        }
        sve::cmla_i32(&cdot_acc, &zeros32, &zeros32, &mut cdot_out);
        if cdot_out != cdot_acc {
            return false;
        }
        sve::cmla_lane_i32(&cdot_acc, &zeros32, &zeros32, &mut cdot_out);
        if cdot_out != cdot_acc {
            return false;
        }
        let mut fcmla_out = [0.0f32; 4];
        sve::fcmla_lane_f32(&[1.0; 4], &zerosf32, &zerosf32, &mut fcmla_out);
        if fcmla_out != [1.0; 4] {
            return false;
        }
    }
    let mut cmp_mask = [0u32; 4];
    let mut mlal_out = [0.0f32; 4];
    let one_f16 = [0x3c00u16; 8];
    unsafe {
        sve::fcmne_f32(&[1.0, 2.0, 3.0, 4.0], &[1.0, 9.0, 3.0, 9.0], &mut cmp_mask);
        if cmp_mask != [0, u32::MAX, 0, u32::MAX] {
            return false;
        }
        sve::fcmne_zero_f32(&[0.0, 1.0, 0.0, 1.0], &mut cmp_mask);
        if cmp_mask != [0, u32::MAX, 0, u32::MAX] {
            return false;
        }
        sve::fmlalb_f32(&[1.0; 4], &one_f16, &one_f16, &mut mlal_out);
        if mlal_out != [2.0; 4] {
            return false;
        }
        sve::fmlalt_f32(&[1.0; 4], &one_f16, &one_f16, &mut mlal_out);
        if mlal_out != [2.0; 4] {
            return false;
        }
        sve::fmlslb_f32(&[3.0; 4], &one_f16, &one_f16, &mut mlal_out);
        if mlal_out != [2.0; 4] {
            return false;
        }
        sve::fmlslt_f32(&[3.0; 4], &one_f16, &one_f16, &mut mlal_out);
        if mlal_out != [2.0; 4] {
            return false;
        }
    }
    let sb_data = [1i8, -2, 3, -4];
    let sh_data = [100i16, -200, 300];
    let sw_data = [1000i32, -2000, 3000];
    let mut load_out = [0i32; 4];
    unsafe {
        sve::ldff1sb_scalar(sb_data.as_ptr(), 0, &mut load_out[..3]);
        if load_out[..3] != [1, -2, 3] {
            return false;
        }
        sve::ldff1sh_scalar(sh_data.as_ptr(), 0, &mut load_out[..3]);
        if load_out[..3] != [100, -200, 300] {
            return false;
        }
        sve::ldff1sw_scalar(sw_data.as_ptr(), 0, &mut load_out[..3]);
        if load_out[..3] != [1000, -2000, 3000] {
            return false;
        }
        sve::ldff1sb_vector(sb_data.as_ptr(), &[0, 2], &mut load_out[..2]);
        if load_out[..2] != [1, 3] {
            return false;
        }
        let bases = [sb_data.as_ptr() as i64, sb_data.as_ptr().add(3) as i64];
        sve::ldff1sb_vec_imm(&bases, 0, &mut load_out[..2]);
        if load_out[..2] != [1, -4] {
            return false;
        }
    }
    unsafe {
        sve::prfb_imm(sb_data.as_ptr().cast(), 0);
        sve::prfb_scalar(sb_data.as_ptr().cast(), 0);
        sve::prfb_vector(sb_data.as_ptr().cast(), &[0, 1]);
        sve::prfb_vec_imm(&[sb_data.as_ptr() as i64], 0);
        sve::prfd_imm(sb_data.as_ptr().cast(), 0);
        sve::prfd_scalar(sb_data.as_ptr().cast(), 0);
        sve::prfd_vector(sb_data.as_ptr().cast(), &[0, 1]);
        sve::prfd_vec_imm(&[sb_data.as_ptr() as i64], 0);
        sve::prfh_imm(sb_data.as_ptr().cast(), 0);
        sve::prfh_scalar(sb_data.as_ptr().cast(), 0);
        sve::prfh_vector(sb_data.as_ptr().cast(), &[0, 1]);
        sve::prfh_vec_imm(&[sb_data.as_ptr() as i64], 0);
        sve::prfw_imm(sb_data.as_ptr().cast(), 0);
        sve::prfw_scalar(sb_data.as_ptr().cast(), 0);
        sve::prfw_vector(sb_data.as_ptr().cast(), &[0, 1]);
        sve::prfw_vec_imm(&[sb_data.as_ptr() as i64], 0);
    }
    let mut index_out = [0i32; 4];
    unsafe {
        sve::index_imm_i32(10, 2, &mut index_out);
        if index_out != [10, 12, 14, 16] {
            return false;
        }
        sve::index_imm_scalar_i32(10, 2, &mut index_out);
        if index_out != [10, 12, 14, 16] {
            return false;
        }
        sve::index_scalar_imm_i32(10, 2, &mut index_out);
        if index_out != [10, 12, 14, 16] {
            return false;
        }
        sve::index_scalar_i32(10, 2, &mut index_out);
        if index_out != [10, 12, 14, 16] {
            return false;
        }
        sve::clasta_vec_i32(
            &[u32::MAX, 0, u32::MAX, 0],
            &[9; 4],
            &[1, 2, 3, 4],
            &mut index_out,
        );
        if index_out != [3, 9, 3, 9] {
            return false;
        }
        sve::clastb_vec_i32(
            &[u32::MAX, 0, u32::MAX, 0],
            &[9; 4],
            &[1, 2, 3, 4],
            &mut index_out,
        );
        if index_out != [1, 9, 1, 9] {
            return false;
        }
    }
    let mut dup_out = [0i32; 4];
    let mut dup_bytes = [0u8; 4];
    let mut dup_words = [0u64; 4];
    unsafe {
        sve::dup_i32(7, &mut dup_out);
        if dup_out != [7; 4] {
            return false;
        }
        sve::dup_lane_i32(&[1, 2, 3, 4], 2, &mut dup_out);
        if dup_out != [3; 4] {
            return false;
        }
        sve::dup_imm_u8(0xab, &mut dup_bytes);
        if dup_bytes != [0xab; 4] {
            return false;
        }
        sve::dupm_u64(0x0123_4567_89ab_cdef, &mut dup_words);
        if dup_words != [0x0123_4567_89ab_cdef; 4] {
            return false;
        }
        if sve::clasta_i32(&[u32::MAX, 0, u32::MAX, 0], -1, &[1, 2, 3, 4]) != 3 {
            return false;
        }
        if sve::clastb_i32(&[u32::MAX, 0, u32::MAX, 0], -1, &[1, 2, 3, 4]) != 1 {
            return false;
        }
    }
    let mut mask_out = [0u32; 4];
    unsafe {
        sve::ands(
            &[u32::MAX, 0, u32::MAX, 0],
            &[u32::MAX, u32::MAX, 0, 0],
            &mut mask_out,
        );
        if mask_out != [u32::MAX, 0, 0, 0] {
            return false;
        }
        sve::bics(
            &[u32::MAX, u32::MAX, 0, 0],
            &[u32::MAX, 0, 0, 0],
            &mut mask_out,
        );
        if mask_out != [0, u32::MAX, 0, 0] {
            return false;
        }
    }
    unsafe {
        core::hint::black_box(sve::ands(&[0u32; 4], &[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::bics(&[0u32; 4], &[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::eon(&[0u32; 4], &[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::eors(&[0u32; 4], &[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::orrs(&[0u32; 4], &[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::brka(&[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::brkas(&[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::brkb(&[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::brkbs(&[0u32; 4], &mut mask_out));
    }
    unsafe {
        core::hint::black_box(sve::brkn(&[0u32; 4], &[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::brkns(&[0u32; 4], &[0u32; 4], &mut mask_out));
        sve::brkpa(&[0, u32::MAX, 0, 0], &[0; 4], &mut mask_out);
        if mask_out != [0, u32::MAX, u32::MAX, u32::MAX] {
            return false;
        }
        core::hint::black_box(sve::brkpas(&[0u32; 4], &[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::brkpb(&[0u32; 4], &[0u32; 4], &mut mask_out));
        core::hint::black_box(sve::brkpbs(&[0u32; 4], &[0u32; 4], &mut mask_out));
    }
    let mut cmp_out = [0u32; 4];
    unsafe {
        sve::cmpeq_i32(&[1, 2, 3, 4], &[1, 9, 3, 9], &mut cmp_out);
        if cmp_out != [u32::MAX, 0, u32::MAX, 0] {
            return false;
        }
        sve::cmpgt_i32(&[2, 1, 4, 3], &[1, 2, 3, 4], &mut cmp_out);
        if cmp_out != [u32::MAX, 0, u32::MAX, 0] {
            return false;
        }
        sve::cmphi_u32(&[2, 1, 4, 3], &[1, 2, 3, 4], &mut cmp_out);
        if cmp_out != [u32::MAX, 0, u32::MAX, 0] {
            return false;
        }
        sve::cmpeq_imm_i32(&[1, 2, 3, 4], 3, &mut cmp_out);
        if cmp_out != [0, 0, u32::MAX, 0] {
            return false;
        }
    }
    unsafe {
        core::hint::black_box(unsafe { sve::cmpeq_i32(&[0i32; 4], &[0i32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpeq_imm_i32(&[0i32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpeq_wide_i32(&[0i32; 4], &[0i64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpge_i32(&[0i32; 4], &[0i32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpge_imm_i32(&[0i32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpge_wide_i32(&[0i32; 4], &[0i64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpgt_i32(&[0i32; 4], &[0i32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpgt_imm_i32(&[0i32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpgt_wide_i32(&[0i32; 4], &[0i64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmphi_u32(&[0u32; 4], &[0u32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmphi_imm_u32(&[0u32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmphi_wide_u32(&[0u32; 4], &[0u64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmphs_u32(&[0u32; 4], &[0u32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmphs_imm_u32(&[0u32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmphs_wide_u32(&[0u32; 4], &[0u64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmple_i32(&[0i32; 4], &[0i32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmple_imm_i32(&[0i32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmple_wide_i32(&[0i32; 4], &[0i64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmplo_u32(&[0u32; 4], &[0u32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmplo_imm_u32(&[0u32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmplo_wide_u32(&[0u32; 4], &[0u64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpls_u32(&[0u32; 4], &[0u32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpls_imm_u32(&[0u32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpls_wide_u32(&[0u32; 4], &[0u64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmplt_i32(&[0i32; 4], &[0i32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmplt_imm_i32(&[0i32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmplt_wide_i32(&[0i32; 4], &[0i64; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpne_i32(&[0i32; 4], &[0i32; 4], &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpne_imm_i32(&[0i32; 4], 0, &mut cmp_out) });
        core::hint::black_box(unsafe { sve::cmpne_wide_i32(&[0i32; 4], &[0i64; 4], &mut cmp_out) });
    }
    let mut out_i32 = [0i32; 4];
    let mut out_i64 = [0i64; 4];
    let mut out_u32 = [0u32; 4];
    let mut out_u64 = [0u64; 4];
    unsafe {
        sve::udiv_i32(&[8, 9], &[2, 3], &mut out_i32[..2]);
        sve::udiv_i64(&[8, 9], &[2, 3], &mut out_i64[..2]);
        sve::udiv_u32(&[8, 9], &[2, 3], &mut out_u32[..2]);
        sve::udiv_u64(&[8, 9], &[2, 3], &mut out_u64[..2]);
    }
    if out_i32[..2] != [4, 3]
        || out_i64[..2] != [4, 3]
        || out_u32[..2] != [4, 3]
        || out_u64[..2] != [4, 3]
    {
        return false;
    }
    let mut out_i32 = [0i32; 2];
    let mut out_i64 = [0i64; 2];
    let mut out_u32 = [0u32; 2];
    let mut out_u64 = [0u64; 2];
    unsafe {
        sve::sdivr_i32(&[1, 1], &[1, 1], &mut out_i32);
        sve::sdivr_i64(&[1, 1], &[1, 1], &mut out_i64);
        sve::udivr_u32(&[1, 1], &[1, 1], &mut out_u32);
        sve::udivr_u64(&[1, 1], &[1, 1], &mut out_u64);
    }
    if out_i32 != [1, 1] || out_i64 != [1, 1] || out_u32 != [1, 1] || out_u64 != [1, 1] {
        return false;
    }
    let mut fexpa_f32 = [0.0f32; 4];
    let mut fexpa_f64 = [0.0f64; 4];
    unsafe {
        sve::fexpa_f32(&[0.0; 4], &mut fexpa_f32);
        sve::fexpa_f64(&[0.0; 4], &mut fexpa_f64);
    }
    core::hint::black_box(&fexpa_f32);
    core::hint::black_box(&fexpa_f64);
    let ones_f32 = [1.0f32; 4];
    let ones_f64 = [1.0f64; 4];
    let mut out_f32 = [0.0f32; 4];
    let mut out_f64 = [0.0f64; 4];
    unsafe {
        sve::fdivr_f32(&ones_f32, &ones_f32, &mut out_f32);
        sve::fdivr_f64(&ones_f64, &ones_f64, &mut out_f64);
    }
    if out_f32 != [1.0; 4] || out_f64 != [1.0; 4] {
        return false;
    }
    unsafe {
        sve::fsubr_f32(&ones_f32, &ones_f32, &mut out_f32);
        sve::fsubr_f64(&ones_f64, &ones_f64, &mut out_f64);
    }
    if out_f32 != [0.0; 4] || out_f64 != [0.0; 4] {
        return false;
    }
    if unsafe { sve::andv_i8(&[0b1100, 0b1010]) } != 0b1000
        || unsafe { sve::andv_i16(&[0b1100, 0b1010]) } != 0b1000
        || unsafe { sve::andv_i32(&[0b1100, 0b1010]) } != 0b1000
        || unsafe { sve::andv_i64(&[0b1100, 0b1010]) } != 0b1000
        || unsafe { sve::andv_u8(&[0b1100, 0b1010]) } != 0b1000
        || unsafe { sve::andv_u16(&[0b1100, 0b1010]) } != 0b1000
        || unsafe { sve::andv_u32(&[0b1100, 0b1010]) } != 0b1000
        || unsafe { sve::andv_u64(&[0b1100, 0b1010]) } != 0b1000
        || unsafe { sve::eorv_i8(&[1, 3]) } != 2
        || unsafe { sve::eorv_i16(&[1, 3]) } != 2
        || unsafe { sve::eorv_i32(&[1, 3]) } != 2
        || unsafe { sve::eorv_i64(&[1, 3]) } != 2
        || unsafe { sve::eorv_u8(&[1, 3]) } != 2
        || unsafe { sve::eorv_u16(&[1, 3]) } != 2
        || unsafe { sve::eorv_u32(&[1, 3]) } != 2
        || unsafe { sve::eorv_u64(&[1, 3]) } != 2
        || unsafe { sve::fadda_f32(&[1.0, 2.0, 3.0], 4.0) } != 10.0
        || unsafe { sve::fadda_f64(&[1.0, 2.0, 3.0], 4.0) } != 10.0
    {
        return false;
    }

    unsafe { sve::lsl_i8(&[1, 2], 2, &mut o8[..2]) };
    if o8[..2] != [4, 8] {
        return false;
    }
    unsafe { sve::lsr_i16(&[8, 16], 3, &mut o16[..2]) };
    if o16[..2] != [1, 2] {
        return false;
    }
    unsafe { sve::asr_i64(&[-8, 8], 2, &mut o64[..2]) };
    if o64[..2] != [-2, 2] {
        return false;
    }
    o32[0] = 0;
    unsafe { sve::sli_i32(&[1], 1, &mut o32[..1]) };
    if o32[0] != 2 {
        return false;
    }
    o32[0] = 1;
    unsafe { sve::sri_i32(&[4], 2, &mut o32[..1]) };
    if o32[0] != 1 {
        return false;
    }
    unsafe { sve::srshl_i32(&[8], &[1], &mut o32[..1]) };
    if o32[0] != 4 {
        return false;
    }
    unsafe { sve::urshl_u32(&[8], &[2], &mut ou32[..1]) };
    if ou32[0] != 2 {
        return false;
    }
    unsafe { sve::sqshl_i32(&[1], &[2], &mut o32[..1]) };
    if o32[0] != 4 {
        return false;
    }
    unsafe { sve::uqshl_u32(&[1], &[3], &mut ou32[..1]) };
    if ou32[0] != 8 {
        return false;
    }
    unsafe { sve::sqrshl_i32(&[8], &[1], &mut o32[..1]) };
    if o32[0] != 4 {
        return false;
    }
    unsafe { sve::uqrshl_u32(&[8], &[2], &mut ou32[..1]) };
    if ou32[0] != 2 {
        return false;
    }
    unsafe { sve::srshr_i32(&[-8], 1, &mut o32[..1]) };
    if o32[0] != -4 {
        return false;
    }
    unsafe { sve::urshr_u32(&[8], 2, &mut ou32[..1]) };
    if ou32[0] != 2 {
        return false;
    }
    unsafe { sve::srsra_i32(&[8], 1, &[4], &mut o32[..1]) };
    if o32[0] != 8 {
        return false;
    }
    unsafe { sve::ursra_u32(&[8], 1, &[4], &mut ou32[..1]) };
    if ou32[0] != 8 {
        return false;
    }
    unsafe { sve::ssra_i32(&[8], 1, &[4], &mut o32[..1]) };
    if o32[0] != 8 {
        return false;
    }
    unsafe { sve::usra_u32(&[8], 1, &[4], &mut ou32[..1]) };
    if ou32[0] != 8 {
        return false;
    }
    unsafe { sve::sqshlu_i32(&[1], 2, &mut o32[..1]) };
    if o32[0] != 4 {
        return false;
    }

    unsafe { sve::fabd_f32(&[1.0, 5.0], &[3.0, 2.0], &mut of32[..2]) };
    if of32[..2] != [2.0, 3.0] {
        return false;
    }
    unsafe { sve::fmulx_f32(&[2.0, 3.0], &[4.0, 5.0], &mut of32[..2]) };
    if of32[..2] != [8.0, 15.0] {
        return false;
    }
    unsafe { sve::frecps_f32(&[1.0], &[1.0], &mut of32[..1]) };
    if of32[0] != 1.0 {
        return false;
    }
    unsafe { sve::frsqrts_f32(&[1.0], &[1.0], &mut of32[..1]) };
    if of32[0] != 1.0 {
        return false;
    }
    unsafe { sve::frecpe_f32(&[2.0], &mut of32[..1]) };
    if of32[0] == 0.0 || !of32[0].is_finite() {
        return false;
    }
    unsafe { sve::frsqrte_f32(&[4.0], &mut of32[..1]) };
    if of32[0] == 0.0 || !of32[0].is_finite() {
        return false;
    }
    unsafe { sve::fcmeq_f32(&[1.0, 2.0], &[1.0, 3.0], &mut ou32[..2]) };
    if ou32[..2] != [u32::MAX, 0] {
        return false;
    }
    unsafe { sve::fcmle_f64(&[1.0, 4.0], &[2.0, 3.0], &mut ou32[..2]) };
    if ou32[..2] != [u32::MAX, 0] {
        return false;
    }
    unsafe { sve::facge_f32(&[-3.0, 1.0], &[2.0, 2.0], &mut ou32[..2]) };
    if ou32[..2] != [u32::MAX, 0] {
        return false;
    }
    if unsafe { sve::reduce_maxnm_f32(&[1.0, 3.0, 2.0]) } != 3.0
        || unsafe { sve::reduce_minnm_f64(&[1.0, 3.0, 2.0]) } != 1.0
    {
        return false;
    }
    unsafe { sve::mad_f32(&[2.0], &[3.0], &[1.0], &mut of32[..1]) };
    if of32[0] != 7.0 {
        return false;
    }
    unsafe { sve::msb_f32(&[2.0], &[3.0], &[10.0], &mut of32[..1]) };
    if of32[0] != 4.0 {
        return false;
    }
    unsafe { sve::fcadd_f32(&[1.0, 2.0], &[3.0, 4.0], &mut of32[..2]) };
    if of32[..2] != [-3.0, 5.0] {
        return false;
    }
    unsafe { sve::fcmla_f32(&[1.0, 2.0], &[3.0, 4.0], &[10.0, 20.0], &mut of32[..2]) };
    if of32[..2].iter().any(|value| !value.is_finite()) {
        return false;
    }
    unsafe { sve::scale_f32(&[1.0, 2.0], &[1, 2], &mut of32[..2]) };
    if of32[..2] != [2.0, 8.0] {
        return false;
    }
    unsafe { sve::fmov_f32(&[1.5, -2.5], &mut of32[..2]) };
    if of32[..2] != [1.5, -2.5] {
        return false;
    }
    unsafe { sve::ftsmul_f32(&[2.0], &[1], &mut of32[..1]) };
    if !of32[0].is_finite() {
        return false;
    }

    unsafe { sve::fcvt_f32_f64(&[1.5, -2.5], &mut of32[..2]) };
    if of32[0] != 1.5 {
        return false;
    }
    if of32[1] != -2.5 {
        return false;
    }
    unsafe { sve::fcvt_f64_f32(&[1.5, -2.5], &mut of64[..2]) };
    if of64[0] != 1.5 {
        return false;
    }
    if of64[1] != -2.5 {
        return false;
    }
    unsafe { sve::fcvtzs_i32_f32(&[1.9, -1.9], &mut o32[..2]) };
    if o32[..2] != [1, -1] {
        return false;
    }
    unsafe { sve::fcvtzu_u32_f64(&[1.9, 2.9], &mut ou32[..2]) };
    if ou32[..2] != [1, 2] {
        return false;
    }
    unsafe { sve::scvtf_f32_i32(&[1, -2], &mut of32[..2]) };
    if of32[..2] != [1.0, -2.0] {
        return false;
    }
    unsafe { sve::ucvtf_f64_u64(&[1, 2], &mut of64[..2]) };
    if of64[..2] != [1.0, 2.0] {
        return false;
    }

    unsafe { sve::cpy_i32(&[1, 2], &mut o32[..2]) };
    if o32[..2] != [1, 2] {
        return false;
    }
    unsafe { sve::mov_i32(&[3, 4], &mut o32[..2]) };
    if o32[..2] != [3, 4] {
        return false;
    }
    unsafe { sve::movprfx_i32(&[5, 6], &mut o32[..2]) };
    if o32[..2] != [5, 6] {
        return false;
    }
    unsafe { sve::compact_i32(&[7, 8, 9], &mut o32[..3]) };
    if o32[..3] != [7, 8, 9] {
        return false;
    }
    unsafe { sve::ext_i32(&[1, 2, 3], &[4, 5, 6], 2, &mut o32[..3]) };
    if o32[..3] != [3, 4, 5] {
        return false;
    }
    unsafe { sve::tbl_i32(&[10, 11, 12], &[0, 1, 2], &mut o32[..3]) };
    if o32[..3] != [10, 11, 12] {
        return false;
    }
    unsafe { sve::sel_i32(&[1, 2], &[3, 4], &mut o32[..2]) };
    if o32[..2] != [1, 2] {
        return false;
    }
    unsafe { sve::csel_i32(&[5, 6], &[7, 8], &mut o32[..2]) };
    if o32[..2] != [5, 6] {
        return false;
    }
    if unsafe { sve::lasta_i32(&[1, 2, 3]) } != 3
        || unsafe { sve::lastb_i32(&[1, 2, 3]) } != 3
        || unsafe { sve::lasta_i32(&[1, 2, 3]) } != 3
    {
        return false;
    }
    unsafe { sve::dup_i32(7, &mut o32[..4]) };
    if o32[..4] != [7, 7, 7, 7] {
        return false;
    }
    unsafe { sve::fdup_f32(1.5, &mut of32[..3]) };
    if of32[..3] != [1.5, 1.5, 1.5] {
        return false;
    }
    unsafe { sve::index_i32(2, 3, &mut o32[..4]) };
    if o32[..4] != [2, 5, 8, 11] {
        return false;
    }
    unsafe { sve::uclamp_u32(&[1, 5, 9], 2, 7, &mut ou32[..3]) };
    if ou32[..3] != [2, 5, 7] {
        return false;
    }

    let bytes = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    unsafe { sve::ld1_u8(&bytes, &mut ou8[..16]) };
    if ou8[..16] != bytes {
        return false;
    }
    unsafe { sve::ld1b_u8(&[1, 2, 3], &mut ou32[..3]) };
    if ou32[..3] != [1, 2, 3] {
        return false;
    }
    unsafe { sve::ld1sb_i8(&[-1, -2], &mut o32[..2]) };
    if o32[..2] != [-1, -2] {
        return false;
    }
    unsafe { sve::ld1h_u16(&[1, 2], &mut ou32[..2]) };
    if ou32[..2] != [1, 2] {
        return false;
    }
    unsafe { sve::ld1w_u32(&[1, 2], &mut ou64[..2]) };
    if ou64[..2] != [1, 2] {
        return false;
    }
    unsafe { sve::ld1d_u64(&[1, 2], &mut ou64[..2]) };
    if ou64[..2] != [1, 2] {
        return false;
    }
    unsafe { sve::ld1rqb_u8(&bytes, &mut ou8[..16]) };
    if ou8[..16] != bytes {
        return false;
    }
    let quad = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut replicated = [0u8; 32];
    unsafe { sve::ld1rqb_u8(&quad, &mut replicated) };
    if replicated[..16] != quad || replicated[16..] != quad {
        return false;
    }
    unsafe { sve::ld1rb_u8(&[7], &mut ou32[..3]) };
    if ou32[..3] != [7, 7, 7] {
        return false;
    }
    unsafe { sve::ld1rsb_i8(&[-3], &mut o32[..2]) };
    if o32[..2] != [-3, -3] {
        return false;
    }
    unsafe { sve::ldnt1b_u8(&bytes[..4], &mut ou8[..4]) };
    if ou8[..4] != bytes[..4] {
        return false;
    }
    unsafe { sve::ldff1b_u8(&bytes[..4], &mut ou8[..4]) };
    if ou8[..4] != bytes[..4] {
        return false;
    }
    unsafe { sve::ldnf1b_u8(&bytes[..4], &mut ou8[..4]) };
    if ou8[..4] != bytes[..4] {
        return false;
    }
    unsafe { sve::st1_u8(&bytes[..4], &mut ou8[..4]) };
    if ou8[..4] != bytes[..4] {
        return false;
    }
    unsafe { sve::st1b_u32(&[258, 3], &mut ou8[..2]) };
    if ou8[..2] != [2, 3] {
        return false;
    }
    unsafe { sve::st1h_u32(&[0x12345], &mut ou16[..1]) };
    if ou16[0] != 0x2345 {
        return false;
    }
    unsafe { sve::st1w_u64(&[0x1_0000_0002], &mut ou32[..1]) };
    if ou32[0] != 2 {
        return false;
    }
    unsafe { sve::st1d_u64(&[1, 2], &mut ou64[..2]) };
    if ou64[..2] != [1, 2] {
        return false;
    }
    let interleaved = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let mut even = [0u8; 4];
    let mut odd = [0u8; 4];
    unsafe { sve::ld2b_u8(&interleaved, &mut even, &mut odd) };
    if even != [1, 3, 5, 7] || odd != [2, 4, 6, 8] {
        return false;
    }
    unsafe { sve::st2b_u8(&even, &odd, &mut ou8[..8]) };
    if ou8[..8] != interleaved {
        return false;
    }
    let interleaved3 = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut a3 = [0u8; 4];
    let mut b3 = [0u8; 4];
    let mut c3 = [0u8; 4];
    unsafe { sve::ld3b_u8(&interleaved3, &mut a3, &mut b3, &mut c3) };
    if a3 != [1, 4, 7, 10] || b3 != [2, 5, 8, 11] || c3 != [3, 6, 9, 12] {
        return false;
    }
    unsafe { sve::st3b_u8(&a3, &b3, &c3, &mut ou8[..12]) };
    if ou8[..12] != interleaved3 {
        return false;
    }
    let interleaved4 = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut a4 = [0u8; 4];
    let mut b4 = [0u8; 4];
    let mut c4 = [0u8; 4];
    let mut d4 = [0u8; 4];
    unsafe { sve::ld4b_u8(&interleaved4, &mut a4, &mut b4, &mut c4, &mut d4) };
    if a4 != [1, 5, 9, 13] || b4 != [2, 6, 10, 14] || c4 != [3, 7, 11, 15] || d4 != [4, 8, 12, 16] {
        return false;
    }
    unsafe { sve::st4b_u8(&a4, &b4, &c4, &d4, &mut ou8[..16]) };
    if ou8[..16] != interleaved4 {
        return false;
    }

    unsafe { sve::sdot_i32(&[1, 2, 3, 4], &[1, 1, 1, 1], &[0], &mut o32[..1]) };
    if o32[0] != 10 {
        return false;
    }
    unsafe { sve::udot_i32(&[1, 2, 3, 4], &[1, 1, 1, 1], &[1], &mut o32[..1]) };
    if o32[0] != 11 {
        return false;
    }
    unsafe { sve::sudot_i32(&[1, -1, 1, -1], &[1, 1, 1, 1], &[0], &mut o32[..1]) };
    if o32[0] != 0 {
        return false;
    }
    unsafe { sve::smmla_i32(&[1, 2, 3, 4], &[1, 1, 1, 1], &[0], &mut o32[..1]) };
    if o32[0] != 10 {
        return false;
    }
    unsafe { sve::saddlb_i8(&[1, 2], &[3, 4], &mut o16[..2]) };
    if o16[..2] != [4, 6] {
        return false;
    }
    unsafe { sve::smullb_i8(&[2, 3], &[4, 5], &mut o16[..2]) };
    if o16[..2] != [8, 15] {
        return false;
    }
    unsafe { sve::smlalb_i8(&[2, 3], &[4, 5], &[1, 2], &mut o16[..2]) };
    if o16[..2] != [9, 17] {
        return false;
    }
    true
}
#[target_feature(enable = "jsconv")]
unsafe fn jsconv_ops() -> bool {
    unsafe { fjcvtzs(1.9) == 1 && fjcvtzs(f64::NAN) == 0 && fjcvtzs(-1.5) == -1 }
}

#[target_feature(enable = "sve")]
unsafe fn f32mm_ops() -> bool {
    let a = [0.0f32; 64];
    let mut acc = [0.0f32; 64];
    unsafe { fmmla_f32(&a, &a, &mut acc) };
    acc.iter().all(|&value| value == 0.0)
}

#[target_feature(enable = "sve")]
unsafe fn f64mm_ops() -> bool {
    let a = [0.0f64; 64];
    let mut acc = [0.0f64; 64];
    unsafe { fmmla_f64(&a, &a, &mut acc) };
    acc.iter().all(|&value| value == 0.0)
}
