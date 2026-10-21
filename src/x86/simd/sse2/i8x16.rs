//! 8-bit integer instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_paddb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pcmpeqb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddb`
///
/// Add the 16 byte lanes, wrapping at 8 bits.
#[inline(always)]
pub fn paddb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `psubb`
///
/// Subtract the 16 byte lanes, wrapping at 8 bits.
#[inline(always)]
pub fn psubb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `pcmpeqb`
///
/// Compare the 16 byte lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpeqb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpeqb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pcmpgtb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pcmpgtb`
///
/// Compare the 16 signed byte lanes. A true lane is all bits one.
#[inline(always)]
pub fn pcmpgtb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pcmpgtb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pminub(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pminub`
///
/// Take the minimum of the 16 unsigned byte lanes.
#[inline(always)]
pub fn pminub(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pminub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pmaxub(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pmaxub`
///
/// Take the maximum of the 16 unsigned byte lanes.
#[inline(always)]
pub fn pmaxub(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pmaxub(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pavgb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pavgb`
///
/// Average the 16 unsigned byte lanes, rounding up.
#[inline(always)]
pub fn pavgb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pavgb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_paddsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_paddusb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubsb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_psubusb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_packsswb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_packuswb(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_sse2_pmovmskb(a: *const u8) -> i32;
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
    /// `paddsb`
    ///
    /// Add the 16 signed byte lanes with signed saturation.
    paddsb,
    rust_isa_sse2_paddsb
);
binary!(
    /// `paddusb`
    ///
    /// Add the 16 unsigned byte lanes with unsigned saturation.
    paddusb,
    rust_isa_sse2_paddusb
);
binary!(
    /// `psubsb`
    ///
    /// Subtract the 16 signed byte lanes with signed saturation.
    psubsb,
    rust_isa_sse2_psubsb
);
binary!(
    /// `psubusb`
    ///
    /// Subtract the 16 unsigned byte lanes with unsigned saturation.
    psubusb,
    rust_isa_sse2_psubusb
);
binary!(
    /// `packsswb`
    ///
    /// Pack 16-bit lanes from `a` and `b` into signed bytes with saturation.
    packsswb,
    rust_isa_sse2_packsswb
);
binary!(
    /// `packuswb`
    ///
    /// Pack 16-bit lanes from `a` and `b` into unsigned bytes with saturation.
    packuswb,
    rust_isa_sse2_packuswb
);

/// `pmovmskb`
///
/// Collect the most significant bit of each byte into the low 16 bits.
#[inline(always)]
pub fn pmovmskb(a: V128) -> u32 {
    unsafe { rust_isa_sse2_pmovmskb(a.as_ptr()) as u32 }
}
