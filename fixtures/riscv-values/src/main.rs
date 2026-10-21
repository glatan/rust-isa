#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::riscv::crypto::zkn::{aes64es, aes64ks1i, sha256sum0};
use rust_isa::riscv::crypto::zks::{sm3p0, sm4ed};
use rust_isa::riscv::float::zfh::{
    fadd_h, fdiv_h, feq_h, fle_h, flh, flt_h, fmax_h, fmin_h, fmul_h, fsh, fsqrt_h, fsub_h,
};
use rust_isa::riscv::integer::zba::{sh1add, sh2add, sh3add};
use rust_isa::riscv::integer::zbb::{andn, clz, cpop, rol, sext_b, xnor};
use rust_isa::riscv::integer::zbc::clmul;
use rust_isa::riscv::integer::zbkb::{brev8, pack, packh, xperm4, xperm8};
use rust_isa::riscv::integer::zbs::{bclr, bext, binv, bset};
use rust_isa::riscv::simd::rv_vector::{
    V128, add, add_i8, add_i16, and, cmpeq_i32, cmpgt_i32, fadd, fmul, mul, mul_i16, or, splat_i32,
    sub_i32, v128_load, v128_store, xor,
};
use rust_isa::riscv::simd::rvv_slice;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "li a7, 93",
            "ecall",
            in("a0") code as usize,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "li a7, 93",
            "ecall",
            in("a0") 255usize,
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
    if !vector_ops() {
        return 3;
    }
    if !scalar_integer_ops() {
        return 4;
    }
    if !slice_ops() {
        return 5;
    }
    if !crypto_ops() {
        return 6;
    }
    if !unsafe { zfh_ops() } {
        return 7;
    }
    0
}

fn i8x16(values: [i8; 16]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane] = *value as u8;
    }
    V128::from_bytes(bytes)
}

fn i8_lanes(value: V128) -> [i8; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| bytes[lane] as i8)
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

fn vector_ops() -> bool {
    unsafe {
        if i8_lanes(add_i8(i8x16([1; 16]), i8x16([2; 16]))) != [3; 16] {
            return false;
        }
        if i16_lanes(add_i16(i16x8([1; 8]), i16x8([2; 8]))) != [3; 8] {
            return false;
        }
        if lanes(sub_i32(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4]))) != [9, 18, 27, 36] {
            return false;
        }
        if i16_lanes(mul_i16(i16x8([3; 8]), i16x8([4; 8]))) != [12; 8] {
            return false;
        }
        let a = V128::from_bytes([0b1010_1010; 16]);
        let b = V128::from_bytes([0b1100_1100; 16]);
        if and(a, b).to_bytes() != [0b1000_1000; 16]
            || or(a, b).to_bytes() != [0b1110_1110; 16]
            || xor(a, b).to_bytes() != [0b0110_0110; 16]
        {
            return false;
        }
        if cmpeq_i32(i32x4([1; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
            || cmpgt_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
        {
            return false;
        }
        if f32_lanes(fadd(
            f32x4([1.0, 2.0, 3.0, 4.0]),
            f32x4([10.0, 20.0, 30.0, 40.0]),
        )) != [11.0, 22.0, 33.0, 44.0]
            || f32_lanes(fmul(
                f32x4([1.0, 2.0, 3.0, 4.0]),
                f32x4([10.0, 20.0, 30.0, 40.0]),
            )) != [10.0, 40.0, 90.0, 160.0]
        {
            return false;
        }
        lanes(splat_i32(7)) == [7; 4]
    }
}

fn scalar_integer_ops() -> bool {
    unsafe {
        if sh1add(3, 1) != 7 || sh2add(3, 1) != 13 || sh3add(3, 1) != 25 {
            return false;
        }
        if andn(0b1010, 0b1100) != 0b0010 || xnor(0b1010, 0b1100) != !0b0110u64 {
            return false;
        }
        if clz(1) != 63 || cpop(0xf0) != 4 || rol(1, 4) != 16 {
            return false;
        }
        if sext_b(0xff) != u64::MAX || clmul(1, 1) != 1 {
            return false;
        }
        if bset(0, 3) != 8 || bclr(0xff, 0) != 0xfe || binv(0, 0) != 1 || bext(8, 3) != 1 {
            return false;
        }
        if pack(0x1111_1111_2222_2222, 0x3333_3333_4444_4444) == 0
            || packh(0xaa, 0xbb) == 0
            || brev8(1) != 0x80
            || xperm8(0x0123_4567_89ab_cdef, 0) == 0
            || xperm4(0x0123_4567_89ab_cdef, 0) == 0
        {
            return false;
        }
    }
    true
}

fn slice_ops() -> bool {
    let a = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let b = [10.0f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0];
    let mut out = [0.0f32; 7];
    unsafe {
        rvv_slice::add_f32(&a, &b, &mut out);
        if out != [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0] {
            return false;
        }
        rvv_slice::mul_f32(&a, &b, &mut out);
        if out != [10.0, 40.0, 90.0, 160.0, 250.0, 360.0, 490.0] {
            return false;
        }
        let a = [1i32, 2, 3, 4, 5];
        let b = [10i32, 20, 30, 40, 50];
        let mut out = [0i32; 5];
        rvv_slice::add_i32(&a, &b, &mut out);
        if out != [11, 22, 33, 44, 55] {
            return false;
        }
        rvv_slice::mul_i32(&a, &b, &mut out);
        out == [10, 40, 90, 160, 250]
    }
}

#[target_feature(enable = "zkn")]
unsafe fn zkn_ops() -> bool {
    unsafe {
        sha256sum0(1) != 1
            && aes64es(1, 2) != 1
            && aes64ks1i(1, 0) != 1
            && aes64ks1i(1, 1) != aes64ks1i(1, 0)
    }
}

#[target_feature(enable = "zks")]
unsafe fn zks_ops() -> bool {
    unsafe { sm3p0(1) != 1 && sm4ed::<0>(1, 2) != 1 }
}

fn crypto_ops() -> bool {
    unsafe { zkn_ops() && zks_ops() }
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
    unsafe {
        if lanes(add(a, b)) != [11, 22, 33, 44] {
            return false;
        }
        if lanes(mul(a, b)) != [10, 40, 90, 160] {
            return false;
        }
        if lanes(kernel_add(&a, &b)) != [21, 42, 63, 84] {
            return false;
        }
        if lanes(kernel_mul(&a, &b)) != [100, 800, 2700, 6400] {
            return false;
        }
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut index = 0;
    while index < n {
        let left = unsafe { *a.add(index) };
        let right = unsafe { *b.add(index) };
        if left != right {
            return 1;
        }
        index += 1;
    }
    0
}

unsafe fn zfh_ops() -> bool {
    let one = 0x3c00u16;
    let two = 0x4000u16;
    let four = 0x4400u16;
    unsafe {
        fadd_h(one, two) == 0x4200
            && fsub_h(two, one) == one
            && fmul_h(two, two) == four
            && fdiv_h(four, two) == two
            && fsqrt_h(four) == two
            && fmin_h(one, two) == one
            && fmax_h(one, two) == two
            && feq_h(one, one)
            && flt_h(one, two)
            && fle_h(one, one)
            && {
                let mut bytes = [0u8; 2];
                fsh(bytes.as_mut_ptr(), two);
                flh(bytes.as_ptr()) == two
            }
    }
}
