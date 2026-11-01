#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::loongarch::integer::bitmanip::{bitrev, bstrins, bstrpick, revb, revh, revw};
use rust_isa::loongarch::simd::lasx;
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
use rust_isa::loongarch::simd::lsx;
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
    vfmul_d, vfsub, vfsub_d, vftint_w_d, vftint_w_s, vftintrz_w_s, vld, vldi, vrepl_b, vrepl_d,
    vrepl_h, vrepl_w, vreplve_b, vreplve_d, vreplve_h, vreplve_w, vsat, vsat_w, vsat_wu, vssr,
    vssrarn_w_d, vssrln_w_d, vst,
};
use rust_isa::loongarch::simd::lsx::{vexth, vfcmp_d, vffint, vftint, vssrarn_wu_du};

use rust_isa::loongarch::simd::lasx::{
    vext2xv_h_b, xvavgr, xvbitsel, xvfadd, xvfcmp, xvfdiv, xvffint, xvfmax, xvfmin, xvfmul, xvfsub,
    xvftint, xvld, xvldi, xvmax, xvmin, xvnor, xvpackev, xvpackod, xvsat, xvseq, xvseqi, xvshuf,
    xvshuf4i, xvsle, xvslei, xvsll, xvslli, xvslt, xvslti, xvsra, xvsrai, xvsrl, xvsrli, xvst,
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

#[target_feature(enable = "lasx")]
unsafe fn generated_ops() -> u32 {
    #[repr(align(64))]
    struct Aligned([u8; 64]);
    let mut storage = Aligned([0; 64]);
    for i in 0..64 {
        storage.0[i] = i as u8;
    }
    let a = i8x16([2; 16]);
    let b = i8x16([3; 16]);
    let c = i8x16([4; 16]);
    if i8_lanes(lsx::vseq_b(a, a)) != [-1; 16] {
        return 1;
    }
    if i8_lanes(lsx::vclz_b(i8x16([1; 16]))) != [7; 16] {
        return 2;
    }
    if i8_lanes(lsx::vmadd_b(a, b, c)) != [14; 16] {
        return 3;
    }
    if i8_lanes(lsx::vreplgr2vr_b(7)) != [7; 16] {
        return 4;
    }
    let loaded = lsx::vldx(storage.0.as_ptr(), 0);
    let mut out = [0u8; 16];
    lsx::vstx(loaded, out.as_mut_ptr(), 0);
    if out != storage.0[..16] {
        return 5;
    }
    core::hint::black_box(lsx::vbitsel_v(a, b, c));
    core::hint::black_box(lsx::vshuf_b(a, b, c));
    core::hint::black_box(lsx::vsle_b(a, b));
    core::hint::black_box(lsx::vslt_b(a, b));
    core::hint::black_box(lsx::vreplve_b::<0>(a));
    let va = V256::from_bytes([2; 32]);
    let vb = V256::from_bytes([3; 32]);
    let vc = V256::from_bytes([4; 32]);
    core::hint::black_box(lasx::xvseq_b(va, va));
    core::hint::black_box(lasx::xvclz_b(va));
    core::hint::black_box(lasx::xvmadd_b(va, vb, vc));
    core::hint::black_box(lasx::xvbitsel_v(va, vb, vc));
    core::hint::black_box(lasx::xvshuf_b(va, vb, vc));
    let vloaded = lasx::xvldx(storage.0.as_ptr(), 0);
    let mut vout = [0u8; 32];
    lasx::xvstx(vloaded, vout.as_mut_ptr(), 0);
    if vout != storage.0[..32] {
        return 6;
    }
    if i8_lanes(lsx::vseqi_b::<2>(a)) != [-1; 16]
        || i8_lanes(lsx::vslei_bu::<3>(a)) != [-1; 16]
        || i8_lanes(lsx::vslti_bu::<1>(a)) != [0; 16]
        || i8_lanes(lsx::vslli_b::<1>(a)) != [4; 16]
        || i8_lanes(lsx::vsrli_b::<1>(i8x16([4; 16]))) != [2; 16]
        || i8_lanes(lsx::vsrai_b::<1>(i8x16([-4; 16]))) != [-2; 16]
        || i8_lanes(lsx::vmaxi_b::<3>(a)) != [3; 16]
        || i8_lanes(lsx::vmini_bu::<3>(i8x16([5; 16]))) != [3; 16]
    {
        return 7;
    }
    if i8_lanes(lsx::vrotri_b::<1>(i8x16([2; 16]))) != [1; 16]
        || i8_lanes(lsx::vbitclri_b::<1>(i8x16([3; 16]))) != [1; 16]
        || i8_lanes(lsx::vbitseti_b::<1>(i8x16([0; 16]))) != [2; 16]
        || i8_lanes(lsx::vbitrevi_b::<1>(i8x16([2; 16]))) != [0; 16]
        || i8_lanes(lsx::vandi_b::<0x0f>(i8x16([0x13; 16]))) != [3; 16]
        || i8_lanes(lsx::vnori_b::<0>(i8x16([0; 16]))) != [-1; 16]
    {
        return 8;
    }
    if i8_lanes(lsx::vaddi_bu::<3>(i8x16([0x7f; 16]))) != [0x82u8 as i8; 16]
        || i8_lanes(lsx::vsubi_bu::<3>(i8x16([2; 16]))) != [0; 16]
        || i8_lanes(lsx::vsrlri_b::<1>(i8x16([2; 16]))) != [1; 16]
        || i8_lanes(lsx::vsrari_b::<1>(i8x16([-3; 16]))) != [-1; 16]
    {
        return 9;
    }
    if f32_lanes(lsx::vfsqrt_s(f32x4([4.0, 9.0, 16.0, 25.0]))) != [2.0, 3.0, 4.0, 5.0] {
        return 10;
    }
    if f32_lanes(lsx::vfrintrz_s(f32x4([1.9, -1.9, 2.5, -2.5]))) != [1.0, -1.0, 2.0, -2.0] {
        return 11;
    }
    let mut st = [0u8; 8];
    unsafe { lsx::vstelm_b::<0>(a, st.as_mut_ptr(), 0) };
    if st[0] != 2
        || i8_lanes(lsx::vreplvei_b::<1>(a)) != [a.to_bytes()[1] as i8; 16]
        || i8_lanes(lsx::vinsgr2vr_b::<0>(a, 7))[0] != 7
        || lsx::vpickve2gr_b::<1>(a) != 2
        || lsx::vpickve2gr_bu::<1>(a) != 2
    {
        return 13;
    }
    if i8_lanes(unsafe { lsx::vldrepl_b(storage.0.as_ptr(), 1) }) != [1; 16] {
        return 14;
    }
    let shift_input = V128::from_bytes(core::array::from_fn(|i| i as u8));
    let mut left_shifted = [0u8; 16];
    left_shifted[1..].copy_from_slice(&shift_input.to_bytes()[..15]);
    let mut right_shifted = [0u8; 16];
    right_shifted[..15].copy_from_slice(&shift_input.to_bytes()[1..]);
    if i8_lanes(lsx::vbsll(shift_input, 1)) != left_shifted.map(|b| b as i8)
        || i8_lanes(lsx::vbsrl(shift_input, 1)) != right_shifted.map(|b| b as i8)
        || !lsx::vseteqz(V128::from_bytes([0; 16]))
        || lsx::vsetnez(V128::from_bytes([0; 16]))
        || !lsx::vsetanyeqz_b(V128::from_bytes([0; 16]))
        || !lsx::vsetallnez_b(a)
        || i8_lanes(lsx::vbitseli_b::<0x0f>(
            V128::from_bytes([0; 16]),
            V128::from_bytes([0xff; 16]),
        )) != [-1; 16]
    {
        return 18;
    }
    let sat = lasx::xvsat_b::<3>(V256::from_bytes([100; 32]));
    if sat.to_bytes()[0] != 7 {
        return 19;
    }
    unsafe {
        core::hint::black_box(lsx::vbitsel_v(a, a, a));
        core::hint::black_box(lsx::vshuf_b(a, a, a));
        core::hint::black_box(lsx::vldx(storage.0.as_ptr(), 0));
        core::hint::black_box(lsx::vstx(a, out.as_mut_ptr(), 0));
        core::hint::black_box(lsx::vseq_b(a, a));
        core::hint::black_box(lsx::vseq_h(a, a));
        core::hint::black_box(lsx::vseq_w(a, a));
        core::hint::black_box(lsx::vseq_d(a, a));
        core::hint::black_box(lsx::vsle_b(a, a));
        core::hint::black_box(lsx::vsle_h(a, a));
        core::hint::black_box(lsx::vsle_w(a, a));
        core::hint::black_box(lsx::vsle_d(a, a));
        core::hint::black_box(lsx::vsle_bu(a, a));
        core::hint::black_box(lsx::vsle_hu(a, a));
        core::hint::black_box(lsx::vsle_wu(a, a));
        core::hint::black_box(lsx::vsle_du(a, a));
        core::hint::black_box(lsx::vslt_b(a, a));
        core::hint::black_box(lsx::vslt_h(a, a));
        core::hint::black_box(lsx::vslt_w(a, a));
        core::hint::black_box(lsx::vslt_d(a, a));
        core::hint::black_box(lsx::vslt_bu(a, a));
        core::hint::black_box(lsx::vslt_hu(a, a));
        core::hint::black_box(lsx::vslt_wu(a, a));
        core::hint::black_box(lsx::vslt_du(a, a));
        core::hint::black_box(lsx::vaddwev_h_b(a, a));
        core::hint::black_box(lsx::vaddwev_w_h(a, a));
        core::hint::black_box(lsx::vaddwev_d_w(a, a));
        core::hint::black_box(lsx::vaddwev_q_d(a, a));
        core::hint::black_box(lsx::vsubwev_h_b(a, a));
        core::hint::black_box(lsx::vsubwev_w_h(a, a));
        core::hint::black_box(lsx::vsubwev_d_w(a, a));
        core::hint::black_box(lsx::vsubwev_q_d(a, a));
        core::hint::black_box(lsx::vaddwod_h_b(a, a));
        core::hint::black_box(lsx::vaddwod_w_h(a, a));
        core::hint::black_box(lsx::vaddwod_d_w(a, a));
        core::hint::black_box(lsx::vaddwod_q_d(a, a));
        core::hint::black_box(lsx::vsubwod_h_b(a, a));
        core::hint::black_box(lsx::vsubwod_w_h(a, a));
        core::hint::black_box(lsx::vsubwod_d_w(a, a));
        core::hint::black_box(lsx::vsubwod_q_d(a, a));
        core::hint::black_box(lsx::vaddwev_h_bu(a, a));
        core::hint::black_box(lsx::vaddwev_w_hu(a, a));
        core::hint::black_box(lsx::vaddwev_d_wu(a, a));
        core::hint::black_box(lsx::vaddwev_q_du(a, a));
        core::hint::black_box(lsx::vsubwev_h_bu(a, a));
        core::hint::black_box(lsx::vsubwev_w_hu(a, a));
        core::hint::black_box(lsx::vsubwev_d_wu(a, a));
        core::hint::black_box(lsx::vsubwev_q_du(a, a));
        core::hint::black_box(lsx::vaddwod_h_bu(a, a));
        core::hint::black_box(lsx::vaddwod_w_hu(a, a));
        core::hint::black_box(lsx::vaddwod_d_wu(a, a));
        core::hint::black_box(lsx::vaddwod_q_du(a, a));
        core::hint::black_box(lsx::vsubwod_h_bu(a, a));
        core::hint::black_box(lsx::vsubwod_w_hu(a, a));
        core::hint::black_box(lsx::vsubwod_d_wu(a, a));
        core::hint::black_box(lsx::vsubwod_q_du(a, a));
        core::hint::black_box(lsx::vaddwev_h_bu_b(a, a));
        core::hint::black_box(lsx::vaddwev_w_hu_h(a, a));
        core::hint::black_box(lsx::vaddwev_d_wu_w(a, a));
        core::hint::black_box(lsx::vaddwev_q_du_d(a, a));
        core::hint::black_box(lsx::vaddwod_h_bu_b(a, a));
        core::hint::black_box(lsx::vaddwod_w_hu_h(a, a));
        core::hint::black_box(lsx::vaddwod_d_wu_w(a, a));
        core::hint::black_box(lsx::vaddwod_q_du_d(a, a));
        core::hint::black_box(lsx::vsadd_b(a, a));
        core::hint::black_box(lsx::vsadd_h(a, a));
        core::hint::black_box(lsx::vsadd_w(a, a));
        core::hint::black_box(lsx::vsadd_d(a, a));
        core::hint::black_box(lsx::vssub_b(a, a));
        core::hint::black_box(lsx::vssub_h(a, a));
        core::hint::black_box(lsx::vssub_w(a, a));
        core::hint::black_box(lsx::vssub_d(a, a));
        core::hint::black_box(lsx::vsadd_bu(a, a));
        core::hint::black_box(lsx::vsadd_hu(a, a));
        core::hint::black_box(lsx::vsadd_wu(a, a));
        core::hint::black_box(lsx::vsadd_du(a, a));
        core::hint::black_box(lsx::vssub_bu(a, a));
        core::hint::black_box(lsx::vssub_hu(a, a));
        core::hint::black_box(lsx::vssub_wu(a, a));
        core::hint::black_box(lsx::vssub_du(a, a));
        core::hint::black_box(lsx::vhaddw_h_b(a, a));
        core::hint::black_box(lsx::vhaddw_w_h(a, a));
        core::hint::black_box(lsx::vhaddw_d_w(a, a));
        core::hint::black_box(lsx::vhaddw_q_d(a, a));
        core::hint::black_box(lsx::vhsubw_h_b(a, a));
        core::hint::black_box(lsx::vhsubw_w_h(a, a));
        core::hint::black_box(lsx::vhsubw_d_w(a, a));
        core::hint::black_box(lsx::vhsubw_q_d(a, a));
        core::hint::black_box(lsx::vhaddw_hu_bu(a, a));
        core::hint::black_box(lsx::vhaddw_wu_hu(a, a));
        core::hint::black_box(lsx::vhaddw_du_wu(a, a));
        core::hint::black_box(lsx::vhaddw_qu_du(a, a));
        core::hint::black_box(lsx::vhsubw_hu_bu(a, a));
        core::hint::black_box(lsx::vhsubw_wu_hu(a, a));
        core::hint::black_box(lsx::vhsubw_du_wu(a, a));
        core::hint::black_box(lsx::vhsubw_qu_du(a, a));
        core::hint::black_box(lsx::vadda_b(a, a));
        core::hint::black_box(lsx::vadda_h(a, a));
        core::hint::black_box(lsx::vadda_w(a, a));
        core::hint::black_box(lsx::vadda_d(a, a));
        core::hint::black_box(lsx::vabsd_b(a, a));
        core::hint::black_box(lsx::vabsd_h(a, a));
        core::hint::black_box(lsx::vabsd_w(a, a));
        core::hint::black_box(lsx::vabsd_d(a, a));
        core::hint::black_box(lsx::vabsd_bu(a, a));
        core::hint::black_box(lsx::vabsd_hu(a, a));
        core::hint::black_box(lsx::vabsd_wu(a, a));
        core::hint::black_box(lsx::vabsd_du(a, a));
        core::hint::black_box(lsx::vavg_b(a, a));
        core::hint::black_box(lsx::vavg_h(a, a));
        core::hint::black_box(lsx::vavg_w(a, a));
        core::hint::black_box(lsx::vavg_d(a, a));
        core::hint::black_box(lsx::vavg_bu(a, a));
        core::hint::black_box(lsx::vavg_hu(a, a));
        core::hint::black_box(lsx::vavg_wu(a, a));
        core::hint::black_box(lsx::vavg_du(a, a));
        core::hint::black_box(lsx::vmax_b(a, a));
        core::hint::black_box(lsx::vmax_h(a, a));
        core::hint::black_box(lsx::vmax_w(a, a));
        core::hint::black_box(lsx::vmax_d(a, a));
        core::hint::black_box(lsx::vmin_b(a, a));
        core::hint::black_box(lsx::vmin_h(a, a));
        core::hint::black_box(lsx::vmin_w(a, a));
        core::hint::black_box(lsx::vmin_d(a, a));
        core::hint::black_box(lsx::vmax_bu(a, a));
        core::hint::black_box(lsx::vmax_hu(a, a));
        core::hint::black_box(lsx::vmax_wu(a, a));
        core::hint::black_box(lsx::vmax_du(a, a));
        core::hint::black_box(lsx::vmin_bu(a, a));
        core::hint::black_box(lsx::vmin_hu(a, a));
        core::hint::black_box(lsx::vmin_wu(a, a));
        core::hint::black_box(lsx::vmin_du(a, a));
        core::hint::black_box(lsx::vmul_b(a, a));
        core::hint::black_box(lsx::vmul_d(a, a));
        core::hint::black_box(lsx::vmuh_b(a, a));
        core::hint::black_box(lsx::vmuh_h(a, a));
        core::hint::black_box(lsx::vmuh_w(a, a));
        core::hint::black_box(lsx::vmuh_d(a, a));
        core::hint::black_box(lsx::vmuh_bu(a, a));
        core::hint::black_box(lsx::vmuh_hu(a, a));
        core::hint::black_box(lsx::vmuh_wu(a, a));
        core::hint::black_box(lsx::vmuh_du(a, a));
        core::hint::black_box(lsx::vmulwev_h_b(a, a));
        core::hint::black_box(lsx::vmulwev_w_h(a, a));
        core::hint::black_box(lsx::vmulwev_d_w(a, a));
        core::hint::black_box(lsx::vmulwev_q_d(a, a));
        core::hint::black_box(lsx::vmulwod_h_b(a, a));
        core::hint::black_box(lsx::vmulwod_w_h(a, a));
        core::hint::black_box(lsx::vmulwod_d_w(a, a));
        core::hint::black_box(lsx::vmulwod_q_d(a, a));
        core::hint::black_box(lsx::vmulwev_h_bu(a, a));
        core::hint::black_box(lsx::vmulwev_w_hu(a, a));
        core::hint::black_box(lsx::vmulwev_d_wu(a, a));
        core::hint::black_box(lsx::vmulwev_q_du(a, a));
        core::hint::black_box(lsx::vmulwod_h_bu(a, a));
        core::hint::black_box(lsx::vmulwod_w_hu(a, a));
        core::hint::black_box(lsx::vmulwod_d_wu(a, a));
        core::hint::black_box(lsx::vmulwod_q_du(a, a));
        core::hint::black_box(lsx::vmulwev_h_bu_b(a, a));
        core::hint::black_box(lsx::vmulwev_w_hu_h(a, a));
        core::hint::black_box(lsx::vmulwev_d_wu_w(a, a));
        core::hint::black_box(lsx::vmulwev_q_du_d(a, a));
        core::hint::black_box(lsx::vmulwod_h_bu_b(a, a));
        core::hint::black_box(lsx::vmulwod_w_hu_h(a, a));
        core::hint::black_box(lsx::vmulwod_d_wu_w(a, a));
        core::hint::black_box(lsx::vmulwod_q_du_d(a, a));
        core::hint::black_box(lsx::vmadd_b(a, a, a));
        core::hint::black_box(lsx::vmadd_h(a, a, a));
        core::hint::black_box(lsx::vmadd_w(a, a, a));
        core::hint::black_box(lsx::vmadd_d(a, a, a));
        core::hint::black_box(lsx::vmsub_b(a, a, a));
        core::hint::black_box(lsx::vmsub_h(a, a, a));
        core::hint::black_box(lsx::vmsub_w(a, a, a));
        core::hint::black_box(lsx::vmsub_d(a, a, a));
        core::hint::black_box(lsx::vmaddwev_h_b(a, a, a));
        core::hint::black_box(lsx::vmaddwev_w_h(a, a, a));
        core::hint::black_box(lsx::vmaddwev_d_w(a, a, a));
        core::hint::black_box(lsx::vmaddwev_q_d(a, a, a));
        core::hint::black_box(lsx::vmaddwod_h_b(a, a, a));
        core::hint::black_box(lsx::vmaddwod_w_h(a, a, a));
        core::hint::black_box(lsx::vmaddwod_d_w(a, a, a));
        core::hint::black_box(lsx::vmaddwod_q_d(a, a, a));
        core::hint::black_box(lsx::vmaddwev_h_bu(a, a, a));
        core::hint::black_box(lsx::vmaddwev_w_hu(a, a, a));
        core::hint::black_box(lsx::vmaddwev_d_wu(a, a, a));
        core::hint::black_box(lsx::vmaddwev_q_du(a, a, a));
        core::hint::black_box(lsx::vmaddwod_h_bu(a, a, a));
        core::hint::black_box(lsx::vmaddwod_w_hu(a, a, a));
        core::hint::black_box(lsx::vmaddwod_d_wu(a, a, a));
        core::hint::black_box(lsx::vmaddwod_q_du(a, a, a));
        core::hint::black_box(lsx::vmaddwev_h_bu_b(a, a, a));
        core::hint::black_box(lsx::vmaddwev_w_hu_h(a, a, a));
        core::hint::black_box(lsx::vmaddwev_d_wu_w(a, a, a));
        core::hint::black_box(lsx::vmaddwev_q_du_d(a, a, a));
        core::hint::black_box(lsx::vmaddwod_h_bu_b(a, a, a));
        core::hint::black_box(lsx::vmaddwod_w_hu_h(a, a, a));
        core::hint::black_box(lsx::vmaddwod_d_wu_w(a, a, a));
        core::hint::black_box(lsx::vmaddwod_q_du_d(a, a, a));
        core::hint::black_box(lsx::vdiv_b(a, a));
        core::hint::black_box(lsx::vdiv_h(a, a));
        core::hint::black_box(lsx::vdiv_w(a, a));
        core::hint::black_box(lsx::vdiv_d(a, a));
        core::hint::black_box(lsx::vmod_b(a, a));
        core::hint::black_box(lsx::vmod_h(a, a));
        core::hint::black_box(lsx::vmod_w(a, a));
        core::hint::black_box(lsx::vmod_d(a, a));
        core::hint::black_box(lsx::vdiv_bu(a, a));
        core::hint::black_box(lsx::vdiv_hu(a, a));
        core::hint::black_box(lsx::vdiv_wu(a, a));
        core::hint::black_box(lsx::vdiv_du(a, a));
        core::hint::black_box(lsx::vmod_bu(a, a));
        core::hint::black_box(lsx::vmod_hu(a, a));
        core::hint::black_box(lsx::vmod_wu(a, a));
        core::hint::black_box(lsx::vmod_du(a, a));
        core::hint::black_box(lsx::vsll_b(a, a));
        core::hint::black_box(lsx::vsll_h(a, a));
        core::hint::black_box(lsx::vsll_w(a, a));
        core::hint::black_box(lsx::vsll_d(a, a));
        core::hint::black_box(lsx::vsrl_b(a, a));
        core::hint::black_box(lsx::vsrl_h(a, a));
        core::hint::black_box(lsx::vsrl_w(a, a));
        core::hint::black_box(lsx::vsrl_d(a, a));
        core::hint::black_box(lsx::vsra_b(a, a));
        core::hint::black_box(lsx::vsra_h(a, a));
        core::hint::black_box(lsx::vsra_w(a, a));
        core::hint::black_box(lsx::vsra_d(a, a));
        core::hint::black_box(lsx::vrotr_b(a, a));
        core::hint::black_box(lsx::vrotr_h(a, a));
        core::hint::black_box(lsx::vrotr_w(a, a));
        core::hint::black_box(lsx::vrotr_d(a, a));
        core::hint::black_box(lsx::vsrlr_b(a, a));
        core::hint::black_box(lsx::vsrlr_h(a, a));
        core::hint::black_box(lsx::vsrlr_w(a, a));
        core::hint::black_box(lsx::vsrlr_d(a, a));
        core::hint::black_box(lsx::vsrar_b(a, a));
        core::hint::black_box(lsx::vsrar_h(a, a));
        core::hint::black_box(lsx::vsrar_w(a, a));
        core::hint::black_box(lsx::vsrar_d(a, a));
        core::hint::black_box(lsx::vsrln_b_h(a, a));
        core::hint::black_box(lsx::vsrln_h_w(a, a));
        core::hint::black_box(lsx::vsrln_w_d(a, a));
        core::hint::black_box(lsx::vsran_b_h(a, a));
        core::hint::black_box(lsx::vsran_h_w(a, a));
        core::hint::black_box(lsx::vsran_w_d(a, a));
        core::hint::black_box(lsx::vsrlrn_b_h(a, a));
        core::hint::black_box(lsx::vsrlrn_h_w(a, a));
        core::hint::black_box(lsx::vsrlrn_w_d(a, a));
        core::hint::black_box(lsx::vsrarn_b_h(a, a));
        core::hint::black_box(lsx::vsrarn_h_w(a, a));
        core::hint::black_box(lsx::vsrarn_w_d(a, a));
        core::hint::black_box(lsx::vssrln_b_h(a, a));
        core::hint::black_box(lsx::vssrln_h_w(a, a));
        core::hint::black_box(lsx::vssran_b_h(a, a));
        core::hint::black_box(lsx::vssran_h_w(a, a));
        core::hint::black_box(lsx::vssran_w_d(a, a));
        core::hint::black_box(lsx::vssrlrn_b_h(a, a));
        core::hint::black_box(lsx::vssrlrn_h_w(a, a));
        core::hint::black_box(lsx::vssrarn_b_h(a, a));
        core::hint::black_box(lsx::vssrarn_h_w(a, a));
        core::hint::black_box(lsx::vssrln_bu_h(a, a));
        core::hint::black_box(lsx::vssrln_hu_w(a, a));
        core::hint::black_box(lsx::vssrln_wu_d(a, a));
        core::hint::black_box(lsx::vssran_bu_h(a, a));
        core::hint::black_box(lsx::vssran_hu_w(a, a));
        core::hint::black_box(lsx::vssran_wu_d(a, a));
        core::hint::black_box(lsx::vssrlrn_bu_h(a, a));
        core::hint::black_box(lsx::vssrlrn_hu_w(a, a));
        core::hint::black_box(lsx::vssrlrn_wu_d(a, a));
        core::hint::black_box(lsx::vssrarn_bu_h(a, a));
        core::hint::black_box(lsx::vssrarn_hu_w(a, a));
        core::hint::black_box(lsx::vssrarn_wu_d(a, a));
        core::hint::black_box(lsx::vbitclr_b(a, a));
        core::hint::black_box(lsx::vbitclr_h(a, a));
        core::hint::black_box(lsx::vbitclr_w(a, a));
        core::hint::black_box(lsx::vbitclr_d(a, a));
        core::hint::black_box(lsx::vbitset_b(a, a));
        core::hint::black_box(lsx::vbitset_h(a, a));
        core::hint::black_box(lsx::vbitset_w(a, a));
        core::hint::black_box(lsx::vbitset_d(a, a));
        core::hint::black_box(lsx::vbitrev_b(a, a));
        core::hint::black_box(lsx::vbitrev_h(a, a));
        core::hint::black_box(lsx::vbitrev_w(a, a));
        core::hint::black_box(lsx::vbitrev_d(a, a));
        core::hint::black_box(lsx::vpackev_b(a, a));
        core::hint::black_box(lsx::vpackev_h(a, a));
        core::hint::black_box(lsx::vpackev_w(a, a));
        core::hint::black_box(lsx::vpackev_d(a, a));
        core::hint::black_box(lsx::vpackod_b(a, a));
        core::hint::black_box(lsx::vpackod_h(a, a));
        core::hint::black_box(lsx::vpackod_w(a, a));
        core::hint::black_box(lsx::vpackod_d(a, a));
        core::hint::black_box(lsx::vilvl_b(a, a));
        core::hint::black_box(lsx::vilvl_h(a, a));
        core::hint::black_box(lsx::vilvl_w(a, a));
        core::hint::black_box(lsx::vilvl_d(a, a));
        core::hint::black_box(lsx::vilvh_b(a, a));
        core::hint::black_box(lsx::vilvh_h(a, a));
        core::hint::black_box(lsx::vilvh_w(a, a));
        core::hint::black_box(lsx::vilvh_d(a, a));
        core::hint::black_box(lsx::vpickev_b(a, a));
        core::hint::black_box(lsx::vpickev_h(a, a));
        core::hint::black_box(lsx::vpickev_w(a, a));
        core::hint::black_box(lsx::vpickev_d(a, a));
        core::hint::black_box(lsx::vpickod_b(a, a));
        core::hint::black_box(lsx::vpickod_h(a, a));
        core::hint::black_box(lsx::vpickod_w(a, a));
        core::hint::black_box(lsx::vpickod_d(a, a));
        core::hint::black_box(lsx::vand_v(a, a));
        core::hint::black_box(lsx::vor_v(a, a));
        core::hint::black_box(lsx::vxor_v(a, a));
        core::hint::black_box(lsx::vnor_v(a, a));
        core::hint::black_box(lsx::vandn_v(a, a));
        core::hint::black_box(lsx::vorn_v(a, a));
        core::hint::black_box(lsx::vfrstp_b(a, a, a));
        core::hint::black_box(lsx::vfrstp_h(a, a, a));
        core::hint::black_box(lsx::vadd_q(a, a));
        core::hint::black_box(lsx::vsub_q(a, a));
        core::hint::black_box(lsx::vsigncov_b(a, a));
        core::hint::black_box(lsx::vsigncov_h(a, a));
        core::hint::black_box(lsx::vsigncov_w(a, a));
        core::hint::black_box(lsx::vsigncov_d(a, a));
        core::hint::black_box(lsx::vfcvt_h_s(a, a));
        core::hint::black_box(lsx::vshuf_h(a, a, a));
        core::hint::black_box(lsx::vshuf_w(a, a, a));
        core::hint::black_box(lsx::vshuf_d(a, a, a));
        core::hint::black_box(lsx::vclo_b(a));
        core::hint::black_box(lsx::vclo_h(a));
        core::hint::black_box(lsx::vclo_w(a));
        core::hint::black_box(lsx::vclo_d(a));
        core::hint::black_box(lsx::vclz_b(a));
        core::hint::black_box(lsx::vclz_h(a));
        core::hint::black_box(lsx::vclz_w(a));
        core::hint::black_box(lsx::vclz_d(a));
        core::hint::black_box(lsx::vpcnt_b(a));
        core::hint::black_box(lsx::vpcnt_h(a));
        core::hint::black_box(lsx::vpcnt_w(a));
        core::hint::black_box(lsx::vpcnt_d(a));
        core::hint::black_box(lsx::vmskltz_b(a));
        core::hint::black_box(lsx::vmskltz_h(a));
        core::hint::black_box(lsx::vmskltz_w(a));
        core::hint::black_box(lsx::vmskltz_d(a));
        core::hint::black_box(lsx::vmskgez_b(a));
        core::hint::black_box(lsx::vmsknz_b(a));
        core::hint::black_box(lsx::vfclass_s(a));
        core::hint::black_box(lsx::vfclass_d(a));
        core::hint::black_box(lsx::vftintl_l_s(a));
        core::hint::black_box(lsx::vftinth_l_s(a));
        core::hint::black_box(lsx::vftintrml_l_s(a));
        core::hint::black_box(lsx::vftintrmh_l_s(a));
        core::hint::black_box(lsx::vftintrpl_l_s(a));
        core::hint::black_box(lsx::vftintrph_l_s(a));
        core::hint::black_box(lsx::vftintrzl_l_s(a));
        core::hint::black_box(lsx::vftintrzh_l_s(a));
        core::hint::black_box(lsx::vftintrnel_l_s(a));
        core::hint::black_box(lsx::vftintrneh_l_s(a));
        core::hint::black_box(lsx::vreplgr2vr_b(1));
        core::hint::black_box(lsx::vreplgr2vr_h(1));
        core::hint::black_box(lsx::vreplgr2vr_w(1));
        core::hint::black_box(lsx::vreplgr2vr_d(1));
        core::hint::black_box(lsx::vextl_q_d(a));
        core::hint::black_box(lasx::xvfcmp_caf_s(va, va));
        core::hint::black_box(lasx::xvfcmp_saf_s(va, va));
        core::hint::black_box(lasx::xvfcmp_clt_s(va, va));
        core::hint::black_box(lasx::xvfcmp_slt_s(va, va));
        core::hint::black_box(lasx::xvfcmp_ceq_s(va, va));
        core::hint::black_box(lasx::xvfcmp_seq_s(va, va));
        core::hint::black_box(lasx::xvfcmp_cle_s(va, va));
        core::hint::black_box(lasx::xvfcmp_sle_s(va, va));
        core::hint::black_box(lasx::xvfcmp_cun_s(va, va));
        core::hint::black_box(lasx::xvfcmp_sun_s(va, va));
        core::hint::black_box(lasx::xvfcmp_cult_s(va, va));
        core::hint::black_box(lasx::xvfcmp_sult_s(va, va));
        core::hint::black_box(lasx::xvfcmp_cueq_s(va, va));
        core::hint::black_box(lasx::xvfcmp_sueq_s(va, va));
        core::hint::black_box(lasx::xvfcmp_cule_s(va, va));
        core::hint::black_box(lasx::xvfcmp_sule_s(va, va));
        core::hint::black_box(lasx::xvfcmp_cne_s(va, va));
        core::hint::black_box(lasx::xvfcmp_sne_s(va, va));
        core::hint::black_box(lasx::xvfcmp_cor_s(va, va));
        core::hint::black_box(lasx::xvfcmp_sor_s(va, va));
        core::hint::black_box(lasx::xvfcmp_cune_s(va, va));
        core::hint::black_box(lasx::xvfcmp_sune_s(va, va));
        core::hint::black_box(lasx::xvfcmp_caf_d(va, va));
        core::hint::black_box(lasx::xvfcmp_saf_d(va, va));
        core::hint::black_box(lasx::xvfcmp_clt_d(va, va));
        core::hint::black_box(lasx::xvfcmp_slt_d(va, va));
        core::hint::black_box(lasx::xvfcmp_ceq_d(va, va));
        core::hint::black_box(lasx::xvfcmp_seq_d(va, va));
        core::hint::black_box(lasx::xvfcmp_cle_d(va, va));
        core::hint::black_box(lasx::xvfcmp_sle_d(va, va));
        core::hint::black_box(lasx::xvfcmp_cun_d(va, va));
        core::hint::black_box(lasx::xvfcmp_sun_d(va, va));
        core::hint::black_box(lasx::xvfcmp_cult_d(va, va));
        core::hint::black_box(lasx::xvfcmp_sult_d(va, va));
        core::hint::black_box(lasx::xvfcmp_cueq_d(va, va));
        core::hint::black_box(lasx::xvfcmp_sueq_d(va, va));
        core::hint::black_box(lasx::xvfcmp_cule_d(va, va));
        core::hint::black_box(lasx::xvfcmp_sule_d(va, va));
        core::hint::black_box(lasx::xvfcmp_cne_d(va, va));
        core::hint::black_box(lasx::xvfcmp_sne_d(va, va));
        core::hint::black_box(lasx::xvfcmp_cor_d(va, va));
        core::hint::black_box(lasx::xvfcmp_sor_d(va, va));
        core::hint::black_box(lasx::xvfcmp_cune_d(va, va));
        core::hint::black_box(lasx::xvfcmp_sune_d(va, va));
        core::hint::black_box(lasx::xvbitsel_v(va, va, va));
        core::hint::black_box(lasx::xvshuf_b(va, va, va));
        core::hint::black_box(lasx::xvldx(storage.0.as_ptr(), 0));
        core::hint::black_box(lasx::xvstx(va, out.as_mut_ptr(), 0));
        core::hint::black_box(lasx::xvseq_b(va, va));
        core::hint::black_box(lasx::xvseq_h(va, va));
        core::hint::black_box(lasx::xvseq_w(va, va));
        core::hint::black_box(lasx::xvseq_d(va, va));
        core::hint::black_box(lasx::xvsle_b(va, va));
        core::hint::black_box(lasx::xvsle_h(va, va));
        core::hint::black_box(lasx::xvsle_w(va, va));
        core::hint::black_box(lasx::xvsle_d(va, va));
        core::hint::black_box(lasx::xvsle_bu(va, va));
        core::hint::black_box(lasx::xvsle_hu(va, va));
        core::hint::black_box(lasx::xvsle_wu(va, va));
        core::hint::black_box(lasx::xvsle_du(va, va));
        core::hint::black_box(lasx::xvslt_b(va, va));
        core::hint::black_box(lasx::xvslt_h(va, va));
        core::hint::black_box(lasx::xvslt_w(va, va));
        core::hint::black_box(lasx::xvslt_d(va, va));
        core::hint::black_box(lasx::xvslt_bu(va, va));
        core::hint::black_box(lasx::xvslt_hu(va, va));
        core::hint::black_box(lasx::xvslt_wu(va, va));
        core::hint::black_box(lasx::xvslt_du(va, va));
        core::hint::black_box(lasx::xvaddwev_h_b(va, va));
        core::hint::black_box(lasx::xvaddwev_w_h(va, va));
        core::hint::black_box(lasx::xvaddwev_d_w(va, va));
        core::hint::black_box(lasx::xvaddwev_q_d(va, va));
        core::hint::black_box(lasx::xvsubwev_h_b(va, va));
        core::hint::black_box(lasx::xvsubwev_w_h(va, va));
        core::hint::black_box(lasx::xvsubwev_d_w(va, va));
        core::hint::black_box(lasx::xvsubwev_q_d(va, va));
        core::hint::black_box(lasx::xvaddwod_h_b(va, va));
        core::hint::black_box(lasx::xvaddwod_w_h(va, va));
        core::hint::black_box(lasx::xvaddwod_d_w(va, va));
        core::hint::black_box(lasx::xvaddwod_q_d(va, va));
        core::hint::black_box(lasx::xvsubwod_h_b(va, va));
        core::hint::black_box(lasx::xvsubwod_w_h(va, va));
        core::hint::black_box(lasx::xvsubwod_d_w(va, va));
        core::hint::black_box(lasx::xvsubwod_q_d(va, va));
        core::hint::black_box(lasx::xvaddwev_h_bu(va, va));
        core::hint::black_box(lasx::xvaddwev_w_hu(va, va));
        core::hint::black_box(lasx::xvaddwev_d_wu(va, va));
        core::hint::black_box(lasx::xvaddwev_q_du(va, va));
        core::hint::black_box(lasx::xvsubwev_h_bu(va, va));
        core::hint::black_box(lasx::xvsubwev_w_hu(va, va));
        core::hint::black_box(lasx::xvsubwev_d_wu(va, va));
        core::hint::black_box(lasx::xvsubwev_q_du(va, va));
        core::hint::black_box(lasx::xvaddwod_h_bu(va, va));
        core::hint::black_box(lasx::xvaddwod_w_hu(va, va));
        core::hint::black_box(lasx::xvaddwod_d_wu(va, va));
        core::hint::black_box(lasx::xvaddwod_q_du(va, va));
        core::hint::black_box(lasx::xvsubwod_h_bu(va, va));
        core::hint::black_box(lasx::xvsubwod_w_hu(va, va));
        core::hint::black_box(lasx::xvsubwod_d_wu(va, va));
        core::hint::black_box(lasx::xvsubwod_q_du(va, va));
        core::hint::black_box(lasx::xvaddwev_h_bu_b(va, va));
        core::hint::black_box(lasx::xvaddwev_w_hu_h(va, va));
        core::hint::black_box(lasx::xvaddwev_d_wu_w(va, va));
        core::hint::black_box(lasx::xvaddwev_q_du_d(va, va));
        core::hint::black_box(lasx::xvaddwod_h_bu_b(va, va));
        core::hint::black_box(lasx::xvaddwod_w_hu_h(va, va));
        core::hint::black_box(lasx::xvaddwod_d_wu_w(va, va));
        core::hint::black_box(lasx::xvaddwod_q_du_d(va, va));
        core::hint::black_box(lasx::xvsadd_b(va, va));
        core::hint::black_box(lasx::xvsadd_h(va, va));
        core::hint::black_box(lasx::xvsadd_w(va, va));
        core::hint::black_box(lasx::xvsadd_d(va, va));
        core::hint::black_box(lasx::xvssub_b(va, va));
        core::hint::black_box(lasx::xvssub_h(va, va));
        core::hint::black_box(lasx::xvssub_w(va, va));
        core::hint::black_box(lasx::xvssub_d(va, va));
        core::hint::black_box(lasx::xvsadd_bu(va, va));
        core::hint::black_box(lasx::xvsadd_hu(va, va));
        core::hint::black_box(lasx::xvsadd_wu(va, va));
        core::hint::black_box(lasx::xvsadd_du(va, va));
        core::hint::black_box(lasx::xvssub_bu(va, va));
        core::hint::black_box(lasx::xvssub_hu(va, va));
        core::hint::black_box(lasx::xvssub_wu(va, va));
        core::hint::black_box(lasx::xvssub_du(va, va));
        core::hint::black_box(lasx::xvhaddw_h_b(va, va));
        core::hint::black_box(lasx::xvhaddw_w_h(va, va));
        core::hint::black_box(lasx::xvhaddw_d_w(va, va));
        core::hint::black_box(lasx::xvhaddw_q_d(va, va));
        core::hint::black_box(lasx::xvhsubw_h_b(va, va));
        core::hint::black_box(lasx::xvhsubw_w_h(va, va));
        core::hint::black_box(lasx::xvhsubw_d_w(va, va));
        core::hint::black_box(lasx::xvhsubw_q_d(va, va));
        core::hint::black_box(lasx::xvhaddw_hu_bu(va, va));
        core::hint::black_box(lasx::xvhaddw_wu_hu(va, va));
        core::hint::black_box(lasx::xvhaddw_du_wu(va, va));
        core::hint::black_box(lasx::xvhaddw_qu_du(va, va));
        core::hint::black_box(lasx::xvhsubw_hu_bu(va, va));
        core::hint::black_box(lasx::xvhsubw_wu_hu(va, va));
        core::hint::black_box(lasx::xvhsubw_du_wu(va, va));
        core::hint::black_box(lasx::xvhsubw_qu_du(va, va));
        core::hint::black_box(lasx::xvadda_b(va, va));
        core::hint::black_box(lasx::xvadda_h(va, va));
        core::hint::black_box(lasx::xvadda_w(va, va));
        core::hint::black_box(lasx::xvadda_d(va, va));
        core::hint::black_box(lasx::xvabsd_b(va, va));
        core::hint::black_box(lasx::xvabsd_h(va, va));
        core::hint::black_box(lasx::xvabsd_w(va, va));
        core::hint::black_box(lasx::xvabsd_d(va, va));
        core::hint::black_box(lasx::xvabsd_bu(va, va));
        core::hint::black_box(lasx::xvabsd_hu(va, va));
        core::hint::black_box(lasx::xvabsd_wu(va, va));
        core::hint::black_box(lasx::xvabsd_du(va, va));
        core::hint::black_box(lasx::xvavg_b(va, va));
        core::hint::black_box(lasx::xvavg_h(va, va));
        core::hint::black_box(lasx::xvavg_w(va, va));
        core::hint::black_box(lasx::xvavg_d(va, va));
        core::hint::black_box(lasx::xvavg_bu(va, va));
        core::hint::black_box(lasx::xvavg_hu(va, va));
        core::hint::black_box(lasx::xvavg_wu(va, va));
        core::hint::black_box(lasx::xvavg_du(va, va));
        core::hint::black_box(lasx::xvavgr_b(va, va));
        core::hint::black_box(lasx::xvavgr_h(va, va));
        core::hint::black_box(lasx::xvavgr_w(va, va));
        core::hint::black_box(lasx::xvavgr_d(va, va));
        core::hint::black_box(lasx::xvavgr_bu(va, va));
        core::hint::black_box(lasx::xvavgr_hu(va, va));
        core::hint::black_box(lasx::xvavgr_wu(va, va));
        core::hint::black_box(lasx::xvavgr_du(va, va));
        core::hint::black_box(lasx::xvmax_b(va, va));
        core::hint::black_box(lasx::xvmax_h(va, va));
        core::hint::black_box(lasx::xvmax_w(va, va));
        core::hint::black_box(lasx::xvmax_d(va, va));
        core::hint::black_box(lasx::xvmin_b(va, va));
        core::hint::black_box(lasx::xvmin_h(va, va));
        core::hint::black_box(lasx::xvmin_w(va, va));
        core::hint::black_box(lasx::xvmin_d(va, va));
        core::hint::black_box(lasx::xvmax_bu(va, va));
        core::hint::black_box(lasx::xvmax_hu(va, va));
        core::hint::black_box(lasx::xvmax_wu(va, va));
        core::hint::black_box(lasx::xvmax_du(va, va));
        core::hint::black_box(lasx::xvmin_bu(va, va));
        core::hint::black_box(lasx::xvmin_hu(va, va));
        core::hint::black_box(lasx::xvmin_wu(va, va));
        core::hint::black_box(lasx::xvmin_du(va, va));
        core::hint::black_box(lasx::xvmul_b(va, va));
        core::hint::black_box(lasx::xvmul_d(va, va));
        core::hint::black_box(lasx::xvmuh_b(va, va));
        core::hint::black_box(lasx::xvmuh_h(va, va));
        core::hint::black_box(lasx::xvmuh_w(va, va));
        core::hint::black_box(lasx::xvmuh_d(va, va));
        core::hint::black_box(lasx::xvmuh_bu(va, va));
        core::hint::black_box(lasx::xvmuh_hu(va, va));
        core::hint::black_box(lasx::xvmuh_wu(va, va));
        core::hint::black_box(lasx::xvmuh_du(va, va));
        core::hint::black_box(lasx::xvmulwev_h_b(va, va));
        core::hint::black_box(lasx::xvmulwev_w_h(va, va));
        core::hint::black_box(lasx::xvmulwev_d_w(va, va));
        core::hint::black_box(lasx::xvmulwev_q_d(va, va));
        core::hint::black_box(lasx::xvmulwod_h_b(va, va));
        core::hint::black_box(lasx::xvmulwod_w_h(va, va));
        core::hint::black_box(lasx::xvmulwod_d_w(va, va));
        core::hint::black_box(lasx::xvmulwod_q_d(va, va));
        core::hint::black_box(lasx::xvmulwev_h_bu(va, va));
        core::hint::black_box(lasx::xvmulwev_w_hu(va, va));
        core::hint::black_box(lasx::xvmulwev_d_wu(va, va));
        core::hint::black_box(lasx::xvmulwev_q_du(va, va));
        core::hint::black_box(lasx::xvmulwod_h_bu(va, va));
        core::hint::black_box(lasx::xvmulwod_w_hu(va, va));
        core::hint::black_box(lasx::xvmulwod_d_wu(va, va));
        core::hint::black_box(lasx::xvmulwod_q_du(va, va));
        core::hint::black_box(lasx::xvmulwev_h_bu_b(va, va));
        core::hint::black_box(lasx::xvmulwev_w_hu_h(va, va));
        core::hint::black_box(lasx::xvmulwev_d_wu_w(va, va));
        core::hint::black_box(lasx::xvmulwev_q_du_d(va, va));
        core::hint::black_box(lasx::xvmulwod_h_bu_b(va, va));
        core::hint::black_box(lasx::xvmulwod_w_hu_h(va, va));
        core::hint::black_box(lasx::xvmulwod_d_wu_w(va, va));
        core::hint::black_box(lasx::xvmulwod_q_du_d(va, va));
        core::hint::black_box(lasx::xvmadd_b(va, va, va));
        core::hint::black_box(lasx::xvmadd_h(va, va, va));
        core::hint::black_box(lasx::xvmadd_w(va, va, va));
        core::hint::black_box(lasx::xvmadd_d(va, va, va));
        core::hint::black_box(lasx::xvmsub_b(va, va, va));
        core::hint::black_box(lasx::xvmsub_h(va, va, va));
        core::hint::black_box(lasx::xvmsub_w(va, va, va));
        core::hint::black_box(lasx::xvmsub_d(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_h_b(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_w_h(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_d_w(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_q_d(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_h_b(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_w_h(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_d_w(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_q_d(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_h_bu(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_w_hu(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_d_wu(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_q_du(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_h_bu(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_w_hu(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_d_wu(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_q_du(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_h_bu_b(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_w_hu_h(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_d_wu_w(va, va, va));
        core::hint::black_box(lasx::xvmaddwev_q_du_d(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_h_bu_b(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_w_hu_h(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_d_wu_w(va, va, va));
        core::hint::black_box(lasx::xvmaddwod_q_du_d(va, va, va));
        core::hint::black_box(lasx::xvdiv_b(va, va));
        core::hint::black_box(lasx::xvdiv_h(va, va));
        core::hint::black_box(lasx::xvdiv_w(va, va));
        core::hint::black_box(lasx::xvdiv_d(va, va));
        core::hint::black_box(lasx::xvmod_b(va, va));
        core::hint::black_box(lasx::xvmod_h(va, va));
        core::hint::black_box(lasx::xvmod_w(va, va));
        core::hint::black_box(lasx::xvmod_d(va, va));
        core::hint::black_box(lasx::xvdiv_bu(va, va));
        core::hint::black_box(lasx::xvdiv_hu(va, va));
        core::hint::black_box(lasx::xvdiv_wu(va, va));
        core::hint::black_box(lasx::xvdiv_du(va, va));
        core::hint::black_box(lasx::xvmod_bu(va, va));
        core::hint::black_box(lasx::xvmod_hu(va, va));
        core::hint::black_box(lasx::xvmod_wu(va, va));
        core::hint::black_box(lasx::xvmod_du(va, va));
        core::hint::black_box(lasx::xvsll_b(va, va));
        core::hint::black_box(lasx::xvsll_h(va, va));
        core::hint::black_box(lasx::xvsll_w(va, va));
        core::hint::black_box(lasx::xvsll_d(va, va));
        core::hint::black_box(lasx::xvsrl_b(va, va));
        core::hint::black_box(lasx::xvsrl_h(va, va));
        core::hint::black_box(lasx::xvsrl_w(va, va));
        core::hint::black_box(lasx::xvsrl_d(va, va));
        core::hint::black_box(lasx::xvsra_b(va, va));
        core::hint::black_box(lasx::xvsra_h(va, va));
        core::hint::black_box(lasx::xvsra_w(va, va));
        core::hint::black_box(lasx::xvsra_d(va, va));
        core::hint::black_box(lasx::xvrotr_b(va, va));
        core::hint::black_box(lasx::xvrotr_h(va, va));
        core::hint::black_box(lasx::xvrotr_w(va, va));
        core::hint::black_box(lasx::xvrotr_d(va, va));
        core::hint::black_box(lasx::xvsrlr_b(va, va));
        core::hint::black_box(lasx::xvsrlr_h(va, va));
        core::hint::black_box(lasx::xvsrlr_w(va, va));
        core::hint::black_box(lasx::xvsrlr_d(va, va));
        core::hint::black_box(lasx::xvsrar_b(va, va));
        core::hint::black_box(lasx::xvsrar_h(va, va));
        core::hint::black_box(lasx::xvsrar_w(va, va));
        core::hint::black_box(lasx::xvsrar_d(va, va));
        core::hint::black_box(lasx::xvsrln_b_h(va, va));
        core::hint::black_box(lasx::xvsrln_h_w(va, va));
        core::hint::black_box(lasx::xvsrln_w_d(va, va));
        core::hint::black_box(lasx::xvsran_b_h(va, va));
        core::hint::black_box(lasx::xvsran_h_w(va, va));
        core::hint::black_box(lasx::xvsran_w_d(va, va));
        core::hint::black_box(lasx::xvsrlrn_b_h(va, va));
        core::hint::black_box(lasx::xvsrlrn_h_w(va, va));
        core::hint::black_box(lasx::xvsrlrn_w_d(va, va));
        core::hint::black_box(lasx::xvsrarn_b_h(va, va));
        core::hint::black_box(lasx::xvsrarn_h_w(va, va));
        core::hint::black_box(lasx::xvsrarn_w_d(va, va));
        core::hint::black_box(lasx::xvssrln_b_h(va, va));
        core::hint::black_box(lasx::xvssrln_h_w(va, va));
        core::hint::black_box(lasx::xvssrln_w_d(va, va));
        core::hint::black_box(lasx::xvssran_b_h(va, va));
        core::hint::black_box(lasx::xvssran_h_w(va, va));
        core::hint::black_box(lasx::xvssran_w_d(va, va));
        core::hint::black_box(lasx::xvssrlrn_b_h(va, va));
        core::hint::black_box(lasx::xvssrlrn_h_w(va, va));
        core::hint::black_box(lasx::xvssrlrn_w_d(va, va));
        core::hint::black_box(lasx::xvssrarn_b_h(va, va));
        core::hint::black_box(lasx::xvssrarn_h_w(va, va));
        core::hint::black_box(lasx::xvssrarn_w_d(va, va));
        core::hint::black_box(lasx::xvssrln_bu_h(va, va));
        core::hint::black_box(lasx::xvssrln_hu_w(va, va));
        core::hint::black_box(lasx::xvssrln_wu_d(va, va));
        core::hint::black_box(lasx::xvssran_bu_h(va, va));
        core::hint::black_box(lasx::xvssran_hu_w(va, va));
        core::hint::black_box(lasx::xvssran_wu_d(va, va));
        core::hint::black_box(lasx::xvssrlrn_bu_h(va, va));
        core::hint::black_box(lasx::xvssrlrn_hu_w(va, va));
        core::hint::black_box(lasx::xvssrlrn_wu_d(va, va));
        core::hint::black_box(lasx::xvssrarn_bu_h(va, va));
        core::hint::black_box(lasx::xvssrarn_hu_w(va, va));
        core::hint::black_box(lasx::xvssrarn_wu_d(va, va));
        core::hint::black_box(lasx::xvbitclr_b(va, va));
        core::hint::black_box(lasx::xvbitclr_h(va, va));
        core::hint::black_box(lasx::xvbitclr_w(va, va));
        core::hint::black_box(lasx::xvbitclr_d(va, va));
        core::hint::black_box(lasx::xvbitset_b(va, va));
        core::hint::black_box(lasx::xvbitset_h(va, va));
        core::hint::black_box(lasx::xvbitset_w(va, va));
        core::hint::black_box(lasx::xvbitset_d(va, va));
        core::hint::black_box(lasx::xvbitrev_b(va, va));
        core::hint::black_box(lasx::xvbitrev_h(va, va));
        core::hint::black_box(lasx::xvbitrev_w(va, va));
        core::hint::black_box(lasx::xvbitrev_d(va, va));
        core::hint::black_box(lasx::xvpackev_b(va, va));
        core::hint::black_box(lasx::xvpackev_h(va, va));
        core::hint::black_box(lasx::xvpackev_w(va, va));
        core::hint::black_box(lasx::xvpackev_d(va, va));
        core::hint::black_box(lasx::xvpackod_b(va, va));
        core::hint::black_box(lasx::xvpackod_h(va, va));
        core::hint::black_box(lasx::xvpackod_w(va, va));
        core::hint::black_box(lasx::xvpackod_d(va, va));
        core::hint::black_box(lasx::xvilvl_b(va, va));
        core::hint::black_box(lasx::xvilvl_h(va, va));
        core::hint::black_box(lasx::xvilvl_w(va, va));
        core::hint::black_box(lasx::xvilvl_d(va, va));
        core::hint::black_box(lasx::xvilvh_b(va, va));
        core::hint::black_box(lasx::xvilvh_h(va, va));
        core::hint::black_box(lasx::xvilvh_w(va, va));
        core::hint::black_box(lasx::xvilvh_d(va, va));
        core::hint::black_box(lasx::xvpickev_b(va, va));
        core::hint::black_box(lasx::xvpickev_h(va, va));
        core::hint::black_box(lasx::xvpickev_w(va, va));
        core::hint::black_box(lasx::xvpickev_d(va, va));
        core::hint::black_box(lasx::xvpickod_b(va, va));
        core::hint::black_box(lasx::xvpickod_h(va, va));
        core::hint::black_box(lasx::xvpickod_w(va, va));
        core::hint::black_box(lasx::xvpickod_d(va, va));
        core::hint::black_box(lasx::xvreplve_b(va, 0));
        core::hint::black_box(lasx::xvreplve_h(va, 0));
        core::hint::black_box(lasx::xvreplve_w(va, 0));
        core::hint::black_box(lasx::xvreplve_d(va, 0));
        core::hint::black_box(lasx::xvand_v(va, va));
        core::hint::black_box(lasx::xvor_v(va, va));
        core::hint::black_box(lasx::xvxor_v(va, va));
        core::hint::black_box(lasx::xvnor_v(va, va));
        core::hint::black_box(lasx::xvandn_v(va, va));
        core::hint::black_box(lasx::xvorn_v(va, va));
        core::hint::black_box(lasx::xvfrstp_b(va, va, va));
        core::hint::black_box(lasx::xvfrstp_h(va, va, va));
        core::hint::black_box(lasx::xvadd_q(va, va));
        core::hint::black_box(lasx::xvsub_q(va, va));
        core::hint::black_box(lasx::xvsigncov_b(va, va));
        core::hint::black_box(lasx::xvsigncov_h(va, va));
        core::hint::black_box(lasx::xvsigncov_w(va, va));
        core::hint::black_box(lasx::xvsigncov_d(va, va));
        core::hint::black_box(lasx::xvfcvt_h_s(va, va));
        core::hint::black_box(lasx::xvftint_w_d(va, va));
        core::hint::black_box(lasx::xvftintrm_w_d(va, va));
        core::hint::black_box(lasx::xvftintrp_w_d(va, va));
        core::hint::black_box(lasx::xvftintrz_w_d(va, va));
        core::hint::black_box(lasx::xvftintrne_w_d(va, va));
        core::hint::black_box(lasx::xvshuf_h(va, va, va));
        core::hint::black_box(lasx::xvshuf_w(va, va, va));
        core::hint::black_box(lasx::xvshuf_d(va, va, va));
        core::hint::black_box(lasx::xvperm_w(va, va));
        core::hint::black_box(lasx::xvclo_b(va));
        core::hint::black_box(lasx::xvclo_h(va));
        core::hint::black_box(lasx::xvclo_w(va));
        core::hint::black_box(lasx::xvclo_d(va));
        core::hint::black_box(lasx::xvclz_b(va));
        core::hint::black_box(lasx::xvclz_h(va));
        core::hint::black_box(lasx::xvclz_w(va));
        core::hint::black_box(lasx::xvclz_d(va));
        core::hint::black_box(lasx::xvpcnt_b(va));
        core::hint::black_box(lasx::xvpcnt_h(va));
        core::hint::black_box(lasx::xvpcnt_w(va));
        core::hint::black_box(lasx::xvpcnt_d(va));
        core::hint::black_box(lasx::xvneg_w(va));
        core::hint::black_box(lasx::xvmskltz_b(va));
        core::hint::black_box(lasx::xvmskltz_h(va));
        core::hint::black_box(lasx::xvmskltz_w(va));
        core::hint::black_box(lasx::xvmskltz_d(va));
        core::hint::black_box(lasx::xvmskgez_b(va));
        core::hint::black_box(lasx::xvmsknz_b(va));
        core::hint::black_box(lasx::xvfclass_s(va));
        core::hint::black_box(lasx::xvfclass_d(va));
        core::hint::black_box(lasx::xvftint_w_s(va));
        core::hint::black_box(lasx::xvftint_l_d(va));
        core::hint::black_box(lasx::xvftintrm_w_s(va));
        core::hint::black_box(lasx::xvftintrm_l_d(va));
        core::hint::black_box(lasx::xvftintrp_w_s(va));
        core::hint::black_box(lasx::xvftintrp_l_d(va));
        core::hint::black_box(lasx::xvftintrz_w_s(va));
        core::hint::black_box(lasx::xvftintrz_l_d(va));
        core::hint::black_box(lasx::xvftintrne_w_s(va));
        core::hint::black_box(lasx::xvftintrne_l_d(va));
        core::hint::black_box(lasx::xvftint_wu_s(va));
        core::hint::black_box(lasx::xvftint_lu_d(va));
        core::hint::black_box(lasx::xvftintrz_wu_s(va));
        core::hint::black_box(lasx::xvftintrz_lu_d(va));
        core::hint::black_box(lasx::xvftintl_l_s(va));
        core::hint::black_box(lasx::xvftinth_l_s(va));
        core::hint::black_box(lasx::xvftintrml_l_s(va));
        core::hint::black_box(lasx::xvftintrmh_l_s(va));
        core::hint::black_box(lasx::xvftintrpl_l_s(va));
        core::hint::black_box(lasx::xvftintrph_l_s(va));
        core::hint::black_box(lasx::xvftintrzl_l_s(va));
        core::hint::black_box(lasx::xvftintrzh_l_s(va));
        core::hint::black_box(lasx::xvftintrnel_l_s(va));
        core::hint::black_box(lasx::xvftintrneh_l_s(va));
        core::hint::black_box(lasx::xvexth_h_b(va));
        core::hint::black_box(lasx::xvexth_w_h(va));
        core::hint::black_box(lasx::xvexth_d_w(va));
        core::hint::black_box(lasx::xvexth_q_d(va));
        core::hint::black_box(lasx::xvexth_hu_bu(va));
        core::hint::black_box(lasx::xvexth_wu_hu(va));
        core::hint::black_box(lasx::xvexth_du_wu(va));
        core::hint::black_box(lasx::xvexth_qu_du(va));
        core::hint::black_box(lasx::xvreplgr2vr_b(1));
        core::hint::black_box(lasx::xvreplgr2vr_h(1));
        core::hint::black_box(lasx::xvreplgr2vr_w(1));
        core::hint::black_box(lasx::xvreplgr2vr_d(1));
        core::hint::black_box(lasx::vext2xv_w_b(va));
        core::hint::black_box(lasx::vext2xv_d_b(va));
        core::hint::black_box(lasx::vext2xv_w_h(va));
        core::hint::black_box(lasx::vext2xv_d_h(va));
        core::hint::black_box(lasx::vext2xv_d_w(va));
        core::hint::black_box(lasx::vext2xv_hu_bu(va));
        core::hint::black_box(lasx::vext2xv_wu_bu(va));
        core::hint::black_box(lasx::vext2xv_du_bu(va));
        core::hint::black_box(lasx::vext2xv_wu_hu(va));
        core::hint::black_box(lasx::vext2xv_du_hu(va));
        core::hint::black_box(lasx::vext2xv_du_wu(va));
        core::hint::black_box(lasx::xvreplve0_b(va));
        core::hint::black_box(lasx::xvreplve0_h(va));
        core::hint::black_box(lasx::xvreplve0_w(va));
        core::hint::black_box(lasx::xvreplve0_d(va));
        core::hint::black_box(lasx::xvreplve0_q(va));
        core::hint::black_box(lasx::xvextl_q_d(va));
        core::hint::black_box(lasx::xvextl_qu_du(va));
        core::hint::black_box(lsx::vseqi_b::<1>(a));
        core::hint::black_box(lsx::vslei_b::<1>(a));
        core::hint::black_box(lsx::vslei_bu::<1>(a));
        core::hint::black_box(lsx::vslti_b::<1>(a));
        core::hint::black_box(lsx::vslti_bu::<1>(a));
        core::hint::black_box(lsx::vslli_b::<1>(a));
        core::hint::black_box(lsx::vsrli_b::<1>(a));
        core::hint::black_box(lsx::vsrai_b::<1>(a));
        core::hint::black_box(lsx::vmaxi_b::<1>(a));
        core::hint::black_box(lsx::vmaxi_bu::<1>(a));
        core::hint::black_box(lsx::vmini_b::<1>(a));
        core::hint::black_box(lsx::vmini_bu::<1>(a));
        core::hint::black_box(lsx::vseqi_h::<1>(a));
        core::hint::black_box(lsx::vslei_h::<1>(a));
        core::hint::black_box(lsx::vslei_hu::<1>(a));
        core::hint::black_box(lsx::vslti_h::<1>(a));
        core::hint::black_box(lsx::vslti_hu::<1>(a));
        core::hint::black_box(lsx::vslli_h::<1>(a));
        core::hint::black_box(lsx::vsrli_h::<1>(a));
        core::hint::black_box(lsx::vsrai_h::<1>(a));
        core::hint::black_box(lsx::vmaxi_h::<1>(a));
        core::hint::black_box(lsx::vmaxi_hu::<1>(a));
        core::hint::black_box(lsx::vmini_h::<1>(a));
        core::hint::black_box(lsx::vmini_hu::<1>(a));
        core::hint::black_box(lsx::vseqi_w::<1>(a));
        core::hint::black_box(lsx::vslei_w::<1>(a));
        core::hint::black_box(lsx::vslei_wu::<1>(a));
        core::hint::black_box(lsx::vslti_w::<1>(a));
        core::hint::black_box(lsx::vslti_wu::<1>(a));
        core::hint::black_box(lsx::vslli_w::<1>(a));
        core::hint::black_box(lsx::vsrli_w::<1>(a));
        core::hint::black_box(lsx::vsrai_w::<1>(a));
        core::hint::black_box(lsx::vmaxi_w::<1>(a));
        core::hint::black_box(lsx::vmaxi_wu::<1>(a));
        core::hint::black_box(lsx::vmini_w::<1>(a));
        core::hint::black_box(lsx::vmini_wu::<1>(a));
        core::hint::black_box(lsx::vseqi_d::<1>(a));
        core::hint::black_box(lsx::vslei_d::<1>(a));
        core::hint::black_box(lsx::vslei_du::<1>(a));
        core::hint::black_box(lsx::vslti_d::<1>(a));
        core::hint::black_box(lsx::vslti_du::<1>(a));
        core::hint::black_box(lsx::vslli_d::<1>(a));
        core::hint::black_box(lsx::vsrli_d::<1>(a));
        core::hint::black_box(lsx::vsrai_d::<1>(a));
        core::hint::black_box(lsx::vmaxi_d::<1>(a));
        core::hint::black_box(lsx::vmaxi_du::<1>(a));
        core::hint::black_box(lsx::vmini_d::<1>(a));
        core::hint::black_box(lsx::vmini_du::<1>(a));
        core::hint::black_box(lasx::xvseqi_b::<1>(va));
        core::hint::black_box(lasx::xvslei_b::<1>(va));
        core::hint::black_box(lasx::xvslei_bu::<1>(va));
        core::hint::black_box(lasx::xvslti_b::<1>(va));
        core::hint::black_box(lasx::xvslti_bu::<1>(va));
        core::hint::black_box(lasx::xvslli_b::<1>(va));
        core::hint::black_box(lasx::xvsrli_b::<1>(va));
        core::hint::black_box(lasx::xvsrai_b::<1>(va));
        core::hint::black_box(lasx::xvmaxi_b::<1>(va));
        core::hint::black_box(lasx::xvmaxi_bu::<1>(va));
        core::hint::black_box(lasx::xvmini_b::<1>(va));
        core::hint::black_box(lasx::xvmini_bu::<1>(va));
        core::hint::black_box(lasx::xvseqi_h::<1>(va));
        core::hint::black_box(lasx::xvslei_h::<1>(va));
        core::hint::black_box(lasx::xvslei_hu::<1>(va));
        core::hint::black_box(lasx::xvslti_h::<1>(va));
        core::hint::black_box(lasx::xvslti_hu::<1>(va));
        core::hint::black_box(lasx::xvslli_h::<1>(va));
        core::hint::black_box(lasx::xvsrli_h::<1>(va));
        core::hint::black_box(lasx::xvsrai_h::<1>(va));
        core::hint::black_box(lasx::xvmaxi_h::<1>(va));
        core::hint::black_box(lasx::xvmaxi_hu::<1>(va));
        core::hint::black_box(lasx::xvmini_h::<1>(va));
        core::hint::black_box(lasx::xvmini_hu::<1>(va));
        core::hint::black_box(lasx::xvseqi_w::<1>(va));
        core::hint::black_box(lasx::xvslei_w::<1>(va));
        core::hint::black_box(lasx::xvslei_wu::<1>(va));
        core::hint::black_box(lasx::xvslti_w::<1>(va));
        core::hint::black_box(lasx::xvslti_wu::<1>(va));
        core::hint::black_box(lasx::xvslli_w::<1>(va));
        core::hint::black_box(lasx::xvsrli_w::<1>(va));
        core::hint::black_box(lasx::xvsrai_w::<1>(va));
        core::hint::black_box(lasx::xvmaxi_w::<1>(va));
        core::hint::black_box(lasx::xvmaxi_wu::<1>(va));
        core::hint::black_box(lasx::xvmini_w::<1>(va));
        core::hint::black_box(lasx::xvmini_wu::<1>(va));
        core::hint::black_box(lasx::xvseqi_d::<1>(va));
        core::hint::black_box(lasx::xvslei_d::<1>(va));
        core::hint::black_box(lasx::xvslei_du::<1>(va));
        core::hint::black_box(lasx::xvslti_d::<1>(va));
        core::hint::black_box(lasx::xvslti_du::<1>(va));
        core::hint::black_box(lasx::xvslli_d::<1>(va));
        core::hint::black_box(lasx::xvsrli_d::<1>(va));
        core::hint::black_box(lasx::xvsrai_d::<1>(va));
        core::hint::black_box(lasx::xvmaxi_d::<1>(va));
        core::hint::black_box(lasx::xvmaxi_du::<1>(va));
        core::hint::black_box(lasx::xvmini_d::<1>(va));
        core::hint::black_box(lasx::xvmini_du::<1>(va));
        core::hint::black_box(lsx::vrotri_b::<1>(a));
        core::hint::black_box(lsx::vbitclri_b::<1>(a));
        core::hint::black_box(lsx::vbitseti_b::<1>(a));
        core::hint::black_box(lsx::vbitrevi_b::<1>(a));
        core::hint::black_box(lsx::vrotri_h::<1>(a));
        core::hint::black_box(lsx::vbitclri_h::<1>(a));
        core::hint::black_box(lsx::vbitseti_h::<1>(a));
        core::hint::black_box(lsx::vbitrevi_h::<1>(a));
        core::hint::black_box(lsx::vrotri_w::<1>(a));
        core::hint::black_box(lsx::vbitclri_w::<1>(a));
        core::hint::black_box(lsx::vbitseti_w::<1>(a));
        core::hint::black_box(lsx::vbitrevi_w::<1>(a));
        core::hint::black_box(lsx::vrotri_d::<1>(a));
        core::hint::black_box(lsx::vbitclri_d::<1>(a));
        core::hint::black_box(lsx::vbitseti_d::<1>(a));
        core::hint::black_box(lsx::vbitrevi_d::<1>(a));
        core::hint::black_box(lsx::vandi_b::<1>(a));
        core::hint::black_box(lsx::vori_b::<1>(a));
        core::hint::black_box(lsx::vxori_b::<1>(a));
        core::hint::black_box(lsx::vnori_b::<1>(a));
        core::hint::black_box(lasx::xvrotri_b::<1>(va));
        core::hint::black_box(lasx::xvbitclri_b::<1>(va));
        core::hint::black_box(lasx::xvbitseti_b::<1>(va));
        core::hint::black_box(lasx::xvbitrevi_b::<1>(va));
        core::hint::black_box(lasx::xvrotri_h::<1>(va));
        core::hint::black_box(lasx::xvbitclri_h::<1>(va));
        core::hint::black_box(lasx::xvbitseti_h::<1>(va));
        core::hint::black_box(lasx::xvbitrevi_h::<1>(va));
        core::hint::black_box(lasx::xvrotri_w::<1>(va));
        core::hint::black_box(lasx::xvbitclri_w::<1>(va));
        core::hint::black_box(lasx::xvbitseti_w::<1>(va));
        core::hint::black_box(lasx::xvbitrevi_w::<1>(va));
        core::hint::black_box(lasx::xvrotri_d::<1>(va));
        core::hint::black_box(lasx::xvbitclri_d::<1>(va));
        core::hint::black_box(lasx::xvbitseti_d::<1>(va));
        core::hint::black_box(lasx::xvbitrevi_d::<1>(va));
        core::hint::black_box(lasx::xvandi_b::<1>(va));
        core::hint::black_box(lasx::xvori_b::<1>(va));
        core::hint::black_box(lasx::xvxori_b::<1>(va));
        core::hint::black_box(lasx::xvnori_b::<1>(va));
        core::hint::black_box(lsx::vaddi_bu::<1>(a));
        core::hint::black_box(lsx::vsubi_bu::<1>(a));
        core::hint::black_box(lsx::vsrlri_b::<1>(a));
        core::hint::black_box(lsx::vsrari_b::<1>(a));
        core::hint::black_box(lsx::vaddi_hu::<1>(a));
        core::hint::black_box(lsx::vsubi_hu::<1>(a));
        core::hint::black_box(lsx::vsrlri_h::<1>(a));
        core::hint::black_box(lsx::vsrari_h::<1>(a));
        core::hint::black_box(lsx::vaddi_wu::<1>(a));
        core::hint::black_box(lsx::vsubi_wu::<1>(a));
        core::hint::black_box(lsx::vsrlri_w::<1>(a));
        core::hint::black_box(lsx::vsrari_w::<1>(a));
        core::hint::black_box(lsx::vaddi_du::<1>(a));
        core::hint::black_box(lsx::vsubi_du::<1>(a));
        core::hint::black_box(lsx::vsrlri_d::<1>(a));
        core::hint::black_box(lsx::vsrari_d::<1>(a));
        core::hint::black_box(lasx::xvaddi_bu::<1>(va));
        core::hint::black_box(lasx::xvsubi_bu::<1>(va));
        core::hint::black_box(lasx::xvsrlri_b::<1>(va));
        core::hint::black_box(lasx::xvsrari_b::<1>(va));
        core::hint::black_box(lasx::xvaddi_hu::<1>(va));
        core::hint::black_box(lasx::xvsubi_hu::<1>(va));
        core::hint::black_box(lasx::xvsrlri_h::<1>(va));
        core::hint::black_box(lasx::xvsrari_h::<1>(va));
        core::hint::black_box(lasx::xvaddi_wu::<1>(va));
        core::hint::black_box(lasx::xvsubi_wu::<1>(va));
        core::hint::black_box(lasx::xvsrlri_w::<1>(va));
        core::hint::black_box(lasx::xvsrari_w::<1>(va));
        core::hint::black_box(lasx::xvaddi_du::<1>(va));
        core::hint::black_box(lasx::xvsubi_du::<1>(va));
        core::hint::black_box(lasx::xvsrlri_d::<1>(va));
        core::hint::black_box(lasx::xvsrari_d::<1>(va));
        core::hint::black_box(lsx::vfrint_s(a));
        core::hint::black_box(lsx::vfrint_d(a));
        core::hint::black_box(lsx::vfrintrm_s(a));
        core::hint::black_box(lsx::vfrintrm_d(a));
        core::hint::black_box(lsx::vfrintrp_s(a));
        core::hint::black_box(lsx::vfrintrp_d(a));
        core::hint::black_box(lsx::vfrintrz_s(a));
        core::hint::black_box(lsx::vfrintrz_d(a));
        core::hint::black_box(lsx::vfrintrne_s(a));
        core::hint::black_box(lsx::vfrintrne_d(a));
        core::hint::black_box(lsx::vfcvtl_s_h(a));
        core::hint::black_box(lsx::vfcvth_s_h(a));
        core::hint::black_box(lasx::xvfadd_s(va, va));
        core::hint::black_box(lasx::xvfadd_d(va, va));
        core::hint::black_box(lasx::xvfsub_s(va, va));
        core::hint::black_box(lasx::xvfsub_d(va, va));
        core::hint::black_box(lasx::xvfmul_s(va, va));
        core::hint::black_box(lasx::xvfmul_d(va, va));
        core::hint::black_box(lasx::xvfdiv_s(va, va));
        core::hint::black_box(lasx::xvfdiv_d(va, va));
        core::hint::black_box(lasx::xvfmax_s(va, va));
        core::hint::black_box(lasx::xvfmax_d(va, va));
        core::hint::black_box(lasx::xvfmin_s(va, va));
        core::hint::black_box(lasx::xvfmin_d(va, va));
        core::hint::black_box(lasx::xvfmaxa_s(va, va));
        core::hint::black_box(lasx::xvfmaxa_d(va, va));
        core::hint::black_box(lasx::xvfmina_s(va, va));
        core::hint::black_box(lasx::xvfmina_d(va, va));
        core::hint::black_box(lasx::xvfcvt_s_d(va, va));
        core::hint::black_box(lasx::xvffint_s_l(va, va));
        core::hint::black_box(lasx::xvflogb_s(va));
        core::hint::black_box(lasx::xvflogb_d(va));
        core::hint::black_box(lasx::xvfsqrt_s(va));
        core::hint::black_box(lasx::xvfsqrt_d(va));
        core::hint::black_box(lasx::xvfrecip_s(va));
        core::hint::black_box(lasx::xvfrecip_d(va));
        core::hint::black_box(lasx::xvfrsqrt_s(va));
        core::hint::black_box(lasx::xvfrsqrt_d(va));
        core::hint::black_box(lasx::xvfrint_s(va));
        core::hint::black_box(lasx::xvfrint_d(va));
        core::hint::black_box(lasx::xvfrintrm_s(va));
        core::hint::black_box(lasx::xvfrintrm_d(va));
        core::hint::black_box(lasx::xvfrintrp_s(va));
        core::hint::black_box(lasx::xvfrintrp_d(va));
        core::hint::black_box(lasx::xvfrintrz_s(va));
        core::hint::black_box(lasx::xvfrintrz_d(va));
        core::hint::black_box(lasx::xvfrintrne_s(va));
        core::hint::black_box(lasx::xvfrintrne_d(va));
        core::hint::black_box(lasx::xvfcvtl_s_h(va));
        core::hint::black_box(lasx::xvfcvth_s_h(va));
        core::hint::black_box(lasx::xvfcvtl_d_s(va));
        core::hint::black_box(lasx::xvfcvth_d_s(va));
        core::hint::black_box(lasx::xvffint_s_w(va));
        core::hint::black_box(lasx::xvffint_s_wu(va));
        core::hint::black_box(lasx::xvffint_d_l(va));
        core::hint::black_box(lasx::xvffint_d_lu(va));
        core::hint::black_box(lasx::xvffintl_d_w(va));
        core::hint::black_box(lasx::xvffinth_d_w(va));
        core::hint::black_box(lsx::vflogb_s(a));
        core::hint::black_box(lsx::vflogb_d(a));
        core::hint::black_box(lsx::vfmaxa_s(a, a));
        core::hint::black_box(lsx::vfmaxa_d(a, a));
        core::hint::black_box(lsx::vfmina_s(a, a));
        core::hint::black_box(lsx::vfmina_d(a, a));
        core::hint::black_box(lsx::vfrecip_s(a));
        core::hint::black_box(lsx::vfrecip_d(a));
        core::hint::black_box(lsx::vfrsqrt_s(a));
        core::hint::black_box(lsx::vfrsqrt_d(a));
        core::hint::black_box(lsx::vfsqrt_d(a));
        core::hint::black_box(lsx::vldrepl_b(storage.0.as_ptr(), 0));
        core::hint::black_box(lsx::vldrepl_h(storage.0.as_ptr(), 0));
        core::hint::black_box(lsx::vldrepl_w(storage.0.as_ptr(), 0));
        core::hint::black_box(lsx::vldrepl_d(storage.0.as_ptr(), 0));
        core::hint::black_box(lsx::vstelm_b::<0>(a, st.as_mut_ptr(), 0));
        core::hint::black_box(lsx::vstelm_h::<0>(a, st.as_mut_ptr(), 0));
        core::hint::black_box(lsx::vstelm_w::<0>(a, st.as_mut_ptr(), 0));
        core::hint::black_box(lsx::vstelm_d::<0>(a, st.as_mut_ptr(), 0));
        core::hint::black_box(lsx::vreplvei_b::<0>(a));
        core::hint::black_box(lsx::vreplvei_h::<0>(a));
        core::hint::black_box(lsx::vreplvei_w::<0>(a));
        core::hint::black_box(lsx::vreplvei_d::<0>(a));
        core::hint::black_box(lsx::vinsgr2vr_b::<0>(a, 1));
        core::hint::black_box(lsx::vinsgr2vr_h::<0>(a, 1));
        core::hint::black_box(lsx::vinsgr2vr_w::<0>(a, 1));
        core::hint::black_box(lsx::vinsgr2vr_d::<0>(a, 1));
        core::hint::black_box(lsx::vpickve2gr_b::<0>(a));
        core::hint::black_box(lsx::vpickve2gr_h::<0>(a));
        core::hint::black_box(lsx::vpickve2gr_w::<0>(a));
        core::hint::black_box(lsx::vpickve2gr_d::<0>(a));
        core::hint::black_box(lsx::vpickve2gr_bu::<0>(a));
        core::hint::black_box(lsx::vpickve2gr_hu::<0>(a));
        core::hint::black_box(lsx::vpickve2gr_wu::<0>(a));
        core::hint::black_box(lsx::vpickve2gr_du::<0>(a));
        core::hint::black_box(lasx::xvldrepl_b(storage.0.as_ptr(), 0));
        core::hint::black_box(lasx::xvldrepl_h(storage.0.as_ptr(), 0));
        core::hint::black_box(lasx::xvldrepl_w(storage.0.as_ptr(), 0));
        core::hint::black_box(lasx::xvldrepl_d(storage.0.as_ptr(), 0));
        core::hint::black_box(lasx::xvstelm_b::<0>(va, st.as_mut_ptr(), 0));
        core::hint::black_box(lasx::xvstelm_h::<0>(va, st.as_mut_ptr(), 0));
        core::hint::black_box(lasx::xvstelm_w::<0>(va, st.as_mut_ptr(), 0));
        core::hint::black_box(lasx::xvstelm_d::<0>(va, st.as_mut_ptr(), 0));
        core::hint::black_box(lasx::xvrepl128vei_b::<0>(va));
        core::hint::black_box(lasx::xvrepl128vei_h::<0>(va));
        core::hint::black_box(lasx::xvrepl128vei_w::<0>(va));
        core::hint::black_box(lasx::xvrepl128vei_d::<0>(va));
        core::hint::black_box(lasx::xvinsgr2vr_w::<0>(va, 1));
        core::hint::black_box(lasx::xvinsgr2vr_d::<0>(va, 1));
        core::hint::black_box(lasx::xvinsve0_w::<0>(va, a));
        core::hint::black_box(lasx::xvinsve0_d::<0>(va, a));
        core::hint::black_box(lasx::xvpickve_w::<0>(va));
        core::hint::black_box(lasx::xvpickve_d::<0>(va));
        core::hint::black_box(lasx::xvpickve2gr_w::<0>(va));
        core::hint::black_box(lasx::xvpickve2gr_d::<0>(va));
        core::hint::black_box(lasx::xvpickve2gr_wu::<0>(va));
        core::hint::black_box(lasx::xvpickve2gr_du::<0>(va));
        core::hint::black_box(lsx::vsllwil_h_b::<1>(a));
        core::hint::black_box(lsx::vsllwil_hu_bu::<1>(a));
        core::hint::black_box(lsx::vsrlni_b_h::<1>(a, a));
        core::hint::black_box(lsx::vsrlrni_b_h::<1>(a, a));
        core::hint::black_box(lsx::vssrlni_b_h::<1>(a, a));
        core::hint::black_box(lsx::vssrlni_bu_h::<1>(a, a));
        core::hint::black_box(lsx::vssrlrni_b_h::<1>(a, a));
        core::hint::black_box(lsx::vssrlrni_bu_h::<1>(a, a));
        core::hint::black_box(lsx::vsrani_b_h::<1>(a, a));
        core::hint::black_box(lsx::vsrarni_b_h::<1>(a, a));
        core::hint::black_box(lsx::vssrani_b_h::<1>(a, a));
        core::hint::black_box(lsx::vssrani_bu_h::<1>(a, a));
        core::hint::black_box(lsx::vssrarni_b_h::<1>(a, a));
        core::hint::black_box(lsx::vssrarni_bu_h::<1>(a, a));
        core::hint::black_box(lsx::vsllwil_w_h::<1>(a));
        core::hint::black_box(lsx::vsllwil_wu_hu::<1>(a));
        core::hint::black_box(lsx::vsrlni_h_w::<1>(a, a));
        core::hint::black_box(lsx::vsrlrni_h_w::<1>(a, a));
        core::hint::black_box(lsx::vssrlni_h_w::<1>(a, a));
        core::hint::black_box(lsx::vssrlni_hu_w::<1>(a, a));
        core::hint::black_box(lsx::vssrlrni_h_w::<1>(a, a));
        core::hint::black_box(lsx::vssrlrni_hu_w::<1>(a, a));
        core::hint::black_box(lsx::vsrani_h_w::<1>(a, a));
        core::hint::black_box(lsx::vsrarni_h_w::<1>(a, a));
        core::hint::black_box(lsx::vssrani_h_w::<1>(a, a));
        core::hint::black_box(lsx::vssrani_hu_w::<1>(a, a));
        core::hint::black_box(lsx::vssrarni_h_w::<1>(a, a));
        core::hint::black_box(lsx::vssrarni_hu_w::<1>(a, a));
        core::hint::black_box(lsx::vsllwil_d_w::<1>(a));
        core::hint::black_box(lsx::vsllwil_du_wu::<1>(a));
        core::hint::black_box(lsx::vsrlni_w_d::<1>(a, a));
        core::hint::black_box(lsx::vsrlrni_w_d::<1>(a, a));
        core::hint::black_box(lsx::vssrlni_w_d::<1>(a, a));
        core::hint::black_box(lsx::vssrlni_wu_d::<1>(a, a));
        core::hint::black_box(lsx::vssrlrni_w_d::<1>(a, a));
        core::hint::black_box(lsx::vssrlrni_wu_d::<1>(a, a));
        core::hint::black_box(lsx::vsrani_w_d::<1>(a, a));
        core::hint::black_box(lsx::vsrarni_w_d::<1>(a, a));
        core::hint::black_box(lsx::vssrani_w_d::<1>(a, a));
        core::hint::black_box(lsx::vssrani_wu_d::<1>(a, a));
        core::hint::black_box(lsx::vssrarni_w_d::<1>(a, a));
        core::hint::black_box(lsx::vssrarni_wu_d::<1>(a, a));
        core::hint::black_box(lsx::vsrlni_d_q::<1>(a, a));
        core::hint::black_box(lsx::vsrlrni_d_q::<1>(a, a));
        core::hint::black_box(lsx::vssrlni_d_q::<1>(a, a));
        core::hint::black_box(lsx::vssrlni_du_q::<1>(a, a));
        core::hint::black_box(lsx::vssrlrni_d_q::<1>(a, a));
        core::hint::black_box(lsx::vssrlrni_du_q::<1>(a, a));
        core::hint::black_box(lsx::vsrani_d_q::<1>(a, a));
        core::hint::black_box(lsx::vsrarni_d_q::<1>(a, a));
        core::hint::black_box(lsx::vssrani_d_q::<1>(a, a));
        core::hint::black_box(lsx::vssrani_du_q::<1>(a, a));
        core::hint::black_box(lsx::vssrarni_d_q::<1>(a, a));
        core::hint::black_box(lsx::vssrarni_du_q::<1>(a, a));
        core::hint::black_box(lasx::xvsllwil_h_b::<1>(va));
        core::hint::black_box(lasx::xvsllwil_hu_bu::<1>(va));
        core::hint::black_box(lasx::xvsrlni_b_h::<1>(va, va));
        core::hint::black_box(lasx::xvsrlrni_b_h::<1>(va, va));
        core::hint::black_box(lasx::xvssrlni_b_h::<1>(va, va));
        core::hint::black_box(lasx::xvssrlni_bu_h::<1>(va, va));
        core::hint::black_box(lasx::xvssrlrni_b_h::<1>(va, va));
        core::hint::black_box(lasx::xvssrlrni_bu_h::<1>(va, va));
        core::hint::black_box(lasx::xvsrani_b_h::<1>(va, va));
        core::hint::black_box(lasx::xvsrarni_b_h::<1>(va, va));
        core::hint::black_box(lasx::xvssrani_b_h::<1>(va, va));
        core::hint::black_box(lasx::xvssrani_bu_h::<1>(va, va));
        core::hint::black_box(lasx::xvssrarni_b_h::<1>(va, va));
        core::hint::black_box(lasx::xvssrarni_bu_h::<1>(va, va));
        core::hint::black_box(lasx::xvsllwil_w_h::<1>(va));
        core::hint::black_box(lasx::xvsllwil_wu_hu::<1>(va));
        core::hint::black_box(lasx::xvsrlni_h_w::<1>(va, va));
        core::hint::black_box(lasx::xvsrlrni_h_w::<1>(va, va));
        core::hint::black_box(lasx::xvssrlni_h_w::<1>(va, va));
        core::hint::black_box(lasx::xvssrlni_hu_w::<1>(va, va));
        core::hint::black_box(lasx::xvssrlrni_h_w::<1>(va, va));
        core::hint::black_box(lasx::xvssrlrni_hu_w::<1>(va, va));
        core::hint::black_box(lasx::xvsrani_h_w::<1>(va, va));
        core::hint::black_box(lasx::xvsrarni_h_w::<1>(va, va));
        core::hint::black_box(lasx::xvssrani_h_w::<1>(va, va));
        core::hint::black_box(lasx::xvssrani_hu_w::<1>(va, va));
        core::hint::black_box(lasx::xvssrarni_h_w::<1>(va, va));
        core::hint::black_box(lasx::xvssrarni_hu_w::<1>(va, va));
        core::hint::black_box(lasx::xvsllwil_d_w::<1>(va));
        core::hint::black_box(lasx::xvsllwil_du_wu::<1>(va));
        core::hint::black_box(lasx::xvsrlni_w_d::<1>(va, va));
        core::hint::black_box(lasx::xvsrlrni_w_d::<1>(va, va));
        core::hint::black_box(lasx::xvssrlni_w_d::<1>(va, va));
        core::hint::black_box(lasx::xvssrlni_wu_d::<1>(va, va));
        core::hint::black_box(lasx::xvssrlrni_w_d::<1>(va, va));
        core::hint::black_box(lasx::xvssrlrni_wu_d::<1>(va, va));
        core::hint::black_box(lasx::xvsrani_w_d::<1>(va, va));
        core::hint::black_box(lasx::xvsrarni_w_d::<1>(va, va));
        core::hint::black_box(lasx::xvssrani_w_d::<1>(va, va));
        core::hint::black_box(lasx::xvssrani_wu_d::<1>(va, va));
        core::hint::black_box(lasx::xvssrarni_w_d::<1>(va, va));
        core::hint::black_box(lasx::xvssrarni_wu_d::<1>(va, va));
        core::hint::black_box(lasx::xvsrlni_d_q::<1>(va, va));
        core::hint::black_box(lasx::xvsrlrni_d_q::<1>(va, va));
        core::hint::black_box(lasx::xvssrlni_d_q::<1>(va, va));
        core::hint::black_box(lasx::xvssrlni_du_q::<1>(va, va));
        core::hint::black_box(lasx::xvssrlrni_d_q::<1>(va, va));
        core::hint::black_box(lasx::xvssrlrni_du_q::<1>(va, va));
        core::hint::black_box(lasx::xvsrani_d_q::<1>(va, va));
        core::hint::black_box(lasx::xvsrarni_d_q::<1>(va, va));
        core::hint::black_box(lasx::xvssrani_d_q::<1>(va, va));
        core::hint::black_box(lasx::xvssrani_du_q::<1>(va, va));
        core::hint::black_box(lasx::xvssrarni_d_q::<1>(va, va));
        core::hint::black_box(lasx::xvssrarni_du_q::<1>(va, va));
        core::hint::black_box(lsx::vbsll(a, 1));
        core::hint::black_box(lsx::vbsrl(a, 1));
        core::hint::black_box(lsx::vseteqz(a));
        core::hint::black_box(lsx::vsetnez(a));
        core::hint::black_box(lsx::vsetanyeqz_b(a));
        core::hint::black_box(lsx::vsetanyeqz_h(a));
        core::hint::black_box(lsx::vsetanyeqz_w(a));
        core::hint::black_box(lsx::vsetanyeqz_d(a));
        core::hint::black_box(lsx::vsetallnez_b(a));
        core::hint::black_box(lsx::vsetallnez_h(a));
        core::hint::black_box(lsx::vsetallnez_w(a));
        core::hint::black_box(lsx::vsetallnez_d(a));
        core::hint::black_box(lsx::vbitseli_b::<0x0f>(a, b));
        core::hint::black_box(lsx::vshuf4i_b::<0>(a));
        core::hint::black_box(lsx::vshuf4i_h::<0>(a));
        core::hint::black_box(lsx::vshuf4i_w::<0>(a));
        core::hint::black_box(lsx::vshuf4i_d::<0>(a, a));
        core::hint::black_box(lsx::vpermi_w::<0>(a, a));
        core::hint::black_box(lasx::xvbsll(va, 1));
        core::hint::black_box(lasx::xvbsrl(va, 1));
        core::hint::black_box(lasx::xvseteqz(va));
        core::hint::black_box(lasx::xvsetnez(va));
        core::hint::black_box(lasx::xvsetanyeqz_b(va));
        core::hint::black_box(lasx::xvsetanyeqz_h(va));
        core::hint::black_box(lasx::xvsetanyeqz_w(va));
        core::hint::black_box(lasx::xvsetanyeqz_d(va));
        core::hint::black_box(lasx::xvsetallnez_b(va));
        core::hint::black_box(lasx::xvsetallnez_h(va));
        core::hint::black_box(lasx::xvsetallnez_w(va));
        core::hint::black_box(lasx::xvsetallnez_d(va));
        core::hint::black_box(lasx::xvsat_b::<1>(va));
        core::hint::black_box(lasx::xvsat_h::<1>(va));
        core::hint::black_box(lasx::xvsat_w::<1>(va));
        core::hint::black_box(lasx::xvsat_d::<1>(va));
        core::hint::black_box(lasx::xvsat_bu::<1>(va));
        core::hint::black_box(lasx::xvsat_hu::<1>(va));
        core::hint::black_box(lasx::xvsat_wu::<1>(va));
        core::hint::black_box(lasx::xvsat_du::<1>(va));
        core::hint::black_box(lasx::xvbitseli_b::<0x0f>(va, vb));
        core::hint::black_box(lasx::xvshuf4i_b::<0>(va));
        core::hint::black_box(lasx::xvshuf4i_h::<0>(va));
        core::hint::black_box(lasx::xvshuf4i_w::<0>(va));
        core::hint::black_box(lasx::xvshuf4i_d::<0>(va, va));
        core::hint::black_box(lasx::xvpermi_w::<0>(va, va));
        core::hint::black_box(lasx::xvpermi_d::<0>(va));
        core::hint::black_box(lasx::xvpermi_q::<0>(va, va));
        core::hint::black_box(lsx::vfrecipe_s(a));
        core::hint::black_box(lsx::vfrecipe_d(a));
        core::hint::black_box(lsx::vfrsqrte_s(a));
        core::hint::black_box(lsx::vfrsqrte_d(a));
        core::hint::black_box(lasx::xvfrecipe_s(va));
        core::hint::black_box(lasx::xvfrecipe_d(va));
        core::hint::black_box(lasx::xvfrsqrte_s(va));
        core::hint::black_box(lasx::xvfrsqrte_d(va));
        core::hint::black_box(lsx::vextrins_b::<0>(a, b));
        core::hint::black_box(lsx::vextrins_h::<0>(a, b));
        core::hint::black_box(lsx::vextrins_w::<0>(a, b));
        core::hint::black_box(lsx::vextrins_d::<0>(a, b));
        core::hint::black_box(lasx::xvextrins_b::<0>(va, vb));
        core::hint::black_box(lasx::xvextrins_h::<0>(va, vb));
        core::hint::black_box(lasx::xvextrins_w::<0>(va, vb));
        core::hint::black_box(lasx::xvextrins_d::<0>(va, vb));
        core::hint::black_box(lsx::vfrstpi_b::<0>(a, b));
        core::hint::black_box(lsx::vfrstpi_h::<0>(a, b));
        core::hint::black_box(lasx::xvfrstpi_b::<0>(va, vb));
        core::hint::black_box(lasx::xvfrstpi_h::<0>(va, vb));
    }
    0
}

#[target_feature(enable = "lasx")]
unsafe fn compare_ops() -> bool {
    let v128 = V128::from_bytes([0; 16]);
    let v256 = V256::from_bytes([0; 32]);
    let _ = lsx::vfcmp_caf_s(v128, v128);
    let _ = lsx::vfcmp_saf_s(v128, v128);
    let _ = lsx::vfcmp_clt_s(v128, v128);
    let _ = lsx::vfcmp_slt_s(v128, v128);
    let _ = lsx::vfcmp_ceq_s(v128, v128);
    let _ = lsx::vfcmp_seq_s(v128, v128);
    let _ = lsx::vfcmp_cle_s(v128, v128);
    let _ = lsx::vfcmp_sle_s(v128, v128);
    let _ = lsx::vfcmp_cun_s(v128, v128);
    let _ = lsx::vfcmp_sun_s(v128, v128);
    let _ = lsx::vfcmp_cult_s(v128, v128);
    let _ = lsx::vfcmp_sult_s(v128, v128);
    let _ = lsx::vfcmp_cueq_s(v128, v128);
    let _ = lsx::vfcmp_sueq_s(v128, v128);
    let _ = lsx::vfcmp_cule_s(v128, v128);
    let _ = lsx::vfcmp_sule_s(v128, v128);
    let _ = lsx::vfcmp_cne_s(v128, v128);
    let _ = lsx::vfcmp_sne_s(v128, v128);
    let _ = lsx::vfcmp_cor_s(v128, v128);
    let _ = lsx::vfcmp_sor_s(v128, v128);
    let _ = lsx::vfcmp_cune_s(v128, v128);
    let _ = lsx::vfcmp_sune_s(v128, v128);
    let _ = lsx::vfcmp_caf_d(v128, v128);
    let _ = lsx::vfcmp_saf_d(v128, v128);
    let _ = lsx::vfcmp_clt_d(v128, v128);
    let _ = lsx::vfcmp_slt_d(v128, v128);
    let _ = lsx::vfcmp_ceq_d(v128, v128);
    let _ = lsx::vfcmp_seq_d(v128, v128);
    let _ = lsx::vfcmp_cle_d(v128, v128);
    let _ = lsx::vfcmp_sle_d(v128, v128);
    let _ = lsx::vfcmp_cun_d(v128, v128);
    let _ = lsx::vfcmp_sun_d(v128, v128);
    let _ = lsx::vfcmp_cult_d(v128, v128);
    let _ = lsx::vfcmp_sult_d(v128, v128);
    let _ = lsx::vfcmp_cueq_d(v128, v128);
    let _ = lsx::vfcmp_sueq_d(v128, v128);
    let _ = lsx::vfcmp_cule_d(v128, v128);
    let _ = lsx::vfcmp_sule_d(v128, v128);
    let _ = lsx::vfcmp_cne_d(v128, v128);
    let _ = lsx::vfcmp_sne_d(v128, v128);
    let _ = lsx::vfcmp_cor_d(v128, v128);
    let _ = lsx::vfcmp_sor_d(v128, v128);
    let _ = lsx::vfcmp_cune_d(v128, v128);
    let _ = lsx::vfcmp_sune_d(v128, v128);
    unsafe {
        lasx::xvfcmp_caf_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_saf_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_clt_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_slt_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_ceq_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_seq_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cle_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sle_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cun_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sun_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cult_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sult_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cueq_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sueq_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cule_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sule_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cne_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sne_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cor_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sor_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cune_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sune_s(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_caf_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_saf_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_clt_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_slt_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_ceq_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_seq_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cle_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sle_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cun_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sun_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cult_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sult_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cueq_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sueq_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cule_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sule_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cne_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sne_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cor_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sor_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_cune_d(v256, v256);
    }
    unsafe {
        lasx::xvfcmp_sune_d(v256, v256);
    }
    let _ = lsx::vfmadd_s(v128, v128, v128);
    let _ = lsx::vfmadd_d(v128, v128, v128);
    let _ = lsx::vfmsub_s(v128, v128, v128);
    let _ = lsx::vfmsub_d(v128, v128, v128);
    let _ = lsx::vfnmadd_s(v128, v128, v128);
    let _ = lsx::vfnmadd_d(v128, v128, v128);
    let _ = lsx::vfnmsub_s(v128, v128, v128);
    let _ = lsx::vfnmsub_d(v128, v128, v128);
    unsafe {
        lasx::xvfmadd_s(v256, v256, v256);
    }
    unsafe {
        lasx::xvfmadd_d(v256, v256, v256);
    }
    unsafe {
        lasx::xvfmsub_s(v256, v256, v256);
    }
    unsafe {
        lasx::xvfmsub_d(v256, v256, v256);
    }
    unsafe {
        lasx::xvfnmadd_s(v256, v256, v256);
    }
    unsafe {
        lasx::xvfnmadd_d(v256, v256, v256);
    }
    unsafe {
        lasx::xvfnmsub_s(v256, v256, v256);
    }
    unsafe {
        lasx::xvfnmsub_d(v256, v256, v256);
    }
    true
}

unsafe fn fixed_semantics_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|i| (i * 17 + 3) as u8));
    let b = V128::from_bytes(core::array::from_fn(|i| (255 - i * 13) as u8));
    if lsx::vbitseli_b::<0x5a>(a, b).to_bytes()
        != [
            0xfe, 0xf2, 0xc0, 0xda, 0xca, 0xfe, 0xd8, 0xde, 0x1e, 0x1a, 0x58, 0x5a, 0x6a, 0x56,
            0x58, 0x3e,
        ]
    {
        return false;
    }
    if lsx::vextrins_b::<0x21>(a, b).to_bytes()
        != [
            0x03, 0x14, 0xf2, 0x36, 0x47, 0x58, 0x69, 0x7a, 0x8b, 0x9c, 0xad, 0xbe, 0xcf, 0xe0,
            0xf1, 0x02,
        ]
        || lsx::vextrins_h::<0x31>(a, b).to_bytes()
            != [
                0x03, 0x14, 0x25, 0x36, 0x47, 0x58, 0xe5, 0xd8, 0x8b, 0x9c, 0xad, 0xbe, 0xcf, 0xe0,
                0xf1, 0x02,
            ]
        || lsx::vextrins_w::<0x13>(a, b).to_bytes()
            != [
                0x03, 0x14, 0x25, 0x36, 0x63, 0x56, 0x49, 0x3c, 0x8b, 0x9c, 0xad, 0xbe, 0xcf, 0xe0,
                0xf1, 0x02,
            ]
    {
        return false;
    }
    let a0 = [0x03, 0x14, 0x25, 0x36, 0x47, 0x58, 0x69, 0x7a];
    let a1 = [0x8b, 0x9c, 0xad, 0xbe, 0xcf, 0xe0, 0xf1, 0x02];
    let b0 = [0xff, 0xf2, 0xe5, 0xd8, 0xcb, 0xbe, 0xb1, 0xa4];
    let mut expected = [0u8; 16];
    expected[..8].copy_from_slice(&a0);
    expected[8..].copy_from_slice(&a0);
    if lsx::vshuf4i_d::<0x0>(a, b).to_bytes() != expected {
        return false;
    }
    expected[..8].copy_from_slice(&a1);
    if lsx::vshuf4i_d::<0x1>(a, b).to_bytes() != expected {
        return false;
    }
    expected[..8].copy_from_slice(&b0);
    if lsx::vshuf4i_d::<0x2>(a, b).to_bytes() != expected {
        return false;
    }
    expected[..8].copy_from_slice(&a0);
    expected[8..].copy_from_slice(&a1);
    if lsx::vshuf4i_d::<0x4>(a, b).to_bytes() != expected {
        return false;
    }
    expected[..8].copy_from_slice(&a0);
    expected[8..].copy_from_slice(&b0);
    if lsx::vshuf4i_d::<0x8>(a, b).to_bytes() != expected {
        return false;
    }
    expected[..8].copy_from_slice(&b0);
    if lsx::vshuf4i_d::<0xa>(a, b).to_bytes() != expected {
        return false;
    }
    if lsx::vpermi_w::<0x1b>(a, b).to_bytes()
        != [
            0x63, 0x56, 0x49, 0x3c, 0x97, 0x8a, 0x7d, 0x70, 0x47, 0x58, 0x69, 0x7a, 0x03, 0x14,
            0x25, 0x36,
        ]
        || lsx::vpermi_w::<0xe4>(a, b).to_bytes()
            != [
                0xff, 0xf2, 0xe5, 0xd8, 0xcb, 0xbe, 0xb1, 0xa4, 0x8b, 0x9c, 0xad, 0xbe, 0xcf, 0xe0,
                0xf1, 0x02,
            ]
    {
        return false;
    }

    let va = V256::from_bytes(core::array::from_fn(|i| (i * 7 + 1) as u8));
    let vb = V256::from_bytes(core::array::from_fn(|i| (200 - i * 5) as u8));
    if lasx::xvbitseli_b::<0x5a>(va, vb).to_bytes()
        != [
            0xc8, 0xcb, 0xba, 0xbb, 0xb8, 0x8b, 0x8a, 0x97, 0x98, 0xdb, 0xd2, 0xdb, 0xd8, 0xdb,
            0xc2, 0x5f, 0x58, 0x5b, 0x5a, 0x6b, 0x68, 0x5b, 0x5a, 0x57, 0x58, 0x5b, 0x52, 0x5b,
            0x78, 0x7b, 0x72, 0x7f,
        ]
    {
        return false;
    }
    if lasx::xvextrins_b::<0x21>(va, vb).to_bytes()
        != [
            0x01, 0x08, 0xc3, 0x16, 0x1d, 0x24, 0x2b, 0x32, 0x39, 0x40, 0x47, 0x4e, 0x55, 0x5c,
            0x63, 0x6a, 0x71, 0x78, 0x73, 0x86, 0x8d, 0x94, 0x9b, 0xa2, 0xa9, 0xb0, 0xb7, 0xbe,
            0xc5, 0xcc, 0xd3, 0xda,
        ]
        || lasx::xvextrins_b::<0x12>(va, vb).to_bytes()
            != [
                0x01, 0xbe, 0x0f, 0x16, 0x1d, 0x24, 0x2b, 0x32, 0x39, 0x40, 0x47, 0x4e, 0x55, 0x5c,
                0x63, 0x6a, 0x71, 0x6e, 0x7f, 0x86, 0x8d, 0x94, 0x9b, 0xa2, 0xa9, 0xb0, 0xb7, 0xbe,
                0xc5, 0xcc, 0xd3, 0xda,
            ]
    {
        return false;
    }
    if lasx::xvshuf4i_d::<0x0>(va, vb).to_bytes()
        != [
            0x01, 0x08, 0x0f, 0x16, 0x1d, 0x24, 0x2b, 0x32, 0x01, 0x08, 0x0f, 0x16, 0x1d, 0x24,
            0x2b, 0x32, 0x71, 0x78, 0x7f, 0x86, 0x8d, 0x94, 0x9b, 0xa2, 0x71, 0x78, 0x7f, 0x86,
            0x8d, 0x94, 0x9b, 0xa2,
        ]
        || lasx::xvshuf4i_d::<0x1>(va, vb).to_bytes()
            != [
                0x39, 0x40, 0x47, 0x4e, 0x55, 0x5c, 0x63, 0x6a, 0x01, 0x08, 0x0f, 0x16, 0x1d, 0x24,
                0x2b, 0x32, 0xa9, 0xb0, 0xb7, 0xbe, 0xc5, 0xcc, 0xd3, 0xda, 0x71, 0x78, 0x7f, 0x86,
                0x8d, 0x94, 0x9b, 0xa2,
            ]
        || lasx::xvshuf4i_d::<0xa>(va, vb).to_bytes()
            != [
                0xc8, 0xc3, 0xbe, 0xb9, 0xb4, 0xaf, 0xaa, 0xa5, 0xc8, 0xc3, 0xbe, 0xb9, 0xb4, 0xaf,
                0xaa, 0xa5, 0x78, 0x73, 0x6e, 0x69, 0x64, 0x5f, 0x5a, 0x55, 0x78, 0x73, 0x6e, 0x69,
                0x64, 0x5f, 0x5a, 0x55,
            ]
    {
        return false;
    }
    if lasx::xvpermi_w::<0x1b>(va, vb).to_bytes()
        != [
            0x8c, 0x87, 0x82, 0x7d, 0xa0, 0x9b, 0x96, 0x91, 0x1d, 0x24, 0x2b, 0x32, 0x01, 0x08,
            0x0f, 0x16, 0x3c, 0x37, 0x32, 0x2d, 0x50, 0x4b, 0x46, 0x41, 0x8d, 0x94, 0x9b, 0xa2,
            0x71, 0x78, 0x7f, 0x86,
        ]
        || lasx::xvpermi_d::<6>(va).to_bytes()
            != [
                0x71, 0x78, 0x7f, 0x86, 0x8d, 0x94, 0x9b, 0xa2, 0x39, 0x40, 0x47, 0x4e, 0x55, 0x5c,
                0x63, 0x6a, 0x01, 0x08, 0x0f, 0x16, 0x1d, 0x24, 0x2b, 0x32, 0x01, 0x08, 0x0f, 0x16,
                0x1d, 0x24, 0x2b, 0x32,
            ]
        || lasx::xvpermi_q::<3>(va, vb).to_bytes()
            != [
                0x71, 0x78, 0x7f, 0x86, 0x8d, 0x94, 0x9b, 0xa2, 0xa9, 0xb0, 0xb7, 0xbe, 0xc5, 0xcc,
                0xd3, 0xda, 0xc8, 0xc3, 0xbe, 0xb9, 0xb4, 0xaf, 0xaa, 0xa5, 0xa0, 0x9b, 0x96, 0x91,
                0x8c, 0x87, 0x82, 0x7d,
            ]
    {
        return false;
    }
    let source = V256::from_bytes(core::array::from_fn(|i| i as u8));
    let mut left = [0u8; 32];
    for half in 0..2 {
        let base = half * 16;
        left[base + 1..base + 16].copy_from_slice(&source.to_bytes()[base..base + 15]);
    }
    let mut right = [0u8; 32];
    for half in 0..2 {
        let base = half * 16;
        right[base..base + 15].copy_from_slice(&source.to_bytes()[base + 1..base + 16]);
    }
    lasx::xvbsll(source, 1).to_bytes() == left && lasx::xvbsrl(source, 1).to_bytes() == right
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
    if !unsafe { compare_ops() } {
        return 64;
    }
    let gen_code = unsafe { generated_ops() };
    if gen_code != 0 {
        return 64 + gen_code;
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
    if !unsafe { fixed_semantics_ops() } {
        return 100;
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
    let converted = vfcvt_s_d(f64x2([1.0, 2.0]), f64x2([3.0, 4.0]));
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
        core::hint::black_box(lasx::xvfcvt_s_d(
            V256::from_bytes([0; 32]),
            V256::from_bytes([0; 32]),
        ));
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
        if lasx::xvreplgr2vr_w(7) != i32x8([7; 8]) {
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
    lanes(vrepl_w(7)) == [7; 4]
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
        && lanes(vreplve_w::<2>(i32x4([10, 20, 30, 40]))) == [30; 4]
}
