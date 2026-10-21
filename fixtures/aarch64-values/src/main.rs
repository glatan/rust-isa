#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::aarch64::crypto::aes::{aesd, aese, aesimc, aesmc};
use rust_isa::aarch64::crypto::crc::crc32b;
use rust_isa::aarch64::crypto::sha2::{sha256h, sha256h2, sha256su0, sha256su1};
use rust_isa::aarch64::crypto::sha3::{bcax, eor3, rax1};
use rust_isa::aarch64::crypto::sm4::{sm4e, sm4ekey};
use rust_isa::aarch64::simd::dotprod::{sdot, udot};
use rust_isa::aarch64::simd::f32mm::fmmla_f32;
use rust_isa::aarch64::simd::f64mm::fmmla_f64;
use rust_isa::aarch64::simd::fcma::{fcmla, fcmla_rot180, fcmla_rot270};
use rust_isa::aarch64::simd::fhm::{fmla_f16, fmls_f16};
use rust_isa::aarch64::simd::fp16::{fadd_f16, fmul_f16};
use rust_isa::aarch64::simd::i8mm::{smmla, ummla, usmmla};
use rust_isa::aarch64::simd::jsconv::fjcvtzs;
use rust_isa::aarch64::simd::neon::{
    V128, abs_i32, add, add_f64, add_i8, add_i16, add_i32, add_i64, addp_s8, addp_s16, addp_s32,
    addv_s8, addv_s16, addv_s32, and, bic, bif, bit, bsl, cls_i8, cls_i16, cls_i32, clz_i8,
    clz_i16, clz_i32, cmge_i8, cmge_i16, cmge_i32, cmle_i8, cmle_i16, cmle_i32, cmlt_i8, cmlt_i16,
    cmlt_i32, cmpeq_i8, cmpeq_i16, cmpeq_i32, cmpeq_i64, cmpgt_i8, cmpgt_i16, cmpgt_i32, cmpgt_i64,
    cmtst_i8, cmtst_i16, cmtst_i32, cnt_i8, eor, extract_i32, fabd_f32, fabd_f64, fabs, facge_f32,
    facge_f64, facgt_f32, facgt_f64, fadd, faddp_f32, faddp_f64, faddv_f32, faddv_f64, fcmeq_f32,
    fcmeq_f64, fcmge_f32, fcmge_f64, fcmgt_f32, fcmgt_f64, fcmle_f32, fcmle_f64, fcmlt_f32,
    fcmlt_f64, fdiv, fmax, fmaxnm_f32, fmaxnm_f64, fmaxnmv_f32, fmaxnmv_f64, fmaxv_f32, fmaxv_f64,
    fmin, fminnm_f32, fminnm_f64, fminnmv_f32, fminnmv_f64, fminv_f32, fminv_f64, fmla_f32,
    fmla_f64, fmls_f32, fmls_f64, fmul, fmulx_f32, fmulx_f64, fneg, fsqrt, fsub, max_i8, max_i16,
    max_i32, min_i8, min_i16, min_i32, mla_s8, mla_s16, mla_s32, mls_s8, mls_s16, mls_s32, mul,
    mul_f64, mul_i8, mul_i16, mul_i32, neg_i32, not, orn, orr, rbit, replace_i32, rev16, rev32,
    rev64, saddl_s8, saddl_s16, saddl_s32, saddw_s8, saddw_s16, saddw_s32, sar_i8, sar_i16,
    sar_i32, shl_i32, shr_i16, smax_s8, smax_s16, smax_s32, smaxp_s8, smaxp_s16, smaxp_s32,
    smaxv_s8, smaxv_s16, smaxv_s32, smin_s8, smin_s16, smin_s32, sminp_s8, sminp_s16, sminp_s32,
    sminv_s8, sminv_s16, sminv_s32, splat_i32, ssubl_s8, ssubl_s16, ssubl_s32, ssubw_s8, ssubw_s16,
    ssubw_s32, sub_i8, sub_i16, sub_i32, sub_i64, tbl, trn1, uaddl_u8, uaddl_u16, uaddl_u32,
    uaddw_u8, uaddw_u16, uaddw_u32, umax_u8, umax_u16, umax_u32, umaxp_u8, umaxp_u16, umaxp_u32,
    umaxv_u8, umaxv_u16, umaxv_u32, umin_u8, umin_u16, umin_u32, uminp_u8, uminp_u16, uminp_u32,
    uminv_u8, uminv_u16, uminv_u32, usubl_u8, usubl_u16, usubl_u32, usubw_u8, usubw_u16, usubw_u32,
    uzp1, v128_load, v128_store, zip1,
};
use rust_isa::aarch64::simd::sve;

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

