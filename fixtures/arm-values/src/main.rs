#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::arm::crypto::aes::{aesd, aese, aesimc, aesmc};
use rust_isa::arm::crypto::crc::crc32b;
use rust_isa::arm::crypto::sha2::{sha256h, sha256h2, sha256su0, sha256su1};
use rust_isa::arm::simd::dotprod::{sdot, udot};
use rust_isa::arm::simd::fp16;
use rust_isa::arm::simd::mve;
use rust_isa::arm::simd::mve_fp;
use rust_isa::arm::simd::neon::{
    V128, add, add_i8, add_i16, add_i64, and, cmpeq_i8, cmpeq_i16, cmpeq_i32, cmpgt_i8, cmpgt_i16,
    cmpgt_i32, eor, fadd, fmul, fsub, mul, mul_i16, mul_i32, orr, splat_i32, sub_i8, sub_i16,
    sub_i32, sub_i64, trn1, uzp1, v128_load, v128_store, zip1,
};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "mov r7, #1",
            "svc #0",
            in("r0") code as u32,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "mov r7, #1",
            "svc #0",
            in("r0") 255u32,
            options(noreturn, nostack)
        );
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_add(a: &V128, b: &V128) -> V128 {
    unsafe { add(add(*a, *b), *b) }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_mul(a: &V128, b: &V128) -> V128 {
    unsafe { mul(mul(*a, *b), *b) }
}

#[cfg(not(feature = "mve"))]
fn run() -> u32 {
    if !load_store() {
        return 1;
    }
    if !integer() {
        return 2;
    }
    if !ops() {
        return 3;
    }
    if !float_ops() {
        return 4;
    }
    if !shuffle_ops() {
        return 5;
    }
    if !dotprod_ops() {
        return 6;
    }
    if !fp16_ops() {
        return 7;
    }
    if !aes_ops() {
        return 8;
    }
    if !sha2_ops() {
        return 9;
    }
    if !crc_ops() {
        return 10;
    }
    0
}

#[cfg(feature = "mve")]
fn run() -> u32 {
    if !mve_ops() {
        return 1;
    }
    if !mve_memory() {
        return 2;
    }
    if !mve_fp_ops() {
        return 3;
    }
    0
}

fn dotprod_ops() -> bool {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    let zero = V128::from_bytes([0; 16]);
    unsafe { lanes(sdot(zero, a, b)) == [8; 4] && lanes(udot(zero, a, b)) == [8; 4] }
}

fn fp16_ops() -> bool {
    let one = f16x8([0x3c00; 8]);
    let two = f16x8([0x4000; 8]);
    let three = f16x8([0x4200; 8]);
    let four = f16x8([0x4400; 8]);
    let nine = f16x8([0x4880; 8]);
    let ten = f16x8([0x4900; 8]);
    unsafe {
        if f16_lanes(fp16::add(one, two)) != [0x4200; 8]
            || f16_lanes(fp16::mul(one, two)) != [0x4000; 8]
            || f16_lanes(fp16::sub(four, one)) != [0x4200; 8]
        {
            return false;
        }
        if f16_lanes(fp16::abs(f16x8([
            0xbc00, 0x4000, 0xc200, 0x4400, 0, 0, 0, 0,
        ]))) != [0x3c00, 0x4000, 0x4200, 0x4400, 0, 0, 0, 0]
        {
            return false;
        }
        if f16_lanes(fp16::neg(one)) != [0xbc00; 8]
            || f16_lanes(fp16::max(one, two)) != [0x4000; 8]
            || f16_lanes(fp16::min(one, two)) != [0x3c00; 8]
            || f16_lanes(fp16::div(four, two)) != [0x4000; 8]
            || f16_lanes(fp16::fma(one, two, three)) != [0x4700; 8]
            || f16_lanes(fp16::fms(ten, two, three)) != [0x4400; 8]
            || f16_lanes(fp16::sqrt(four)) != [0x4000; 8]
            || f16_lanes(fp16::sqrt(nine)) != [0x4200; 8]
        {
            return false;
        }
        if fp16::cmp(one, one).to_bytes() != [0xff; 16] || fp16::cmp(one, two).to_bytes() != [0; 16]
        {
            return false;
        }
        if f32_lanes(fp16::cvt_f32(one)) != [1.0, 1.0, 1.0, 1.0]
            || f32_lanes(fp16::cvt_high_f32(f16x8([
                0x3c00, 0x4000, 0x4200, 0x4400, 0x4500, 0x4700, 0x4800, 0x4900,
            ]))) != [5.0, 7.0, 8.0, 10.0]
        {
            return false;
        }
        if f16_lanes(fp16::cvt_f16(f32x4([1.0, 2.0, 3.0, 4.0])))
            != [0x3c00, 0x4000, 0x4200, 0x4400, 0, 0, 0, 0]
        {
            return false;
        }
        f16_lanes(fp16::cvt_high_f16(one, f32x4([2.0, 3.0, 4.0, 5.0])))
            == [
                0x3c00, 0x3c00, 0x3c00, 0x3c00, 0x4000, 0x4200, 0x4400, 0x4500,
            ]
    }
}

fn mve_ops() -> bool {
    unsafe {
        if lanes(mve::abs(i32x4([-1, 2, -3, 4]))) != [1, 2, 3, 4]
            || lanes(mve::add(i32x4([1, 2, 3, 4]), i32x4([10, 20, 30, 40]))) != [11, 22, 33, 44]
            || lanes(mve::sub(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4]))) != [9, 18, 27, 36]
            || lanes(mve::mul(i32x4([2, 3, 4, 5]), i32x4([6, 7, 8, 9]))) != [12, 21, 32, 45]
        {
            return false;
        }
        if mve::addv(i32x4([1, 2, 3, 4])) != 10
            || mve::addlv(i32x4([1, 2, 3, 4])) != 10
            || mve::maxv(i32x4([3, 1, 4, 2])) != 4
            || mve::minv(i32x4([3, 1, 4, 2])) != 1
        {
            return false;
        }
        let a = u32x4([0b1010_1010; 4]);
        let b = u32x4([0b1100_1100; 4]);
        if u32_lanes(mve::and(a, b)) != [0b1000_1000; 4]
            || u32_lanes(mve::orr(a, b)) != [0b1110_1110; 4]
            || u32_lanes(mve::eor(a, b)) != [0b0110_0110; 4]
            || u32_lanes(mve::bic(u32x4([0xffff_ffff; 4]), u32x4([0x0f0f_0f0f; 4])))
                != [0xf0f0_f0f0; 4]
            || u32_lanes(mve::orn(u32x4([0x0f0f_0f0f; 4]), u32x4([0x00ff_00ff; 4])))
                != [0xff0f_ff0f; 4]
            || u32_lanes(mve::mvn(u32x4([0; 4]))) != [0xffff_ffff; 4]
        {
            return false;
        }
        if lanes(mve::abd(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4]))) != [9, 18, 27, 36]
            || mve::abav(0, V128::from_bytes([1; 16]), V128::from_bytes([3; 16])) != 32
        {
            return false;
        }
        if lanes(mve::brsr(mve::dup(1), 32)) != [0x8000_0000u32 as i32; 4]
            || lanes(mve::clz(mve::dup(1))) != [31; 4]
            || lanes(mve::clz(mve::dup(0))) != [32; 4]
        {
            return false;
        }
        if lanes(mve::cmp(mve::dup(7), mve::dup(7))) != [-1; 4]
            || lanes(mve::cmp(mve::dup(7), mve::dup(8))) != [0; 4]
            || mve::ctp(5) != 0x1f
        {
            return false;
        }
        if lanes(mve::dup(7)) != [7; 4] || mve::mov(a) != a {
            return false;
        }
        if lanes(mve::hadd(i32x4([2; 4]), i32x4([6; 4]))) != [4; 4]
            || lanes(mve::hsub(i32x4([2; 4]), i32x4([6; 4]))) != [-2; 4]
            || lanes(mve::max(i32x4([1, 5, 2, 7]), i32x4([3, 4, 6, 0]))) != [3, 5, 6, 7]
            || lanes(mve::min(i32x4([1, 5, 2, 7]), i32x4([3, 4, 6, 0]))) != [1, 4, 2, 0]
        {
            return false;
        }
        if u32_lanes(mve::idup(10, 1)) != [10, 11, 12, 13]
            || u32_lanes(mve::idup(0, 2)) != [0, 2, 4, 6]
            || u32_lanes(mve::iwdup(0, 3, 1)) != [0, 1, 2, 0]
        {
            return false;
        }
        let acc = i32x4([1, 2, 3, 4]);
        let x = i32x4([2, 3, 4, 5]);
        let y = i32x4([6, 7, 8, 9]);
        if lanes(mve::mla(acc, x, y)) != [13, 23, 35, 49]
            || lanes(mve::mls(acc, x, y)) != [-11, -19, -29, -41]
        {
            return false;
        }
        let x16 = i16x8([1, 2, 3, 4, 5, 6, 7, 8]);
        let y16 = i16x8([5, 6, 7, 8, 9, 10, 11, 12]);
        if lanes(mve::mull(x16, y16)) != [5, 21, 45, 77]
            || lanes(mve::mlal(acc, x16, y16)) != [6, 23, 48, 81]
            || lanes(mve::mlsl(acc, x16, y16)) != [-4, -19, -42, -73]
        {
            return false;
        }
        let half = mve::dup(0x4000_0000);
        if lanes(mve::qdmulh(half, half)) != [0x2000_0000; 4]
            || lanes(mve::qrdmulh(half, half)) != [0x2000_0000; 4]
            || lanes(mve::rmulh(half, half)) != [0x1000_0000; 4]
        {
            return false;
        }
        let quarter = i16x8([0x4000; 8]);
        if lanes(mve::qdmull(quarter, quarter)) != [0x2000_0000; 4] {
            return false;
        }
        if i16_lanes(mve::qmovn(i32x4([7, 0x1_0000, -7, 0x7fff])))
            != [7, 0, 0x7fff, 0, -7, 0, 0x7fff, 0]
        {
            return false;
        }
        if lanes(mve::qabs(i32x4([-5, 5, 0, 1]))) != [5, 5, 0, 1]
            || lanes(mve::qneg(i32x4([-5, 5, 0, 1]))) != [5, -5, 0, -1]
            || lanes(mve::qadd(i32x4([1, 2, 3, 4]), i32x4([10, 20, 30, 40]))) != [11, 22, 33, 44]
            || lanes(mve::qsub(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4]))) != [9, 18, 27, 36]
        {
            return false;
        }
        if lanes(mve::qshl(mve::dup(0x4000_0000), 1)) != [0x7fff_ffff; 4] {
            return false;
        }
        let bytes = V128::from_bytes(core::array::from_fn(|index| index as u8));
        let bytes2 = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
        if mve::rev16(bytes).to_bytes() != [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]
            || mve::rev32(bytes).to_bytes()
                != [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]
            || mve::rev64(bytes).to_bytes()
                != [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]
        {
            return false;
        }
        if lanes(mve::shl(mve::dup(8), 1)) != [16; 4] || lanes(mve::shr(mve::dup(-8), 1)) != [-4; 4]
        {
            return false;
        }
        if lanes(mve::sli(mve::dup(0), mve::dup(0x1234_5678), 4)) != [0x2345_6780; 4]
            || lanes(mve::sri(mve::dup(0), mve::dup(0x1234_5678), 4)) != [0x0123_4567; 4]
        {
            return false;
        }
        if i16_lanes(mve::subn(
            i32x4([0x1_0000, 0x2_0000, 0x3_0000, 0x4_0000]),
            i32x4([0; 4]),
        )) != [1, 2, 3, 4, 0, 0, 0, 0]
            || lanes(mve::subw(
                i32x4([10, 20, 30, 40]),
                i16x8([1, 2, 3, 4, 5, 6, 7, 8]),
            )) != [9, 17, 25, 33]
        {
            return false;
        }
        if mve::uzp(bytes, bytes2).to_bytes()
            != [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
            || mve::zip(bytes, bytes2).to_bytes()
                != [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23]
        {
            return false;
        }
        true
    }
}

