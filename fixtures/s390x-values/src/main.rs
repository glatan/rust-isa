#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use rust_isa::s390x::simd::nnp_assist::{vcfn, vclfnh, vclfnl, vcnf, vcrnf};
use rust_isa::s390x::simd::vector::{
    V128, add, add_i8, add_i16, add_i64, and, cmpeq_i32, cmpgt_i32, fadd, fdiv, fmul, fsub, mul,
    mul_i16, mul_i32, or, permute, splat_i32, sub_i8, sub_i16, sub_i32, sub_i64, v128_load,
    v128_store, vab, vaf, vag, vah, vavg, vavgl, vceq, vch, vcl, vmax, vmin, vsb, vsel, vsf, vsg,
    vsh, vsl, vsr, xor,
};
use rust_isa::s390x::simd::vector::{vchl, vclf, vclg, vclh, vle, vperm, vpks, vst};
use rust_isa::s390x::simd::vector::{vcksm, vlef, vme, vmh, vml, vmo, vpkls, vstef};
use rust_isa::s390x::simd::vector::{
    vfae, vfee, vfene, vfi_f64, vftci_f64, vgef, vgeg, vistr, vlgv, vmle, vmlh, vmlo, vrep, vscbi,
    vseg, vstrc,
};
use rust_isa::s390x::simd::vector::{vgbm, vrepb, vrepf, vrepg, vreph, vuph, vupl, vuplh, vupll};
use rust_isa::s390x::simd::vector_enhancements_1::{
    cntlz_i32, fma, popcnt_i8, popcnt_i32, vclz, vclzg, vctz, vctzg, vfa, vfce, vfch, vfche, vfd,
    vfi_f32, vfll, vflr, vfm, vfma, vfms, vfnma, vfnms, vfs, vfsq, vftci_f32, vnn, vpopct,
};
use rust_isa::s390x::simd::vector_enhancements_2::{vpopctg, vsld, vsrd};
use rust_isa::s390x::simd::vector_packed_decimal::{
    vap, vcvb, vcvbg, vcvd, vpkz, vsdp, vsp, vsrp, vstrl, vupkz, vupkzh, vupkzl,
};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let code = run();
    unsafe {
        asm!(
            "svc 1",
            in("r1") 1usize,
            in("r2") code as usize,
            options(noreturn, nostack)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "svc 1",
            in("r1") 1usize,
            in("r2") 255usize,
            options(noreturn, nostack)
        );
    }
}

/// Calls the instructions that QEMU does not implement so that their
/// assembly is checked at build time. This function is never executed.
#[unsafe(no_mangle)]
pub extern "C" fn compile_only() {
    unsafe {
        let x = V128::from_bytes([0; 16]);
        let mut buffer = [0u8; 32];
        let _ = vap::<0x05, 0>(x, x);
        let _ = vsp::<0x05, 0>(x, x);
        let _ = vsdp::<0x08, 0>(x, x);
        let _ = vsrp::<0, 0, 0x05>(x);
        let _ = vpkz::<5>(buffer.as_ptr());
        vupkz::<5>(x, buffer.as_mut_ptr());
        let _ = vupkzh::<0>(x);
        let _ = vupkzl::<0>(x);
        vstrl::<5>(x, buffer.as_mut_ptr());
        let _ = vcvd::<0x05, 0>(1);
        let _: u32 = vcvb::<0>(x);
        let _: u64 = vcvbg::<0>(x);
        let _ = vcfn(x);
        let _ = vcnf(x);
        let _ = vclfnh(x);
        let _ = vclfnl(x);
        let _ = vcrnf(x, x);
    }
}

#[target_feature(enable = "vector")]
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_add(a: &V128, b: &V128) -> V128 {
    unsafe { add(add(*a, *b), *b) }
}

#[target_feature(enable = "vector")]
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
    if !integer_ops() {
        return 3;
    }
    if !float_ops() {
        return 4;
    }
    if !permute_ops() {
        return 5;
    }
    if !enhancement_ops() {
        return 6;
    }
    if !vector_more_ops() {
        return 7;
    }
    if !find_and_string_ops() {
        return 8;
    }
    if !enhancement_more_ops() {
        return 9;
    }
    if !double_shift_ops() {
        return 10;
    }
    0
}

