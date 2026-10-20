//! `V512` and `v512.*`.

/// A 512-bit value.
///
/// The instruction decides what the lanes mean. The layout is 64 bytes in
/// memory, aligned to 64.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct V512([u8; 64]);

impl V512 {
    /// Keep the 64 bytes unchanged. Multi-byte lanes are little-endian.
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Return the 64 bytes.
    pub const fn to_bytes(self) -> [u8; 64] {
        self.0
    }

    /// Pointer to the first byte. The alignment is 64.
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Mutable pointer to the first byte. The alignment is 64.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

unsafe extern "C" {
    fn rust_isa_avx512f_v512_load(p: *const u8, out: *mut u8);
    fn rust_isa_avx512f_v512_store(p: *mut u8, value: *const u8);
}

/// `v512.load`
///
/// Load 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F. `p` must be valid for reading 64 bytes.
/// The load does not require alignment.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn v512_load(p: *const u8) -> V512 {
    let mut out = V512::from_bytes([0; 64]);
    unsafe {
        rust_isa_avx512f_v512_load(p, out.as_mut_ptr());
    }
    out
}

/// `v512.store`
///
/// Store 64 bytes.
///
/// # Safety
///
/// The caller must enable AVX-512F. `p` must be valid for writing 64 bytes.
/// The store does not require alignment.
#[target_feature(enable = "avx512f")]
#[inline]
pub unsafe fn v512_store(p: *mut u8, value: V512) {
    unsafe {
        rust_isa_avx512f_v512_store(p, value.as_ptr());
    }
}
