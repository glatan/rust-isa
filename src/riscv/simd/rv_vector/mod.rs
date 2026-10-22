//! RISC-V V extension.
//!
//! The V extension is unstable. The caller must compile with
//! `-C target-feature=+v`; the wrappers do not carry `#[target_feature]`.

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_rv_vector_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadd.vv`
///
/// Add the four 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.vv`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rv_vector_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_mul_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_splat_i32(value: i32, out: *mut u8);
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
    /// `vadd.vv` on 8-bit lanes.
    add_i8,
    rust_isa_rv_vector_add_i8
);
binary!(
    /// `vadd.vv` on 16-bit lanes.
    add_i16,
    rust_isa_rv_vector_add_i16
);
binary!(
    /// `vadd.vv` on 64-bit lanes.
    add_i64,
    rust_isa_rv_vector_add_i64
);
binary!(
    /// `vsub.vv` on 8-bit lanes.
    sub_i8,
    rust_isa_rv_vector_sub_i8
);
binary!(
    /// `vsub.vv` on 16-bit lanes.
    sub_i16,
    rust_isa_rv_vector_sub_i16
);
binary!(
    /// `vsub.vv` on 32-bit lanes.
    sub_i32,
    rust_isa_rv_vector_sub_i32
);
binary!(
    /// `vsub.vv` on 64-bit lanes.
    sub_i64,
    rust_isa_rv_vector_sub_i64
);
binary!(
    /// `vmul.vv` on 8-bit lanes, keeping the low 8 bits.
    mul_i8,
    rust_isa_rv_vector_mul_i8
);
binary!(
    /// `vmul.vv` on 16-bit lanes, keeping the low 16 bits.
    mul_i16,
    rust_isa_rv_vector_mul_i16
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_rv_vector_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_rv_vector_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_rv_vector_xor
);
binary!(
    /// `vmseq.vv` on 32-bit lanes. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_rv_vector_cmpeq_i32
);
binary!(
    /// `vmsgt.vv` on signed 32-bit lanes. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_rv_vector_cmpgt_i32
);
binary!(
    /// `vfadd.vv` on 32-bit float lanes.
    fadd,
    rust_isa_rv_vector_fadd
);
binary!(
    /// `vfsub.vv` on 32-bit float lanes.
    fsub,
    rust_isa_rv_vector_fsub
);
binary!(
    /// `vfmul.vv` on 32-bit float lanes.
    fmul,
    rust_isa_rv_vector_fmul
);
binary!(
    /// `vfdiv.vv` on 32-bit float lanes.
    fdiv,
    rust_isa_rv_vector_fdiv
);

/// Broadcast a 32-bit value to all four lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_splat_i32(value, out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_rv_vector_vdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vdivu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vrem(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vremu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsll(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsrl(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsra(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vminu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vmaxu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vmseq_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmsne_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmslt_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmsle_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmsgt_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmsge_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmsltu_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmsleu_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmsgtu_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmsgeu_mask(a: *const u8, b: *const u8) -> u16;
    fn rust_isa_rv_vector_vmacc(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vnmsac(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vmadd(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vnmsub(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vmv(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vmv_x_s(a: *const u8) -> i32;
    fn rust_isa_rv_vector_vmv_s_x(value: i32, out: *mut u8);
}

/// Expand a four-lane mask into all-ones 32-bit lanes.
fn mask_to_v128_i32(mask: u16) -> V128 {
    let mut bytes = [0u8; 16];
    for lane in 0..4 {
        if mask & (1 << lane) != 0 {
            bytes[lane * 4..lane * 4 + 4].copy_from_slice(&(-1i32).to_le_bytes());
        }
    }
    V128::from_bytes(bytes)
}

/// `vadd.vv` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vadd(a: V128, b: V128) -> V128 {
    unsafe { add(a, b) }
}

/// `vsub.vv` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vsub(a: V128, b: V128) -> V128 {
    unsafe { sub_i32(a, b) }
}

/// `vmul.vv` on 32-bit integer lanes, keeping the low 32 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmul(a: V128, b: V128) -> V128 {
    unsafe { mul(a, b) }
}

/// `vand.vv`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vand(a: V128, b: V128) -> V128 {
    unsafe { and(a, b) }
}

/// `vor.vv`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vor(a: V128, b: V128) -> V128 {
    unsafe { or(a, b) }
}

/// `vxor.vv`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vxor(a: V128, b: V128) -> V128 {
    unsafe { xor(a, b) }
}

binary!(
    /// `vdiv.vv` on 32-bit integer lanes.
    vdiv,
    rust_isa_rv_vector_vdiv
);
binary!(
    /// `vdivu.vv` on 32-bit unsigned integer lanes.
    vdivu,
    rust_isa_rv_vector_vdivu
);
binary!(
    /// `vrem.vv` on 32-bit integer lanes.
    vrem,
    rust_isa_rv_vector_vrem
);
binary!(
    /// `vremu.vv` on 32-bit unsigned integer lanes.
    vremu,
    rust_isa_rv_vector_vremu
);
binary!(
    /// `vsll.vv` on 32-bit integer lanes.
    vsll,
    rust_isa_rv_vector_vsll
);
binary!(
    /// `vsrl.vv` on 32-bit integer lanes.
    vsrl,
    rust_isa_rv_vector_vsrl
);
binary!(
    /// `vsra.vv` on 32-bit integer lanes.
    vsra,
    rust_isa_rv_vector_vsra
);
binary!(
    /// `vmin.vv` on signed 32-bit integer lanes.
    vmin,
    rust_isa_rv_vector_vmin
);
binary!(
    /// `vmax.vv` on signed 32-bit integer lanes.
    vmax,
    rust_isa_rv_vector_vmax
);
binary!(
    /// `vminu.vv` on unsigned 32-bit integer lanes.
    vminu,
    rust_isa_rv_vector_vminu
);
binary!(
    /// `vmaxu.vv` on unsigned 32-bit integer lanes.
    vmaxu,
    rust_isa_rv_vector_vmaxu
);

