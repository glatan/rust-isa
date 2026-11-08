#![allow(unused_imports)]

use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::{Mask2, Mask4, Mask8, Mask16, Mask32, Mask64};

use super::avx512::support::*;

#[repr(align(64))]
struct Aligned([u8; 4096]);

impl Aligned {
    fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

fn avx10_2_available() -> bool {
    use core::arch::x86_64::__cpuid_count;
    let leaf7 = __cpuid_count(7, 0);
    if leaf7.eax < 1 {
        return false;
    }
    let avx10 = __cpuid_count(7, 1);
    if avx10.edx & (1 << 19) == 0 {
        return false;
    }
    let version = __cpuid_count(0x24, 0);
    (version.ebx & 0xff) >= 2
}

#[test]
#[ignore = "requires an AVX512BMM CPU"]
fn avx512bmm_smoke() {
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bmm::vbitrevb_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bmm::vbitrevb_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bmm::vbitrevb_v512(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512bmm::vbmacor16x16x16_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512bmm::vbmacor16x16x16_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512bmm::vbmacxor16x16x16_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512bmm::vbmacxor16x16x16_v512(v512, v512, v512)
        });
    }
}

#[test]
fn avx10_1_smoke() {
    if !(f_enabled()
        && is_x86_feature_detected!("avx512vl")
        && is_x86_feature_detected!("avx512bw")
        && is_x86_feature_detected!("avx512dq")
        && is_x86_feature_detected!("avx512vbmi")
        && is_x86_feature_detected!("avx512vbmi2")
        && is_x86_feature_detected!("avx512vnni")
        && is_x86_feature_detected!("avx512ifma")
        && is_x86_feature_detected!("avx512bitalg")
        && is_x86_feature_detected!("avx512vpopcntdq")
        && is_x86_feature_detected!("avx512vp2intersect"))
    {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vaddps_v512(v512, v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vcmppd_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vcmppd_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vcmpps_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vcmpps_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vcompresspd_v256(v256, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vcompresspd_v512(v512, Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vcompressps_v512(v512, Mask16::from_bits(0))
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtdq2ps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtdq2ps_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtpd2dq_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtpd2ps_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtpd2qq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtpd2uqq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtpd2uqq_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtps2dq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtps2dq_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtps2qq_v256(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtps2qq_v512(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtps2uqq_v256(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtps2uqq_v512(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtqq2pd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtqq2pd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtqq2ps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtqq2ps_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttpd2dq_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttpd2qq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttpd2qq_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttpd2uqq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttpd2uqq_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttps2dq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttps2dq_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttps2qq_v256(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttps2qq_v512(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttps2uqq_v256(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvttps2uqq_v512(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtuqq2pd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtuqq2pd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtuqq2ps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vcvtuqq2ps_v512(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vexpandpd_v256(v256, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vexpandpd_v512(v512, Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vexpandps_v256(v256, Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vexpandps_v512(v512, Mask16::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd132pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd132pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd132ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd132ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd213pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd213pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd213ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd213ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd231pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd231pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd231ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmadd231ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub132pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub132pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub132ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub132ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub213pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub213pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub213ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub213ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub231pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub231pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub231ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmaddsub231ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsub213pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsub213pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsub213ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsub213ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsub231pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsub231pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsub231ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsub231ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd132pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd132pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd132ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd132ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd213pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd213pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd213ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd213ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd231pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd231pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd231ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfmsubadd231ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd132pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd132pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd132ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd132ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd213pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd213pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd213ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd213ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd231pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd231pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd231ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmadd231ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub132pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub132pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub132ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub132ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub213pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub213pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub213ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub213ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub231pd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub231pd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub231ps_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vfnmsub231ps_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vgatherdpd_v256(
                v256,
                storage.as_ptr(),
                v128,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vgatherdpd_v512(
                v512,
                storage.as_ptr(),
                v256,
                Mask8::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vgatherdps_v512(
                v512,
                storage.as_ptr(),
                v512,
                Mask16::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vgatherqpd_v256(
                v256,
                storage.as_ptr(),
                v256,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vgatherqpd_v512(
                v512,
                storage.as_ptr(),
                v512,
                Mask8::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vgatherqps_v256(
                v128,
                storage.as_ptr(),
                v256,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vgatherqps_v512(
                v256,
                storage.as_ptr(),
                v512,
                Mask8::from_bits(0),
            )
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vmulps_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpaddb_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpaddd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpaddd_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpaddq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpaddw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpaddw_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpandd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpandd_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpandnd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpandnd_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpandnq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpandnq_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpandq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpandq_v512(v512, v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpb_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpb_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpq_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpq_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpub_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpub_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpud_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpud_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpuq_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpuq_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpuw_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpuw_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpw_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpcmpw_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpconflictd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpconflictd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpconflictq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpconflictq_v512(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpdpbusd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpdpbusd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpdpbusds_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpdpwssd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpdpwssd_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpdpwssds_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpdpwssds_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpermb_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpermd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpermd_v512(v512, v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermi2b_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermi2b_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermi2d_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermi2d_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermi2q_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermi2q_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermi2w_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermi2w_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpermq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpermq_v512(v512, v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermt2b_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermt2b_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermt2d_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermt2d_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermt2q_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermt2q_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermt2w_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpermt2w_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vplzcntd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vplzcntd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vplzcntq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vplzcntq_v512(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpmadd52huq_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpmadd52huq_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpmadd52luq_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpmadd52luq_v512(v512, v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmaddubsw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmaddubsw_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmaddwd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmaddwd_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmulhuw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmulhuw_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmulhw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmulhw_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmulld_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmulld_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmullq_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmullw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpmullw_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpopcntb_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpopcntb_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpopcntd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpopcntq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpopcntq_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpopcntw_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpopcntw_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpord_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpord_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vporq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vporq_v512(v512, v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshldd_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshldq_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshldq_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshldw_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshldw_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshrdd_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshrdd_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshrdq_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshrdq_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshrdw_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vpshrdw_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpsubb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpsubb_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpsubd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpsubd_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpsubq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpsubq_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpsubw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpsubw_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestmb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestmb_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestmd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestmd_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestmq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestmq_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestmw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestmw_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestnmb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestnmb_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestnmd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestnmq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestnmq_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestnmw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vptestnmw_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpxord_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpxord_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpxorq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vpxorq_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vrcp14pd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vrcp14pd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vrcp14ps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vrcp14ps_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vrsqrt14pd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vrsqrt14pd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vrsqrt14ps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vrsqrt14ps_v512(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vscatterdpd_v256(
                storage.as_mut_ptr(),
                v128,
                v256,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vscatterdpd_v512(
                storage.as_mut_ptr(),
                v256,
                v512,
                Mask8::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vscatterdps_v256(
                storage.as_mut_ptr(),
                v256,
                v256,
                Mask8::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vscatterdps_v512(
                storage.as_mut_ptr(),
                v512,
                v512,
                Mask16::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vscatterqpd_v256(
                storage.as_mut_ptr(),
                v256,
                v256,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vscatterqpd_v512(
                storage.as_mut_ptr(),
                v512,
                v512,
                Mask8::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vscatterqps_v256(
                storage.as_mut_ptr(),
                v256,
                v128,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_1::vscatterqps_v512(
                storage.as_mut_ptr(),
                v512,
                v256,
                Mask8::from_bits(0),
            )
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vsqrtpd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vsqrtpd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_1::vsqrtps_v256(v256) });
    }
}

#[test]
fn avx10_2_smoke() {
    if !(avx10_2_available() && f_enabled()) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vcmppd_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vcmppd_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vcmpph_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vcmpph_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vcmpps_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vcmpps_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vcomisbf16(v128, v128) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vfixupimmpd_v256::<0>(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vfixupimmpd_v512::<0>(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vfixupimmps_v256::<0>(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vfixupimmps_v512::<0>(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vfixupimmsd_v128::<0>(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vfixupimmss_v128::<0>(v128, v128, v128)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vfpclasspd_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vfpclasspd_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vfpclassph_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vfpclassph_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vfpclassps_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vfpclassps_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetexppd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetexppd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetexpph_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetexpph_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetexpps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetexpps_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetmantpd_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetmantpd_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetmantph_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetmantph_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetmantps_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vgetmantps_v512::<0>(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vgetmantsd_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vgetmantss_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vminmaxpd_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vminmaxpd_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vminmaxph_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vminmaxph_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vminmaxps_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vminmaxsd_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vminmaxsh_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vminmaxss_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vrangepd_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vrangepd_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vrangeps_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vrangeps_v512::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vrangesd_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vrangess_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrcp14pd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrcp14pd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrcp14ps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrcp14ps_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrcp14sd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrcp14ss_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vreducepd_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vreducepd_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vreduceph_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vreduceph_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vreduceps_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vreduceps_v512::<0>(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vreducesd_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx10_2::vreducess_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrndscalepd_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrndscalepd_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrndscaleph_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrndscaleph_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrndscaleps_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrndscaleps_v512::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrsqrt14pd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrsqrt14pd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrsqrt14ps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrsqrt14ps_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrsqrt14sd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vrsqrt14ss_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vscalefpd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vscalefpd_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vscalefph_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vscalefph_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vscalefps_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vscalefps_v512(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vscalefsd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vscalefss_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vsqrtpd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vsqrtpd_v512(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vsqrtps_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vsqrtsd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vsqrtss_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vucomisd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx10_2::vucomiss_v128(v128, v128) });
    }
}

#[test]
fn avx512vl_smoke() {
    if !(f_enabled()
        && is_x86_feature_detected!("avx512vl")
        && is_x86_feature_detected!("avx512bw")
        && is_x86_feature_detected!("avx512dq")
        && is_x86_feature_detected!("avx512vbmi")
        && is_x86_feature_detected!("avx512vbmi2")
        && is_x86_feature_detected!("avx512vnni")
        && is_x86_feature_detected!("avx512ifma")
        && is_x86_feature_detected!("avx512bitalg")
        && is_x86_feature_detected!("avx512vpopcntdq")
        && is_x86_feature_detected!("avx512vp2intersect"))
    {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vblendmpd_v128(v128, v128, Mask2::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vblendmpd_v256(v256, v256, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vblendmps_v128(v128, v128, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vblendmps_v256(v256, v256, Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vcompresspd_v128(v128, Mask2::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vcompresspd_v256(v256, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vcompressps_v128(v128, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vexpandpd_v128(v128, Mask2::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vexpandpd_v256(v256, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vexpandps_v128(v128, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vexpandps_v256(v256, Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vgatherdpd_v128(
                v128,
                storage.as_ptr(),
                v128,
                Mask2::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vgatherdpd_v256(
                v256,
                storage.as_ptr(),
                v128,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vgatherdps_v128(
                v128,
                storage.as_ptr(),
                v128,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vgatherqpd_v128(
                v128,
                storage.as_ptr(),
                v128,
                Mask2::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vgatherqpd_v256(
                v256,
                storage.as_ptr(),
                v256,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vgatherqps_v128(
                v128,
                storage.as_ptr(),
                v128,
                Mask2::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vgatherqps_v256(
                v128,
                storage.as_ptr(),
                v256,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vmovapd_v128(v128, v128, Mask2::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vmovapd_v256(v256, v256, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vmovaps_v128(v128, v128, Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vmovaps_v256(v256, v256, Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vp2intersectd_v256(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vp2intersectq_v128(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vp2intersectq_v256(v256, v256)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddq_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddsb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddsb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddsw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddsw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddusb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddusb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddusw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddusw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpaddw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpandd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpandd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpandnd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpandnd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpandnq_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpandnq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpandq_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpandq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpavgb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpavgb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpavgw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpavgw_v256(v256, v256) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpb_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpb_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpd_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpq_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpq_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpub_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpub_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpud_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpud_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpuq_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpuq_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpuw_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpuw_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpw_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcmpw_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpcompressw_v128(v128, Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpdpbusd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpdpbusds_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpdpwssd_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpdpwssds_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpermb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpermd_v256(v256, v256) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpermi2b_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpermi2b_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpermilpd_v128::<0>(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpermilpd_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpermilps_v128::<0>(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpermilps_v256::<0>(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpermq_v256(v256, v256) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpermt2b_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpermt2b_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpexpandb_v128(v128, Mask16::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpexpandw_v128(v128, Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmadd52huq_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmadd52luq_v256(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmaddubsw_v128(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmaddubsw_v256(v256, v256)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmaddwd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmaddwd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmaxsb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmaxsb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmaxsw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmaxsw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmaxub_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmaxub_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpminsb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpminsb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpminsw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpminsw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpminub_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpminub_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmovb2m_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmovb2m_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmovd2m_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmovd2m_v256(v256) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmovm2b_v128(Mask16::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmovm2b_v256(Mask32::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmovm2d_v128(Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmovm2d_v256(Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmovm2q_v128(Mask2::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmovm2q_v256(Mask4::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmovm2w_v128(Mask8::from_bits(0))
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpmovm2w_v256(Mask16::from_bits(0))
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmovq2m_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmovq2m_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmovw2m_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmovw2m_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmulhuw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmulhuw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmulhw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmulhw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmullw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpmullw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpopcntb_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpopcntd_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpopcntd_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpopcntq_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpopcntq_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpopcntw_v128(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpopcntw_v256(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpord_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpord_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vporq_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vporq_v256(v256, v256) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshldq_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshldq_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshldw_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshldw_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshrdd_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshrdd_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshrdq_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshrdw_v128::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshrdw_v256::<0>(v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpshufbitqmb_v256(v256, v256)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubq_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubsb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubsb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubsw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubsw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubusb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubusb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubusw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubusw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpsubw_v256(v256, v256) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpternlogd_v128::<0>(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpternlogd_v256::<0>(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpternlogq_v128::<0>(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vpternlogq_v256::<0>(v256, v256, v256)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestmb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestmd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestmd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestmq_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestmq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestmw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestmw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestnmb_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestnmb_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestnmd_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestnmd_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestnmq_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestnmq_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestnmw_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vptestnmw_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpxord_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpxord_v256(v256, v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpxorq_v128(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512vl::vpxorq_v256(v256, v256) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vscatterdpd_v128(
                storage.as_mut_ptr(),
                v128,
                v128,
                Mask2::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vscatterdpd_v256(
                storage.as_mut_ptr(),
                v128,
                v256,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vscatterdps_v128(
                storage.as_mut_ptr(),
                v128,
                v128,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vscatterdps_v256(
                storage.as_mut_ptr(),
                v256,
                v256,
                Mask8::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vscatterqpd_v128(
                storage.as_mut_ptr(),
                v128,
                v128,
                Mask2::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vscatterqpd_v256(
                storage.as_mut_ptr(),
                v256,
                v256,
                Mask4::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vscatterqps_v128(
                storage.as_mut_ptr(),
                v128,
                v128,
                Mask2::from_bits(0),
            )
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vl::vscatterqps_v256(
                storage.as_mut_ptr(),
                v256,
                v128,
                Mask4::from_bits(0),
            )
        });
    }
}

#[test]
fn avx512dq_smoke() {
    if !(f_enabled()
        && is_x86_feature_detected!("avx512dq")
        && is_x86_feature_detected!("avx512vl"))
    {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvtpd2uqq(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvtps2qq(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvtps2uqq(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvtqq2ps(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvttpd2qq(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvttpd2uqq(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvttps2qq(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvttps2uqq(v256) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvtuqq2pd(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vcvtuqq2ps(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512dq::vfixupimmps::<0>(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512dq::vfixupimmsd::<0>(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512dq::vfixupimmss::<0>(v128, v128, v128)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vfpclasssd::<0>(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vfpclassss::<0>(v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vgetexpsd(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vgetexpss(v128, v128) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512dq::vgetmantsd::<0>(v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512dq::vgetmantss::<0>(v128, v128)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vorpd(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vpmovm2q(Mask8::NONE) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vpmovq2m(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrangepd::<0>(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrangeps::<0>(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrangesd::<0>(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrangess::<0>(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrcp14pd(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrcp14sd(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrcp14ss(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vreduceps::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vreducesd::<0>(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vreducess::<0>(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrsqrt14pd(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrsqrt14ps(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrsqrt14sd(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vrsqrt14ss(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vscalefsd(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512dq::vscalefss(v128, v128) });
    }
}

#[test]
fn avx512fp16_smoke() {
    if !(is_x86_feature_detected!("avx512fp16")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vcomish(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vcvtpd2ph(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vcvtsd2sh(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vcvtsh2sd(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vcvtsh2ss(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vcvtss2sh(v128, v128) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vdivph(v512, v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmadd132ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmadd213ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmadd231ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmaddsub132ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmaddsub213ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmaddsub231ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmsub132ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmsub213ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmsub231ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmsubadd132ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmsubadd213ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfmsubadd231ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfnmadd132ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfnmadd213ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfnmadd231ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfnmsub132ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfnmsub213ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512fp16::vfnmsub231ph(v512, v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vfpclassph::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vgetexpph(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vgetmantph::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vminph(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vmulph(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vrcpph(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vrndscaleph::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vrsqrtph(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vscalefph(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vsubph(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512fp16::vucomish(v128, v128) });
    }
}

#[test]
fn avx512bw_smoke() {
    if !(is_x86_feature_detected!("avx512bw")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpavgw(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpcmpub::<0>(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpcmpuw::<0>(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpcmpw::<0>(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovdb(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovdw(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovm2w(Mask32::NONE) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovqb(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovqw(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovsdb(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovsdw(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovsqb(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovsqw(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovusdb(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovusdw(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovusqb(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovusqw(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovuswb(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpmovw2m(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpsrldq::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vpsrlw(v512, 0) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vptestmw(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vptestnmb(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bw::vptestnmw(v512, v512) });
    }
}

#[test]
fn avx512f_smoke() {
    if !(f_enabled()) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vblendmpd(v512, v512, Mask8::NONE)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vcompresspd(v512, Mask8::NONE)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vcvtpd2dq(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vcvttpd2dq(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vexpandpd(v512, Mask8::NONE)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vextractf64x4::<0>(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vgatherdpd(v512, storage.as_ptr(), v256, Mask8::NONE)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vgatherqpd(v512, storage.as_ptr(), v512, Mask8::NONE)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vgatherqps(v256, storage.as_ptr(), v512, Mask8::NONE)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vinsertf64x4::<0>(v512, v256)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vinsertps::<0>(v128, v128) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vmovapd(v512, v512, Mask8::NONE)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vpandnq(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vpcmpuq::<0>(v512, v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vpternlogq::<0>(v512, v512, v512)
        });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vptestmq(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vptestnmq(v512, v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vrndscalepd::<0>(v512) });
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512f::vrsqrtps(v512) });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vscatterdpd(storage.as_mut_ptr(), v256, v512, Mask8::NONE)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vscatterqpd(storage.as_mut_ptr(), v512, v512, Mask8::NONE)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512f::vscatterqps(storage.as_mut_ptr(), v512, v256, Mask8::NONE)
        });
    }
}

#[test]
fn avx512vbmi2_smoke() {
    if !(is_x86_feature_detected!("avx512vbmi2")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vbmi2::vpcompressw(v512, Mask32::NONE)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vbmi2::vpexpandw(v512, Mask32::NONE)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vbmi2::vpshldd::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vbmi2::vpshldq::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vbmi2::vpshldw::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vbmi2::vpshrdd::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vbmi2::vpshrdq::<0>(v512, v512)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vbmi2::vpshrdw::<0>(v512, v512)
        });
    }
}

#[test]
fn avx512bitalg_smoke() {
    if !(is_x86_feature_detected!("avx512bitalg")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512bitalg::vpopcntw(v512) });
    }
}

#[test]
fn avx512cd_smoke() {
    if !(is_x86_feature_detected!("avx512cd")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe { rust_isa::x86::simd::avx512cd::vplzcntq(v512) });
    }
}

#[test]
fn avx512vp2intersect_smoke() {
    if !(is_x86_feature_detected!("avx512vp2intersect")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avx512vp2intersect::vp2intersectq(v512, v512)
        });
    }
}

#[test]
fn avxvnniint8_smoke() {
    if !(is_x86_feature_detected!("avxvnniint8")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint8::vpdpbssds(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint8::vpdpbssds_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint8::vpdpbsud(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint8::vpdpbsud_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint8::vpdpbsuds(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint8::vpdpbsuds_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint8::vpdpbuud_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint8::vpdpbuuds(v256, v256, v256)
        });
    }
}

#[test]
fn avxvnniint16_smoke() {
    if !(is_x86_feature_detected!("avxvnniint16")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwsud(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwsud_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwsuds(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwsuds_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwusd(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwusd_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwusds(v256, v256, v256)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwuud_v128(v128, v128, v128)
        });
        core::hint::black_box(unsafe {
            rust_isa::x86::simd::avxvnniint16::vpdpwuuds_v128(v128, v128, v128)
        });
    }
}

#[test]
fn avxneconvert_smoke() {
    if !(is_x86_feature_detected!("avxneconvert")) {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut storage = Aligned([0u8; 4096]);
    let v128 = V128::from_bytes([1; 16]);
    let v256 = V256::from_bytes([1; 32]);
    let v512 = V512::from_bytes([1; 64]);
    unsafe {
        core::hint::black_box(unsafe { rust_isa::x86::simd::avxneconvert::vbcstnesh2ps(0) });
    }
}
