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
    if !neon_ext_ops() {
        return 25;
    }
    if !unsafe { fp16_ext_ops() } {
        return 26;
    }
    if !unsafe { sha3_xar_ops() } {
        return 27;
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
    if !i64_ok(
        rust_isa::aarch64::simd::neon::shl_i64(i64x2([1, 2]), 2),
        [4, 8],
    ) || lanes(shl_i32(i32x4([1, 2, 3, 4]), 2)) != [4, 8, 12, 16]
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
    if rust_isa::aarch64::simd::neon::zip2(a, b).to_bytes()
        != [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31]
    {
        return false;
    }
    if rust_isa::aarch64::simd::neon::uzp2(a, b).to_bytes()
        != [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]
    {
        return false;
    }
    if rust_isa::aarch64::simd::neon::trn2(a, b).to_bytes()
        != [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31]
    {
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
    let poly = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    unsafe { aese(one, one) }.to_bytes() != zero.to_bytes()
        && unsafe { aesd(one, one) }.to_bytes() != zero.to_bytes()
        && unsafe { aesmc(one) }.to_bytes() != zero.to_bytes()
        && unsafe { aesimc(one) }.to_bytes() != zero.to_bytes()
        && unsafe { neon_extra::pmull_p64(poly, poly) }.to_bytes() == poly.to_bytes()
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

use rust_isa::aarch64::crypto::sha3::xar;
use rust_isa::aarch64::simd::fp16 as fp16_extra;
use rust_isa::aarch64::simd::neon as neon_extra;
use rust_isa::aarch64::simd::neon::{dup_i8, dup_i16, dup_i32, dup_i64, ext, ins_i8};

fn u8x16(bytes: [u8; 16]) -> V128 {
    V128::from_bytes(bytes)
}

fn u16x8(values: [u16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn u16_lanes(value: V128) -> [u16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn u64x2(values: [u64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

fn u64_lanes(value: V128) -> [u64; 2] {
    let bytes = value.to_bytes();
    [
        u64::from_le_bytes(bytes[..8].try_into().unwrap()),
        u64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

fn f16_bits(value: u16) -> V128 {
    u16x8([value; 8])
}

fn f16_lane(value: V128, lane: usize) -> u16 {
    u16_lanes(value)[lane]
}

fn neon_ext_ops() -> bool {
    if !i64_ok(neon_extra::abs_i64(i64x2([-5, 6])), [5, 6]) {
        return false;
    }
    if neon_extra::sabd_i8(u8x16([1; 16]), u8x16([10; 16])).to_bytes() != [9; 16]
        || neon_extra::uabd_u8(u8x16([10; 16]), u8x16([1; 16])).to_bytes() != [9; 16]
        || neon_extra::sabd_i16(i16x8([1; 8]), i16x8([4; 8])) != i16x8([3; 8])
        || i16_lanes(neon_extra::sabdl_i8(u8x16([1; 16]), u8x16([10; 16]))) != [9; 8]
        || lanes(neon_extra::uabdl_u16(i16x8([10; 8]), i16x8([1; 8]))) != [9; 4]
    {
        return false;
    }
    if neon_extra::shadd_s8(u8x16([2; 16]), u8x16([6; 16])).to_bytes() != [4; 16]
        || neon_extra::shsub_s8(u8x16([6; 16]), u8x16([2; 16])).to_bytes() != [2; 16]
        || neon_extra::srhadd_s8(u8x16([1; 16]), u8x16([2; 16])).to_bytes() != [2; 16]
        || neon_extra::uhadd_u8(u8x16([2; 16]), u8x16([6; 16])).to_bytes() != [4; 16]
        || neon_extra::uhsub_u8(u8x16([6; 16]), u8x16([2; 16])).to_bytes() != [2; 16]
        || neon_extra::urhadd_u8(u8x16([1; 16]), u8x16([2; 16])).to_bytes() != [2; 16]
    {
        return false;
    }
    if i16_lanes(neon_extra::smull_s8(u8x16([2; 16]), u8x16([3; 16]))) != [6; 8] {
        return false;
    }
    if lanes(neon_extra::smull2_s16(i16x8([2; 8]), i16x8([3; 8]))) != [6; 4] {
        return false;
    }
    if !i64_ok(neon_extra::umull_u32(i32x4([2; 4]), i32x4([3; 4])), [6, 6]) {
        return false;
    }
    if i16_lanes(neon_extra::smlal_s8(
        i16x8([1; 8]),
        u8x16([2; 16]),
        u8x16([3; 16]),
    )) != [7; 8]
        || lanes(neon_extra::umlal2_u16(
            i32x4([9; 4]),
            i16x8([2; 8]),
            i16x8([3; 8]),
        )) != [15; 4]
        || i16_lanes(neon_extra::smlsl_s8(
            i16x8([10; 8]),
            u8x16([2; 16]),
            u8x16([3; 16]),
        )) != [4; 8]
        || !i64_ok(
            neon_extra::umlsl2_u32(i64x2([9; 2]), i32x4([2; 4]), i32x4([3; 4])),
            [3, 3],
        )
    {
        return false;
    }
    if lanes(neon_extra::sqdmull_s16(i16x8([3; 8]), i16x8([4; 8]))) != [24; 4]
        || !i64_ok(
            neon_extra::sqdmlal_s32(i64x2([1; 2]), i32x4([2; 4]), i32x4([3; 4])),
            [13, 13],
        )
        || lanes(neon_extra::sqdmlsl2_s16(
            i32x4([10; 4]),
            i16x8([2; 8]),
            i16x8([3; 8]),
        )) != [u32::MAX - 1; 4]
        || neon_extra::sqdmulh_s16(i16x8([2; 8]), i16x8([3; 8])) != i16x8([0; 8])
        || neon_extra::sqrdmulh_s16(i16x8([2; 8]), i16x8([3; 8])) != i16x8([0; 8])
    {
        return false;
    }
    if neon_extra::sqadd_s8(u8x16([127; 16]), u8x16([1; 16])).to_bytes() != [127; 16]
        || neon_extra::sqsub_s8(u8x16([0x80; 16]), u8x16([1; 16])).to_bytes() != [0x80; 16]
        || neon_extra::uqadd_u8(u8x16([255; 16]), u8x16([1; 16])).to_bytes() != [255; 16]
        || neon_extra::uqsub_u8(u8x16([0; 16]), u8x16([1; 16])).to_bytes() != [0; 16]
        || neon_extra::sqabs_s8(u8x16([0x80; 16])).to_bytes() != [0x7f; 16]
        || neon_extra::sqneg_s8(u8x16([0x80; 16])).to_bytes() != [0x7f; 16]
        || !i64_ok(
            neon_extra::sqadd_s64(i64x2([i64::MAX, 1]), i64x2([1, 1])),
            [i64::MAX, 2],
        )
    {
        return false;
    }
    if neon_extra::xtn_s8(i16x8([1, 2, 3, 4, 5, 6, 7, 8])).to_bytes()
        != [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]
        || neon_extra::xtn_u8(u16x8([1, 2, 3, 4, 5, 6, 7, 8])).to_bytes()
            != [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]
        || neon_extra::xtn2_s8(
            i16x8([9, 10, 11, 12, 13, 14, 15, 16]),
            u8x16([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
        .to_bytes()
            != [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    {
        return false;
    }
    if neon_extra::addhn_i16(u16x8([0x0102; 8]), u16x8([0; 8])).to_bytes()
        != [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        || neon_extra::raddhn_i16(u16x8([0x0180; 8]), u16x8([0; 8])).to_bytes()
            != [2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0]
        || neon_extra::subhn_i16(u16x8([0x0504; 8]), u16x8([0x0102; 8])).to_bytes()
            != [4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0]
        || neon_extra::rsubhn_i32(i32x4([0x0005_0004; 4]), i32x4([0x0001_0002; 4])).to_bytes()
            != [4, 0, 4, 0, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        || neon_extra::addhn2_i16(u16x8([0x0201; 8]), u16x8([0; 8]), u8x16([9; 16])).to_bytes()
            != [9, 9, 9, 9, 9, 9, 9, 9, 2, 2, 2, 2, 2, 2, 2, 2]
    {
        return false;
    }
    if neon_extra::pmul(u8x16([1; 16]), u8x16([1; 16])).to_bytes() != [1; 16]
        || u16_lanes(neon_extra::pmull_p8(u8x16([1; 16]), u8x16([1; 16])))
            != [1, 1, 1, 1, 1, 1, 1, 1]
    {
        return false;
    }
    if f32_lanes(neon_extra::scvtf_f32_s32(i32x4([1, 2, 3, 4]))) != [1.0, 2.0, 3.0, 4.0]
        || f32_lanes(neon_extra::ucvtf_f32_u32(i32x4([1, 2, 3, 4]))) != [1.0, 2.0, 3.0, 4.0]
        || f64_lanes(neon_extra::scvtf_f64_s64(i64x2([1, 2]))) != [1.0, 2.0]
        || f64_lanes(neon_extra::ucvtf_f64_u64(i64x2([1, 2]))) != [1.0, 2.0]
        || lanes(neon_extra::fcvtzs_s32_f32(f32x4([1.9, -1.9, 0.0, 2.0]))) != [1, u32::MAX, 0, 2]
        || lanes(neon_extra::fcvtzu_u32_f32(f32x4([1.9, 2.9, 0.0, 3.0]))) != [1, 2, 0, 3]
        || !i64_ok(neon_extra::fcvtzs_s64_f64(f64x2([1.9, -1.9])), [1, -1])
        || !i64_ok(neon_extra::fcvtzu_u64_f64(f64x2([1.9, 2.9])), [1, 2])
    {
        return false;
    }
    if neon_extra::shl_s8(u8x16([1; 16]), u8x16([1; 16])).to_bytes() != [2; 16]
        || neon_extra::sshl_s16(i16x8([1; 8]), i16x8([2; 8])) != i16x8([4; 8])
        || neon_extra::srshl_s16(i16x8([6; 8]), i16x8([1; 8])) != i16x8([12; 8])
        || neon_extra::sqshl_s8(u8x16([64; 16]), u8x16([2; 16])).to_bytes() != [127; 16]
        || neon_extra::sqrshl_s8(u8x16([64; 16]), u8x16([2; 16])).to_bytes() != [127; 16]
        || neon_extra::ushl_u8(u8x16([1; 16]), u8x16([1; 16])).to_bytes() != [2; 16]
        || neon_extra::urshl_u16(u16x8([6; 8]), i16x8([1; 8])) != u16x8([12; 8])
        || neon_extra::uqshl_u8(u8x16([255; 16]), u8x16([1; 16])).to_bytes() != [255; 16]
        || neon_extra::uqrshl_u8(u8x16([255; 16]), u8x16([1; 16])).to_bytes() != [255; 16]
    {
        return false;
    }
    if neon_extra::sshr_s16(i16x8([4; 8]), 2) != i16x8([1; 8])
        || neon_extra::srshr_s16(i16x8([6; 8]), 1) != i16x8([3; 8])
        || neon_extra::ssra_s16(i16x8([4; 8]), i16x8([1; 8]), 1) != i16x8([3; 8])
        || neon_extra::srsra_s16(i16x8([4; 8]), i16x8([1; 8]), 1) != i16x8([3; 8])
        || neon_extra::shrn_i16(
            u16x8([
                0x0100, 0x0200, 0x0300, 0x0400, 0x0500, 0x0600, 0x0700, 0x0800,
            ]),
            8,
        )
        .to_bytes()
            != [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]
        || neon_extra::shrn2_i16(u16x8([0x0100; 8]), u8x16([9; 16]), 8).to_bytes()
            != [9, 9, 9, 9, 9, 9, 9, 9, 1, 1, 1, 1, 1, 1, 1, 1]
        || neon_extra::sqshrn_i16(i16x8([0x7fff; 8]), 1).to_bytes()
            != [
                0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || neon_extra::sqshrun_i16(i16x8([0x7fff; 8]), 1).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || neon_extra::sqrshrn_i16(i16x8([0x7fff; 8]), 1).to_bytes()
            != [
                0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || neon_extra::uqshrn_i16(u16x8([0xffff; 8]), 1).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        || neon_extra::uqrshrn_i16(u16x8([0xffff; 8]), 1).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
    {
        return false;
    }
    if neon_extra::shll_i8(u8x16([1; 16]), 1) != u16x8([2; 8]) {
        return false;
    }
    if neon_extra::sli_s8(u8x16([0x0f; 16]), u8x16([0x0a; 16]), 4).to_bytes() != [0xfa; 16] {
        return false;
    }
    if neon_extra::sri_s8(u8x16([0xf0; 16]), u8x16([0xa0; 16]), 4).to_bytes() != [0xaf; 16] {
        return false;
    }
    if neon_extra::sqshlu_s8(u8x16([100; 16]), 2).to_bytes() != [255; 16] {
        return false;
    }
    if neon_extra::urecpe_u32(i32x4([1; 4])).to_bytes() == [0; 16]
        || neon_extra::ursqrte_u32(i32x4([4; 4])).to_bytes() == [0; 16]
        || neon_extra::cnt_u64(0b1011) != 3
        || neon_extra::clz_u64(1) != 63
        || neon_extra::cls_u64(0) != 63
    {
        return false;
    }
    if neon_extra::frecpx_f32(1.0) == 0.0
        || neon_extra::frecpx_f64(1.0) == 0.0
        || f32_lanes(neon_extra::frecpe_f32(f32x4([1.0, 2.0, 4.0, 8.0])))
            .iter()
            .any(|value| *value == 0.0 || !value.is_finite())
        || f64_lanes(neon_extra::frsqrte_f64(f64x2([1.0, 4.0])))
            .iter()
            .any(|value| *value == 0.0 || !value.is_finite())
        || f32_lanes(neon_extra::frecps_f32(f32x4([1.0; 4]), f32x4([1.0; 4]))) != [1.0; 4]
        || f64_lanes(neon_extra::frsqrts_f64(f64x2([1.0; 2]), f64x2([1.0; 2]))) != [1.0; 2]
    {
        return false;
    }
    neon_ext_memory_ops() && neon_ext_lane_ops()
}

fn neon_ext_memory_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V128::from_bytes(core::array::from_fn(|index| (index + 16) as u8));
    if unsafe { neon_extra::ld1(a.as_ptr()) } != a {
        return false;
    }
    let mut one = [0u8; 16];
    unsafe { neon_extra::st1(one.as_mut_ptr(), a) };
    if one != a.to_bytes() {
        return false;
    }
    let mut interleaved = [0u8; 32];
    unsafe { neon_extra::st2(interleaved.as_mut_ptr(), a, b) };
    let (p, q) = unsafe { neon_extra::ld2(interleaved.as_ptr()) };
    if p != a || q != b {
        return false;
    }
    let mut interleaved4 = [0u8; 64];
    unsafe { neon_extra::st4(interleaved4.as_mut_ptr(), a, b, a, b) };
    let (p, q, r, s) = unsafe { neon_extra::ld4(interleaved4.as_ptr()) };
    if p != a || q != b || r != a || s != b {
        return false;
    }
    let mut interleaved3 = [0u8; 48];
    unsafe { neon_extra::st3(interleaved3.as_mut_ptr(), a, b, a) };
    let (p, q, r) = unsafe { neon_extra::ld3(interleaved3.as_ptr()) };
    if p != a || q != b || r != a {
        return false;
    }
    let mut pair = [0u8; 32];
    unsafe { neon_extra::stp(pair.as_mut_ptr(), a, b) };
    let (p, q) = unsafe { neon_extra::ldp(pair.as_ptr()) };
    if p != a || q != b {
        return false;
    }
    unsafe { neon_extra::stnp(pair.as_mut_ptr(), b, a) };
    let (p, q) = unsafe { neon_extra::ldnp(pair.as_ptr()) };
    if p != b || q != a {
        return false;
    }
    let one_element = [0x34u8, 0x12, 0, 0, 0, 0, 0, 0];
    if u16_lanes(unsafe { neon_extra::ld1r_i16(one_element.as_ptr()) }) != [0x1234; 8] {
        return false;
    }
    let two_elements = [1u8, 0, 2, 0, 0, 0, 0, 0];
    let (p, q) = unsafe { neon_extra::ld2r_i16(two_elements.as_ptr()) };
    if u16_lanes(p) != [1; 8] || u16_lanes(q) != [2; 8] {
        return false;
    }
    let three_elements = [1u8, 0, 2, 0, 3, 0, 0, 0];
    let (p, q, r) = unsafe { neon_extra::ld3r_i16(three_elements.as_ptr()) };
    if u16_lanes(p) != [1; 8] || u16_lanes(q) != [2; 8] || u16_lanes(r) != [3; 8] {
        return false;
    }
    let four_elements = [1u8, 0, 2, 0, 3, 0, 4, 0];
    let (p, q, r, s) = unsafe { neon_extra::ld4r_i16(four_elements.as_ptr()) };
    u16_lanes(p) == [1; 8]
        && u16_lanes(q) == [2; 8]
        && u16_lanes(r) == [3; 8]
        && u16_lanes(s) == [4; 8]
}

fn neon_ext_lane_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    if dup_i8::<3>(a).to_bytes() != [3; 16]
        || u16_lanes(dup_i16::<2>(u16x8([1, 2, 3, 4, 5, 6, 7, 8]))) != [3; 8]
        || lanes(dup_i32::<1>(i32x4([1, 2, 3, 4]))) != [2; 4]
        || !i64_ok(dup_i64::<1>(i64x2([7, 9])), [9, 9])
        || ins_i8::<2>(a, 0xaa_u8 as i8).to_bytes()
            != [0, 1, 0xaa, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        || ext::<4>(a, a).to_bytes() != [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3]
    {
        return false;
    }
    true
}

#[target_feature(enable = "fp16")]
unsafe fn fp16_ext_ops() -> bool {
    let one = f16_bits(0x3c00);
    let two = f16_bits(0x4000);
    let four = f16_bits(0x4400);
    let neg_one = f16_bits(0xbc00);
    let check = |value: V128, expected: u16| f16_lane(value, 0) == expected;
    if !check(unsafe { fp16_extra::fabd_f16(one, four) }, 0x4200) {
        return false;
    }
    if !check(unsafe { fp16_extra::fabs_f16(neg_one) }, 0x3c00) {
        return false;
    }
    if !check(unsafe { fp16_extra::fneg_f16(one) }, 0xbc00) {
        return false;
    }
    if !check(unsafe { fp16_extra::fsqrt_f16(four) }, 0x4000) {
        return false;
    }
    if !check(unsafe { fp16_extra::fmax_f16(one, two) }, 0x4000) {
        return false;
    }
    if !check(unsafe { fp16_extra::fmaxnm_f16(one, two) }, 0x4000) {
        return false;
    }
    if !check(unsafe { fp16_extra::fmin_f16(one, two) }, 0x3c00) {
        return false;
    }
    if !check(unsafe { fp16_extra::fminnm_f16(one, two) }, 0x3c00) {
        return false;
    }
    if !check(unsafe { fp16_extra::frecps_f16(one, one) }, 0x3c00) {
        return false;
    }
    if !check(unsafe { fp16_extra::frsqrts_f16(one, one) }, 0x3c00) {
        return false;
    }
    if unsafe { fp16_extra::facge_f16(two, one) }.to_bytes() != [0xff; 16]
        || unsafe { fp16_extra::facgt_f16(one, two) }.to_bytes() != [0; 16]
        || unsafe { fp16_extra::fcmeq_f16(one, one) }.to_bytes() != [0xff; 16]
        || unsafe { fp16_extra::fcmge_f16(two, one) }.to_bytes() != [0xff; 16]
        || unsafe { fp16_extra::fcmgt_f16(one, two) }.to_bytes() != [0; 16]
        || unsafe { fp16_extra::fcmle_f16(one, two) }.to_bytes() != [0xff; 16]
        || unsafe { fp16_extra::fcmlt_f16(one, two) }.to_bytes() != [0xff; 16]
    {
        return false;
    }
    if unsafe { fp16_extra::frecpe_f16(one) }.to_bytes() == [0; 16]
        || unsafe { fp16_extra::frsqrte_f16(one) }.to_bytes() == [0; 16]
    {
        return false;
    }
    let zero_f32 = f32x4([0.0; 4]);
    let low = unsafe { fp16_extra::fmlal_f16(zero_f32, one, two) };
    let high = unsafe { fp16_extra::fmlal2_f16(zero_f32, two, two) };
    let sub = unsafe { fp16_extra::fmlsl_f16(f32x4([10.0; 4]), one, two) };
    if f32_lanes(low)[..2] != [2.0, 2.0]
        || f32_lanes(high)[..2] != [4.0, 4.0]
        || f32_lanes(sub)[..2] != [8.0, 8.0]
    {
        return false;
    }
    if f32_lanes(unsafe { fp16_extra::fcvtl_f32_f16(one) })[..4] != [1.0; 4]
        || f32_lanes(unsafe { fp16_extra::fcvtl2_f32_f16(one) })[..4] != [1.0; 4]
        || f16_lane(unsafe { fp16_extra::fcvtn_f16_f32(f32x4([1.0; 4])) }, 0) != 0x3c00
        || f16_lane(
            unsafe { fp16_extra::fcvtn2_f16_f32(f32x4([1.0; 4]), f16_bits(0)) },
            4,
        ) != 0x3c00
        || f16_lane(unsafe { fp16_extra::fcvtxn_f16_f64(f64x2([1.0; 2])) }, 0) != 0x3c00
    {
        return false;
    }
    if lanes(unsafe { fp16_extra::fcvtzs_f16_s32(two) }) != [2; 4]
        || !i64_ok(unsafe { fp16_extra::fcvtzs_f16_s64(two) }, [2, 2])
        || lanes(unsafe { fp16_extra::fcvtzu_f16_u32(two) }) != [2; 4]
        || !i64_ok(unsafe { fp16_extra::fcvtzu_f16_u64(two) }, [2, 2])
        || f16_lane(unsafe { fp16_extra::scvtf_f16_s32(i32x4([1; 4])) }, 0) != 0x3c00
        || f16_lane(unsafe { fp16_extra::scvtf_f16_s64(i64x2([2; 2])) }, 0) != 0x4000
        || f16_lane(unsafe { fp16_extra::ucvtf_f16_u32(i32x4([2; 4])) }, 0) != 0x4000
        || f16_lane(unsafe { fp16_extra::ucvtf_f16_u64(i64x2([1; 2])) }, 0) != 0x3c00
        || unsafe { fp16_extra::fcvt_f16_f32(1.0) } != 0x3c00
        || unsafe { fp16_extra::fcvt_f32_f16(0x3c00) } != 1.0
        || unsafe { fp16_extra::fcvt_f16_f64(2.0) } != 0x4000
        || unsafe { fp16_extra::fcvt_f64_f16(0x4000) } != 2.0
    {
        return false;
    }
    true
}

#[target_feature(enable = "sha3")]
unsafe fn sha3_xar_ops() -> bool {
    let a = u64x2([1, 0]);
    let b = u64x2([0, 0]);
    u64_lanes(unsafe { xar::<1>(a, b) }) == [0x8000_0000_0000_0000, 0]
        && u64_lanes(unsafe { xar::<0>(a, b) }) == [1, 0]
        && u64_lanes(unsafe { xar::<63>(a, b) }) == [2, 0]
}
