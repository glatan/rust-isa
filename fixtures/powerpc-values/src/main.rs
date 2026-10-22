#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::powerpc::crypto::power8_crypto::{
    cipher, cipherlast, ncipher, ncipherlast, vcipher, vcipherlast, vncipher, vncipherlast,
    vpermxor, vpmsumb, vpmsumd, vpmsumh, vpmsumw, vsbox, vshasigmad, vshasigmaw,
};
use rust_isa::powerpc::matrix::mma::{
    MmaAccumulator, xvi4ger8, xvi4ger8pp, xvi8ger4, xvi8ger4pp, xvi8ger4spp, xvi16ger2,
    xvi16ger2pp, xvi16ger2s, xvi16ger2spp, xxmfacc, xxmtacc, xxsetaccz,
};
use rust_isa::powerpc::simd::altivec::{
    V128, add, add_i8, add_i16, add_i64, and, cmpeq_i32, cmpgt_i32, fadd, fdiv, fmul, fsub, mul,
    mul_i16, mul_i32, or, permute, splat_i32, sub_i8, sub_i16, sub_i32, sub_i64, v128_load,
    v128_store, vaddubm, vaddudm, vadduhm, vadduwm, vand, vandc, vavg, vavgl, vcmpeq, vcmpgt, vmax,
    vmin, vmuleub, vmuleuh, vmuleuw as av_muleuw, vmuloub, vmulouh, vmulouw as av_mulouw, vnor,
    vnot, vor, vperm, vsel, vsl, vsldoi, vslo, vsr, vsro, vsububm, vsubudm, vsubuhm, vsubuwm, vxor,
    xor,
};
use rust_isa::powerpc::simd::power8_vector::{
    add_u64, mul_even_u16, mul_u32, sub_u64, vclzd as p8_vclzd, vctzd as p8_vctzd, vmaxsd, vmaxud,
    vminsd, vminud, vmuleuw, vmulouw, vpksdss, vpksdus, vpkudum, vpopcntd as p8_vpopcntd, vupkhsw,
    vupklsw,
};
use rust_isa::powerpc::simd::power8_vector::{
    vaddudm as p8_vaddudm, vmuleuh as p8_vmuleuh, vmuluwm as p8_vmuluwm, vsubudm as p8_vsubudm,
};
use rust_isa::powerpc::simd::power9_altivec::{vctzd as altivec_vctzd, vnegw as altivec_vnegw};
use rust_isa::powerpc::simd::power9_vector::{
    abs_i32 as p9_abs_i32, neg_i32 as p9_neg_i32, vabsd, vabsduw, vctzb, vctzd as p9_vctzd, vctzh,
    vctzlsbb, vctzw, vextsb2d, vextsb2w, vextsh2d, vextsh2w, vextsw2d, vnegd, vnegw,
};
use rust_isa::powerpc::simd::power10_vector::{
    cntlz_i32 as p10_cntlz_i32, popcnt_i32 as p10_popcnt_i32, vclzd, vctzd, vpopcntd, vpopcntw,
};
use rust_isa::powerpc::simd::vsx::{
    abs_f64, add_f32 as vsx_add_f32, add_f64, div_f64, mul_f64, sqrt_f64, sub_f64,
    vcmpgt as vsx_cmpgt, xvabs, xvadd, xvand, xvcmpeq, xvcpsgn, xvdiv, xvmax, xvmin, xvmul, xvneg,
    xvor, xvperm, xvrdpi, xvrdpic, xvrdpim, xvrdpip, xvrdpiz, xvsel, xvsl, xvsldoi, xvsr, xvsub,
    xvxor,
};
use rust_isa::powerpc::simd::vsx::{
    xscvdpsp, xscvdpspn, xscvspdp, xscvspdpn, xvcvdpsxds, xvcvdpsxws, xvcvdpuxds, xvcvdpuxws,
    xvcvsxdsp, xvcvsxwsp, xvcvuxdsp, xvcvuxwsp,
};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "li 0, 1",
            "sc",
            in("r0") 1usize,
            in("r3") code as usize,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "li 0, 1",
            "sc",
            in("r0") 1usize,
            in("r3") 255usize,
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
    if !ops_arithmetic() {
        return 31;
    }
    if !(unsafe {
        and(
            V128::from_bytes([0b1010_1010; 16]),
            V128::from_bytes([0b1100_1100; 16]),
        )
        .to_bytes()
            == [0b1000_1000; 16]
    }) {
        return 321;
    }
    if !(unsafe {
        or(
            V128::from_bytes([0b1010_1010; 16]),
            V128::from_bytes([0b1100_1100; 16]),
        )
        .to_bytes()
            == [0b1110_1110; 16]
    }) {
        return 322;
    }
    if !(unsafe {
        xor(
            V128::from_bytes([0b1010_1010; 16]),
            V128::from_bytes([0b1100_1100; 16]),
        )
        .to_bytes()
            == [0b0110_0110; 16]
    }) {
        return 323;
    }
    let eq = unsafe { cmpeq_i32(i32x4([1; 4]), i32x4([1; 4])) }.to_bytes();
    if eq != [0xff; 16] {
        return eq[0] as u32;
    }
    if !(unsafe { cmpgt_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16] }) {
        return 325;
    }
    if !ops_wide() {
        return 33;
    }
    if !float_ops() {
        return 4;
    }
    if !permute_ops() {
        return 5;
    }
    if !vsx_ops() {
        return 6;
    }
    if !power8_ops() {
        return 7;
    }
    if !crypto_ops() {
        return 8;
    }
    if !power9_ops() {
        return 9;
    }
    if !power10_ops() {
        return 10;
    }
    if !mma_ops() {
        return 12;
    }
    if !altivec_ops() {
        return 11;
    }
    0
}

