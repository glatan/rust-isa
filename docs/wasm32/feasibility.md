# wasm32 の実装可否

対象は `wasm32-unknown-unknown` である。`simd128` と `relaxed-simd` は既定ではないが、安定である。

wasm32 では、モジュールが含む命令の feature を全体として要求する。このため、cfg ゲートを外して関数を無条件に置くと、feature が無効なビルドにも feature を要求する。`simd128` と `relaxed-simd` は cfg ゲート方式で実装してある。

| feature | 種別 | 判定 | 実装 | 備考 |
| --- | --- | --- | --- | --- |
| `simd128` | SIMD | 安定 | 済み | 既定ではない |
| `relaxed-simd` | SIMD | 安定 | 済み | 既定ではない |
| `bulk-memory` | 制御 | 既定 | 未 | rustc がコピーから出す。命令 API にはしない |
| `multivalue` | 制御 | 既定 | 未 | 命令 API にはしない |
| `mutable-globals` | 制御 | 既定 | 未 | 命令 API にはしない |
| `nontrapping-fptoint` | 制御 | 既定 | 未 | rustc がキャストから出す。命令 API にはしない |
| `reference-types` | 制御 | 既定 | 未 | 命令 API にはしない |
| `sign-ext` | 制御 | 既定 | 未 | rustc がキャストから出す。命令 API にはしない |
| `tail-call` | 制御 | 安定 | 未 | 命令 API にはしない |
| `extended-const` | 制御 | 安定 | 未 | 命令 API にはしない |
| `wide-arithmetic` | 整数 | 不安定 | 未 | 未実装。Cargo feature `wide-arithmetic` で opt-in する方針 |
| `atomics` | その他 | 不安定 | 未 |  |
| `exception-handling` | 制御 | 不安定 | 未 | 命令 API にはしない |
| `gc` | 制御 | 不安定 | 未 | 命令 API にはしない |
| `fp16` | SIMD | LLVM のみ | 未 |  |

## 確認したこと

- `-C target-feature=+wide-arithmetic` は警告を出し、`wide-arithmetic` の cfg は立たなかった。
- `-mwide-arithmetic` の C は clang 23.1.1 でコンパイルできた。モジュールには `i64.add128` と `target_features` の `wide-arithmetic` が入る。
- この環境の `wasm-validate`、`wasm-opt`、`wasmtime` 49.0.2 は `wide-arithmetic` を既定で受け付けた。古い実行環境では拒否されうる。
- `wide-arithmetic` は、cfg ゲート方式では関数を出せない。C を無条件にリンクすると、モジュール全体が `wide-arithmetic` を要求する。Cargo feature `wide-arithmetic` で opt-in する方針である。詳しくは [wasm32](README.md) に書く。

## ゼロコスト

呼び出し側が `-C target-feature=+simd128,+relaxed-simd` と `-C linker-plugin-lto` を付けたとき、C の bitcode が呼び出し側へインラインされ、`v128` の往復が消える。確認は [wasm32 のリンク](linking.md) に書く。
