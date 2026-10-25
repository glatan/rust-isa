//! WebAssembly SIMD128.
//!
//! Every value is a [`V128`]. The instruction decides what the bits mean.
//! `i32x4.add` and `f32x4.add` are different instructions, so [`V128`] does not implement `Add`.
//!
//! Behavior follows [Fixed-width SIMD]. The lane operations are specified with the [vector operations].
//!
//! [Fixed-width SIMD]: https://webassembly.github.io/spec/core/syntax/instructions.html#vector-instructions
//! [vector operations]: https://webassembly.github.io/spec/core/exec/numerics.html#vector-operations
//!
//! This module is compiled only when `target_feature = "simd128"` is enabled.
//! Dependents do not inherit rustflags from this crate. Set both flags with `RUSTFLAGS` or the target's `rustflags`:
//!
//! ```text
//! -C target-feature=+simd128 -C linker-plugin-lto
//! ```
//!
//! `cargo rustc -- -C target-feature=+simd128` and a `#[target_feature]` attribute on only the caller do not enable this module's `cfg` or its build script.
//! Without `linker-plugin-lto`, the linked binary keeps a call into the compiled C and a `v128.store` / `v128.load` pair for each temporary result.
//!
//! A wasm32 build with this feature also needs `clang`, including a wasm32 backend and `wasm_simd128.h`, and `llvm-ar`.
//! The build script compiles the bundled C and links it as a static library. Prebuilt objects are not published.
//!
//! Arithmetic and comparisons are safe. Loads, stores, and lane loads and stores are `unsafe`.
//! A misaligned address does not trap. An out-of-bounds address traps.
//!
//! An immediate lane index is `const LANE: u32`. A constant outside the lane count does not compile.
//!
//! A comparison writes all bits one to a true lane and all bits zero to a false lane.
//! For `f32` and `f64`, a comparison involving NaN is false, except `ne`, which is true.
//!
//! `i8x16.shuffle` is intentionally excluded: its 16 lane indices are an immediate in the
//! instruction encoding, and stable Rust cannot express them without `core::arch`.
//! [`i8x16_swizzle`] takes those indices from a register.
//!
//! ```
//! use rust_isa::wasm32::simd::simd128::{
//!     f32x4_add, f32x4_extract_lane, f32x4_splat, i8x16_eq, i8x16_splat,
//! };
//!
//! let sum = f32x4_add(f32x4_splat(1.5), f32x4_splat(2.5));
//! assert_eq!(f32x4_extract_lane::<0>(sum), 4.0);
//! assert_eq!(
//!     i8x16_eq(i8x16_splat(1), i8x16_splat(1)).to_bytes(),
//!     [0xff; 16]
//! );
//! assert_eq!(i8x16_eq(i8x16_splat(1), i8x16_splat(2)).to_bytes(), [0; 16]);
//! ```