fn power9_ops() -> bool {
    unsafe {
        lanes(p9_neg_i32(i32x4([1, -2, 3, -4]))) == [-1, 2, -3, 4]
            && lanes(p9_abs_i32(i32x4([-1, 2, -3, 4]))) == [1, 2, 3, 4]
            && lanes(vnegw(i32x4([1, -2, 3, -4]))) == [-1, 2, -3, 4]
            && lanes(altivec_vnegw(i32x4([1, -2, 3, -4]))) == [-1, 2, -3, 4]
            && vnegd(V128::from_bytes([0; 16])).to_bytes() == [0; 16]
            && lanes(vabsduw(i32x4([1, 4, 10, 20]), i32x4([4, 1, 12, 30]))) == [3, 3, 2, 10]
            && lanes(vabsd(i32x4([1, 4, 10, 20]), i32x4([4, 1, 12, 30]))) == [3, 3, 2, 10]
            && lanes(vextsb2w(V128::from_bytes([0xff; 16]))) == [-1; 4]
            && lanes(vextsh2w(V128::from_bytes([0xff; 16]))) == [-1; 4]
            && vctzb(V128::from_bytes([0xff; 16])).to_bytes() == [0; 16]
            && vctzh(V128::from_bytes([0xff; 16])).to_bytes() == [0; 16]
            && vctzw(V128::from_bytes([0xff; 16])).to_bytes() == [0; 16]
            && {
                let bytes = p9_vctzd(V128::from_bytes([0x02; 16])).to_bytes();
                bytes[0] == 1 && bytes[8] == 1
            }
            && {
                let bytes = altivec_vctzd(V128::from_bytes([0x02; 16])).to_bytes();
                bytes[0] == 1 && bytes[8] == 1
            }
            && vctzlsbb(V128::from_bytes([0xff; 16])) == 0
            && vextsb2d(V128::from_bytes([0xff; 16])).to_bytes()
                == V128::from_bytes([0xff; 16]).to_bytes()
            && vextsh2d(V128::from_bytes([0xff; 16])).to_bytes()
                == V128::from_bytes([0xff; 16]).to_bytes()
            && vextsw2d(V128::from_bytes([0xff; 16])).to_bytes()
                == V128::from_bytes([0xff; 16]).to_bytes()
    }
}

fn power10_ops() -> bool {
    unsafe {
        lanes(p10_popcnt_i32(i32x4([-1; 4]))) == [32; 4]
            && lanes(p10_cntlz_i32(i32x4([1, 2, 4, 8]))) == [31, 30, 29, 28]
            && lanes(vpopcntw(i32x4([-1; 4]))) == [32; 4]
            && {
                let bytes = vpopcntd(V128::from_bytes([0xff; 16])).to_bytes();
                bytes[0] == 64 && bytes[8] == 64
            }
            && {
                let bytes = vclzd(V128::from_bytes([0; 16])).to_bytes();
                bytes[0] == 64 && bytes[8] == 64
            }
            && {
                let bytes = vctzd(V128::from_bytes([0; 16])).to_bytes();
                bytes[0] == 64 && bytes[8] == 64
            }
    }
}

fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    [
        f64::from_le_bytes(bytes[..8].try_into().unwrap()),
        f64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

fn vsx_ops() -> bool {
    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    let f = f32x4([1.0, 2.0, 3.0, 4.0]);
    let g = f32x4([10.0, 20.0, 30.0, 40.0]);
    unsafe {
        f64_lanes(add_f64(a, b)) == [11.0, 22.0]
            && f64_lanes(sub_f64(b, a)) == [9.0, 18.0]
            && f64_lanes(mul_f64(a, b)) == [10.0, 40.0]
            && f64_lanes(div_f64(b, a)) == [10.0, 10.0]
            && f32_lanes(vsx_add_f32(f, g)) == [11.0, 22.0, 33.0, 44.0]
            && f64_lanes(abs_f64(f64x2([-1.0, -2.0]))) == [1.0, 2.0]
            && f64_lanes(sqrt_f64(f64x2([4.0, 9.0]))) == [2.0, 3.0]
            && f64_lanes(xvadd(a, b)) == [11.0, 22.0]
            && f64_lanes(xvsub(b, a)) == [9.0, 18.0]
            && f64_lanes(xvmul(a, b)) == [10.0, 40.0]
            && f64_lanes(xvdiv(b, a)) == [10.0, 10.0]
            && f64_lanes(xvabs(f64x2([-1.0, -2.0]))) == [1.0, 2.0]
            && f64_lanes(xvneg(f64x2([1.0, 2.0]))) == [-1.0, -2.0]
            && f64_lanes(xvmax(a, b)) == [10.0, 20.0]
            && f64_lanes(xvmin(a, b)) == [1.0, 2.0]
            && xvcmpeq(a, a).to_bytes() == [0xff; 16]
            && vsx_cmpgt(b, a).to_bytes() == [0xff; 16]
            && xvand(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes()
                == [0x88; 16]
            && xvor(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes()
                == [0xee; 16]
            && xvxor(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes()
                == [0x66; 16]
            && xvsel(
                V128::from_bytes([1; 16]),
                V128::from_bytes([2; 16]),
                V128::from_bytes([0xff; 16]),
            )
            .to_bytes()
                == [2; 16]
            && xvperm(
                V128::from_bytes([1; 16]),
                V128::from_bytes([2; 16]),
                V128::from_bytes([0; 16]),
            )
            .to_bytes()
                != [0; 16]
            && xvsl(V128::from_bytes([1; 16]), V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && xvsr(V128::from_bytes([2; 16]), V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && xvrdpi(f64x2([1.5, 2.5])).to_bytes() != [0; 16]
            && xvrdpic(f64x2([1.5, 2.5])).to_bytes() != [0; 16]
            && xvrdpim(f64x2([1.5, 2.5])).to_bytes() != [0; 16]
            && xvrdpip(f64x2([1.5, 2.5])).to_bytes() != [0; 16]
            && xvrdpiz(f64x2([1.5, 2.5])).to_bytes() != [0; 16]
            && xvcpsgn(f64x2([1.0, 1.0]), f64x2([-2.0, 2.0])).to_bytes() != [0; 16]
            && xvsldoi::<4>(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes()
                != [0; 16]
            && f32_lanes(xscvdpsp(f64x2([1.5, 2.5])))[0] == 1.5
            && f32_lanes(xscvdpspn(f64x2([1.5, 2.5])))[0] == 1.5
            && f64_lanes(xscvspdp(f32x4([1.5, 2.5, 3.5, 4.5])))[0] == 1.5
            && f64_lanes(xscvspdpn(f32x4([1.5, 2.5, 3.5, 4.5])))[0] == 1.5
            && xvcvsxdsp(V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && xvcvuxdsp(V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && xvcvsxwsp(V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && xvcvuxwsp(V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && xvcvdpsxds(f64x2([1.0, 2.0])).to_bytes() != [0; 16]
            && xvcvdpsxws(f64x2([1.0, 2.0])).to_bytes() != [0; 16]
            && xvcvdpuxds(f64x2([1.0, 2.0])).to_bytes() != [0; 16]
            && xvcvdpuxws(f64x2([1.0, 2.0])).to_bytes() != [0; 16]
    }
}

fn power8_ops() -> bool {
    let a = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    let b = V128::from_bytes([2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]);
    unsafe {
        let sum = add_u64(a, b).to_bytes();
        let diff = sub_u64(b, a).to_bytes();
        if i64::from_le_bytes(sum[..8].try_into().unwrap()) != 3
            || i64::from_le_bytes(sum[8..].try_into().unwrap()) != 5
            || i64::from_le_bytes(diff[..8].try_into().unwrap()) != 1
            || i64::from_le_bytes(diff[8..].try_into().unwrap()) != 1
        {
            return false;
        }
        if lanes(vmaxud(i32x4([1, 4, 2, 8]), i32x4([3, 2, 6, 4]))) != [3, 4, 6, 8]
            || lanes(vminud(i32x4([1, 4, 2, 8]), i32x4([3, 2, 6, 4]))) != [1, 2, 2, 4]
            || lanes(vmaxsd(i32x4([-1, 4, -2, 8]), i32x4([-3, 2, -6, 4]))) != [-1, 4, -2, 8]
            || lanes(vminsd(i32x4([-1, 4, -2, 8]), i32x4([-3, 2, -6, 4]))) != [-3, 2, -6, 4]
            || p8_vclzd(V128::from_bytes([0; 16])).to_bytes()[0] != 64
            || p8_vpopcntd(V128::from_bytes([0xff; 16])).to_bytes()[0] != 64
            || vpkudum(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [0; 16]
            || vpksdus(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [0; 16]
            || vpksdss(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [0; 16]
            || vupkhsw(i32x4([1, 2, 3, 4])).to_bytes() == [0; 16]
            || vupklsw(i32x4([1, 2, 3, 4])).to_bytes() == [0; 16]
            || vmuleuw(i32x4([1, 2, 3, 4]), i32x4([1, 2, 3, 4])).to_bytes() == [0; 16]
            || vmulouw(i32x4([1, 2, 3, 4]), i32x4([1, 2, 3, 4])).to_bytes() == [0; 16]
            || {
                let bytes = p8_vctzd(V128::from_bytes([0x02; 16])).to_bytes();
                bytes[0] != 1 || bytes[8] != 1
            }
        {
            return false;
        }
        if p8_vaddudm(a, b) != add_u64(a, b)
            || p8_vsubudm(b, a) != sub_u64(b, a)
            || p8_vmuluwm(i32x4([1, 2, 3, 4]), i32x4([5, 6, 7, 8]))
                != mul_u32(i32x4([1, 2, 3, 4]), i32x4([5, 6, 7, 8]))
            || p8_vmuleuh(V128::from_bytes([1; 16]), V128::from_bytes([2; 16]))
                != mul_even_u16(V128::from_bytes([1; 16]), V128::from_bytes([2; 16]))
        {
            return false;
        }
        let c = i32x4([2, 3, 4, 5]);
        let d = i32x4([6, 7, 8, 9]);
        if lanes(mul_u32(c, d)) != [12, 21, 32, 45] {
            return false;
        }
        let ones = i16x8([3; 8]);
        let twos = i16x8([4; 8]);
        lanes(mul_even_u16(ones, twos)) == [12; 4]
    }
}

fn u32x4_logical(values: [u32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        let offset = (3 - lane) * 4;
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn u32_logical_lanes(value: V128) -> [u32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        let offset = (3 - lane) * 4;
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    })
}

fn u64x2_logical(values: [u64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        let offset = (1 - lane) * 8;
        bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn u64_logical_lanes(value: V128) -> [u64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        let offset = (1 - lane) * 8;
        u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
    })
}

fn sha256_sigma<const ST: u32, const SIX: u32>(values: [u32; 4]) -> [u32; 4] {
    core::array::from_fn(|lane| {
        let value = values[lane];
        let select = ((SIX >> (3 - lane)) & 1) != 0;
        match (ST, select) {
            (0, false) => value.rotate_right(7) ^ value.rotate_right(18) ^ (value >> 3),
            (0, true) => value.rotate_right(17) ^ value.rotate_right(19) ^ (value >> 10),
            (1, false) => value.rotate_right(2) ^ value.rotate_right(13) ^ value.rotate_right(22),
            _ => value.rotate_right(6) ^ value.rotate_right(11) ^ value.rotate_right(25),
        }
    })
}

fn sha512_sigma<const ST: u32, const SIX: u32>(values: [u64; 2]) -> [u64; 2] {
    core::array::from_fn(|lane| {
        let value = values[lane];
        let select = ((SIX >> (3 - lane * 2)) & 1) != 0;
        match (ST, select) {
            (0, false) => value.rotate_right(1) ^ value.rotate_right(8) ^ (value >> 7),
            (0, true) => value.rotate_right(19) ^ value.rotate_right(61) ^ (value >> 6),
            (1, false) => value.rotate_right(28) ^ value.rotate_right(34) ^ value.rotate_right(39),
            _ => value.rotate_right(14) ^ value.rotate_right(18) ^ value.rotate_right(41),
        }
    })
}

fn clmul_u8(a: u8, b: u8) -> u16 {
    let mut result = 0u16;
    for bit in 0..8u32 {
        if (b >> bit) & 1 == 1 {
            result ^= (a as u16) << bit;
        }
    }
    result
}

fn clmul_u16(a: u16, b: u16) -> u32 {
    let mut result = 0u32;
    for bit in 0..16u32 {
        if (b >> bit) & 1 == 1 {
            result ^= (a as u32) << bit;
        }
    }
    result
}

fn clmul_u32(a: u32, b: u32) -> u64 {
    let mut result = 0u64;
    for bit in 0..32u32 {
        if (b >> bit) & 1 == 1 {
            result ^= (a as u64) << bit;
        }
    }
    result
}

fn clmul_u64(a: u64, b: u64) -> u128 {
    let mut result = 0u128;
    for bit in 0..64u32 {
        if (b >> bit) & 1 == 1 {
            result ^= (a as u128) << bit;
        }
    }
    result
}

fn vpmsumb_ref(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for lane in 0..8 {
        let offset = lane * 2;
        let product = clmul_u8(a[offset], b[offset]) ^ clmul_u8(a[offset + 1], b[offset + 1]);
        out[offset..offset + 2].copy_from_slice(&product.to_le_bytes());
    }
    out
}

fn vpmsumh_ref(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for lane in 0..4 {
        let offset = lane * 4;
        let a0 = u16::from_le_bytes(a[offset..offset + 2].try_into().unwrap());
        let a1 = u16::from_le_bytes(a[offset + 2..offset + 4].try_into().unwrap());
        let b0 = u16::from_le_bytes(b[offset..offset + 2].try_into().unwrap());
        let b1 = u16::from_le_bytes(b[offset + 2..offset + 4].try_into().unwrap());
        let product = clmul_u16(a0, b0) ^ clmul_u16(a1, b1);
        out[offset..offset + 4].copy_from_slice(&product.to_le_bytes());
    }
    out
}

fn vpmsumw_ref(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for lane in 0..2 {
        let offset = lane * 8;
        let a0 = u32::from_le_bytes(a[offset..offset + 4].try_into().unwrap());
        let a1 = u32::from_le_bytes(a[offset + 4..offset + 8].try_into().unwrap());
        let b0 = u32::from_le_bytes(b[offset..offset + 4].try_into().unwrap());
        let b1 = u32::from_le_bytes(b[offset + 4..offset + 8].try_into().unwrap());
        let product = clmul_u32(a0, b0) ^ clmul_u32(a1, b1);
        out[offset..offset + 8].copy_from_slice(&product.to_le_bytes());
    }
    out
}

fn vpmsumd_ref(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let a0 = u64::from_le_bytes(a[..8].try_into().unwrap());
    let a1 = u64::from_le_bytes(a[8..].try_into().unwrap());
    let b0 = u64::from_le_bytes(b[..8].try_into().unwrap());
    let b1 = u64::from_le_bytes(b[8..].try_into().unwrap());
    (clmul_u64(a0, b0) ^ clmul_u64(a1, b1)).to_le_bytes()
}

fn vpermxor_ref(a: &[u8; 16], b: &[u8; 16], c: &[u8; 16]) -> [u8; 16] {
    core::array::from_fn(|index| {
        let high = (c[index] >> 4) as usize;
        let low = (c[index] & 0xf) as usize;
        a[15 - high] ^ b[15 - low]
    })
}

fn crypto_ops() -> bool {
    let one = V128::from_bytes([1; 16]);
    let two = V128::from_bytes([2; 16]);
    let zero = V128::from_bytes([0; 16]);
    unsafe {
        if cipher(one, two).to_bytes() == zero.to_bytes()
            || ncipher(one, two).to_bytes() == zero.to_bytes()
            || cipherlast(one, two).to_bytes() == zero.to_bytes()
            || ncipherlast(one, two).to_bytes() == zero.to_bytes()
        {
            return false;
        }
        if vcipher(one, two) != cipher(one, two)
            || vncipher(one, two) != ncipher(one, two)
            || vcipherlast(one, two) != cipherlast(one, two)
            || vncipherlast(one, two) != ncipherlast(one, two)
        {
            return false;
        }
        if vsbox(zero).to_bytes() != [0x63; 16]
            || vsbox(V128::from_bytes([1; 16])).to_bytes() != [0x7c; 16]
        {
            return false;
        }
        let pmsum_a = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a,
            0x69, 0x78,
        ];
        let pmsum_b = [
            0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa,
            0x99, 0x88,
        ];
        let pmsum_input_a = V128::from_bytes(pmsum_a);
        let pmsum_input_b = V128::from_bytes(pmsum_b);
        if vpmsumb(pmsum_input_a, pmsum_input_b).to_bytes() != vpmsumb_ref(&pmsum_a, &pmsum_b)
            || vpmsumh(pmsum_input_a, pmsum_input_b).to_bytes() != vpmsumh_ref(&pmsum_a, &pmsum_b)
            || vpmsumw(pmsum_input_a, pmsum_input_b).to_bytes() != vpmsumw_ref(&pmsum_a, &pmsum_b)
            || vpmsumd(pmsum_input_a, pmsum_input_b).to_bytes() != vpmsumd_ref(&pmsum_a, &pmsum_b)
        {
            return false;
        }
        let ones = [0xff; 16];
        if vpmsumb(V128::from_bytes(ones), V128::from_bytes(ones)).to_bytes()
            != vpmsumb_ref(&ones, &ones)
            || vpmsumh(V128::from_bytes(ones), V128::from_bytes(ones)).to_bytes()
                != vpmsumh_ref(&ones, &ones)
            || vpmsumw(V128::from_bytes(ones), V128::from_bytes(ones)).to_bytes()
                != vpmsumw_ref(&ones, &ones)
            || vpmsumd(V128::from_bytes(ones), V128::from_bytes(ones)).to_bytes()
                != vpmsumd_ref(&ones, &ones)
            || vpmsumd(zero, V128::from_bytes(ones)).to_bytes() != [0; 16]
        {
            return false;
        }
        let perm_a: [u8; 16] = core::array::from_fn(|index| index as u8);
        let perm_b: [u8; 16] = core::array::from_fn(|index| index as u8 + 0x10);
        let perm_controls = [
            [0u8; 16],
            [0x1f; 16],
            core::array::from_fn(|index| index as u8),
            core::array::from_fn(|index| (index as u8).wrapping_mul(0x13).wrapping_add(7)),
        ];
        for control in perm_controls {
            if vpermxor(
                V128::from_bytes(perm_a),
                V128::from_bytes(perm_b),
                V128::from_bytes(control),
            )
            .to_bytes()
                != vpermxor_ref(&perm_a, &perm_b, &control)
            {
                return false;
            }
        }
        if vpermxor(
            V128::from_bytes(perm_a),
            V128::from_bytes(perm_b),
            V128::from_bytes(core::array::from_fn(|index| index as u8)),
        )
        .to_bytes()
            != perm_b
        {
            return false;
        }
        let words = [0x0123_4567, 0x89ab_cdef, 0xfedc_ba98, 0x7654_3210];
        let word_input = u32x4_logical(words);
        if u32_logical_lanes(vshasigmaw::<0, 0>(word_input)) != sha256_sigma::<0, 0>(words)
            || u32_logical_lanes(vshasigmaw::<0, 0xA>(word_input)) != sha256_sigma::<0, 0xA>(words)
            || u32_logical_lanes(vshasigmaw::<1, 0>(word_input)) != sha256_sigma::<1, 0>(words)
            || u32_logical_lanes(vshasigmaw::<1, 0xF>(word_input)) != sha256_sigma::<1, 0xF>(words)
        {
            return false;
        }
        let doublewords = [0x0123_4567_89ab_cdef, 0xfedc_ba98_7654_3210];
        let doubleword_input = u64x2_logical(doublewords);
        if u64_logical_lanes(vshasigmad::<0, 0>(doubleword_input))
            != sha512_sigma::<0, 0>(doublewords)
            || u64_logical_lanes(vshasigmad::<0, 0xF>(doubleword_input))
                != sha512_sigma::<0, 0xF>(doublewords)
            || u64_logical_lanes(vshasigmad::<0, 0x2>(doubleword_input))
                != sha512_sigma::<0, 0x2>(doublewords)
            || u64_logical_lanes(vshasigmad::<1, 0>(doubleword_input))
                != sha512_sigma::<1, 0>(doublewords)
            || u64_logical_lanes(vshasigmad::<1, 0xF>(doubleword_input))
                != sha512_sigma::<1, 0xF>(doublewords)
        {
            return false;
        }
        true
    }
}

fn mma_input_words(value: &[u8; 16]) -> [u32; 4] {
    core::array::from_fn(|lane| {
        let offset = (3 - lane) * 4;
        u32::from_le_bytes(value[offset..offset + 4].try_into().unwrap())
    })
}

fn mma_sign_extend(value: u32, bits: u32) -> i64 {
    let shift = 64 - bits;
    (((value as u64) << shift) as i64) >> shift
}

fn mma_clamp(value: i64) -> i32 {
    if value > i32::MAX as i64 {
        i32::MAX
    } else if value < i32::MIN as i64 {
        i32::MIN
    } else {
        value as i32
    }
}

fn mma_ref_4(a: &[u8; 16], b: &[u8; 16], acc: [i32; 16], accumulate: bool) -> [i32; 16] {
    let a_words = mma_input_words(a);
    let b_words = mma_input_words(b);
    let mut result = [0; 16];
    for row in 0..4 {
        for column in 0..4 {
            let mut sum = 0;
            for element in 0..8 {
                let a_value = mma_sign_extend((a_words[row] >> (element * 4)) & 0xf, 4);
                let b_value = mma_sign_extend((b_words[column] >> (element * 4)) & 0xf, 4);
                sum += a_value * b_value;
            }
            let total = if accumulate {
                sum + acc[row * 4 + column] as i64
            } else {
                sum
            };
            result[row * 4 + column] = total as i32;
        }
    }
    result
}

fn mma_ref_8(
    a: &[u8; 16],
    b: &[u8; 16],
    acc: [i32; 16],
    accumulate: bool,
    saturate: bool,
) -> [i32; 16] {
    let a_words = mma_input_words(a);
    let b_words = mma_input_words(b);
    let mut result = [0; 16];
    for row in 0..4 {
        for column in 0..4 {
            let mut sum = 0;
            for element in 0..4 {
                let a_value = mma_sign_extend((a_words[row] >> (element * 8)) & 0xff, 8);
                let b_value = ((b_words[column] >> (element * 8)) & 0xff) as i64;
                sum += a_value * b_value;
            }
            let total = if accumulate {
                sum + acc[row * 4 + column] as i64
            } else {
                sum
            };
            result[row * 4 + column] = if saturate {
                mma_clamp(total)
            } else {
                total as i32
            };
        }
    }
    result
}

fn mma_ref_16(
    a: &[u8; 16],
    b: &[u8; 16],
    acc: [i32; 16],
    accumulate: bool,
    saturate: bool,
) -> [i32; 16] {
    let a_words = mma_input_words(a);
    let b_words = mma_input_words(b);
    let mut result = [0; 16];
    for row in 0..4 {
        for column in 0..4 {
            let mut sum = 0;
            for element in 0..2 {
                let a_value = mma_sign_extend((a_words[row] >> (element * 16)) & 0xffff, 16);
                let b_value = mma_sign_extend((b_words[column] >> (element * 16)) & 0xffff, 16);
                sum += a_value * b_value;
            }
            let total = if accumulate {
                sum + acc[row * 4 + column] as i64
            } else {
                sum
            };
            result[row * 4 + column] = if saturate {
                mma_clamp(total)
            } else {
                total as i32
            };
        }
    }
    result
}

fn mma_acc_from_i32(values: [i32; 16]) -> MmaAccumulator {
    let mut bytes = [0u8; 64];
    for row in 0..4 {
        for column in 0..4 {
            let offset = (3 - row) * 16 + (3 - column) * 4;
            bytes[offset..offset + 4].copy_from_slice(&values[row * 4 + column].to_le_bytes());
        }
    }
    MmaAccumulator::from_bytes(bytes)
}

fn mma_acc_i32(acc: &MmaAccumulator) -> [i32; 16] {
    let bytes = acc.to_bytes();
    core::array::from_fn(|index| {
        let row = index / 4;
        let column = index % 4;
        let offset = (3 - row) * 16 + (3 - column) * 4;
        i32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    })
}

fn mma_ops() -> bool {
    if core::mem::size_of::<MmaAccumulator>() != 64 || core::mem::align_of::<MmaAccumulator>() != 16
    {
        return false;
    }
    unsafe {
        let values = [
            V128::from_bytes([1; 16]),
            V128::from_bytes([2; 16]),
            V128::from_bytes([3; 16]),
            V128::from_bytes([4; 16]),
        ];
        let mut acc = MmaAccumulator::new();
        if acc.to_bytes() != [0; 64] {
            return false;
        }
        xxmtacc(&mut acc, values);
        if xxmfacc(&acc) != values {
            return false;
        }
        xxsetaccz(&mut acc);
        if acc.to_bytes() != [0; 64] {
            return false;
        }

        let init: [i32; 16] = core::array::from_fn(|index| index as i32 - 8);
        let a4 = V128::from_bytes([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x10, 0x32, 0x54, 0x76, 0x98, 0xba,
            0xdc, 0xfe,
        ]);
        let b4 = V128::from_bytes([
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
            0xff, 0x00,
        ]);
        let mut acc = mma_acc_from_i32(init);
        xvi4ger8(&mut acc, a4, b4);
        if mma_acc_i32(&acc) != mma_ref_4(&a4.to_bytes(), &b4.to_bytes(), init, false) {
            return false;
        }
        let mut acc = mma_acc_from_i32(init);
        xvi4ger8pp(&mut acc, a4, b4);
        if mma_acc_i32(&acc) != mma_ref_4(&a4.to_bytes(), &b4.to_bytes(), init, true) {
            return false;
        }

        let a8 = V128::from_bytes([
            0x02, 0xfe, 0x7f, 0x80, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa,
            0xbb, 0xcc,
        ]);
        let b8 = V128::from_bytes([
            0x03, 0xff, 0x01, 0x80, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xa9, 0xba,
            0xcb, 0xdc,
        ]);
        let mut acc = mma_acc_from_i32(init);
        xvi8ger4(&mut acc, a8, b8);
        if mma_acc_i32(&acc) != mma_ref_8(&a8.to_bytes(), &b8.to_bytes(), init, false, false) {
            return false;
        }
        let mut acc = mma_acc_from_i32(init);
        xvi8ger4pp(&mut acc, a8, b8);
        if mma_acc_i32(&acc) != mma_ref_8(&a8.to_bytes(), &b8.to_bytes(), init, true, false) {
            return false;
        }
        let mut acc = mma_acc_from_i32(init);
        xvi8ger4spp(&mut acc, a8, b8);
        if mma_acc_i32(&acc) != mma_ref_8(&a8.to_bytes(), &b8.to_bytes(), init, true, true) {
            return false;
        }

        let a16 = V128::from_bytes([
            0x01, 0x00, 0xff, 0xff, 0x02, 0x00, 0xfe, 0xff, 0x03, 0x00, 0xfd, 0xff, 0x04, 0x00,
            0xfc, 0xff,
        ]);
        let b16 = V128::from_bytes([
            0x05, 0x00, 0xfb, 0xff, 0x06, 0x00, 0xfa, 0xff, 0x07, 0x00, 0xf9, 0xff, 0x08, 0x00,
            0xf8, 0xff,
        ]);
        let mut acc = mma_acc_from_i32(init);
        xvi16ger2(&mut acc, a16, b16);
        if mma_acc_i32(&acc) != mma_ref_16(&a16.to_bytes(), &b16.to_bytes(), init, false, false) {
            return false;
        }
        let mut acc = mma_acc_from_i32(init);
        xvi16ger2s(&mut acc, a16, b16);
        if mma_acc_i32(&acc) != mma_ref_16(&a16.to_bytes(), &b16.to_bytes(), init, false, true) {
            return false;
        }
        let mut acc = mma_acc_from_i32(init);
        xvi16ger2pp(&mut acc, a16, b16);
        if mma_acc_i32(&acc) != mma_ref_16(&a16.to_bytes(), &b16.to_bytes(), init, true, false) {
            return false;
        }
        let mut acc = mma_acc_from_i32(init);
        xvi16ger2spp(&mut acc, a16, b16);
        if mma_acc_i32(&acc) != mma_ref_16(&a16.to_bytes(), &b16.to_bytes(), init, true, true) {
            return false;
        }

        let max = [i32::MAX; 16];
        let mut acc = mma_acc_from_i32(max);
        xvi8ger4spp(
            &mut acc,
            V128::from_bytes([2; 16]),
            V128::from_bytes([3; 16]),
        );
        if mma_acc_i32(&acc) != max {
            return false;
        }
        let min = [i32::MIN; 16];
        let mut acc = mma_acc_from_i32(min);
        xvi8ger4spp(
            &mut acc,
            V128::from_bytes([0xfe; 16]),
            V128::from_bytes([3; 16]),
        );
        if mma_acc_i32(&acc) != min {
            return false;
        }
        let mut acc = mma_acc_from_i32(max);
        xvi16ger2spp(
            &mut acc,
            V128::from_bytes([1; 16]),
            V128::from_bytes([1; 16]),
        );
        if mma_acc_i32(&acc) != max {
            return false;
        }
        let mut acc = mma_acc_from_i32(min);
        xvi16ger2spp(
            &mut acc,
            V128::from_bytes([0xff; 16]),
            V128::from_bytes([1; 16]),
        );
        if mma_acc_i32(&acc) != min {
            return false;
        }
        true
    }
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

fn ops() -> bool {
    if !ops_arithmetic() {
        return false;
    }
    if !ops_bits() {
        return false;
    }
    ops_wide()
}

fn ops_arithmetic() -> bool {
    unsafe {
        if add_i8(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [3; 16] {
            return false;
        }
        if i16_lanes(add_i16(i16x8([1; 8]), i16x8([2; 8]))) != [3; 8] {
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
        lanes(mul_i32(i32x4([2, 3, 4, 5]), i32x4([6, 7, 8, 9]))) == [12, 21, 32, 45]
    }
}

fn ops_bits() -> bool {
    unsafe {
        let a = V128::from_bytes([0b1010_1010; 16]);
        let b = V128::from_bytes([0b1100_1100; 16]);
        if and(a, b).to_bytes() != [0b1000_1000; 16]
            || or(a, b).to_bytes() != [0b1110_1110; 16]
            || xor(a, b).to_bytes() != [0b0110_0110; 16]
        {
            return false;
        }
        cmpeq_i32(i32x4([1; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16]
            && cmpgt_i32(i32x4([2; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16]
    }
}

fn ops_wide() -> bool {
    unsafe {
        let a = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        let b = V128::from_bytes([2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]);
        let sum = add_i64(a, b).to_bytes();
        let diff = sub_i64(b, a).to_bytes();
        if i64::from_le_bytes(sum[..8].try_into().unwrap()) != 3
            || i64::from_le_bytes(sum[8..].try_into().unwrap()) != 5
            || i64::from_le_bytes(diff[..8].try_into().unwrap()) != 1
            || i64::from_le_bytes(diff[8..].try_into().unwrap()) != 1
        {
            return false;
        }
        lanes(splat_i32(7)) == [7; 4]
    }
}

fn float_ops() -> bool {
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    unsafe {
        f32_lanes(fadd(a, b)) == [11.0, 22.0, 33.0, 44.0]
            && f32_lanes(fsub(b, a)) == [9.0, 18.0, 27.0, 36.0]
            && f32_lanes(fmul(a, b)) == [10.0, 40.0, 90.0, 160.0]
            && f32_lanes(fdiv(b, a)) == [10.0; 4]
    }
}

fn permute_ops() -> bool {
    let a = V128::from_bytes(core::array::from_fn(|index| index as u8));
    let b = V128::from_bytes(core::array::from_fn(|index| index as u8 + 16));
    let identity = V128::from_bytes(core::array::from_fn(|index| index as u8));
    unsafe {
        permute(a, b, identity).to_bytes() == a.to_bytes()
            && permute(a, b, V128::from_bytes([0x10; 16])).to_bytes() == [16; 16]
    }
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

fn altivec_ops() -> bool {
    unsafe {
        vaddubm(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [3; 16]
            && vsububm(V128::from_bytes([3; 16]), V128::from_bytes([2; 16])).to_bytes() == [1; 16]
            && vand(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes()
                == [0x88; 16]
            && vandc(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes()
                == [0x22; 16]
            && vor(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes()
                == [0xee; 16]
            && vxor(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes()
                == [0x66; 16]
            && vnor(V128::from_bytes([0xaa; 16]), V128::from_bytes([0xcc; 16])).to_bytes()
                == [0x11; 16]
            && vnot(V128::from_bytes([0xaa; 16])).to_bytes() == [0x55; 16]
            && vsel(
                V128::from_bytes([1; 16]),
                V128::from_bytes([2; 16]),
                V128::from_bytes([0xff; 16]),
            )
            .to_bytes()
                == [2; 16]
            && vcmpeq(i32x4([1; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16]
            && vcmpgt(i32x4([2; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16]
            && vavg(V128::from_bytes([1; 16]), V128::from_bytes([3; 16])).to_bytes() == [2; 16]
            && vavgl(V128::from_bytes([1; 16]), V128::from_bytes([3; 16])).to_bytes() == [2; 16]
            && vmax(i32x4([1, 4, 2, 8]), i32x4([3, 2, 6, 4])).to_bytes() != [0; 16]
            && vmin(i32x4([1, 4, 2, 8]), i32x4([3, 2, 6, 4])).to_bytes() != [0; 16]
            && vsl(i32x4([2, 4, 6, 8]), i32x4([1; 4])).to_bytes() != [0; 16]
            && vsr(i32x4([4, 8, 12, 16]), i32x4([1; 4])).to_bytes() != [0; 16]
            && vmuleub(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0; 16]
            && vmuleuh(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0; 16]
            && av_muleuw(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0; 16]
            && vmuloub(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0; 16]
            && vmulouh(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0; 16]
            && av_mulouw(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0; 16]
            && vsldoi::<4>(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes()
                != [0; 16]
            && vperm(
                V128::from_bytes([1; 16]),
                V128::from_bytes([2; 16]),
                V128::from_bytes([0; 16]),
            )
            .to_bytes()
                != [0; 16]
            && vslo(V128::from_bytes([1; 16]), V128::from_bytes([0; 16])).to_bytes() != [0; 16]
            && vsro(V128::from_bytes([1; 16]), V128::from_bytes([0; 16])).to_bytes() != [0; 16]
            && vadduhm(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [3; 16]
            && vadduwm(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [3; 16]
            && vaddudm(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [3; 16]
            && vsubuhm(V128::from_bytes([3; 16]), V128::from_bytes([2; 16])).to_bytes() == [1; 16]
            && vsubuwm(V128::from_bytes([3; 16]), V128::from_bytes([2; 16])).to_bytes() == [1; 16]
            && vsubudm(V128::from_bytes([3; 16]), V128::from_bytes([2; 16])).to_bytes() == [1; 16]
    }
}
