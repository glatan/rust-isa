# rust-isa

`rust-isa` は、arch ごとの ISA 拡張を置く `no_std` クレートである。`edition` は 2024、`rust-version` は 1.99.1 である。`core::arch` は使わない。

公開パスは `{arch}::{family}::{extension}::{instruction}` である。`family` は名前空間で、命令は extension から再公開する。

```rust
use rust_isa::wasm32::simd::simd128::{f32x4_add, V128};
```

命令の動作と、その extension を有効にする条件は、各モジュールの rustdoc に英語で書いてある。このリポジトリを変更する手順は [CONTRIBUTING.md](CONTRIBUTING.md) に、実装方針は [docs/policy.md](docs/policy.md) にある。

native でゼロコストを要求する場合の LTO の条件は [docs/zero-cost.md](docs/zero-cost.md) に、arch ごとのフラグと確認結果は各 arch の文書にある（例: [x86](docs/x86/linking-and-zero-cost.md)）。

## 利用上の保証

| 項目         | 保証                                                           |
| ------------ | -------------------------------------------------------------- |
| `no_std`     | `alloc` なし。実行時ディスパッチなし。                         |
| `core::arch` | 型・intrinsic・feature 検出を使わない。                        |
| native       | 無条件公開。任意 feature は `unsafe`。CPU 確認は利用者の責任。 |
| wasm32       | `simd128` などの cfg ゲートで公開。                            |
| ゼロコスト   | LTO 条件を満たすビルドのみ。既定は呼び出しベース。             |

詳細は [実装方針](docs/policy.md)、[ゼロコストの確認](docs/zero-cost.md)、[テストの役割](docs/testing.md) を見る。

## ライセンス

MIT OR Apache-2.0
