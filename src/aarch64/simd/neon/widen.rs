//! NEON widening arithmetic instructions.

use super::V128;
unsafe extern "C" {
    fn rust_isa_neon_saddl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubl_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubl_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubl_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubl_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubl_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubl_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddw_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddw_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_saddw_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddw_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddw_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_uaddw_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubw_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubw_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_ssubw_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubw_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubw_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_usubw_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smull_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smull2_s8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal2_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl2_s8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smull_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smull2_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal2_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl2_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smull_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smull2_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlal2_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_smlsl2_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umull_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umull2_u8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal2_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl2_u8(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umull_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umull2_u16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal_u16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal2_u16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl_u16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl2_u16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umull_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umull2_u32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal_u32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlal2_u32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl_u32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_umlsl2_u32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmull_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmull2_s16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlal_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlal2_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlsl_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlsl2_s16(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmull_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmull2_s32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlal_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlal2_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlsl_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_neon_sqdmlsl2_s32(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}

/// `saddl_s8`
///
/// Add the low half of signed lanes and widen the result.
#[inline(always)]
pub fn saddl_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddl_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddl_s16`
///
/// Add the low half of signed lanes and widen the result.
#[inline(always)]
pub fn saddl_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddl_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddl_s32`
///
/// Add the low half of signed lanes and widen the result.
#[inline(always)]
pub fn saddl_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddl_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddl_u8`
///
/// Add the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn uaddl_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddl_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddl_u16`
///
/// Add the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn uaddl_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddl_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddl_u32`
///
/// Add the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn uaddl_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddl_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubl_s8`
///
/// Subtract the low half of signed lanes and widen the result.
#[inline(always)]
pub fn ssubl_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubl_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubl_s16`
///
/// Subtract the low half of signed lanes and widen the result.
#[inline(always)]
pub fn ssubl_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubl_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubl_s32`
///
/// Subtract the low half of signed lanes and widen the result.
#[inline(always)]
pub fn ssubl_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubl_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubl_u8`
///
/// Subtract the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn usubl_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubl_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubl_u16`
///
/// Subtract the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn usubl_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubl_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubl_u32`
///
/// Subtract the low half of unsigned lanes and widen the result.
#[inline(always)]
pub fn usubl_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubl_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddw_s8`
///
/// Add the low half of signed lanes to widened signed lanes.
#[inline(always)]
pub fn saddw_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddw_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddw_s16`
///
/// Add the low half of signed lanes to widened signed lanes.
#[inline(always)]
pub fn saddw_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddw_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `saddw_s32`
///
/// Add the low half of signed lanes to widened signed lanes.
#[inline(always)]
pub fn saddw_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_saddw_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddw_u8`
///
/// Add the low half of unsigned lanes to widened unsigned lanes.
#[inline(always)]
pub fn uaddw_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddw_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddw_u16`
///
/// Add the low half of unsigned lanes to widened unsigned lanes.
#[inline(always)]
pub fn uaddw_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddw_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `uaddw_u32`
///
/// Add the low half of unsigned lanes to widened unsigned lanes.
#[inline(always)]
pub fn uaddw_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_uaddw_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubw_s8`
///
/// Subtract the low half of signed lanes from widened signed lanes.
#[inline(always)]
pub fn ssubw_s8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubw_s8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubw_s16`
///
/// Subtract the low half of signed lanes from widened signed lanes.
#[inline(always)]
pub fn ssubw_s16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubw_s16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `ssubw_s32`
///
/// Subtract the low half of signed lanes from widened signed lanes.
#[inline(always)]
pub fn ssubw_s32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ssubw_s32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubw_u8`
///
/// Subtract the low half of unsigned lanes from widened unsigned lanes.
#[inline(always)]
pub fn usubw_u8(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubw_u8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubw_u16`
///
/// Subtract the low half of unsigned lanes from widened unsigned lanes.
#[inline(always)]
pub fn usubw_u16(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubw_u16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `usubw_u32`
///
/// Subtract the low half of unsigned lanes from widened unsigned lanes.
#[inline(always)]
pub fn usubw_u32(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_usubw_u32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
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

macro_rules! acc {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe { $c(a.as_ptr(), b.as_ptr(), acc.as_ptr(), out.as_mut_ptr()); }
            out
        }
    };
}

binary!(
    /// `smull` from 8-bit signed lanes to 16-bit signed lanes.
    smull_s8,
    rust_isa_neon_smull_s8
);

binary!(
    /// `smull2` from the high 8-bit signed lanes to 16-bit signed lanes.
    smull2_s8,
    rust_isa_neon_smull2_s8
);

acc!(
    /// `smlal` from 8-bit signed lanes into a 16-bit accumulator.
    smlal_s8,
    rust_isa_neon_smlal_s8
);

acc!(
    /// `smlal2` from the high 8-bit signed lanes into a 16-bit accumulator.
    smlal2_s8,
    rust_isa_neon_smlal2_s8
);

acc!(
    /// `smlsl` from 8-bit signed lanes into a 16-bit accumulator.
    smlsl_s8,
    rust_isa_neon_smlsl_s8
);

acc!(
    /// `smlsl2` from the high 8-bit signed lanes into a 16-bit accumulator.
    smlsl2_s8,
    rust_isa_neon_smlsl2_s8
);

binary!(
    /// `smull` from 16-bit signed lanes to 32-bit signed lanes.
    smull_s16,
    rust_isa_neon_smull_s16
);

binary!(
    /// `smull2` from the high 16-bit signed lanes to 32-bit signed lanes.
    smull2_s16,
    rust_isa_neon_smull2_s16
);

acc!(
    /// `smlal` from 16-bit signed lanes into a 32-bit accumulator.
    smlal_s16,
    rust_isa_neon_smlal_s16
);

acc!(
    /// `smlal2` from the high 16-bit signed lanes into a 32-bit accumulator.
    smlal2_s16,
    rust_isa_neon_smlal2_s16
);

acc!(
    /// `smlsl` from 16-bit signed lanes into a 32-bit accumulator.
    smlsl_s16,
    rust_isa_neon_smlsl_s16
);

acc!(
    /// `smlsl2` from the high 16-bit signed lanes into a 32-bit accumulator.
    smlsl2_s16,
    rust_isa_neon_smlsl2_s16
);

binary!(
    /// `smull` from 32-bit signed lanes to 64-bit signed lanes.
    smull_s32,
    rust_isa_neon_smull_s32
);

binary!(
    /// `smull2` from the high 32-bit signed lanes to 64-bit signed lanes.
    smull2_s32,
    rust_isa_neon_smull2_s32
);

acc!(
    /// `smlal` from 32-bit signed lanes into a 64-bit accumulator.
    smlal_s32,
    rust_isa_neon_smlal_s32
);

acc!(
    /// `smlal2` from the high 32-bit signed lanes into a 64-bit accumulator.
    smlal2_s32,
    rust_isa_neon_smlal2_s32
);

acc!(
    /// `smlsl` from 32-bit signed lanes into a 64-bit accumulator.
    smlsl_s32,
    rust_isa_neon_smlsl_s32
);

acc!(
    /// `smlsl2` from the high 32-bit signed lanes into a 64-bit accumulator.
    smlsl2_s32,
    rust_isa_neon_smlsl2_s32
);

binary!(
    /// `umull` from 8-bit unsigned lanes to 16-bit unsigned lanes.
    umull_u8,
    rust_isa_neon_umull_u8
);

binary!(
    /// `umull2` from the high 8-bit unsigned lanes to 16-bit unsigned lanes.
    umull2_u8,
    rust_isa_neon_umull2_u8
);

acc!(
    /// `umlal` from 8-bit unsigned lanes into a 16-bit accumulator.
    umlal_u8,
    rust_isa_neon_umlal_u8
);

acc!(
    /// `umlal2` from the high 8-bit unsigned lanes into a 16-bit accumulator.
    umlal2_u8,
    rust_isa_neon_umlal2_u8
);

acc!(
    /// `umlsl` from 8-bit unsigned lanes into a 16-bit accumulator.
    umlsl_u8,
    rust_isa_neon_umlsl_u8
);

acc!(
    /// `umlsl2` from the high 8-bit unsigned lanes into a 16-bit accumulator.
    umlsl2_u8,
    rust_isa_neon_umlsl2_u8
);

binary!(
    /// `umull` from 16-bit unsigned lanes to 32-bit unsigned lanes.
    umull_u16,
    rust_isa_neon_umull_u16
);

binary!(
    /// `umull2` from the high 16-bit unsigned lanes to 32-bit unsigned lanes.
    umull2_u16,
    rust_isa_neon_umull2_u16
);

acc!(
    /// `umlal` from 16-bit unsigned lanes into a 32-bit accumulator.
    umlal_u16,
    rust_isa_neon_umlal_u16
);

acc!(
    /// `umlal2` from the high 16-bit unsigned lanes into a 32-bit accumulator.
    umlal2_u16,
    rust_isa_neon_umlal2_u16
);

acc!(
    /// `umlsl` from 16-bit unsigned lanes into a 32-bit accumulator.
    umlsl_u16,
    rust_isa_neon_umlsl_u16
);

acc!(
    /// `umlsl2` from the high 16-bit unsigned lanes into a 32-bit accumulator.
    umlsl2_u16,
    rust_isa_neon_umlsl2_u16
);

binary!(
    /// `umull` from 32-bit unsigned lanes to 64-bit unsigned lanes.
    umull_u32,
    rust_isa_neon_umull_u32
);

binary!(
    /// `umull2` from the high 32-bit unsigned lanes to 64-bit unsigned lanes.
    umull2_u32,
    rust_isa_neon_umull2_u32
);

acc!(
    /// `umlal` from 32-bit unsigned lanes into a 64-bit accumulator.
    umlal_u32,
    rust_isa_neon_umlal_u32
);

acc!(
    /// `umlal2` from the high 32-bit unsigned lanes into a 64-bit accumulator.
    umlal2_u32,
    rust_isa_neon_umlal2_u32
);

acc!(
    /// `umlsl` from 32-bit unsigned lanes into a 64-bit accumulator.
    umlsl_u32,
    rust_isa_neon_umlsl_u32
);

acc!(
    /// `umlsl2` from the high 32-bit unsigned lanes into a 64-bit accumulator.
    umlsl2_u32,
    rust_isa_neon_umlsl2_u32
);

binary!(
    /// `sqdmull` from 16-bit signed lanes to 32-bit signed lanes.
    sqdmull_s16,
    rust_isa_neon_sqdmull_s16
);

binary!(
    /// `sqdmull2` from the high 16-bit signed lanes to 32-bit signed lanes.
    sqdmull2_s16,
    rust_isa_neon_sqdmull2_s16
);

acc!(
    /// `sqdmlal` from 16-bit signed lanes into a 32-bit accumulator.
    sqdmlal_s16,
    rust_isa_neon_sqdmlal_s16
);

acc!(
    /// `sqdmlal2` from the high 16-bit signed lanes into a 32-bit accumulator.
    sqdmlal2_s16,
    rust_isa_neon_sqdmlal2_s16
);

acc!(
    /// `sqdmlsl` from 16-bit signed lanes into a 32-bit accumulator.
    sqdmlsl_s16,
    rust_isa_neon_sqdmlsl_s16
);

acc!(
    /// `sqdmlsl2` from the high 16-bit signed lanes into a 32-bit accumulator.
    sqdmlsl2_s16,
    rust_isa_neon_sqdmlsl2_s16
);

binary!(
    /// `sqdmull` from 32-bit signed lanes to 64-bit signed lanes.
    sqdmull_s32,
    rust_isa_neon_sqdmull_s32
);

binary!(
    /// `sqdmull2` from the high 32-bit signed lanes to 64-bit signed lanes.
    sqdmull2_s32,
    rust_isa_neon_sqdmull2_s32
);

acc!(
    /// `sqdmlal` from 32-bit signed lanes into a 64-bit accumulator.
    sqdmlal_s32,
    rust_isa_neon_sqdmlal_s32
);

acc!(
    /// `sqdmlal2` from the high 32-bit signed lanes into a 64-bit accumulator.
    sqdmlal2_s32,
    rust_isa_neon_sqdmlal2_s32
);

acc!(
    /// `sqdmlsl` from 32-bit signed lanes into a 64-bit accumulator.
    sqdmlsl_s32,
    rust_isa_neon_sqdmlsl_s32
);

acc!(
    /// `sqdmlsl2` from the high 32-bit signed lanes into a 64-bit accumulator.
    sqdmlsl2_s32,
    rust_isa_neon_sqdmlsl2_s32
);
