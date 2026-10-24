#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::x86::simd::avx2::{V256, vpaddd, vpmulld};
use rust_isa::x86::simd::avx512f::{V512, vpaddd as vpaddd_512, vpmulld as vpmulld_512};
use rust_isa::x86::simd::sse2::{V128, paddd, pmuludq, pslld, punpcklbw, v128_load, v128_store};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "int 0x80",
            in("eax") 1u32,
            in("ebx") code,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "int 0x80",
            in("eax") 1u32,
            in("ebx") 255u32,
            options(noreturn, nostack)
        );
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_sse2(a: &V128, b: &V128) -> V128 {
    pmuludq(paddd(paddd(*a, *b), *b), *b)
}

#[inline(never)]
#[target_feature(enable = "avx2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_avx2(a: &V256, b: &V256) -> V256 {
    unsafe { vpmulld(vpaddd(vpaddd(*a, *b), *b), *b) }
}

#[target_feature(enable = "avx512f")]
unsafe fn avx512() -> bool {
    let mut a = [0u8; 64];
    let mut b = [0u8; 64];
    for lane in 0..16 {
        a[lane * 4..lane * 4 + 4].copy_from_slice(&(lane as i32 + 1).to_le_bytes());
        b[lane * 4..lane * 4 + 4].copy_from_slice(&((lane as i32 + 1) * 10).to_le_bytes());
    }
    let sum = unsafe { vpaddd_512(V512::from_bytes(a), V512::from_bytes(b)) }.to_bytes();
    let product = unsafe { vpmulld_512(V512::from_bytes(a), V512::from_bytes(b)) }.to_bytes();
    for lane in 0..16 {
        let sum_lane = i32::from_le_bytes(sum[lane * 4..lane * 4 + 4].try_into().unwrap());
        let product_lane = i32::from_le_bytes(product[lane * 4..lane * 4 + 4].try_into().unwrap());
        if sum_lane != (lane as i32 + 1) * 11
            || product_lane != (lane as i32 + 1) * (lane as i32 + 1) * 10
        {
            return false;
        }
    }
    true
}

fn run() -> u32 {
    let a128 = V128::from_bytes([1; 16]);
    let b128 = V128::from_bytes([2; 16]);
    let a256 = V256::from_bytes([1; 32]);
    let b256 = V256::from_bytes([2; 32]);
    core::hint::black_box(kernel_sse2(&a128, &b128));
    core::hint::black_box(unsafe { kernel_avx2(&a256, &b256) });

    let load = load_store();
    if load != 0 {
        return load;
    }
    if !integer() {
        return 2;
    }
    if !shuffle() {
        return 3;
    }
    if !unsafe { avx2() } {
        return 4;
    }
    if !unsafe { avx512() } {
        return 5;
    }
    0
}

#[target_feature(enable = "avx2")]
unsafe fn avx2() -> bool {
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    for lane in 0..8 {
        a[lane * 4..lane * 4 + 4].copy_from_slice(&(lane as i32 + 1).to_le_bytes());
        b[lane * 4..lane * 4 + 4].copy_from_slice(&((lane as i32 + 1) * 10).to_le_bytes());
    }
    let out = unsafe { vpaddd(V256::from_bytes(a), V256::from_bytes(b)) }.to_bytes();
    for lane in 0..8 {
        let value = i32::from_le_bytes(out[lane * 4..lane * 4 + 4].try_into().unwrap());
        if value != (lane as i32 + 1) * 11 {
            return false;
        }
    }
    true
}

fn load_store() -> u32 {
    let source = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let value = unsafe { v128_load(source.as_ptr()) };
    if value.to_bytes() != source {
        return 10;
    }
    let mut output = [0u8; 16];
    unsafe {
        v128_store(output.as_mut_ptr(), value);
    }
    if output != source {
        return 11;
    }
    0
}

fn integer() -> bool {
    let a = V128::from_bytes([1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
    let b = V128::from_bytes([10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0]);
    if paddd(a, b).to_bytes() != [11, 0, 0, 0, 22, 0, 0, 0, 33, 0, 0, 0, 44, 0, 0, 0] {
        return false;
    }
    if pslld(a, 1).to_bytes() != [2, 0, 0, 0, 4, 0, 0, 0, 6, 0, 0, 0, 8, 0, 0, 0] {
        return false;
    }
    true
}

fn shuffle() -> bool {
    let a = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let b = V128::from_bytes([
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    ]);
    punpcklbw(a, b).to_bytes() == [1, 11, 2, 12, 3, 13, 4, 14, 5, 15, 6, 16, 7, 17, 8, 18]
}
