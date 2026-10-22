//! NEON absolute difference instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_sabd_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabd_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabd_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabd_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabd_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabd_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabdl_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabdl_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sabdl_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabdl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabdl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uabdl_u32(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

binary!(
    /// `sabd` on 8-bit signed lanes: absolute difference.
    sabd_i8,
    rust_isa_neon_sabd_i8
);

binary!(
    /// `sabd` on 16-bit signed lanes: absolute difference.
    sabd_i16,
    rust_isa_neon_sabd_i16
);

binary!(
    /// `sabd` on 32-bit signed lanes: absolute difference.
    sabd_i32,
    rust_isa_neon_sabd_i32
);

binary!(
    /// `uabd` on 8-bit unsigned lanes: absolute difference.
    uabd_u8,
    rust_isa_neon_uabd_u8
);

binary!(
    /// `uabd` on 16-bit unsigned lanes: absolute difference.
    uabd_u16,
    rust_isa_neon_uabd_u16
);

binary!(
    /// `uabd` on 32-bit unsigned lanes: absolute difference.
    uabd_u32,
    rust_isa_neon_uabd_u32
);

binary!(
    /// `sabdl` on 8-bit signed lanes: absolute difference widened to the next size.
    sabdl_i8,
    rust_isa_neon_sabdl_i8
);

binary!(
    /// `sabdl` on 16-bit signed lanes: absolute difference widened to the next size.
    sabdl_i16,
    rust_isa_neon_sabdl_i16
);

binary!(
    /// `sabdl` on 32-bit signed lanes: absolute difference widened to the next size.
    sabdl_i32,
    rust_isa_neon_sabdl_i32
);

binary!(
    /// `uabdl` on 8-bit unsigned lanes: absolute difference widened to the next size.
    uabdl_u8,
    rust_isa_neon_uabdl_u8
);

binary!(
    /// `uabdl` on 16-bit unsigned lanes: absolute difference widened to the next size.
    uabdl_u16,
    rust_isa_neon_uabdl_u16
);

binary!(
    /// `uabdl` on 32-bit unsigned lanes: absolute difference widened to the next size.
    uabdl_u32,
    rust_isa_neon_uabdl_u32
);
