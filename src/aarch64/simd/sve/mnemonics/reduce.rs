//! SVE reduction instructions.

unsafe extern "C" {
    fn rust_isa_sve_addv_i8(a: *const i8, len: usize) -> i8;
}
/// Reduce `add` over a `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_addv_i8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_i16(a: *const i16, len: usize) -> i16;
}
/// Reduce `add` over a `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_addv_i16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_i64(a: *const i64, len: usize) -> i64;
}
/// Reduce `add` over a `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_addv_i64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_u8(a: *const u8, len: usize) -> u8;
}
/// Reduce `add` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_addv_u8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_u16(a: *const u16, len: usize) -> u16;
}
/// Reduce `add` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_addv_u16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_u32(a: *const u32, len: usize) -> u32;
}
/// Reduce `add` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_addv_u32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_addv_u64(a: *const u64, len: usize) -> u64;
}
/// Reduce `add` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_add_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_addv_u64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_i8(a: *const i8, len: usize) -> i8;
}
/// Reduce `max` over a `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_maxv_i8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_i8(a: *const i8, len: usize) -> i8;
}
/// Reduce `min` over a `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_minv_i8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_i16(a: *const i16, len: usize) -> i16;
}
/// Reduce `max` over a `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_maxv_i16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_i16(a: *const i16, len: usize) -> i16;
}
/// Reduce `min` over a `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_minv_i16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_i64(a: *const i64, len: usize) -> i64;
}
/// Reduce `max` over a `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_maxv_i64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_i64(a: *const i64, len: usize) -> i64;
}
/// Reduce `min` over a `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_minv_i64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_u8(a: *const u8, len: usize) -> u8;
}
/// Reduce `max` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_maxv_u8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_u8(a: *const u8, len: usize) -> u8;
}
/// Reduce `min` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_minv_u8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_u16(a: *const u16, len: usize) -> u16;
}
/// Reduce `max` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_maxv_u16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_u16(a: *const u16, len: usize) -> u16;
}
/// Reduce `min` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_minv_u16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_u32(a: *const u32, len: usize) -> u32;
}
/// Reduce `max` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_maxv_u32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_u32(a: *const u32, len: usize) -> u32;
}
/// Reduce `min` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_minv_u32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxv_u64(a: *const u64, len: usize) -> u64;
}
/// Reduce `max` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_max_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_maxv_u64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minv_u64(a: *const u64, len: usize) -> u64;
}
/// Reduce `min` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_min_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_minv_u64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_andv_i32(a: *const i32, len: usize) -> i32;
}
/// Reduce `and` over a `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_and_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_andv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_eorv_i32(a: *const i32, len: usize) -> i32;
}
/// Reduce `eor` over a `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_eor_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_eorv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxnmv_f32(a: *const f32, len: usize) -> f32;
}
/// Reduce `maxnm` over a `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_maxnm_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_maxnmv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minnmv_f32(a: *const f32, len: usize) -> f32;
}
/// Reduce `minnm` over a `f32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_minnm_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_minnmv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_maxnmv_f64(a: *const f64, len: usize) -> f64;
}
/// Reduce `maxnm` over a `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_maxnm_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_maxnmv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_minnmv_f64(a: *const f64, len: usize) -> f64;
}
/// Reduce `minnm` over a `f64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn reduce_minnm_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_minnmv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_faddv_f32(a: *const f32, len: usize) -> f32;
}
/// `faddv` over a `f32` slice (same semantics as `addv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn faddv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_faddv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxv_f32(a: *const f32, len: usize) -> f32;
}
/// `fmaxv` over a `f32` slice (same semantics as `maxv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fmaxv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fminv_f32(a: *const f32, len: usize) -> f32;
}
/// `fminv` over a `f32` slice (same semantics as `minv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fminv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxnmv_f32(a: *const f32, len: usize) -> f32;
}
/// `fmaxnmv` over a `f32` slice (same semantics as `maxnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnmv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fmaxnmv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fminnmv_f32(a: *const f32, len: usize) -> f32;
}
/// `fminnmv` over a `f32` slice (same semantics as `minnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnmv_f32(a: &[f32]) -> f32 {
    unsafe { rust_isa_sve_fminnmv_f32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_faddv_f64(a: *const f64, len: usize) -> f64;
}
/// `faddv` over a `f64` slice (same semantics as `addv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn faddv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_faddv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxv_f64(a: *const f64, len: usize) -> f64;
}
/// `fmaxv` over a `f64` slice (same semantics as `maxv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fmaxv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fminv_f64(a: *const f64, len: usize) -> f64;
}
/// `fminv` over a `f64` slice (same semantics as `minv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fminv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fmaxnmv_f64(a: *const f64, len: usize) -> f64;
}
/// `fmaxnmv` over a `f64` slice (same semantics as `maxnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fmaxnmv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fmaxnmv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fminnmv_f64(a: *const f64, len: usize) -> f64;
}
/// `fminnmv` over a `f64` slice (same semantics as `minnmv`).
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fminnmv_f64(a: &[f64]) -> f64 {
    unsafe { rust_isa_sve_fminnmv_f64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_orv_i8(a: *const i8, len: usize) -> i8;
}
/// `orv` over an `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn orv_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_orv_i8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_orv_i16(a: *const i16, len: usize) -> i16;
}
/// `orv` over an `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn orv_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_orv_i16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_orv_i32(a: *const i32, len: usize) -> i32;
}
/// `orv` over an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn orv_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_orv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_orv_i64(a: *const i64, len: usize) -> i64;
}
/// `orv` over an `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn orv_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_orv_i64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_orv_u8(a: *const u8, len: usize) -> u8;
}
/// `orv` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn orv_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_orv_u8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_orv_u16(a: *const u16, len: usize) -> u16;
}
/// `orv` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn orv_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_orv_u16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_orv_u32(a: *const u32, len: usize) -> u32;
}
/// `orv` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn orv_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_orv_u32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_orv_u64(a: *const u64, len: usize) -> u64;
}
/// `orv` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn orv_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_orv_u64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_saddv_i8(a: *const i8, len: usize) -> i64;
}
/// `saddv` over an `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saddv_i8(a: &[i8]) -> i64 {
    unsafe { rust_isa_sve_saddv_i8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_saddv_i16(a: *const i16, len: usize) -> i64;
}
/// `saddv` over an `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saddv_i16(a: &[i16]) -> i64 {
    unsafe { rust_isa_sve_saddv_i16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_saddv_i32(a: *const i32, len: usize) -> i64;
}
/// `saddv` over an `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saddv_i32(a: &[i32]) -> i64 {
    unsafe { rust_isa_sve_saddv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_saddv_i64(a: *const i64, len: usize) -> i64;
}
/// `saddv` over an `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn saddv_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_saddv_i64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_uaddv_u8(a: *const u8, len: usize) -> u64;
}
/// `uaddv` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaddv_u8(a: &[u8]) -> u64 {
    unsafe { rust_isa_sve_uaddv_u8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_uaddv_u16(a: *const u16, len: usize) -> u64;
}
/// `uaddv` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaddv_u16(a: &[u16]) -> u64 {
    unsafe { rust_isa_sve_uaddv_u16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_uaddv_u32(a: *const u32, len: usize) -> u64;
}
/// `uaddv` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaddv_u32(a: &[u32]) -> u64 {
    unsafe { rust_isa_sve_uaddv_u32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_uaddv_u64(a: *const u64, len: usize) -> u64;
}
/// `uaddv` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn uaddv_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_uaddv_u64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_andv_i8(a: *const i8, len: usize) -> i8;
}
/// `andv` over a `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn andv_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_andv_i8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_andv_i16(a: *const i16, len: usize) -> i16;
}
/// `andv` over a `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn andv_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_andv_i16(a.as_ptr(), a.len()) }
}

