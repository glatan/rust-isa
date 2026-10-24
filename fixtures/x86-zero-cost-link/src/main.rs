use std::hint::black_box;

use rust_isa::x86::crypto::gfni::gf2p8affineqb;
use rust_isa::x86::crypto::sha::sha256rnds2;
use rust_isa::x86::crypto::vaes::vaesenc;
use rust_isa::x86::crypto::vpclmulqdq::vpclmulqdq_v256;
use rust_isa::x86::simd::avx::{V256, vaddps, vmulps};
use rust_isa::x86::simd::avx2::vpshufb;
use rust_isa::x86::simd::avx10_1::{vpaddd_v512, vpmulld_v512};
use rust_isa::x86::simd::avx512bf16::vdpbf16ps;
use rust_isa::x86::simd::avx512bitalg::vpopcntb;
use rust_isa::x86::simd::avx512bw::vpaddb;
use rust_isa::x86::simd::avx512cd::vplzcntd;
use rust_isa::x86::simd::avx512dq::vpmullq;
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512fp16::vaddph;
use rust_isa::x86::simd::avx512ifma::vpmadd52luq;
use rust_isa::x86::simd::avx512vbmi::vpermb;
use rust_isa::x86::simd::avx512vbmi2::vpcompressb;
use rust_isa::x86::simd::avx512vnni::vpdpbusd;
use rust_isa::x86::simd::avx512vp2intersect::vp2intersectd;
use rust_isa::x86::simd::avx512vpopcntdq::vpopcntd;
use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::sse3::addsubps;
use rust_isa::x86::simd::sse4_1::pmulld;
use rust_isa::x86::simd::sse4_2::pcmpgtq;
use rust_isa::x86::simd::ssse3::pshufb;

