# relaxed-simd

`relaxed_simd.c` は `src/wasm32/simd/relaxed_simd` にコミットしてある。`build.rs` は、対象が wasm32 で `simd128` と `relaxed-simd` が両方有効なとき、この C を `-msimd128` と `-mrelaxed-simd` 付きでコンパイルし、`librust_isa_relaxed_simd.a` としてリンクする。

ファイルとテストの置き方は [ソースとテストの置き方](../source-layout.md)、テストの役割は [テストの役割](../testing.md) にある。
