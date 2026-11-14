//! SVE read-only loads and vector/predicate load-store instructions.

/// `ld1rob` scalar plus immediate form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rob_imm_u8(base: *const u8, offset: i64, out: &mut [u8]) {
    ld1ro_impl(base, offset, out);
}

/// `ld1rob` scalar plus scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rob_scalar_u8(base: *const u8, offset: i64, out: &mut [u8]) {
    ld1ro_impl(base, offset, out);
}

/// `ld1roh` scalar plus immediate form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1roh_imm_u16(base: *const u16, offset: i64, out: &mut [u16]) {
    ld1ro_impl(base, offset, out);
}

/// `ld1roh` scalar plus scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1roh_scalar_u16(base: *const u16, offset: i64, out: &mut [u16]) {
    ld1ro_impl(base, offset, out);
}

/// `ld1row` scalar plus immediate form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1row_imm_u32(base: *const u32, offset: i64, out: &mut [u32]) {
    ld1ro_impl(base, offset, out);
}

/// `ld1row` scalar plus scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1row_scalar_u32(base: *const u32, offset: i64, out: &mut [u32]) {
    ld1ro_impl(base, offset, out);
}

/// `ld1rod` scalar plus immediate form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rod_imm_u64(base: *const u64, offset: i64, out: &mut [u64]) {
    ld1ro_impl(base, offset, out);
}

/// `ld1rod` scalar plus scalar form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ld1rod_scalar_u64(base: *const u64, offset: i64, out: &mut [u64]) {
    ld1ro_impl(base, offset, out);
}

#[inline(always)]
unsafe fn ld1ro_impl<T: Copy>(base: *const T, offset: i64, out: &mut [T]) {
    for (lane, elem) in out.iter_mut().enumerate() {
        *elem = unsafe { *base.offset(offset as isize + lane as isize) };
    }
}

/// `ldr` predicate form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldr_pred(mem: *const u8) -> u64 {
    let mut bytes = [0u8; 8];
    unsafe {
        core::ptr::copy_nonoverlapping(mem, bytes.as_mut_ptr(), 8);
    }
    u64::from_le_bytes(bytes)
}

/// `str` predicate form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be writable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn str_pred(mem: *mut u8, value: u64) {
    unsafe {
        core::ptr::copy_nonoverlapping(value.to_le_bytes().as_ptr(), mem, 8);
    }
}

/// `ldr` vector form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be readable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn ldr_vector(mem: *const u8, out: &mut [u8]) {
    unsafe {
        core::ptr::copy_nonoverlapping(mem, out.as_mut_ptr(), out.len());
    }
}

/// `str` vector form.
///
/// # Safety
///
/// The caller must enable SVE and the range must be writable.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn str_vector(mem: *mut u8, value: &[u8]) {
    unsafe {
        core::ptr::copy_nonoverlapping(value.as_ptr(), mem, value.len());
    }
}

macro_rules! nonfault_load {
    ($(#[$meta:meta])* $name:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable SVE and the range must be readable.
        #[target_feature(enable = "sve")]
        #[inline]
        pub unsafe fn $name(base: *const $ty, out: &mut [i32]) {
            for (lane, elem) in out.iter_mut().enumerate() {
                *elem = unsafe { *base.add(lane) } as i32;}
        }
    };
}

nonfault_load!(
    /// `ldnf1sb`
    ldnf1sb_i32,
    i8
);
nonfault_load!(
    /// `ldnf1sh`
    ldnf1sh_i32,
    i16
);
nonfault_load!(
    /// `ldnf1sw`
    ldnf1sw_i32,
    i32
);
nonfault_load!(
    /// `ldnt1sb`
    ldnt1sb_i32,
    i8
);
nonfault_load!(
    /// `ldnt1sh`
    ldnt1sh_i32,
    i16
);
nonfault_load!(
    /// `ldnt1sw`
    ldnt1sw_i32,
    i32
);
