#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::riscv::crypto::zkn::{aes64es, aes64ks1i, sha256sum0};
use rust_isa::riscv::crypto::zks::{sm3p0, sm4ed};
use rust_isa::riscv::float::zfh::{
    fadd_d, fadd_s, fdiv_d, fdiv_s, feq_d, feq_s, fle_d, fle_s, flt_d, flt_s, fmax_d, fmax_s,
    fmaxm_d, fmaxm_s, fmin_d, fmin_s, fminm_d, fminm_s, fmul_d, fmul_s, fround_d, fround_s,
    froundnx_d, froundnx_s, fsqrt_d, fsqrt_s, fsub_d, fsub_s,
}; // single and double precision
use rust_isa::riscv::float::zfh::{
    fadd_h, fdiv_h, feq_h, fle_h, flh, flt_h, fmax_h, fmin_h, fmul_h, fsh, fsqrt_h, fsub_h,
};
use rust_isa::riscv::float::zfh::{
    fclass_h, fcvt_d_h, fcvt_h_d, fcvt_h_l, fcvt_h_lu, fcvt_h_s, fcvt_h_w, fcvt_h_wu, fcvt_l_h,
    fcvt_lu_h, fcvt_s_h, fcvt_w_h, fcvt_wu_h, fli_h, fmv_h_x, fmv_x_h,
}; // conversions
use rust_isa::riscv::float::zfh::{
    fcvt_d_l, fcvt_d_lu, fcvt_d_s, fcvt_d_w, fcvt_d_wu, fcvt_l_d, fcvt_l_s, fcvt_lu_d, fcvt_lu_s,
    fcvt_s_d, fcvt_s_l, fcvt_s_lu, fcvt_s_w, fcvt_s_wu, fcvt_w_d, fcvt_w_s, fcvt_wu_d, fcvt_wu_s,
    fli_d, fli_s, fmv_d_x, fmv_w_x, fmv_x_d, fmv_x_w, fmvh_x_d, fmvp_d_x,
}; // single and double precision conversions
use rust_isa::riscv::float::zfh::{fcvtmod_w_d, fleq_d, fleq_s, fltq_d, fltq_s}; // zfa single and double precision
use rust_isa::riscv::float::zfh::{
    fcvtmod_w_h, fleq_h, fltq_h, fmaxm_h, fminm_h, fround_h, froundnx_h,
}; // zfa
use rust_isa::riscv::integer::zba::{sh1add, sh2add, sh3add};
use rust_isa::riscv::integer::zbb::{andn, clz, cpop, rol, sext_b, xnor};
use rust_isa::riscv::integer::zbc::clmul;
use rust_isa::riscv::integer::zbkb::{brev8, pack, packh, xperm4, xperm8};
use rust_isa::riscv::integer::zbs::{bclr, bext, binv, bset};
use rust_isa::riscv::simd::rv_vector::{
    V128, add, add_i8, add_i16, and, cmpeq_i32, cmpgt_i32, fadd, fmul, mul, mul_i16, or, splat_i32,
    sub_i32, v128_load, v128_store, vadd, vand, vdiv, vdivu, vmacc, vmadd, vmax, vmaxu, vmin,
    vminu, vmseq, vmseq_mask, vmsgeu_mask, vmsgt, vmsgt_mask, vmslt, vmsne_mask, vmul, vmv,
    vmv_s_x, vmv_v_x, vmv_x_s, vnmsac, vnmsub, vor, vrem, vremu, vsll, vsra, vsrl, vsub, vxor, xor,
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
    if !unsafe { scalar_float_ops() } {
        return 8;
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
        if lanes(splat_i32(7)) != [7; 4] {
            return false;
        }
        if lanes(vadd(i32x4([1, 2, 3, 4]), i32x4([10, 20, 30, 40]))) != [11, 22, 33, 44]
            || lanes(vsub(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4]))) != [9, 18, 27, 36]
            || lanes(vmul(i32x4([1, 2, 3, 4]), i32x4([10, 20, 30, 40]))) != [10, 40, 90, 160]
        {
            return false;
        }
        if lanes(vdiv(i32x4([10, -20, 30, -40]), i32x4([2, 3, 4, 5]))) != [5, -6, 7, -8]
            || lanes(vdiv(i32x4([1, 0, 0, 0]), i32x4([0; 4]))) != [-1; 4]
        {
            return false;
        }
        if lanes(vrem(i32x4([10, -20, 30, -40]), i32x4([3, 6, 7, 9]))) != [1, -2, 2, -4] {
            return false;
        }
        if lanes(vdivu(i32x4([-1, 7, 0, 9]), i32x4([2; 4]))) != [0x7fff_ffff, 3, 0, 4]
            || lanes(vremu(i32x4([-1, 7, 9, 11]), i32x4([3; 4]))) != [0, 1, 0, 2]
        {
            return false;
        }
        if lanes(vsll(i32x4([1, 2, 3, 4]), i32x4([1, 2, 3, 4]))) != [2, 8, 24, 64] {
            return false;
        }
        if lanes(vsrl(i32x4([-8, -16, 16, 32]), i32x4([1, 2, 3, 4])))
            != [0x7fff_fffc, 0x3fff_fffc, 2, 2]
        {
            return false;
        }
        if lanes(vsra(i32x4([-8, -16, 16, 32]), i32x4([1, 2, 3, 4]))) != [-4, -4, 2, 2] {
            return false;
        }
        if lanes(vmin(i32x4([-1, 5, 3, 7]), i32x4([2, 4, 6, 0]))) != [-1, 4, 3, 0]
            || lanes(vmax(i32x4([-1, 5, 3, 7]), i32x4([2, 4, 6, 0]))) != [2, 5, 6, 7]
        {
            return false;
        }
        if lanes(vminu(i32x4([-1, 5, 3, 7]), i32x4([2, 4, 6, 0]))) != [2, 4, 3, 0]
            || lanes(vmaxu(i32x4([-1, 5, 3, 7]), i32x4([2, 4, 6, 0]))) != [-1, 5, 6, 7]
        {
            return false;
        }
        if vmseq_mask(i32x4([1, 2, 3, 4]), i32x4([1, 0, 3, 0])) != 0b0101
            || vmsne_mask(i32x4([1, 2, 3, 4]), i32x4([1, 0, 3, 0])) != 0b1010
            || vmslt(i32x4([1, 2, 3, 4]), i32x4([2, 2, 2, 2])).to_bytes()
                != [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            || vmseq(i32x4([1, 2, 3, 4]), i32x4([1, 0, 3, 0])).to_bytes()
                != [
                    0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
                ]
        {
            return false;
        }
        if vmsgeu_mask(i32x4([-1, 0, 1, 2]), i32x4([0; 4])) != 0b1111
            || vmsgt_mask(i32x4([1, 2, 3, 4]), i32x4([2, 2, 2, 2])) != 0b1100
        {
            return false;
        }
        if vmsgt(i32x4([2, 2, 2, 2]), i32x4([1, 2, 3, 4])).to_bytes()
            != [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        {
            return false;
        }
        if lanes(vmacc(
            i32x4([10, 20, 30, 40]),
            i32x4([1, 2, 3, 4]),
            i32x4([2, 3, 4, 5]),
        )) != [12, 26, 42, 60]
            || lanes(vnmsac(
                i32x4([10, 20, 30, 40]),
                i32x4([1, 2, 3, 4]),
                i32x4([2, 3, 4, 5]),
            )) != [8, 14, 18, 20]
            || lanes(vmadd(
                i32x4([10, 20, 30, 40]),
                i32x4([1, 2, 3, 4]),
                i32x4([2, 3, 4, 5]),
            )) != [12, 43, 94, 165]
            || lanes(vnmsub(
                i32x4([10, 20, 30, 40]),
                i32x4([1, 2, 3, 4]),
                i32x4([2, 3, 4, 5]),
            )) != [-8, -37, -86, -155]
        {
            return false;
        }
        if vand(i32x4([0b1010; 4]), i32x4([0b1100; 4])) != i32x4([0b1000; 4])
            || vor(i32x4([0b1010; 4]), i32x4([0b1100; 4])) != i32x4([0b1110; 4])
            || vxor(i32x4([0b1010; 4]), i32x4([0b1100; 4])) != i32x4([0b0110; 4])
        {
            return false;
        }
        if vmv(i32x4([9, 8, 7, 6])).to_bytes() != i32x4([9, 8, 7, 6]).to_bytes()
            || lanes(vmv_v_x(5)) != [5; 4]
            || vmv_x_s(i32x4([-1, 2, 3, 4])) != -1
            || lanes(vmv_s_x(-2)) != [-2, 0, 0, 0]
        {
            return false;
        }
        true
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
    let mut mask = [0i32; 7];
    unsafe {
        rvv_slice::add_f32(&a, &b, &mut out);
        if out != [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0] {
            return false;
        }
        rvv_slice::mul_f32(&a, &b, &mut out);
        if out != [10.0, 40.0, 90.0, 160.0, 250.0, 360.0, 490.0] {
            return false;
        }
        rvv_slice::sub_f32(&b, &a, &mut out);
        if out != [9.0, 18.0, 27.0, 36.0, 45.0, 54.0, 63.0] {
            return false;
        }
        rvv_slice::div_f32(&b, &a, &mut out);
        if out != [10.0; 7] {
            return false;
        }
        rvv_slice::min_f32(&a, &b, &mut out);
        if out != a {
            return false;
        }
        rvv_slice::max_f32(&a, &b, &mut out);
        if out != b {
            return false;
        }
        rvv_slice::cmpeq_f32(&a, &a, &mut mask);
        if mask != [-1; 7] {
            return false;
        }
        rvv_slice::cmpne_f32(&a, &b, &mut mask);
        if mask != [-1; 7] {
            return false;
        }
        rvv_slice::cmplt_f32(&a, &b, &mut mask);
        if mask != [-1; 7] {
            return false;
        }
        rvv_slice::cmple_f32(&a, &a, &mut mask);
        if mask != [-1; 7] {
            return false;
        }
        rvv_slice::cmpgt_f32(&b, &a, &mut mask);
        if mask != [-1; 7] {
            return false;
        }
        rvv_slice::cmpge_f32(&a, &a, &mut mask);
        if mask != [-1; 7] {
            return false;
        }

        let a = [1i32, 2, 3, 4, 5];
        let b = [10i32, 20, 30, 40, 50];
        let mut out = [0i32; 5];
        let mut mask = [0i32; 5];
        rvv_slice::add_i32(&a, &b, &mut out);
        if out != [11, 22, 33, 44, 55] {
            return false;
        }
        rvv_slice::mul_i32(&a, &b, &mut out);
        if out != [10, 40, 90, 160, 250] {
            return false;
        }
        rvv_slice::sub_i32(&b, &a, &mut out);
        if out != [9, 18, 27, 36, 45] {
            return false;
        }
        rvv_slice::div_i32(&b, &a, &mut out);
        if out != [10; 5] {
            return false;
        }
        rvv_slice::min_i32(&a, &b, &mut out);
        if out != a {
            return false;
        }
        rvv_slice::max_i32(&a, &b, &mut out);
        if out != b {
            return false;
        }
        rvv_slice::cmpeq_i32(&a, &a, &mut mask);
        if mask != [-1; 5] {
            return false;
        }
        rvv_slice::cmpne_i32(&a, &b, &mut mask);
        if mask != [-1; 5] {
            return false;
        }
        rvv_slice::cmplt_i32(&a, &b, &mut mask);
        if mask != [-1; 5] {
            return false;
        }
        rvv_slice::cmple_i32(&a, &a, &mut mask);
        if mask != [-1; 5] {
            return false;
        }
        rvv_slice::cmpgt_i32(&b, &a, &mut mask);
        if mask != [-1; 5] {
            return false;
        }
        rvv_slice::cmpge_i32(&a, &a, &mut mask);
        if mask != [-1; 5] {
            return false;
        }

        let source_f32 = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        let mut loaded_f32 = [0.0f32; 5];
        rvv_slice::load_f32(source_f32.as_ptr(), &mut loaded_f32);
        if loaded_f32 != source_f32 {
            return false;
        }
        let mut stored_f32 = [0.0f32; 5];
        rvv_slice::store_f32(stored_f32.as_mut_ptr(), &source_f32);
        if stored_f32 != source_f32 {
            return false;
        }
        let source_i32 = [1i32, 2, 3, 4, 5];
        let mut loaded_i32 = [0i32; 5];
        rvv_slice::load_i32(source_i32.as_ptr(), &mut loaded_i32);
        if loaded_i32 != source_i32 {
            return false;
        }
        let mut stored_i32 = [0i32; 5];
        rvv_slice::store_i32(stored_i32.as_mut_ptr(), &source_i32);
        stored_i32 == source_i32
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
            && fmin_h(0x7e00, one) == one
            && fmax_h(one, 0x7e00) == one
            && fmin_h(0x0000, 0x8000) == 0x8000
            && fmax_h(0x0000, 0x8000) == 0x0000
            && fminm_h(0x7e00, one) == 0x7e00
            && fmaxm_h(one, 0x7e00) == 0x7e00
            && feq_h(one, one)
            && flt_h(one, two)
            && fle_h(one, one)
            && {
                let mut bytes = [0u8; 2];
                fsh(bytes.as_mut_ptr(), two);
                flh(bytes.as_ptr()) == two
            }
            && fcvt_h_s(0x3fc0_0000) == 0x3e00
            && fcvt_s_h(0x3e00) == 0x3fc0_0000
            && fcvt_h_d(0x3ff8_0000_0000_0000) == 0x3e00
            && fcvt_d_h(0x3e00) == 0x3ff8_0000_0000_0000
            && fcvt_w_h(two) == 2
            && fcvt_wu_h(two) == 2
            && fcvt_l_h(two) == 2
            && fcvt_lu_h(two) == 2
            && fcvt_h_w(2) == two
            && fcvt_h_wu(2) == two
            && fcvt_h_l(2) == two
            && fcvt_h_lu(2) == two
            && fmv_x_h(0xffff) == -1
            && fmv_h_x(-1) == 0xffff
            && fminm_h(one, two) == one
            && fmaxm_h(one, two) == two
            && fround_h(0x3e00) == two
            && froundnx_h(0x3e00) == two
            && fleq_h(one, two)
            && fltq_h(one, two)
            && fcvtmod_w_h(0x3e00) == 1
            && fcvtmod_w_h(0x7c00) == 0
            && fcvtmod_w_h(0x7e00) == 0
            && fclass_h(0x0000) == 1 << 4
            && fclass_h(0x8000) == 1 << 3
            && fclass_h(0x7c00) == 1 << 7
            && fclass_h(0xfc00) == 1 << 0
            && fclass_h(0x7c01) == 1 << 8
            && fclass_h(0x7e00) == 1 << 9
            && fclass_h(0x3c00) == 1 << 6
            && fclass_h(0xbc00) == 1 << 1
            && fclass_h(0x0001) == 1 << 5
            && fclass_h(0x8001) == 1 << 2
            && fli_h(0) == 0xbc00
            && fli_h(1) == 0x0400
            && fli_h(2) == 0x0100
            && fli_h(16) == 0x3c00
            && fli_h(20) == 0x4000
            && fli_h(30) == 0x7c00
            && fli_h(31) == 0x7e00
    }
}

unsafe fn scalar_float_ops() -> bool {
    let zero_s = 0x0000_0000u32;
    let negative_zero_s = 0x8000_0000u32;
    let one_s = 0x3f80_0000u32;
    let one_half_s = 0x3fc0_0000u32;
    let two_s = 0x4000_0000u32;
    let three_s = 0x4040_0000u32;
    let four_s = 0x4080_0000u32;
    let canonical_nan_s = 0x7fc0_0000u32;
    let quiet_nan_s = 0x7fc0_0001u32;
    let zero_d = 0x0000_0000_0000_0000u64;
    let negative_zero_d = 0x8000_0000_0000_0000u64;
    let one_d = 0x3ff0_0000_0000_0000u64;
    let one_half_d = 0x3ff8_0000_0000_0000u64;
    let two_d = 0x4000_0000_0000_0000u64;
    let three_d = 0x4008_0000_0000_0000u64;
    let four_d = 0x4010_0000_0000_0000u64;
    let canonical_nan_d = 0x7ff8_0000_0000_0000u64;
    let quiet_nan_d = 0x7ff8_0000_0000_0001u64;
    unsafe {
        fadd_s(one_s, two_s) == three_s
            && fsub_s(two_s, one_s) == one_s
            && fmul_s(two_s, two_s) == four_s
            && fdiv_s(four_s, two_s) == two_s
            && fsqrt_s(four_s) == two_s
            && fmin_s(one_s, two_s) == one_s
            && fmax_s(one_s, two_s) == two_s
            && fmin_s(quiet_nan_s, one_s) == one_s
            && fmax_s(one_s, quiet_nan_s) == one_s
            && fmin_s(zero_s, negative_zero_s) == negative_zero_s
            && fmax_s(zero_s, negative_zero_s) == zero_s
            && fminm_s(one_s, two_s) == one_s
            && fmaxm_s(one_s, two_s) == two_s
            && fminm_s(quiet_nan_s, one_s) == canonical_nan_s
            && fmaxm_s(one_s, quiet_nan_s) == canonical_nan_s
            && feq_s(one_s, one_s)
            && !feq_s(one_s, two_s)
            && flt_s(one_s, two_s)
            && fle_s(one_s, one_s)
            && fleq_s(one_s, two_s)
            && fltq_s(one_s, two_s)
            && fround_s(one_half_s) == two_s
            && fround_s(0x4020_0000) == two_s
            && fround_s(0x4060_0000) == four_s
            && froundnx_s(one_half_s) == two_s
            && fadd_d(one_d, two_d) == three_d
            && fsub_d(two_d, one_d) == one_d
            && fmul_d(two_d, two_d) == four_d
            && fdiv_d(four_d, two_d) == two_d
            && fsqrt_d(four_d) == two_d
            && fmin_d(one_d, two_d) == one_d
            && fmax_d(one_d, two_d) == two_d
            && fmin_d(quiet_nan_d, one_d) == one_d
            && fmax_d(one_d, quiet_nan_d) == one_d
            && fmin_d(zero_d, negative_zero_d) == negative_zero_d
            && fmax_d(zero_d, negative_zero_d) == zero_d
            && fminm_d(one_d, two_d) == one_d
            && fmaxm_d(one_d, two_d) == two_d
            && fminm_d(quiet_nan_d, one_d) == canonical_nan_d
            && fmaxm_d(one_d, quiet_nan_d) == canonical_nan_d
            && feq_d(one_d, one_d)
            && !feq_d(one_d, two_d)
            && flt_d(one_d, two_d)
            && fle_d(one_d, one_d)
            && fleq_d(one_d, two_d)
            && fltq_d(one_d, two_d)
            && fround_d(one_half_d) == two_d
            && fround_d(0x4004_0000_0000_0000) == two_d
            && fround_d(0x400c_0000_0000_0000) == four_d
            && froundnx_d(one_half_d) == two_d
            && fcvt_s_d(one_half_d) == one_half_s
            && fcvt_d_s(one_half_s) == one_half_d
            && fcvt_w_s(two_s) == 2
            && fcvt_wu_s(two_s) == 2
            && fcvt_l_s(two_s) == 2
            && fcvt_lu_s(two_s) == 2
            && fcvt_s_w(2) == two_s
            && fcvt_s_wu(2) == two_s
            && fcvt_s_l(2) == two_s
            && fcvt_s_lu(2) == two_s
            && fcvt_w_d(two_d) == 2
            && fcvt_wu_d(two_d) == 2
            && fcvt_l_d(two_d) == 2
            && fcvt_lu_d(two_d) == 2
            && fcvt_d_w(2) == two_d
            && fcvt_d_wu(2) == two_d
            && fcvt_d_l(2) == two_d
            && fcvt_d_lu(2) == two_d
            && fcvtmod_w_d(one_half_d) == 1
            && fcvtmod_w_d(0xbff8_0000_0000_0000) == -1
            && fcvtmod_w_d(0x41ef_ffff_ffe0_0000) == -1
            && fcvtmod_w_d(0x41f0_0000_0010_0000) == 1
            && fcvtmod_w_d(canonical_nan_d) == 0
            && fmv_x_w(one_s) == 0x3f80_0000
            && fmv_x_w(0xffff_ffff) == -1
            && fmv_w_x(0x3f80_0000) == one_s
            && fmv_w_x(-1) == 0xffff_ffff
            && fmv_x_d(one_d) == 0x3ff0_0000_0000_0000
            && fmv_x_d(0xffff_ffff_ffff_ffff) == -1
            && fmv_d_x(0x3ff0_0000_0000_0000) == one_d
            && fmv_d_x(-1) == 0xffff_ffff_ffff_ffff
            && fmvh_x_d(one_d) == 0x3ff0_0000
            && fmvp_d_x(0, 0x3ff0_0000) == one_d
            && fli_s(0) == 0xbf80_0000
            && fli_s(1) == 0x0080_0000
            && fli_s(2) == 0x3780_0000
            && fli_s(16) == 0x3f80_0000
            && fli_s(20) == 0x4000_0000
            && fli_s(23) == 0x4080_0000
            && fli_s(30) == 0x7f80_0000
            && fli_s(31) == 0x7fc0_0000
            && fli_d(0) == 0xbff0_0000_0000_0000
            && fli_d(1) == 0x0010_0000_0000_0000
            && fli_d(2) == 0x3ef0_0000_0000_0000
            && fli_d(16) == 0x3ff0_0000_0000_0000
            && fli_d(20) == 0x4000_0000_0000_0000
            && fli_d(23) == 0x4010_0000_0000_0000
            && fli_d(30) == 0x7ff0_0000_0000_0000
            && fli_d(31) == 0x7ff8_0000_0000_0000
    }
}
