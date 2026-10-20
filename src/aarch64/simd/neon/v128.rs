//! `V128` and `v128.*`.

/// A 128-bit value.
///
/// The instruction decides what the lanes mean. The layout is 16 bytes in
/// memory, aligned to 16.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct V128([u8; 16]);

impl V128 {
    /// Keep the 16 bytes unchanged. Multi-byte lanes are little-endian.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Return the 16 bytes.
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0
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
    fn rust_isa_neon_v128_load(p: *const u8, out: *mut u8);
    fn rust_isa_neon_v128_store(p: *mut u8, value: *const u8);
}

/// `ldr q`
///
/// Load 16 bytes.
///
/// # Safety
///
/// `p` must be valid for reading 16 bytes.
#[inline(always)]
pub unsafe fn v128_load(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_v128_load(p, out.as_mut_ptr());
    }
    out
}

/// `str q`
///
/// Store 16 bytes.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes.
#[inline(always)]
pub unsafe fn v128_store(p: *mut u8, value: V128) {
    unsafe {
        rust_isa_neon_v128_store(p, value.as_ptr());
    }
}
