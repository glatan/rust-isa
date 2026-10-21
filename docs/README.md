# 開発者向けの文書

このディレクトリは、リポジトリを変更する手順である。命令の動作と、利用者が extension を有効にする条件は rustdoc に書く。

## 共通

- [開発環境](development.md)
- [実装方針](policy.md)
- [実装可否](feasibility.md)
- [ゼロコストの確認](zero-cost.md)
- [スケーラブルベクタと状態](scalable-state.md)
- [native の値テスト](native-testing.md)
- [ソースの置き方](source-layout.md)
- [文書の位置](documentation.md)

## arch ごと

各 arch のディレクトリには、対象・実装方式・命令ごとの実装状況を書いた `README.md` と、`cfg(target_feature)` の観測表を書いた `feasibility.md` を置く。ゼロコストを確認した arch は、リンクの条件を `linking.md` に分ける（例: [x86](x86/linking.md)）。

- [x86](x86/README.md)
- [AArch64](aarch64/README.md)
- [32-bit ARM](arm/README.md)
- [PowerPC](powerpc/README.md)
- [RISC-V](riscv/README.md)
- [LoongArch](loongarch/README.md)
- [s390x](s390x/README.md)
- [MIPS](mips/README.md)
- [wasm32](wasm32/README.md)
