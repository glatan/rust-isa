#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_isa::mips::simd::msa::{
    V128, add, add_i8, and, cmpeq_i32, cmpgt_i32, fadd, fadd_f64, fdiv, fdiv_f64, fmul, fmul_f64,
    fsub, fsub_f64, mul, or, sub_i8, sub_i16, v128_load, v128_store, xor,
};

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
