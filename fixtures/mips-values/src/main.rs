#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_isa::mips::simd::msa::*;

unsafe extern "C" {
    fn fixture_exit_rust(code: u32) -> !;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { fixture_exit_rust(255) }
}

fn repeat4(value: [u8; 4]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for lane in 0..4 {
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value);
    }
    out
}

fn u32x4(values: [u32; 4]) -> V128 {
    let mut out = [0u8; 16];
    for lane in 0..4 {
        out[lane * 4..lane * 4 + 4].copy_from_slice(&values[lane].to_be_bytes());
    }
    V128::from_bytes(out)
}

fn i32x4(values: [i32; 4]) -> V128 {
    u32x4(values.map(|value| value as u32))
}

fn f32x4(values: [f32; 4]) -> V128 {
    u32x4(values.map(|value| value.to_bits()))
}

fn integer() -> u32 {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    unsafe {
        if add(a, b).to_bytes() != [3; 16] {
            return 10;
        }
        if mul(V128::from_bytes([1; 16]), V128::from_bytes([1; 16])).to_bytes()
            != repeat4([4, 3, 2, 1])
        {
            return 11;
        }
        if add_i8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [3; 16] {
            return 12;
        }
        if sub_i8(V128::from_bytes([5; 16]), V128::from_bytes([2; 16])).to_bytes() != [3; 16] {
            return 13;
        }
        if sub_i16(V128::from_bytes([3; 16]), V128::from_bytes([1; 16])).to_bytes() != [2; 16] {
            return 14;
        }
        let a = V128::from_bytes([0b1010_1010; 16]);
        let b = V128::from_bytes([0b1100_1100; 16]);
        if and(a, b).to_bytes() != [0b1000_1000; 16]
            || or(a, b).to_bytes() != [0b1110_1110; 16]
            || xor(a, b).to_bytes() != [0b0110_0110; 16]
        {
            return 15;
        }
        let one = V128::from_bytes(repeat4([0x3f, 0x80, 0, 0]));
        let two = V128::from_bytes(repeat4([0x40, 0, 0, 0]));
        let three = V128::from_bytes(repeat4([0x40, 0x40, 0, 0]));
        if fadd(one, two).to_bytes() != three.to_bytes() {
            return 16;
        }
        if fsub(three, two).to_bytes() != one.to_bytes() {
            return 17;
        }
        if fmul(one, two).to_bytes() != two.to_bytes() {
            return 18;
        }
        if fdiv(two, one).to_bytes() != two.to_bytes() {
            return 19;
        }
        let one64 = V128::from_bytes([0x3f, 0xf0, 0, 0, 0, 0, 0, 0, 0x3f, 0xf0, 0, 0, 0, 0, 0, 0]);
        let two64 = V128::from_bytes([0x40, 0, 0, 0, 0, 0, 0, 0, 0x40, 0, 0, 0, 0, 0, 0, 0]);
        let three64 =
            V128::from_bytes([0x40, 0x08, 0, 0, 0, 0, 0, 0, 0x40, 0x08, 0, 0, 0, 0, 0, 0]);
        if fadd_f64(one64, two64).to_bytes() != three64.to_bytes() {
            return 20;
        }
        if fsub_f64(three64, two64).to_bytes() != one64.to_bytes() {
            return 21;
        }
        if fmul_f64(one64, two64).to_bytes() != two64.to_bytes() {
            return 22;
        }
        if fdiv_f64(two64, one64).to_bytes() != two64.to_bytes() {
            return 23;
        }
        if cmpeq_i32(
            V128::from_bytes(repeat4([0, 0, 0, 1])),
            V128::from_bytes(repeat4([0, 0, 0, 1])),
        )
        .to_bytes()
            != [0xff; 16]
        {
            return 24;
        }
        if cmpgt_i32(
            V128::from_bytes(repeat4([0, 0, 0, 2])),
            V128::from_bytes(repeat4([0, 0, 0, 1])),
        )
        .to_bytes()
            != [0xff; 16]
        {
            return 25;
        }
    }
    0
}

