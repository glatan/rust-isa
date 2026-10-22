use rust_isa::x86::simd::avx::{
    vmaskmovpd_load, vmaskmovpd_store, vmaskmovps_load, vmaskmovps_store,
};

use super::support::*;

#[test]
fn masked_memory() {
    if !enabled() {
        return;
    }
    let data = f32x8([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
    assert_eq!(
        f32_lanes(unsafe { vmaskmovps_load(data.as_ptr(), f32x8([-1.0; 8])) }),
        [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]
    );
    let mut output = [0.0f32; 8];
    unsafe {
        vmaskmovps_store(
            output.as_mut_ptr().cast(),
            f32x8([-1.0, -1.0, 0.0, -1.0, -1.0, -1.0, -1.0, -1.0]),
            data,
        );
    }
    assert_eq!(output, [10.0, 20.0, 0.0, 40.0, 50.0, 60.0, 70.0, 80.0]);

    let data = f64x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        f64_lanes(unsafe { vmaskmovpd_load(data.as_ptr(), f64x4([-1.0; 4])) }),
        [10.0, 20.0, 30.0, 40.0]
    );
    let mut output = [0.0f64; 4];
    unsafe {
        vmaskmovpd_store(
            output.as_mut_ptr().cast(),
            f64x4([-1.0, 0.0, -1.0, -1.0]),
            data,
        );
    }
    assert_eq!(output, [10.0, 0.0, 30.0, 40.0]);
}
