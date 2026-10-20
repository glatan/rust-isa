#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::s390x::simd::vector::{add, mul, v128_load, v128_store, V128};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "svc 1",
            in("r1") 1usize,
            in("r2") code as usize,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "svc 1",
            in("r1") 1usize,
            in("r2") 255usize,
            options(noreturn, nostack)
        );
    }
}

#[target_feature(enable = "vector")]
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_add(a: &V128, b: &V128) -> V128 {
    unsafe { add(add(*a, *b), *b) }
}

#[target_feature(enable = "vector")]
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
    0
}

fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_be_bytes());
    }
    V128::from_bytes(bytes)
}

fn lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|index| {
        i32::from_be_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
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
