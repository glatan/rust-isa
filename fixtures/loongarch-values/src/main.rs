#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::loongarch::integer::bitmanip::{bitrev, bstrins, bstrpick, revb, revh, revw};
use rust_isa::loongarch::simd::lasx::xvabs_b;
use rust_isa::loongarch::simd::lasx::xvabs_d;
use rust_isa::loongarch::simd::lasx::xvabs_h;
use rust_isa::loongarch::simd::lasx::xvadd_w;
use rust_isa::loongarch::simd::lasx::xvmul_w;
use rust_isa::loongarch::simd::lasx::xvneg_b;
use rust_isa::loongarch::simd::lasx::xvneg_d;
use rust_isa::loongarch::simd::lasx::xvneg_h;
use rust_isa::loongarch::simd::lasx::xvsub_w;
use rust_isa::loongarch::simd::lasx::{
    V256, add as lasx_add, add_i64 as lasx_add_i64, and as lasx_and, fadd as lasx_fadd,
    fdiv as lasx_fdiv, fmul as lasx_fmul, fsub as lasx_fsub, mul as lasx_mul,
    mul_i32 as lasx_mul_i32, or as lasx_or, splat_i32 as lasx_splat_i32, sub as lasx_sub,
    sub_i64 as lasx_sub_i64, v256_load, v256_store, xor as lasx_xor, xvadd, xvand, xvmul, xvor,
    xvsub, xvxor,
};
use rust_isa::loongarch::simd::lasx::{
    xvabs, xvadd_b, xvadd_d, xvadd_h, xvmul_h, xvneg, xvsub_b, xvsub_d, xvsub_h,
};
use rust_isa::loongarch::simd::lsx::vabs_b;
use rust_isa::loongarch::simd::lsx::vabs_d;
use rust_isa::loongarch::simd::lsx::vabs_h;
use rust_isa::loongarch::simd::lsx::vadd_w;
use rust_isa::loongarch::simd::lsx::vbitsel;
use rust_isa::loongarch::simd::lsx::vmax;
use rust_isa::loongarch::simd::lsx::vmin;
use rust_isa::loongarch::simd::lsx::vmul_w;
use rust_isa::loongarch::simd::lsx::vneg_b;
use rust_isa::loongarch::simd::lsx::vneg_d;
use rust_isa::loongarch::simd::lsx::vneg_h;
use rust_isa::loongarch::simd::lsx::vneg_w;
use rust_isa::loongarch::simd::lsx::vnor;
use rust_isa::loongarch::simd::lsx::vpackev;
use rust_isa::loongarch::simd::lsx::vpackod;
use rust_isa::loongarch::simd::lsx::vseq;
use rust_isa::loongarch::simd::lsx::vseqi;
use rust_isa::loongarch::simd::lsx::vshuf;
use rust_isa::loongarch::simd::lsx::vshuf4i;
use rust_isa::loongarch::simd::lsx::vsle;
use rust_isa::loongarch::simd::lsx::vslei;
use rust_isa::loongarch::simd::lsx::vsll;
use rust_isa::loongarch::simd::lsx::vslli;
use rust_isa::loongarch::simd::lsx::vslt;
use rust_isa::loongarch::simd::lsx::vslti;
use rust_isa::loongarch::simd::lsx::vsra;
use rust_isa::loongarch::simd::lsx::vsrai;
use rust_isa::loongarch::simd::lsx::vsrl;
use rust_isa::loongarch::simd::lsx::vsrli;
use rust_isa::loongarch::simd::lsx::vsub_w;
use rust_isa::loongarch::simd::lsx::{
    V128, add, add_i8, and, cmpeq_i32, cmpgt_i32, fadd, fdiv, fmul, fsub, mul, mul_i16, or,
    splat_i32, sub_i16, sub_i32, v128_load, v128_store, vabs, vadd, vand, vmul, vneg, vor, vsub,
    vxor, xor,
};
use rust_isa::loongarch::simd::lsx::{add_i16, sub_i8, vadd_b, vadd_h, vmul_h, vsub_b, vsub_h};
use rust_isa::loongarch::simd::lsx::{add_i64, sub_i64, vadd_d, vsub_d};
use rust_isa::loongarch::simd::lsx::{
    vavgr, vavgr_d, vavgr_w, vext, vexth_h_b, vexth_hu_bu, vextl, vextl_qu_du, vfadd, vfadd_d,
    vfcmp, vfcmp_ceq_d, vfcmp_ceq_s, vfcmp_clt_s, vfcmp_cun_s, vfcvt_s_d, vfcvth_d_s, vfcvtl_d_s,
    vfdiv, vfdiv_d, vffint_d_l, vffint_s_l, vffint_s_w, vfmax, vfmax_d, vfmin, vfmin_d, vfmul,
    vfmul_d, vfsub, vfsub_d, vftint_w_d, vftint_w_s, vftintrz_w_s, vld, vldi, vrepl, vrepl_b,
    vrepl_d, vrepl_h, vrepl_w, vreplve, vreplve_b, vreplve_d, vreplve_h, vreplve_w, vsat, vsat_w,
    vsat_wu, vssr, vssrarn_w_d, vssrln_w_d, vst,
};
use rust_isa::loongarch::simd::lsx::{vexth, vfcmp_d, vfcvt, vffint, vftint, vssrarn_wu_du};

