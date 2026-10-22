//! NEON scalar instructions.

unsafe extern "C" {
    fn rust_isa_neon_cnt_u64(a: u64) -> u64;
    fn rust_isa_neon_clz_u64(a: u64) -> u64;
    fn rust_isa_neon_cls_u64(a: u64) -> u64;
}

/// `cnt` on a 64-bit scalar: count of set bits.
#[inline(always)]
pub fn cnt_u64(a: u64) -> u64 {
    unsafe { rust_isa_neon_cnt_u64(a) }
}

/// `clz` on a 64-bit scalar: count of leading zero bits.
#[inline(always)]
pub fn clz_u64(a: u64) -> u64 {
    unsafe { rust_isa_neon_clz_u64(a) }
}

/// `cls` on a 64-bit scalar: count of leading sign bits minus one.
#[inline(always)]
pub fn cls_u64(a: u64) -> u64 {
    unsafe { rust_isa_neon_cls_u64(a) }
}
