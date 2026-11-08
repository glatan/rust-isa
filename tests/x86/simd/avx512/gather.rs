use rust_isa::x86::simd::Mask16;
use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn avx512f_gather_and_scatter() {
    if !f_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let data = [10.0f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let indices = i32x16([0, 2, 4, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let gathered = unsafe {
        vgatherdps(
            f32x16([0.0; 16]),
            data.as_ptr().cast(),
            indices,
            Mask16::from_bits(0x000f),
        )
    };
    assert_eq!(f32_lanes(gathered)[..4], [10.0, 30.0, 50.0, 70.0]);

    let mut output = [0.0f32; 8];
    unsafe {
        vscatterdps(
            output.as_mut_ptr().cast(),
            indices,
            f32x16([
                1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]),
            Mask16::from_bits(0x000f),
        );
    }
    assert_eq!(output, [1.0, 0.0, 2.0, 0.0, 3.0, 0.0, 4.0, 0.0]);
}