//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::wasm32::simd::simd128::f32x4_abs as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_add as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_ceil as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_convert_i32x4_s as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_convert_i32x4_u as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_demote_f64x2_zero as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_div as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_eq as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_extract_lane as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_floor as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_ge as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_gt as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_le as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_lt as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_max as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_min as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_mul as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_ne as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_nearest as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_neg as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_pmax as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_pmin as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_replace_lane as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_splat as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_sqrt as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_sub as _;
//! use rust_isa::wasm32::simd::simd128::f32x4_trunc as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_abs as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_add as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_ceil as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_convert_low_i32x4_s as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_convert_low_i32x4_u as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_div as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_eq as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_extract_lane as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_floor as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_ge as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_gt as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_le as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_lt as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_max as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_min as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_mul as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_ne as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_nearest as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_neg as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_pmax as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_pmin as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_promote_low_f32x4 as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_replace_lane as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_splat as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_sqrt as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_sub as _;
//! use rust_isa::wasm32::simd::simd128::f64x2_trunc as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_abs as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_add as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_add_sat_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_add_sat_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_all_true as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_avgr_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_bitmask as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_eq as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extadd_pairwise_i8x16_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extadd_pairwise_i8x16_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extend_high_i8x16_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extend_high_i8x16_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extend_low_i8x16_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extend_low_i8x16_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extmul_high_i8x16_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extmul_high_i8x16_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extmul_low_i8x16_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extmul_low_i8x16_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extract_lane_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_extract_lane_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_ge_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_ge_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_gt_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_gt_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_le_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_le_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_lt_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_lt_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_max_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_max_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_min_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_min_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_mul as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_narrow_i32x4_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_narrow_i32x4_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_ne as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_neg as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_q15mulr_sat_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_replace_lane as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_shl as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_shr_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_shr_u as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_splat as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_sub as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_sub_sat_s as _;
//! use rust_isa::wasm32::simd::simd128::i16x8_sub_sat_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_abs as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_add as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_all_true as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_bitmask as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_dot_i16x8_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_eq as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extadd_pairwise_i16x8_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extadd_pairwise_i16x8_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extend_high_i16x8_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extend_high_i16x8_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extend_low_i16x8_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extend_low_i16x8_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extmul_high_i16x8_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extmul_high_i16x8_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extmul_low_i16x8_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extmul_low_i16x8_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_extract_lane as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_ge_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_ge_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_gt_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_gt_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_le_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_le_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_lt_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_lt_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_max_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_max_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_min_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_min_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_mul as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_ne as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_neg as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_replace_lane as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_shl as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_shr_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_shr_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_splat as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_sub as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_trunc_sat_f32x4_s as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_trunc_sat_f32x4_u as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_trunc_sat_f64x2_s_zero as _;
//! use rust_isa::wasm32::simd::simd128::i32x4_trunc_sat_f64x2_u_zero as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_abs as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_add as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_all_true as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_bitmask as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_eq as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extend_high_i32x4_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extend_high_i32x4_u as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extend_low_i32x4_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extend_low_i32x4_u as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extmul_high_i32x4_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extmul_high_i32x4_u as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extmul_low_i32x4_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extmul_low_i32x4_u as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_extract_lane as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_ge_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_gt_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_le_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_lt_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_mul as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_ne as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_neg as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_replace_lane as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_shl as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_shr_s as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_shr_u as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_splat as _;
//! use rust_isa::wasm32::simd::simd128::i64x2_sub as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_abs as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_add as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_add_sat_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_add_sat_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_all_true as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_avgr_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_bitmask as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_eq as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_extract_lane_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_extract_lane_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_ge_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_ge_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_gt_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_gt_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_le_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_le_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_lt_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_lt_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_max_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_max_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_min_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_min_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_narrow_i16x8_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_narrow_i16x8_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_ne as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_neg as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_popcnt as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_replace_lane as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_shl as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_shr_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_shr_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_splat as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_sub as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_sub_sat_s as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_sub_sat_u as _;
//! use rust_isa::wasm32::simd::simd128::i8x16_swizzle as _;
//! use rust_isa::wasm32::simd::simd128::v128_and as _;
//! use rust_isa::wasm32::simd::simd128::v128_andnot as _;
//! use rust_isa::wasm32::simd::simd128::v128_any_true as _;
//! use rust_isa::wasm32::simd::simd128::v128_bitselect as _;
//! use rust_isa::wasm32::simd::simd128::v128_load as _;
//! use rust_isa::wasm32::simd::simd128::v128_load16_lane as _;
//! use rust_isa::wasm32::simd::simd128::v128_load16_splat as _;
//! use rust_isa::wasm32::simd::simd128::v128_load16x4_s as _;
//! use rust_isa::wasm32::simd::simd128::v128_load16x4_u as _;
//! use rust_isa::wasm32::simd::simd128::v128_load32_lane as _;
//! use rust_isa::wasm32::simd::simd128::v128_load32_splat as _;
//! use rust_isa::wasm32::simd::simd128::v128_load32_zero as _;
//! use rust_isa::wasm32::simd::simd128::v128_load32x2_s as _;
//! use rust_isa::wasm32::simd::simd128::v128_load32x2_u as _;
//! use rust_isa::wasm32::simd::simd128::v128_load64_lane as _;
//! use rust_isa::wasm32::simd::simd128::v128_load64_splat as _;
//! use rust_isa::wasm32::simd::simd128::v128_load64_zero as _;
//! use rust_isa::wasm32::simd::simd128::v128_load8_lane as _;
//! use rust_isa::wasm32::simd::simd128::v128_load8_splat as _;
//! use rust_isa::wasm32::simd::simd128::v128_load8x8_s as _;
//! use rust_isa::wasm32::simd::simd128::v128_load8x8_u as _;
//! use rust_isa::wasm32::simd::simd128::v128_not as _;
//! use rust_isa::wasm32::simd::simd128::v128_or as _;
//! use rust_isa::wasm32::simd::simd128::v128_store as _;
//! use rust_isa::wasm32::simd::simd128::v128_store16_lane as _;
//! use rust_isa::wasm32::simd::simd128::v128_store32_lane as _;
//! use rust_isa::wasm32::simd::simd128::v128_store64_lane as _;
//! use rust_isa::wasm32::simd::simd128::v128_store8_lane as _;
//! use rust_isa::wasm32::simd::simd128::v128_xor as _;
//! ```
//! <!-- doctest-inventory:end -->

mod f32x4;
mod f64x2;
mod i16x8;
mod i32x4;
mod i64x2;
mod i8x16;
mod v128;

pub use f32x4::*;
pub use f64x2::*;
pub use i8x16::*;
pub use i16x8::*;
pub use i32x4::*;
pub use i64x2::*;
pub use v128::*;
