//! `V128`, `V64`, and `v128.*`.

/// A 64-bit value.
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct V64([u8; 8]);

impl V64 {
    /// Keep the 8 bytes unchanged.
    pub const fn from_bytes(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }

    /// Return the 8 bytes.
    pub const fn to_bytes(self) -> [u8; 8] {
        self.0
    }

    /// Pointer to the first byte.
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Mutable pointer to the first byte.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

/// A 128-bit value.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct V128([u8; 16]);

impl V128 {
    /// Keep the 16 bytes unchanged.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Return the 16 bytes.
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0
    }

    /// Pointer to the first byte.
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Mutable pointer to the first byte.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

unsafe extern "C" {
    fn rust_isa_arm_neon_v128_load(p: *const u8, out: *mut u8);
    fn rust_isa_arm_neon_v128_store(p: *mut u8, value: *const u8);
}

/// Load 16 bytes.
///
/// # Safety
///
/// The caller must compile with NEON enabled. `p` must be valid for reading
/// 16 bytes.
#[inline(always)]
pub unsafe fn v128_load(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_arm_neon_v128_load(p, out.as_mut_ptr());
    }
    out
}

/// Store 16 bytes.
///
/// # Safety
///
/// The caller must compile with NEON enabled. `p` must be valid for writing
/// 16 bytes.
#[inline(always)]
pub unsafe fn v128_store(p: *mut u8, value: V128) {
    unsafe {
        rust_isa_arm_neon_v128_store(p, value.as_ptr());
    }
}
