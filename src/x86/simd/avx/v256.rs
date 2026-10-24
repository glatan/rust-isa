//! `V256` and its byte accessors.

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
