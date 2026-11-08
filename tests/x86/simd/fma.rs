use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::fma::*;
use rust_isa::x86::simd::sse2::V128;

fn enabled() -> bool {
    is_x86_feature_detected!("fma") && is_x86_feature_detected!("avx2")
}

fn ps(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn pd(values: [f64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn ss(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn sd(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn ps_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn pd_lanes(value: V256) -> [f64; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn ss_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn sd_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

#[test]
fn packed_single() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = ps([2.0; 8]);
    let b = ps([3.0; 8]);
    let c = ps([4.0; 8]);
    assert_eq!(
        ps_lanes(unsafe { vfmadd132ps(a, b, c) }),
        [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmadd213ps(a, b, c) }),
        [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmadd231ps(a, b, c) }),
        [14.0, 14.0, 14.0, 14.0, 14.0, 14.0, 14.0, 14.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmsub132ps(a, b, c) }),
        [2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmsub213ps(a, b, c) }),
        [2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmsub231ps(a, b, c) }),
        [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfnmadd132ps(a, b, c) }),
        [-2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfnmadd213ps(a, b, c) }),
        [-2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfnmadd231ps(a, b, c) }),
        [-10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfnmsub132ps(a, b, c) }),
        [-10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfnmsub213ps(a, b, c) }),
        [-10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0, -10.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfnmsub231ps(a, b, c) }),
        [-14.0, -14.0, -14.0, -14.0, -14.0, -14.0, -14.0, -14.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmaddsub132ps(a, b, c) }),
        [2.0, 10.0, 2.0, 10.0, 2.0, 10.0, 2.0, 10.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmaddsub213ps(a, b, c) }),
        [2.0, 10.0, 2.0, 10.0, 2.0, 10.0, 2.0, 10.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmaddsub231ps(a, b, c) }),
        [10.0, 14.0, 10.0, 14.0, 10.0, 14.0, 10.0, 14.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmsubadd132ps(a, b, c) }),
        [10.0, 2.0, 10.0, 2.0, 10.0, 2.0, 10.0, 2.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmsubadd213ps(a, b, c) }),
        [10.0, 2.0, 10.0, 2.0, 10.0, 2.0, 10.0, 2.0]
    );
    assert_eq!(
        ps_lanes(unsafe { vfmsubadd231ps(a, b, c) }),
        [14.0, 10.0, 14.0, 10.0, 14.0, 10.0, 14.0, 10.0]
    );
}

#[test]
fn packed_double() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = pd([2.0; 4]);
    let b = pd([3.0; 4]);
    let c = pd([4.0; 4]);
    assert_eq!(
        pd_lanes(unsafe { vfmadd132pd(a, b, c) }),
        [10.0, 10.0, 10.0, 10.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmadd213pd(a, b, c) }),
        [10.0, 10.0, 10.0, 10.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmadd231pd(a, b, c) }),
        [14.0, 14.0, 14.0, 14.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmsub132pd(a, b, c) }),
        [2.0, 2.0, 2.0, 2.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmsub213pd(a, b, c) }),
        [2.0, 2.0, 2.0, 2.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmsub231pd(a, b, c) }),
        [10.0, 10.0, 10.0, 10.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfnmadd132pd(a, b, c) }),
        [-2.0, -2.0, -2.0, -2.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfnmadd213pd(a, b, c) }),
        [-2.0, -2.0, -2.0, -2.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfnmadd231pd(a, b, c) }),
        [-10.0, -10.0, -10.0, -10.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfnmsub132pd(a, b, c) }),
        [-10.0, -10.0, -10.0, -10.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfnmsub213pd(a, b, c) }),
        [-10.0, -10.0, -10.0, -10.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfnmsub231pd(a, b, c) }),
        [-14.0, -14.0, -14.0, -14.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmaddsub132pd(a, b, c) }),
        [2.0, 10.0, 2.0, 10.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmaddsub213pd(a, b, c) }),
        [2.0, 10.0, 2.0, 10.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmaddsub231pd(a, b, c) }),
        [10.0, 14.0, 10.0, 14.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmsubadd132pd(a, b, c) }),
        [10.0, 2.0, 10.0, 2.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmsubadd213pd(a, b, c) }),
        [10.0, 2.0, 10.0, 2.0]
    );
    assert_eq!(
        pd_lanes(unsafe { vfmsubadd231pd(a, b, c) }),
        [14.0, 10.0, 14.0, 10.0]
    );
}

#[test]
fn scalar_single() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = ss([2.0, 7.0, 8.0, 9.0]);
    let b = ss([3.0; 4]);
    let c = ss([4.0; 4]);
    assert_eq!(
        ss_lanes(unsafe { vfmadd132ss(a, b, c) }),
        [10.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfmadd213ss(a, b, c) }),
        [10.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfmadd231ss(a, b, c) }),
        [14.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfmsub132ss(a, b, c) }),
        [2.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfmsub213ss(a, b, c) }),
        [2.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfmsub231ss(a, b, c) }),
        [10.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfnmadd132ss(a, b, c) }),
        [-2.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfnmadd213ss(a, b, c) }),
        [-2.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfnmadd231ss(a, b, c) }),
        [-10.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfnmsub132ss(a, b, c) }),
        [-10.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfnmsub213ss(a, b, c) }),
        [-10.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        ss_lanes(unsafe { vfnmsub231ss(a, b, c) }),
        [-14.0, 7.0, 8.0, 9.0]
    );
}

#[test]
fn scalar_double() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = sd([2.0, 7.0]);
    let b = sd([3.0; 2]);
    let c = sd([4.0; 2]);
    assert_eq!(sd_lanes(unsafe { vfmadd132sd(a, b, c) }), [10.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfmadd213sd(a, b, c) }), [10.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfmadd231sd(a, b, c) }), [14.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfmsub132sd(a, b, c) }), [2.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfmsub213sd(a, b, c) }), [2.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfmsub231sd(a, b, c) }), [10.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfnmadd132sd(a, b, c) }), [-2.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfnmadd213sd(a, b, c) }), [-2.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfnmadd231sd(a, b, c) }), [-10.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfnmsub132sd(a, b, c) }), [-10.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfnmsub213sd(a, b, c) }), [-10.0, 7.0]);
    assert_eq!(sd_lanes(unsafe { vfnmsub231sd(a, b, c) }), [-14.0, 7.0]);
}
