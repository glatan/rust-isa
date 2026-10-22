//! NEON narrowing instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_xtn_s8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_s16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_s32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_u8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_u16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn_u32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_xtn2_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn2_i16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn2_i32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_addhn2_i64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn2_i16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn2_i32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_raddhn2_i64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn2_i16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn2_i32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_subhn2_i64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn2_i16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn2_i32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_rsubhn2_i64(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

macro_rules! narrow2 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, low: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), low.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
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

macro_rules! addhn2 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128, low: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), low.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

unary!(
    /// `xtn` from 16-bit signed lanes to 8-bit signed lanes.
    xtn_s8,
    rust_isa_neon_xtn_s8
);

narrow2!(
    /// `xtn2` from 16-bit signed lanes into the upper half of the result.
    xtn2_s8,
    rust_isa_neon_xtn2_s8
);

unary!(
    /// `xtn` from 32-bit signed lanes to 16-bit signed lanes.
    xtn_s16,
    rust_isa_neon_xtn_s16
);

narrow2!(
    /// `xtn2` from 32-bit signed lanes into the upper half of the result.
    xtn2_s16,
    rust_isa_neon_xtn2_s16
);

unary!(
    /// `xtn` from 64-bit signed lanes to 32-bit signed lanes.
    xtn_s32,
    rust_isa_neon_xtn_s32
);

narrow2!(
    /// `xtn2` from 64-bit signed lanes into the upper half of the result.
    xtn2_s32,
    rust_isa_neon_xtn2_s32
);

unary!(
    /// `xtn` from 16-bit unsigned lanes to 8-bit unsigned lanes.
    xtn_u8,
    rust_isa_neon_xtn_u8
);

narrow2!(
    /// `xtn2` from 16-bit unsigned lanes into the upper half of the result.
    xtn2_u8,
    rust_isa_neon_xtn2_u8
);

unary!(
    /// `xtn` from 32-bit unsigned lanes to 16-bit unsigned lanes.
    xtn_u16,
    rust_isa_neon_xtn_u16
);

narrow2!(
    /// `xtn2` from 32-bit unsigned lanes into the upper half of the result.
    xtn2_u16,
    rust_isa_neon_xtn2_u16
);

unary!(
    /// `xtn` from 64-bit unsigned lanes to 32-bit unsigned lanes.
    xtn_u32,
    rust_isa_neon_xtn_u32
);

narrow2!(
    /// `xtn2` from 64-bit unsigned lanes into the upper half of the result.
    xtn2_u32,
    rust_isa_neon_xtn2_u32
);

binary!(
    /// `addhn` from 16-bit lanes, keeping the high half of each result.
    addhn_i16,
    rust_isa_neon_addhn_i16
);

addhn2!(
    /// `addhn2` from 16-bit lanes into the upper half of the result.
    addhn2_i16,
    rust_isa_neon_addhn2_i16
);

binary!(
    /// `addhn` from 32-bit lanes, keeping the high half of each result.
    addhn_i32,
    rust_isa_neon_addhn_i32
);

addhn2!(
    /// `addhn2` from 32-bit lanes into the upper half of the result.
    addhn2_i32,
    rust_isa_neon_addhn2_i32
);

binary!(
    /// `addhn` from 64-bit lanes, keeping the high half of each result.
    addhn_i64,
    rust_isa_neon_addhn_i64
);

addhn2!(
    /// `addhn2` from 64-bit lanes into the upper half of the result.
    addhn2_i64,
    rust_isa_neon_addhn2_i64
);

binary!(
    /// `raddhn` from 16-bit lanes, keeping the high half of each result.
    raddhn_i16,
    rust_isa_neon_raddhn_i16
);

addhn2!(
    /// `raddhn2` from 16-bit lanes into the upper half of the result.
    raddhn2_i16,
    rust_isa_neon_raddhn2_i16
);

binary!(
    /// `raddhn` from 32-bit lanes, keeping the high half of each result.
    raddhn_i32,
    rust_isa_neon_raddhn_i32
);

addhn2!(
    /// `raddhn2` from 32-bit lanes into the upper half of the result.
    raddhn2_i32,
    rust_isa_neon_raddhn2_i32
);

binary!(
    /// `raddhn` from 64-bit lanes, keeping the high half of each result.
    raddhn_i64,
    rust_isa_neon_raddhn_i64
);

addhn2!(
    /// `raddhn2` from 64-bit lanes into the upper half of the result.
    raddhn2_i64,
    rust_isa_neon_raddhn2_i64
);

binary!(
    /// `subhn` from 16-bit lanes, keeping the high half of each result.
    subhn_i16,
    rust_isa_neon_subhn_i16
);

addhn2!(
    /// `subhn2` from 16-bit lanes into the upper half of the result.
    subhn2_i16,
    rust_isa_neon_subhn2_i16
);

binary!(
    /// `subhn` from 32-bit lanes, keeping the high half of each result.
    subhn_i32,
    rust_isa_neon_subhn_i32
);

addhn2!(
    /// `subhn2` from 32-bit lanes into the upper half of the result.
    subhn2_i32,
    rust_isa_neon_subhn2_i32
);

binary!(
    /// `subhn` from 64-bit lanes, keeping the high half of each result.
    subhn_i64,
    rust_isa_neon_subhn_i64
);

addhn2!(
    /// `subhn2` from 64-bit lanes into the upper half of the result.
    subhn2_i64,
    rust_isa_neon_subhn2_i64
);

binary!(
    /// `rsubhn` from 16-bit lanes, keeping the high half of each result.
    rsubhn_i16,
    rust_isa_neon_rsubhn_i16
);

addhn2!(
    /// `rsubhn2` from 16-bit lanes into the upper half of the result.
    rsubhn2_i16,
    rust_isa_neon_rsubhn2_i16
);

binary!(
    /// `rsubhn` from 32-bit lanes, keeping the high half of each result.
    rsubhn_i32,
    rust_isa_neon_rsubhn_i32
);

addhn2!(
    /// `rsubhn2` from 32-bit lanes into the upper half of the result.
    rsubhn2_i32,
    rust_isa_neon_rsubhn2_i32
);

binary!(
    /// `rsubhn` from 64-bit lanes, keeping the high half of each result.
    rsubhn_i64,
    rust_isa_neon_rsubhn_i64
);

addhn2!(
    /// `rsubhn2` from 64-bit lanes into the upper half of the result.
    rsubhn2_i64,
    rust_isa_neon_rsubhn2_i64
);
