//! RISC-V V extension.
//!
//! The V extension is unstable. The caller must compile with
//! `-C target-feature=+v`; the wrappers do not carry `#[target_feature]`.
//! Every operation is `unsafe`. The values are [`V128`]. Some key-schedule
//! functions take a runtime round number; the valid range is documented on each
//! function.
//!
//! Instruction behavior follows the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

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
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
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

unsafe extern "C" {
    fn rust_isa_rv_vector_vsadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vssub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsaddu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vssubu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vaadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vaaddu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vasub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vasubu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwmacc(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwmaccu(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwmaccsu(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwmaccus(acc: *const u8, scalar: u32, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vzext(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsext(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwaddu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwsubu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwmulu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vwmulsu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vnsrl(a: *const u8, shift: u32, out: *mut u8);
    fn rust_isa_rv_vector_vnsra(a: *const u8, shift: u32, out: *mut u8);
    fn rust_isa_rv_vector_vnclip(a: *const u8, shift: u32, out: *mut u8);
    fn rust_isa_rv_vector_vnclipu(a: *const u8, shift: u32, out: *mut u8);
}

binary!(
    /// `vsadd.vv` on signed 32-bit integer lanes. Saturates to `i32::MIN..=i32::MAX`.
    vsadd,
    rust_isa_rv_vector_vsadd
);
binary!(
    /// `vssub.vv` on signed 32-bit integer lanes. Saturates to `i32::MIN..=i32::MAX`.
    vssub,
    rust_isa_rv_vector_vssub
);
binary!(
    /// `vsaddu.vv` on unsigned 32-bit integer lanes. Saturates to `u32::MAX`.
    vsaddu,
    rust_isa_rv_vector_vsaddu
);
binary!(
    /// `vssubu.vv` on unsigned 32-bit integer lanes. Saturates to zero.
    vssubu,
    rust_isa_rv_vector_vssubu
);
binary!(
    /// `vaadd.vv` on signed 32-bit integer lanes.
    ///
    /// Rounds to nearest, ties away from zero (`vxrm = 0`).
    vaadd,
    rust_isa_rv_vector_vaadd
);
binary!(
    /// `vaaddu.vv` on unsigned 32-bit integer lanes.
    ///
    /// Rounds to nearest, ties away from zero (`vxrm = 0`).
    vaaddu,
    rust_isa_rv_vector_vaaddu
);
binary!(
    /// `vasub.vv` on signed 32-bit integer lanes.
    ///
    /// Rounds to nearest, ties away from zero (`vxrm = 0`).
    vasub,
    rust_isa_rv_vector_vasub
);
binary!(
    /// `vasubu.vv` on unsigned 32-bit integer lanes.
    ///
    /// Rounds to nearest, ties away from zero (`vxrm = 0`).
    vasubu,
    rust_isa_rv_vector_vasubu
);

ternary!(
    /// `vwmacc.vv`: `acc + sign_extend(a) * sign_extend(b)`.
    ///
    /// The low four 16-bit lanes of `a` and `b` are widened to 32 bits.
    vwmacc,
    rust_isa_rv_vector_vwmacc
);
ternary!(
    /// `vwmaccu.vv`: `acc + zero_extend(a) * zero_extend(b)`.
    ///
    /// The low four 16-bit lanes of `a` and `b` are widened to 32 bits.
    vwmaccu,
    rust_isa_rv_vector_vwmaccu
);
ternary!(
    /// `vwmaccsu.vv`: `acc + sign_extend(a) * zero_extend(b)`.
    ///
    /// The low four 16-bit lanes of `a` and `b` are widened to 32 bits.
    vwmaccsu,
    rust_isa_rv_vector_vwmaccsu
);

/// `vwmaccus.vx`: `acc + zero_extend(scalar) * sign_extend(b)`.
///
/// The low four 16-bit lanes of `b` are widened to 32 bits.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vwmaccus(acc: V128, scalar: u32, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vwmaccus(acc.as_ptr(), scalar, b.as_ptr(), out.as_mut_ptr());
    }
    out
}

macro_rules! widen {
    ($(#[$meta:meta])* $name:ident, $c:ident, $doc:literal) => {
        $(#[$meta])*
        #[doc = $doc]
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

widen!(
    /// `vzext.vf2`.
    vzext,
    rust_isa_rv_vector_vzext,
    "The low eight bytes are zero-extended to eight 16-bit lanes."
);
widen!(
    /// `vsext.vf2`.
    vsext,
    rust_isa_rv_vector_vsext,
    "The low eight bytes are sign-extended to eight 16-bit lanes."
);

binary!(
    /// `vwadd.vv`.
    ///
    /// The low four 16-bit lanes are sign-extended and added into 32-bit lanes.
    vwadd,
    rust_isa_rv_vector_vwadd
);
binary!(
    /// `vwaddu.vv`.
    ///
    /// The low four 16-bit lanes are zero-extended and added into 32-bit lanes.
    vwaddu,
    rust_isa_rv_vector_vwaddu
);
binary!(
    /// `vwsub.vv`.
    ///
    /// The low four 16-bit lanes are sign-extended and subtracted into 32-bit lanes.
    vwsub,
    rust_isa_rv_vector_vwsub
);
binary!(
    /// `vwsubu.vv`.
    ///
    /// The low four 16-bit lanes are zero-extended and subtracted into 32-bit lanes.
    vwsubu,
    rust_isa_rv_vector_vwsubu
);
binary!(
    /// `vwmul.vv`.
    ///
    /// The low four 16-bit lanes are sign-extended and multiplied into 32-bit lanes.
    vwmul,
    rust_isa_rv_vector_vwmul
);
binary!(
    /// `vwmulu.vv`.
    ///
    /// The low four 16-bit lanes are zero-extended and multiplied into 32-bit lanes.
    vwmulu,
    rust_isa_rv_vector_vwmulu
);
binary!(
    /// `vwmulsu.vv`.
    ///
    /// `a` is sign-extended and `b` is zero-extended before multiplying.
    vwmulsu,
    rust_isa_rv_vector_vwmulsu
);

macro_rules! narrow {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// The low four 32-bit lanes are narrowed to four 16-bit lanes. The
        /// remaining bytes are zero.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, shift: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), shift, out.as_mut_ptr());
            }
            out
        }
    };
}

narrow!(
    /// `vnsrl.wx`.
    vnsrl,
    rust_isa_rv_vector_vnsrl
);
narrow!(
    /// `vnsra.wx`.
    vnsra,
    rust_isa_rv_vector_vnsra
);
narrow!(
    /// `vnclip.wx`, rounding to nearest, ties away from zero (`vxrm = 0`).
    vnclip,
    rust_isa_rv_vector_vnclip
);
narrow!(
    /// `vnclipu.wx`, rounding to nearest, ties away from zero (`vxrm = 0`).
    vnclipu,
    rust_isa_rv_vector_vnclipu
);

unsafe extern "C" {
    fn rust_isa_rv_vector_vmerge(a: *const u8, b: *const u8, mask: u16, out: *mut u8);
    fn rust_isa_rv_vector_vslideup(dest: *const u8, src: *const u8, amount: u32, out: *mut u8);
    fn rust_isa_rv_vector_vslidedown(src: *const u8, amount: u32, out: *mut u8);
    fn rust_isa_rv_vector_vslide1up(src: *const u8, value: i32, out: *mut u8);
    fn rust_isa_rv_vector_vslide1down(src: *const u8, value: i32, out: *mut u8);
    fn rust_isa_rv_vector_vrgather(a: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vrgatherei16(a: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vcompress(a: *const u8, mask: u16, out: *mut u8);
    fn rust_isa_rv_vector_vmand(a: u16, b: u16) -> u16;
    fn rust_isa_rv_vector_vmnand(a: u16, b: u16) -> u16;
    fn rust_isa_rv_vector_vmandn(a: u16, b: u16) -> u16;
    fn rust_isa_rv_vector_vmxor(a: u16, b: u16) -> u16;
    fn rust_isa_rv_vector_vmor(a: u16, b: u16) -> u16;
    fn rust_isa_rv_vector_vmnor(a: u16, b: u16) -> u16;
    fn rust_isa_rv_vector_vmorn(a: u16, b: u16) -> u16;
    fn rust_isa_rv_vector_vmxnor(a: u16, b: u16) -> u16;
    fn rust_isa_rv_vector_vcpop(mask: u16) -> u64;
    fn rust_isa_rv_vector_vfirst(mask: u16) -> i64;
    fn rust_isa_rv_vector_vmsbf(mask: u16) -> u16;
    fn rust_isa_rv_vector_vmsof(mask: u16) -> u16;
    fn rust_isa_rv_vector_vmsif(mask: u16) -> u16;
    fn rust_isa_rv_vector_viota(mask: u16, out: *mut u8);
    fn rust_isa_rv_vector_vid(out: *mut u8);
    fn rust_isa_rv_vector_vredsum(a: *const u8) -> u32;
    fn rust_isa_rv_vector_vredand(a: *const u8) -> i32;
    fn rust_isa_rv_vector_vredor(a: *const u8) -> i32;
    fn rust_isa_rv_vector_vredxor(a: *const u8) -> i32;
    fn rust_isa_rv_vector_vredmin(a: *const u8) -> i32;
    fn rust_isa_rv_vector_vredmax(a: *const u8) -> i32;
    fn rust_isa_rv_vector_vredminu(a: *const u8) -> u32;
    fn rust_isa_rv_vector_vredmaxu(a: *const u8) -> u32;
    fn rust_isa_rv_vector_vwredsum(a: *const u8) -> i64;
    fn rust_isa_rv_vector_vwredsumu(a: *const u8) -> u64;
}

/// `vmerge.vvm` on 32-bit integer lanes.
///
/// Lane `i` comes from `b` when bit `i` of `mask` is set, otherwise from `a`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vmerge(a: V128, b: V128, mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vmerge(a.as_ptr(), b.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

/// `vslideup.vx` on 32-bit integer lanes.
///
/// Slides `src` up by `amount`; the low lanes come from `dest`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vslideup(dest: V128, src: V128, amount: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vslideup(dest.as_ptr(), src.as_ptr(), amount, out.as_mut_ptr());
    }
    out
}

/// `vslidedown.vx` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vslidedown(src: V128, amount: u32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vslidedown(src.as_ptr(), amount, out.as_mut_ptr());
    }
    out
}

/// `vslide1up.vx` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vslide1up(src: V128, value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vslide1up(src.as_ptr(), value, out.as_mut_ptr());
    }
    out
}

/// `vslide1down.vx` on 32-bit integer lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vslide1down(src: V128, value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vslide1down(src.as_ptr(), value, out.as_mut_ptr());
    }
    out
}

