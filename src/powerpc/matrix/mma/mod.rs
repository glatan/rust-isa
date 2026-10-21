//! PowerPC Power10 matrix-multiply assist (MMA) instructions.
//!
//! MMA is not part of the PowerPC baseline. Every operation is `unsafe` and
//! the caller must enable the Power10 MMA feature set. The C file is compiled
//! with `-mcpu=power10 -mmma`.
//!
//! [`MmaAccumulator`] owns the 64-byte image of the four 128-bit accumulator
//! registers. The instruction wrappers move that image to and from the
//! hardware accumulator, so the value can be kept in memory between calls.
//! `xxmfacc` returns the four accumulator registers and `xxmtacc` replaces
//! them.

use core::marker::PhantomData;

use crate::powerpc::simd::altivec::V128;

unsafe extern "C" {
    fn rust_isa_mma_xxsetaccz(acc: *mut u8);
    fn rust_isa_mma_xxmfacc(acc: *const u8, out: *mut u8);
    fn rust_isa_mma_xxmtacc(values: *const u8, out: *mut u8);
    fn rust_isa_mma_xvi4ger8(acc: *mut u8, a: *const u8, b: *const u8);
    fn rust_isa_mma_xvi4ger8pp(acc: *mut u8, a: *const u8, b: *const u8);
    fn rust_isa_mma_xvi8ger4(acc: *mut u8, a: *const u8, b: *const u8);
    fn rust_isa_mma_xvi8ger4pp(acc: *mut u8, a: *const u8, b: *const u8);
    fn rust_isa_mma_xvi8ger4spp(acc: *mut u8, a: *const u8, b: *const u8);
    fn rust_isa_mma_xvi16ger2(acc: *mut u8, a: *const u8, b: *const u8);
    fn rust_isa_mma_xvi16ger2s(acc: *mut u8, a: *const u8, b: *const u8);
    fn rust_isa_mma_xvi16ger2pp(acc: *mut u8, a: *const u8, b: *const u8);
    fn rust_isa_mma_xvi16ger2spp(acc: *mut u8, a: *const u8, b: *const u8);
}

/// An MMA accumulator.
///
/// The 64 bytes are the four 128-bit accumulator registers in the order
/// produced by `xxmfacc`. The type is neither `Send` nor `Sync` because the
/// instruction stream belongs to one hardware accumulator state.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MmaAccumulator {
    bytes: [u8; 64],
    _not_send_or_sync: PhantomData<*const ()>,
}

impl MmaAccumulator {
    /// Create a zeroed accumulator.
    ///
    /// This is equivalent to `xxsetaccz`.
    ///
    /// # Safety
    ///
    /// The caller must enable the Power10 MMA feature set.
    pub unsafe fn new() -> Self {
        let mut acc = Self {
            bytes: [0; 64],
            _not_send_or_sync: PhantomData,
        };
        unsafe {
            xxsetaccz(&mut acc);
        }
        acc
    }

    /// Keep the 64 bytes unchanged.
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self {
            bytes,
            _not_send_or_sync: PhantomData,
        }
    }

    /// Return the 64 bytes.
    pub const fn to_bytes(self) -> [u8; 64] {
        self.bytes
    }
}

fn bytes_from_values(values: [V128; 4]) -> [u8; 64] {
    let mut bytes = [0; 64];
    let mut index = 0;
    while index < 4 {
        let value = values[index].to_bytes();
        let mut byte = 0;
        while byte < 16 {
            bytes[index * 16 + byte] = value[byte];
            byte += 1;
        }
        index += 1;
    }
    bytes
}

fn values_from_bytes(bytes: [u8; 64]) -> [V128; 4] {
    let mut values = [V128::from_bytes([0; 16]); 4];
    let mut index = 0;
    while index < 4 {
        let mut value = [0; 16];
        let mut byte = 0;
        while byte < 16 {
            value[byte] = bytes[index * 16 + byte];
            byte += 1;
        }
        values[index] = V128::from_bytes(value);
        index += 1;
    }
    values
}

