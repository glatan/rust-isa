//! PowerPC Power8 vector instructions.
//!
//! Power8 vector instructions are not part of the PowerPC baseline. Every
//! operation is `unsafe` and the caller must enable the Power8 feature set. The
//! C file is compiled with `-mcpu=power8`.
//!
//! The values are [`V128`]. The module contains the 64-bit integer add and
//! subtract (`vaddudm`, `vsubudm`), 32-bit multiply (`vmuluwm`) and the even
//! halfword widening multiply.

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_power8_vector_add_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_sub_u64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_mul_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_mul_even_u16(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `vaddudm`.
    add_u64,
    rust_isa_power8_vector_add_u64
);
binary!(
    /// `vsubudm`.
    sub_u64,
    rust_isa_power8_vector_sub_u64
);
binary!(
    /// `vmuluwm`.
    mul_u32,
    rust_isa_power8_vector_mul_u32
);
binary!(
    /// `vmuleuh`.
    mul_even_u16,
    rust_isa_power8_vector_mul_even_u16
);

unsafe extern "C" {
    fn rust_isa_power8_vector_vmuleuw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vmulouw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vmaxud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vminud(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vmaxsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vminsd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vpkudum(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vpksdus(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vpksdss(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vupkhsw(a: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vupklsw(a: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vclzd(a: *const u8, out: *mut u8);
    fn rust_isa_power8_vector_vpopcntd(a: *const u8, out: *mut u8);
}

macro_rules! bin8 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
macro_rules! un8 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}
bin8!(
    /// `vmuleuw`.
    vmuleuw,
    rust_isa_power8_vector_vmuleuw
);
bin8!(
    /// `vmulouw`.
    vmulouw,
    rust_isa_power8_vector_vmulouw
);
bin8!(
    /// `vmaxud`.
    vmaxud,
    rust_isa_power8_vector_vmaxud
);
bin8!(
    /// `vminud`.
    vminud,
    rust_isa_power8_vector_vminud
);
bin8!(
    /// `vmaxsd`.
    vmaxsd,
    rust_isa_power8_vector_vmaxsd
);
bin8!(
    /// `vminsd`.
    vminsd,
    rust_isa_power8_vector_vminsd
);
bin8!(
    /// `vpkudum`.
    vpkudum,
    rust_isa_power8_vector_vpkudum
);
bin8!(
    /// `vpksdus`.
    vpksdus,
    rust_isa_power8_vector_vpksdus
);
bin8!(
    /// `vpksdss`.
    vpksdss,
    rust_isa_power8_vector_vpksdss
);
un8!(
    /// `vupkhsw`.
    vupkhsw,
    rust_isa_power8_vector_vupkhsw
);
un8!(
    /// `vupklsw`.
    vupklsw,
    rust_isa_power8_vector_vupklsw
);
un8!(
    /// `vclzd`.
    vclzd,
    rust_isa_power8_vector_vclzd
);
un8!(
    /// `vpopcntd`.
    vpopcntd,
    rust_isa_power8_vector_vpopcntd
);