/// `vrgather.vv` on 32-bit integer lanes.
///
/// Out-of-range indices produce zero.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vrgather(a: V128, indices: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vrgather(a.as_ptr(), indices.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrgatherei16.vv` on 32-bit integer lanes.
///
/// The low four 16-bit lanes of `indices` select the source lanes.
/// Out-of-range indices produce zero.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vrgatherei16(a: V128, indices: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vrgatherei16(a.as_ptr(), indices.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vcompress.vm` on 32-bit integer lanes.
///
/// Active lanes are packed to the low end; the remaining lanes are zero.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vcompress(a: V128, mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vcompress(a.as_ptr(), mask, out.as_mut_ptr());
    }
    out
}

macro_rules! mask_binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: u16, b: u16) -> u16 {
            unsafe { $c(a, b) }
        }
    };
}

mask_binary!(
    /// `vmand.mm`.
    vmand,
    rust_isa_rv_vector_vmand
);
mask_binary!(
    /// `vmnand.mm`.
    vmnand,
    rust_isa_rv_vector_vmnand
);
mask_binary!(
    /// `vmandn.mm`.
    vmandn,
    rust_isa_rv_vector_vmandn
);
mask_binary!(
    /// `vmxor.mm`.
    vmxor,
    rust_isa_rv_vector_vmxor
);
mask_binary!(
    /// `vmor.mm`.
    vmor,
    rust_isa_rv_vector_vmor
);
mask_binary!(
    /// `vmnor.mm`.
    vmnor,
    rust_isa_rv_vector_vmnor
);
mask_binary!(
    /// `vmorn.mm`.
    vmorn,
    rust_isa_rv_vector_vmorn
);
mask_binary!(
    /// `vmxnor.mm`.
    vmxnor,
    rust_isa_rv_vector_vmxnor
);

