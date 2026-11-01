//! AVX-512 / AVX10 mask values.
//!
//! A mask is represented by its lane count, not by the physical k-register
//! width. The underlying integer is only the storage used at the C boundary.
//! `from_bits` truncates bits above the lane count; `from_bits_checked`
//! rejects them.
//!
//! `NONE` is an explicit all-false mask. It is not the same as omitting the
//! mask operand (`k0`); instructions without a mask argument do not use these
//! types.

macro_rules! mask_type {
    ($name:ident, $bits:ty, $lanes:expr, $all:expr, $doc:literal) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        pub struct $name($bits);

        impl $name {
            /// The number of lanes represented by this mask.
            pub const LANES: u32 = $lanes;

            /// An all-false mask.
            pub const NONE: Self = Self(0);

            /// An all-true mask.
            pub const ALL: Self = Self($all);

            /// Keep the low `LANES` bits and discard the rest.
            pub const fn from_bits(bits: $bits) -> Self {
                Self(bits & $all)
            }

            /// Return `None` when a bit above `LANES` is set.
            pub const fn from_bits_checked(bits: $bits) -> Option<Self> {
                if bits & !$all == 0 {
                    Some(Self(bits))
                } else {
                    None
                }
            }

            /// Return the stored bits.
            pub const fn bits(self) -> $bits {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::NONE
            }
        }

        impl core::ops::Not for $name {
            type Output = Self;

            fn not(self) -> Self {
                Self(!self.0 & $all)
            }
        }

        impl core::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl core::ops::BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }
    };
}

mask_type!(
    Mask1,
    u8,
    1,
    0b1,
    "A one-lane mask for scalar AVX-512 instructions."
);
mask_type!(
    Mask2,
    u8,
    2,
    0b11,
    "A two-lane mask for 128-bit instructions with 64-bit elements."
);
mask_type!(
    Mask4,
    u8,
    4,
    0b1111,
    "A four-lane mask for 128-bit instructions with 32-bit elements."
);
mask_type!(
    Mask8,
    u8,
    8,
    0xff,
    "An eight-lane mask for 256-bit instructions with 32-bit elements."
);
mask_type!(
    Mask16,
    u16,
    16,
    0xffff,
    "A sixteen-lane mask for 512-bit instructions with 32-bit elements."
);
mask_type!(Mask32, u32, 32, 0xffff_ffff, "A thirty-two-lane mask.");
mask_type!(
    Mask64,
    u64,
    64,
    0xffff_ffff_ffff_ffff,
    "A sixty-four-lane mask."
);
