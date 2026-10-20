use rust_isa::wasm32::wide_arithmetic::{add128, mul_wide_s, mul_wide_u, sub128};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_add(alo: u64, ahi: u64, blo: u64, bhi: u64, out: *mut u64) {
    let (lo, hi) = unsafe { add128((alo, ahi), (blo, bhi)) };
    unsafe {
        *out = lo;
        *out.add(1) = hi;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_sub(alo: u64, ahi: u64, blo: u64, bhi: u64, out: *mut u64) {
    let (lo, hi) = unsafe { sub128((alo, ahi), (blo, bhi)) };
    unsafe {
        *out = lo;
        *out.add(1) = hi;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_mul_u(a: u64, b: u64, out: *mut u64) {
    let (lo, hi) = unsafe { mul_wide_u(a, b) };
    unsafe {
        *out = lo;
        *out.add(1) = hi;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_mul_s(a: i64, b: i64, out: *mut i64) {
    let (lo, hi) = unsafe { mul_wide_s(a, b) };
    unsafe {
        *out = lo;
        *out.add(1) = hi;
    }
}