/// `vcpop.m`.
///
/// Returns the number of set bits in the 16-bit mask.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vcpop(mask: u16) -> u64 {
    unsafe { rust_isa_rv_vector_vcpop(mask) }
}

/// `vfirst.m`.
///
/// Returns the index of the first set bit, or `-1` when the mask is zero.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfirst(mask: u16) -> i64 {
    unsafe { rust_isa_rv_vector_vfirst(mask) }
}

macro_rules! mask_unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(mask: u16) -> u16 {
            unsafe { $c(mask) }
        }
    };
}

mask_unary!(
    /// `vmsbf.m`.
    vmsbf,
    rust_isa_rv_vector_vmsbf
);
mask_unary!(
    /// `vmsof.m`.
    vmsof,
    rust_isa_rv_vector_vmsof
);
mask_unary!(
    /// `vmsif.m`.
    vmsif,
    rust_isa_rv_vector_vmsif
);

/// `viota.m` on 8-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn viota(mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_viota(mask, out.as_mut_ptr());
    }
    out
}

/// `vid.v` on 8-bit lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vid() -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vid(out.as_mut_ptr());
    }
    out
}

macro_rules! reduce {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> $ty {
            unsafe { $c(a.as_ptr()) }
        }
    };
}

reduce!(
    /// `vredsum.vs` on 32-bit integer lanes.
    vredsum,
    rust_isa_rv_vector_vredsum,
    u32
);
reduce!(
    /// `vredand.vs` on 32-bit integer lanes.
    vredand,
    rust_isa_rv_vector_vredand,
    i32
);
reduce!(
    /// `vredor.vs` on 32-bit integer lanes.
    vredor,
    rust_isa_rv_vector_vredor,
    i32
);
reduce!(
    /// `vredxor.vs` on 32-bit integer lanes.
    vredxor,
    rust_isa_rv_vector_vredxor,
    i32
);
reduce!(
    /// `vredmin.vs` on signed 32-bit integer lanes.
    vredmin,
    rust_isa_rv_vector_vredmin,
    i32
);
reduce!(
    /// `vredmax.vs` on signed 32-bit integer lanes.
    vredmax,
    rust_isa_rv_vector_vredmax,
    i32
);
reduce!(
    /// `vredminu.vs` on unsigned 32-bit integer lanes.
    vredminu,
    rust_isa_rv_vector_vredminu,
    u32
);
reduce!(
    /// `vredmaxu.vs` on unsigned 32-bit integer lanes.
    vredmaxu,
    rust_isa_rv_vector_vredmaxu,
    u32
);
reduce!(
    /// `vwredsum.vs` from signed 32-bit to 64-bit lanes.
    vwredsum,
    rust_isa_rv_vector_vwredsum,
    i64
);
reduce!(
    /// `vwredsumu.vs` from unsigned 32-bit to 64-bit lanes.
    vwredsumu,
    rust_isa_rv_vector_vwredsumu,
    u64
);

