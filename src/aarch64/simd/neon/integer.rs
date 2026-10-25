//! NEON integer instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_add_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_mul_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_orr(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_eor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_bic(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_orn(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_not(a: *const u8, out: *mut u8);
    fn rust_isa_neon_abs_i8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_abs_i16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_abs_i32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_neg_i8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_neg_i16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_neg_i32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_bsl(mask: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_bit(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_neon_bif(a: *const u8, b: *const u8, mask: *const u8, out: *mut u8);
    fn rust_isa_neon_cls_i8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_cls_i16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_cls_i32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_clz_i8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_clz_i16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_clz_i32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_cnt_i8(a: *const u8, out: *mut u8);
    fn rust_isa_neon_rbit(a: *const u8, out: *mut u8);
    fn rust_isa_neon_rev16(a: *const u8, out: *mut u8);
    fn rust_isa_neon_rev32(a: *const u8, out: *mut u8);
    fn rust_isa_neon_rev64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_abs_i64(a: *const u8, out: *mut u8);
    fn rust_isa_neon_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_mul(a: *const u8, b: *const u8, out: *mut u8);
}

macro_rules! binary {
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

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `add` on 8-bit lanes.
    add_i8,
    rust_isa_neon_add_i8
);
binary!(
    /// `add` on 16-bit lanes.
    add_i16,
    rust_isa_neon_add_i16
);
binary!(
    /// `add` on 32-bit lanes.
    add_i32,
    rust_isa_neon_add_i32
);
binary!(
    /// `add` on 64-bit lanes.
    add_i64,
    rust_isa_neon_add_i64
);
binary!(
    /// `sub` on 8-bit lanes.
    sub_i8,
    rust_isa_neon_sub_i8
);
binary!(
    /// `sub` on 16-bit lanes.
    sub_i16,
    rust_isa_neon_sub_i16
);
binary!(
    /// `sub` on 32-bit lanes.
    sub_i32,
    rust_isa_neon_sub_i32
);
binary!(
    /// `sub` on 64-bit lanes.
    sub_i64,
    rust_isa_neon_sub_i64
);
binary!(
    /// `mul` on 8-bit lanes, keeping the low 8 bits.
    mul_i8,
    rust_isa_neon_mul_i8
);
binary!(
    /// `mul` on 16-bit lanes, keeping the low 16 bits.
    mul_i16,
    rust_isa_neon_mul_i16
);
binary!(
    /// `mul` on 32-bit lanes, keeping the low 32 bits.
    mul_i32,
    rust_isa_neon_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_neon_and
);
binary!(
    /// Bitwise OR.
    orr,
    rust_isa_neon_orr
);
binary!(
    /// Bitwise XOR.
    eor,
    rust_isa_neon_eor
);
binary!(
    /// Bitwise AND with the complement of `b`.
    bic,
    rust_isa_neon_bic
);
binary!(
    /// Bitwise OR with the complement of `b`.
    orn,
    rust_isa_neon_orn
);
unary!(
    /// Bitwise NOT.
    not,
    rust_isa_neon_not
);
unary!(
    /// Absolute value on 8-bit lanes.
    abs_i8,
    rust_isa_neon_abs_i8
);
unary!(
    /// Absolute value on 16-bit lanes.
    abs_i16,
    rust_isa_neon_abs_i16
);
unary!(
    /// Absolute value on 32-bit lanes.
    abs_i32,
    rust_isa_neon_abs_i32
);
unary!(
    /// Negate 8-bit lanes.
    neg_i8,
    rust_isa_neon_neg_i8
);
unary!(
    /// Negate 16-bit lanes.
    neg_i16,
    rust_isa_neon_neg_i16
);
unary!(
    /// Negate 32-bit lanes.
    neg_i32,
    rust_isa_neon_neg_i32
);

/// `bsl`
///
/// Select bits from `a` where the corresponding bit of `mask` is one, and
/// from `b` where it is zero.
#[inline(always)]
pub fn bsl(mask: V128, a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_bsl(mask.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `bit`
///
/// Insert bits from `b` into `a` where the corresponding bit of `mask` is one.
#[inline(always)]
pub fn bit(a: V128, b: V128, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_bit(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `bif`
///
/// Insert bits from `b` into `a` where the corresponding bit of `mask` is zero.
#[inline(always)]
pub fn bif(a: V128, b: V128, mask: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_bif(a.as_ptr(), b.as_ptr(), mask.as_ptr(), out.as_mut_ptr());
    }
    out
}

unary!(
    /// Count leading sign bits in each 8-bit lane.
    cls_i8,
    rust_isa_neon_cls_i8
);
unary!(
    /// Count leading sign bits in each 16-bit lane.
    cls_i16,
    rust_isa_neon_cls_i16
);
unary!(
    /// Count leading sign bits in each 32-bit lane.
    cls_i32,
    rust_isa_neon_cls_i32
);
unary!(
    /// Count leading zero bits in each 8-bit lane.
    clz_i8,
    rust_isa_neon_clz_i8
);
unary!(
    /// Count leading zero bits in each 16-bit lane.
    clz_i16,
    rust_isa_neon_clz_i16
);
unary!(
    /// Count leading zero bits in each 32-bit lane.
    clz_i32,
    rust_isa_neon_clz_i32
);
unary!(
    /// Count set bits in each byte.
    cnt_i8,
    rust_isa_neon_cnt_i8
);
unary!(
    /// Reverse the bits in each byte.
    rbit,
    rust_isa_neon_rbit
);
unary!(
    /// Reverse the bytes in each 16-bit lane.
    rev16,
    rust_isa_neon_rev16
);
unary!(
    /// Reverse the bytes in each 32-bit lane.
    rev32,
    rust_isa_neon_rev32
);
unary!(
    /// Reverse the bytes in each 64-bit lane.
    rev64,
    rust_isa_neon_rev64
);

unary!(
    /// `abs` on 64-bit signed lanes.
    abs_i64,
    rust_isa_neon_abs_i64
);

/// `add`
///
/// Add the four 32-bit integer lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::aarch64::simd::neon::{add, V128};
///
/// let a = V128::from_bytes([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
/// let b = V128::from_bytes([2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0]);
/// assert_eq!(
///     add(a, b).to_bytes(),
///     [3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0]
/// );
/// ```
#[inline(always)]
pub fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `mul`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
#[inline(always)]
pub fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
