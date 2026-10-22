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
