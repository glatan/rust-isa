use super::V128;

// Lane, immediate, and memory instructions.

/// `vldrepl.b`
#[inline(always)]
pub unsafe fn vldrepl_b(p: *const u8, offset: i64) -> V128 {
    assert!(offset >= -2048 && offset <= 2047);
    let mut chunk = [0u8; 1];
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), chunk.as_mut_ptr(), 1);
    }
    let mut out = [0u8; 16];
    out.fill(chunk[0]);
    V128::from_bytes(out)
}

/// `vldrepl.h`
#[inline(always)]
pub unsafe fn vldrepl_h(p: *const u8, offset: i64) -> V128 {
    assert!(offset >= -1024 && offset <= 1023);
    let mut chunk = [0u8; 2];
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), chunk.as_mut_ptr(), 2);
    }
    let mut out = [0u8; 16];
    for out_chunk in out.as_chunks_mut::<2>().0.iter_mut() {
        *out_chunk = chunk;
    }
    V128::from_bytes(out)
}

/// `vldrepl.w`
#[inline(always)]
pub unsafe fn vldrepl_w(p: *const u8, offset: i64) -> V128 {
    assert!(offset >= -512 && offset <= 511);
    let mut chunk = [0u8; 4];
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), chunk.as_mut_ptr(), 4);
    }
    let mut out = [0u8; 16];
    for out_chunk in out.as_chunks_mut::<4>().0.iter_mut() {
        *out_chunk = chunk;
    }
    V128::from_bytes(out)
}

/// `vldrepl.d`
#[inline(always)]
pub unsafe fn vldrepl_d(p: *const u8, offset: i64) -> V128 {
    assert!(offset >= -256 && offset <= 255);
    let mut chunk = [0u8; 8];
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), chunk.as_mut_ptr(), 8);
    }
    let mut out = [0u8; 16];
    for out_chunk in out.as_chunks_mut::<8>().0.iter_mut() {
        *out_chunk = chunk;
    }
    V128::from_bytes(out)
}

