#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::loongarch::integer::lbt::{bitrev, bstrins, bstrpick, ext, ins, revb, revh, revw};
use rust_isa::loongarch::simd::lasx::{xvdiv_s};
use rust_isa::loongarch::simd::lasx::{xvmul_w};
use rust_isa::loongarch::simd::lasx::{xvsub_w};
use rust_isa::loongarch::simd::lasx::{xvadd_w};
use rust_isa::loongarch::simd::lasx::{xvneg_d};
use rust_isa::loongarch::simd::lasx::{xvneg_h};
use rust_isa::loongarch::simd::lasx::{xvneg_b};
use rust_isa::loongarch::simd::lasx::{xvabs_d};
use rust_isa::loongarch::simd::lasx::{xvabs_h};
use rust_isa::loongarch::simd::lasx::{xvabs_b};
use rust_isa::loongarch::simd::lasx::{
    V256, add as lasx_add, add_i64 as lasx_add_i64, and as lasx_and, fadd as lasx_fadd,
    fdiv as lasx_fdiv, fmul as lasx_fmul, fsub as lasx_fsub, mul as lasx_mul,
    mul_i32 as lasx_mul_i32, or as lasx_or, splat_i32 as lasx_splat_i32, sub as lasx_sub,
    sub_i64 as lasx_sub_i64, v256_load, v256_store, xor as lasx_xor, xvadd, xvand, xvmul, xvor,
    xvsub, xvxor,
};
use rust_isa::loongarch::simd::lasx::{
    xvabs, xvadd_b, xvadd_d, xvadd_h, xvdiv, xvmul_h, xvneg, xvsub_b, xvsub_d, xvsub_h,
};
use rust_isa::loongarch::simd::lsx::{vslt};
use rust_isa::loongarch::simd::lsx::{vsle};
use rust_isa::loongarch::simd::lsx::{vseq};
use rust_isa::loongarch::simd::lsx::{vmin};
use rust_isa::loongarch::simd::lsx::{vmax};
use rust_isa::loongarch::simd::lsx::{vneg_d};
use rust_isa::loongarch::simd::lsx::{vneg_h};
use rust_isa::loongarch::simd::lsx::{vneg_b};
use rust_isa::loongarch::simd::lsx::{vabs_d};
use rust_isa::loongarch::simd::lsx::{vabs_h};
use rust_isa::loongarch::simd::lsx::{vabs_b};
use rust_isa::loongarch::simd::lsx::{vneg_w};
use rust_isa::loongarch::simd::lsx::{vmul_w};
use rust_isa::loongarch::simd::lsx::{vsub_w};
use rust_isa::loongarch::simd::lsx::{vadd_w};
use rust_isa::loongarch::simd::lsx::{
    V128, add, add_i8, and, cmpeq_i32, cmpgt_i32, fadd, fdiv, fmul, fsub, mul, mul_i16, or,
    splat_i32, sub_i16, sub_i32, v128_load, v128_store, vabs, vadd, vand, vmul, vneg, vor, vsub,
    vxor, xor,
};
use rust_isa::loongarch::simd::lsx::{add_i16, sub_i8, vadd_b, vadd_h, vmul_h, vsub_b, vsub_h};
use rust_isa::loongarch::simd::lsx::{add_i64, sub_i64, vadd_d, vdiv, vsub_d};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "li.d $a7, 93",
            "syscall 0",
            in("$a0") code as usize,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "li.d $a7, 93",
            "syscall 0",
            in("$a0") 255usize,
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
pub extern "C" fn kernel_mul(a: &V128, b: &V128) -> V128 {
    mul(mul(*a, *b), *b)
}

