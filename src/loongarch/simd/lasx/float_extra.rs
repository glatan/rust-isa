use super::V256;

// Float intrinsic wrappers.

/// `xvfadd.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfadd_s(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfadd_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfadd.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfadd_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfadd_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfsub.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfsub_s(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfsub_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfsub.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfsub_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfsub_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmul.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmul_s(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmul_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmul.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmul_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmul_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfdiv.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfdiv_s(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfdiv_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfdiv.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfdiv_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfdiv_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmax.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmax_s(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmax_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmax.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmax_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmax_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmin.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmin_s(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmin_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmin.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmin_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmin_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmaxa.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmaxa_s(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmaxa_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmaxa.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmaxa_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmaxa_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmina.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmina_s(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmina_s(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfmina.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfmina_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfmina_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfcvt.s.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcvt_s_d(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcvt_s_d(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffint.s.l`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffint_s_l(a0: V256, a1: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvffint_s_l(a0.as_ptr(), a1.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvflogb.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvflogb_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvflogb_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvflogb.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvflogb_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvflogb_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfsqrt.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfsqrt_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfsqrt_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfsqrt.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfsqrt_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfsqrt_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrecip.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrecip_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrecip_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrecip.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrecip_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrecip_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrsqrt.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrsqrt_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrsqrt_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrsqrt.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrsqrt_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrsqrt_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrint.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrint_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrint_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrint.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrint_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrint_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrintrm.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrintrm_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrintrm_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrintrm.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrintrm_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrintrm_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrintrp.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrintrp_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrintrp_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrintrp.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrintrp_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrintrp_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrintrz.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrintrz_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrintrz_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrintrz.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrintrz_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrintrz_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrintrne.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrintrne_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrintrne_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrintrne.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrintrne_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrintrne_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfcvtl.s.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcvtl_s_h(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcvtl_s_h(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfcvth.s.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcvth_s_h(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcvth_s_h(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfcvtl.d.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcvtl_d_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcvtl_d_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfcvth.d.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfcvth_d_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfcvth_d_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffint.s.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffint_s_w(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvffint_s_w(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffint.s.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffint_s_wu(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvffint_s_wu(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffint.d.l`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffint_d_l(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvffint_d_l(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffint.d.lu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffint_d_lu(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvffint_d_lu(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffintl.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffintl_d_w(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvffintl_d_w(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvffinth.d.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvffinth_d_w(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvffinth_d_w(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lasx_xvfrsqrte_d(a0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrsqrte_s(a0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrecipe_d(a0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrecipe_s(a0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfadd_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfadd_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfsub_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfsub_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmul_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmul_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfdiv_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfdiv_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmax_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmax_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmin_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmin_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmaxa_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmaxa_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmina_s(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfmina_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfcvt_s_d(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvffint_s_l(p0: *const u8, p1: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvflogb_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvflogb_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfsqrt_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfsqrt_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrecip_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrecip_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrsqrt_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrsqrt_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrint_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrint_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrintrm_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrintrm_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrintrp_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrintrp_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrintrz_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrintrz_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrintrne_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfrintrne_d(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfcvtl_s_h(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfcvth_s_h(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfcvtl_d_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvfcvth_d_s(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvffint_s_w(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvffint_s_wu(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvffint_d_l(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvffint_d_lu(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvffintl_d_w(p0: *const u8, out: *mut u8);
    fn rust_isa_lasx_xvffinth_d_w(p0: *const u8, out: *mut u8);
}

/// `xvfrecipe.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrecipe_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrecipe_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrecipe.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrecipe_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrecipe_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrsqrte.s`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrsqrte_s(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrsqrte_s(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `xvfrsqrte.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvfrsqrte_d(a0: V256) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_lasx_xvfrsqrte_d(a0.as_ptr(), out.as_mut_ptr());
    }
    out
}