use rust_isa::loongarch::simd::lasx::{
    vext2xv_h_b, xvavgr, xvbitsel, xvexth, xvextl, xvfadd, xvfcmp, xvfcvt, xvfdiv, xvffint, xvfmax,
    xvfmin, xvfmul, xvfsub, xvftint, xvld, xvldi, xvmax, xvmin, xvnor, xvpackev, xvpackod, xvrepl,
    xvsat, xvseq, xvseqi, xvshuf, xvshuf4i, xvsle, xvslei, xvsll, xvslli, xvslt, xvslti, xvsra,
    xvsrai, xvsrl, xvsrli, xvssr, xvst,
};
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

#[inline(never)]
#[target_feature(enable = "lasx")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_xvadd(a: &V256, b: &V256, c: &V256) -> V256 {
    unsafe { xvadd(xvadd(*a, *b), *c) }
}

#[inline(never)]
#[target_feature(enable = "lasx")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_xvmul(a: &V256, b: &V256, c: &V256) -> V256 {
    unsafe { xvmul(xvmul(*a, *b), *c) }
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
    if !bitmanip_ops() {
        return 5;
    }
    if !lsx_width_ops() {
        return 6;
    }
    if !lasx_width_ops() {
        return 7;
    }
    if !lasx_alias_ops() {
        return 8;
    }
    if !lsx_abs_ops() {
        return 9;
    }
    if !lasx_abs_ops() {
        return 10;
    }
    if !lsx_div_ops() {
        return 11;
    }
    if !lasx_div_ops() {
        return 12;
    }
    if !lsx_alias_ops() {
        return 13;
    }
    if !alias_vadd_w() {
        return 14;
    }
    if !alias_vsub_w() {
        return 15;
    }
    if !alias_vmul_w() {
        return 16;
    }
    if !alias_vneg_w() {
        return 17;
    }
    if !alias_vabs_b() {
        return 18;
    }
    if !alias_vabs_h() {
        return 19;
    }
    if !alias_vabs_d() {
        return 20;
    }
    if !alias_vneg_b() {
        return 21;
    }
    if !alias_vneg_h() {
        return 22;
    }
    if !alias_vneg_d() {
        return 23;
    }
    if !alias_xvabs_b() {
        return 24;
    }
    if !alias_xvabs_h() {
        return 25;
    }
    if !alias_xvabs_d() {
        return 26;
    }
    if !alias_xvneg_b() {
        return 27;
    }
    if !alias_xvneg_h() {
        return 28;
    }
    if !alias_xvneg_d() {
        return 29;
    }
    if !alias_xvadd_w() {
        return 30;
    }
    if !alias_xvsub_w() {
        return 31;
    }
    if !alias_xvmul_w() {
        return 32;
    }
    if !alias_vmax() {
        return 33;
    }
    if !alias_vmin() {
        return 34;
    }
    if !alias_vseq() {
        return 35;
    }
    if !alias_vsle() {
        return 36;
    }
    if !alias_vslt() {
        return 37;
    }
    if !alias_vseqi() {
        return 38;
    }
    if !alias_vslei() {
        return 39;
    }
    if !alias_vslti() {
        return 40;
    }
    if !alias_vnor() {
        return 41;
    }
    if !alias_vbitsel() {
        return 42;
    }
    if !alias_vshuf() {
        return 43;
    }
    if !alias_vshuf4i() {
        return 44;
    }
    if !alias_vsll() {
        return 45;
    }
    if !alias_vsrl() {
        return 46;
    }
    if !alias_vsra() {
        return 47;
    }
    if !alias_vslli() {
        return 48;
    }
    if !alias_vsrli() {
        return 49;
    }
    if !alias_vsrai() {
        return 50;
    }
    if !alias_vpackev() {
        return 51;
    }
    if !alias_vpackod() {
        return 52;
    }
    if !lsx_extend_ops() {
        return 53;
    }
    if !lsx_vsat_ops() {
        return 54;
    }
    if !lsx_vssr_arith_ops() {
        return 55;
    }
    if !lsx_vssr_logical_ops() {
        return 56;
    }
    if !lsx_vssr_ops() {
        return 57;
    }
    if !lsx_float_ops() {
        return 58;
    }
    if !lsx_convert_ops() {
        return 59;
    }
    if !lsx_load_store_ops() {
        return 60;
    }
    if !lsx_vldi_ops() {
        return 61;
    }
    if !lsx_vrepl_ops() {
        return 62;
    }
    if !lsx_vreplve_ops() {
        return 63;
    }
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
    let a = V256::from_bytes([1; 32]);
    let b = V256::from_bytes([2; 32]);
    let c = V256::from_bytes([3; 32]);
    core::hint::black_box(unsafe { kernel_xvadd(&a, &b, &c) });
    core::hint::black_box(unsafe { kernel_xvmul(&a, &b, &c) });

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

