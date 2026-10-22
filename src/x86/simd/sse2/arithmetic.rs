//! Arithmetic instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_addps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `addps`
///
/// Add the four 32-bit float lanes.
#[inline(always)]
pub fn addps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_addps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_subps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `subps`
///
/// Subtract the four 32-bit float lanes.
#[inline(always)]
pub fn subps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_subps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_mulps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `mulps`
///
/// Multiply the four 32-bit float lanes.
#[inline(always)]
pub fn mulps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_mulps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_divps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `divps`
///
/// Divide the four 32-bit float lanes.
#[inline(always)]
pub fn divps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_divps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_minps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `minps`
///
/// Take the minimum of the four 32-bit float lanes.
#[inline(always)]
pub fn minps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_minps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_maxps(a: *const u8, b: *const u8, out: *mut u8);
}

/// `maxps`
///
/// Take the maximum of the four 32-bit float lanes.
#[inline(always)]
pub fn maxps(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_maxps(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_sqrtps(a: *const u8, out: *mut u8);
}

/// `sqrtps`
///
/// Take the square root of the four 32-bit float lanes.
#[inline(always)]
pub fn sqrtps(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_sqrtps(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_addpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `addpd`
///
/// Add the two 64-bit float lanes.
#[inline(always)]
pub fn addpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_addpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_subpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `subpd`
///
/// Subtract the two 64-bit float lanes.
#[inline(always)]
pub fn subpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_subpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_mulpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `mulpd`
///
/// Multiply the two 64-bit float lanes.
#[inline(always)]
pub fn mulpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_mulpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_divpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `divpd`
///
/// Divide the two 64-bit float lanes.
#[inline(always)]
pub fn divpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_divpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_minpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `minpd`
///
/// Take the minimum of the two 64-bit float lanes.
#[inline(always)]
pub fn minpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_minpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_maxpd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `maxpd`
///
/// Take the maximum of the two 64-bit float lanes.
#[inline(always)]
pub fn maxpd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_maxpd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_sqrtpd(a: *const u8, out: *mut u8);
}

/// `sqrtpd`
///
/// Take the square root of the two 64-bit float lanes.
#[inline(always)]
pub fn sqrtpd(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_sqrtpd(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_paddb(a: *const u8, b: *const u8, out: *mut u8);
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

unsafe extern "C" {
    fn rust_isa_sse2_psubb(a: *const u8, b: *const u8, out: *mut u8);
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
}

/// `paddsb`
///
/// Add the 16 signed byte lanes with signed saturation.
#[inline(always)]
pub fn paddsb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_paddusb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddusb`
///
/// Add the 16 unsigned byte lanes with unsigned saturation.
#[inline(always)]
pub fn paddusb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddusb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psubsb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `psubsb`
///
/// Subtract the 16 signed byte lanes with signed saturation.
#[inline(always)]
pub fn psubsb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubsb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psubusb(a: *const u8, b: *const u8, out: *mut u8);
}

/// `psubusb`
///
/// Subtract the 16 unsigned byte lanes with unsigned saturation.
#[inline(always)]
pub fn psubusb(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubusb(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_paddw(a: *const u8, b: *const u8, out: *mut u8);
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

unsafe extern "C" {
    fn rust_isa_sse2_psubw(a: *const u8, b: *const u8, out: *mut u8);
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

unsafe extern "C" {
    fn rust_isa_sse2_pmullw(a: *const u8, b: *const u8, out: *mut u8);
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

unsafe extern "C" {
    fn rust_isa_sse2_paddsw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddsw`
///
/// Add the eight signed 16-bit lanes with signed saturation.
#[inline(always)]
pub fn paddsw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_paddusw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddusw`
///
/// Add the eight unsigned 16-bit lanes with unsigned saturation.
#[inline(always)]
pub fn paddusw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddusw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psubsw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `psubsw`
///
/// Subtract the eight signed 16-bit lanes with signed saturation.
#[inline(always)]
pub fn psubsw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubsw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psubusw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `psubusw`
///
/// Subtract the eight unsigned 16-bit lanes with unsigned saturation.
#[inline(always)]
pub fn psubusw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubusw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pmulhw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pmulhw`
///
/// Multiply the eight signed 16-bit lanes and keep the high 16 bits.
#[inline(always)]
pub fn pmulhw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pmulhw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pmulhuw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pmulhuw`
///
/// Multiply the eight unsigned 16-bit lanes and keep the high 16 bits.
#[inline(always)]
pub fn pmulhuw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pmulhuw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_paddd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddd`
///
/// Add the four 32-bit integer lanes.
///
/// ```
/// use rust_isa::x86::simd::sse2::{paddd, V128};
///
/// let a = V128::from_bytes([
///     1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0,
/// ]);
/// let b = V128::from_bytes([
///     10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0,
/// ]);
/// let c = paddd(a, b);
/// assert_eq!(c.to_bytes()[0], 11);
/// assert_eq!(c.to_bytes()[4], 22);
/// ```
#[inline(always)]
pub fn paddd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psubd(a: *const u8, b: *const u8, out: *mut u8);
}

/// `psubd`
///
/// Subtract the four 32-bit integer lanes, wrapping at 32 bits.
#[inline(always)]
pub fn psubd(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_pmuludq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `pmuludq`
///
/// Multiply the low unsigned 32-bit value of each 64-bit lane.
#[inline(always)]
pub fn pmuludq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_pmuludq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_paddq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `paddq`
///
/// Add the two 64-bit integer lanes, wrapping at 64 bits.
#[inline(always)]
pub fn paddq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_paddq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psubq(a: *const u8, b: *const u8, out: *mut u8);
}

/// `psubq`
///
/// Subtract the two 64-bit integer lanes, wrapping at 64 bits.
#[inline(always)]
pub fn psubq(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psubq(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_sse2_psadbw(a: *const u8, b: *const u8, out: *mut u8);
}

/// `psadbw`
///
/// Sum the absolute differences of the byte lanes into two 64-bit lanes.
#[inline(always)]
pub fn psadbw(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_psadbw(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}