/// `vstelm.b`
#[inline(always)]
pub unsafe fn vstelm_b<const INDEX: u32>(value: V128, p: *mut u8, offset: i64) {
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

/// `vstelm.h`
#[inline(always)]
pub unsafe fn vstelm_h<const INDEX: u32>(value: V128, p: *mut u8, offset: i64) {
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

/// `vstelm.w`
#[inline(always)]
pub unsafe fn vstelm_w<const INDEX: u32>(value: V128, p: *mut u8, offset: i64) {
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

/// `vstelm.d`
#[inline(always)]
pub unsafe fn vstelm_d<const INDEX: u32>(value: V128, p: *mut u8, offset: i64) {
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

/// `vreplvei.b`
#[inline(always)]
pub fn vreplvei_b<const INDEX: u32>(a: V128) -> V128 {
    const { assert!(INDEX < 16, "index out of range") };
    let bytes = a.to_bytes();
    let mut chunk = [0u8; 1];
    chunk.copy_from_slice(&bytes[INDEX as usize * 1..INDEX as usize * 1 + 1]);
    let mut out = [0u8; 16];
    out.fill(chunk[0]);
    V128::from_bytes(out)
}

/// `vreplvei.h`
#[inline(always)]
pub fn vreplvei_h<const INDEX: u32>(a: V128) -> V128 {
    const { assert!(INDEX < 8, "index out of range") };
    let bytes = a.to_bytes();
    let mut chunk = [0u8; 2];
    chunk.copy_from_slice(&bytes[INDEX as usize * 2..INDEX as usize * 2 + 2]);
    let mut out = [0u8; 16];
    for out_chunk in out.as_chunks_mut::<2>().0.iter_mut() {
        *out_chunk = chunk;
    }
    V128::from_bytes(out)
}

/// `vreplvei.w`
#[inline(always)]
pub fn vreplvei_w<const INDEX: u32>(a: V128) -> V128 {
    const { assert!(INDEX < 4, "index out of range") };
    let bytes = a.to_bytes();
    let mut chunk = [0u8; 4];
    chunk.copy_from_slice(&bytes[INDEX as usize * 4..INDEX as usize * 4 + 4]);
    let mut out = [0u8; 16];
    for out_chunk in out.as_chunks_mut::<4>().0.iter_mut() {
        *out_chunk = chunk;
    }
    V128::from_bytes(out)
}

/// `vreplvei.d`
#[inline(always)]
pub fn vreplvei_d<const INDEX: u32>(a: V128) -> V128 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let mut chunk = [0u8; 8];
    chunk.copy_from_slice(&bytes[INDEX as usize * 8..INDEX as usize * 8 + 8]);
    let mut out = [0u8; 16];
    for out_chunk in out.as_chunks_mut::<8>().0.iter_mut() {
        *out_chunk = chunk;
    }
    V128::from_bytes(out)
}

/// `vinsgr2vr.b`
#[inline(always)]
pub fn vinsgr2vr_b<const INDEX: u32>(a: V128, value: i32) -> V128 {
    const { assert!(INDEX < 16, "index out of range") };
    let mut out = a.to_bytes();
    out[INDEX as usize * 1..INDEX as usize * 1 + 1].copy_from_slice(&(value as u8).to_le_bytes());
    V128::from_bytes(out)
}

/// `vinsgr2vr.h`
#[inline(always)]
pub fn vinsgr2vr_h<const INDEX: u32>(a: V128, value: i32) -> V128 {
    const { assert!(INDEX < 8, "index out of range") };
    let mut out = a.to_bytes();
    out[INDEX as usize * 2..INDEX as usize * 2 + 2].copy_from_slice(&(value as u16).to_le_bytes());
    V128::from_bytes(out)
}

/// `vinsgr2vr.w`
#[inline(always)]
pub fn vinsgr2vr_w<const INDEX: u32>(a: V128, value: i32) -> V128 {
    const { assert!(INDEX < 4, "index out of range") };
    let mut out = a.to_bytes();
    out[INDEX as usize * 4..INDEX as usize * 4 + 4].copy_from_slice(&(value as u32).to_le_bytes());
    V128::from_bytes(out)
}

/// `vinsgr2vr.d`
#[inline(always)]
pub fn vinsgr2vr_d<const INDEX: u32>(a: V128, value: i64) -> V128 {
    const { assert!(INDEX < 2, "index out of range") };
    let mut out = a.to_bytes();
    out[INDEX as usize * 8..INDEX as usize * 8 + 8].copy_from_slice(&(value as u64).to_le_bytes());
    V128::from_bytes(out)
}

/// `vpickve2gr.b`
#[inline(always)]
pub fn vpickve2gr_b<const INDEX: u32>(a: V128) -> i32 {
    const { assert!(INDEX < 16, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 1;
    let value = i8::from_le_bytes(bytes[start..start + 1].try_into().unwrap());
    value as i32
}

/// `vpickve2gr.h`
#[inline(always)]
pub fn vpickve2gr_h<const INDEX: u32>(a: V128) -> i32 {
    const { assert!(INDEX < 8, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 2;
    let value = i16::from_le_bytes(bytes[start..start + 2].try_into().unwrap());
    value as i32
}

/// `vpickve2gr.w`
#[inline(always)]
pub fn vpickve2gr_w<const INDEX: u32>(a: V128) -> i32 {
    const { assert!(INDEX < 4, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 4;
    let value = i32::from_le_bytes(bytes[start..start + 4].try_into().unwrap());
    value as i32
}

/// `vpickve2gr.d`
#[inline(always)]
pub fn vpickve2gr_d<const INDEX: u32>(a: V128) -> i64 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 8;
    let value = i64::from_le_bytes(bytes[start..start + 8].try_into().unwrap());
    value as i64
}

/// `vpickve2gr.bu`
#[inline(always)]
pub fn vpickve2gr_bu<const INDEX: u32>(a: V128) -> u32 {
    const { assert!(INDEX < 16, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 1;
    let value = u8::from_le_bytes(bytes[start..start + 1].try_into().unwrap());
    value as u32
}

/// `vpickve2gr.hu`
#[inline(always)]
pub fn vpickve2gr_hu<const INDEX: u32>(a: V128) -> u32 {
    const { assert!(INDEX < 8, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 2;
    let value = u16::from_le_bytes(bytes[start..start + 2].try_into().unwrap());
    value as u32
}

/// `vpickve2gr.wu`
#[inline(always)]
pub fn vpickve2gr_wu<const INDEX: u32>(a: V128) -> u32 {
    const { assert!(INDEX < 4, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 4;
    let value = u32::from_le_bytes(bytes[start..start + 4].try_into().unwrap());
    value as u32
}

/// `vpickve2gr.du`
#[inline(always)]
pub fn vpickve2gr_du<const INDEX: u32>(a: V128) -> u64 {
    const { assert!(INDEX < 2, "index out of range") };
    let bytes = a.to_bytes();
    let start = INDEX as usize * 8;
    let value = u64::from_le_bytes(bytes[start..start + 8].try_into().unwrap());
    value as u64
}
