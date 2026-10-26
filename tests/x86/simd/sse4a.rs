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
    control_bytes[0] = 4;
    control_bytes[1] = 4;
    let control = V128::from_bytes(control_bytes);
    let extracted = unsafe { extrq_imm::<4, 4>(value) }.to_bytes();
    assert_eq!(u64::from_le_bytes(extracted[..8].try_into().unwrap()), 0x0e);
    assert_eq!(
        extracted,
        unsafe { extrq(value, control) }.to_bytes(),
        "immediate and register forms disagree"
    );

    let mut inserted_bytes = [0u8; 16];
    inserted_bytes[0] = 0xab;
    inserted_bytes[8] = 4;
    inserted_bytes[9] = 4;
    let inserted = V128::from_bytes(inserted_bytes);
    let inserted_value = unsafe { insertq_imm::<4, 4>(value, inserted) }.to_bytes();
    assert_eq!(
        u64::from_le_bytes(inserted_value[..8].try_into().unwrap()),
        0x0123_4567_89ab_cdbf
    );
    assert_eq!(
        inserted_value,
        unsafe { insertq(value, inserted) }.to_bytes(),
        "immediate and register forms disagree"
    );

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
