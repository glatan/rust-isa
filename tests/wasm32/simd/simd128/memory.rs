use rust_isa::wasm32::simd::simd128::*;

use super::lanes::{f32_lanes, f32x4, i8x16, i16_lanes, i16x8, i32_lanes, i32x4, i64_lanes, i64x2};

#[test]
fn unaligned_load_store_and_zero_extend_loads() {
    let value = f32x4([1.0, 2.0, 3.0, 4.0]);
    let mut buffer = [0u8; 40];
    unsafe {
        v128_store(buffer.as_mut_ptr().add(1), value);
        assert_eq!(
            f32_lanes(v128_load(buffer.as_ptr().add(1))),
            [1.0, 2.0, 3.0, 4.0]
        );

        buffer[8] = 9;
        assert_eq!(v128_load8_splat(buffer.as_ptr().add(8)).to_bytes(), [9; 16]);
        buffer[9] = 0x34;
        buffer[10] = 0x12;
        assert_eq!(
            i16_lanes(v128_load16_splat(buffer.as_ptr().add(9))),
            [0x1234; 8]
        );
        buffer[11] = 1;
        buffer[12] = 0;
        buffer[13] = 0;
        buffer[14] = 0;
        assert_eq!(
            i32_lanes(v128_load32_splat(buffer.as_ptr().add(11))),
            [1; 4]
        );
        assert_eq!(
            i32_lanes(v128_load32_zero(buffer.as_ptr().add(11))),
            [1, 0, 0, 0]
        );

        let mut eight = [0u8; 8];
        eight[0] = 0xff;
        eight[1] = 0x02;
        assert_eq!(i16_lanes(v128_load8x8_s(eight.as_ptr()))[0], -1);
        assert_eq!(i16_lanes(v128_load8x8_u(eight.as_ptr()))[0], 255);
        assert_eq!(i16_lanes(v128_load8x8_s(eight.as_ptr()))[1], 2);
        assert_eq!(i64_lanes(v128_load64_zero(eight.as_ptr())), [0x02ff, 0]);

        let wide = [0xff, 0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x80];
        assert_eq!(
            i32_lanes(v128_load16x4_s(wide.as_ptr())),
            [-1, 2, 0, -32768]
        );
        assert_eq!(
            i32_lanes(v128_load16x4_u(wide.as_ptr())).map(|lane| lane as u32),
            [65535, 2, 0, 32768]
        );
        let words = [0xff, 0xff, 0xff, 0xff, 0x02, 0x00, 0x00, 0x00];
        assert_eq!(i64_lanes(v128_load32x2_s(words.as_ptr())), [-1, 2]);
        assert_eq!(
            i64_lanes(v128_load32x2_u(words.as_ptr())).map(|lane| lane as u64),
            [0xffff_ffff, 2]
        );
        assert_eq!(
            i64_lanes(v128_load64_splat(words.as_ptr())),
            [0x0000_0002_ffff_ffff; 2]
        );
    }
}

#[test]
fn lane_load_and_store() {
    let mut buffer = [0u8; 8];
    buffer[0] = 7;
    buffer[1] = 0x34;
    buffer[2] = 0x12;
    unsafe {
        let loaded = v128_load8_lane::<2>(buffer.as_ptr(), i8x16_splat(0));
        assert_eq!(i8x16_extract_lane_u::<2>(loaded), 7);
        assert_eq!(i8x16_extract_lane_u::<0>(loaded), 0);
        v128_store8_lane::<1>(
            buffer.as_mut_ptr().add(3),
            i8x16([
                10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            ]),
        );
        assert_eq!(buffer[3], 11);
        assert_eq!(buffer[2], 0x12);
        assert_eq!(buffer[4], 0);

        let i16s = v128_load16_lane::<1>(buffer.as_ptr().add(1), i16x8([0; 8]));
        assert_eq!(i16_lanes(i16s)[1], 0x1234);
        v128_store16_lane::<1>(
            buffer.as_mut_ptr().add(4),
            i16x8([0x0102, 0x0304, 0, 0, 0, 0, 0, 0]),
        );
        assert_eq!(&buffer[4..6], &[0x04, 0x03]);
        assert_eq!(buffer[6], 0);

        let word = 0x0403_0201i32;
        let bytes = word.to_le_bytes();
        let i32s = v128_load32_lane::<3>(bytes.as_ptr(), i32x4([0; 4]));
        assert_eq!(i32_lanes(i32s)[3], word);
        v128_store32_lane::<3>(buffer.as_mut_ptr(), i32x4([0, 0, 0, word]));
        assert_eq!(&buffer[0..4], &bytes);

        let wide = 0x0807_0605_0403_0201i64;
        let wide_bytes = wide.to_le_bytes();
        let i64s = v128_load64_lane::<1>(wide_bytes.as_ptr(), i64x2([0, 0]));
        assert_eq!(i64_lanes(i64s), [0, wide]);
        let mut out = [0u8; 8];
        v128_store64_lane::<1>(out.as_mut_ptr(), i64x2([0, wide]));
        assert_eq!(out, wide_bytes);
    }
}