macro_rules! compare_mask {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// Returns a `u16` with bit `i` set when lane `i` compares true.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> u16 {
            unsafe { $c(a.as_ptr(), b.as_ptr()) }
        }
    };
}

compare_mask!(
    /// `vmseq.vv` on 32-bit integer lanes.
    vmseq_mask,
    rust_isa_rv_vector_vmseq_mask
);
compare_mask!(
    /// `vmsne.vv` on 32-bit integer lanes.
    vmsne_mask,
    rust_isa_rv_vector_vmsne_mask
);
compare_mask!(
    /// `vmslt.vv` on signed 32-bit integer lanes.
    vmslt_mask,
    rust_isa_rv_vector_vmslt_mask
);
compare_mask!(
    /// `vmsle.vv` on signed 32-bit integer lanes.
    vmsle_mask,
    rust_isa_rv_vector_vmsle_mask
);
compare_mask!(
    /// `vmsgt.vv` on signed 32-bit integer lanes.
    vmsgt_mask,
    rust_isa_rv_vector_vmsgt_mask
);
compare_mask!(
    /// `vmsge.vv` on signed 32-bit integer lanes.
    vmsge_mask,
    rust_isa_rv_vector_vmsge_mask
);
compare_mask!(
    /// `vmsltu.vv` on unsigned 32-bit integer lanes.
    vmsltu_mask,
    rust_isa_rv_vector_vmsltu_mask
);
compare_mask!(
    /// `vmsleu.vv` on unsigned 32-bit integer lanes.
    vmsleu_mask,
    rust_isa_rv_vector_vmsleu_mask
);
compare_mask!(
    /// `vmsgtu.vv` on unsigned 32-bit integer lanes.
    vmsgtu_mask,
    rust_isa_rv_vector_vmsgtu_mask
);
compare_mask!(
    /// `vmsgeu.vv` on unsigned 32-bit integer lanes.
    vmsgeu_mask,
    rust_isa_rv_vector_vmsgeu_mask
);

macro_rules! compare_v128 {
    ($(#[$meta:meta])* $name:ident, $mask:ident) => {
        $(#[$meta])*
        ///
        /// A true lane is all bits one.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            mask_to_v128_i32(unsafe { $mask(a, b) })
        }
    };
}

compare_v128!(
    /// `vmseq.vv` on 32-bit integer lanes.
    vmseq,
    vmseq_mask
);
compare_v128!(
    /// `vmsne.vv` on 32-bit integer lanes.
    vmsne,
    vmsne_mask
);
compare_v128!(
    /// `vmslt.vv` on signed 32-bit integer lanes.
    vmslt,
    vmslt_mask
);
compare_v128!(
    /// `vmsle.vv` on signed 32-bit integer lanes.
    vmsle,
    vmsle_mask
);
compare_v128!(
    /// `vmsgt.vv` on signed 32-bit integer lanes.
    vmsgt,
    vmsgt_mask
);
compare_v128!(
    /// `vmsge.vv` on signed 32-bit integer lanes.
    vmsge,
    vmsge_mask
);
compare_v128!(
    /// `vmsltu.vv` on unsigned 32-bit integer lanes.
    vmsltu,
    vmsltu_mask
);
compare_v128!(
    /// `vmsleu.vv` on unsigned 32-bit integer lanes.
    vmsleu,
    vmsleu_mask
);
compare_v128!(
    /// `vmsgtu.vv` on unsigned 32-bit integer lanes.
    vmsgtu,
    vmsgtu_mask
);
compare_v128!(
    /// `vmsgeu.vv` on unsigned 32-bit integer lanes.
    vmsgeu,
    vmsgeu_mask
);

macro_rules! ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(
                    acc.as_ptr(),
                    a.as_ptr(),
                    b.as_ptr(),
                    out.as_mut_ptr(),
                );
            }
            out
        }
    };
}

ternary!(
    /// `vmacc.vv`: `acc + a * b` on 32-bit integer lanes.
    vmacc,
    rust_isa_rv_vector_vmacc
);
ternary!(
    /// `vnmsac.vv`: `acc - a * b` on 32-bit integer lanes.
    vnmsac,
    rust_isa_rv_vector_vnmsac
);
ternary!(
    /// `vmadd.vv`: `a * acc + b` on 32-bit integer lanes.
    vmadd,
    rust_isa_rv_vector_vmadd
);
ternary!(
    /// `vnmsub.vv`: `b - a * acc` on 32-bit integer lanes.
    vnmsub,
    rust_isa_rv_vector_vnmsub
);

/// `vmv.v.v`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vmv(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmv.v.x`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv_v_x(value: i32) -> V128 {
    unsafe { splat_i32(value) }
}

/// `vmv.x.s`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv_x_s(a: V128) -> i32 {
    unsafe { rust_isa_rv_vector_vmv_x_s(a.as_ptr()) }
}

/// `vmv.s.x`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmv_s_x(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vmv_s_x(value, out.as_mut_ptr());
    }
    out
}
