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
use rust_isa::aarch64::simd::fcma::fcmla;
use rust_isa::aarch64::simd::fhm::fmla_f16;
use rust_isa::aarch64::simd::fp16::{fadd_f16, fmul_f16};
use rust_isa::aarch64::simd::i8mm::{smmla, usmmla};
use rust_isa::aarch64::simd::neon::{
    V128, abs_i32, add, add_f64, add_i8, add_i16, add_i32, add_i64, and, bic, cmpeq_i8, cmpeq_i16,
    cmpeq_i32, cmpgt_i8, cmpgt_i16, cmpgt_i32, eor, extract_i32, fabs, fadd, fdiv, fmax, fmin,
    fmul, fneg, fsqrt, fsub, max_i8, max_i16, max_i32, min_i8, min_i16, min_i32, mla_s8, mla_s16,
    mla_s32, mls_s8, mls_s16, mls_s32, mul, mul_f64, mul_i8, mul_i16, mul_i32, neg_i32, not, orn,
    orr, replace_i32, sar_i8, sar_i16, sar_i32, shl_i32, shr_i16, smax_s8, smax_s16, smax_s32,
    smaxp_s8, smaxp_s16, smaxp_s32, smin_s8, smin_s16, smin_s32, sminp_s8, sminp_s16, sminp_s32,
    splat_i32, sub_i8, sub_i16, sub_i32, sub_i64, tbl, trn1, umax_u8, umax_u16, umax_u32, umaxp_u8,
    umaxp_u16, umaxp_u32, umin_u8, umin_u16, umin_u32, uminp_u8, uminp_u16, uminp_u32, uzp1,
    v128_load, v128_store, zip1,
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
}

#[target_feature(enable = "fcma")]
unsafe fn fcma_ops() -> bool {
    let a = f32x4([1.0, 0.0, 1.0, 0.0]);
    let b = f32x4([2.0, 0.0, 2.0, 0.0]);
    let out = unsafe { fcmla(V128::from_bytes([0; 16]), a, b) }.to_bytes();
    f32_lanes(V128::from_bytes(out)) == [2.0, 0.0, 2.0, 0.0]
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
    u16::from_le_bytes(out[0..2].try_into().unwrap()) == 0x4000
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

#[target_feature(enable = "sve")]
unsafe fn sve_ops() -> bool {
    let a = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let b = [10.0f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0];
    let mut out = [0.0f32; 7];
    unsafe { sve::add_f32(&a, &b, &mut out) };
    if out != [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0] {
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
