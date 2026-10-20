use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::fma::vfmadd132ps;

fn enabled() -> bool {
    is_x86_feature_detected!("fma") && is_x86_feature_detected!("avx2")
}

#[test]
fn fused_multiply_add() {
    if !enabled() {
        return;
    }
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    let mut c = [0u8; 32];
    for lane in 0..8 {
        a[lane * 4..lane * 4 + 4].copy_from_slice(&2.0f32.to_le_bytes());
        b[lane * 4..lane * 4 + 4].copy_from_slice(&3.0f32.to_le_bytes());
        c[lane * 4..lane * 4 + 4].copy_from_slice(&4.0f32.to_le_bytes());
    }
    let out = unsafe {
        vfmadd132ps(
            V256::from_bytes(a),
            V256::from_bytes(b),
            V256::from_bytes(c),
        )
    };
    for lane in 0..8 {
        let bytes = out.to_bytes();
        assert_eq!(
            f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap()),
            10.0
        );
    }
}
