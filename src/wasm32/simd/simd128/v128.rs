//! `v128.*` instructions and the [`V128`] value.
//!
//! Rules shared by every SIMD128 instruction are documented on the [parent module](super).
//!
//! # Compile-time lane check
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{i16x8_splat, v128_load16_lane};
//!
//! let buffer = [0u8; 2];
//! let _ = unsafe { v128_load16_lane::<8>(buffer.as_ptr(), i16x8_splat(0)) };
//! ```
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{i32x4_splat, v128_load32_lane};
//!
//! let buffer = [0u8; 4];
//! let _ = unsafe { v128_load32_lane::<4>(buffer.as_ptr(), i32x4_splat(0)) };
//! ```
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{i64x2_splat, v128_load64_lane};
//!
//! let buffer = [0u8; 8];
//! let _ = unsafe { v128_load64_lane::<2>(buffer.as_ptr(), i64x2_splat(0)) };
//! ```
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{i8x16_splat, v128_store8_lane};
//!
//! let mut buffer = [0u8; 1];
//! unsafe { v128_store8_lane::<16>(buffer.as_mut_ptr(), i8x16_splat(0)) };
//! ```
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{i16x8_splat, v128_store16_lane};
//!
//! let mut buffer = [0u8; 2];
//! unsafe { v128_store16_lane::<8>(buffer.as_mut_ptr(), i16x8_splat(0)) };
//! ```
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{i32x4_splat, v128_store32_lane};
//!
//! let mut buffer = [0u8; 4];
//! unsafe { v128_store32_lane::<4>(buffer.as_mut_ptr(), i32x4_splat(0)) };
//! ```
//!
//! ```compile_fail,E0080
//! use rust_isa::wasm32::simd::simd128::{i64x2_splat, v128_store64_lane};
//!
//! let mut buffer = [0u8; 8];
//! unsafe { v128_store64_lane::<2>(buffer.as_mut_ptr(), i64x2_splat(0)) };
//! ```

/// A WebAssembly `v128`: 16 bytes.
///
/// The instruction decides what the lanes mean. [`Self::from_bytes`], [`Self::to_bytes`], and [`Self::from_u64x2`] are const and emit no SIMD.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct V128([u8; 16]);

