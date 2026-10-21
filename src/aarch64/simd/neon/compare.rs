//! NEON comparison instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_cmpeq_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmpeq_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmpgt_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmpgt_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! compare {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

compare!(
    /// Compare 8-bit lanes for equality. A true lane is all bits one.
    cmpeq_i8,
    rust_isa_neon_cmpeq_i8
);
compare!(
    /// Compare 16-bit lanes for equality. A true lane is all bits one.
    cmpeq_i16,
    rust_isa_neon_cmpeq_i16
);
compare!(
    /// Compare 32-bit lanes for equality. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_neon_cmpeq_i32
);
compare!(
    /// Compare signed 8-bit lanes with greater-than.
    cmpgt_i8,
    rust_isa_neon_cmpgt_i8
);
compare!(
    /// Compare signed 16-bit lanes with greater-than.
    cmpgt_i16,
    rust_isa_neon_cmpgt_i16
);
compare!(
    /// Compare signed 32-bit lanes with greater-than.
    cmpgt_i32,
    rust_isa_neon_cmpgt_i32
);
