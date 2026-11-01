#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::riscv::crypto::zkn::{aes64es, aes64ks1i, sha256sum0};
use rust_isa::riscv::crypto::zks::{sm3p0, sm3p1, sm4ed, sm4ks};
use rust_isa::riscv::float::scalar::{
    fadd_d, fadd_h, fadd_s, fclass_h, fcvt_d_h, fcvt_d_l, fcvt_d_lu, fcvt_d_s, fcvt_d_w, fcvt_d_wu,
    fcvt_h_d, fcvt_h_l, fcvt_h_lu, fcvt_h_s, fcvt_h_w, fcvt_h_wu, fcvt_l_d, fcvt_l_h, fcvt_l_s,
    fcvt_lu_d, fcvt_lu_h, fcvt_lu_s, fcvt_s_d, fcvt_s_h, fcvt_s_l, fcvt_s_lu, fcvt_s_w, fcvt_s_wu,
    fcvt_w_d, fcvt_w_h, fcvt_w_s, fcvt_wu_d, fcvt_wu_h, fcvt_wu_s, fdiv_d, fdiv_h, fdiv_s, feq_d,
    feq_h, feq_s, fle_d, fle_h, fle_s, flh, flt_d, flt_h, flt_s, fmax_d, fmax_h, fmax_s, fmin_d,
    fmin_h, fmin_s, fmul_d, fmul_h, fmul_s, fmv_d_x, fmv_h_x, fmv_w_x, fmv_x_d, fmv_x_h, fmv_x_w,
    fsh, fsqrt_d, fsqrt_h, fsqrt_s, fsub_d, fsub_h, fsub_s,
};
use rust_isa::riscv::float::zfa::{
    fcvtmod_w_d, fcvtmod_w_h, fleq_d, fleq_h, fleq_s, fli_d, fli_h, fli_s, fltq_d, fltq_h, fltq_s,
    fmaxm_d, fmaxm_h, fmaxm_s, fminm_d, fminm_h, fminm_s, fmvh_x_d, fmvp_d_x, fround_d, fround_h,
    fround_s, froundnx_d, froundnx_h, froundnx_s,
};
use rust_isa::riscv::integer::zba::{sh1add, sh2add, sh3add};
use rust_isa::riscv::integer::zbb::{
    andn, clz, cpop, max, maxu, min, minu, orc_b, rol, sext_b, xnor,
};
use rust_isa::riscv::integer::zbc::{clmul, clmulh, clmulr};
use rust_isa::riscv::integer::zbkb::{brev8, pack, packh, unzip, zip};
use rust_isa::riscv::integer::zbkx::{xperm4, xperm8};
use rust_isa::riscv::integer::zbs::{bclr, bext, binv, bset};
use rust_isa::riscv::simd::rvv::{
    V128, add, add_i8, add_i16, and, cmpeq_i32, cmpgt_i32, fadd, fmul, mul, mul_i16, or, splat_i32,
    sub_i32, v128_load, v128_store, vaadd, vaaddu, vadc_vim, vadc_vvm, vadc_vxm, vadd, vand, vasub,
    vasubu, vcompress, vcpop, vdiv, vdivu, vfabs, vfadd, vfclass, vfcvt_f_x, vfcvt_f_xu,
    vfcvt_rtz_x_f, vfcvt_rtz_xu_f, vfcvt_x_f, vfcvt_xu_f, vfdiv, vfirst, vfma, vfmacc_vf,
    vfmacc_vv, vfmadd_vf, vfmadd_vv, vfmax, vfmerge, vfmin, vfms, vfmsac_vf, vfmsac_vv, vfmsub_vf,
    vfmsub_vv, vfmul, vfmv, vfmv_f_s, vfncvt_f_f_w, vfncvt_f_x_w, vfncvt_f_xu_w, vfncvt_rod_f_f_w,
    vfncvt_rtz_x_f_w, vfncvt_rtz_xu_f_w, vfncvt_x_f_w, vfncvt_xu_f_w, vfneg, vfnma, vfnmacc_vf,
    vfnmacc_vv, vfnmadd_vf, vfnmadd_vv, vfnms, vfnmsac_vf, vfnmsac_vv, vfnmsub_vf, vfnmsub_vv,
    vfrdiv, vfrec7, vfredmax, vfredmin, vfredosum, vfredsum, vfredusum, vfrsqrt7, vfrsub_vf,
    vfsgnj, vfsgnjn, vfsgnjx, vfslide1down, vfslide1up, vfsqrt, vfsub, vfwadd, vfwcvt_f_f_v,
    vfwcvt_f_x_v, vfwcvt_f_xu_v, vfwcvt_rtz_x_f_v, vfwcvt_rtz_xu_f_v, vfwcvt_x_f_v, vfwcvt_xu_f_v,
    vfwmacc, vfwmsac, vfwmul, vfwnmacc, vfwnmsac, vfwredosum_vs, vfwredusum_vs, vfwsub, vid, viota,
    vl1re8, vl1re16, vl1re32, vl1re64, vl2re8, vl2re16, vl2re32, vl2re64, vl4re8, vl4re16, vl4re32,
    vl4re64, vl8re8, vl8re16, vl8re32, vl8re64, vle8, vle8ff, vle16, vle16ff, vle32, vle32ff,
    vle64, vle64ff, vlm, vloxei8, vloxei16, vloxei32, vloxei64, vlse32, vlseg2, vlseg3, vlseg4,
    vlseg5, vlseg6, vlseg7, vlseg8, vlsseg2, vlsseg3, vlsseg4, vlsseg5, vlsseg6, vlsseg7, vluxei32,
    vmacc, vmadc_vi, vmadc_vim, vmadc_vv, vmadc_vvm, vmadc_vx, vmadc_vxm, vmadd, vmand, vmandn,
    vmax, vmaxu, vmerge, vmfeq_vf, vmfeq_vv, vmfge_vf, vmfge_vv, vmfgt_vf, vmfgt_vv, vmfle_vf,
    vmfle_vv, vmflt_vf, vmflt_vv, vmfne_vf, vmfne_vv, vmin, vminu, vmnand, vmnor, vmor, vmorn,
    vmsbc_vv, vmsbc_vvm, vmsbc_vx, vmsbc_vxm, vmsbf, vmseq, vmseq_mask, vmsgeu_mask, vmsgt,
    vmsgt_mask, vmsif, vmslt, vmsne_mask, vmsof, vmul, vmulh_vv, vmulh_vx, vmulhsu_vv, vmulhsu_vx,
    vmulhu_vv, vmulhu_vx, vmv, vmv_s_x, vmv_v_x, vmv_x_s, vmv1r, vmv2r, vmv4r, vmv8r, vmxnor,
    vmxor, vnclip, vnclipu, vnmsac, vnmsub, vnsra, vnsrl, vor, vredand, vredmax, vredmaxu, vredmin,
    vredminu, vredor, vredsum, vredxor, vrem, vremu, vrgather, vrgatherei16, vrsub_vi, vrsub_vx,
    vs1r, vs2r, vs4r, vs8r, vsadd, vsaddu, vsbc_vvm, vsbc_vxm, vse8, vse32, vse64, vsetivli,
    vsetvl, vsetvli, vsext, vslide1down, vslide1up, vslidedown, vslideup, vsll, vsm, vsmul_vv,
    vsmul_vx, vsoxei32, vsra, vsrl, vsse32, vsseg2, vsseg8, vssra_vi, vssra_vv, vssra_vx, vssrl_vi,
    vssrl_vv, vssrl_vx, vssseg2, vssub, vssubu, vsub, vsuxei32, vwadd, vwaddu, vwmacc, vwmaccsu,
    vwmaccu, vwmaccus, vwmul, vwmulsu, vwmulu, vwredsum, vwredsumu, vwsub, vwsubu, vxor, vzext,
    xor,
};
use rust_isa::riscv::simd::rvv_slice;
use rust_isa::riscv::simd::zv;
use rust_isa::riscv::simd::zv::{
    vaesdf, vaesdm, vaesef, vaesem, vaeskf1, vaeskf2, vaesz, vclmul, vclmulh, vghsh, vgmul,
    vsha2ch, vsha2cl, vsha2ms, vsm3c, vsm3me, vsm4k, vsm4r,
};

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

