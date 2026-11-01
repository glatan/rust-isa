use super::V256;
use crate::loongarch::simd::lsx::V128;

// Lane, immediate, and memory instructions.

/// `xvldrepl.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvldrepl_b(p: *const u8, offset: i64) -> V256 {
    assert!(offset >= -2048 && offset <= 2047);
    let mut chunk = [0u8; 1];
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), chunk.as_mut_ptr(), 1);
    }
    let mut out = [0u8; 32];
    for lane in 0..32 {
        out[lane * 1..lane * 1 + 1].copy_from_slice(&chunk);
    }
    V256::from_bytes(out)
}

/// `xvldrepl.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvldrepl_h(p: *const u8, offset: i64) -> V256 {
    assert!(offset >= -1024 && offset <= 1023);
    let mut chunk = [0u8; 2];
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), chunk.as_mut_ptr(), 2);
    }
    let mut out = [0u8; 32];
    for lane in 0..16 {
        out[lane * 2..lane * 2 + 2].copy_from_slice(&chunk);
    }
    V256::from_bytes(out)
}

/// `xvldrepl.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvldrepl_w(p: *const u8, offset: i64) -> V256 {
    assert!(offset >= -512 && offset <= 511);
    let mut chunk = [0u8; 4];
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), chunk.as_mut_ptr(), 4);
    }
    let mut out = [0u8; 32];
    for lane in 0..8 {
        out[lane * 4..lane * 4 + 4].copy_from_slice(&chunk);
    }
    V256::from_bytes(out)
}

/// `xvldrepl.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvldrepl_d(p: *const u8, offset: i64) -> V256 {
    assert!(offset >= -256 && offset <= 255);
    let mut chunk = [0u8; 8];
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), chunk.as_mut_ptr(), 8);
    }
    let mut out = [0u8; 32];
    for lane in 0..4 {
        out[lane * 8..lane * 8 + 8].copy_from_slice(&chunk);
    }
    V256::from_bytes(out)
}

/// `xvstelm.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvstelm_b<const INDEX: u32>(value: V256, p: *mut u8, offset: i64) {
    const { assert!(INDEX < 16, "index out of range") };
    assert!(offset >= -128 && offset <= 127);
    let bytes = value.to_bytes();
    unsafe {
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(INDEX as usize * 1),
            p.offset(offset as isize),
            1,
        );
    }
}

/// `xvstelm.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvstelm_h<const INDEX: u32>(value: V256, p: *mut u8, offset: i64) {
    const { assert!(INDEX < 8, "index out of range") };
    assert!(offset >= -128 && offset <= 127);
    let bytes = value.to_bytes();
    unsafe {
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(INDEX as usize * 2),
            p.offset(offset as isize),
            2,
        );
    }
}

/// `xvstelm.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvstelm_w<const INDEX: u32>(value: V256, p: *mut u8, offset: i64) {
    const { assert!(INDEX < 4, "index out of range") };
    assert!(offset >= -128 && offset <= 127);
    let bytes = value.to_bytes();
    unsafe {
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(INDEX as usize * 4),
            p.offset(offset as isize),
            4,
        );
    }
}

/// `xvstelm.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvstelm_d<const INDEX: u32>(value: V256, p: *mut u8, offset: i64) {
    const { assert!(INDEX < 2, "index out of range") };
    assert!(offset >= -128 && offset <= 127);
    let bytes = value.to_bytes();
    unsafe {
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(INDEX as usize * 8),
            p.offset(offset as isize),
            8,
        );
    }
}

/// `xvrepl128vei.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrepl128vei_b<const INDEX: u32>(a: V256) -> V256 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let start = INDEX as usize * 16;
    out[0..16].copy_from_slice(&bytes[start..start + 16]);
    out[16..32].copy_from_slice(&bytes[start..start + 16]);
    V256::from_bytes(out)
}