fn run() -> u32 {
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
        return 17;
    }
    0
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
    let out = unsafe { kernel_dot(&a, &b, &acc) }.to_bytes();
    for lane in 0..4 {
        let value = i32::from_le_bytes(out[lane * 4..lane * 4 + 4].try_into().unwrap());
        if value != 16 {
            return false;
        }
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

fn table() -> bool {
    let source = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let reverse = V128::from_bytes(core::array::from_fn(|index| 15 - index as u8));
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
        min_i8(
            V128::from_bytes([1, 0xff, 3, 0x80, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            V128::from_bytes([0; 16]),
        ),
        0xff,
    ) {
        return false;
    }
    if i16_lanes(max_i16(i16x8([1, -2, 3, -4, 5, -6, 7, -8]), i16x8([0; 8])))
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
    if i8_signed(
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
    if f64_lanes(add_f64(f64x2([1.0, 2.0]), f64x2([3.0, 4.0]))) != [4.0, 6.0]
        || f64_lanes(mul_f64(f64x2([2.0, 3.0]), f64x2([4.0, 5.0]))) != [8.0, 15.0]
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
    u16::from_le_bytes(sum[0..2].try_into().unwrap()) == 0x4200
        && u16::from_le_bytes(product[0..2].try_into().unwrap()) == 0x4000
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

#[target_feature(enable = "i8mm")]
unsafe fn i8mm_ops() -> bool {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([1; 16]);
    let zero = V128::from_bytes([0; 16]);
    unsafe { smmla(zero, a, b) }.to_bytes() != zero.to_bytes()
        && unsafe { usmmla(zero, a, b) }.to_bytes() != zero.to_bytes()
        && unsafe { ummla(zero, a, b) }.to_bytes() != zero.to_bytes()
}

#[target_feature(enable = "fcma")]
unsafe fn fcma_ops() -> bool {
    let a = f32x4([1.0, 0.0, 1.0, 0.0]);
    let b = f32x4([2.0, 0.0, 2.0, 0.0]);
    let zero = V128::from_bytes([0; 16]);
    let out = unsafe { fcmla(zero, a, b) };
    let rot180 = unsafe { fcmla_rot180(zero, a, b) };
    let rot270 = unsafe { fcmla_rot270(zero, a, b) };
    out.to_bytes() != zero.to_bytes()
        && rot180.to_bytes() != zero.to_bytes()
        && rot180.to_bytes() != out.to_bytes()
        && (rot270.to_bytes() == zero.to_bytes() || rot270.to_bytes() != out.to_bytes())
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
    let out = unsafe { fmla_f16(V128::from_bytes([0; 16]), one, two) }.to_bytes();
    let sub = unsafe { fmls_f16(V128::from_bytes([0; 16]), one, two) }.to_bytes();
    u16::from_le_bytes(out[0..2].try_into().unwrap()) == 0x4000
        && u16::from_le_bytes(sub[0..2].try_into().unwrap()) == 0xc000
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
    crc == 0x340b_c6d9
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
    unsafe { sve::mul_f32(&a, &b, &mut out) };
    if out != [10.0, 40.0, 90.0, 160.0, 250.0, 360.0, 490.0] {
        return false;
    }
    let a = [1i32, 2, 3, 4, 5];
    let b = [10i32, 20, 30, 40, 50];
    let mut out = [0i32; 5];
    unsafe { sve::add_i32(&a, &b, &mut out) };
    out == [11, 22, 33, 44, 55]
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
