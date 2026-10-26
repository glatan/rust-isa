#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::arm::crypto::aes::{aesd, aese, aesimc, aesmc};
use rust_isa::arm::crypto::crc::crc32b;
use rust_isa::arm::crypto::sha2::{sha256h, sha256h2, sha256su0, sha256su1};
use rust_isa::arm::simd::dotprod::{sdot, udot};
use rust_isa::arm::simd::fp16;
use rust_isa::arm::simd::i8mm::{smmla, vummla, vusmmla};
use rust_isa::arm::simd::mve;
use rust_isa::arm::simd::mve_fp;
use rust_isa::arm::simd::neon::{
    V64, vext, vld1, vld2, vld3, vld4, vmov, vpadd, vpmax, vpmin, vrev16, vrev32, vrev64, vshr,
    vswp, vtbl, vtbx, vtrn, vuzp, vzip,
};
use rust_isa::arm::simd::neon::{
    V128, add, add_i8, add_i16, add_i64, and, cmpeq_i8, cmpeq_i16, cmpeq_i32, cmpgt_i8, cmpgt_i16,
    cmpgt_i32, eor, fadd, fmul, fsub, mul, mul_i16, mul_i32, orr, splat_i32, sub_i8, sub_i16,
    sub_i32, sub_i64, v128_load, v128_store, vaba, vabd, vabs, vbic, vbsl, vceq, vcge, vcgt, vcle,
    vcls, vclt, vclz, vcnt, vcvt, vfma, vfms, vfnma, vfnms, vhadd, vhsub, vmax, vmin, vmla, vmls,
    vmvn, vneg, vorn, vqabs, vqadd, vqdmulh, vqneg, vqrdmulh, vqshl, vqsub, vrecpe, vrecps, vrshl,
    vrsqrte, vrsqrts, vshl, vtst,
};
use rust_isa::arm::simd::neon::{
    vabal, vabdl, vaddhn, vaddl, vaddw, vmlal, vmlsl, vmovn, vmull, vpadal, vqdmlal, vqdmlsl,
    vqdmull, vqmovn, vqmovun, vqrshl, vqrshrn, vqrshrun, vqshlu, vqshrn, vqshrun, vraddhn, vrshr,
    vrshrn, vrsra, vrsubhn, vshll, vshrn, vsli, vsra, vsri, vsubhn, vsubl, vsubw,
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
    if !i8mm_ops() {
        return 11;
    }
    if !arith_ops() {
        return 12;
    }
    if !widen_ops() {
        return 13;
    }
    if !memory_ops() {
        return 14;
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

fn i8mm_ops() -> bool {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([1; 16]);
    let zero = V128::from_bytes([0; 16]);
    unsafe {
        let signed = smmla(zero, a, b);
        let unsigned = vummla(zero, a, b);
        let mixed = vusmmla(zero, a, b);
        signed == unsigned && unsigned == mixed && signed != zero
    }
}

fn arith_ops() -> bool {
    unsafe {
        if vaba(
            V128::from_bytes([0; 16]),
            V128::from_bytes([10; 16]),
            V128::from_bytes([3; 16]),
        )
        .to_bytes()
            != [7; 16]
            || vabd(V128::from_bytes([10; 16]), V128::from_bytes([3; 16])).to_bytes() != [7; 16]
            || lanes(vabs(i32x4([-1, 2, -3, 4]))) != [1, 2, 3, 4]
        {
            return false;
        }
        let ones = V128::from_bytes([1; 16]);
        let twos = V128::from_bytes([2; 16]);
        if vceq(ones, ones).to_bytes() != [0xff; 16]
            || vceq(ones, twos).to_bytes() != [0; 16]
            || vcge(twos, ones).to_bytes() != [0xff; 16]
            || vcge(ones, ones).to_bytes() != [0xff; 16]
            || vcge(ones, twos).to_bytes() != [0; 16]
            || vcgt(twos, ones).to_bytes() != [0xff; 16]
            || vcgt(ones, ones).to_bytes() != [0; 16]
            || vcle(ones, twos).to_bytes() != [0xff; 16]
            || vcle(twos, ones).to_bytes() != [0; 16]
            || vclt(ones, twos).to_bytes() != [0xff; 16]
            || vclt(ones, ones).to_bytes() != [0; 16]
        {
            return false;
        }
        let cls_input = V128::from_bytes([0, 1, 0x7f, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        if vcls(cls_input).to_bytes() != [7, 6, 0, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7] {
            return false;
        }
        let clz_input = V128::from_bytes([0, 1, 0x80, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        if vclz(clz_input).to_bytes() != [8, 7, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8] {
            return false;
        }
        let cnt_input = V128::from_bytes([0, 1, 0x0f, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        if vcnt(cnt_input).to_bytes() != [0, 1, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
            return false;
        }
        if lanes(vcvt(f32x4([1.9, -1.9, 2.5, -2.5]))) != [1, -1, 2, -2] {
            return false;
        }
        if lanes(vhadd(i32x4([2, 4, 6, 8]), i32x4([6, 8, 10, 12]))) != [4, 6, 8, 10]
            || lanes(vhsub(i32x4([6, 8, 10, 12]), i32x4([2, 4, 6, 8]))) != [2, 2, 2, 2]
            || lanes(vmax(i32x4([1, 5, 2, 7]), i32x4([3, 4, 6, 0]))) != [3, 5, 6, 7]
            || lanes(vmin(i32x4([1, 5, 2, 7]), i32x4([3, 4, 6, 0]))) != [1, 4, 2, 0]
        {
            return false;
        }
        let acc = i32x4([1, 2, 3, 4]);
        let x = i32x4([2, 3, 4, 5]);
        let y = i32x4([6, 7, 8, 9]);
        if lanes(vmla(acc, x, y)) != [13, 23, 35, 49]
            || lanes(vmls(acc, x, y)) != [-11, -19, -29, -41]
            || lanes(vneg(i32x4([1, -2, 3, -4]))) != [-1, 2, -3, 4]
        {
            return false;
        }
        if lanes(vqabs(i32x4([i32::MIN, -5, 0, 5]))) != [i32::MAX, 5, 0, 5]
            || lanes(vqadd(i32x4([i32::MAX, 1, -1, 10]), i32x4([1, 1, -1, 20])))
                != [i32::MAX, 2, -2, 30]
            || lanes(vqneg(i32x4([i32::MIN, -5, 0, 5]))) != [i32::MAX, 5, 0, -5]
            || lanes(vqsub(i32x4([i32::MIN, 1, -1, 10]), i32x4([1, 1, -1, 20])))
                != [i32::MIN, 0, 0, -10]
        {
            return false;
        }
        let half = i16x8([0x4000; 8]);
        if i16_lanes(vqdmulh(half, half)) != [0x2000; 8]
            || i16_lanes(vqrdmulh(half, half)) != [0x2000; 8]
        {
            return false;
        }
        if lanes(vqshl(i32x4([0x4000_0000; 4]), i32x4([1; 4]))) != [i32::MAX; 4] {
            return false;
        }
        if lanes(vrshl(i32x4([8; 4]), i32x4([1, -1, 1, -1]))) != [16, 4, 16, 4]
            || lanes(vshl(i32x4([1, 2, 3, 4]), i32x4([1, 1, -1, -1]))) != [2, 4, 1, 2]
        {
            return false;
        }
        let a = V128::from_bytes([0xff; 16]);
        let b = V128::from_bytes([0x0f; 16]);
        if vbic(a, b).to_bytes() != [0xf0; 16]
            || vbsl(V128::from_bytes([0xaa; 16]), a, V128::from_bytes([0; 16])).to_bytes()
                != [0xaa; 16]
            || vmvn(V128::from_bytes([0; 16])).to_bytes() != [0xff; 16]
            || vorn(V128::from_bytes([0x0f; 16]), V128::from_bytes([0; 16])).to_bytes()
                != [0xff; 16]
            || vtst(V128::from_bytes([0x0f; 16]), V128::from_bytes([0x01; 16])).to_bytes()
                != [0xff; 16]
            || vtst(V128::from_bytes([0x0f; 16]), V128::from_bytes([0x80; 16])).to_bytes()
                != [0; 16]
        {
            return false;
        }
        if f32_lanes(vfma(f32x4([1.0; 4]), f32x4([2.0; 4]), f32x4([3.0; 4]))) != [7.0; 4]
            || f32_lanes(vfms(f32x4([10.0; 4]), f32x4([2.0; 4]), f32x4([3.0; 4]))) != [4.0; 4]
            || f32_lanes(vfnma(f32x4([10.0; 4]), f32x4([2.0; 4]), f32x4([3.0; 4]))) != [-4.0; 4]
            || f32_lanes(vfnms(f32x4([10.0; 4]), f32x4([2.0; 4]), f32x4([3.0; 4]))) != [-16.0; 4]
        {
            return false;
        }
        (f32_lanes(vrecpe(f32x4([1.0; 4])))[0] - 1.0).abs() < 0.01
            && f32_lanes(vrecps(f32x4([1.0; 4]), f32x4([1.0; 4]))) == [1.0; 4]
            && (f32_lanes(vrsqrte(f32x4([1.0; 4])))[0] - 1.0).abs() < 0.01
            && f32_lanes(vrsqrts(f32x4([1.0; 4]), f32x4([1.0; 4]))) == [1.0; 4]
    }
}

fn widen_ops() -> bool {
    let tens = V128::from_bytes([10; 16]);
    let threes = V128::from_bytes([3; 16]);
    unsafe {
        if i16_lanes(vabal(i16x8([1; 8]), tens, threes)) != [8; 8]
            || i16_lanes(vabdl(tens, threes)) != [7; 8]
            || low_bytes(vaddhn(
                i16x8([
                    0x0102, 0x0304, 0x0506, 0x0708, 0x090a, 0x0b0c, 0x0d0e, 0x0f10,
                ]),
                i16x8([0; 8]),
            )) != [1, 3, 5, 7, 9, 11, 13, 15]
        {
            return false;
        }
        let low_ascending = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]);
        let low_tens = V128::from_bytes([10, 20, 30, 40, 50, 60, 70, 80, 0, 0, 0, 0, 0, 0, 0, 0]);
        if i16_lanes(vaddl(low_ascending, low_tens)) != [11, 22, 33, 44, 55, 66, 77, 88]
            || i16_lanes(vaddw(i16x8([1, 2, 3, 4, 5, 6, 7, 8]), low_tens))
                != [11, 22, 33, 44, 55, 66, 77, 88]
            || i16_lanes(vsubl(low_tens, low_ascending)) != [9, 18, 27, 36, 45, 54, 63, 72]
            || i16_lanes(vsubw(
                i16x8([10, 20, 30, 40, 50, 60, 70, 80]),
                low_ascending,
            )) != [9, 18, 27, 36, 45, 54, 63, 72]
        {
            return false;
        }
        let acc = i32x4([1, 2, 3, 4]);
        let x16 = i16x8([2, 3, 4, 5, 0, 0, 0, 0]);
        let y16 = i16x8([6, 7, 8, 9, 0, 0, 0, 0]);
        if lanes(vmlal(acc, x16, y16)) != [13, 23, 35, 49]
            || lanes(vmlsl(acc, x16, y16)) != [-11, -19, -29, -41]
            || lanes(vmull(x16, y16)) != [12, 21, 32, 45]
            || lanes(vpadal(acc, i16x8([1, 2, 3, 4, 5, 6, 7, 8]))) != [4, 9, 14, 19]
        {
            return false;
        }
        let quarter = i16x8([0x4000; 8]);
        if lanes(vqdmlal(acc, quarter, quarter))
            != [0x2000_0001, 0x2000_0002, 0x2000_0003, 0x2000_0004]
            || lanes(vqdmlsl(acc, quarter, quarter))
                != [-0x1fff_ffff, -0x1fff_fffe, -0x1fff_fffd, -0x1fff_fffc]
            || lanes(vqdmull(quarter, quarter)) != [0x2000_0000; 4]
        {
            return false;
        }
        if low_bytes(vmovn(i16x8([
            0x0102, 0x0304, 0x0506, 0x0708, 0x090a, 0x0b0c, 0x0d0e, 0x0f10,
        ]))) != [2, 4, 6, 8, 10, 12, 14, 16]
        {
            return false;
        }
        let movn_input = i16x8([7, 0x0100, -7, 0x7fff, -0x8000, 0, 1, -1]);
        if low_i8(vqmovn(movn_input)) != [7, 127, -7, 127, -128, 0, 1, -1] {
            return false;
        }
        let movun = vqmovun(movn_input);
        if movun.to_bytes()[..8] != [7, 255, 0, 255, 0, 0, 1, 0] || movun.to_bytes()[8..] != [0; 8]
        {
            return false;
        }
        if i16_lanes(vqrshl(i16x8([8; 8]), i16x8([1, -1, 1, -1, 1, -1, 1, -1])))
            != [16, 4, 16, 4, 16, 4, 16, 4]
        {
            return false;
        }
        let shift_input = i16x8([
            0x0011, 0x0021, 0x0031, 0x0041, 0x0051, 0x0061, 0x0071, 0x0081,
        ]);
        if low_bytes(vqrshrn::<4>(shift_input)) != [1, 2, 3, 4, 5, 6, 7, 8]
            || low_bytes(vqrshrun::<4>(shift_input)) != [1, 2, 3, 4, 5, 6, 7, 8]
            || low_bytes(vqshrn::<4>(shift_input)) != [1, 2, 3, 4, 5, 6, 7, 8]
            || low_bytes(vqshrun::<4>(shift_input)) != [1, 2, 3, 4, 5, 6, 7, 8]
            || low_bytes(vrshrn::<4>(shift_input)) != [1, 2, 3, 4, 5, 6, 7, 8]
            || low_bytes(vshrn::<4>(shift_input)) != [1, 2, 3, 4, 5, 6, 7, 8]
        {
            return false;
        }
        if u16_lanes(vqshlu::<4>(i16x8([-1, 1, 2, 3, 0x7fff, -0x8000, 16, 17])))
            != [0, 16, 32, 48, 65535, 0, 256, 272]
        {
            return false;
        }
        if low_bytes(vraddhn(
            i16x8([
                0x0102, 0x0304, 0x0506, 0x0708, 0x090a, 0x0b0c, 0x0d0e, 0x0f10,
            ]),
            i16x8([0; 8]),
        )) != [1, 3, 5, 7, 9, 11, 13, 15]
            || low_bytes(vrsubhn(
                i16x8([
                    0x0300, 0x0500, 0x0700, 0x0900, 0x0b00, 0x0d00, 0x0f00, 0x1100,
                ]),
                i16x8([0x0100; 8]),
            )) != [2, 4, 6, 8, 10, 12, 14, 16]
            || low_bytes(vsubhn(
                i16x8([
                    0x0300, 0x0500, 0x0700, 0x0900, 0x0b00, 0x0d00, 0x0f00, 0x1100,
                ]),
                i16x8([0x0100; 8]),
            )) != [2, 4, 6, 8, 10, 12, 14, 16]
        {
            return false;
        }
        if lanes(vshll::<4>(i16x8([1, 2, 3, 4, 0, 0, 0, 0]))) != [16, 32, 48, 64] {
            return false;
        }
        if i16_lanes(vrshr::<4>(i16x8([
            0x0010, 0x0020, 0x0030, 0x0040, 0x0050, 0x0060, 0x0070, 0x0080,
        ]))) != [1, 2, 3, 4, 5, 6, 7, 8]
            || i16_lanes(vsra::<4>(
                i16x8([1; 8]),
                i16x8([
                    0x0010, 0x0020, 0x0030, 0x0040, 0x0050, 0x0060, 0x0070, 0x0080,
                ]),
            )) != [2, 3, 4, 5, 6, 7, 8, 9]
            || i16_lanes(vrsra::<4>(
                i16x8([1; 8]),
                i16x8([
                    0x0010, 0x0020, 0x0030, 0x0040, 0x0050, 0x0060, 0x0070, 0x0080,
                ]),
            )) != [2, 3, 4, 5, 6, 7, 8, 9]
        {
            return false;
        }
        if u16_lanes(vsli::<4>(i16x8([-1; 8]), i16x8([0x1234; 8]))) != [0x234f; 8]
            || u16_lanes(vsri::<4>(i16x8([-1; 8]), i16x8([0x1234; 8]))) != [0xf123; 8]
        {
            return false;
        }
        true
    }
}

fn memory_ops() -> bool {
    let ascending = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let plus_16 = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    let mut load2 = [0u8; 32];
    for (index, byte) in load2.iter_mut().enumerate() {
        *byte = index as u8;
    }
    let mut load3 = [0u8; 48];
    for (index, byte) in load3.iter_mut().enumerate() {
        *byte = index as u8;
    }
    let mut load4 = [0u8; 64];
    for (index, byte) in load4.iter_mut().enumerate() {
        *byte = index as u8;
    }
    unsafe {
        if vld1(load2.as_ptr()) != V128::from_bytes(core::array::from_fn(|i| i as u8)) {
            return false;
        }
        let (ld2a, ld2b) = vld2(load2.as_ptr());
        if ld2a.to_bytes() != core::array::from_fn(|i| (i * 2) as u8)
            || ld2b.to_bytes() != core::array::from_fn(|i| (i * 2 + 1) as u8)
        {
            return false;
        }
        let (ld3a, ld3b, ld3c) = vld3(load3.as_ptr());
        if ld3a.to_bytes() != core::array::from_fn(|i| (i * 3) as u8)
            || ld3b.to_bytes() != core::array::from_fn(|i| (i * 3 + 1) as u8)
            || ld3c.to_bytes() != core::array::from_fn(|i| (i * 3 + 2) as u8)
        {
            return false;
        }
        let (ld4a, ld4b, ld4c, ld4d) = vld4(load4.as_ptr());
        if ld4a.to_bytes() != core::array::from_fn(|i| (i * 4) as u8)
            || ld4b.to_bytes() != core::array::from_fn(|i| (i * 4 + 1) as u8)
            || ld4c.to_bytes() != core::array::from_fn(|i| (i * 4 + 2) as u8)
            || ld4d.to_bytes() != core::array::from_fn(|i| (i * 4 + 3) as u8)
        {
            return false;
        }
        if vmov(ascending) != ascending {
            return false;
        }
        if lanes(vpadd(i32x4([1, 2, 3, 4]), i32x4([10, 20, 30, 40]))) != [3, 7, 30, 70]
            || lanes(vpmax(i32x4([1, 5, 2, 7]), i32x4([3, 4, 6, 0]))) != [5, 7, 4, 6]
            || lanes(vpmin(i32x4([1, 5, 2, 7]), i32x4([3, 4, 6, 0]))) != [1, 2, 3, 0]
        {
            return false;
        }
        if vrev16(ascending).to_bytes() != [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]
            || vrev32(ascending).to_bytes()
                != [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]
            || vrev64(ascending).to_bytes()
                != [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]
        {
            return false;
        }
        if vswp(ascending, plus_16) != (plus_16, ascending) {
            return false;
        }
        let (trn_a, trn_b) = vtrn(ascending, plus_16);
        if trn_a.to_bytes() != [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30]
            || trn_b.to_bytes() != [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31]
        {
            return false;
        }
        let (uzp_a, uzp_b) = vuzp(ascending, plus_16);
        if uzp_a.to_bytes() != [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
            || uzp_b.to_bytes() != [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]
        {
            return false;
        }
        let (zip_a, zip_b) = vzip(ascending, plus_16);
        if zip_a.to_bytes() != [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23]
            || zip_b.to_bytes() != [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31]
        {
            return false;
        }
        let indices = V64::from_bytes([0, 1, 15, 16, 255, 3, 4, 5]);
        if vtbl(ascending, indices).to_bytes() != [0, 1, 15, 0, 0, 3, 4, 5] {
            return false;
        }
        let mut source = [0u8; 8];
        for (index, byte) in source.iter_mut().enumerate() {
            *byte = index as u8 + 100;
        }
        if vtbx(ascending, indices, V64::from_bytes(source)).to_bytes()
            != [0, 1, 15, 103, 104, 3, 4, 5]
        {
            return false;
        }
        if vext::<4>(ascending, plus_16).to_bytes()
            != [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
            || vext::<0>(ascending, plus_16) != ascending
        {
            return false;
        }
        if i16_lanes(vshr::<4>(i16x8([
            0x0010, 0x0020, 0x0030, 0x0040, 0x0050, 0x0060, 0x0070, 0x0080,
        ]))) != [1, 2, 3, 4, 5, 6, 7, 8]
            || i16_lanes(vshr::<4>(i16x8([-16; 8]))) != [-1; 8]
        {
            return false;
        }
        true
    }
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

fn u16_lanes(value: V128) -> [u16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn low_bytes(value: V128) -> [u8; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|index| bytes[index])
}

fn low_i8(value: V128) -> [i8; 8] {
    let bytes = low_bytes(value);
    core::array::from_fn(|index| bytes[index] as i8)
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
        vzip(a, b).0.to_bytes() == [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23]
            && vzip(a, b).1.to_bytes()
                == [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31]
            && vuzp(a, b).0.to_bytes()
                == [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
            && vuzp(a, b).1.to_bytes()
                == [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]
            && vtrn(a, b).0.to_bytes()
                == [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30]
            && vtrn(a, b).1.to_bytes()
                == [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31]
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