fn arithmetic() -> u32 {
    unsafe {
        if div_s(i32x4([7, -7, 7, -7]), i32x4([2, 2, -2, -2])) != i32x4([3, -3, -3, 3]) {
            return 30;
        }
        if div_u(u32x4([7, 8, 9, 10]), u32x4([2, 3, 4, 5])) != u32x4([3, 2, 2, 2]) {
            return 31;
        }
        if mod_s(i32x4([7, -8, 9, -10]), i32x4([2, 3, -4, 5])) != i32x4([1, -2, 1, 0]) {
            return 32;
        }
        if mod_u(u32x4([7, 8, 9, 10]), u32x4([2, 3, 4, 5])) != u32x4([1, 2, 1, 0]) {
            return 33;
        }
        if add_a(i32x4([-3, 5, -7, 9]), i32x4([4, -6, 8, -10])) != i32x4([7, 11, 15, 19]) {
            return 34;
        }
        if adds_a(i32x4([i32::MAX, 1, -5, 100]), i32x4([1, 1, 6, -100]))
            != i32x4([i32::MAX, 2, 11, 200])
        {
            return 35;
        }
        if adds_s(i32x4([i32::MAX, i32::MIN, 1, -1]), i32x4([1, -1, 2, -2]))
            != i32x4([i32::MAX, i32::MIN, 3, -3])
        {
            return 36;
        }
        if adds_u(u32x4([u32::MAX, 0, 5, 10]), u32x4([1, 1, 5, 10])) != u32x4([u32::MAX, 1, 10, 20])
        {
            return 37;
        }
        if subs_s(i32x4([i32::MAX, i32::MIN, 5, -5]), i32x4([-1, 1, 3, -3]))
            != i32x4([i32::MAX, i32::MIN, 2, -2])
        {
            return 38;
        }
        if subs_u(u32x4([u32::MAX, 0, 5, 10]), u32x4([1, 1, 10, 20]))
            != u32x4([u32::MAX - 1, 0, 0, 0])
        {
            return 39;
        }
        if max_s(i32x4([-1, 5, -7, 9]), i32x4([3, -5, 7, -9])) != i32x4([3, 5, 7, 9]) {
            return 40;
        }
        if max_u(u32x4([1, u32::MAX, 7, 9]), u32x4([3, 5, u32::MAX, 2]))
            != u32x4([3, u32::MAX, u32::MAX, 9])
        {
            return 41;
        }
        if min_s(i32x4([-1, 5, -7, 9]), i32x4([3, -5, 7, -9])) != i32x4([-1, -5, -7, -9]) {
            return 42;
        }
        if min_u(u32x4([1, u32::MAX, 7, 9]), u32x4([3, 5, u32::MAX, 2])) != u32x4([1, 5, 7, 2]) {
            return 43;
        }
        if nor(
            u32x4([0x0000_000f, 0x0000_00f0, 0x0000_0f00, 0x0000_f000]),
            u32x4([0x0000_00f0, 0x0000_000f, 0x0000_f000, 0x0000_0f00]),
        ) != u32x4([0xffff_ff00, 0xffff_ff00, 0xffff_00ff, 0xffff_00ff])
        {
            return 44;
        }
        if seq(u32x4([1, 2, 3, 4]), u32x4([1, 0, 4, 5])) != u32x4([u32::MAX, 0, 0, 0]) {
            return 45;
        }
        if sle_s(i32x4([-1, 2, -3, 4]), i32x4([0, 1, -4, -4])) != u32x4([u32::MAX, 0, 0, 0]) {
            return 46;
        }
        if sle_u(u32x4([0, u32::MAX, 3, 4]), u32x4([1, 1, 2, 4]))
            != u32x4([u32::MAX, 0, 0, u32::MAX])
        {
            return 47;
        }
        if slt_s(i32x4([-1, 2, -3, 4]), i32x4([0, 1, -4, -4])) != u32x4([u32::MAX, 0, 0, 0]) {
            return 48;
        }
        if slt_u(u32x4([0, u32::MAX, 3, 4]), u32x4([1, 1, 2, 4])) != u32x4([u32::MAX, 0, 0, 0]) {
            return 49;
        }
    }
    0
}

