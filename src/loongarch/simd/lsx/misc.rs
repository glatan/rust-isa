use super::V128;

// Miscellaneous vector instructions.

/// `vbsll.v`
#[inline(always)]
pub fn vbsll(a: V128, shift: u32) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    let shift = (shift & 15) as usize;
    out[shift..].copy_from_slice(&bytes[..16 - shift]);
    V128::from_bytes(out)
}

/// `vbsrl.v`
#[inline(always)]
pub fn vbsrl(a: V128, shift: u32) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    let shift = (shift & 15) as usize;
    out[..16 - shift].copy_from_slice(&bytes[shift..]);
    V128::from_bytes(out)
}

/// `vseteqz.v`
#[inline(always)]
pub fn vseteqz(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes.iter().all(|&b| b == 0)
}

/// `vsetnez.v`
#[inline(always)]
pub fn vsetnez(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes.iter().any(|&b| b != 0)
}

/// `vsetanyeqz.b`
#[inline(always)]
pub fn vsetanyeqz_b(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes.contains(&0)
}

/// `vsetanyeqz.h`
#[inline(always)]
pub fn vsetanyeqz_h(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .any(|c| u16::from_le_bytes(*c) == 0)
}

/// `vsetanyeqz.w`
#[inline(always)]
pub fn vsetanyeqz_w(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .any(|c| u32::from_le_bytes(*c) == 0)
}

/// `vsetanyeqz.d`
#[inline(always)]
pub fn vsetanyeqz_d(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes
        .as_chunks::<8>()
        .0
        .iter()
        .any(|c| u64::from_le_bytes(*c) == 0)
}

/// `vsetallnez.b`
#[inline(always)]
pub fn vsetallnez_b(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes.iter().all(|&b| b != 0)
}

/// `vsetallnez.h`
#[inline(always)]
pub fn vsetallnez_h(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .all(|c| u16::from_le_bytes(*c) != 0)
}

/// `vsetallnez.w`
#[inline(always)]
pub fn vsetallnez_w(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .all(|c| u32::from_le_bytes(*c) != 0)
}

/// `vsetallnez.d`
#[inline(always)]
pub fn vsetallnez_d(a: V128) -> bool {
    let bytes = a.to_bytes();
    bytes
        .as_chunks::<8>()
        .0
        .iter()
        .all(|c| u64::from_le_bytes(*c) != 0)
}

/// `vbitseli.b`
#[inline(always)]
pub fn vbitseli_b<const IMM: u8>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 255, "immediate must fit in u8") };
    let bytes = a.to_bytes();
    let other = b.to_bytes();
    let mut out = [0u8; 16];
    for ((out_elem, bytes_elem), other_elem) in out.iter_mut().zip(bytes.iter()).zip(other.iter()) {
        *out_elem = (*bytes_elem & IMM) | (!*bytes_elem & *other_elem);
    }
    V128::from_bytes(out)
}

/// `vshuf4i.b`
#[inline(always)]
pub fn vshuf4i_b<const IMM: u8>(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for block in 0..4 {
        for lane in 0..4 {
            let sel = ((IMM >> (2 * lane)) & 3) as usize;
            let dst = (block * 4 + lane) * 1;
            let src = (block * 4 + sel) * 1;
            out[dst..dst + 1].copy_from_slice(&bytes[src..src + 1]);
        }
    }
    V128::from_bytes(out)
}

/// `vshuf4i.h`
#[inline(always)]
pub fn vshuf4i_h<const IMM: u8>(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for block in 0..2 {
        for lane in 0..4 {
            let sel = ((IMM >> (2 * lane)) & 3) as usize;
            let dst = (block * 4 + lane) * 2;
            let src = (block * 4 + sel) * 2;
            out[dst..dst + 2].copy_from_slice(&bytes[src..src + 2]);
        }
    }
    V128::from_bytes(out)
}

/// `vshuf4i.w`
#[inline(always)]
pub fn vshuf4i_w<const IMM: u8>(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for block in 0..1 {
        for lane in 0..4 {
            let sel = ((IMM >> (2 * lane)) & 3) as usize;
            let dst = (block * 4 + lane) * 4;
            let src = (block * 4 + sel) * 4;
            out[dst..dst + 4].copy_from_slice(&bytes[src..src + 4]);
        }
    }
    V128::from_bytes(out)
}

