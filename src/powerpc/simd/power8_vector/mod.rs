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