impl V128 {
    /// Keep the 16 bytes unchanged. Multi-byte lanes are little-endian.
    ///
    /// ```
    /// use rust_isa::wasm32::simd::simd128::{i32x4_extract_lane, V128};
    ///
    /// let value = V128::from_bytes([
    ///     0x78, 0x56, 0x34, 0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /// ]);
    /// assert_eq!(i32x4_extract_lane::<0>(value), 0x1234_5678);
    /// assert_eq!(value.to_bytes()[0], 0x78);
    /// ```
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Return the 16 bytes.
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0
    }

    /// Place `lo` in the low 8 bytes and `hi` in the high 8 bytes, each little-endian.
    ///
    /// ```
    /// use rust_isa::wasm32::simd::simd128::V128;
    ///
    /// const VALUE: V128 = V128::from_u64x2(0x0807_0605_0403_0201, 0x100f_0e0d_0c0b_0a09);
    /// assert_eq!(VALUE.to_bytes()[0], 1);
    /// assert_eq!(VALUE.to_bytes()[8], 9);
    /// ```
    pub const fn from_u64x2(lo: u64, hi: u64) -> Self {
        let mut bytes = [0; 16];
        let mut i = 0;
        while i < 8 {
            bytes[i] = (lo >> (i * 8)) as u8;
            bytes[i + 8] = (hi >> (i * 8)) as u8;
            i += 1;
        }
        Self(bytes)
    }

    /// Pointer to the first byte. The alignment is 16.
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Mutable pointer to the first byte. The alignment is 16.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load(p: *const u8, out: *mut u8);
}
/// `v128.load`
///
/// Load 16 bytes.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_extract_lane, f32x4_splat, v128_load, v128_store};
///
/// let mut buffer = [0u8; 17];
/// unsafe {
///     v128_store(buffer.as_mut_ptr().add(1), f32x4_splat(1.5));
///     assert_eq!(
///         f32x4_extract_lane::<0>(v128_load(buffer.as_ptr().add(1))),
///         1.5,
///     );
/// }
/// ```
///
/// # Safety
///
/// `ptr` must be valid for 16 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_splat(p: *const u8, out: *mut u8);
}
/// `v128.load8_splat`
///
/// Load 1 byte and replicate it into all 16 lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::v128_load8_splat;
///
/// let buffer = [9u8];
/// assert_eq!(unsafe { v128_load8_splat(buffer.as_ptr()) }.to_bytes(), [9; 16]);
/// ```
///
/// # Safety
///
/// `ptr` must be valid for 1 byte. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load8_splat(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load8_splat(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_splat(p: *const u8, out: *mut u8);
}
/// `v128.load16_splat`
///
/// Load 2 bytes and replicate that `i16` into all 8 lanes.
///
/// # Safety
///
/// `ptr` must be valid for 2 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load16_splat(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load16_splat(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load32_splat(p: *const u8, out: *mut u8);
}
/// `v128.load32_splat`
///
/// Load 4 bytes and replicate that `i32` into all 4 lanes.
///
/// # Safety
///
/// `ptr` must be valid for 4 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load32_splat(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load32_splat(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load64_splat(p: *const u8, out: *mut u8);
}
/// `v128.load64_splat`
///
/// Load 8 bytes and replicate that `i64` into both lanes.
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load64_splat(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load64_splat(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load32_zero(p: *const u8, out: *mut u8);
}
/// `v128.load32_zero`
///
/// Load 4 bytes into the low bytes. The remaining bytes are `0`.
///
/// # Safety
///
/// `ptr` must be valid for 4 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load32_zero(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load32_zero(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load64_zero(p: *const u8, out: *mut u8);
}
/// `v128.load64_zero`
///
/// Load 8 bytes into the low bytes. The remaining bytes are `0`.
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load64_zero(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load64_zero(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_0(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_1(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_2(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_3(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_4(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_5(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_6(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_7(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_8(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_9(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_10(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_11(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_12(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_13(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_14(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load8_lane_15(p: *const u8, vec: *const u8, out: *mut u8);
}
/// `v128.load8_lane`
///
/// Load 1 byte into `LANE`. The other lanes stay as they are in `vec`.
///
/// `LANE` is an immediate in `0..16`. A constant outside that range does not compile.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i8x16_extract_lane_u, i8x16_splat, v128_load8_lane};
///
/// let buffer = [7u8];
/// let loaded = unsafe { v128_load8_lane::<2>(buffer.as_ptr(), i8x16_splat(0)) };
/// assert_eq!(i8x16_extract_lane_u::<2>(loaded), 7);
/// assert_eq!(i8x16_extract_lane_u::<0>(loaded), 0);
/// ```
///
/// ```compile_fail,E0080
/// use rust_isa::wasm32::simd::simd128::{i8x16_splat, v128_load8_lane};
///
/// let buffer = [0u8; 1];
/// let _ = unsafe { v128_load8_lane::<16>(buffer.as_ptr(), i8x16_splat(0)) };
/// ```
///
/// # Safety
///
/// `ptr` must be valid for 1 byte. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load8_lane<const LANE: u32>(ptr: *const u8, vec: V128) -> V128 {
    const {
        assert!(LANE < 16);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_0(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_1(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            2 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_2(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            3 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_3(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            4 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_4(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            5 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_5(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            6 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_6(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            7 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_7(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            8 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_8(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            9 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_9(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            10 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_10(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            11 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_11(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            12 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_12(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            13 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_13(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            14 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_14(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            15 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load8_lane_15(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_lane_0(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_lane_1(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_lane_2(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_lane_3(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_lane_4(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_lane_5(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_lane_6(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load16_lane_7(p: *const u8, vec: *const u8, out: *mut u8);
}
/// `v128.load16_lane`
///
/// Load 2 bytes into `LANE`. The other lanes stay as they are in `vec`.
///
/// `LANE` is an immediate in `0..8`. A constant outside that range does not compile.
///
/// # Safety
///
/// `ptr` must be valid for 2 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load16_lane<const LANE: u32>(ptr: *const u8, vec: V128) -> V128 {
    const {
        assert!(LANE < 8);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load16_lane_0(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load16_lane_1(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            2 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load16_lane_2(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            3 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load16_lane_3(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            4 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load16_lane_4(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            5 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load16_lane_5(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            6 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load16_lane_6(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            7 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load16_lane_7(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load32_lane_0(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load32_lane_1(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load32_lane_2(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load32_lane_3(p: *const u8, vec: *const u8, out: *mut u8);
}
/// `v128.load32_lane`
///
/// Load 4 bytes into `LANE`. The other lanes stay as they are in `vec`.
///
/// `LANE` is an immediate in `0..4`. A constant outside that range does not compile.
///
/// # Safety
///
/// `ptr` must be valid for 4 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load32_lane<const LANE: u32>(ptr: *const u8, vec: V128) -> V128 {
    const {
        assert!(LANE < 4);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load32_lane_0(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load32_lane_1(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            2 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load32_lane_2(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            3 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load32_lane_3(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load64_lane_0(p: *const u8, vec: *const u8, out: *mut u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_load64_lane_1(p: *const u8, vec: *const u8, out: *mut u8);
}
/// `v128.load64_lane`
///
/// Load 8 bytes into `LANE`. The other lane stays as it is in `vec`.
///
/// `LANE` is an immediate in `0..2`. A constant outside that range does not compile.
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load64_lane<const LANE: u32>(ptr: *const u8, vec: V128) -> V128 {
    const {
        assert!(LANE < 2);
    }
    unsafe {
        match LANE {
            0 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load64_lane_0(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            1 => {
                let mut out = V128::from_bytes([0; 16]);
                rust_isa_simd128_v128_load64_lane_1(ptr, vec.as_ptr(), out.as_mut_ptr());
                out
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_store(p: *mut u8, a: *const u8);
}
/// `v128.store`
///
/// Store 16 bytes.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{f32x4_splat, v128_store};
///
/// let mut buffer = [0u8; 17];
/// unsafe {
///     v128_store(buffer.as_mut_ptr().add(1), f32x4_splat(1.5));
/// }
/// assert_eq!(&buffer[1..5], &1.5f32.to_le_bytes());
/// ```
///
/// # Safety
///
/// `ptr` must be valid for 16 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_store(ptr: *mut u8, value: V128) {
    unsafe {
        rust_isa_simd128_v128_store(ptr, value.as_ptr());
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_0(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_1(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_2(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_3(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_4(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_5(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_6(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_7(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_8(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_9(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_10(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_11(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_12(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_13(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_14(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store8_lane_15(p: *mut u8, vec: *const u8);
}
/// `v128.store8_lane`
///
/// Store the 1 byte in `LANE`.
///
/// `LANE` is an immediate in `0..16`. A constant outside that range does not compile.
///
/// # Safety
///
/// `ptr` must be valid for 1 byte. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_store8_lane<const LANE: u32>(ptr: *mut u8, value: V128) {
    const {
        assert!(LANE < 16);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_v128_store8_lane_0(ptr, value.as_ptr()),
            1 => rust_isa_simd128_v128_store8_lane_1(ptr, value.as_ptr()),
            2 => rust_isa_simd128_v128_store8_lane_2(ptr, value.as_ptr()),
            3 => rust_isa_simd128_v128_store8_lane_3(ptr, value.as_ptr()),
            4 => rust_isa_simd128_v128_store8_lane_4(ptr, value.as_ptr()),
            5 => rust_isa_simd128_v128_store8_lane_5(ptr, value.as_ptr()),
            6 => rust_isa_simd128_v128_store8_lane_6(ptr, value.as_ptr()),
            7 => rust_isa_simd128_v128_store8_lane_7(ptr, value.as_ptr()),
            8 => rust_isa_simd128_v128_store8_lane_8(ptr, value.as_ptr()),
            9 => rust_isa_simd128_v128_store8_lane_9(ptr, value.as_ptr()),
            10 => rust_isa_simd128_v128_store8_lane_10(ptr, value.as_ptr()),
            11 => rust_isa_simd128_v128_store8_lane_11(ptr, value.as_ptr()),
            12 => rust_isa_simd128_v128_store8_lane_12(ptr, value.as_ptr()),
            13 => rust_isa_simd128_v128_store8_lane_13(ptr, value.as_ptr()),
            14 => rust_isa_simd128_v128_store8_lane_14(ptr, value.as_ptr()),
            15 => rust_isa_simd128_v128_store8_lane_15(ptr, value.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_store16_lane_0(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store16_lane_1(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store16_lane_2(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store16_lane_3(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store16_lane_4(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store16_lane_5(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store16_lane_6(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store16_lane_7(p: *mut u8, vec: *const u8);
}
/// `v128.store16_lane`
///
/// Store the 2 bytes in `LANE`.
///
/// `LANE` is an immediate in `0..8`. A constant outside that range does not compile.
///
/// # Safety
///
/// `ptr` must be valid for 2 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_store16_lane<const LANE: u32>(ptr: *mut u8, value: V128) {
    const {
        assert!(LANE < 8);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_v128_store16_lane_0(ptr, value.as_ptr()),
            1 => rust_isa_simd128_v128_store16_lane_1(ptr, value.as_ptr()),
            2 => rust_isa_simd128_v128_store16_lane_2(ptr, value.as_ptr()),
            3 => rust_isa_simd128_v128_store16_lane_3(ptr, value.as_ptr()),
            4 => rust_isa_simd128_v128_store16_lane_4(ptr, value.as_ptr()),
            5 => rust_isa_simd128_v128_store16_lane_5(ptr, value.as_ptr()),
            6 => rust_isa_simd128_v128_store16_lane_6(ptr, value.as_ptr()),
            7 => rust_isa_simd128_v128_store16_lane_7(ptr, value.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_store32_lane_0(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store32_lane_1(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store32_lane_2(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store32_lane_3(p: *mut u8, vec: *const u8);
}
/// `v128.store32_lane`
///
/// Store the 4 bytes in `LANE`.
///
/// `LANE` is an immediate in `0..4`. A constant outside that range does not compile.
///
/// # Safety
///
/// `ptr` must be valid for 4 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_store32_lane<const LANE: u32>(ptr: *mut u8, value: V128) {
    const {
        assert!(LANE < 4);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_v128_store32_lane_0(ptr, value.as_ptr()),
            1 => rust_isa_simd128_v128_store32_lane_1(ptr, value.as_ptr()),
            2 => rust_isa_simd128_v128_store32_lane_2(ptr, value.as_ptr()),
            3 => rust_isa_simd128_v128_store32_lane_3(ptr, value.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_store64_lane_0(p: *mut u8, vec: *const u8);
}
unsafe extern "C" {
    fn rust_isa_simd128_v128_store64_lane_1(p: *mut u8, vec: *const u8);
}
/// `v128.store64_lane`
///
/// Store the 8 bytes in `LANE`.
///
/// `LANE` is an immediate in `0..2`. A constant outside that range does not compile.
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_store64_lane<const LANE: u32>(ptr: *mut u8, value: V128) {
    const {
        assert!(LANE < 2);
    }
    unsafe {
        match LANE {
            0 => rust_isa_simd128_v128_store64_lane_0(ptr, value.as_ptr()),
            1 => rust_isa_simd128_v128_store64_lane_1(ptr, value.as_ptr()),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_not(a: *const u8, out: *mut u8);
}
/// `v128.not`
///
/// Bitwise NOT.
#[inline(always)]
pub fn v128_not(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_not(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_and(a: *const u8, b: *const u8, out: *mut u8);
}
/// `v128.and`
///
/// Bitwise AND.
#[inline(always)]
pub fn v128_and(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_and(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_or(a: *const u8, b: *const u8, out: *mut u8);
}
/// `v128.or`
///
/// Bitwise OR.
#[inline(always)]
pub fn v128_or(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_or(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_xor(a: *const u8, b: *const u8, out: *mut u8);
}
/// `v128.xor`
///
/// Bitwise XOR.
#[inline(always)]
pub fn v128_xor(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_xor(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_andnot(a: *const u8, b: *const u8, out: *mut u8);
}
/// `v128.andnot`
///
/// Bitwise `a & !b`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{v128_andnot, V128};
///
/// let a = V128::from_bytes([0xff; 16]);
/// let b = V128::from_bytes([0x0f; 16]);
/// assert_eq!(v128_andnot(a, b).to_bytes(), [0xf0; 16]);
/// ```
#[inline(always)]
pub fn v128_andnot(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_andnot(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_any_true(a: *const u8) -> i32;
}
/// `v128.any_true`
///
/// Return `1` when any bit is set, otherwise `0`.
#[inline(always)]
pub fn v128_any_true(a: V128) -> i32 {
    unsafe { rust_isa_simd128_v128_any_true(a.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_bitselect(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `v128.bitselect`
///
/// For each bit, a `1` in `c` selects `a` and a `0` in `c` selects `b`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{v128_bitselect, V128};
///
/// let a = V128::from_bytes([0xff; 16]);
/// let b = V128::from_bytes([0x00; 16]);
/// let c = V128::from_bytes([0x0f; 16]);
/// assert_eq!(v128_bitselect(a, b, c).to_bytes(), [0x0f; 16]);
/// ```
#[inline(always)]
pub fn v128_bitselect(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_bitselect(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load8x8_s(p: *const u8, out: *mut u8);
}
/// `v128.load8x8_s`
///
/// Load 8 bytes and sign-extend each byte to `i16`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i16x8_extract_lane_s, v128_load8x8_s};
///
/// let bytes = [0xffu8, 0x02, 0, 0, 0, 0, 0, 0];
/// let loaded = unsafe { v128_load8x8_s(bytes.as_ptr()) };
/// assert_eq!(i16x8_extract_lane_s::<0>(loaded), -1);
/// assert_eq!(i16x8_extract_lane_s::<1>(loaded), 2);
/// ```
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load8x8_s(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load8x8_s(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load8x8_u(p: *const u8, out: *mut u8);
}
/// `v128.load8x8_u`
///
/// Load 8 bytes and zero-extend each byte to `i16`.
///
/// # Examples
///
/// ```
/// use rust_isa::wasm32::simd::simd128::{i16x8_extract_lane_s, v128_load8x8_u};
///
/// let bytes = [0xffu8, 0x02, 0, 0, 0, 0, 0, 0];
/// assert_eq!(
///     i16x8_extract_lane_s::<0>(unsafe { v128_load8x8_u(bytes.as_ptr()) }),
///     255,
/// );
/// ```
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load8x8_u(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load8x8_u(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load16x4_s(p: *const u8, out: *mut u8);
}
/// `v128.load16x4_s`
///
/// Load four `i16` values and sign-extend each to `i32`.
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load16x4_s(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load16x4_s(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load16x4_u(p: *const u8, out: *mut u8);
}
/// `v128.load16x4_u`
///
/// Load four `i16` values and zero-extend each to `i32`.
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load16x4_u(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load16x4_u(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load32x2_s(p: *const u8, out: *mut u8);
}
/// `v128.load32x2_s`
///
/// Load two `i32` values and sign-extend each to `i64`.
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load32x2_s(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load32x2_s(ptr, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_simd128_v128_load32x2_u(p: *const u8, out: *mut u8);
}
/// `v128.load32x2_u`
///
/// Load two `i32` values and zero-extend each to `i64`.
///
/// # Safety
///
/// `ptr` must be valid for 8 bytes. A misaligned address does not trap. An out-of-bounds address traps.
#[inline(always)]
pub unsafe fn v128_load32x2_u(ptr: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_simd128_v128_load32x2_u(ptr, out.as_mut_ptr());
    }
    out
}