fn enhancement_ops() -> bool {
    unsafe {
        if lanes(cntlz_i32(i32x4([1, 2, 3, 4]))) != [31, 30, 30, 29] {
            return false;
        }
        if popcnt_i8(V128::from_bytes([0xff; 16])).to_bytes() != [8; 16] {
            return false;
        }
        if lanes(popcnt_i32(i32x4([-1; 4]))) != [32; 4] {
            return false;
        }
        let acc = f32x4([1.0; 4]);
        let a = f32x4([2.0; 4]);
        let b = f32x4([3.0; 4]);
        if f32_lanes(fma(acc, a, b)) != [7.0; 4]
            || f32_lanes(vfa(a, b)) != [5.0; 4]
            || f32_lanes(vfs(b, a)) != [1.0; 4]
            || f32_lanes(vfm(a, b)) != [6.0; 4]
            || f32_lanes(vfd(b, a)) != [1.5; 4]
            || f32_lanes(vfma(acc, a, b)) != [7.0; 4]
            || vfms(acc, a, b).to_bytes() == [0; 16]
            || vfnma(acc, a, b).to_bytes() == [0; 16]
            || vfnms(acc, a, b).to_bytes() == [0; 16]
            || f32_lanes(vfsq(f32x4([4.0; 4]))) != [2.0; 4]
            || vfce(a, a).to_bytes() != [0xff; 16]
            || vfch(b, a).to_bytes() != [0xff; 16]
            || vfche(a, a).to_bytes() != [0xff; 16]
            || vclz(V128::from_bytes([1; 16])).to_bytes() != [7; 16]
            || vctz(V128::from_bytes([2; 16])).to_bytes() != [1; 16]
            || vpopct(V128::from_bytes([0xff; 16])).to_bytes() != [8; 16]
            || {
                let b = vclzg(V128::from_bytes([0; 16])).to_bytes();
                b[7] != 64 || b[15] != 64
            }
            || {
                let b = vctzg(V128::from_bytes([0; 16])).to_bytes();
                b[7] != 64 || b[15] != 64
            }
            || {
                let b = vpopctg(V128::from_bytes([0xff; 16])).to_bytes();
                b[7] != 64 || b[15] != 64
            }
            || {
                let mut input = [0u8; 16];
                input[8..].copy_from_slice(&1u64.to_be_bytes());
                let b = vclzg(V128::from_bytes(input)).to_bytes();
                b[7] != 64 || b[15] != 63
            }
            || {
                let mut input = [0u8; 16];
                input[8..].copy_from_slice(&2u64.to_be_bytes());
                let b = vctzg(V128::from_bytes(input)).to_bytes();
                b[7] != 64 || b[15] != 1
            }
            || {
                let mut input = [0xffu8; 16];
                input[8..].fill(0);
                let b = vpopctg(V128::from_bytes(input)).to_bytes();
                b[7] != 64 || b[15] != 0
            }
        {
            return false;
        }
        true
    }
}

fn i16x8(values: [i16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_be_bytes());
    }
    V128::from_bytes(bytes)
}

fn i16_lanes(value: V128) -> [i16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i16::from_be_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&value.to_be_bytes());
    }
    V128::from_bytes(bytes)
}

fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_be_bytes());
    }
    V128::from_bytes(bytes)
}

fn f32_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_be_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn integer_ops() -> bool {
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
        if lanes(mul_i32(i32x4([2, 3, 4, 5]), i32x4([6, 7, 8, 9]))) != [12, 21, 32, 45] {
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
        let a = V128::from_bytes([0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2]);
        let b = V128::from_bytes([0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3]);
        let sum = add_i64(a, b).to_bytes();
        let diff = sub_i64(b, a).to_bytes();
        if i64::from_be_bytes(sum[..8].try_into().unwrap()) != 3
            || i64::from_be_bytes(sum[8..].try_into().unwrap()) != 5
            || i64::from_be_bytes(diff[..8].try_into().unwrap()) != 1
            || i64::from_be_bytes(diff[8..].try_into().unwrap()) != 1
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
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_be_bytes());
    }
    V128::from_bytes(bytes)
}