/// `xxsetaccz`
///
/// Set all accumulator registers to zero.
///
/// # Safety
///
/// The caller must enable the Power10 MMA feature set.
#[inline(always)]
pub unsafe fn xxsetaccz(acc: &mut MmaAccumulator) {
    unsafe {
        rust_isa_mma_xxsetaccz(acc.bytes.as_mut_ptr());
    }
}

/// `xxmfacc`
///
/// Return the four 128-bit accumulator registers.
///
/// # Safety
///
/// The caller must enable the Power10 MMA feature set.
#[inline(always)]
pub unsafe fn xxmfacc(acc: &MmaAccumulator) -> [V128; 4] {
    let mut bytes = [0; 64];
    unsafe {
        rust_isa_mma_xxmfacc(acc.bytes.as_ptr(), bytes.as_mut_ptr());
    }
    values_from_bytes(bytes)
}

/// `xxmtacc`
///
/// Replace the four 128-bit accumulator registers.
///
/// # Safety
///
/// The caller must enable the Power10 MMA feature set.
#[inline(always)]
pub unsafe fn xxmtacc(acc: &mut MmaAccumulator, values: [V128; 4]) {
    let bytes = bytes_from_values(values);
    unsafe {
        rust_isa_mma_xxmtacc(bytes.as_ptr(), acc.bytes.as_mut_ptr());
    }
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable the Power10 MMA feature set.
        #[inline(always)]
        pub unsafe fn $name(acc: &mut MmaAccumulator, a: V128, b: V128) {
            unsafe {
                $c(acc.bytes.as_mut_ptr(), a.as_ptr(), b.as_ptr());
            }
        }
    };
}

binary!(
    /// `xvi4ger8`
    ///
    /// Replace the accumulator with the rank-8 outer product of signed 4-bit
    /// integers.
    xvi4ger8,
    rust_isa_mma_xvi4ger8
);
binary!(
    /// `xvi4ger8pp`
    ///
    /// Add the rank-8 outer product of signed 4-bit integers to the
    /// accumulator.
    xvi4ger8pp,
    rust_isa_mma_xvi4ger8pp
);
binary!(
    /// `xvi8ger4`
    ///
    /// Replace the accumulator with the rank-4 outer product of signed 8-bit
    /// and unsigned 8-bit integers.
    xvi8ger4,
    rust_isa_mma_xvi8ger4
);
binary!(
    /// `xvi8ger4pp`
    ///
    /// Add the rank-4 outer product of signed 8-bit and unsigned 8-bit
    /// integers to the accumulator.
    xvi8ger4pp,
    rust_isa_mma_xvi8ger4pp
);
binary!(
    /// `xvi8ger4spp`
    ///
    /// Add the rank-4 outer product of signed 8-bit and unsigned 8-bit
    /// integers to the accumulator with signed saturation.
    xvi8ger4spp,
    rust_isa_mma_xvi8ger4spp
);
binary!(
    /// `xvi16ger2`
    ///
    /// Replace the accumulator with the rank-2 outer product of signed 16-bit
    /// integers.
    xvi16ger2,
    rust_isa_mma_xvi16ger2
);
binary!(
    /// `xvi16ger2s`
    ///
    /// Replace the accumulator with the rank-2 outer product of signed 16-bit
    /// integers. The result is saturated to the signed 32-bit range.
    xvi16ger2s,
    rust_isa_mma_xvi16ger2s
);
binary!(
    /// `xvi16ger2pp`
    ///
    /// Add the rank-2 outer product of signed 16-bit integers to the
    /// accumulator.
    xvi16ger2pp,
    rust_isa_mma_xvi16ger2pp
);
binary!(
    /// `xvi16ger2spp`
    ///
    /// Add the rank-2 outer product of signed 16-bit integers to the
    /// accumulator with signed saturation.
    xvi16ger2spp,
    rust_isa_mma_xvi16ger2spp
);
