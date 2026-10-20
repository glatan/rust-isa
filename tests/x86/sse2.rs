use rust_isa::x86::simd::sse2::{paddd, v128_load, v128_store, V128};

#[test]
fn paddd_adds_each_i32_lane() {
    let a = V128::from_bytes([
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0,
    ]);
    let b = V128::from_bytes([
        10, 0, 0, 0, 20, 0, 0, 0, 30, 0, 0, 0, 40, 0, 0, 0,
    ]);
    let c = paddd(a, b);
    assert_eq!(
        c.to_bytes(),
        [
            11, 0, 0, 0, 22, 0, 0, 0, 33, 0, 0, 0, 44, 0, 0, 0,
        ]
    );
}

#[test]
fn load_store_roundtrip() {
    let source = [
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ];
    let value = unsafe { v128_load(source.as_ptr()) };
    assert_eq!(value.to_bytes(), source);

    let mut output = [0u8; 16];
    unsafe {
        v128_store(output.as_mut_ptr(), value);
    }
    assert_eq!(output, source);
}
