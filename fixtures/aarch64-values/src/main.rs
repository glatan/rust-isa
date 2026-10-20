#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::aarch64::simd::dotprod::sdot;
use rust_isa::aarch64::simd::neon::{
    add, fadd, fmul, mul, tbl, v128_load, v128_store, V128,
};

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

fn i32x4(values: [u32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn lanes(value: V128) -> [u32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|index| u32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap()))
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