unsafe extern "C" {
    fn rust_isa_rv_vector_vle8(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vle16(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vle32(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vle64(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vse8(p: *mut u8, value: *const u8);
    fn rust_isa_rv_vector_vse16(p: *mut u8, value: *const u8);
    fn rust_isa_rv_vector_vse32(p: *mut u8, value: *const u8);
    fn rust_isa_rv_vector_vse64(p: *mut u8, value: *const u8);
    fn rust_isa_rv_vector_vlse8(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vlse16(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vlse32(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vlse64(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vsse8(p: *mut u8, stride: isize, value: *const u8);
    fn rust_isa_rv_vector_vsse16(p: *mut u8, stride: isize, value: *const u8);
    fn rust_isa_rv_vector_vsse32(p: *mut u8, stride: isize, value: *const u8);
    fn rust_isa_rv_vector_vsse64(p: *mut u8, stride: isize, value: *const u8);
    fn rust_isa_rv_vector_vloxei8(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vloxei16(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vloxei32(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vloxei64(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vluxei8(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vluxei16(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vluxei32(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vluxei64(base: *const u8, indices: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsoxei8(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rv_vector_vsoxei16(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rv_vector_vsoxei32(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rv_vector_vsoxei64(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rv_vector_vsuxei8(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rv_vector_vsuxei16(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rv_vector_vsuxei32(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rv_vector_vsuxei64(base: *mut u8, indices: *const u8, value: *const u8);
    fn rust_isa_rv_vector_vlm(p: *const u8) -> u16;
    fn rust_isa_rv_vector_vsm(p: *mut u8, mask: u16);
}

macro_rules! load {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled. `p` must be
        /// valid for reading 16 bytes.
        #[inline(always)]
        pub unsafe fn $name(p: *const $ty) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(p.cast(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! store {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled. `p` must be
        /// valid for writing 16 bytes.
        #[inline(always)]
        pub unsafe fn $name(p: *mut $ty, value: V128) {
            unsafe {
                $c(p.cast(), value.as_ptr());
            }
        }
    };
}

load!(
    /// `vle8.v`.
    vle8,
    rust_isa_rv_vector_vle8,
    u8
);
load!(
    /// `vle16.v`.
    vle16,
    rust_isa_rv_vector_vle16,
    u16
);
load!(
    /// `vle32.v`.
    vle32,
    rust_isa_rv_vector_vle32,
    i32
);
load!(
    /// `vle64.v`.
    vle64,
    rust_isa_rv_vector_vle64,
    u64
);
store!(
    /// `vse8.v`.
    vse8,
    rust_isa_rv_vector_vse8,
    u8
);
store!(
    /// `vse16.v`.
    vse16,
    rust_isa_rv_vector_vse16,
    u16
);
store!(
    /// `vse32.v`.
    vse32,
    rust_isa_rv_vector_vse32,
    i32
);
store!(
    /// `vse64.v`.
    vse64,
    rust_isa_rv_vector_vse64,
    u64
);

macro_rules! strided_load {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// `stride` is in bytes.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled. `p + i * stride`
        /// must be valid for reading one element for each loaded lane.
        #[inline(always)]
        pub unsafe fn $name(p: *const $ty, stride: isize) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(p.cast(), stride, out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! strided_store {
    ($(#[$meta:meta])* $name:ident, $c:ident, $ty:ty) => {
        $(#[$meta])*
        ///
        /// `stride` is in bytes.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled. `p + i * stride`
        /// must be valid for writing one element for each stored lane.
        #[inline(always)]
        pub unsafe fn $name(p: *mut $ty, stride: isize, value: V128) {
            unsafe {
                $c(p.cast(), stride, value.as_ptr());
            }
        }
    };
}

strided_load!(
    /// `vlse8.v`.
    vlse8,
    rust_isa_rv_vector_vlse8,
    u8
);
strided_load!(
    /// `vlse16.v`.
    vlse16,
    rust_isa_rv_vector_vlse16,
    u16
);
strided_load!(
    /// `vlse32.v`.
    vlse32,
    rust_isa_rv_vector_vlse32,
    i32
);
strided_load!(
    /// `vlse64.v`.
    vlse64,
    rust_isa_rv_vector_vlse64,
    u64
);
strided_store!(
    /// `vsse8.v`.
    vsse8,
    rust_isa_rv_vector_vsse8,
    u8
);
strided_store!(
    /// `vsse16.v`.
    vsse16,
    rust_isa_rv_vector_vsse16,
    u16
);
strided_store!(
    /// `vsse32.v`.
    vsse32,
    rust_isa_rv_vector_vsse32,
    i32
);
strided_store!(
    /// `vsse64.v`.
    vsse64,
    rust_isa_rv_vector_vsse64,
    u64
);

macro_rules! indexed_load {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// The low bytes of `indices` hold byte offsets for the 32-bit data
        /// elements.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and the base
        /// pointer must be valid for every selected element.
        #[inline(always)]
        pub unsafe fn $name(base: *const i32, indices: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(base.cast(), indices.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! indexed_store {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// The low bytes of `indices` hold byte offsets for the 32-bit data
        /// elements.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and the base
        /// pointer must be valid for every selected element.
        #[inline(always)]
        pub unsafe fn $name(base: *mut i32, indices: V128, value: V128) {
            unsafe {
                $c(base.cast(), indices.as_ptr(), value.as_ptr());
            }
        }
    };
}

indexed_load!(
    /// `vloxei8.v`.
    vloxei8,
    rust_isa_rv_vector_vloxei8
);
indexed_load!(
    /// `vloxei16.v`.
    vloxei16,
    rust_isa_rv_vector_vloxei16
);
indexed_load!(
    /// `vloxei32.v`.
    vloxei32,
    rust_isa_rv_vector_vloxei32
);
indexed_load!(
    /// `vloxei64.v`. Only the first two lanes are written.
    vloxei64,
    rust_isa_rv_vector_vloxei64
);
indexed_load!(
    /// `vluxei8.v`.
    vluxei8,
    rust_isa_rv_vector_vluxei8
);
indexed_load!(
    /// `vluxei16.v`.
    vluxei16,
    rust_isa_rv_vector_vluxei16
);
indexed_load!(
    /// `vluxei32.v`.
    vluxei32,
    rust_isa_rv_vector_vluxei32
);
indexed_load!(
    /// `vluxei64.v`. Only the first two lanes are written.
    vluxei64,
    rust_isa_rv_vector_vluxei64
);
indexed_store!(
    /// `vsoxei8.v`.
    vsoxei8,
    rust_isa_rv_vector_vsoxei8
);
indexed_store!(
    /// `vsoxei16.v`.
    vsoxei16,
    rust_isa_rv_vector_vsoxei16
);
indexed_store!(
    /// `vsoxei32.v`.
    vsoxei32,
    rust_isa_rv_vector_vsoxei32
);
indexed_store!(
    /// `vsoxei64.v`. Only the first two lanes are written.
    vsoxei64,
    rust_isa_rv_vector_vsoxei64
);
indexed_store!(
    /// `vsuxei8.v`.
    vsuxei8,
    rust_isa_rv_vector_vsuxei8
);
indexed_store!(
    /// `vsuxei16.v`.
    vsuxei16,
    rust_isa_rv_vector_vsuxei16
);
indexed_store!(
    /// `vsuxei32.v`.
    vsuxei32,
    rust_isa_rv_vector_vsuxei32
);
indexed_store!(
    /// `vsuxei64.v`. Only the first two lanes are written.
    vsuxei64,
    rust_isa_rv_vector_vsuxei64
);

unsafe extern "C" {
    fn rust_isa_rv_vector_vlseg2(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsseg2(p: *mut u8, values: *const u8);
    fn rust_isa_rv_vector_vlsseg2(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vssseg2(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rv_vector_vlseg3(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsseg3(p: *mut u8, values: *const u8);
    fn rust_isa_rv_vector_vlsseg3(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vssseg3(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rv_vector_vlseg4(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsseg4(p: *mut u8, values: *const u8);
    fn rust_isa_rv_vector_vlsseg4(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vssseg4(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rv_vector_vlseg5(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsseg5(p: *mut u8, values: *const u8);
    fn rust_isa_rv_vector_vlsseg5(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vssseg5(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rv_vector_vlseg6(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsseg6(p: *mut u8, values: *const u8);
    fn rust_isa_rv_vector_vlsseg6(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vssseg6(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rv_vector_vlseg7(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsseg7(p: *mut u8, values: *const u8);
    fn rust_isa_rv_vector_vlsseg7(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vssseg7(p: *mut u8, stride: isize, values: *const u8);
    fn rust_isa_rv_vector_vlseg8(p: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsseg8(p: *mut u8, values: *const u8);
    fn rust_isa_rv_vector_vlsseg8(p: *const u8, stride: isize, out: *mut u8);
    fn rust_isa_rv_vector_vssseg8(p: *mut u8, stride: isize, values: *const u8);
}

macro_rules! segment {
    ($(#[$meta:meta])* $load:ident, $store:ident, $sload:ident, $sstore:ident,
     $cload:ident, $cstore:ident, $csload:ident, $csstore:ident, $n:expr) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and the base
        /// pointer must be valid for the requested number of interleaved
        /// elements.
        #[inline(always)]
        pub unsafe fn $load(p: *const i32) -> [V128; $n] {
            let mut out = [V128::from_bytes([0; 16]); $n];
            unsafe {
                $cload(p.cast(), out.as_mut_ptr().cast());
            }
            out
        }

        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and the base
        /// pointer must be valid for the requested number of interleaved
        /// elements.
        #[inline(always)]
        pub unsafe fn $store(p: *mut i32, values: [V128; $n]) {
            unsafe {
                $cstore(p.cast(), values.as_ptr().cast());
            }
        }

        $(#[$meta])*
        ///
        /// `stride` is in bytes.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and every
        /// selected element must be valid.
        #[inline(always)]
        pub unsafe fn $sload(p: *const i32, stride: isize) -> [V128; $n] {
            let mut out = [V128::from_bytes([0; 16]); $n];
            unsafe {
                $csload(p.cast(), stride, out.as_mut_ptr().cast());
            }
            out
        }

        $(#[$meta])*
        ///
        /// `stride` is in bytes.
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled and every
        /// selected element must be valid.
        #[inline(always)]
        pub unsafe fn $sstore(p: *mut i32, stride: isize, values: [V128; $n]) {
            unsafe {
                $csstore(p.cast(), stride, values.as_ptr().cast());
            }
        }
    };
}

segment!(
    /// `vlseg2e32.v`, `vsseg2e32.v`, `vlsseg2e32.v`, `vssseg2e32.v`.
    vlseg2,
    vsseg2,
    vlsseg2,
    vssseg2,
    rust_isa_rv_vector_vlseg2,
    rust_isa_rv_vector_vsseg2,
    rust_isa_rv_vector_vlsseg2,
    rust_isa_rv_vector_vssseg2,
    2
);
segment!(
    /// `vlseg3e32.v`, `vsseg3e32.v`, `vlsseg3e32.v`, `vssseg3e32.v`.
    vlseg3,
    vsseg3,
    vlsseg3,
    vssseg3,
    rust_isa_rv_vector_vlseg3,
    rust_isa_rv_vector_vsseg3,
    rust_isa_rv_vector_vlsseg3,
    rust_isa_rv_vector_vssseg3,
    3
);
segment!(
    /// `vlseg4e32.v`, `vsseg4e32.v`, `vlsseg4e32.v`, `vssseg4e32.v`.
    vlseg4,
    vsseg4,
    vlsseg4,
    vssseg4,
    rust_isa_rv_vector_vlseg4,
    rust_isa_rv_vector_vsseg4,
    rust_isa_rv_vector_vlsseg4,
    rust_isa_rv_vector_vssseg4,
    4
);
segment!(
    /// `vlseg5e32.v`, `vsseg5e32.v`, `vlsseg5e32.v`, `vssseg5e32.v`.
    vlseg5,
    vsseg5,
    vlsseg5,
    vssseg5,
    rust_isa_rv_vector_vlseg5,
    rust_isa_rv_vector_vsseg5,
    rust_isa_rv_vector_vlsseg5,
    rust_isa_rv_vector_vssseg5,
    5
);
segment!(
    /// `vlseg6e32.v`, `vsseg6e32.v`, `vlsseg6e32.v`, `vssseg6e32.v`.
    vlseg6,
    vsseg6,
    vlsseg6,
    vssseg6,
    rust_isa_rv_vector_vlseg6,
    rust_isa_rv_vector_vsseg6,
    rust_isa_rv_vector_vlsseg6,
    rust_isa_rv_vector_vssseg6,
    6
);
segment!(
    /// `vlseg7e32.v`, `vsseg7e32.v`, `vlsseg7e32.v`, `vssseg7e32.v`.
    vlseg7,
    vsseg7,
    vlsseg7,
    vssseg7,
    rust_isa_rv_vector_vlseg7,
    rust_isa_rv_vector_vsseg7,
    rust_isa_rv_vector_vlsseg7,
    rust_isa_rv_vector_vssseg7,
    7
);
segment!(
    /// `vlseg8e32.v`, `vsseg8e32.v`, `vlsseg8e32.v`, `vssseg8e32.v`.
    vlseg8,
    vsseg8,
    vlsseg8,
    vssseg8,
    rust_isa_rv_vector_vlseg8,
    rust_isa_rv_vector_vsseg8,
    rust_isa_rv_vector_vlsseg8,
    rust_isa_rv_vector_vssseg8,
    8
);

/// `vlm.v`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled. `p` must be valid for
/// reading two bytes.
#[inline(always)]
pub unsafe fn vlm(p: *const u8) -> u16 {
    unsafe { rust_isa_rv_vector_vlm(p) }
}

/// `vsm.v`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled. `p` must be valid for
/// writing two bytes.
#[inline(always)]
pub unsafe fn vsm(p: *mut u8, mask: u16) {
    unsafe {
        rust_isa_rv_vector_vsm(p, mask);
    }
}

unsafe extern "C" {
    fn rust_isa_rv_vector_vfrdiv(a: *const u8, scalar: f32, out: *mut u8);
    fn rust_isa_rv_vector_vfmin(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfmax(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfsgnj(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfsgnjn(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfsgnjx(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfabs(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfneg(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfsqrt(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfrsqrt7(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfrec7(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfmerge(a: *const u8, scalar: f32, mask: u16, out: *mut u8);
    fn rust_isa_rv_vector_vfmv(scalar: f32, out: *mut u8);
    fn rust_isa_rv_vector_vfmv_f_s(a: *const u8) -> f32;
    fn rust_isa_rv_vector_vfma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfms(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfnma(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfnms(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfredsum(a: *const u8) -> f32;
    fn rust_isa_rv_vector_vfredosum(a: *const u8) -> f32;
    fn rust_isa_rv_vector_vfredusum(a: *const u8) -> f32;
    fn rust_isa_rv_vector_vfredmax(a: *const u8) -> f32;
    fn rust_isa_rv_vector_vfredmin(a: *const u8) -> f32;
    fn rust_isa_rv_vector_vfwadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfwsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfwmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfwmacc(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfwnmacc(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfwmsac(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfwnmsac(acc: *const u8, a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfclass(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfcvt_xu_f(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfcvt_x_f(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfcvt_rtz_xu_f(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfcvt_rtz_x_f(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfcvt_f_xu(a: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vfcvt_f_x(a: *const u8, out: *mut u8);
}

/// `vfadd.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfadd(a: V128, b: V128) -> V128 {
    unsafe { fadd(a, b) }
}

/// `vfsub.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfsub(a: V128, b: V128) -> V128 {
    unsafe { fsub(a, b) }
}

/// `vfmul.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmul(a: V128, b: V128) -> V128 {
    unsafe { fmul(a, b) }
}

/// `vfdiv.vv` on 32-bit float lanes.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfdiv(a: V128, b: V128) -> V128 {
    unsafe { fdiv(a, b) }
}

/// `vfrdiv.vf` on 32-bit float lanes.
///
/// Computes `scalar / a`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfrdiv(a: V128, scalar: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vfrdiv(a.as_ptr(), scalar, out.as_mut_ptr());
    }
    out
}

binary!(
    /// `vfmin.vv` on 32-bit float lanes.
    vfmin,
    rust_isa_rv_vector_vfmin
);
binary!(
    /// `vfmax.vv` on 32-bit float lanes.
    vfmax,
    rust_isa_rv_vector_vfmax
);
binary!(
    /// `vfsgnj.vv` on 32-bit float lanes.
    vfsgnj,
    rust_isa_rv_vector_vfsgnj
);
binary!(
    /// `vfsgnjn.vv` on 32-bit float lanes.
    vfsgnjn,
    rust_isa_rv_vector_vfsgnjn
);
binary!(
    /// `vfsgnjx.vv` on 32-bit float lanes.
    vfsgnjx,
    rust_isa_rv_vector_vfsgnjx
);

macro_rules! float_unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

float_unary!(
    /// `vfabs.v`.
    vfabs,
    rust_isa_rv_vector_vfabs
);
float_unary!(
    /// `vfneg.v`.
    vfneg,
    rust_isa_rv_vector_vfneg
);
float_unary!(
    /// `vfsqrt.v`.
    vfsqrt,
    rust_isa_rv_vector_vfsqrt
);
float_unary!(
    /// `vfrsqrt7.v`.
    vfrsqrt7,
    rust_isa_rv_vector_vfrsqrt7
);
float_unary!(
    /// `vfrec7.v`.
    vfrec7,
    rust_isa_rv_vector_vfrec7
);

/// `vfmerge.vfm` on 32-bit float lanes.
///
/// Lane `i` becomes `scalar` when bit `i` of `mask` is set.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmerge(a: V128, scalar: f32, mask: u16) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vfmerge(a.as_ptr(), scalar, mask, out.as_mut_ptr());
    }
    out
}

/// `vfmv.v.f`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmv(scalar: f32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vfmv(scalar, out.as_mut_ptr());
    }
    out
}

/// `vfmv.f.s`.
///
/// # Safety
///
/// The caller must compile with the V extension enabled.
#[inline(always)]
pub unsafe fn vfmv_f_s(a: V128) -> f32 {
    unsafe { rust_isa_rv_vector_vfmv_f_s(a.as_ptr()) }
}

ternary!(
    /// `vfmadd.vv`: `acc * a + b`.
    vfma,
    rust_isa_rv_vector_vfma
);
ternary!(
    /// `vfmsub.vv`: `acc * a - b`.
    vfms,
    rust_isa_rv_vector_vfms
);
ternary!(
    /// `vfnmadd.vv`: `-(acc * a) - b`.
    vfnma,
    rust_isa_rv_vector_vfnma
);
ternary!(
    /// `vfnmsub.vv`: `-(acc * a) + b`.
    vfnms,
    rust_isa_rv_vector_vfnms
);

macro_rules! float_reduce {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the V extension enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> f32 {
            unsafe { $c(a.as_ptr()) }
        }
    };
}

float_reduce!(
    /// `vfredsum.vs`, the unordered reduction.
    vfredsum,
    rust_isa_rv_vector_vfredsum
);
float_reduce!(
    /// `vfredusum.vs`, the unordered reduction.
    vfredusum,
    rust_isa_rv_vector_vfredusum
);
float_reduce!(
    /// `vfredosum.vs`, the ordered reduction.
    vfredosum,
    rust_isa_rv_vector_vfredosum
);
float_reduce!(
    /// `vfredmax.vs`.
    vfredmax,
    rust_isa_rv_vector_vfredmax
);
float_reduce!(
    /// `vfredmin.vs`.
    vfredmin,
    rust_isa_rv_vector_vfredmin
);

binary!(
    /// `vfwadd.vv`, widening the low two 32-bit lanes to 64-bit lanes.
    vfwadd,
    rust_isa_rv_vector_vfwadd
);
binary!(
    /// `vfwsub.vv`, widening the low two 32-bit lanes to 64-bit lanes.
    vfwsub,
    rust_isa_rv_vector_vfwsub
);
binary!(
    /// `vfwmul.vv`, widening the low two 32-bit lanes to 64-bit lanes.
    vfwmul,
    rust_isa_rv_vector_vfwmul
);
ternary!(
    /// `vfwmacc.vv`: `acc + a * b` on two 64-bit accumulator lanes.
    vfwmacc,
    rust_isa_rv_vector_vfwmacc
);
ternary!(
    /// `vfwnmacc.vv`: `-(a * b) - acc` on two 64-bit accumulator lanes.
    vfwnmacc,
    rust_isa_rv_vector_vfwnmacc
);
ternary!(
    /// `vfwmsac.vv`: `a * b - acc` on two 64-bit accumulator lanes.
    vfwmsac,
    rust_isa_rv_vector_vfwmsac
);
ternary!(
    /// `vfwnmsac.vv`: `-(a * b) + acc` on two 64-bit accumulator lanes.
    vfwnmsac,
    rust_isa_rv_vector_vfwnmsac
);

float_unary!(
    /// `vfclass.v`, producing a 10-bit class mask in each 32-bit lane.
    vfclass,
    rust_isa_rv_vector_vfclass
);

float_unary!(
    /// `vfcvt.xu.f.v` on 32-bit lanes.
    vfcvt_xu_f,
    rust_isa_rv_vector_vfcvt_xu_f
);
float_unary!(
    /// `vfcvt.x.f.v` on 32-bit lanes.
    vfcvt_x_f,
    rust_isa_rv_vector_vfcvt_x_f
);
float_unary!(
    /// `vfcvt.rtz.xu.f.v` on 32-bit lanes.
    vfcvt_rtz_xu_f,
    rust_isa_rv_vector_vfcvt_rtz_xu_f
);
float_unary!(
    /// `vfcvt.rtz.x.f.v` on 32-bit lanes.
    vfcvt_rtz_x_f,
    rust_isa_rv_vector_vfcvt_rtz_x_f
);
float_unary!(
    /// `vfcvt.f.xu.v` on 32-bit lanes.
    vfcvt_f_xu,
    rust_isa_rv_vector_vfcvt_f_xu
);
float_unary!(
    /// `vfcvt.f.x.v` on 32-bit lanes.
    vfcvt_f_x,
    rust_isa_rv_vector_vfcvt_f_x
);

unsafe extern "C" {
    fn rust_isa_rv_vector_vaesef(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vaesem(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vaesdf(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vaesdm(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vaesz(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vaeskf1(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rv_vector_vaeskf2(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rv_vector_vsm4k(key: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rv_vector_vsm4r(state: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsha2ms(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsha2ch(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsha2cl(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsm3me(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vsm3c(state: *const u8, round: u32, out: *mut u8);
    fn rust_isa_rv_vector_vclmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vclmulh(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vghsh(acc: *const u8, value: *const u8, key: *const u8, out: *mut u8);
    fn rust_isa_rv_vector_vgmul(acc: *const u8, key: *const u8, out: *mut u8);
}

macro_rules! crypto_binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the vector cryptography extensions
        /// enabled.
        #[inline(always)]
        pub unsafe fn $name(state: V128, key: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(state.as_ptr(), key.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

crypto_binary!(
    /// `vaesef.vv`.
    vaesef,
    rust_isa_rv_vector_vaesef
);
crypto_binary!(
    /// `vaesem.vv`.
    vaesem,
    rust_isa_rv_vector_vaesem
);
crypto_binary!(
    /// `vaesdf.vv`.
    vaesdf,
    rust_isa_rv_vector_vaesdf
);
crypto_binary!(
    /// `vaesdm.vv`.
    vaesdm,
    rust_isa_rv_vector_vaesdm
);
crypto_binary!(
    /// `vaesz.vs`.
    vaesz,
    rust_isa_rv_vector_vaesz
);

macro_rules! crypto_key_schedule {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// `round` must be at most the largest immediate accepted by the
        /// instruction.
        ///
        /// # Safety
        ///
        /// The caller must compile with the vector cryptography extensions
        /// enabled and pass a valid round number.
        #[inline(always)]
        pub unsafe fn $name(key: V128, round: u32) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(key.as_ptr(), round, out.as_mut_ptr());
            }
            out
        }
    };
}

crypto_key_schedule!(
    /// `vaeskf1.vi`, with `round` in `1..=10`.
    vaeskf1,
    rust_isa_rv_vector_vaeskf1
);
crypto_key_schedule!(
    /// `vaeskf2.vi`, with `round` in `2..=10`.
    vaeskf2,
    rust_isa_rv_vector_vaeskf2
);
crypto_key_schedule!(
    /// `vsm4k.vi`, with `round` in `0..=7`.
    vsm4k,
    rust_isa_rv_vector_vsm4k
);

crypto_binary!(
    /// `vsm4r.vv`.
    vsm4r,
    rust_isa_rv_vector_vsm4r
);

macro_rules! crypto_ternary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must compile with the vector cryptography extensions
        /// enabled.
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128, c: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

crypto_ternary!(
    /// `vsha2ms.vv`.
    vsha2ms,
    rust_isa_rv_vector_vsha2ms
);
crypto_ternary!(
    /// `vsha2ch.vv`.
    vsha2ch,
    rust_isa_rv_vector_vsha2ch
);
crypto_ternary!(
    /// `vsha2cl.vv`.
    vsha2cl,
    rust_isa_rv_vector_vsha2cl
);
crypto_ternary!(
    /// `vghsh.vv`.
    vghsh,
    rust_isa_rv_vector_vghsh
);

/// `vsm3me.vv` on a 256-bit element group.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled.
#[inline(always)]
pub unsafe fn vsm3me(a: [V128; 2], b: [V128; 2]) -> [V128; 2] {
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rv_vector_vsm3me(
            a.as_ptr().cast(),
            b.as_ptr().cast(),
            out.as_mut_ptr().cast(),
        );
    }
    out
}

/// `vsm3c.vi` on a 256-bit element group.
///
/// `round` must be at most 31.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled and
/// pass a valid round number.
#[inline(always)]
pub unsafe fn vsm3c(state: [V128; 2], round: u32) -> [V128; 2] {
    let mut out = [V128::from_bytes([0; 16]); 2];
    unsafe {
        rust_isa_rv_vector_vsm3c(state.as_ptr().cast(), round, out.as_mut_ptr().cast());
    }
    out
}

crypto_binary!(
    /// `vclmul.vv` on 64-bit lanes.
    vclmul,
    rust_isa_rv_vector_vclmul
);
crypto_binary!(
    /// `vclmulh.vv` on 64-bit lanes.
    vclmulh,
    rust_isa_rv_vector_vclmulh
);

/// `vgmul.vv`.
///
/// # Safety
///
/// The caller must compile with the vector cryptography extensions enabled.
#[inline(always)]
pub unsafe fn vgmul(acc: V128, key: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_rv_vector_vgmul(acc.as_ptr(), key.as_ptr(), out.as_mut_ptr());
    }
    out
}
