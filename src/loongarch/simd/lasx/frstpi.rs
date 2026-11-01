//! LASX first-set-position immediate instructions.

use super::V256;

/// `xvfrstpi.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrstpi_b<const IMM: u32>(a: V256, b: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let mut out = V256::from_bytes([0; 32]);
    match IMM {
        0 => unsafe { rust_isa_lasx_xvfrstpi_b_0(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        1 => unsafe { rust_isa_lasx_xvfrstpi_b_1(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        2 => unsafe { rust_isa_lasx_xvfrstpi_b_2(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        3 => unsafe { rust_isa_lasx_xvfrstpi_b_3(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        4 => unsafe { rust_isa_lasx_xvfrstpi_b_4(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        5 => unsafe { rust_isa_lasx_xvfrstpi_b_5(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        6 => unsafe { rust_isa_lasx_xvfrstpi_b_6(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        7 => unsafe { rust_isa_lasx_xvfrstpi_b_7(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        8 => unsafe { rust_isa_lasx_xvfrstpi_b_8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        9 => unsafe { rust_isa_lasx_xvfrstpi_b_9(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        10 => unsafe { rust_isa_lasx_xvfrstpi_b_10(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        11 => unsafe { rust_isa_lasx_xvfrstpi_b_11(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        12 => unsafe { rust_isa_lasx_xvfrstpi_b_12(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        13 => unsafe { rust_isa_lasx_xvfrstpi_b_13(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        14 => unsafe { rust_isa_lasx_xvfrstpi_b_14(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        15 => unsafe { rust_isa_lasx_xvfrstpi_b_15(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        16 => unsafe { rust_isa_lasx_xvfrstpi_b_16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        17 => unsafe { rust_isa_lasx_xvfrstpi_b_17(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        18 => unsafe { rust_isa_lasx_xvfrstpi_b_18(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        19 => unsafe { rust_isa_lasx_xvfrstpi_b_19(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        20 => unsafe { rust_isa_lasx_xvfrstpi_b_20(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        21 => unsafe { rust_isa_lasx_xvfrstpi_b_21(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        22 => unsafe { rust_isa_lasx_xvfrstpi_b_22(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        23 => unsafe { rust_isa_lasx_xvfrstpi_b_23(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        24 => unsafe { rust_isa_lasx_xvfrstpi_b_24(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        25 => unsafe { rust_isa_lasx_xvfrstpi_b_25(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        26 => unsafe { rust_isa_lasx_xvfrstpi_b_26(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        27 => unsafe { rust_isa_lasx_xvfrstpi_b_27(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        28 => unsafe { rust_isa_lasx_xvfrstpi_b_28(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        29 => unsafe { rust_isa_lasx_xvfrstpi_b_29(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        30 => unsafe { rust_isa_lasx_xvfrstpi_b_30(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        31 => unsafe { rust_isa_lasx_xvfrstpi_b_31(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        _ => unreachable!(),
    }
    out
}

/// `xvfrstpi.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrstpi_h<const IMM: u32>(a: V256, b: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let mut out = V256::from_bytes([0; 32]);
    match IMM {
        0 => unsafe { rust_isa_lasx_xvfrstpi_h_0(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        1 => unsafe { rust_isa_lasx_xvfrstpi_h_1(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        2 => unsafe { rust_isa_lasx_xvfrstpi_h_2(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        3 => unsafe { rust_isa_lasx_xvfrstpi_h_3(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        4 => unsafe { rust_isa_lasx_xvfrstpi_h_4(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        5 => unsafe { rust_isa_lasx_xvfrstpi_h_5(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        6 => unsafe { rust_isa_lasx_xvfrstpi_h_6(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        7 => unsafe { rust_isa_lasx_xvfrstpi_h_7(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        8 => unsafe { rust_isa_lasx_xvfrstpi_h_8(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        9 => unsafe { rust_isa_lasx_xvfrstpi_h_9(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        10 => unsafe { rust_isa_lasx_xvfrstpi_h_10(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        11 => unsafe { rust_isa_lasx_xvfrstpi_h_11(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        12 => unsafe { rust_isa_lasx_xvfrstpi_h_12(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        13 => unsafe { rust_isa_lasx_xvfrstpi_h_13(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        14 => unsafe { rust_isa_lasx_xvfrstpi_h_14(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        15 => unsafe { rust_isa_lasx_xvfrstpi_h_15(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        16 => unsafe { rust_isa_lasx_xvfrstpi_h_16(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        17 => unsafe { rust_isa_lasx_xvfrstpi_h_17(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        18 => unsafe { rust_isa_lasx_xvfrstpi_h_18(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        19 => unsafe { rust_isa_lasx_xvfrstpi_h_19(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        20 => unsafe { rust_isa_lasx_xvfrstpi_h_20(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        21 => unsafe { rust_isa_lasx_xvfrstpi_h_21(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        22 => unsafe { rust_isa_lasx_xvfrstpi_h_22(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        23 => unsafe { rust_isa_lasx_xvfrstpi_h_23(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        24 => unsafe { rust_isa_lasx_xvfrstpi_h_24(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        25 => unsafe { rust_isa_lasx_xvfrstpi_h_25(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        26 => unsafe { rust_isa_lasx_xvfrstpi_h_26(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        27 => unsafe { rust_isa_lasx_xvfrstpi_h_27(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        28 => unsafe { rust_isa_lasx_xvfrstpi_h_28(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        29 => unsafe { rust_isa_lasx_xvfrstpi_h_29(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        30 => unsafe { rust_isa_lasx_xvfrstpi_h_30(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        31 => unsafe { rust_isa_lasx_xvfrstpi_h_31(a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) },
        _ => unreachable!(),
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfrstpi_b_0(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_1(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_2(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_3(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_4(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_5(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_6(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_7(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_9(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_10(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_11(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_12(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_13(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_14(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_15(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_17(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_18(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_19(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_20(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_21(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_22(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_23(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_24(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_25(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_26(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_27(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_28(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_29(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_30(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_b_31(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_0(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_1(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_2(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_3(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_4(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_5(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_6(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_7(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_9(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_10(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_11(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_12(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_13(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_14(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_15(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_17(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_18(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_19(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_20(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_21(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_22(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_23(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_24(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_25(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_26(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_27(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_28(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_29(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_30(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrstpi_h_31(a: *const u8, b: *const u8, out: *mut u8);
}
