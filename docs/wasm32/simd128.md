# simd128

`simd128.c` は `src/wasm32/simd/simd128` にコミットしてある。`build.rs` は、対象が wasm32 で `simd128` が有効なとき、この C を `-msimd128` 付きでコンパイルし、`librust_isa_simd128.a` としてリンクする。

ファイルとテストの置き方は [ソースとテストの置き方](../source-layout.md)、テストの役割は [テストの役割](../testing.md) にある。
