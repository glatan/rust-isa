//! NEON shift instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_shl_i8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shl_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shl_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shr_i8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shr_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_shr_i32(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sar_i8(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sar_i16(a: *const u8, count: i32, out: *mut u8);
    fn rust_isa_neon_sar_i32(a: *const u8, count: i32, out: *mut u8);
}

macro_rules! shift {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, count: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), count as i32, out.as_mut_ptr());
            }
            out
        }
    };
}

shift!(
    /// Shift 8-bit lanes left by `count` bits. `count` must be less than 8.
    shl_i8,
    rust_isa_neon_shl_i8
);
shift!(
    /// Shift 16-bit lanes left by `count` bits. `count` must be less than 16.
    shl_i16,
    rust_isa_neon_shl_i16
);
shift!(
    /// Shift 32-bit lanes left by `count` bits. `count` must be less than 32.
    shl_i32,
    rust_isa_neon_shl_i32
);
shift!(
    /// Shift 8-bit lanes right by `count` bits, filling with zero.
    shr_i8,
    rust_isa_neon_shr_i8
);
shift!(
    /// Shift 16-bit lanes right by `count` bits, filling with zero.
    shr_i16,
    rust_isa_neon_shr_i16
);
shift!(
    /// Shift 32-bit lanes right by `count` bits, filling with zero.
    shr_i32,
    rust_isa_neon_shr_i32
);
shift!(
    /// Shift signed 8-bit lanes right by `count` bits, filling with the sign.
    sar_i8,
    rust_isa_neon_sar_i8
);
shift!(
    /// Shift signed 16-bit lanes right by `count` bits, filling with the sign.
    sar_i16,
    rust_isa_neon_sar_i16
);
shift!(
    /// Shift signed 32-bit lanes right by `count` bits, filling with the sign.
    sar_i32,
    rust_isa_neon_sar_i32
);
