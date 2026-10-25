//! ADX.
//!
//! ADX is not part of the x86-64 baseline. Every instruction wrapper is `unsafe` and the
//! caller must enable ADX with `#[target_feature(enable = "adx")]` or a global
//! target feature.
//!
//! [`adcx_u32`] adds with the carry flag and [`adox_u32`] adds with the overflow
//! flag. They are intended to form two independent carry chains. The functions
//! return the result and the new carry (or overflow) flag.
//!
//! The 64-bit forms are available only on x86-64.
//!
//! Instruction behavior follows the [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).
//!
//! The wrappers use `asm!`.
//!
//! Instructions not listed in this module are not exposed.

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::x86::integer::adx::adcx_u32 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::integer::adx::adcx_u64 as _;
//! use rust_isa::x86::integer::adx::adox_u32 as _;
//! #[cfg(target_arch = "x86_64")]
//! use rust_isa::x86::integer::adx::adox_u64 as _;
//! ```
//! <!-- doctest-inventory:end -->

/// `adcx` with 32-bit operands.
///
/// # Safety
///
/// The caller must enable ADX.
#[target_feature(enable = "adx")]
#[inline]
pub unsafe fn adcx_u32(a: u32, b: u32, carry: bool) -> (u32, bool) {
    let mut result = a;
    let mut carry_out = 0u8;
    unsafe {
        core::arch::asm!(
            "bt {carry:e}, 0",
            "adcx {result:e}, {b:e}",
            "setc {carry_out}",
            carry = in(reg) carry as u32,
            result = inout(reg) result,
            b = in(reg) b,
            carry_out = out(reg_byte) carry_out,
            options(nostack),
        );
    }
    (result, carry_out != 0)
}

/// `adox` with 32-bit operands.
///
/// # Safety
///
/// The caller must enable ADX.
#[target_feature(enable = "adx")]
#[inline]
pub unsafe fn adox_u32(a: u32, b: u32, carry: bool) -> (u32, bool) {
    let mut result = a;
    let mut carry_out = 0u8;
    let flag = 0x7fff_ffffu32;
    unsafe {
        core::arch::asm!(
            "add {flag:e}, {carry:e}",
            "adox {result:e}, {b:e}",
            "seto {carry_out}",
            flag = inout(reg) flag => _,
            carry = in(reg) carry as u32,
            result = inout(reg) result,
            b = in(reg) b,
            carry_out = out(reg_byte) carry_out,
            options(nostack),
        );
    }
    (result, carry_out != 0)
}

/// `adcx` with 64-bit operands.
///
/// # Safety
///
/// The caller must enable ADX.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "adx")]
#[inline]
pub unsafe fn adcx_u64(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let mut result = a;
    let mut carry_out = 0u8;
    unsafe {
        core::arch::asm!(
            "bt {carry}, 0",
            "adcx {result}, {b}",
            "setc {carry_out}",
            carry = in(reg) carry as u64,
            result = inout(reg) result,
            b = in(reg) b,
            carry_out = out(reg_byte) carry_out,
            options(nostack),
        );
    }
    (result, carry_out != 0)
}

/// `adox` with 64-bit operands.
///
/// # Safety
///
/// The caller must enable ADX.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "adx")]
#[inline]
pub unsafe fn adox_u64(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let mut result = a;
    let mut carry_out = 0u8;
    let flag = 0x7fff_ffff_ffff_ffffu64;
    unsafe {
        core::arch::asm!(
            "add {flag}, {carry}",
            "adox {result}, {b}",
            "seto {carry_out}",
            flag = inout(reg) flag => _,
            carry = in(reg) carry as u64,
            result = inout(reg) result,
            b = in(reg) b,
            carry_out = out(reg_byte) carry_out,
            options(nostack),
        );
    }
    (result, carry_out != 0)
}
