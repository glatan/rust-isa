//! RISC-V Zfh half-precision floating-point instructions.
//!
//! Zfh is not part of the baseline. Every operation is `unsafe` and the caller
//! must compile with Zfh enabled. The wrappers do not carry `#[target_feature]`
//! because rustc does not expose it on stable.
//!
//! Values are binary16 bit patterns in `u16`.

unsafe extern "C" {
    fn rust_isa_riscv_zfh_fadd(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fsub(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fmul(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fdiv(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fsqrt(a: u16) -> u16;
    fn rust_isa_riscv_zfh_fmin(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_fmax(a: u16, b: u16) -> u16;
    fn rust_isa_riscv_zfh_feq(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfh_flt(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfh_fle(a: u16, b: u16) -> u32;
    fn rust_isa_riscv_zfh_flh(p: *const u8, out: *mut u8);
    fn rust_isa_riscv_zfh_fsh(p: *mut u8, a: *const u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline]
        pub unsafe fn $name(a: u16, b: u16) -> u16 {
            unsafe { $c(a, b) }
        }
    };
}
macro_rules! compare {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline]
        pub unsafe fn $name(a: u16, b: u16) -> bool {
            unsafe { $c(a, b) != 0 }
        }
    };
}

binary!(
    /// `fadd.h`.
    fadd_h,
    rust_isa_riscv_zfh_fadd
);
binary!(
    /// `fsub.h`.
    fsub_h,
    rust_isa_riscv_zfh_fsub
);
binary!(
    /// `fmul.h`.
    fmul_h,
    rust_isa_riscv_zfh_fmul
);
binary!(
    /// `fdiv.h`.
    fdiv_h,
    rust_isa_riscv_zfh_fdiv
);
binary!(
    /// `fmin.h`.
    fmin_h,
    rust_isa_riscv_zfh_fmin
);
binary!(
    /// `fmax.h`.
    fmax_h,
    rust_isa_riscv_zfh_fmax
);
compare!(
    /// `feq.h`.
    feq_h,
    rust_isa_riscv_zfh_feq
);
compare!(
    /// `flt.h`.
    flt_h,
    rust_isa_riscv_zfh_flt
);
compare!(
    /// `fle.h`.
    fle_h,
    rust_isa_riscv_zfh_fle
);

/// `fsqrt.h`.
#[inline]
pub unsafe fn fsqrt_h(a: u16) -> u16 {
    unsafe { rust_isa_riscv_zfh_fsqrt(a) }
}

/// `flh`.
#[inline]
pub unsafe fn flh(p: *const u8) -> u16 {
    let mut out = [0u8; 2];
    unsafe { rust_isa_riscv_zfh_flh(p, out.as_mut_ptr()) };
    u16::from_le_bytes(out)
}

/// `fsh`.
#[inline]
pub unsafe fn fsh(p: *mut u8, value: u16) {
    let bytes = value.to_le_bytes();
    unsafe { rust_isa_riscv_zfh_fsh(p, bytes.as_ptr()) }
}

unsafe extern "C" {
    fn rust_isa_riscv_zfh_fcvt_h_s(value: u32) -> u16;
    fn rust_isa_riscv_zfh_fcvt_s_h(value: u16) -> u32;
    fn rust_isa_riscv_zfh_fcvt_h_d(value: u64) -> u16;
    fn rust_isa_riscv_zfh_fcvt_d_h(value: u16) -> u64;
    fn rust_isa_riscv_zfh_fcvt_w_h(value: u16) -> i32;
    fn rust_isa_riscv_zfh_fcvt_wu_h(value: u16) -> u32;
    fn rust_isa_riscv_zfh_fcvt_h_w(value: i32) -> u16;
    fn rust_isa_riscv_zfh_fcvt_h_wu(value: u32) -> u16;
    fn rust_isa_riscv_zfh_fmv_x_h(value: u16) -> i16;
    fn rust_isa_riscv_zfh_fmv_h_x(value: i16) -> u16;
}

macro_rules! convert {
    ($(#[$meta:meta])* $name:ident, $c:ident, $from:ty, $to:ty) => {
        $(#[$meta])*
        #[inline]
        pub unsafe fn $name(value: $from) -> $to {
            unsafe { $c(value) }
        }
    };
}
convert!(
    /// `fcvt.h.s`.
    fcvt_h_s,
    rust_isa_riscv_zfh_fcvt_h_s,
    u32,
    u16
);
convert!(
    /// `fcvt.s.h`.
    fcvt_s_h,
    rust_isa_riscv_zfh_fcvt_s_h,
    u16,
    u32
);
convert!(
    /// `fcvt.h.d`.
    fcvt_h_d,
    rust_isa_riscv_zfh_fcvt_h_d,
    u64,
    u16
);
convert!(
    /// `fcvt.d.h`.
    fcvt_d_h,
    rust_isa_riscv_zfh_fcvt_d_h,
    u16,
    u64
);
convert!(
    /// `fcvt.w.h`.
    fcvt_w_h,
    rust_isa_riscv_zfh_fcvt_w_h,
    u16,
    i32
);
convert!(
    /// `fcvt.wu.h`.
    fcvt_wu_h,
    rust_isa_riscv_zfh_fcvt_wu_h,
    u16,
    u32
);
convert!(
    /// `fcvt.h.w`.
    fcvt_h_w,
    rust_isa_riscv_zfh_fcvt_h_w,
    i32,
    u16
);
convert!(
    /// `fcvt.h.wu`.
    fcvt_h_wu,
    rust_isa_riscv_zfh_fcvt_h_wu,
    u32,
    u16
);
convert!(
    /// `fmv.x.h`.
    fmv_x_h,
    rust_isa_riscv_zfh_fmv_x_h,
    u16,
    i16
);
convert!(
    /// `fmv.h.x`.
    fmv_h_x,
    rust_isa_riscv_zfh_fmv_h_x,
    i16,
    u16
);
