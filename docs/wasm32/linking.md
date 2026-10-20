# wasm32 のリンク

[実装方針](../policy.md) と [wasm32](README.md) のとおり、clang の bitcode は呼び出し側へインラインされる。wasm32 では、その結果として演算と演算のあいだにある `v128` の往復が消える。

`wasm-opt` はライブラリのコンパイルには使わない。リンク後の最終バイナリに `wasm-opt -O3` を掛ける後処理である。`-C linker-plugin-lto` が無いリンクでは、`wasm-opt -O3` は C 関数への `call` を消すが、結果スロットの `v128.store` と、それを読む `v128.load` は残る。

`cargo test` は `fixtures/simd128-link` をリンクし、`wasm-opt -O3` のあとで `f32x4.add` と `f32x4.relaxed_madd` を確認する。それぞれの命令は残り、C シンボルへの `call` と、演算と演算のあいだの `v128.store` は残らない。

`simd128` を外した wasm32 ビルドでは、`rust_isa_simd128_` と `rust_isa_relaxed_simd_` のシンボルが出ないことも確認する。
