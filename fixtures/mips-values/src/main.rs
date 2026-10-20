#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_isa::mips::simd::msa::{add, mul, v128_load, v128_store, V128};

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

fn integer() -> u32 {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    unsafe {
        if add(a, b).to_bytes() != [3; 16] {
            return 10;
        }
        if mul(V128::from_bytes([1; 16]), V128::from_bytes([1; 16])).to_bytes() != repeat4([4, 3, 2, 1]) {
            return 11;
        }
    }
    0
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    if !load_store() {
        return 1;
    }
    integer()
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
