//! PowerPC VSX.
//!
//! VSX is not part of the PowerPC baseline. Every operation is `unsafe` and the
//! caller must enable VSX with a global target feature. The C file is compiled
//! with `-maltivec -mvsx`.
//!
//! The values are [`V128`]. The `f64` functions use two 64-bit float lanes and
//! the `f32` functions use four 32-bit float lanes.

use super::altivec::V128;

unsafe extern "C" {
    fn rust_isa_vsx_add_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_sub_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_mul_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_div_f64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_add_f32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_vsx_abs_f64(a: *const u8, out: *mut u8);
    fn rust_isa_vsx_sqrt_f64(a: *const u8, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// VSX 64-bit float add.
    add_f64,
    rust_isa_vsx_add_f64
);
binary!(
    /// VSX 64-bit float subtract.
    sub_f64,
    rust_isa_vsx_sub_f64
);
binary!(
    /// VSX 64-bit float multiply.
    mul_f64,
    rust_isa_vsx_mul_f64
);
binary!(
    /// VSX 64-bit float divide.
    div_f64,
    rust_isa_vsx_div_f64
);
binary!(
    /// VSX 32-bit float add.
    add_f32,
    rust_isa_vsx_add_f32
);

/// VSX 64-bit float absolute value.
///
/// # Safety
///
/// The caller must enable VSX.
#[inline(always)]
pub unsafe fn abs_f64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vsx_abs_f64(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// VSX 64-bit float square root.
///
/// # Safety
///
/// The caller must enable VSX.
#[inline(always)]
pub unsafe fn sqrt_f64(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_vsx_sqrt_f64(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