/// `xvrepl128vei.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrepl128vei_h<const INDEX: u32>(a: V256) -> V256 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let start = INDEX as usize * 16;
    out[0..16].copy_from_slice(&bytes[start..start + 16]);
    out[16..32].copy_from_slice(&bytes[start..start + 16]);
    V256::from_bytes(out)
}

/// `xvrepl128vei.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrepl128vei_w<const INDEX: u32>(a: V256) -> V256 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let start = INDEX as usize * 16;
    out[0..16].copy_from_slice(&bytes[start..start + 16]);
    out[16..32].copy_from_slice(&bytes[start..start + 16]);
    V256::from_bytes(out)
}

/// `xvrepl128vei.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrepl128vei_d<const INDEX: u32>(a: V256) -> V256 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let start = INDEX as usize * 16;
    out[0..16].copy_from_slice(&bytes[start..start + 16]);
    out[16..32].copy_from_slice(&bytes[start..start + 16]);
    V256::from_bytes(out)
}

/// `xvinsgr2vr.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvinsgr2vr_w<const INDEX: u32>(a: V256, value: i32) -> V256 {
    const { assert!(INDEX < 8, "index out of range") };
    let mut out = a.to_bytes();
    out[INDEX as usize * 4..INDEX as usize * 4 + 4].copy_from_slice(&(value as u32).to_le_bytes());
    V256::from_bytes(out)
}

/// `xvinsgr2vr.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvinsgr2vr_d<const INDEX: u32>(a: V256, value: i64) -> V256 {
    const { assert!(INDEX < 4, "index out of range") };
    let mut out = a.to_bytes();
    out[INDEX as usize * 8..INDEX as usize * 8 + 8].copy_from_slice(&(value as u64).to_le_bytes());
    V256::from_bytes(out)
}

/// `xvinsve0.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvinsve0_w<const INDEX: u32>(a: V256, value: V128) -> V256 {
    const { assert!(INDEX < 2, "index out of range") };
    let mut out = a.to_bytes();
    let src = value.to_bytes();
    let start = INDEX as usize * 16;
    out[start..start + 16].copy_from_slice(&src);
    V256::from_bytes(out)
}

/// `xvinsve0.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvinsve0_d<const INDEX: u32>(a: V256, value: V128) -> V256 {
    const { assert!(INDEX < 2, "index out of range") };
    let mut out = a.to_bytes();
    let src = value.to_bytes();
    let start = INDEX as usize * 16;
    out[start..start + 16].copy_from_slice(&src);
    V256::from_bytes(out)
}

/// `xvpickve.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickve_w<const INDEX: u32>(a: V256) -> V128 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 16;
    let mut out = [0u8; 16];
    out.copy_from_slice(&bytes[start..start + 16]);
    V128::from_bytes(out)
}

/// `xvpickve.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickve_d<const INDEX: u32>(a: V256) -> V128 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 16;
    let mut out = [0u8; 16];
    out.copy_from_slice(&bytes[start..start + 16]);
    V128::from_bytes(out)
}

/// `xvpickve2gr.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickve2gr_w<const INDEX: u32>(a: V256) -> i32 {
    const { assert!(INDEX < 8, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 4;
    let value = i32::from_le_bytes(bytes[start..start + 4].try_into().unwrap());
    value as i32
}

/// `xvpickve2gr.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickve2gr_d<const INDEX: u32>(a: V256) -> i64 {
    const { assert!(INDEX < 4, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 8;
    let value = i64::from_le_bytes(bytes[start..start + 8].try_into().unwrap());
    value as i64
}

/// `xvpickve2gr.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickve2gr_wu<const INDEX: u32>(a: V256) -> u32 {
    const { assert!(INDEX < 8, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 4;
    let value = u32::from_le_bytes(bytes[start..start + 4].try_into().unwrap());
    value as u32
}

/// `xvpickve2gr.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpickve2gr_du<const INDEX: u32>(a: V256) -> u64 {
    const { assert!(INDEX < 4, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 8;
    let value = u64::from_le_bytes(bytes[start..start + 8].try_into().unwrap());
    value as u64
}
