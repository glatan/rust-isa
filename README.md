# rust-isa

`rust-isa` は、arch ごとの ISA 拡張を置く `no_std` クレートである。`edition` は 2024、`rust-version` は 1.99.1 である。`core::arch` は使わない。

公開パスは `{arch}::{family}::{extension}::{instruction}` である。`family` は名前空間で、命令は extension から再公開する。

```rust
use rust_isa::wasm32::simd::simd128::{f32x4_add, V128};
```

命令の動作と、その extension を有効にする条件は、各モジュールの rustdoc に英語で書いてある。このリポジトリを変更する手順は [CONTRIBUTING.md](CONTRIBUTING.md) に、実装方針は [docs/policy.md](docs/policy.md) にある。

native でゼロコストを要求する場合の LTO の条件は [docs/zero-cost.md](docs/zero-cost.md) に、arch ごとのフラグと確認結果は各 arch の文書にある（例: [x86](docs/x86/linking-and-zero-cost.md)）。

## ライセンス

MIT OR Apache-2.0