fn lsx_more_ops() -> bool {
    let source = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let expected_high = {
        let mut bytes = [0u8; 16];
        for lane in 0..8 {
            let value = (source.to_bytes()[8 + lane] as i8 as i16).to_le_bytes();
            bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value);
        }
        bytes
    };
    if vexth(source).to_bytes() != expected_high {
        return false;
    }
    if f32_lanes(vffint(i32x4([1, 2, 3, 4]))) != [1.0, 2.0, 3.0, 4.0] {
        return false;
    }
    let converted = vfcvt(f64x2([1.0, 2.0]), f64x2([3.0, 4.0]));
    if f32_lanes(converted) != [3.0, 4.0, 1.0, 2.0] {
        return false;
    }
    if vfcmp_d::<4>(f64x2([1.0, 2.0]), f64x2([1.0, 2.0])).to_bytes() != [0xff; 16] {
        return false;
    }
    let a = V128::from_bytes({
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&1i64.to_le_bytes());
        bytes[8..].copy_from_slice(&3i64.to_le_bytes());
        bytes
    });
    let b = V128::from_bytes({
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&5i64.to_le_bytes());
        bytes[8..].copy_from_slice(&7i64.to_le_bytes());
        bytes
    });
    lanes(vssrarn_wu_du::<1>(a, b)) == [1, 2, 3, 4]
        && lanes(vftint(f32x4([1.9, -1.9, 2.1, -2.1]))) == [1, -1, 2, -2]
}