unsafe fn zvbb_ops() -> bool {
    let v = V128::from_bytes([0; 16]);
    let _ = zv::vandn_vv(v, v);
    let _ = zv::vandn_vx(v, 1);
    let _ = zv::vbrev(v);
    let _ = zv::vbrev8(v);
    let _ = zv::vrev8(v);
    let _ = zv::vclz(v);
    let _ = zv::vctz(v);
    let _ = zv::vcpop(v);
    let _ = zv::vrol_vv(v, v);
    let _ = zv::vrol_vx(v, 1);
    let _ = zv::vror_vv(v, v);
    let _ = zv::vror_vx(v, 1);
    let _ = zv::vror_vi::<1>(v);
    let _ = zv::vwsll_vv(v, v);
    let _ = zv::vwsll_vx(v, 1);
    let _ = zv::vwsll_vi::<1>(v);
    true
}

fn vxrm_ops() -> bool {
    let before: usize;
    let after: usize;
    unsafe {
        asm!("csrr {}, vxrm", out(reg) before, options(nomem, nostack));
        asm!("csrwi vxrm, 1", options(nomem, nostack));
        let value = vssrl_vi::<1>(i32x4([16, 32, 64, 128]));
        asm!("csrr {}, vxrm", out(reg) after, options(nomem, nostack));
        asm!("csrw vxrm, {}", in(reg) before, options(nomem, nostack));
        lanes(value) == [8, 16, 32, 64] && after == 1
    }
}

