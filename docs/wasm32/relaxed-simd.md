# relaxed-simd

`relaxed_simd` は `src/wasm32/simd/relaxed_simd` にある。`V128` は `simd128` の型を再公開する。`f32x4.rs`、`f64x2.rs`、`i8x16.rs`、`i16x8.rs`、`i32x4.rs`、`i64x2.rs` には、その接頭辞の命令を置く。

`relaxed_simd.c` は同じディレクトリにコミットしてある。`build.rs` は、対象が wasm32 で `simd128` と `relaxed-simd` が両方有効なとき、この C を `-msimd128` と `-mrelaxed-simd` 付きでコンパイルし、`librust_isa_relaxed_simd.a` としてリンクする。

`cargo test` は、Rust と C のシンボルが一致することを確認する。
