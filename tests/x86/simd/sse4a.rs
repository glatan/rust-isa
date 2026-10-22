use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::sse4a::{extrq, extrq_imm, insertq, insertq_imm, movntsd, movntss};

#[test]
fn sse4a_immediate_forms_and_stores() {
    if !is_x86_feature_detected!("sse4a") {
        return;
    }
    let mut value_bytes = [0u8; 16];
    value_bytes[..8].copy_from_slice(&0x0123_4567_89ab_cdefu64.to_le_bytes());
    let value = V128::from_bytes(value_bytes);
    let mut control_bytes = [0u8; 16];
    control_bytes[0] = 8;
    control_bytes[1] = 8;
    let control = V128::from_bytes(control_bytes);
    assert_eq!(unsafe { extrq_imm::<8, 8>(value) }, unsafe {
        extrq(value, control)
    });

    let mut inserted_bytes = [0u8; 16];
    inserted_bytes[0] = 0xab;
    inserted_bytes[8] = 8;
    inserted_bytes[9] = 8;
    let inserted = V128::from_bytes(inserted_bytes);
    assert_eq!(unsafe { insertq_imm::<8, 8>(value, inserted) }, unsafe {
        insertq(value, inserted)
    });

    let mut sd = 0.0f64;
    let mut sd_bytes = [0u8; 16];
    sd_bytes[..8].copy_from_slice(&1.5f64.to_le_bytes());
    unsafe { movntsd(&mut sd, V128::from_bytes(sd_bytes)) };
    assert_eq!(sd, 1.5);

    let mut ss = 0.0f32;
    let mut ss_bytes = [0u8; 16];
    ss_bytes[..4].copy_from_slice(&2.5f32.to_le_bytes());
    unsafe { movntss(&mut ss, V128::from_bytes(ss_bytes)) };
    assert_eq!(ss, 2.5);
}
