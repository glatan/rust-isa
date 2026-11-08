use rust_isa::x86::simd::avx2::{
    vpmaskmovd_load, vpmaskmovd_store, vpmaskmovq_load, vpmaskmovq_store,
};

use super::support::*;

#[test]
fn mask_load_and_store() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let data = [10i32, 20, 30, 40, 50, 60, 70, 80];
    assert_eq!(
        i32_lanes(unsafe { vpmaskmovd_load(data.as_ptr().cast(), i32x8([-1; 8])) }),
        data
    );
    assert_eq!(
        i32_lanes(unsafe {
            vpmaskmovd_load(data.as_ptr().cast(), i32x8([0, -1, -1, -1, -1, -1, -1, -1]))
        }),
        [0, 20, 30, 40, 50, 60, 70, 80]
    );

    let mut output = [0i32; 8];
    unsafe {
        vpmaskmovd_store(
            output.as_mut_ptr().cast(),
            i32x8([-1, -1, 0, -1, -1, -1, -1, -1]),
            i32x8(data),
        );
    }
    assert_eq!(output, [10, 20, 0, 40, 50, 60, 70, 80]);

    let data = [10i64, 20, 30, 40];
    assert_eq!(
        i64_lanes(unsafe { vpmaskmovq_load(data.as_ptr().cast(), i64x4([-1; 4])) }),
        data
    );
    let mut output = [0i64; 4];
    unsafe {
        vpmaskmovq_store(
            output.as_mut_ptr().cast(),
            i64x4([-1, 0, -1, -1]),
            i64x4(data),
        );
    }
    assert_eq!(output, [10, 0, 30, 40]);
}