/// `andv` over a `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn andv_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_andv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_andv_i64(a: *const i64, len: usize) -> i64;
}
/// `andv` over a `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn andv_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_andv_i64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_andv_u8(a: *const u8, len: usize) -> u8;
}
/// `andv` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn andv_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_andv_u8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_andv_u16(a: *const u16, len: usize) -> u16;
}
/// `andv` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn andv_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_andv_u16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_andv_u32(a: *const u32, len: usize) -> u32;
}
/// `andv` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn andv_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_andv_u32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_andv_u64(a: *const u64, len: usize) -> u64;
}
/// `andv` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn andv_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_andv_u64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_eorv_i8(a: *const i8, len: usize) -> i8;
}
/// `eorv` over a `i8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn eorv_i8(a: &[i8]) -> i8 {
    unsafe { rust_isa_sve_eorv_i8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_eorv_i16(a: *const i16, len: usize) -> i16;
}
/// `eorv` over a `i16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn eorv_i16(a: &[i16]) -> i16 {
    unsafe { rust_isa_sve_eorv_i16(a.as_ptr(), a.len()) }
}

/// `eorv` over a `i32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn eorv_i32(a: &[i32]) -> i32 {
    unsafe { rust_isa_sve_eorv_i32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_eorv_i64(a: *const i64, len: usize) -> i64;
}
/// `eorv` over a `i64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn eorv_i64(a: &[i64]) -> i64 {
    unsafe { rust_isa_sve_eorv_i64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_eorv_u8(a: *const u8, len: usize) -> u8;
}
/// `eorv` over a `u8` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn eorv_u8(a: &[u8]) -> u8 {
    unsafe { rust_isa_sve_eorv_u8(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_eorv_u16(a: *const u16, len: usize) -> u16;
}
/// `eorv` over a `u16` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn eorv_u16(a: &[u16]) -> u16 {
    unsafe { rust_isa_sve_eorv_u16(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_eorv_u32(a: *const u32, len: usize) -> u32;
}
/// `eorv` over a `u32` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn eorv_u32(a: &[u32]) -> u32 {
    unsafe { rust_isa_sve_eorv_u32(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_eorv_u64(a: *const u64, len: usize) -> u64;
}
/// `eorv` over a `u64` slice.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn eorv_u64(a: &[u64]) -> u64 {
    unsafe { rust_isa_sve_eorv_u64(a.as_ptr(), a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fadda_f32(a: *const f32, acc: f32, len: usize) -> f32;
}
/// `fadda` over a `f32` slice with a scalar accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fadda_f32(a: &[f32], acc: f32) -> f32 {
    unsafe { rust_isa_sve_fadda_f32(a.as_ptr(), acc, a.len()) }
}

unsafe extern "C" {
    fn rust_isa_sve_fadda_f64(a: *const f64, acc: f64, len: usize) -> f64;
}
/// `fadda` over a `f64` slice with a scalar accumulator.
///
/// # Safety
///
/// The caller must enable SVE and the CPU must support it.
#[target_feature(enable = "sve")]
#[inline]
pub unsafe fn fadda_f64(a: &[f64], acc: f64) -> f64 {
    unsafe { rust_isa_sve_fadda_f64(a.as_ptr(), acc, a.len()) }
}
