//! NEON structure load and store instructions.
//!
//! Every function takes a raw pointer and is `unsafe`. The pointer must be
//! valid for the number of bytes the instruction reads or writes, and it must
//! satisfy the alignment required by the instruction: 16 bytes for the `q`
//! forms, 8 bytes for the `r` forms, and 16 bytes for a pair.

use super::V128;

unsafe extern "C" {
    fn rust_isa_neon_ld2(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_neon_ld3(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8);
    fn rust_isa_neon_ld4(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8, o3: *mut u8);
    fn rust_isa_neon_st2(p: *mut u8, v0: *const u8, v1: *const u8);
    fn rust_isa_neon_st3(p: *mut u8, v0: *const u8, v1: *const u8, v2: *const u8);
    fn rust_isa_neon_st4(p: *mut u8, v0: *const u8, v1: *const u8, v2: *const u8, v3: *const u8);
    fn rust_isa_neon_ldp(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_neon_ldnp(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_neon_stp(p: *mut u8, v0: *const u8, v1: *const u8);
    fn rust_isa_neon_stnp(p: *mut u8, v0: *const u8, v1: *const u8);
    fn rust_isa_neon_ld1r_i8(p: *const u8, out: *mut u8);
    fn rust_isa_neon_ld1r_i16(p: *const u8, out: *mut u8);
    fn rust_isa_neon_ld1r_i32(p: *const u8, out: *mut u8);
    fn rust_isa_neon_ld1r_i64(p: *const u8, out: *mut u8);
}

unsafe extern "C" {
    fn rust_isa_neon_ld2r_i8(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_neon_ld2r_i16(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_neon_ld2r_i32(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_neon_ld2r_i64(p: *const u8, o0: *mut u8, o1: *mut u8);
    fn rust_isa_neon_ld3r_i8(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8);
    fn rust_isa_neon_ld3r_i16(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8);
    fn rust_isa_neon_ld3r_i32(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8);
    fn rust_isa_neon_ld3r_i64(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8);
    fn rust_isa_neon_ld4r_i8(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8, o3: *mut u8);
    fn rust_isa_neon_ld4r_i16(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8, o3: *mut u8);
    fn rust_isa_neon_ld4r_i32(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8, o3: *mut u8);
    fn rust_isa_neon_ld4r_i64(p: *const u8, o0: *mut u8, o1: *mut u8, o2: *mut u8, o3: *mut u8);
}

/// `ld1` loads one 16-byte vector.
///
/// # Safety
///
/// `p` must be valid for reading 16 bytes.
#[inline(always)]
pub unsafe fn ld1(p: *const u8) -> V128 {
    unsafe { super::v128_load(p) }
}

/// `st1` stores one 16-byte vector.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes.
#[inline(always)]
pub unsafe fn st1(p: *mut u8, a: V128) {
    unsafe {
        super::v128_store(p, a);
    }
}

/// `ld2` loads and deinterleaves two byte vectors.
///
/// # Safety
///
/// `p` must be valid for reading 32 bytes.
#[inline(always)]
pub unsafe fn ld2(p: *const u8) -> (V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ld2(p, a.as_mut_ptr(), b.as_mut_ptr());
    }
    (a, b)
}

/// `ld3` loads and deinterleaves three byte vectors.
///
/// # Safety
///
/// `p` must be valid for reading 48 bytes.
#[inline(always)]
pub unsafe fn ld3(p: *const u8) -> (V128, V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    let mut c = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ld3(p, a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    }
    (a, b, c)
}

/// `ld4` loads and deinterleaves four byte vectors.
///
/// # Safety
///
/// `p` must be valid for reading 64 bytes.
#[inline(always)]
pub unsafe fn ld4(p: *const u8) -> (V128, V128, V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    let mut c = V128::from_bytes([0; 16]);
    let mut d = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ld4(
            p,
            a.as_mut_ptr(),
            b.as_mut_ptr(),
            c.as_mut_ptr(),
            d.as_mut_ptr(),
        );
    }
    (a, b, c, d)
}

/// `st2` interleaves and stores two byte vectors.
///
/// # Safety
///
/// `p` must be valid for writing 32 bytes.
#[inline(always)]
pub unsafe fn st2(p: *mut u8, a: V128, b: V128) {
    unsafe {
        rust_isa_neon_st2(p, a.as_ptr(), b.as_ptr());
    }
}

/// `st3` interleaves and stores three byte vectors.
///
/// # Safety
///
/// `p` must be valid for writing 48 bytes.
#[inline(always)]
pub unsafe fn st3(p: *mut u8, a: V128, b: V128, c: V128) {
    unsafe {
        rust_isa_neon_st3(p, a.as_ptr(), b.as_ptr(), c.as_ptr());
    }
}

/// `st4` interleaves and stores four byte vectors.
///
/// # Safety
///
/// `p` must be valid for writing 64 bytes.
#[inline(always)]
pub unsafe fn st4(p: *mut u8, a: V128, b: V128, c: V128, d: V128) {
    unsafe {
        rust_isa_neon_st4(p, a.as_ptr(), b.as_ptr(), c.as_ptr(), d.as_ptr());
    }
}

/// `ldp` loads a pair of 16-byte vectors.
///
/// # Safety
///
/// `p` must be valid for reading 32 bytes.
#[inline(always)]
pub unsafe fn ldp(p: *const u8) -> (V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ldp(p, a.as_mut_ptr(), b.as_mut_ptr());
    }
    (a, b)
}

/// `ldnp` loads a pair of 16-byte vectors without an ordering requirement.
///
/// # Safety
///
/// `p` must be valid for reading 32 bytes.
#[inline(always)]
pub unsafe fn ldnp(p: *const u8) -> (V128, V128) {
    let mut a = V128::from_bytes([0; 16]);
    let mut b = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_neon_ldnp(p, a.as_mut_ptr(), b.as_mut_ptr());
    }
    (a, b)
}

/// `stp` stores a pair of 16-byte vectors.
///
/// # Safety
///
/// `p` must be valid for writing 32 bytes.
#[inline(always)]
pub unsafe fn stp(p: *mut u8, a: V128, b: V128) {
    unsafe {
        rust_isa_neon_stp(p, a.as_ptr(), b.as_ptr());
    }
}

/// `stnp` stores a pair of 16-byte vectors without an ordering requirement.
///
/// # Safety
///
/// `p` must be valid for writing 32 bytes.
#[inline(always)]
pub unsafe fn stnp(p: *mut u8, a: V128, b: V128) {
    unsafe {
        rust_isa_neon_stnp(p, a.as_ptr(), b.as_ptr());
    }
}

macro_rules! ld1r {
    ($name:ident, $c:ident) => {
        /// `ld1r` loads one element and broadcasts it to every lane.
        ///
        /// # Safety
        ///
        /// `p` must be valid for reading one element.
        #[inline(always)]
        pub unsafe fn $name(p: *const u8) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(p, out.as_mut_ptr());
            }
            out
        }
    };
}