fn lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|index| {
        i32::from_be_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
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

fn vector_more_ops() -> bool {
    unsafe {
        vab(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [3; 16]
            && vah(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [3; 16]
            && vaf(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [3; 16]
            && vag(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() == [3; 16]
            && vsb(V128::from_bytes([3; 16]), V128::from_bytes([2; 16])).to_bytes() == [1; 16]
            && vsh(V128::from_bytes([3; 16]), V128::from_bytes([2; 16])).to_bytes() == [1; 16]
            && vsf(V128::from_bytes([3; 16]), V128::from_bytes([2; 16])).to_bytes() == [1; 16]
            && vsg(V128::from_bytes([3; 16]), V128::from_bytes([2; 16])).to_bytes() == [1; 16]
            && vmin(i32x4([1; 4]), i32x4([2; 4])).to_bytes() != [0; 16]
            && vmax(i32x4([1; 4]), i32x4([2; 4])).to_bytes() != [0; 16]
            && vavg(V128::from_bytes([1; 16]), V128::from_bytes([3; 16])).to_bytes() != [0; 16]
            && vavgl(V128::from_bytes([1; 16]), V128::from_bytes([3; 16])).to_bytes() != [0; 16]
            && vceq(i32x4([1; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16]
            && vch(i32x4([2; 4]), i32x4([1; 4])).to_bytes() == [0xff; 16]
            && vcl(i32x4([1; 4]), i32x4([2; 4])).to_bytes() == [0xff; 16]
            && vsel(
                V128::from_bytes([1; 16]),
                V128::from_bytes([2; 16]),
                V128::from_bytes([0xff; 16]),
            )
            .to_bytes()
                == [1; 16]
            && vsl(
                V128::from_bytes([1; 16]),
                V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]),
            )
            .to_bytes()
                != [0; 16]
            && vsr(
                V128::from_bytes([2; 16]),
                V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]),
            )
            .to_bytes()
                != [0; 16]
            && vclf(f32x4([1.0, 4.0, 2.0, 8.0]), f32x4([3.0, 2.0, 6.0, 4.0])).to_bytes() != [0; 16]
            && vclg(f64x2([1.0, 4.0]), f64x2([3.0, 2.0])).to_bytes() != [0; 16]
            && vclh(i16x8([1; 8]), i16x8([2; 8])).to_bytes() == [0xff; 16]
            && vchl(
                V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]),
                V128::from_bytes([0; 16]),
            )
            .to_bytes()
                == [0xff; 16]
            && vpks(i32x4([1; 4]), i32x4([0; 4])).to_bytes() != [0; 16]
            && {
                let source = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
                let loaded = vle(source.as_ptr());
                let mut output = [0u8; 16];
                vst(output.as_mut_ptr(), loaded);
                output == source
            }
            && vperm(
                V128::from_bytes([1; 16]),
                V128::from_bytes([2; 16]),
                V128::from_bytes([0; 16]),
            )
            .to_bytes()
                != [0; 16]
            && vcksm(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes() != [0; 16]
            && vme(
                V128::from_bytes([1; 16]),
                V128::from_bytes([1; 16]),
                V128::from_bytes([0; 16]),
            )
            .to_bytes()
                != [0; 16]
            && vmh(
                V128::from_bytes([1; 16]),
                V128::from_bytes([1; 16]),
                V128::from_bytes([0; 16]),
            )
            .to_bytes()
                != [0; 16]
            && vml(
                V128::from_bytes([1; 16]),
                V128::from_bytes([1; 16]),
                V128::from_bytes([0; 16]),
            )
            .to_bytes()
                != [0; 16]
            && vmo(
                V128::from_bytes([1; 16]),
                V128::from_bytes([1; 16]),
                V128::from_bytes([0; 16]),
            )
            .to_bytes()
                != [0; 16]
            && vpkls(i32x4([1; 4]), i32x4([0; 4])).to_bytes() != [0; 16]
            && {
                let source = [1.0f32, 2.0, 3.0, 4.0];
                let loaded = vlef(source.as_ptr());
                let mut output = [0.0f32; 4];
                vstef(output.as_mut_ptr(), loaded);
                output == source
            }
            && vgbm::<0xffff>().to_bytes() == [0xff; 16]
            && vgbm::<0x0000>().to_bytes() == [0; 16]
            && {
                let b = vrepb::<0>(V128::from_bytes(core::array::from_fn(|i| i as u8))).to_bytes();
                b.iter().all(|&x| x == b[0])
            }
            && {
                let b = vreph::<0>(V128::from_bytes(core::array::from_fn(|i| i as u8))).to_bytes();
                b.chunks_exact(2).all(|c| c == &b[..2])
            }
            && {
                let b = vrepf::<0>(V128::from_bytes(core::array::from_fn(|i| i as u8))).to_bytes();
                b.chunks_exact(4).all(|c| c == &b[..4])
            }
            && {
                let b = vrepg::<0>(V128::from_bytes(core::array::from_fn(|i| i as u8))).to_bytes();
                b.chunks_exact(8).all(|c| c == &b[..8])
            }
            && vuph(V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && vupl(V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && vuplh(V128::from_bytes([1; 16])).to_bytes() != [0; 16]
            && vupll(V128::from_bytes([1; 16])).to_bytes() != [0; 16]
    }
}

fn u16_lanes(value: V128) -> [u16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_be_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn u32x4(values: [u32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_be_bytes());
    }
    V128::from_bytes(bytes)
}

fn u64x2(values: [u64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_be_bytes());
    }
    V128::from_bytes(bytes)
}

fn u64_lanes(value: V128) -> [u64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u64::from_be_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_be_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn find_and_string_ops() -> bool {
    unsafe {
        let ascending = V128::from_bytes(core::array::from_fn(|i| i as u8));
        if vrep::<0, 5>(ascending).to_bytes() != [5; 16] {
            return false;
        }
        if vrep::<2, 3>(ascending).to_bytes()
            != [
                12, 13, 14, 15, 12, 13, 14, 15, 12, 13, 14, 15, 12, 13, 14, 15,
            ]
        {
            return false;
        }
        if vrep::<3, 1>(ascending).to_bytes()
            != [8, 9, 10, 11, 12, 13, 14, 15, 8, 9, 10, 11, 12, 13, 14, 15]
        {
            return false;
        }

        let mut bytes = [0u8; 16];
        bytes[7] = 0x80;
        bytes[15] = 0x7f;
        if u64_lanes(vseg::<0>(V128::from_bytes(bytes))) != [(-128i64) as u64, 127] {
            return false;
        }
        let words = u32x4([0, 0xffff_ffff, 0, 1]);
        if u64_lanes(vseg::<2>(words)) != [u64::MAX, 1] {
            return false;
        }

        let a = V128::from_bytes(core::array::from_fn(|i| i as u8 + 1));
        let ones = V128::from_bytes([1; 16]);
        if u16_lanes(vmle::<0>(a, ones)) != [1, 3, 5, 7, 9, 11, 13, 15] {
            return false;
        }
        if u16_lanes(vmlo::<0>(a, ones)) != [2, 4, 6, 8, 10, 12, 14, 16] {
            return false;
        }
        if vmlh::<0>(V128::from_bytes([0x10; 16]), V128::from_bytes([0x10; 16])).to_bytes()
            != [1; 16]
        {
            return false;
        }
        if lanes(vmlh::<2>(u32x4([0x0001_0000; 4]), u32x4([0x0001_0000; 4]))) != [1; 4] {
            return false;
        }

        let string = V128::from_bytes([1, 2, 0, 4, 5, 0, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        if vistr::<0>(string).to_bytes() != [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
            return false;
        }

        let strings = V128::from_bytes([1, 5, 3, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let ranges = V128::from_bytes([5; 16]);
        let controls = V128::from_bytes([0x80; 16]);
        let mask = vstrc::<0, 4>(strings, ranges, controls).to_bytes();
        if mask[0] != 0 || mask[1] != 0xff || mask[2] != 0 || mask[3] != 0 {
            return false;
        }
        let index = vstrc::<0, 0>(strings, ranges, controls).to_bytes();
        if u64::from_be_bytes(index[..8].try_into().unwrap()) != 1 {
            return false;
        }

        let any = V128::from_bytes([1, 2, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let needles = V128::from_bytes([5, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let mask = vfae::<0, 4>(any, needles).to_bytes();
        if mask[0] != 0 || mask[1] != 0 || mask[2] != 0xff {
            return false;
        }
        let index = vfee::<0, 0>(
            V128::from_bytes([1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            V128::from_bytes([9, 9, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
        .to_bytes();
        if u64::from_be_bytes(index[..8].try_into().unwrap()) != 2 {
            return false;
        }
        let index = vfene::<0, 0>(
            V128::from_bytes([1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            V128::from_bytes([1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        )
        .to_bytes();
        if u64::from_be_bytes(index[..8].try_into().unwrap()) != 1 {
            return false;
        }

        if vscbi::<0>(V128::from_bytes([0; 16]), V128::from_bytes([1; 16])).to_bytes() != [0; 16] {
            return false;
        }
        if vscbi::<0>(V128::from_bytes([1; 16]), V128::from_bytes([0; 16])).to_bytes() != [1; 16] {
            return false;
        }

        if vlgv::<2, 1>(i32x4([10, 20, 30, 40])) != 20 {
            return false;
        }
        if vlgv::<0, 15>(ascending) != 15 || vlgv::<1, 7>(ascending) != 0x0e0f {
            return false;
        }

        let base = [10u8, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0];
        let offsets = u32x4([0, 8, 0, 0]);
        let gathered = vgef::<1>(V128::from_bytes([0; 16]), offsets, base.as_ptr()).to_bytes();
        if u32::from_be_bytes(gathered[4..8].try_into().unwrap()) != 0x1e00_0000 {
            return false;
        }
        let offsets = u64x2([8, 0]);
        let gathered = vgeg::<0>(V128::from_bytes([0; 16]), offsets, base.as_ptr()).to_bytes();
        if u64::from_be_bytes(gathered[..8].try_into().unwrap()) != 0x1e00_0000_2800_0000 {
            return false;
        }

        if vftci_f64::<0x200>(f64x2([1.0, 2.0])).to_bytes() != [0xff; 16] {
            return false;
        }
        if vftci_f64::<0x200>(f64x2([0.0, 0.0])).to_bytes() != [0; 16] {
            return false;
        }
        if f64_lanes(vfi_f64::<4, 5>(f64x2([2.9, -2.9]))) != [2.0, -2.0] {
            return false;
        }
        true
    }
}
fn enhancement_more_ops() -> bool {
    unsafe {
        if vnn(V128::from_bytes([0x0f; 16]), V128::from_bytes([0x33; 16])).to_bytes() != [0xfc; 16]
        {
            return false;
        }
        if f64_lanes(vfll(f32x4([1.0, 2.0, 3.0, 4.0]))) != [1.0, 3.0] {
            return false;
        }
        if f32_lanes(vflr::<0, 0>(f64x2([1.5, 2.5]))) != [1.5, 0.0, 2.5, 0.0] {
            return false;
        }
        if vftci_f32::<0x200>(f32x4([1.0, 0.0, 3.0, 0.0])).to_bytes()
            != [
                0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
            ]
        {
            return false;
        }
        if f32_lanes(vfi_f32::<4, 5>(f32x4([2.9, -2.9, 0.0, 1.0]))) != [2.0, -2.0, 0.0, 1.0] {
            return false;
        }
        true
    }
}

fn double_shift_ops() -> bool {
    unsafe {
        let a = V128::from_bytes(core::array::from_fn(|i| i as u8));
        let b = V128::from_bytes(core::array::from_fn(|i| i as u8 + 16));
        if vsld::<0>(a, b).to_bytes() != a.to_bytes() {
            return false;
        }
        if vsrd::<0>(a, b).to_bytes() != b.to_bytes() {
            return false;
        }
        let a = V128::from_bytes([0x11; 16]);
        let b = V128::from_bytes([0x22; 16]);
        let mut expected_left = [0x11; 16];
        expected_left[15] = 0x12;
        if vsld::<4>(a, b).to_bytes() != expected_left {
            return false;
        }
        let mut expected_right = [0x22; 16];
        expected_right[0] = 0x12;
        vsrd::<4>(a, b).to_bytes() == expected_right
    }
}
