//! NEON halving instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_shadd_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shsub_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srhadd_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shadd_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shsub_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srhadd_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_shsub_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_srhadd_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhadd_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhsub_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urhadd_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhadd_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhsub_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urhadd_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhadd_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uhsub_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_urhadd_u32(a: *const u8, b: *const u8, out: *mut u8);
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
    /// `shadd` on 8-bit signed lanes: halving add.
    shadd_s8,
    rust_isa_neon_shadd_s8
);

binary!(
    /// `shsub` on 8-bit signed lanes: halving subtract.
    shsub_s8,
    rust_isa_neon_shsub_s8
);

binary!(
    /// `srhadd` on 8-bit signed lanes: rounding halving add.
    srhadd_s8,
    rust_isa_neon_srhadd_s8
);

binary!(
    /// `shadd` on 16-bit signed lanes: halving add.
    shadd_s16,
    rust_isa_neon_shadd_s16
);

binary!(
    /// `shsub` on 16-bit signed lanes: halving subtract.
    shsub_s16,
    rust_isa_neon_shsub_s16
);

binary!(
    /// `srhadd` on 16-bit signed lanes: rounding halving add.
    srhadd_s16,
    rust_isa_neon_srhadd_s16
);

binary!(
    /// `shadd` on 32-bit signed lanes: halving add.
    shadd_s32,
    rust_isa_neon_shadd_s32
);

binary!(
    /// `shsub` on 32-bit signed lanes: halving subtract.
    shsub_s32,
    rust_isa_neon_shsub_s32
);

binary!(
    /// `srhadd` on 32-bit signed lanes: rounding halving add.
    srhadd_s32,
    rust_isa_neon_srhadd_s32
);

binary!(
    /// `uhadd` on 8-bit unsigned lanes: halving add.
    uhadd_u8,
    rust_isa_neon_uhadd_u8
);

binary!(
    /// `uhsub` on 8-bit unsigned lanes: halving subtract.
    uhsub_u8,
    rust_isa_neon_uhsub_u8
);

binary!(
    /// `urhadd` on 8-bit unsigned lanes: rounding halving add.
    urhadd_u8,
    rust_isa_neon_urhadd_u8
);

binary!(
    /// `uhadd` on 16-bit unsigned lanes: halving add.
    uhadd_u16,
    rust_isa_neon_uhadd_u16
);

binary!(
    /// `uhsub` on 16-bit unsigned lanes: halving subtract.
    uhsub_u16,
    rust_isa_neon_uhsub_u16
);

binary!(
    /// `urhadd` on 16-bit unsigned lanes: rounding halving add.
    urhadd_u16,
    rust_isa_neon_urhadd_u16
);

binary!(
    /// `uhadd` on 32-bit unsigned lanes: halving add.
    uhadd_u32,
    rust_isa_neon_uhadd_u32
);

binary!(
    /// `uhsub` on 32-bit unsigned lanes: halving subtract.
    uhsub_u32,
    rust_isa_neon_uhsub_u32
);

binary!(
    /// `urhadd` on 32-bit unsigned lanes: rounding halving add.
    urhadd_u32,
    rust_isa_neon_urhadd_u32
);