fn mve_memory() -> bool {
    let source = V128::from_bytes(core::array::from_fn(|index| index as u8));
    unsafe {
        if mve::ldr(source.as_ptr()) != source || mve::ldrb(source.as_ptr()) != source {
            return false;
        }
        let mut word = V128::from_bytes([0; 16]);
        mve::str(word.as_mut_ptr(), source);
        if word != source {
            return false;
        }
        let mut bytes = V128::from_bytes([0; 16]);
        mve::strb(bytes.as_mut_ptr(), source);
        bytes == source
    }
}

fn mve_fp_ops() -> bool {
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    unsafe {
        if f32_lanes(mve_fp::abs(f32x4([-1.0, 2.0, -3.0, 4.0]))) != [1.0, 2.0, 3.0, 4.0]
            || f32_lanes(mve_fp::add(a, b)) != [11.0, 22.0, 33.0, 44.0]
            || f32_lanes(mve_fp::sub(b, a)) != [9.0, 18.0, 27.0, 36.0]
            || f32_lanes(mve_fp::mul(a, b)) != [10.0, 40.0, 90.0, 160.0]
            || f32_lanes(mve_fp::div(a, f32x4([1.0, 2.0, 4.0, 8.0]))) != [1.0, 1.0, 0.75, 0.5]
        {
            return false;
        }
        if f32_lanes(mve_fp::fma(
            f32x4([1.0; 4]),
            f32x4([2.0, 3.0, 4.0, 5.0]),
            f32x4([10.0, 20.0, 30.0, 40.0]),
        )) != [21.0, 61.0, 121.0, 201.0]
            || f32_lanes(mve_fp::fms(
                f32x4([100.0; 4]),
                f32x4([2.0, 3.0, 4.0, 5.0]),
                f32x4([10.0, 20.0, 30.0, 40.0]),
            )) != [80.0, 40.0, -20.0, -100.0]
        {
            return false;
        }
        if f32_lanes(mve_fp::max(a, f32x4([2.0, 1.0, 4.0, 3.0]))) != [2.0, 2.0, 4.0, 4.0]
            || f32_lanes(mve_fp::min(a, f32x4([2.0, 1.0, 4.0, 3.0]))) != [1.0, 1.0, 3.0, 3.0]
            || f32_lanes(mve_fp::neg(a)) != [-1.0, -2.0, -3.0, -4.0]
            || f32_lanes(mve_fp::sqrt(f32x4([4.0, 9.0, 16.0, 25.0]))) != [2.0, 3.0, 4.0, 5.0]
        {
            return false;
        }
        if mve_fp::cmp(a, a).to_bytes() != [0xff; 16] || mve_fp::cmp(a, b).to_bytes() != [0; 16] {
            return false;
        }
        if f32_lanes(mve::fma(f32x4([1.0; 4]), f32x4([2.0; 4]), f32x4([3.0; 4]))) != [7.0; 4]
            || f32_lanes(mve::fms(f32x4([10.0; 4]), f32x4([2.0; 4]), f32x4([3.0; 4]))) != [4.0; 4]
        {
            return false;
        }
        if lanes(mve_fp::cvt(f32x4([1.0, -2.0, 3.0, -4.0]))) != [1, -2, 3, -4]
            || f32_lanes(mve_fp::cvt_f32(i32x4([1, -2, 3, -4]))) != [1.0, -2.0, 3.0, -4.0]
        {
            return false;
        }
        f32_lanes(mve_fp::sqrt(f32x4([1.0, 4.0, 9.0, 16.0]))) == [1.0, 2.0, 3.0, 4.0]
    }
}