fn run() -> u32 {
    if !load_store() {
        return 1;
    }
    if !integer() {
        return 2;
    }
    if !lsx_ops() {
        return 3;
    }
    if !unsafe { lasx_ops() } {
        return 4;
    }
    if !lbt_ops() {
        return 5;
    }
    if !lsx_width_ops() {
        return 6;
    }
    if !lasx_width_ops() {
        return 7;
    }
    if !lasx_alias_ops() {
        return 6;
    }
    if !lsx_abs_ops() {
        return 7;
    }
    if !lasx_abs_ops() {
        return 8;
    }
    if !lsx_div_ops() {
        return 9;
    }
    if !lasx_div_ops() {
        return 10;
    }
    if !lsx_alias_ops() {
        return 6;
    }
    if !alias_vadd_w() { return 101; }
    if !alias_vsub_w() { return 102; }
    if !alias_vmul_w() { return 103; }
    if !alias_vneg_w() { return 104; }
    if !alias_vabs_b() { return 105; }
    if !alias_vabs_h() { return 201; }
    if !alias_vabs_d() { return 202; }
    if !alias_vneg_b() { return 203; }
    if !alias_vneg_h() { return 204; }
    if !alias_vneg_d() { return 205; }
    if !alias_xvabs_b() { return 301; }
    if !alias_xvabs_h() { return 302; }
    if !alias_xvabs_d() { return 303; }
    if !alias_xvneg_b() { return 304; }
    if !alias_xvneg_h() { return 305; }
    if !alias_xvneg_d() { return 401; }
    if !alias_xvadd_w() { return 402; }
    if !alias_xvsub_w() { return 421; }
    if !alias_xvmul_w() { return 422; }
    if !alias_xvdiv_s() { return 423; }
    if !alias_vmax() { return 501; }
    if !alias_vmin() { return 511; }
    if !alias_vseq() { return 512; }
    if !alias_vsle() { return 513; }
    if !alias_vslt() { return 514; }
    0
}

