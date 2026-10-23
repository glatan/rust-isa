//! Shift instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_psllw(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_sse2_pslld(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_sse2_psllq(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_sse2_psrlw(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_sse2_psrld(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_sse2_psrlq(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_sse2_psraw(a: *const u8, count: u32, out: *mut u8);
    fn rust_isa_sse2_psrad(a: *const u8, count: u32, out: *mut u8);
}

/// `psllw`
///
/// Shift the eight 16-bit integer lanes left by `count`.
#[inline(always)]
pub fn psllw(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psllw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `pslld`
///
/// Shift the four 32-bit integer lanes left by `count`.
#[inline(always)]
pub fn pslld(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pslld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `psllq`
///
/// Shift the two 64-bit integer lanes left by `count`.
#[inline(always)]
pub fn psllq(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psllq(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `psrlw`
///
/// Shift the eight 16-bit integer lanes right by `count`.
#[inline(always)]
pub fn psrlw(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psrlw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `psrld`
///
/// Shift the four 32-bit integer lanes right by `count`.
#[inline(always)]
pub fn psrld(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psrld(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `psrlq`
///
/// Shift the two 64-bit integer lanes right by `count`.
#[inline(always)]
pub fn psrlq(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psrlq(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `psraw`
///
/// Shift the eight signed 16-bit integer lanes right by `count`.
#[inline(always)]
pub fn psraw(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psraw(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

/// `psrad`
///
/// Shift the four signed 32-bit integer lanes right by `count`.
#[inline(always)]
pub fn psrad(a: V128, count: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psrad(a.as_ptr(), count, out.as_mut_ptr());
    }
    out
}

macro_rules! byte_shift {
    ($(#[$meta:meta])* $name:ident, $prefix:ident, [$($imm:literal => $f:ident),* $(,)?]) => {
        unsafe extern "C" {
            $(fn $f(a: *const u8, out: *mut u8);)*
        }

        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u8>(a: V128) -> V128 {
            const { assert!(IMM <= 15, "SSE2 byte shift must be in 0..=15") };
            let mut out = V128::from_bytes([0; 16]);
            macro_rules! call {
                ($func:ident) => {
                    unsafe { $func(a.as_ptr(), out.as_mut_ptr()) }
                };
            }
            match IMM {
                $($imm => call!($f),)*
                _ => unreachable!(),
            }
            out
        }
    };
}

byte_shift!(
    /// `pslldq`
    ///
    /// Shift the 16 bytes left by `IMM` bytes, filling with zero. `IMM` must
    /// be in `0..=15`.
    pslldq,
    pslldq,
    [
        0 => rust_isa_sse2_pslldq_0,
        1 => rust_isa_sse2_pslldq_1,
        2 => rust_isa_sse2_pslldq_2,
        3 => rust_isa_sse2_pslldq_3,
        4 => rust_isa_sse2_pslldq_4,
        5 => rust_isa_sse2_pslldq_5,
        6 => rust_isa_sse2_pslldq_6,
        7 => rust_isa_sse2_pslldq_7,
        8 => rust_isa_sse2_pslldq_8,
        9 => rust_isa_sse2_pslldq_9,
        10 => rust_isa_sse2_pslldq_10,
        11 => rust_isa_sse2_pslldq_11,
        12 => rust_isa_sse2_pslldq_12,
        13 => rust_isa_sse2_pslldq_13,
        14 => rust_isa_sse2_pslldq_14,
        15 => rust_isa_sse2_pslldq_15,
    ]
);

byte_shift!(
    /// `psrldq`
    ///
    /// Shift the 16 bytes right by `IMM` bytes, filling with zero. `IMM` must
    /// be in `0..=15`.
    psrldq,
    psrldq,
    [
        0 => rust_isa_sse2_psrldq_0,
        1 => rust_isa_sse2_psrldq_1,
        2 => rust_isa_sse2_psrldq_2,
        3 => rust_isa_sse2_psrldq_3,
        4 => rust_isa_sse2_psrldq_4,
        5 => rust_isa_sse2_psrldq_5,
        6 => rust_isa_sse2_psrldq_6,
        7 => rust_isa_sse2_psrldq_7,
        8 => rust_isa_sse2_psrldq_8,
        9 => rust_isa_sse2_psrldq_9,
        10 => rust_isa_sse2_psrldq_10,
        11 => rust_isa_sse2_psrldq_11,
        12 => rust_isa_sse2_psrldq_12,
        13 => rust_isa_sse2_psrldq_13,
        14 => rust_isa_sse2_psrldq_14,
        15 => rust_isa_sse2_psrldq_15,
    ]
);
