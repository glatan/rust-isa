//! LoongArch Binary Translation (LBT) bit-manipulation instructions.
//!
//! The operations are expressed on 64-bit scalar values.
//!
//! Instruction behavior follows the [LoongArch Documentation](https://loongson.github.io/LoongArch-Documentation/).

/// `revb.d`: reverse the byte order of a 64-bit value.
#[inline(always)]
pub fn revb(value: u64) -> u64 {
    value.swap_bytes()
}

/// `revh.d`: reverse the order of the four 16-bit halfwords.
#[inline(always)]
pub fn revh(value: u64) -> u64 {
    ((value & 0x0000_0000_0000_ffff) << 48)
        | ((value & 0x0000_0000_ffff_0000) << 16)
        | ((value & 0x0000_ffff_0000_0000) >> 16)
        | ((value & 0xffff_0000_0000_0000) >> 48)
}

/// `revw.d`: reverse the order of the two 32-bit words.
#[inline(always)]
pub fn revw(value: u64) -> u64 {
    value.rotate_left(32)
}

/// `bitrev.d`: reverse all 64 bits.
#[inline(always)]
pub fn bitrev(value: u64) -> u64 {
    value.reverse_bits()
}

/// `bstrins.d`: insert the low bits of `src` into `dst` at `LSB`.
///
/// `MSB` and `LSB` are inclusive and must satisfy `LSB <= MSB <= 63`.
#[inline(always)]
pub fn bstrins<const MSB: u32, const LSB: u32>(dst: u64, src: u64) -> u64 {
    const { assert!(LSB <= MSB && MSB <= 63, "invalid bit range") };
    let width = MSB - LSB + 1;
    let mask = if width == 64 {
        u64::MAX
    } else {
        ((1u64 << width) - 1) << LSB
    };
    (dst & !mask) | ((src << LSB) & mask)
}

/// `bstrpick.d`: extract bits `LSB..=MSB` from `src`.
#[inline(always)]
pub fn bstrpick<const MSB: u32, const LSB: u32>(src: u64) -> u64 {
    const { assert!(LSB <= MSB && MSB <= 63, "invalid bit range") };
    let width = MSB - LSB + 1;
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };
    (src >> LSB) & mask
}

/// `ext` alias for [`bstrpick`].
#[inline(always)]
pub fn ext<const MSB: u32, const LSB: u32>(src: u64) -> u64 {
    bstrpick::<MSB, LSB>(src)
}

/// `ins` alias for [`bstrins`].
#[inline(always)]
pub fn ins<const MSB: u32, const LSB: u32>(dst: u64, src: u64) -> u64 {
    bstrins::<MSB, LSB>(dst, src)
}