fn run() -> u32 {
    if !load_store() {
        return 1;
    }
    let ff_data = [0u8; 16];
    let (_, vl8) = unsafe { vle8ff(ff_data.as_ptr()) };
    let (_, vl16) = unsafe { vle16ff(ff_data.as_ptr().cast()) };
    let (_, vl32) = unsafe { vle32ff(ff_data.as_ptr().cast()) };
    let (_, vl64) = unsafe { vle64ff(ff_data.as_ptr().cast()) };
    if vl8 != 16 || vl16 != 8 || vl32 != 4 || vl64 != 2 {
        return 14;
    }
    if !integer() {
        return 2;
    }
    if !vector_ops() {
        return 3;
    }
    if !integer_fixed_ops() {
        return 9;
    }
    if !mask_memory_ops() {
        return 10;
    }
    if !float_crypto_ops() {
        return 11;
    }
    if !unsafe { zvbb_ops() } {
        return 12;
    }
    if !scalar_integer_ops() {
        return 4;
    }
    if !slice_ops() {
        return 5;
    }
    if unsafe { vsetivli::<4, 0>() } != 4
        || unsafe { vsetvli::<0>(4) } != 4
        || unsafe { vsetvl(4, 0) } != 4
    {
        return 13;
    }
    if !crypto_ops() {
        return 6;
    }
    if !unsafe { fp16_zfa_ops() } {
        return 7;
    }
    if !unsafe { scalar_float_ops() } {
        return 8;
    }
    if !vxrm_ops() {
        return 15;
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

fn u16_lanes(value: V128) -> [u16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
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
            || f32_lanes(vfadd(
                f32x4([1.0, 2.0, 3.0, 4.0]),
                f32x4([10.0, 20.0, 30.0, 40.0]),
            )) != [11.0, 22.0, 33.0, 44.0]
            || f32_lanes(vfsub(
                f32x4([10.0, 20.0, 30.0, 40.0]),
                f32x4([1.0, 2.0, 3.0, 4.0]),
            )) != [9.0, 18.0, 27.0, 36.0]
            || f32_lanes(vfmul(
                f32x4([1.0, 2.0, 3.0, 4.0]),
                f32x4([10.0, 20.0, 30.0, 40.0]),
            )) != [10.0, 40.0, 90.0, 160.0]
            || f32_lanes(vfdiv(
                f32x4([10.0, 40.0, 90.0, 160.0]),
                f32x4([1.0, 2.0, 3.0, 4.0]),
            )) != [10.0, 20.0, 30.0, 40.0]
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
            || lanes(vdiv(i32x4([4, 0, 0, 0]), i32x4([0; 4]))) != [-1; 4]
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

fn integer_fixed_ops() -> bool {
    unsafe {
        if lanes(vsadd(
            i32x4([i32::MAX, i32::MIN, 100, -100]),
            i32x4([1, -1, 50, -50]),
        )) != [i32::MAX, i32::MIN, 150, -150]
        {
            return false;
        }
        if lanes(vssub(
            i32x4([i32::MIN, i32::MAX, 100, -100]),
            i32x4([1, -1, 50, -50]),
        )) != [i32::MIN, i32::MAX, 50, -50]
        {
            return false;
        }
        if lanes(vsaddu(i32x4([-1, 0, 1, 2]), i32x4([1; 4]))) != [-1, 1, 2, 3]
            || lanes(vssubu(i32x4([0, 1, 2, 3]), i32x4([1; 4]))) != [0, 0, 1, 2]
        {
            return false;
        }
        if lanes(vaadd(i32x4([1, 2, 3, 4]), i32x4([2, 3, 4, 5]))) != [2, 3, 4, 5]
            || lanes(vaaddu(i32x4([1, 2, 3, 4]), i32x4([2, 3, 4, 5]))) != [2, 3, 4, 5]
        {
            return false;
        }
        if lanes(vasub(i32x4([2, 3, 4, 5]), i32x4([1; 4]))) != [1, 1, 2, 2]
            || lanes(vasubu(i32x4([2, 3, 4, 5]), i32x4([1; 4]))) != [1, 1, 2, 2]
        {
            return false;
        }

        let wide_a = i16x8([1, -2, 30000, -30000, 5, 6, 7, 8]);
        let wide_b = i16x8([2, 3, 30000, 30000, 9, 10, 11, 12]);
        if lanes(vwadd(wide_a, wide_b)) != [3, 1, 60000, 0]
            || lanes(vwsub(wide_a, wide_b)) != [-1, -5, 0, -60000]
            || lanes(vwmul(wide_a, wide_b)) != [2, -6, 900_000_000, -900_000_000]
        {
            return false;
        }
        if lanes(vwaddu(
            i16x8([-1, 1, 0, 2, 0, 0, 0, 0]),
            i16x8([1, 1, 1, 1, 0, 0, 0, 0]),
        )) != [65536, 2, 1, 3]
            || lanes(vwsubu(
                i16x8([-1, 0, 2, 3, 0, 0, 0, 0]),
                i16x8([1, 1, 1, 1, 0, 0, 0, 0]),
            )) != [65534, -1, 1, 2]
        {
            return false;
        }
        if lanes(vwmulu(
            i16x8([-1, 2, 3, 4, 0, 0, 0, 0]),
            i16x8([2, 3, 4, 5, 0, 0, 0, 0]),
        )) != [131070, 6, 12, 20]
            || lanes(vwmulsu(
                i16x8([-1, 2, -3, 4, 0, 0, 0, 0]),
                i16x8([2, 3, 4, 5, 0, 0, 0, 0]),
            )) != [-2, 6, -12, 20]
        {
            return false;
        }
        if lanes(vwmacc(
            i32x4([10, 20, 30, 40]),
            i16x8([1, 2, 3, 4, 0, 0, 0, 0]),
            i16x8([2, 3, 4, 5, 0, 0, 0, 0]),
        )) != [12, 26, 42, 60]
            || lanes(vwmaccu(
                i32x4([10, 20, 30, 40]),
                i16x8([-1, 2, 3, 4, 0, 0, 0, 0]),
                i16x8([2, 3, 4, 5, 0, 0, 0, 0]),
            )) != [131080, 26, 42, 60]
            || lanes(vwmaccsu(
                i32x4([10, 20, 30, 40]),
                i16x8([-1, 2, -3, 4, 0, 0, 0, 0]),
                i16x8([2, 3, 4, 5, 0, 0, 0, 0]),
            )) != [8, 26, 18, 60]
            || lanes(vwmaccus(
                i32x4([10, 20, 30, 40]),
                2,
                i16x8([-1, 2, -3, 4, 0, 0, 0, 0]),
            )) != [8, 24, 24, 48]
        {
            return false;
        }

        let narrow_source = i8x16([0, 1, -128, -1, 127, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0]);
        if i16_lanes(vzext(narrow_source)) != [0, 1, 128, 255, 127, 2, 3, 4]
            || i16_lanes(vsext(narrow_source)) != [0, 1, -128, -1, 127, 2, 3, 4]
        {
            return false;
        }
        let narrow = i16_lanes(vnsrl(i32x4([0x0000_1200, 0x0000_3400, -1, 0x007f_ff00]), 8));
        if narrow != [0x12, 0x34, -1, 0x7fff, 0, 0, 0, 0] {
            return false;
        }
        let narrow = i16_lanes(vnsra(
            i32x4([-1, -256, 0x007f_ff00, 0xff80_0000u32 as i32]),
            8,
        ));
        if narrow != [-1, -1, 0x7fff, -0x8000, 0, 0, 0, 0] {
            return false;
        }
        let narrow = i16_lanes(vnclip(i32x4([40000, -40000, 100, -100]), 0));
        if narrow != [0x7fff, -0x8000, 100, -100, 0, 0, 0, 0] {
            return false;
        }
        let narrow = u16_lanes(vnclipu(i32x4([70000, 100, 0, 40000]), 0));
        if narrow != [65535, 100, 0, 40000, 0, 0, 0, 0] {
            return false;
        }
        true
    }
}

fn mask_memory_ops() -> bool {
    unsafe {
        if vmand(0xf0f0, 0xff00) != 0xf000
            || vmnand(0xf0f0, 0xff00) != 0x0fff
            || vmandn(0xf0f0, 0xff00) != 0x00f0
            || vmxor(0xf0f0, 0xff00) != 0x0ff0
            || vmor(0xf0f0, 0xff00) != 0xfff0
            || vmnor(0xf0f0, 0xff00) != 0x000f
            || vmorn(0xf0f0, 0xff00) != 0xf0ff
            || vmxnor(0xf0f0, 0xff00) != 0xf00f
        {
            return false;
        }
        if vcpop(0xf0f0) != 8 || vfirst(0x0100) != 8 || vfirst(0) != -1 {
            return false;
        }
        if vmsbf(0x0100) != 0x00ff || vmsof(0x0100) != 0x0100 || vmsif(0x0100) != 0x01ff {
            return false;
        }
        let iota = viota(0x000f).to_bytes();
        if iota[..5] != [0, 1, 2, 3, 4] {
            return false;
        }
        if vid().to_bytes() != core::array::from_fn::<u8, 16, _>(|i| i as u8) {
            return false;
        }

        if lanes(vslideup(i32x4([10, 20, 30, 40]), i32x4([1, 2, 3, 4]), 2)) != [10, 20, 1, 2]
            || lanes(vslidedown(i32x4([1, 2, 3, 4]), 2)) != [3, 4, 0, 0]
            || lanes(vslide1up(i32x4([1, 2, 3, 4]), 9)) != [9, 1, 2, 3]
            || lanes(vslide1down(i32x4([1, 2, 3, 4]), 9)) != [2, 3, 4, 9]
        {
            return false;
        }
        if lanes(vrgather(i32x4([10, 20, 30, 40]), i32x4([3, 2, 1, 0]))) != [40, 30, 20, 10]
            || lanes(vrgather(i32x4([10, 20, 30, 40]), i32x4([0, 4, 1, 2]))) != [10, 0, 20, 30]
        {
            return false;
        }
        if lanes(vrgatherei16(
            i32x4([10, 20, 30, 40]),
            i16x8([2, 0, 3, 1, 0, 0, 0, 0]),
        )) != [30, 10, 40, 20]
        {
            return false;
        }
        if lanes(vmerge(i32x4([1, 2, 3, 4]), i32x4([10, 20, 30, 40]), 0b1010)) != [1, 20, 3, 40]
            || lanes(vcompress(i32x4([1, 2, 3, 4]), 0b0101)) != [1, 3, 0, 0]
        {
            return false;
        }

        let bytes = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let value = vle8(bytes.as_ptr());
        if value.to_bytes() != bytes {
            return false;
        }
        let mut stored = [0u8; 16];
        vse8(stored.as_mut_ptr(), value);
        if stored != bytes {
            return false;
        }
        let words = [1i32, 2, 3, 4];
        if lanes(vle32(words.as_ptr())) != [1, 2, 3, 4] {
            return false;
        }
        let halves = [1u16, 2, 3, 4, 5, 6, 7, 8];
        if vle16(halves.as_ptr()).to_bytes() != i16x8([1, 2, 3, 4, 5, 6, 7, 8]).to_bytes() {
            return false;
        }
        let doubles = [1u64, 2];
        if vle64(doubles.as_ptr()).to_bytes() != {
            let mut expected = [0u8; 16];
            expected[..8].copy_from_slice(&1u64.to_le_bytes());
            expected[8..].copy_from_slice(&2u64.to_le_bytes());
            expected
        } {
            return false;
        }
        let mut stores = [0i32; 4];
        vse32(stores.as_mut_ptr(), i32x4([7, 8, 9, 10]));
        if stores != [7, 8, 9, 10] {
            return false;
        }

        let strided = [1i32, 2, 3, 4, 5, 6, 7, 8];
        if lanes(vlse32(strided.as_ptr(), 8)) != [1, 3, 5, 7] {
            return false;
        }
        let mut strided_out = [0i32; 8];
        vsse32(strided_out.as_mut_ptr(), 8, i32x4([10, 20, 30, 40]));
        if strided_out != [10, 0, 20, 0, 30, 0, 40, 0] {
            return false;
        }

        let base = [10i32, 20, 30, 40, 50];
        let indices = i32x4([16, 0, 8, 4]);
        if lanes(vloxei32(base.as_ptr(), indices)) != [50, 10, 30, 20]
            || lanes(vluxei32(base.as_ptr(), indices)) != [50, 10, 30, 20]
        {
            return false;
        }
        let mut gathered = [0i32; 5];
        vsoxei32(gathered.as_mut_ptr(), indices, i32x4([1, 2, 3, 4]));
        if gathered != [2, 4, 3, 0, 1] {
            return false;
        }
        let mut scattered = [0i32; 5];
        vsuxei32(scattered.as_mut_ptr(), indices, i32x4([5, 6, 7, 8]));
        if scattered != [6, 8, 7, 0, 5] {
            return false;
        }
        let bytes_indices = V128::from_bytes([4, 0, 12, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        if lanes(vloxei8(base.as_ptr(), bytes_indices)) != [20, 10, 40, 30]
            || lanes(vloxei16(base.as_ptr(), i16x8([4, 0, 12, 8, 0, 0, 0, 0]))) != [20, 10, 40, 30]
            || lanes(vloxei64(base.as_ptr(), i32x4([4, 0, 0, 0]))) != [20, 10, 0, 0]
        {
            return false;
        }

        let interleaved = [1i32, 10, 2, 20, 3, 30, 4, 40];
        let pair = vlseg2(interleaved.as_ptr());
        if lanes(pair[0]) != [1, 2, 3, 4] || lanes(pair[1]) != [10, 20, 30, 40] {
            return false;
        }
        let mut interleaved_out = [0i32; 8];
        vsseg2(interleaved_out.as_mut_ptr(), pair);
        if interleaved_out != interleaved {
            return false;
        }
        let mut strided_segments = [0i32; 8];
        vssseg2(strided_segments.as_mut_ptr(), 8, pair);
        if strided_segments != interleaved {
            return false;
        }
        let strided_pair = vlsseg2(interleaved.as_ptr(), 8);
        if lanes(strided_pair[0]) != [1, 2, 3, 4] || lanes(strided_pair[1]) != [10, 20, 30, 40] {
            return false;
        }
        let mut eight = [0i32; 32];
        for i in 0..4 {
            for s in 0..8 {
                eight[i * 8 + s] = (s as i32) * 10 + i as i32;
            }
        }
        let segments3 = vlseg3(eight.as_ptr());
        let segments4 = vlseg4(eight.as_ptr());
        let segments5 = vlseg5(eight.as_ptr());
        let segments6 = vlseg6(eight.as_ptr());
        let segments7 = vlseg7(eight.as_ptr());
        core::hint::black_box(&segments3);
        core::hint::black_box(&segments4);
        core::hint::black_box(&segments5);
        core::hint::black_box(&segments6);
        core::hint::black_box(&segments7);
        let strided3 = vlsseg3(eight.as_ptr(), 32);
        let strided4 = vlsseg4(eight.as_ptr(), 32);
        let strided5 = vlsseg5(eight.as_ptr(), 32);
        let strided6 = vlsseg6(eight.as_ptr(), 32);
        let strided7 = vlsseg7(eight.as_ptr(), 32);
        core::hint::black_box(&strided3);
        core::hint::black_box(&strided4);
        core::hint::black_box(&strided5);
        core::hint::black_box(&strided6);
        core::hint::black_box(&strided7);

        let segments = vlseg8(eight.as_ptr());
        for s in 0..8 {
            let expected = [
                (s as i32) * 10,
                (s as i32) * 10 + 1,
                (s as i32) * 10 + 2,
                (s as i32) * 10 + 3,
            ];
            if lanes(segments[s]) != expected {
                return false;
            }
        }
        let mut eight_out = [0i32; 32];
        vsseg8(eight_out.as_mut_ptr(), segments);
        if eight_out != eight {
            return false;
        }

        let mask_bytes = [0x34u8, 0x12];
        let whole = [0u8; 128];
        let _ = vl1re8(whole.as_ptr());
        let _ = vl2re8(whole.as_ptr());
        let _ = vl4re8(whole.as_ptr());
        let _ = vl8re8(whole.as_ptr());
        let _ = vl1re16(whole.as_ptr());
        let _ = vl2re16(whole.as_ptr());
        let _ = vl4re16(whole.as_ptr());
        let _ = vl8re16(whole.as_ptr());
        let _ = vl1re32(whole.as_ptr());
        let _ = vl2re32(whole.as_ptr());
        let _ = vl4re32(whole.as_ptr());
        let _ = vl8re32(whole.as_ptr());
        let _ = vl1re64(whole.as_ptr());
        let _ = vl2re64(whole.as_ptr());
        let _ = vl4re64(whole.as_ptr());
        let _ = vl8re64(whole.as_ptr());
        let mut whole_out = [0u8; 128];
        let v = V128::from_bytes([1; 16]);
        vs1r(whole_out.as_mut_ptr(), v);
        vs2r(whole_out.as_mut_ptr(), [v; 2]);
        vs4r(whole_out.as_mut_ptr(), [v; 4]);
        vs8r(whole_out.as_mut_ptr(), [v; 8]);
        core::hint::black_box(&whole_out);
        let mask = vlm(mask_bytes.as_ptr());
        if mask != 0x1234 {
            return false;
        }
        let mut mask_out = [0u8; 2];
        vsm(mask_out.as_mut_ptr(), mask);
        if mask_out != mask_bytes {
            return false;
        }

        if vredsum(i32x4([1, 2, 3, 4])) != 10
            || vredand(i32x4([0b1100, 0b1010, 0b1111, 0b0110])) != 0
            || vredor(i32x4([0b1100, 0b1010, 0b1111, 0b0110])) != 0b1111
            || vredxor(i32x4([0b1100, 0b1010, 0b1111, 0b0110])) != 0b1111
        {
            return false;
        }
        if vredmin(i32x4([4, 2, 7, -1])) != -1
            || vredmax(i32x4([4, 2, 7, -1])) != 7
            || vredminu(i32x4([-1, 2, 7, 1])) != 1
            || vredmaxu(i32x4([-1, 2, 7, 1])) != 0xffff_ffff
        {
            return false;
        }
        if vwredsum(i32x4([1, 2, 3, 4])) != 10 || vwredsumu(i32x4([-1, 1, 2, 3])) != 0x1_0000_0005 {
            return false;
        }
        true
    }
}

fn float_crypto_ops() -> bool {
    unsafe {
        unsafe {
            if lanes(vrsub_vx(i32x4([1, 2, 3, 4]), 10)) != [9, 8, 7, 6]
                || lanes(vrsub_vi::<10>(i32x4([1, 2, 3, 4]))) != [9, 8, 7, 6]
                || lanes(vadc_vvm(i32x4([1, 2, 3, 4]), i32x4([1, 1, 1, 1]), 0x0f)) != [3, 4, 5, 6]
                || lanes(vadc_vxm(i32x4([1, 2, 3, 4]), 1, 0x0f)) != [3, 4, 5, 6]
                || lanes(vadc_vim::<3>(i32x4([1, 1, 1, 1]), 0x0f)) != [5, 5, 5, 5]
            {
                return false;
            }
            if lanes(vmadc_vvm(i32x4([-1, 0, 0, 0]), i32x4([1, 0, 0, 0]), 0)) != [-1, 0, 0, 0]
                || lanes(vmadc_vv(i32x4([-1, 0, 0, 0]), i32x4([1, 0, 0, 0]))) != [-1, 0, 0, 0]
                || lanes(vsbc_vvm(i32x4([3, 4, 5, 6]), i32x4([1, 1, 1, 1]), 0x0f)) != [1, 2, 3, 4]
                || lanes(vmsbc_vv(i32x4([0, 1, 2, 3]), i32x4([1, 0, 0, 0]))) != [-1, 0, 0, 0]
            {
                return false;
            }
            if lanes(vsmul_vv(i32x4([1073741824, 2, 3, 4]), i32x4([2, 2, 2, 2]))) != [1, 0, 0, 0]
                || lanes(vssrl_vv(i32x4([16, 32, 64, 128]), i32x4([1, 2, 3, 4]))) != [8, 8, 8, 8]
                || lanes(vssrl_vx(i32x4([16, 32, 64, 128]), 1)) != [8, 16, 32, 64]
                || lanes(vssrl_vi::<1>(i32x4([16, 32, 64, 128]))) != [8, 16, 32, 64]
                || lanes(vssra_vv(i32x4([-16, -32, -64, -128]), i32x4([1, 2, 3, 4])))
                    != [-8, -8, -8, -8]
                || lanes(vssra_vx(i32x4([-16, -32, -64, -128]), 1)) != [-8, -16, -32, -64]
                || lanes(vssra_vi::<1>(i32x4([-16, -32, -64, -128]))) != [-8, -16, -32, -64]
            {
                return false;
            }
            core::hint::black_box(vfwredusum_vs(f32x4([1.0, 2.0, 3.0, 4.0]), f32x4([0.0; 4])));
            core::hint::black_box(vfwredosum_vs(f32x4([1.0, 2.0, 3.0, 4.0]), f32x4([0.0; 4])));
            core::hint::black_box(vmadc_vxm(i32x4([0x7fffffff; 4]), 1, 0));
            core::hint::black_box(vmadc_vx(i32x4([0x7fffffff; 4]), 1));
            core::hint::black_box(vmadc_vim::<1>(i32x4([0x7fffffff; 4]), 0));
            core::hint::black_box(vmadc_vi::<1>(i32x4([0x7fffffff; 4])));
            core::hint::black_box(vsbc_vxm(i32x4([3, 4, 5, 6]), 1, 0x0f));
            core::hint::black_box(vmsbc_vvm(i32x4([3, 4, 5, 6]), i32x4([1, 1, 1, 1]), 0));
            core::hint::black_box(vmsbc_vxm(i32x4([3, 4, 5, 6]), 1, 0));
            core::hint::black_box(vmsbc_vx(i32x4([3, 4, 5, 6]), 1));
            core::hint::black_box(vsmul_vx(i32x4([1, 2, 3, 4]), 2));
            let acc = f32x4([1.0, 1.0, 1.0, 1.0]);
            let a = f32x4([2.0, 2.0, 2.0, 2.0]);
            let b = f32x4([3.0, 3.0, 3.0, 3.0]);
            if f32_lanes(vfmacc_vv(acc, a, b)) != [7.0; 4]
                || f32_lanes(vfmacc_vf(acc, a, 3.0)) != [7.0; 4]
            {
                return false;
            }
            core::hint::black_box(vfnmacc_vv(acc, a, b));
            core::hint::black_box(vfnmacc_vf(acc, a, 3.0));
            core::hint::black_box(vfmsac_vv(acc, a, b));
            core::hint::black_box(vfmsac_vf(acc, a, 3.0));
            core::hint::black_box(vfnmsac_vv(acc, a, b));
            core::hint::black_box(vfnmsac_vf(acc, a, 3.0));
            core::hint::black_box(vmulhu_vv(i32x4([1, 2, 3, 4]), i32x4([2, 3, 4, 5])));
            core::hint::black_box(vmulhu_vx(i32x4([1, 2, 3, 4]), 2));
            core::hint::black_box(vmulhsu_vv(i32x4([1, 2, 3, 4]), i32x4([2, 3, 4, 5])));
            core::hint::black_box(vmulhsu_vx(i32x4([1, 2, 3, 4]), 2));
            core::hint::black_box(vmulh_vv(i32x4([1, 2, 3, 4]), i32x4([2, 3, 4, 5])));
            core::hint::black_box(vmulh_vx(i32x4([1, 2, 3, 4]), 2));
            let v = V128::from_bytes([7; 16]);
            core::hint::black_box(vmv1r(v));
            core::hint::black_box(vmv2r([v; 2]));
            core::hint::black_box(vmv4r([v; 4]));
            core::hint::black_box(vmv8r([v; 8]));
            if f64_lanes(vfwcvt_f_f_v(f32x4([1.0, 2.0, 3.0, 4.0]))) != [1.0, 2.0] {
                return false;
            }
            if f32_lanes(vfncvt_f_f_w(f64x2([1.0, 2.0]))) != [1.0, 2.0, 0.0, 0.0] {
                return false;
            }
            core::hint::black_box(vfwcvt_xu_f_v(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfwcvt_x_f_v(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfwcvt_f_xu_v(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfwcvt_f_x_v(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfwcvt_rtz_xu_f_v(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfwcvt_rtz_x_f_v(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfncvt_xu_f_w(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfncvt_x_f_w(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfncvt_f_xu_w(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfncvt_f_x_w(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfncvt_rod_f_f_w(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfncvt_rtz_xu_f_w(f32x4([1.0, 2.0, 3.0, 4.0])));
            core::hint::black_box(vfncvt_rtz_x_f_w(f32x4([1.0, 2.0, 3.0, 4.0])));
            let acc = f32x4([1.0, 1.0, 1.0, 1.0]);
            let a = f32x4([2.0, 2.0, 2.0, 2.0]);
            let b = f32x4([3.0, 3.0, 3.0, 3.0]);
            if f32_lanes(vfmadd_vv(acc, a, b)) != [5.0; 4]
                || f32_lanes(vfmadd_vf(acc, a, 3.0)) != [5.0; 4]
                || f32_lanes(vfrsub_vf(f32x4([1.0, 2.0, 3.0, 4.0]), 10.0)) != [9.0, 8.0, 7.0, 6.0]
            {
                return false;
            }
            core::hint::black_box(vfmsub_vv(acc, a, b));
            core::hint::black_box(vfmsub_vf(acc, a, 3.0));
            core::hint::black_box(vfnmadd_vv(acc, a, b));
            core::hint::black_box(vfnmadd_vf(acc, a, 3.0));
            core::hint::black_box(vfnmsub_vv(acc, a, b));
            core::hint::black_box(vfnmsub_vf(acc, a, 3.0));
            if f32_lanes(vfslide1up(f32x4([1.0, 2.0, 3.0, 4.0]), 9.0)) != [9.0, 1.0, 2.0, 3.0]
                || f32_lanes(vfslide1down(f32x4([1.0, 2.0, 3.0, 4.0]), 9.0)) != [2.0, 3.0, 4.0, 9.0]
            {
                return false;
            }
            if lanes(vmfeq_vv(
                f32x4([1.0, 2.0, 3.0, 4.0]),
                f32x4([1.0, 9.0, 3.0, 9.0]),
            )) != [-1, 0, -1, 0]
                || lanes(vmfeq_vf(f32x4([1.0, 2.0, 3.0, 4.0]), 1.0)) != [-1, 0, 0, 0]
            {
                return false;
            }
            let a = f32x4([1.0, 2.0, 3.0, 4.0]);
            let b = f32x4([4.0, 3.0, 2.0, 1.0]);
            core::hint::black_box(vmfle_vv(a, b));
            core::hint::black_box(vmfle_vf(a, 2.0));
            core::hint::black_box(vmflt_vv(a, b));
            core::hint::black_box(vmflt_vf(a, 2.0));
            core::hint::black_box(vmfne_vv(a, b));
            core::hint::black_box(vmfne_vf(a, 2.0));
            core::hint::black_box(vmfgt_vv(a, b));
            core::hint::black_box(vmfgt_vf(a, 2.0));
            core::hint::black_box(vmfge_vv(a, b));
            core::hint::black_box(vmfge_vf(a, 2.0));
        }
        if f32_lanes(vfrdiv(f32x4([1.0, 2.0, 4.0, 8.0]), 16.0)) != [16.0, 8.0, 4.0, 2.0] {
            return false;
        }
        if f32_lanes(vfmin(
            f32x4([1.0, 5.0, 3.0, 7.0]),
            f32x4([2.0, 4.0, 6.0, 0.0]),
        )) != [1.0, 4.0, 3.0, 0.0]
            || f32_lanes(vfmax(
                f32x4([1.0, 5.0, 3.0, 7.0]),
                f32x4([2.0, 4.0, 6.0, 0.0]),
            )) != [2.0, 5.0, 6.0, 7.0]
        {
            return false;
        }
        let signed = f32x4([1.0, -2.0, 3.0, -4.0]);
        let signs = f32x4([-0.0, 0.0, -1.0, 1.0]);
        if f32_lanes(vfsgnj(signed, signs)) != [-1.0, 2.0, -3.0, 4.0]
            || f32_lanes(vfsgnjn(signed, signs)) != [1.0, -2.0, 3.0, -4.0]
            || f32_lanes(vfsgnjx(signed, signs)) != [-1.0, -2.0, -3.0, -4.0]
        {
            return false;
        }
        if f32_lanes(vfabs(signed)) != [1.0, 2.0, 3.0, 4.0]
            || f32_lanes(vfneg(signed)) != [-1.0, 2.0, -3.0, 4.0]
        {
            return false;
        }
        if f32_lanes(vfsqrt(f32x4([4.0, 9.0, 16.0, 25.0]))) != [2.0, 3.0, 4.0, 5.0] {
            return false;
        }
        if (vfmv_f_s(vfrec7(vfmv(2.0))) - 0.5).abs() > 0.01
            || (vfmv_f_s(vfrsqrt7(vfmv(4.0))) - 0.5).abs() > 0.01
        {
            return false;
        }
        if f32_lanes(vfmerge(f32x4([1.0, 2.0, 3.0, 4.0]), 9.0, 0b1010)) != [1.0, 9.0, 3.0, 9.0]
            || f32_lanes(vfmv(7.0)) != [7.0; 4]
            || vfmv_f_s(f32x4([5.0, 6.0, 7.0, 8.0])) != 5.0
        {
            return false;
        }
        let acc = f32x4([2.0, 3.0, 4.0, 5.0]);
        let a = f32x4([10.0, 20.0, 30.0, 40.0]);
        let b = f32x4([1.0, 2.0, 3.0, 4.0]);
        if f32_lanes(vfma(acc, a, b)) != [21.0, 62.0, 123.0, 204.0]
            || f32_lanes(vfms(acc, a, b)) != [19.0, 58.0, 117.0, 196.0]
            || f32_lanes(vfnma(acc, a, b)) != [-21.0, -62.0, -123.0, -204.0]
            || f32_lanes(vfnms(acc, a, b)) != [-19.0, -58.0, -117.0, -196.0]
        {
            return false;
        }
        let values = f32x4([1.0, 2.0, 3.0, 4.0]);
        if vfredsum(values) != 10.0
            || vfredusum(values) != 10.0
            || vfredosum(values) != 10.0
            || vfredmax(values) != 4.0
            || vfredmin(values) != 1.0
        {
            return false;
        }
        let wide_a = f32x4([1.0, 2.0, 3.0, 4.0]);
        let wide_b = f32x4([10.0, 20.0, 30.0, 40.0]);
        if f64_lanes(vfwadd(wide_a, wide_b)) != [11.0, 22.0]
            || f64_lanes(vfwsub(wide_b, wide_a)) != [9.0, 18.0]
            || f64_lanes(vfwmul(wide_a, wide_b)) != [10.0, 40.0]
        {
            return false;
        }
        let wide_acc = f64x2([10.0, 20.0]);
        let mul_a = f32x4([2.0, 3.0, 4.0, 5.0]);
        let mul_b = f32x4([4.0, 5.0, 6.0, 7.0]);
        if f64_lanes(vfwmacc(wide_acc, mul_a, mul_b)) != [18.0, 35.0]
            || f64_lanes(vfwnmacc(wide_acc, mul_a, mul_b)) != [-18.0, -35.0]
            || f64_lanes(vfwmsac(wide_acc, mul_a, mul_b)) != [-2.0, -5.0]
            || f64_lanes(vfwnmsac(wide_acc, mul_a, mul_b)) != [2.0, 5.0]
        {
            return false;
        }

        let classes = vfclass(f32x4([
            0.0,
            -0.0,
            f32::from_bits(0x7f80_0000),
            f32::from_bits(0xff80_0000),
        ]));
        if lanes(classes) != [1 << 4, 1 << 3, 1 << 7, 1 << 0] {
            return false;
        }
        let classes = vfclass(f32x4([
            f32::from_bits(0x7f80_0001),
            f32::from_bits(0x7fc0_0000),
            1.0,
            -1.0,
        ]));
        if lanes(classes) != [1 << 8, 1 << 9, 1 << 6, 1 << 1] {
            return false;
        }
        let classes = vfclass(f32x4([
            f32::from_bits(0x0000_0001),
            f32::from_bits(0x8000_0001),
            0.0,
            0.0,
        ]));
        if lanes(classes) != [1 << 5, 1 << 2, 1 << 4, 1 << 4] {
            return false;
        }

        if lanes(vfcvt_x_f(f32x4([1.5, 2.5, -1.5, -2.5]))) != [2, 2, -2, -2]
            || lanes(vfcvt_rtz_x_f(f32x4([1.5, 2.5, -1.5, -2.5]))) != [1, 2, -1, -2]
            || lanes(vfcvt_xu_f(f32x4([1.5, 2.5, 3.5, 4.5]))) != [2, 2, 4, 4]
            || lanes(vfcvt_rtz_xu_f(f32x4([1.5, 2.5, 3.5, 4.5]))) != [1, 2, 3, 4]
        {
            return false;
        }
        if f32_lanes(vfcvt_f_x(i32x4([1, -2, 3, -4]))) != [1.0, -2.0, 3.0, -4.0]
            || f32_lanes(vfcvt_f_xu(i32x4([1, 2, 3, 4]))) != [1.0, 2.0, 3.0, 4.0]
        {
            return false;
        }

        let zero = V128::from_bytes([0; 16]);
        if vaesef(zero, zero).to_bytes() != [0x63; 16]
            || vaesem(zero, zero).to_bytes() != [0x63; 16]
            || vaesdf(zero, zero).to_bytes() != [0x52; 16]
            || vaesdm(zero, zero).to_bytes() != [0x52; 16]
            || vaesz(zero, V128::from_bytes([0x11; 16])).to_bytes() != [0x11; 16]
        {
            return false;
        }
        let key_schedule = vaeskf1(zero, 1).to_bytes();
        let mut expected_schedule = [0u8; 16];
        for word in expected_schedule.chunks_mut(4) {
            word.copy_from_slice(&[0x62, 0x63, 0x63, 0x63]);
        }
        if key_schedule != expected_schedule {
            return false;
        }
        if vaeskf1(zero, 2).to_bytes() == [0; 16]
            || vaeskf2(zero, 2).to_bytes() == [0; 16]
            || vsm4k(zero, 0).to_bytes() == [0; 16]
            || vsm4r(zero, zero).to_bytes() == [0; 16]
        {
            return false;
        }
        if vsha2ms(zero, zero, zero).to_bytes() != [0; 16]
            || vsha2ch(zero, zero, zero).to_bytes() != [0; 16]
            || vsha2cl(zero, zero, zero).to_bytes() != [0; 16]
        {
            return false;
        }
        let zero_pair = [zero; 2];
        if vsm3me(zero_pair, zero_pair) != zero_pair {
            return false;
        }
        if vsm3c(zero_pair, 0) == zero_pair {
            return false;
        }

        let carry_a = V128::from_bytes({
            let mut bytes = [0u8; 16];
            bytes[..8].copy_from_slice(&3u64.to_le_bytes());
            bytes[8..].copy_from_slice(&5u64.to_le_bytes());
            bytes
        });
        let carry_b = V128::from_bytes({
            let mut bytes = [0u8; 16];
            bytes[..8].copy_from_slice(&7u64.to_le_bytes());
            bytes[8..].copy_from_slice(&11u64.to_le_bytes());
            bytes
        });
        let low = vclmul(carry_a, carry_b).to_bytes();
        if u64::from_le_bytes(low[..8].try_into().unwrap()) != 9
            || u64::from_le_bytes(low[8..].try_into().unwrap()) != 39
            || vclmulh(carry_a, carry_b).to_bytes() != [0; 16]
        {
            return false;
        }
        let one = i32x4([1, 0, 0, 0]);
        let product = V128::from_bytes([0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        if vghsh(zero, one, one).to_bytes() != product.to_bytes()
            || vgmul(one, one).to_bytes() != product.to_bytes()
        {
            return false;
        }
        true
    }
}

fn scalar_integer_ops() -> bool {
    if unsafe { maxu(1, 2) } != 2
        || unsafe { minu(1, 2) } != 1
        || unsafe { max(-1i64 as u64, 0) } != 0
        || unsafe { min(-1i64 as u64, 0) } != -1i64 as u64
        || unsafe { orc_b(0x0100) } != 0xff00
    {
        return false;
    }
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
        if sext_b(0xff) != u64::MAX
            || clmul(1, 1) != 1
            || clmulh(1 << 63, 2) != 1
            || clmulr(1 << 63, 2) != 2
        {
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
        if zip(0x0000_0001) != 0x0000_0001
            || zip(0x0001_0000) != 0x0000_0002
            || zip(0x8000_0000) != 0x8000_0000
            || unzip(0x0000_0002) != 0x0001_0000
            || unzip(0x0000_0001) != 0x0000_0001
            || unzip(zip(0x1234_5678)) != 0x1234_5678
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
            && sha256sum0(1) != sha256sum0(2)
            && aes64es(1, 2) != 1
            && aes64es(1, 2) != aes64es(3, 4)
            && aes64ks1i(1, 0) != 1
            && aes64ks1i(1, 1) != aes64ks1i(1, 0)
    }
}

#[target_feature(enable = "zks")]
unsafe fn zks_ops() -> bool {
    unsafe {
        sm3p0(1) == 0x0002_0201
            && sm3p1(1) == 0x0080_8001
            && sm3p1(0x1234_5678) == 0x0501_4549
            && sm4ed::<0>(1, 2) != 1
            && sm4ks::<0>(1, 2) == 0x209d_74e8
            && sm4ks::<1>(1, 2) == 0x1a6b_d6c1
            && sm4ks::<2>(1, 2) == 0x6bd6_c01b
            && sm4ks::<3>(1, 2) == 0xd6c0_1a6a
    }
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

unsafe fn fp16_zfa_ops() -> bool {
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
