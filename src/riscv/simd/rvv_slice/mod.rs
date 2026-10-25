//! RISC-V V slice API.
//!
//! The V extension is unstable. The caller must compile the whole crate with
//! `-C target-feature=+v`; the wrappers do not carry `#[target_feature]`.
//! The slice operations are `unsafe`.
//!
//! SVE-like runtime vector lengths do not fit
//! [`V128`](crate::riscv::simd::rvv::V128). These functions take
//! slices and let the C loop choose VL with `vsetvl`. The slices use the element
//! type in each function name. All slices must have the same length; a different
//! length panics. The functions do not allocate.
//!
//! The fixed-length [`V128`](crate::riscv::simd::rvv::V128) API is
//! separate from this slice API.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! # Examples
//!
//! ```
//! use rust_isa::riscv::simd::rvv_slice::add_f32;
//!
//! let a = [1.0f32, 2.0];
//! let b = [10.0f32, 20.0];
//! let mut out = [0.0f32; 2];
//! unsafe { add_f32(&a, &b, &mut out) };
//! assert_eq!(out, [11.0, 22.0]);
//! ```
//!
//! <details>
//! <summary>Compile-only instruction examples</summary>
//!
//! # Compile-only instruction examples
//!
//! ```no_run
//! use rust_isa::riscv::simd::rvv_slice::*;
//! use rust_isa::riscv::simd::rvv::V128;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = unsafe { add_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0.0f32; 4]) };
//! let _ = unsafe { add_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { cmpeq_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0; 4]) };
//! let _ = unsafe { cmpeq_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { cmpge_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0; 4]) };
//! let _ = unsafe { cmpge_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { cmpgt_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0; 4]) };
//! let _ = unsafe { cmpgt_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { cmple_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0; 4]) };
//! let _ = unsafe { cmple_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { cmplt_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0; 4]) };
//! let _ = unsafe { cmplt_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { cmpne_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0; 4]) };
//! let _ = unsafe { cmpne_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { div_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0.0f32; 4]) };
//! let _ = unsafe { div_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { load_f32(storage.0.as_ptr().cast(), &mut[0.0f32; 4]) };
//! let _ = unsafe { load_i32(storage.0.as_ptr().cast(), &mut[0; 4]) };
//! let _ = unsafe { max_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0.0f32; 4]) };
//! let _ = unsafe { max_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { min_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0.0f32; 4]) };
//! let _ = unsafe { min_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { mul_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0.0f32; 4]) };
//! let _ = unsafe { mul_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! let _ = unsafe { store_f32(storage.0.as_mut_ptr().cast(), &[0.0f32; 4]) };
//! let _ = unsafe { store_i32(storage.0.as_mut_ptr().cast(), &[0; 4]) };
//! let _ = unsafe { sub_f32(&[0.0f32; 4], &[0.0f32; 4], &mut[0.0f32; 4]) };
//! let _ = unsafe { sub_i32(&[0; 4], &[0; 4], &mut[0; 4]) };
//! ```
//!
//! </details>
//!
//! <details>
//! <summary>Instruction inventory</summary>
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::riscv::simd::rvv_slice::add_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::add_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmpeq_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmpeq_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmpge_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmpge_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmpgt_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmpgt_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmple_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmple_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmplt_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmplt_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmpne_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::cmpne_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::div_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::div_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::load_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::load_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::max_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::max_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::min_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::min_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::mul_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::mul_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::store_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::store_i32 as _;
//! use rust_isa::riscv::simd::rvv_slice::sub_f32 as _;
//! use rust_isa::riscv::simd::rvv_slice::sub_i32 as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

