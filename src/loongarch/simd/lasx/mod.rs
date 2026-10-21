//! LASX.
//!
//! LASX is not part of the LoongArch baseline. Every operation is `unsafe` and
//! the caller must enable LASX with `#[target_feature(enable = "lasx")]` or a
//! global target feature.
//!
//! The values are [`V256`]. The C implementation uses GCC vector extensions and
//! ordinary loads and stores so that LTO can keep values in registers. The
//! byte-shuffle instruction is intentionally omitted because intrinsics leave a
//! store/load round trip.

mod v256;

pub use v256::*;

unsafe extern "C" {
    fn rust_isa_lasx_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_mul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_sub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[target_feature(enable = "lasx")]
        #[inline]
        pub unsafe fn $name(a: V256, b: V256) -> V256 {
            let mut out = V256::from_bytes([0; 32]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `xvadd.w` on 32-bit lanes.
    add,
    rust_isa_lasx_add
);
binary!(
    /// `xvmul.w` on 32-bit lanes, keeping the low 32 bits.
    mul,
    rust_isa_lasx_mul
);
binary!(
    /// `xvsub.w` on 32-bit lanes.
    sub,
    rust_isa_lasx_sub
);
binary!(
    /// `xvadd.d` on 64-bit lanes.
    add_i64,
    rust_isa_lasx_add_i64
);
binary!(
    /// `xvsub.d` on 64-bit lanes.
    sub_i64,
    rust_isa_lasx_sub_i64
);
binary!(
    /// `xvmul.w` on 32-bit lanes.
    mul_i32,
    rust_isa_lasx_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_lasx_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_lasx_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_lasx_xor
);
binary!(
    /// `xvadd.s` on 32-bit float lanes.
    fadd,
    rust_isa_lasx_fadd
);
binary!(
    /// `xvsub.s` on 32-bit float lanes.
    fsub,
    rust_isa_lasx_fsub
);
binary!(
    /// `xvmul.s` on 32-bit float lanes.
    fmul,
    rust_isa_lasx_fmul
);
binary!(
    /// `xvdiv.s` on 32-bit float lanes.
    fdiv,
    rust_isa_lasx_fdiv
);

/// Broadcast a 32-bit value to all eight lanes.
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn splat_i32(value: i32) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_splat_i32(value, out.as_mut_ptr());
    }
    out
}

/// `xvadd.w`.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvadd(a: V256, b: V256) -> V256 {
    unsafe { add(a, b) }
}
/// `xvsub.w`.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsub(a: V256, b: V256) -> V256 {
    unsafe { sub(a, b) }
}
/// `xvmul.w`.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmul(a: V256, b: V256) -> V256 {
    unsafe { mul(a, b) }
}
/// `xvand.v`.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvand(a: V256, b: V256) -> V256 {
    unsafe { and(a, b) }
}
/// `xvor.v`.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvor(a: V256, b: V256) -> V256 {
    unsafe { or(a, b) }
}
/// `xvxor.v`.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvxor(a: V256, b: V256) -> V256 {
    unsafe { xor(a, b) }
}