/// `vshuf4i.d`
#[inline(always)]
pub fn vshuf4i_d<const IMM: u8>(a: V128, b: V128) -> V128 {
    let left = a.to_bytes();
    let right = b.to_bytes();
    let mut out = [0u8; 16];
    for (lane, chunk) in out.as_chunks_mut::<8>().0.iter_mut().enumerate() {
        let field = (IMM >> (2 * lane)) & 3;
        let source = if field & 2 != 0 { &right } else { &left };
        let index = (field & 1) as usize;
        chunk.copy_from_slice(&source[index * 8..index * 8 + 8]);
    }
    V128::from_bytes(out)
}

/// `vpermi.w`
#[inline(always)]
pub fn vpermi_w<const IMM: u8>(a: V128, b: V128) -> V128 {
    let left = a.to_bytes();
    let right = b.to_bytes();
    let mut out = [0u8; 16];
    for (lane, chunk) in out.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        let index = ((IMM >> (2 * lane)) & 3) as usize;
        let source = if lane < 2 { &right } else { &left };
        chunk.copy_from_slice(&source[index * 4..index * 4 + 4]);
    }
    V128::from_bytes(out)
}

/// `vextrins.b`
#[inline(always)]
pub fn vextrins_b<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = a.to_bytes();
    let other = b.to_bytes();
    let index = IMM as usize;
    let dst = ((index >> 4) & 15) * 1;
    let src = (index & 15) * 1;
    out[dst..dst + 1].copy_from_slice(&other[src..src + 1]);
    V128::from_bytes(out)
}

/// `vextrins.h`
#[inline(always)]
pub fn vextrins_h<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = a.to_bytes();
    let other = b.to_bytes();
    let index = IMM as usize;
    let dst = ((index >> 4) & 7) * 2;
    let src = (index & 7) * 2;
    out[dst..dst + 2].copy_from_slice(&other[src..src + 2]);
    V128::from_bytes(out)
}

/// `vextrins.w`
#[inline(always)]
pub fn vextrins_w<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = a.to_bytes();
    let other = b.to_bytes();
    let index = IMM as usize;
    let dst = ((index >> 4) & 3) * 4;
    let src = (index & 3) * 4;
    out[dst..dst + 4].copy_from_slice(&other[src..src + 4]);
    V128::from_bytes(out)
}

/// `vextrins.d`
#[inline(always)]
pub fn vextrins_d<const IMM: u8>(a: V128, b: V128) -> V128 {
    let mut out = a.to_bytes();
    let other = b.to_bytes();
    let index = IMM as usize;
    let dst = ((index >> 4) & 1) * 8;
    let src = (index & 1) * 8;
    out[dst..dst + 8].copy_from_slice(&other[src..src + 8]);
    V128::from_bytes(out)
}

