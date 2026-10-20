//! `V256` and `v256.*`.

/// A 256-bit value.
///
/// The instruction decides what the lanes mean. The layout is 32 bytes in
/// memory, aligned to 32.
#[repr(C, align(32))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct V256([u8; 32]);

impl V256 {
    /// Keep the 32 bytes unchanged. Multi-byte lanes are little-endian.
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Return the 32 bytes.
    pub const fn to_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Pointer to the first byte. The alignment is 32.
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Mutable pointer to the first byte. The alignment is 32.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

unsafe extern "C" {
    fn rust_isa_avx2_v256_load(p: *const u8, out: *mut u8);
    fn rust_isa_avx2_v256_store(p: *mut u8, value: *const u8);
}

/// `v256.load`
///
/// Load 32 bytes.
///
/// # Safety
///
/// The caller must enable AVX2. `p` must be valid for reading 32 bytes. The
/// load does not require alignment.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn v256_load(p: *const u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_v256_load(p, out.as_mut_ptr());
    }
    out
}

/// `v256.store`
///
/// Store 32 bytes.
///
/// # Safety
///
/// The caller must enable AVX2. `p` must be valid for writing 32 bytes. The
/// store does not require alignment.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn v256_store(p: *mut u8, value: V256) {
    unsafe {
        rust_isa_avx2_v256_store(p, value.as_ptr());
    }
}