fn shift_and_bits() -> u32 {
    unsafe {
        if sll(i32x4([1, 2, 3, 4]), i32x4([1, 2, 3, 4])) != i32x4([2, 8, 24, 64]) {
            return 50;
        }
        if srl(u32x4([0x8000_0000, u32::MAX, 8, 16]), i32x4([1, 4, 2, 3]))
            != u32x4([0x4000_0000, 0x0fff_ffff, 2, 2])
        {
            return 51;
        }
        if sra(i32x4([-8, -16, 8, 16]), i32x4([1, 2, 2, 3])) != i32x4([-4, -4, 2, 2]) {
            return 52;
        }
        if slli::<1>(i32x4([1, 2, 4, 8])) != i32x4([2, 4, 8, 16]) {
            return 53;
        }
        if srli::<1>(u32x4([0x8000_0000, 2, 4, 8])) != u32x4([0x4000_0000, 1, 2, 4]) {
            return 54;
        }
        if srai::<1>(i32x4([-8, -2, 2, 8])) != i32x4([-4, -1, 1, 4]) {
            return 55;
        }
        if bclr(u32x4([u32::MAX; 4]), u32x4([0, 1, 31, 5]))
            != u32x4([0xffff_fffe, 0xffff_fffd, 0x7fff_ffff, 0xffff_ffdf])
        {
            return 56;
        }
        if bset(u32x4([0; 4]), u32x4([0, 1, 31, 5])) != u32x4([1, 2, 0x8000_0000, 0x20]) {
            return 57;
        }
        if bneg(u32x4([u32::MAX; 4]), u32x4([0, 1, 31, 5]))
            != u32x4([0xffff_fffe, 0xffff_fffd, 0x7fff_ffff, 0xffff_ffdf])
        {
            return 58;
        }
        if binsl(u32x4([0; 4]), u32x4([u32::MAX; 4]), u32x4([0, 7, 15, 31]))
            != u32x4([0x8000_0000, 0xff00_0000, 0xffff_0000, u32::MAX])
        {
            return 59;
        }
        if binsr(u32x4([0; 4]), u32x4([u32::MAX; 4]), u32x4([0, 7, 15, 31]))
            != u32x4([1, 0xff, 0xffff, u32::MAX])
        {
            return 60;
        }
        if sat_s::<4>(i32x4([10, -10, 7, -8])) != i32x4([7, -8, 7, -8]) {
            return 61;
        }
        if sat_u::<4>(u32x4([16, 15, 0, 255])) != u32x4([15, 15, 0, 15]) {
            return 62;
        }
        if sld(i32x4([10, 11, 12, 13]), i32x4([0, 1, 2, 3]), 1) != i32x4([1, 2, 3, 10]) {
            return 63;
        }
        if vshf(
            i32x4([10, 11, 12, 13]),
            i32x4([20, 21, 22, 23]),
            u32x4([0, 4, 5, 0x40]),
        ) != i32x4([20, 10, 11, 0])
        {
            return 64;
        }
        if shf::<0x1b>(i32x4([1, 2, 3, 4])) != i32x4([4, 3, 2, 1]) {
            return 65;
        }
        if pcnt(u32x4([0, 0xff, u32::MAX, 0x8000_0000])) != u32x4([0, 8, 32, 1]) {
            return 66;
        }
        if nlzc(u32x4([0, 0xff, u32::MAX, 0x8000_0000])) != u32x4([32, 24, 0, 0]) {
            return 67;
        }
        if nloc(u32x4([0, 0xff, u32::MAX, 0x8000_0000])) != u32x4([0, 0, 32, 1]) {
            return 68;
        }
    }
    0
}