/// ``vfrstpi.b
#[inline(always)]
pub fn vfrstpi_b<const IMM: u32>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let mut out = V128::from_bytes([0; 16]);
    match IMM {
        0 => unsafe { rust_isa_lsx_vfrstpi_b_0(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        1 => unsafe { rust_isa_lsx_vfrstpi_b_1(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        2 => unsafe { rust_isa_lsx_vfrstpi_b_2(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        3 => unsafe { rust_isa_lsx_vfrstpi_b_3(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        4 => unsafe { rust_isa_lsx_vfrstpi_b_4(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        5 => unsafe { rust_isa_lsx_vfrstpi_b_5(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        6 => unsafe { rust_isa_lsx_vfrstpi_b_6(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        7 => unsafe { rust_isa_lsx_vfrstpi_b_7(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        8 => unsafe { rust_isa_lsx_vfrstpi_b_8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        9 => unsafe { rust_isa_lsx_vfrstpi_b_9(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        10 => unsafe { rust_isa_lsx_vfrstpi_b_10(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        11 => unsafe { rust_isa_lsx_vfrstpi_b_11(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        12 => unsafe { rust_isa_lsx_vfrstpi_b_12(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        13 => unsafe { rust_isa_lsx_vfrstpi_b_13(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        14 => unsafe { rust_isa_lsx_vfrstpi_b_14(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        15 => unsafe { rust_isa_lsx_vfrstpi_b_15(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        16 => unsafe { rust_isa_lsx_vfrstpi_b_16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        17 => unsafe { rust_isa_lsx_vfrstpi_b_17(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        18 => unsafe { rust_isa_lsx_vfrstpi_b_18(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        19 => unsafe { rust_isa_lsx_vfrstpi_b_19(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        20 => unsafe { rust_isa_lsx_vfrstpi_b_20(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        21 => unsafe { rust_isa_lsx_vfrstpi_b_21(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        22 => unsafe { rust_isa_lsx_vfrstpi_b_22(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        23 => unsafe { rust_isa_lsx_vfrstpi_b_23(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        24 => unsafe { rust_isa_lsx_vfrstpi_b_24(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        25 => unsafe { rust_isa_lsx_vfrstpi_b_25(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        26 => unsafe { rust_isa_lsx_vfrstpi_b_26(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        27 => unsafe { rust_isa_lsx_vfrstpi_b_27(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        28 => unsafe { rust_isa_lsx_vfrstpi_b_28(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        29 => unsafe { rust_isa_lsx_vfrstpi_b_29(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        30 => unsafe { rust_isa_lsx_vfrstpi_b_30(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        31 => unsafe { rust_isa_lsx_vfrstpi_b_31(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        _ => unreachable!(),
    }
    out
}

/// ``vfrstpi.h
#[inline(always)]
pub fn vfrstpi_h<const IMM: u32>(a: V128, b: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let mut out = V128::from_bytes([0; 16]);
    match IMM {
        0 => unsafe { rust_isa_lsx_vfrstpi_h_0(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        1 => unsafe { rust_isa_lsx_vfrstpi_h_1(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        2 => unsafe { rust_isa_lsx_vfrstpi_h_2(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        3 => unsafe { rust_isa_lsx_vfrstpi_h_3(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        4 => unsafe { rust_isa_lsx_vfrstpi_h_4(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        5 => unsafe { rust_isa_lsx_vfrstpi_h_5(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        6 => unsafe { rust_isa_lsx_vfrstpi_h_6(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        7 => unsafe { rust_isa_lsx_vfrstpi_h_7(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        8 => unsafe { rust_isa_lsx_vfrstpi_h_8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        9 => unsafe { rust_isa_lsx_vfrstpi_h_9(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        10 => unsafe { rust_isa_lsx_vfrstpi_h_10(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        11 => unsafe { rust_isa_lsx_vfrstpi_h_11(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        12 => unsafe { rust_isa_lsx_vfrstpi_h_12(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        13 => unsafe { rust_isa_lsx_vfrstpi_h_13(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        14 => unsafe { rust_isa_lsx_vfrstpi_h_14(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        15 => unsafe { rust_isa_lsx_vfrstpi_h_15(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        16 => unsafe { rust_isa_lsx_vfrstpi_h_16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        17 => unsafe { rust_isa_lsx_vfrstpi_h_17(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        18 => unsafe { rust_isa_lsx_vfrstpi_h_18(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        19 => unsafe { rust_isa_lsx_vfrstpi_h_19(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        20 => unsafe { rust_isa_lsx_vfrstpi_h_20(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        21 => unsafe { rust_isa_lsx_vfrstpi_h_21(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        22 => unsafe { rust_isa_lsx_vfrstpi_h_22(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        23 => unsafe { rust_isa_lsx_vfrstpi_h_23(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        24 => unsafe { rust_isa_lsx_vfrstpi_h_24(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        25 => unsafe { rust_isa_lsx_vfrstpi_h_25(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        26 => unsafe { rust_isa_lsx_vfrstpi_h_26(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        27 => unsafe { rust_isa_lsx_vfrstpi_h_27(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        28 => unsafe { rust_isa_lsx_vfrstpi_h_28(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        29 => unsafe { rust_isa_lsx_vfrstpi_h_29(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        30 => unsafe { rust_isa_lsx_vfrstpi_h_30(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        31 => unsafe { rust_isa_lsx_vfrstpi_h_31(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        _ => unreachable!(),
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfrstpi_b_0(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_1(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_2(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_3(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_4(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_5(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_6(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_7(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_9(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_10(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_11(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_12(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_13(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_14(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_15(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_17(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_18(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_19(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_20(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_21(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_22(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_23(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_24(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_25(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_26(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_27(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_28(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_29(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_30(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_b_31(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_0(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_1(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_2(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_3(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_4(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_5(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_6(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_7(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_9(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_10(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_11(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_12(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_13(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_14(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_15(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_17(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_18(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_19(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_20(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_21(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_22(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_23(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_24(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_25(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_26(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_27(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_28(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_29(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_30(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstpi_h_31(a: *const u8, b: *const u8, out: *mut u8);
}
