//! 16-bit integer instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_paddw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pcmpeqw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pmullw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddw`
///
/// Add the eight 16-bit integer lanes, wrapping at 16 bits.
#[inline(always)]
pub fn paddw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `psubw`
///
/// Subtract the eight 16-bit integer lanes, wrapping at 16 bits.
#[inline(always)]
pub fn psubw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pcmpeqw`
///
/// Compare the eight 16-bit integer lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpeqw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpeqw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pmullw`
///
/// Multiply the eight 16-bit integer lanes, keeping the low 16 bits.
#[inline(always)]
pub fn pmullw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pmullw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pcmpgtw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pcmpgtw`
///
/// Compare the eight signed 16-bit integer lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpgtw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpgtw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pmaddwd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pmaddwd`
///
/// Multiply and add adjacent 16-bit integer lanes into 32-bit lanes.
#[inline(always)]
pub fn pmaddwd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pmaddwd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pminw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pminw`
///
/// Take the minimum of the eight signed 16-bit integer lanes.
#[inline(always)]
pub fn pminw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pminw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pmaxw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pmaxw`
///
/// Take the maximum of the eight signed 16-bit integer lanes.
#[inline(always)]
pub fn pmaxw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pmaxw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pavgw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pavgw`
///
/// Average the eight unsigned 16-bit integer lanes, rounding up.
#[inline(always)]
pub fn pavgw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pavgw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_paddsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_paddusw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubsw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubusw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_packssdw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pmulhw(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pmulhuw(a: *const u8, b: *const u8, out: *mut u8);
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

binary!(
    /// `paddsw`
    ///
    /// Add the eight signed 16-bit lanes with signed saturation.
    paddsw,
    rust_isa_sse2_paddsw
);
binary!(
    /// `paddusw`
    ///
    /// Add the eight unsigned 16-bit lanes with unsigned saturation.
    paddusw,
    rust_isa_sse2_paddusw
);
binary!(
    /// `psubsw`
    ///
    /// Subtract the eight signed 16-bit lanes with signed saturation.
    psubsw,
    rust_isa_sse2_psubsw
);
binary!(
    /// `psubusw`
    ///
    /// Subtract the eight unsigned 16-bit lanes with unsigned saturation.
    psubusw,
    rust_isa_sse2_psubusw
);
binary!(
    /// `packssdw`
    ///
    /// Pack 32-bit lanes from `a` and `b` into signed words with saturation.
    packssdw,
    rust_isa_sse2_packssdw
);
binary!(
    /// `pmulhw`
    ///
    /// Multiply the eight signed 16-bit lanes and keep the high 16 bits.
    pmulhw,
    rust_isa_sse2_pmulhw
);
binary!(
    /// `pmulhuw`
    ///
    /// Multiply the eight unsigned 16-bit lanes and keep the high 16 bits.
    pmulhuw,
    rust_isa_sse2_pmulhuw
);

/// `pminsw`
///
/// Take the minimum of the eight signed 16-bit integer lanes.
#[inline(always)]
pub fn pminsw(a: V128, b: V128) -> V128 {
    pminw(a, b)
}

/// `pmaxsw`
///
/// Take the maximum of the eight signed 16-bit integer lanes.
#[inline(always)]
pub fn pmaxsw(a: V128, b: V128) -> V128 {
    pmaxw(a, b)
}