fn lasx_extra_ops() -> bool {
    let a = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i32x8([8, 7, 6, 5, 4, 3, 2, 1]);
    let ones = V256::from_bytes([0xff; 32]);
    let zeros = V256::from_bytes([0; 32]);
    unsafe {
        if i32x8_lanes(xvmax(a, b)) != [8, 7, 6, 5, 5, 6, 7, 8]
            || i32x8_lanes(xvmin(a, b)) != [1, 2, 3, 4, 4, 3, 2, 1]
        {
            return false;
        }
        if xvnor(V256::from_bytes([0xaa; 32]), V256::from_bytes([0xcc; 32])).to_bytes()
            != [0x11; 32]
            || xvavgr(V256::from_bytes([1; 32]), V256::from_bytes([3; 32])).to_bytes() != [2; 32]
        {
            return false;
        }
        if xvbitsel(a, b, ones) != a || xvbitsel(a, b, zeros) != b {
            return false;
        }
        if i32x8_lanes(xvfcmp::<4>(f32x8([1.0; 8]), f32x8([1.0; 8]))) != [-1; 8] {
            return false;
        }
        core::hint::black_box(xvfcvt(V256::from_bytes([0; 32]), V256::from_bytes([0; 32])));
        let bytes = V256::from_bytes(core::array::from_fn(|index| index as u8));
        let other_bytes = V256::from_bytes(core::array::from_fn(|index| index as u8 + 32));
        if xvpackev(bytes, other_bytes).to_bytes()
            != core::array::from_fn::<u8, 32, _>(|index| {
                if index % 2 == 0 {
                    index as u8
                } else {
                    (index - 1 + 32) as u8
                }
            })
            || xvpackod(bytes, other_bytes).to_bytes()
                != core::array::from_fn::<u8, 32, _>(|index| {
                    if index % 2 == 0 {
                        (index + 1) as u8
                    } else {
                        (index + 32) as u8
                    }
                })
        {
            return false;
        }
        if xvrepl(7) != i32x8([7; 8]) {
            return false;
        }
        if i32x8_lanes(xvsat::<4>(i32x8([-100, -16, 0, 15, 16, 100, -1, 1])))
            != [-16, -16, 0, 15, 15, 15, -1, 1]
        {
            return false;
        }
        if i32x8_lanes(xvseq(a, a)) != [-1; 8]
            || i32x8_lanes(xvseqi(a, 3)) != [0, 0, -1, 0, 0, 0, 0, 0]
            || i32x8_lanes(xvsle(a, b)) != [-1, -1, -1, -1, 0, 0, 0, 0]
            || i32x8_lanes(xvslt(a, b)) != [-1, -1, -1, 0, 0, 0, 0, 0]
            || i32x8_lanes(xvslei(a, 4)) != [-1, -1, -1, -1, 0, 0, 0, 0]
            || i32x8_lanes(xvslti(a, 4)) != [-1, -1, -1, 0, 0, 0, 0, 0]
        {
            return false;
        }
        if i32x8_lanes(xvslli::<1>(a)) != [2, 4, 6, 8, 10, 12, 14, 16]
            || i32x8_lanes(xvsrli::<1>(i32x8([2, 4, 6, 8, 10, 12, 14, 16])))
                != [1, 2, 3, 4, 5, 6, 7, 8]
            || i32x8_lanes(xvsrai::<1>(i32x8([-2, -4, -6, -8, 2, 4, 6, 8])))
                != [-1, -2, -3, -4, 1, 2, 3, 4]
            || i32x8_lanes(xvsll(a, 1)) != [2, 4, 6, 8, 10, 12, 14, 16]
            || i32x8_lanes(xvsrl(i32x8([2, 4, 6, 8, 10, 12, 14, 16]), 1))
                != [1, 2, 3, 4, 5, 6, 7, 8]
            || i32x8_lanes(xvsra(i32x8([-2, -4, -6, -8, 2, 4, 6, 8]), 1))
                != [-1, -2, -3, -4, 1, 2, 3, 4]
        {
            return false;
        }
        let source = V256::from_bytes(core::array::from_fn(|index| index as u8));
        let other = V256::from_bytes(core::array::from_fn(|index| index as u8 + 32));
        let control = V256::from_bytes(core::array::from_fn(|index| index as u8 + 32));
        if xvshuf(source, other, control).to_bytes() != other.to_bytes() {
            return false;
        }
        let reversed = xvshuf4i::<0b0001_1011>(source).to_bytes();
        if reversed
            != core::array::from_fn::<u8, 32, _>(|index| {
                let group = index / 4;
                let lane = 3 - index % 4;
                (group * 4 + lane) as u8
            })
        {
            return false;
        }
        let mut output = [0u8; 32];
        xvst(output.as_mut_ptr(), 0, xvld(source.as_ptr(), 0));
        if output != source.to_bytes() || xvldi::<0>().to_bytes() != [0; 32] {
            return false;
        }
        let fa = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let fb = f32x8([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
        if f32x8_lanes(xvfadd(fa, fb)) != [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0, 88.0]
            || f32x8_lanes(xvfsub(fb, fa)) != [9.0, 18.0, 27.0, 36.0, 45.0, 54.0, 63.0, 72.0]
            || f32x8_lanes(xvfmul(fa, fb)) != [10.0, 40.0, 90.0, 160.0, 250.0, 360.0, 490.0, 640.0]
            || f32x8_lanes(xvfdiv(fb, fa)) != [10.0; 8]
            || f32x8_lanes(xvfmax(fa, fb)) != [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]
            || f32x8_lanes(xvfmin(fa, fb)) != [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
        {
            return false;
        }
        if f32x8_lanes(xvffint(a)) != [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
            || i32x8_lanes(xvftint(f32x8([1.9, -1.9, 2.1, -2.1, 3.9, -3.9, 4.1, -4.1])))
                != [1, -1, 2, -2, 3, -3, 4, -4]
        {
            return false;
        }
        let widened = vext2xv_h_b(V256::from_bytes(core::array::from_fn(|index| {
            index as u8 + 1
        })));
        widened.to_bytes()
            == core::array::from_fn::<u8, 32, _>(|index| {
                if index % 2 == 0 {
                    index as u8 / 2 + 1
                } else {
                    0
                }
            })
    }
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
    unsafe { xvadd_d(a, b) == lasx_add_i64(a, b) && xvsub_d(b, a) == lasx_sub_i64(b, a) }
}

fn lsx_div_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|i| i as u8));
    let b = V128::from_bytes(core::array::from_fn(|i| (i + 1) as u8));
    vadd_d(a, b) == add_i64(a, b) && vsub_d(b, a) == sub_i64(b, a)
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

fn bitmanip_ops() -> bool {
    revb(0x0102_0304_0506_0708) == 0x0807_0605_0403_0201
        && revh(0x0102_0304_0506_0708) == 0x0708_0506_0304_0102
        && revw(0x0102_0304_0506_0708) == 0x0506_0708_0102_0304
        && bitrev(1) == 1u64 << 63
        && bstrpick::<7, 4>(0xab) == 0xa
        && bstrins::<7, 4>(0, 0xa) == 0xa0
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

fn alias_vadd_w() -> bool {
    vadd_w(i32x4([1, 2, 3, 4]), i32x4([5, 6, 7, 8]))
        == add(i32x4([1, 2, 3, 4]), i32x4([5, 6, 7, 8]))
}

fn alias_vsub_w() -> bool {
    vsub_w(i32x4([5, 6, 7, 8]), i32x4([1, 2, 3, 4]))
        == sub_i32(i32x4([5, 6, 7, 8]), i32x4([1, 2, 3, 4]))
}

fn alias_vmul_w() -> bool {
    vmul_w(i32x4([1, 2, 3, 4]), i32x4([5, 6, 7, 8]))
        == mul(i32x4([1, 2, 3, 4]), i32x4([5, 6, 7, 8]))
}

fn alias_vneg_w() -> bool {
    vneg_w(i32x4([1, -2, 3, -4])) == vneg(i32x4([1, -2, 3, -4]))
}

fn alias_vabs_b() -> bool {
    vabs_b(V128::from_bytes([0xff; 16])) == V128::from_bytes([1; 16])
}

fn alias_vabs_h() -> bool {
    i16_lanes(vabs_h(i16x8([-1; 8]))) == [1; 8]
}

fn alias_vabs_d() -> bool {
    vabs_d(V128::from_bytes([0xff; 16])).to_bytes() == {
        let mut b = [0u8; 16];
        b[0] = 1;
        b[8] = 1;
        b
    }
}

fn alias_vneg_b() -> bool {
    vneg_b(V128::from_bytes([1; 16])).to_bytes() == [0xff; 16]
}

fn alias_vneg_h() -> bool {
    i16_lanes(vneg_h(i16x8([1; 8]))) == [-1; 8]
}

fn alias_vneg_d() -> bool {
    {
        let mut x = [0u8; 16];
        x[0] = 1;
        let mut e = [0u8; 16];
        for i in 0..8 {
            e[i] = 0xff;
        }
        vneg_d(V128::from_bytes(x)).to_bytes() == e
    }
}

fn alias_xvabs_b() -> bool {
    unsafe { xvabs_b(V256::from_bytes([0xff; 32])).to_bytes() == [1; 32] }
}

fn alias_xvabs_h() -> bool {
    unsafe {
        xvabs_h(V256::from_bytes([0xff; 32])).to_bytes()
            == V256::from_bytes(core::array::from_fn(|i| (i % 2 == 0) as u8)).to_bytes()
    }
}

fn alias_xvabs_d() -> bool {
    unsafe {
        xvabs_d(V256::from_bytes([0xff; 32])).to_bytes()
            == V256::from_bytes(core::array::from_fn(|i| (i % 8 == 0) as u8)).to_bytes()
    }
}

fn alias_xvneg_b() -> bool {
    unsafe { xvneg_b(V256::from_bytes([1; 32])).to_bytes() == [0xff; 32] }
}

fn alias_xvneg_h() -> bool {
    unsafe {
        xvneg_h(V256::from_bytes([1; 32])).to_bytes()
            == V256::from_bytes(core::array::from_fn(
                |i| if i % 2 == 0 { 0xff } else { 0xfe },
            ))
            .to_bytes()
    }
}

fn alias_xvneg_d() -> bool {
    unsafe {
        {
            let mut x = [0u8; 32];
            x[0] = 1;
            let mut e = [0u8; 32];
            for i in 0..8 {
                e[i] = 0xff;
            }
            xvneg_d(V256::from_bytes(x)).to_bytes() == e
        }
    }
}

fn alias_xvadd_w() -> bool {
    unsafe {
        xvadd_w(V256::from_bytes([1; 32]), V256::from_bytes([2; 32]))
            == lasx_add(V256::from_bytes([1; 32]), V256::from_bytes([2; 32]))
    }
}

fn alias_xvsub_w() -> bool {
    unsafe {
        xvsub_w(V256::from_bytes([3; 32]), V256::from_bytes([2; 32]))
            == lasx_sub(V256::from_bytes([3; 32]), V256::from_bytes([2; 32]))
    }
}

fn alias_xvmul_w() -> bool {
    unsafe {
        xvmul_w(V256::from_bytes([1; 32]), V256::from_bytes([2; 32]))
            == lasx_mul(V256::from_bytes([1; 32]), V256::from_bytes([2; 32]))
    }
}

fn alias_vmax() -> bool {
    lanes(vmax(i32x4([1, 4, 2, 8]), i32x4([3, 2, 6, 4]))) == [3, 4, 6, 8]
}

fn alias_vmin() -> bool {
    lanes(vmin(i32x4([1, 4, 2, 8]), i32x4([3, 2, 6, 4]))) == [1, 2, 2, 4]
}

fn alias_vseq() -> bool {
    vseq(i32x4([1, 2, 3, 4]), i32x4([1, 0, 3, 4])).to_bytes()
        == [
            0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]
}

fn alias_vsle() -> bool {
    vsle(i32x4([1, 4, 2, 8]), i32x4([3, 2, 6, 4])).to_bytes()
        == [
            0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
        ]
}

fn alias_vslt() -> bool {
    vslt(i32x4([1, 4, 2, 8]), i32x4([3, 2, 6, 4])).to_bytes()
        == [
            0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
        ]
}

fn alias_vseqi() -> bool {
    vseqi(i32x4([1, 2, 3, 4]), 2).to_bytes()
        == [0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]
}

fn alias_vslei() -> bool {
    vslei(i32x4([1, 2, 3, 4]), 2).to_bytes()
        == [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
}

fn alias_vslti() -> bool {
    vslti(i32x4([1, 2, 3, 4]), 2).to_bytes()
        == [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}

fn alias_vnor() -> bool {
    vnor(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes() == [0x11; 16]
}

fn alias_vbitsel() -> bool {
    vbitsel(
        V128::from_bytes([1; 16]),
        V128::from_bytes([2; 16]),
        V128::from_bytes([0xff; 16]),
    )
    .to_bytes()
        == [1; 16]
}

fn alias_vshuf() -> bool {
    vshuf(
        V128::from_bytes([1; 16]),
        V128::from_bytes([2; 16]),
        V128::from_bytes([0; 16]),
    )
    .to_bytes()
        == [1; 16]
}

fn alias_vshuf4i() -> bool {
    vshuf4i::<0>(V128::from_bytes(core::array::from_fn(|i| i as u8))).to_bytes()
        == [0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12]
}

fn alias_vsll() -> bool {
    lanes(vsll(i32x4([1, 2, 3, 4]), 1)) == [2, 4, 6, 8]
}

fn alias_vsrl() -> bool {
    lanes(vsrl(i32x4([2, 4, 6, 8]), 1)) == [1, 2, 3, 4]
}

fn alias_vsra() -> bool {
    lanes(vsra(i32x4([-2, -4, 4, 8]), 1)) == [-1, -2, 2, 4]
}

fn alias_vslli() -> bool {
    lanes(vslli::<1>(i32x4([1, 2, 3, 4]))) == [2, 4, 6, 8]
}

fn alias_vsrli() -> bool {
    lanes(vsrli::<1>(i32x4([2, 4, 6, 8]))) == [1, 2, 3, 4]
}

fn alias_vsrai() -> bool {
    lanes(vsrai::<1>(i32x4([-2, -4, 4, 8]))) == [-1, -2, 2, 4]
}

fn alias_vpackev() -> bool {
    vpackev(
        V128::from_bytes(core::array::from_fn(|i| i as u8)),
        V128::from_bytes(core::array::from_fn(|i| (i + 16) as u8)),
    )
    .to_bytes()
        == [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30]
}

fn alias_vpackod() -> bool {
    vpackod(
        V128::from_bytes(core::array::from_fn(|i| i as u8)),
        V128::from_bytes(core::array::from_fn(|i| (i + 16) as u8)),
    )
    .to_bytes()
        == [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31]
}

fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i64_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn lsx_extend_ops() -> bool {
    let source = i8x16([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    if i16_lanes(vext(source)) != [0, 1, 2, 3, 4, 5, 6, 7]
        || i16_lanes(vexth_h_b(source)) != [8, 9, 10, 11, 12, 13, 14, 15]
        || i16_lanes(vexth_hu_bu(i8x16([-1; 16]))) != [255; 8]
        || i64_lanes(vextl(i64x2([-2, 5]))) != [-2, -1]
        || i64_lanes(vextl_qu_du(i64x2([-2, 5]))) != [-2, 0]
    {
        return false;
    }
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V128::from_bytes(core::array::from_fn(|index| (15 - index) as u8));
    if vavgr(a, b).to_bytes() != [8; 16]
        || vavgr(V128::from_bytes([255; 16]), V128::from_bytes([255; 16])).to_bytes() != [255; 16]
    {
        return false;
    }
    lanes(vavgr_w(i32x4([-1, -2, 5, 6]), i32x4([-2, 2, 5, 7]))) == [-1, 0, 5, 7]
        && i64_lanes(vavgr_d(i64x2([1, 2]), i64x2([2, 4]))) == [2, 3]
}

fn lsx_vsat_ops() -> bool {
    if lanes(vsat::<7>(i32x4([-200, -128, -1, 0]))) != [-128, -128, -1, 0] {
        return false;
    }
    if lanes(vsat::<7>(i32x4([1, 127, 128, 200]))) != [1, 127, 127, 127] {
        return false;
    }
    if lanes(vsat_w::<4>(i32x4([100, -100, 7, -8]))) != [15, -16, 7, -8] {
        return false;
    }
    lanes(vsat_wu::<3>(i32x4([-1, 0, 16, 7]))) == [15, 0, 15, 7]
}

fn lsx_vssr_ops() -> bool {
    let expected = {
        let mut out = [0u8; 16];
        out[0] = 255;
        out[1] = 255;
        out[2] = 16;
        out[3] = 255;
        out
    };
    vssr(
        i16x8([256, -1, 256, -1, 0, 0, 0, 0]),
        i16x8([0, 0, 4, 8, 0, 0, 0, 0]),
    )
    .to_bytes()
        == expected
        && vssr(V128::from_bytes([0; 16]), V128::from_bytes([0; 16])).to_bytes() == [0; 16]
}

fn lsx_vssr_arith_ops() -> bool {
    lanes(vssrarn_w_d::<1>(
        i64x2([-9, 10]),
        i64x2([i64::MAX, i64::MIN]),
    )) == [-4, 5, i32::MAX, i32::MIN]
}

fn lsx_vssr_logical_ops() -> bool {
    lanes(vssrln_w_d::<1>(
        i64x2([10, -1]),
        i64x2([u32::MAX as i64, 3]),
    )) == [5, 2_147_483_647, 2_147_483_647, 1]
}

fn lsx_float_ops() -> bool {
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    if f32_lanes(vfadd(a, b)) != [11.0, 22.0, 33.0, 44.0]
        || f32_lanes(vfsub(b, a)) != [9.0, 18.0, 27.0, 36.0]
        || f32_lanes(vfmul(a, b)) != [10.0, 40.0, 90.0, 160.0]
        || f32_lanes(vfdiv(b, a)) != [10.0, 10.0, 10.0, 10.0]
        || f32_lanes(vfmax(a, f32x4([2.0, 1.0, 4.0, 3.0]))) != [2.0, 2.0, 4.0, 4.0]
        || f32_lanes(vfmin(a, f32x4([2.0, 1.0, 4.0, 3.0]))) != [1.0, 1.0, 3.0, 3.0]
    {
        return false;
    }
    let da = f64x2([1.0, 2.0]);
    let db = f64x2([10.0, 20.0]);
    if f64_lanes(vfadd_d(da, db)) != [11.0, 22.0]
        || f64_lanes(vfsub_d(db, da)) != [9.0, 18.0]
        || f64_lanes(vfmul_d(da, db)) != [10.0, 40.0]
        || f64_lanes(vfdiv_d(db, da)) != [10.0, 10.0]
        || f64_lanes(vfmax_d(da, f64x2([2.0, 1.0]))) != [2.0, 2.0]
        || f64_lanes(vfmin_d(da, f64x2([2.0, 1.0]))) != [1.0, 1.0]
    {
        return false;
    }
    let left = f32x4([1.0, 2.0, f32::NAN, 4.0]);
    let right = f32x4([2.0, 2.0, 3.0, 4.0]);
    if lanes(vfcmp::<4>(left, right)) != [0, -1, 0, -1]
        || lanes(vfcmp::<2>(left, right)) != [-1, 0, 0, 0]
        || lanes(vfcmp::<6>(left, right)) != [-1, -1, 0, -1]
        || lanes(vfcmp::<16>(left, right)) != [-1, 0, 0, 0]
        || lanes(vfcmp::<24>(left, right)) != [-1, 0, -1, 0]
        || lanes(vfcmp::<0>(left, right)) != [0; 4]
        || lanes(vfcmp::<20>(left, right)) != [-1, -1, 0, -1]
    {
        return false;
    }
    if lanes(vfcmp_ceq_s(a, f32x4([1.0, 0.0, 3.0, 4.0]))) != [-1, 0, -1, -1]
        || lanes(vfcmp_clt_s(a, f32x4([3.0, 2.0, 6.0, 4.0]))) != [-1, 0, -1, 0]
        || lanes(vfcmp_cun_s(
            f32x4([f32::NAN, 1.0, 2.0, 3.0]),
            f32x4([0.0, 1.0, 2.0, 3.0]),
        )) != [-1, 0, 0, 0]
    {
        return false;
    }
    if vfcmp_ceq_d(da, f64x2([1.0, 0.0])).to_bytes()
        != [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    {
        return false;
    }
    let nan_a = f32x4([1.0, 5.0, -2.0, f32::NAN]);
    let nan_b = f32x4([4.0, 2.0, -2.0, 6.0]);
    f32_lanes(vfmax(nan_a, nan_b)) == [4.0, 5.0, -2.0, 6.0]
        && f32_lanes(vfmin(nan_a, nan_b)) == [1.0, 2.0, -2.0, 6.0]
}

fn lsx_convert_ops() -> bool {
    if f32_lanes(vfcvt_s_d(f64x2([1.0, 2.0]), f64x2([3.0, 4.0]))) != [3.0, 4.0, 1.0, 2.0] {
        return false;
    }
    if f64_lanes(vfcvtl_d_s(f32x4([1.5, 2.5, 3.5, 4.5]))) != [1.5, 2.5]
        || f64_lanes(vfcvth_d_s(f32x4([1.5, 2.5, 3.5, 4.5]))) != [3.5, 4.5]
    {
        return false;
    }
    if lanes(vftint_w_s(f32x4([1.0, -2.0, 3.0, -4.0]))) != [1, -2, 3, -4]
        || lanes(vftintrz_w_s(f32x4([1.9, -2.9, 3.4, -4.4]))) != [1, -2, 3, -4]
        || lanes(vftint_w_d(f64x2([1.0, -2.0]), f64x2([3.0, -4.0]))) != [3, -4, 1, -2]
    {
        return false;
    }
    if f32_lanes(vffint_s_w(i32x4([1, -2, 3, -4]))) != [1.0, -2.0, 3.0, -4.0]
        || f64_lanes(vffint_d_l(i64x2([1, -2]))) != [1.0, -2.0]
        || f32_lanes(vffint_s_l(i64x2([1, 2]), i64x2([3, 4]))) != [3.0, 4.0, 1.0, 2.0]
    {
        return false;
    }
    true
}

fn lsx_load_store_ops() -> bool {
    let mut buffer: [u8; 64] = core::array::from_fn(|index| (index * 3) as u8);
    let mut expected = [0u8; 16];
    expected.copy_from_slice(&buffer[9..25]);
    let loaded = unsafe { vld(buffer.as_ptr(), 9) };
    if loaded.to_bytes() != expected {
        return false;
    }
    let negative = unsafe { vld(buffer.as_ptr().add(41), -32) };
    if negative.to_bytes() != expected {
        return false;
    }
    unsafe { vst(buffer.as_mut_ptr(), 5, loaded) };
    let mut stored = [0u8; 16];
    stored.copy_from_slice(&buffer[5..21]);
    stored == expected
}

fn lsx_vldi_ops() -> bool {
    let mut expand = [0u8; 16];
    for index in 0..16 {
        expand[index] = if index % 2 == 0 { 0x00 } else { 0xff };
    }
    let mut mode4 = [0u8; 16];
    for index in 0..16 {
        mode4[index] = if index % 2 == 0 { 0x12 } else { 0x00 };
    }
    let mut mode0 = [0u8; 16];
    for index in 0..16 {
        if index % 8 == 0 || index % 8 == 4 {
            mode0[index] = 0x12;
        }
    }
    vldi::<0x3ff>().to_bytes() == [0xff; 16]
        && vldi::<0xbfe>().to_bytes() == i32x4([-2; 4]).to_bytes()
        && vldi::<0x523>().to_bytes() == i16x8([291; 8]).to_bytes()
        && i64_lanes(vldi::<0xffd>()) == [-3, -3]
        && vldi::<0x185a>().to_bytes() == [0x5a; 16]
        && vldi::<0x19aa>().to_bytes() == expand
        && vldi::<0x1412>().to_bytes() == mode4
        && vldi::<0x1012>().to_bytes() == mode0
}

fn lsx_vrepl_ops() -> bool {
    lanes(vrepl(7)) == [7; 4]
        && vrepl_b(0x1234).to_bytes() == [0x34; 16]
        && vrepl_h(-1).to_bytes() == [0xff; 16]
        && lanes(vrepl_w(-2)) == [-2; 4]
        && i64_lanes(vrepl_d(-2)) == [-2, -2]
}

fn lsx_vreplve_ops() -> bool {
    let source = i8x16([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    i8_lanes(vreplve_b::<3>(source)) == [3; 16]
        && i16_lanes(vreplve_h::<2>(i16x8([10, 11, 12, 13, 14, 15, 16, 17]))) == [12; 8]
        && lanes(vreplve_w::<1>(i32x4([10, 20, 30, 40]))) == [20; 4]
        && i64_lanes(vreplve_d::<1>(i64x2([10, 20]))) == [20, 20]
        && lanes(vreplve::<2>(i32x4([10, 20, 30, 40]))) == [30; 4]
}