ld1r!(ld1r_i8, rust_isa_neon_ld1r_i8);
ld1r!(ld1r_i16, rust_isa_neon_ld1r_i16);
ld1r!(ld1r_i32, rust_isa_neon_ld1r_i32);
ld1r!(ld1r_i64, rust_isa_neon_ld1r_i64);

macro_rules! ld2r {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(p: *const u8) -> (V128, V128) {
            let mut a = V128::from_bytes([0; 16]);
            let mut b = V128::from_bytes([0; 16]);
            unsafe {
                $c(p, a.as_mut_ptr(), b.as_mut_ptr());
            }
            (a, b)
        }
    };
}

ld2r!(
    /// `ld2r` loads two consecutive elements and broadcasts each to all lanes.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading two elements.
    ld2r_i8,
    rust_isa_neon_ld2r_i8
);
ld2r!(
    /// `ld2r` on 16-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading two elements.
    ld2r_i16,
    rust_isa_neon_ld2r_i16
);
ld2r!(
    /// `ld2r` on 32-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading two elements.
    ld2r_i32,
    rust_isa_neon_ld2r_i32
);
ld2r!(
    /// `ld2r` on 64-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading two elements.
    ld2r_i64,
    rust_isa_neon_ld2r_i64
);

macro_rules! ld3r {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(p: *const u8) -> (V128, V128, V128) {
            let mut a = V128::from_bytes([0; 16]);
            let mut b = V128::from_bytes([0; 16]);
            let mut c = V128::from_bytes([0; 16]);
            unsafe {
                $c(p, a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
            }
            (a, b, c)
        }
    };
}

ld3r!(
    /// `ld3r` loads three consecutive elements and broadcasts each to all lanes.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading three elements.
    ld3r_i8,
    rust_isa_neon_ld3r_i8
);
ld3r!(
    /// `ld3r` on 16-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading three elements.
    ld3r_i16,
    rust_isa_neon_ld3r_i16
);
ld3r!(
    /// `ld3r` on 32-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading three elements.
    ld3r_i32,
    rust_isa_neon_ld3r_i32
);
ld3r!(
    /// `ld3r` on 64-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading three elements.
    ld3r_i64,
    rust_isa_neon_ld3r_i64
);

macro_rules! ld4r {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(p: *const u8) -> (V128, V128, V128, V128) {
            let mut a = V128::from_bytes([0; 16]);
            let mut b = V128::from_bytes([0; 16]);
            let mut c = V128::from_bytes([0; 16]);
            let mut d = V128::from_bytes([0; 16]);
            unsafe {
                $c(p, a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr(), d.as_mut_ptr());
            }
            (a, b, c, d)
        }
    };
}

ld4r!(
    /// `ld4r` loads four consecutive elements and broadcasts each to all lanes.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading four elements.
    ld4r_i8,
    rust_isa_neon_ld4r_i8
);
ld4r!(
    /// `ld4r` on 16-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading four elements.
    ld4r_i16,
    rust_isa_neon_ld4r_i16
);
ld4r!(
    /// `ld4r` on 32-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading four elements.
    ld4r_i32,
    rust_isa_neon_ld4r_i32
);
ld4r!(
    /// `ld4r` on 64-bit elements.
    ///
    /// # Safety
    ///
    /// `p` must be valid for reading four elements.
    ld4r_i64,
    rust_isa_neon_ld4r_i64
);