fn float_ops() -> u32 {
    unsafe {
        if fmax(f32x4([1.0, 2.0, 3.0, 4.0]), f32x4([4.0, 3.0, 2.0, 1.0]))
            != f32x4([4.0, 3.0, 3.0, 4.0])
        {
            return 69;
        }
        if fmin(f32x4([1.0, 2.0, 3.0, 4.0]), f32x4([4.0, 3.0, 2.0, 1.0]))
            != f32x4([1.0, 2.0, 2.0, 1.0])
        {
            return 70;
        }
        if fexp2(f32x4([1.0, 2.0, 4.0, 0.5]), i32x4([1, 2, 3, -1])) != f32x4([2.0, 8.0, 32.0, 0.25])
        {
            return 71;
        }
        if flog2(f32x4([1.0, 2.0, 4.0, 8.0])) != f32x4([0.0, 1.0, 2.0, 3.0]) {
            return 72;
        }
        if frcp(f32x4([1.0, 2.0, 4.0, 0.5])) != f32x4([1.0, 0.5, 0.25, 2.0]) {
            return 73;
        }
        if frsqrt(f32x4([1.0, 4.0, 16.0, 0.25])) != f32x4([1.0, 0.5, 0.25, 2.0]) {
            return 74;
        }
        if fsqrt(f32x4([1.0, 4.0, 9.0, 16.0])) != f32x4([1.0, 2.0, 3.0, 4.0]) {
            return 75;
        }
        if ftint_s(f32x4([1.0, -2.0, 3.0, -4.0])) != i32x4([1, -2, 3, -4]) {
            return 76;
        }
        if ftint_u(f32x4([1.0, 2.0, 3.0, 4.0])) != u32x4([1, 2, 3, 4]) {
            return 77;
        }
        if ffint_s(i32x4([-1, -2, 3, 4])) != f32x4([-1.0, -2.0, 3.0, 4.0]) {
            return 78;
        }
        if ffint_u(u32x4([1, 2, 3, 4])) != f32x4([1.0, 2.0, 3.0, 4.0]) {
            return 79;
        }
        if ftint::<true>(f32x4([1.0, -2.0, 3.0, -4.0])) != i32x4([1, -2, 3, -4]) {
            return 80;
        }
        if ftint::<false>(f32x4([1.0, 2.0, 3.0, 4.0])) != u32x4([1, 2, 3, 4]) {
            return 81;
        }
        if ffint::<true>(i32x4([-1, -2, 3, 4])) != f32x4([-1.0, -2.0, 3.0, 4.0]) {
            return 82;
        }
        if ffint::<false>(u32x4([1, 2, 3, 4])) != f32x4([1.0, 2.0, 3.0, 4.0]) {
            return 83;
        }
        if fclass(f32x4([0.0, -0.0, 1.0, -1.0])) != u32x4([1 << 9, 1 << 5, 1 << 7, 1 << 3]) {
            return 84;
        }
        if fclass(u32x4([0x7f80_0000, 0xff80_0000, 0x7fc0_0000, 0x7f80_0001]))
            != u32x4([1 << 6, 1 << 2, 1 << 1, 1 << 0])
        {
            return 85;
        }
        let a = u32x4([0x3f80_0000, 0x7fc0_0000, 0xc000_0000, 0x4040_0000]);
        let b = u32x4([0x3f80_0000, 0x4000_0000, 0xbf80_0000, 0x7fc0_0000]);
        if fcmp(a, b, Fcmp::Eq) != u32x4([u32::MAX, 0, 0, 0]) {
            return 86;
        }
        if fcmp(a, b, Fcmp::Lt) != u32x4([0, 0, u32::MAX, 0]) {
            return 87;
        }
        if fcmp(
            u32x4([0x7fc0_0000, 0x7fc0_0000, 0xbf80_0000, 0x4040_0000]),
            u32x4([0x3f80_0000, 0x4000_0000, 0xbf80_0000, 0x4080_0000]),
            Fcmp::Un,
        ) != u32x4([u32::MAX, u32::MAX, 0, 0])
        {
            return 88;
        }
        if fcmp(
            f32x4([1.0, 2.0, 3.0, 4.0]),
            f32x4([1.0, 2.0, 5.0, 6.0]),
            Fcmp::Seq,
        ) != u32x4([u32::MAX, u32::MAX, 0, 0])
        {
            return 89;
        }
    }
    0
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_add(a: &V128, b: &V128, c: &V128) -> V128 {
    unsafe { add(add(*a, *b), *c) }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_mul(a: &V128, b: &V128, c: &V128) -> V128 {
    unsafe { mul(mul(*a, *b), *c) }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    let c = V128::from_bytes([3; 16]);
    core::hint::black_box(unsafe { kernel_add(&a, &b, &c) });
    core::hint::black_box(unsafe { kernel_mul(&a, &b, &c) });

    if !load_store() {
        return 1;
    }
    let code = integer();
    if code != 0 {
        return code;
    }
    let code = arithmetic();
    if code != 0 {
        return code;
    }
    let code = shift_and_bits();
    if code != 0 {
        return code;
    }
    let code = float_ops();
    if code != 0 {
        return code;
    }
    0
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