#[inline(never)]
#[target_feature(enable = "sse3")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sse3(a: &V128, b: &V128) -> V128 {
    unsafe { addsubps(addsubps(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "ssse3")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_ssse3(a: &V128, b: &V128) -> V128 {
    unsafe { pshufb(pshufb(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "sse4.1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sse41(a: &V128, b: &V128) -> V128 {
    unsafe { pmulld(pmulld(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "sse4.2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sse42(a: &V128, b: &V128) -> V128 {
    unsafe { pcmpgtq(pcmpgtq(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "avx")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_avx(a: &V256, b: &V256) -> V256 {
    let sum = unsafe { vaddps(vaddps(*a, *b), *b) };
    unsafe { vmulps(vmulps(sum, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512cd")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_cd(a: &V512) -> V512 {
    unsafe { vplzcntd(vplzcntd(*a)) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_vbmi(a: &V512, idx: &V512) -> V512 {
    unsafe { vpermb(vpermb(*a, *idx), *idx) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512vpopcntdq")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_vpopcnt(a: &V512) -> V512 {
    unsafe { vpopcntd(vpopcntd(*a)) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512bitalg")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_bitalg(a: &V512) -> V512 {
    unsafe { vpopcntb(vpopcntb(*a)) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512vbmi2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_vbmi2(a: &V512, mask: u64) -> V512 {
    unsafe { vpcompressb(vpcompressb(*a, mask), mask) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512bf16")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_bf16(acc: &V512, a: &V512, b: &V512) -> V512 {
    unsafe { vdpbf16ps(vdpbf16ps(*acc, *a, *b), *a, *b) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512fp16")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_fp16(a: &V512, b: &V512, c: &V512) -> V512 {
    unsafe { vaddph(vaddph(*a, *b), *c) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512vp2intersect")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_vp2intersect(a: &V512, b: &V512) -> (u16, u16) {
    let (a1, b1) = unsafe { vp2intersectd(*a, *b) };
    let (a2, b2) = unsafe { vp2intersectd(*b, *a) };
    (a1 | a2, b1 | b2)
}

#[inline(never)]
#[target_feature(enable = "sha")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sha(a: &V128, b: &V128, k: &V128) -> V128 {
    unsafe { sha256rnds2(sha256rnds2(*a, *b, *k), *b, *k) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512bw")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_bw(a: &V512, b: &V512) -> V512 {
    unsafe { vpaddb(vpaddb(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512dq")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_dq(a: &V512, b: &V512) -> V512 {
    unsafe { vpmullq(vpmullq(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512ifma")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_ifma(acc: &V512, a: &V512, b: &V512) -> V512 {
    unsafe { vpmadd52luq(vpmadd52luq(*acc, *a, *b), *a, *b) }
}

#[inline(never)]
#[target_feature(enable = "avx512f,avx512vnni")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_vnni(acc: &V512, a: &V512, b: &V512) -> V512 {
    unsafe { vpdpbusd(vpdpbusd(*acc, *a, *b), *a, *b) }
}

#[inline(never)]
#[target_feature(enable = "gfni")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_gfni(a: &V128, b: &V128) -> V128 {
    unsafe { gf2p8affineqb::<0>(gf2p8affineqb::<0>(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "avx2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_pshufb(a: &V256, b: &V256) -> V256 {
    unsafe { vpshufb(vpshufb(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "avx2,vaes")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_vaes(a: &V256, b: &V256) -> V256 {
    unsafe { vaesenc(vaesenc(*a, *b), *b) }
}

#[inline(never)]
#[target_feature(enable = "avx2,avx512f,avx512vl,vpclmulqdq")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_vpclmul(a: &V256, b: &V256) -> V256 {
    unsafe { vpclmulqdq_v256(vpclmulqdq_v256(*a, *b, 0), *b, 0) }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_avx10_1(a: &V512, b: &V512) -> V512 {
    let sum = unsafe { vpaddd_v512(vpaddd_v512(*a, *b), *b) };
    unsafe { vpmulld_v512(vpmulld_v512(sum, *b), *b) }
}

fn main() {
    let a128 = black_box(V128::from_bytes([1; 16]));
    let b128 = black_box(V128::from_bytes([2; 16]));
    let k128 = black_box(V128::from_bytes([3; 16]));
    let a256 = black_box(V256::from_bytes([1; 32]));
    let b256 = black_box(V256::from_bytes([2; 32]));
    let a512 = black_box(V512::from_bytes([1; 64]));
    let b512 = black_box(V512::from_bytes([2; 64]));

    black_box(unsafe { kernel_sse3(&a128, &b128) });
    black_box(unsafe { kernel_sse41(&a128, &b128) });
    black_box(unsafe { kernel_sse42(&a128, &b128) });
    black_box(unsafe { kernel_avx(&a256, &b256) });
    black_box(unsafe { kernel_cd(&a512) });
    black_box(unsafe { kernel_vbmi(&a512, &b512) });
    black_box(unsafe { kernel_vpopcnt(&a512) });
    black_box(unsafe { kernel_bitalg(&a512) });
    black_box(unsafe { kernel_vbmi2(&a512, black_box(0x5555_5555_5555_5555)) });
    black_box(unsafe { kernel_bf16(&a512, &a512, &b512) });
    black_box(unsafe { kernel_fp16(&a512, &b512, &a512) });
    black_box(unsafe { kernel_vp2intersect(&a512, &b512) });
    black_box(unsafe { kernel_ssse3(&a128, &b128) });
    black_box(unsafe { kernel_sha(&a128, &b128, &k128) });
    black_box(unsafe { kernel_bw(&a512, &b512) });
    black_box(unsafe { kernel_dq(&a512, &b512) });
    black_box(unsafe { kernel_ifma(&a512, &a512, &b512) });
    black_box(unsafe { kernel_vnni(&a512, &a512, &b512) });
    black_box(unsafe { kernel_gfni(&a128, &b128) });
    black_box(unsafe { kernel_pshufb(&a256, &b256) });
    black_box(unsafe { kernel_vaes(&a256, &b256) });
    black_box(unsafe { kernel_vpclmul(&a256, &b256) });
    black_box(unsafe { kernel_avx10_1(&a512, &b512) });
}