fn aes_ops() -> bool {
    let one = V128::from_bytes([1; 16]);
    let zero = V128::from_bytes([0; 16]);
    unsafe {
        aese(one, one).to_bytes() != zero.to_bytes()
            && aesd(one, one).to_bytes() != zero.to_bytes()
            && aesmc(one).to_bytes() != zero.to_bytes()
            && aesimc(one).to_bytes() != zero.to_bytes()
    }
}

fn sha2_ops() -> bool {
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

fn crc_ops() -> bool {
    let mut crc = 0xffff_ffffu32;
    for byte in b"123456789" {
        crc = unsafe { crc32b(crc, *byte) };
    }
    crc == 0x340b_c6d9
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

fn f16x8(values: [u16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f16_lanes(value: V128) -> [u16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn u32x4(values: [u32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn u32_lanes(value: V128) -> [u32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
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

fn ops() -> bool {
    unsafe {
        if add_i8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [3; 16] {
            return false;
        }
        if i16_lanes(add_i16(i16x8([1; 8]), i16x8([2; 8]))) != [3; 8] {
            return false;
        }
        if i16_lanes(sub_i16(i16x8([5; 8]), i16x8([2; 8]))) != [3; 8] {
            return false;
        }
        if lanes(sub_i32(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4]))) != [9, 18, 27, 36] {
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
        {
            return false;
        }
        if cmpeq_i8(V128::from_bytes([1; 16]), V128::from_bytes([1; 16])).to_bytes() != [0xff; 16]
            || cmpeq_i16(i16x8([1; 8]), i16x8([2; 8])).to_bytes() != [0; 16]
            || cmpeq_i32(i32x4([1; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
            || cmpgt_i8(V128::from_bytes([2; 16]), V128::from_bytes([1; 16])).to_bytes()
                != [0xff; 16]
            || cmpgt_i16(i16x8([1; 8]), i16x8([2; 8])).to_bytes() != [0; 16]
            || cmpgt_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
        {
            return false;
        }
        let a = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        let b = V128::from_bytes([2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]);
        let sum = add_i64(a, b).to_bytes();
        let diff = sub_i64(b, a).to_bytes();
        if i64::from_le_bytes(sum[..8].try_into().unwrap()) != 3
            || i64::from_le_bytes(sum[8..].try_into().unwrap()) != 5
            || i64::from_le_bytes(diff[..8].try_into().unwrap()) != 1
            || i64::from_le_bytes(diff[8..].try_into().unwrap()) != 1
        {
            return false;
        }
        lanes(splat_i32(7)) == [7; 4]
    }
}

fn float_ops() -> bool {
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    unsafe {
        f32_lanes(fadd(a, b)) == [11.0, 22.0, 33.0, 44.0]
            && f32_lanes(fsub(b, a)) == [9.0, 18.0, 27.0, 36.0]
            && f32_lanes(fmul(a, b)) == [10.0, 40.0, 90.0, 160.0]
    }
}

fn shuffle_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    unsafe {
        zip1(a, b).to_bytes() == [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23]
            && uzp1(a, b).to_bytes() == [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
            && trn1(a, b).to_bytes() == [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30]
    }
}

fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|index| {
        i32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
    })
}

fn load_store() -> bool {
    let source = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    unsafe {
        let value = v128_load(source.as_ptr());
        if value.to_bytes() != source {
            return false;
        }
        let mut output = [0u8; 16];
        v128_store(output.as_mut_ptr(), value);
        output == source
    }
}

fn integer() -> bool {
    let a = i32x4([1, 2, 3, 4]);
    let b = i32x4([10, 20, 30, 40]);
    unsafe {
        lanes(add(a, b)) == [11, 22, 33, 44]
            && lanes(mul(a, b)) == [10, 40, 90, 160]
            && lanes(kernel_add(&a, &b)) == [21, 42, 63, 84]
            && lanes(kernel_mul(&a, &b)) == [100, 800, 2700, 6400]
    }
}
