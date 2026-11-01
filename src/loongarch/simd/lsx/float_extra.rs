use super::V128;

// Float intrinsic wrappers.

/// `vfmaxa.s`
#[inline(always)]
pub fn vfmaxa_s(a0: V128, a1: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfmaxa_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmaxa.d`
#[inline(always)]
pub fn vfmaxa_d(a0: V128, a1: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfmaxa_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmina.s`
#[inline(always)]
pub fn vfmina_s(a0: V128, a1: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfmina_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfmina.d`
#[inline(always)]
pub fn vfmina_d(a0: V128, a1: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfmina_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vflogb.s`
#[inline(always)]
pub fn vflogb_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vflogb_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vflogb.d`
#[inline(always)]
pub fn vflogb_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vflogb_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfsqrt.s`
#[inline(always)]
pub fn vfsqrt_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfsqrt_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfsqrt.d`
#[inline(always)]
pub fn vfsqrt_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfsqrt_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrecip.s`
#[inline(always)]
pub fn vfrecip_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrecip_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrecip.d`
#[inline(always)]
pub fn vfrecip_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrecip_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrsqrt.s`
#[inline(always)]
pub fn vfrsqrt_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrsqrt_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrsqrt.d`
#[inline(always)]
pub fn vfrsqrt_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrsqrt_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrint.s`
#[inline(always)]
pub fn vfrint_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrint_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrint.d`
#[inline(always)]
pub fn vfrint_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrint_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrintrm.s`
#[inline(always)]
pub fn vfrintrm_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrintrm_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrintrm.d`
#[inline(always)]
pub fn vfrintrm_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrintrm_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrintrp.s`
#[inline(always)]
pub fn vfrintrp_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrintrp_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrintrp.d`
#[inline(always)]
pub fn vfrintrp_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrintrp_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrintrz.s`
#[inline(always)]
pub fn vfrintrz_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrintrz_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrintrz.d`
#[inline(always)]
pub fn vfrintrz_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrintrz_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrintrne.s`
#[inline(always)]
pub fn vfrintrne_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrintrne_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrintrne.d`
#[inline(always)]
pub fn vfrintrne_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrintrne_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfcvtl.s.h`
#[inline(always)]
pub fn vfcvtl_s_h(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfcvtl_s_h(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfcvth.s.h`
#[inline(always)]
pub fn vfcvth_s_h(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfcvth_s_h(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfrsqrte_d(a0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrsqrte_s(a0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrecipe_d(a0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrecipe_s(a0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmaxa_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmaxa_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmina_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmina_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lsx_vflogb_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vflogb_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfsqrt_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfsqrt_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrecip_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrecip_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrsqrt_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrsqrt_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrint_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrint_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrintrm_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrintrm_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrintrp_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrintrp_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrintrz_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrintrz_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrintrne_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrintrne_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfcvtl_s_h(p0: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfcvth_s_h(p0: *const u8, out: *mut u8);
}

/// `vfrecipe.s`
#[inline(always)]
pub fn vfrecipe_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrecipe_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrecipe.d`
#[inline(always)]
pub fn vfrecipe_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrecipe_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrsqrte.s`
#[inline(always)]
pub fn vfrsqrte_s(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrsqrte_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrsqrte.d`
#[inline(always)]
pub fn vfrsqrte_d(a0: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrsqrte_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}
