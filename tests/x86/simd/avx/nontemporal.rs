use rust_isa::x86::simd::avx::{vmovntpd, vmovntps};

use super::support::*;

#[repr(align(32))]
struct Aligned([u8; 32]);

#[test]
fn non_temporal_stores() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let value = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let mut output = Aligned([0; 32]);
    unsafe {
        vmovntps(output.0.as_mut_ptr(), value);
    }
    assert_eq!(output.0, value.to_bytes());

    let value = f64x4([1.0, 2.0, 3.0, 4.0]);
    let mut output = Aligned([0; 32]);
    unsafe {
        vmovntpd(output.0.as_mut_ptr(), value);
    }
    assert_eq!(output.0, value.to_bytes());
}