fn i8x16(values: [i8; 16]) -> V128 {
    V128::from_bytes(core::array::from_fn(|lane| values[lane] as u8))
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

fn lsx_ops() -> bool {
    if i8_lanes(add_i8(i8x16([1; 16]), i8x16([2; 16]))) != [3; 16] {
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
    let a = V128::from_bytes([0b1010_1010; 16]);
    let b = V128::from_bytes([0b1100_1100; 16]);
    if and(a, b).to_bytes() != [0b1000_1000; 16]
        || or(a, b).to_bytes() != [0b1110_1110; 16]
        || xor(a, b).to_bytes() != [0b0110_0110; 16]
        || cmpeq_i32(i32x4([1; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
        || cmpgt_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() != [0xff; 16]
    {
        return false;
    }
    if f32_lanes(fadd(
        f32x4([1.0, 2.0, 3.0, 4.0]),
        f32x4([10.0, 20.0, 30.0, 40.0]),
    )) != [11.0, 22.0, 33.0, 44.0]
        || f32_lanes(fsub(
            f32x4([10.0, 20.0, 30.0, 40.0]),
            f32x4([1.0, 2.0, 3.0, 4.0]),
        )) != [9.0, 18.0, 27.0, 36.0]
        || f32_lanes(fmul(
            f32x4([1.0, 2.0, 3.0, 4.0]),
            f32x4([10.0, 20.0, 30.0, 40.0]),
        )) != [10.0, 40.0, 90.0, 160.0]
        || f32_lanes(fdiv(
            f32x4([10.0, 40.0, 90.0, 160.0]),
            f32x4([1.0, 2.0, 3.0, 4.0]),
        )) != [10.0, 20.0, 30.0, 40.0]
    {
        return false;
    }
    lanes(splat_i32(7)) == [7; 4]
}

fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x8_lanes(value: V256) -> [i32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f32x8(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn f32x8_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

#[target_feature(enable = "lasx")]
unsafe fn lasx_ops() -> bool {
    let source = core::array::from_fn::<u8, 32, _>(|index| index as u8);
    let mut unaligned = [0u8; 33];
    unaligned[1..].copy_from_slice(&source);
    if unsafe { v256_load(unaligned.as_ptr().add(1)) }.to_bytes() != source {
        return false;
    }
    let value = unsafe { v256_load(source.as_ptr()) };
    if value.to_bytes() != source {
        return false;
    }
    let mut output = [0u8; 32];
    unsafe { v256_store(output.as_mut_ptr(), value) };
    if output != source {
        return false;
    }
    let a = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i32x8([10, 20, 30, 40, 50, 60, 70, 80]);
    if i32x8_lanes(unsafe { lasx_add(a, b) }) != [11, 22, 33, 44, 55, 66, 77, 88] {
        return false;
    }
    if i32x8_lanes(unsafe { lasx_mul(a, b) }) != [10, 40, 90, 160, 250, 360, 490, 640] {
        return false;
    }
    if i32x8_lanes(unsafe { lasx_sub(b, a) }) != [9, 18, 27, 36, 45, 54, 63, 72] {
        return false;
    }
    let a64 = V256::from_bytes({
        let mut bytes = [0u8; 32];
        for lane in 0..4 {
            bytes[lane * 8..lane * 8 + 8].copy_from_slice(&(lane as i64 + 1).to_le_bytes());
        }
        bytes
    });
    let b64 = V256::from_bytes({
        let mut bytes = [0u8; 32];
        for lane in 0..4 {
            bytes[lane * 8..lane * 8 + 8].copy_from_slice(&((lane as i64 + 1) * 10).to_le_bytes());
        }
        bytes
    });
    let sum = unsafe { lasx_add_i64(a64, b64) }.to_bytes();
    let diff = unsafe { lasx_sub_i64(b64, a64) }.to_bytes();
    for lane in 0..4 {
        let left = i64::from_le_bytes(sum[lane * 8..lane * 8 + 8].try_into().unwrap());
        let right = i64::from_le_bytes(diff[lane * 8..lane * 8 + 8].try_into().unwrap());
        if left != (lane as i64 + 1) * 11 || right != (lane as i64 + 1) * 9 {
            return false;
        }
    }
    if i32x8_lanes(unsafe { lasx_mul_i32(a, b) }) != [10, 40, 90, 160, 250, 360, 490, 640] {
        return false;
    }
    let bits_a = V256::from_bytes([0b1010_1010; 32]);
    let bits_b = V256::from_bytes([0b1100_1100; 32]);
    if unsafe { lasx_and(bits_a, bits_b) }.to_bytes() != [0b1000_1000; 32]
        || unsafe { lasx_or(bits_a, bits_b) }.to_bytes() != [0b1110_1110; 32]
        || unsafe { lasx_xor(bits_a, bits_b) }.to_bytes() != [0b0110_0110; 32]
    {
        return false;
    }
    let fa = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let fb = f32x8([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
    if f32x8_lanes(unsafe { lasx_fadd(fa, fb) }) != [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0, 88.0]
        || f32x8_lanes(unsafe { lasx_fsub(fb, fa) })
            != [9.0, 18.0, 27.0, 36.0, 45.0, 54.0, 63.0, 72.0]
        || f32x8_lanes(unsafe { lasx_fmul(fa, fb) })
            != [10.0, 40.0, 90.0, 160.0, 250.0, 360.0, 490.0, 640.0]
        || f32x8_lanes(unsafe { lasx_fdiv(fb, fa) }) != [10.0; 8]
    {
        return false;
    }
    i32x8_lanes(unsafe { lasx_splat_i32(7) }) == [7; 8]
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
    let mut unaligned = [0u8; 33];
    unaligned[1..17].copy_from_slice(&source);
    if unsafe { v128_load(unaligned.as_ptr().add(1)) }.to_bytes() != source {
        return false;
    }
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
    lanes(add(a, b)) == [11, 22, 33, 44]
        && lanes(mul(a, b)) == [10, 40, 90, 160]
        && lanes(kernel_add(&a, &b)) == [21, 42, 63, 84]
        && lanes(kernel_mul(&a, &b)) == [100, 800, 2700, 6400]
}

fn lasx_abs_ops() -> bool {
    let a = i32x8([-1, 2, -3, 4, -5, 6, -7, 8]);
    unsafe {
        i32x8_lanes(xvabs(a)) == [1, 2, 3, 4, 5, 6, 7, 8]
            && i32x8_lanes(xvneg(a)) == [1, -2, 3, -4, 5, -6, 7, -8]
    }
}

fn lsx_abs_ops() -> bool {
    lanes(vabs(i32x4([-1, 2, -3, 4]))) == [1, 2, 3, 4]
        && lanes(vneg(i32x4([1, -2, 3, -4]))) == [-1, 2, -3, 4]
}

fn lasx_div_ops() -> bool {
    let a = V256::from_bytes(core::array::from_fn(|i| i as u8));
    let b = V256::from_bytes(core::array::from_fn(|i| (i + 1) as u8));
    unsafe {
        xvdiv(a, b) == lasx_fdiv(a, b)
            && xvadd_d(a, b) == lasx_add_i64(a, b)
            && xvsub_d(b, a) == lasx_sub_i64(b, a)
    }
}

fn lsx_div_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|i| i as u8));
    let b = V128::from_bytes(core::array::from_fn(|i| (i + 1) as u8));
    vdiv(a, b) == fdiv(a, b) && vadd_d(a, b) == add_i64(a, b) && vsub_d(b, a) == sub_i64(b, a)
}

fn lasx_width_ops() -> bool {
    let a = V256::from_bytes([1; 32]);
    let b = V256::from_bytes([2; 32]);
    unsafe {
        xvadd_b(a, b).to_bytes() == [3; 32]
            && xvadd_h(a, b).to_bytes() == [3; 32]
            && xvsub_b(b, a).to_bytes() == [1; 32]
            && xvsub_h(b, a).to_bytes() == [1; 32]
            && xvmul_h(b, b).to_bytes() != [0; 32]
    }
}

fn lsx_width_ops() -> bool {
    let a = V128::from_bytes([1; 16]);
    let b = V128::from_bytes([2; 16]);
    vadd_b(a, b) == add_i8(a, b)
        && vadd_h(a, b) == add_i16(a, b)
        && vsub_b(b, a) == sub_i8(b, a)
        && vsub_h(b, a) == sub_i16(b, a)
        && vmul_h(a, b) == mul_i16(a, b)
}

fn lbt_ops() -> bool {
    revb(0x0102_0304_0506_0708) == 0x0807_0605_0403_0201
        && revh(0x0102_0304_0506_0708) == 0x0708_0506_0304_0102
        && revw(0x0102_0304_0506_0708) == 0x0506_0708_0102_0304
        && bitrev(1) == 1u64 << 63
        && bstrpick::<7, 4>(0xab) == 0xa
        && bstrins::<7, 4>(0, 0xa) == 0xa0
        && ext::<7, 4>(0xab) == 0xa
        && ins::<7, 4>(0, 0xa) == 0xa0
}

fn lsx_alias_ops() -> bool {
    let a = i32x4([1, 2, 3, 4]);
    let b = i32x4([5, 6, 7, 8]);
    vadd(a, b) == add(a, b)
        && vsub(b, a) == sub_i32(b, a)
        && vmul(a, b) == mul(a, b)
        && vand(a, b) == and(a, b)
        && vor(a, b) == or(a, b)
        && vxor(a, b) == xor(a, b)
}

fn lasx_alias_ops() -> bool {
    let a = V256::from_bytes([1; 32]);
    let b = V256::from_bytes([2; 32]);
    unsafe {
        xvadd(a, b) == lasx_add(a, b)
            && xvsub(b, a) == lasx_sub(b, a)
            && xvmul(a, b) == lasx_mul(a, b)
            && xvand(a, b) == lasx_and(a, b)
            && xvor(a, b) == lasx_or(a, b)
            && xvxor(a, b) == lasx_xor(a, b)
    }
}

fn alias_vadd_w() -> bool { vadd_w(i32x4([1,2,3,4]), i32x4([5,6,7,8])) == add(i32x4([1,2,3,4]), i32x4([5,6,7,8])) }

fn alias_vsub_w() -> bool { vsub_w(i32x4([5,6,7,8]), i32x4([1,2,3,4])) == sub_i32(i32x4([5,6,7,8]), i32x4([1,2,3,4])) }

fn alias_vmul_w() -> bool { vmul_w(i32x4([1,2,3,4]), i32x4([5,6,7,8])) == mul(i32x4([1,2,3,4]), i32x4([5,6,7,8])) }

fn alias_vneg_w() -> bool { vneg_w(i32x4([1,-2,3,-4])) == vneg(i32x4([1,-2,3,-4])) }

fn alias_vabs_b() -> bool { vabs_b(V128::from_bytes([0xff; 16])) == V128::from_bytes([1; 16]) }

fn alias_vabs_h() -> bool { i16_lanes(vabs_h(i16x8([-1;8]))) == [1;8] }

fn alias_vabs_d() -> bool { vabs_d(V128::from_bytes([0xff;16])).to_bytes() == { let mut b=[0u8;16]; b[0]=1; b[8]=1; b } }

fn alias_vneg_b() -> bool { vneg_b(V128::from_bytes([1;16])).to_bytes() == [0xff;16] }

fn alias_vneg_h() -> bool { i16_lanes(vneg_h(i16x8([1;8]))) == [-1;8] }

fn alias_vneg_d() -> bool { { let mut x=[0u8;16]; x[0]=1; let mut e=[0u8;16]; for i in 0..8 { e[i]=0xff; } vneg_d(V128::from_bytes(x)).to_bytes() == e } }

fn alias_xvabs_b() -> bool { unsafe { xvabs_b(V256::from_bytes([0xff;32])).to_bytes() == [1;32] } }

fn alias_xvabs_h() -> bool { unsafe { xvabs_h(V256::from_bytes([0xff;32])).to_bytes() == V256::from_bytes(core::array::from_fn(|i| (i%2==0) as u8)).to_bytes() } }

fn alias_xvabs_d() -> bool { unsafe { xvabs_d(V256::from_bytes([0xff;32])).to_bytes() == V256::from_bytes(core::array::from_fn(|i| (i%8==0) as u8)).to_bytes() } }

fn alias_xvneg_b() -> bool { unsafe { xvneg_b(V256::from_bytes([1;32])).to_bytes() == [0xff;32] } }

fn alias_xvneg_h() -> bool { unsafe { xvneg_h(V256::from_bytes([1;32])).to_bytes() == V256::from_bytes(core::array::from_fn(|i| if i%2==0 {0xff} else {0xfe})).to_bytes() } }

fn alias_xvneg_d() -> bool { unsafe { { let mut x=[0u8;32]; x[0]=1; let mut e=[0u8;32]; for i in 0..8 { e[i]=0xff; } xvneg_d(V256::from_bytes(x)).to_bytes()==e } } }

fn alias_xvadd_w() -> bool { unsafe { xvadd_w(V256::from_bytes([1;32]), V256::from_bytes([2;32])) == lasx_add(V256::from_bytes([1;32]), V256::from_bytes([2;32])) } }

fn alias_xvsub_w() -> bool { unsafe { xvsub_w(V256::from_bytes([3;32]), V256::from_bytes([2;32])) == lasx_sub(V256::from_bytes([3;32]), V256::from_bytes([2;32])) } }

fn alias_xvmul_w() -> bool { unsafe { xvmul_w(V256::from_bytes([1;32]), V256::from_bytes([2;32])) == lasx_mul(V256::from_bytes([1;32]), V256::from_bytes([2;32])) } }

fn alias_xvdiv_s() -> bool { unsafe { xvdiv_s(V256::from_bytes([1;32]), V256::from_bytes([2;32])) == lasx_fdiv(V256::from_bytes([1;32]), V256::from_bytes([2;32])) } }

fn alias_vmax() -> bool { lanes(vmax(i32x4([1,4,2,8]),i32x4([3,2,6,4]))) == [3,4,6,8] }

fn alias_vmin() -> bool { lanes(vmin(i32x4([1,4,2,8]),i32x4([3,2,6,4]))) == [1,2,2,4] }

fn alias_vseq() -> bool { vseq(i32x4([1,2,3,4]),i32x4([1,0,3,4])).to_bytes() == [0xff,0xff,0xff,0xff,0,0,0,0,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff] }

fn alias_vsle() -> bool { vsle(i32x4([1,4,2,8]),i32x4([3,2,6,4])).to_bytes() == [0xff,0xff,0xff,0xff,0,0,0,0,0xff,0xff,0xff,0xff,0,0,0,0] }

fn alias_vslt() -> bool { vslt(i32x4([1,4,2,8]),i32x4([3,2,6,4])).to_bytes() == [0xff,0xff,0xff,0xff,0,0,0,0,0xff,0xff,0xff,0xff,0,0,0,0] }
