# simd128

`simd128` は `src/wasm32/simd/simd128` にある。`mod.rs` に共通の契約と再公開を置く。`v128.rs` には `V128` と `v128.*` を置く。`i8x16.rs`、`i16x8.rs`、`i32x4.rs`、`i64x2.rs`、`f32x4.rs`、`f64x2.rs` には、その接頭辞の命令を置く。

`simd128.c` は同じディレクトリにコミットしてある。`build.rs` は、対象が wasm32 で `simd128` が有効なとき、この C を `-msimd128` 付きでコンパイルし、`librust_isa_simd128.a` としてリンクする。

`cargo test` は、Rust と C のシンボルが一致することと、レーン即値の分岐がレーン数と一致することを確認する。
