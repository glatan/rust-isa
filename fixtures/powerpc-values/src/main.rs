#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::powerpc::crypto::power8_crypto::{cipher, cipherlast, ncipher};
use rust_isa::powerpc::simd::altivec::{
    V128, add, add_i8, add_i16, add_i64, and, cmpeq_i32, cmpgt_i32, fadd, fdiv, fmul, fsub, mul,
    mul_i16, mul_i32, or, permute, splat_i32, sub_i8, sub_i16, sub_i32, sub_i64, v128_load,
    v128_store, xor,
};
use rust_isa::powerpc::simd::power8_vector::{add_u64, mul_even_u16, mul_u32, sub_u64};
use rust_isa::powerpc::simd::power9_vector::{abs_i32 as p9_abs_i32, neg_i32 as p9_neg_i32};
use rust_isa::powerpc::simd::power10_vector::{
    cntlz_i32 as p10_cntlz_i32, popcnt_i32 as p10_popcnt_i32,
};
use rust_isa::powerpc::simd::vsx::{
    abs_f64, add_f32 as vsx_add_f32, add_f64, div_f64, mul_f64, sqrt_f64, sub_f64,
};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "li 0, 1",
            "sc",
            in("r0") 1usize,
            in("r3") code as usize,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "li 0, 1",
            "sc",
            in("r0") 1usize,
            in("r3") 255usize,
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

fn run() -> u32 {
    if !load_store() {
        return 1;
    }
    if !integer() {
        return 2;
    }
    if !ops_arithmetic() {
        return 31;
    }
    if !(unsafe {
        and(
            V128::from_bytes([0b1010_1010; 16]),
            V128::from_bytes([0b1100_1100; 16]),
        )
        .to_bytes()
            == [0b1000_1000; 16]
    }) {
        return 321;
    }
    if !(unsafe {
        or(
            V128::from_bytes([0b1010_1010; 16]),
            V128::from_bytes([0b1100_1100; 16]),
        )
        .to_bytes()
            == [0b1110_1110; 16]
    }) {
        return 322;
    }
    if !(unsafe {
        xor(
            V128::from_bytes([0b1010_1010; 16]),
            V128::from_bytes([0b1100_1100; 16]),
        )
        .to_bytes()
            == [0b0110_0110; 16]
    }) {
        return 323;
    }
    let eq = unsafe { cmpeq_i32(i32x4([1; 4]), i32x4([1; 4])) }.to_bytes();
    if eq != [0xff; 16] {
        return eq[0] as u32;
    }
    if !(unsafe { cmpgt_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16] }) {
        return 325;
    }
    if !ops_wide() {
        return 33;
    }
    if !float_ops() {
        return 4;
    }
    if !permute_ops() {
        return 5;
    }
    if !vsx_ops() {
        return 6;
    }
    if !power8_ops() {
        return 7;
    }
    if !crypto_ops() {
        return 8;
    }
    if !power9_ops() {
        return 9;
    }
    if !power10_ops() {
        return 10;
    }
    0
}

fn power9_ops() -> bool {
    unsafe {
        lanes(p9_neg_i32(i32x4([1, -2, 3, -4]))) == [-1, 2, -3, 4]
            && lanes(p9_abs_i32(i32x4([-1, 2, -3, 4]))) == [1, 2, 3, 4]
    }
}

fn power10_ops() -> bool {
    unsafe {
        lanes(p10_popcnt_i32(i32x4([-1; 4]))) == [32; 4]
            && lanes(p10_cntlz_i32(i32x4([1, 2, 4, 8]))) == [31, 30, 29, 28]
    }
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

fn vsx_ops() -> bool {
    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    let f = f32x4([1.0, 2.0, 3.0, 4.0]);
    let g = f32x4([10.0, 20.0, 30.0, 40.0]);
    unsafe {
        f64_lanes(add_f64(a, b)) == [11.0, 22.0]
            && f64_lanes(sub_f64(b, a)) == [9.0, 18.0]
            && f64_lanes(mul_f64(a, b)) == [10.0, 40.0]
            && f64_lanes(div_f64(b, a)) == [10.0, 10.0]
            && f32_lanes(vsx_add_f32(f, g)) == [11.0, 22.0, 33.0, 44.0]
            && f64_lanes(abs_f64(f64x2([-1.0, -2.0]))) == [1.0, 2.0]
            && f64_lanes(sqrt_f64(f64x2([4.0, 9.0]))) == [2.0, 3.0]
    }
}

fn power8_ops() -> bool {
    let a = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    let b = V128::from_bytes([2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]);
    unsafe {
        let sum = add_u64(a, b).to_bytes();
        let diff = sub_u64(b, a).to_bytes();
        if i64::from_le_bytes(sum[..8].try_into().unwrap()) != 3
            || i64::from_le_bytes(sum[8..].try_into().unwrap()) != 5
            || i64::from_le_bytes(diff[..8].try_into().unwrap()) != 1
            || i64::from_le_bytes(diff[8..].try_into().unwrap()) != 1
        {
            return false;
        }
        let c = i32x4([2, 3, 4, 5]);
        let d = i32x4([6, 7, 8, 9]);
        if lanes(mul_u32(c, d)) != [12, 21, 32, 45] {
            return false;
        }
        let ones = i16x8([3; 8]);
        let twos = i16x8([4; 8]);
        lanes(mul_even_u16(ones, twos)) == [12; 4]
    }
}

fn crypto_ops() -> bool {
    let one = V128::from_bytes([1; 16]);
    let two = V128::from_bytes([2; 16]);
    let zero = V128::from_bytes([0; 16]);
    unsafe {
        cipher(one, two).to_bytes() != zero.to_bytes()
            && ncipher(one, two).to_bytes() != zero.to_bytes()
            && cipherlast(one, two).to_bytes() != zero.to_bytes()
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
    if !ops_arithmetic() {
        return false;
    }
    if !ops_bits() {
        return false;
    }
    ops_wide()
}

fn ops_arithmetic() -> bool {
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
        lanes(mul_i32(i32x4([2, 3, 4, 5]), i32x4([6, 7, 8, 9]))) == [12, 21, 32, 45]
    }
}

fn ops_bits() -> bool {
    unsafe {
        let a = V128::from_bytes([0b1010_1010; 16]);
        let b = V128::from_bytes([0b1100_1100; 16]);
        if and(a, b).to_bytes() != [0b1000_1000; 16]
            || or(a, b).to_bytes() != [0b1110_1110; 16]
            || xor(a, b).to_bytes() != [0b0110_0110; 16]
        {
            return false;
        }
        cmpeq_i32(i32x4([1; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16]
            && cmpgt_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16]
    }
}

fn ops_wide() -> bool {
    unsafe {
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
            && f32_lanes(fdiv(b, a)) == [10.0; 4]
    }
}

fn permute_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    let identity = V128::from_bytes(core::array::from_fn(|index| index as u8));
    unsafe {
        permute(a, b, identity).to_bytes() == a.to_bytes()
            && permute(a, b, V128::from_bytes([0x10; 16])).to_bytes() == [16; 16]
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