unsafe extern "C" {
    fn rust_isa_rvv_slice_add_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_mul_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_sub_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_div_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_min_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_max_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_add_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_mul_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_sub_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_div_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_min_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_max_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmpeq_f32(a: *const f32, b: *const f32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmpne_f32(a: *const f32, b: *const f32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmplt_f32(a: *const f32, b: *const f32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmple_f32(a: *const f32, b: *const f32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmpgt_f32(a: *const f32, b: *const f32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmpge_f32(a: *const f32, b: *const f32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmpeq_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmpne_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmplt_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmple_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmpgt_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_cmpge_i32(a: *const i32, b: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_load_f32(src: *const f32, out: *mut f32, len: usize);
    fn rust_isa_rvv_slice_load_i32(src: *const i32, out: *mut i32, len: usize);
    fn rust_isa_rvv_slice_store_f32(dst: *mut f32, src: *const f32, len: usize);
    fn rust_isa_rvv_slice_store_i32(dst: *mut i32, src: *const i32, len: usize);
}

/// Add two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn add_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
    assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
    unsafe {
        rust_isa_rvv_slice_add_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `f32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn mul_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
    assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
    unsafe {
        rust_isa_rvv_slice_mul_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Add two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn add_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
    assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
    unsafe {
        rust_isa_rvv_slice_add_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

/// Multiply two `i32` slices.
///
/// # Panics
///
/// Panics when the slice lengths differ.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn mul_i32(a: &[i32], b: &[i32], out: &mut [i32]) {
    assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
    assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
    unsafe {
        rust_isa_rvv_slice_mul_i32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
    }
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Panics
        ///
        /// Panics when the slice lengths differ.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: &[$ty], b: &[$ty], out: &mut [$ty]) {
            assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
            assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
            }
        }
    };
}

macro_rules! compare {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// A true lane is all bits one.
        ///
        /// # Panics
        ///
        /// Panics when the slice lengths differ.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: &[$ty], b: &[$ty], out: &mut [i32]) {
            assert_eq!(a.len(), b.len(), "RVV slices must have the same length");
            assert_eq!(a.len(), out.len(), "RVV slices must have the same length");
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len());
            }
        }
    };
}

binary!(
    /// `vfsub.vv` on `f32` lanes.
    sub_f32,
    rust_isa_rvv_slice_sub_f32,
    f32
);
binary!(
    /// `vfdiv.vv` on `f32` lanes.
    div_f32,
    rust_isa_rvv_slice_div_f32,
    f32
);
binary!(
    /// `vfmin.vv` on `f32` lanes.
    min_f32,
    rust_isa_rvv_slice_min_f32,
    f32
);
binary!(
    /// `vfmax.vv` on `f32` lanes.
    max_f32,
    rust_isa_rvv_slice_max_f32,
    f32
);
binary!(
    /// `vsub.vv` on `i32` lanes.
    sub_i32,
    rust_isa_rvv_slice_sub_i32,
    i32
);
binary!(
    /// `vdiv.vv` on `i32` lanes.
    div_i32,
    rust_isa_rvv_slice_div_i32,
    i32
);
binary!(
    /// `vmin.vv` on signed `i32` lanes.
    min_i32,
    rust_isa_rvv_slice_min_i32,
    i32
);
binary!(
    /// `vmax.vv` on signed `i32` lanes.
    max_i32,
    rust_isa_rvv_slice_max_i32,
    i32
);

compare!(
    /// `vmfeq.vv` on `f32` lanes.
    cmpeq_f32,
    rust_isa_rvv_slice_cmpeq_f32,
    f32
);
compare!(
    /// `vmfne.vv` on `f32` lanes. Ordered; a NaN lane compares false.
    cmpne_f32,
    rust_isa_rvv_slice_cmpne_f32,
    f32
);
compare!(
    /// `vmflt.vv` on `f32` lanes.
    cmplt_f32,
    rust_isa_rvv_slice_cmplt_f32,
    f32
);
compare!(
    /// `vmfle.vv` on `f32` lanes.
    cmple_f32,
    rust_isa_rvv_slice_cmple_f32,
    f32
);
compare!(
    /// `vmfgt.vv` on `f32` lanes.
    cmpgt_f32,
    rust_isa_rvv_slice_cmpgt_f32,
    f32
);
compare!(
    /// `vmfge.vv` on `f32` lanes.
    cmpge_f32,
    rust_isa_rvv_slice_cmpge_f32,
    f32
);
compare!(
    /// `vmseq.vv` on `i32` lanes.
    cmpeq_i32,
    rust_isa_rvv_slice_cmpeq_i32,
    i32
);
compare!(
    /// `vmsne.vv` on `i32` lanes.
    cmpne_i32,
    rust_isa_rvv_slice_cmpne_i32,
    i32
);
compare!(
    /// `vmslt.vv` on signed `i32` lanes.
    cmplt_i32,
    rust_isa_rvv_slice_cmplt_i32,
    i32
);
compare!(
    /// `vmsle.vv` on signed `i32` lanes.
    cmple_i32,
    rust_isa_rvv_slice_cmple_i32,
    i32
);
compare!(
    /// `vmsgt.vv` on signed `i32` lanes.
    cmpgt_i32,
    rust_isa_rvv_slice_cmpgt_i32,
    i32
);
compare!(
    /// `vmsge.vv` on signed `i32` lanes.
    cmpge_i32,
    rust_isa_rvv_slice_cmpge_i32,
    i32
);

/// `vle32.v` and `vse32.v` on `f32` lanes.
///
/// Copy `out.len()` elements from `src` into `out` with vector loads and
/// stores.
///
/// # Safety
///
/// The caller must compile with the V extension enabled. `src` must be valid
/// for reading `out.len()` elements.
#[inline(always)]
pub unsafe fn load_f32(src: *const f32, out: &mut [f32]) {
    unsafe {
        rust_isa_rvv_slice_load_f32(src, out.as_mut_ptr(), out.len());
    }
}

/// `vle32.v` and `vse32.v` on `i32` lanes.
///
/// Copy `out.len()` elements from `src` into `out` with vector loads and
/// stores.
///
/// # Safety
///
/// The caller must compile with the V extension enabled. `src` must be valid
/// for reading `out.len()` elements.
#[inline(always)]
pub unsafe fn load_i32(src: *const i32, out: &mut [i32]) {
    unsafe {
        rust_isa_rvv_slice_load_i32(src, out.as_mut_ptr(), out.len());
    }
}

/// `vle32.v` and `vse32.v` on `f32` lanes.
///
/// Copy `src` into `dst` with vector loads and stores.
///
/// # Safety
///
/// The caller must compile with the V extension enabled. `dst` must be valid
/// for writing `src.len()` elements.
#[inline(always)]
pub unsafe fn store_f32(dst: *mut f32, src: &[f32]) {
    unsafe {
        rust_isa_rvv_slice_store_f32(dst, src.as_ptr(), src.len());
    }
}

/// `vle32.v` and `vse32.v` on `i32` lanes.
///
/// Copy `src` into `dst` with vector loads and stores.
///
/// # Safety
///
/// The caller must compile with the V extension enabled. `dst` must be valid
/// for writing `src.len()` elements.
#[inline(always)]
pub unsafe fn store_i32(dst: *mut i32, src: &[i32]) {
    unsafe {
        rust_isa_rvv_slice_store_i32(dst, src.as_ptr(), src.len());
    }
}
