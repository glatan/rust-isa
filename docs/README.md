# 開発者向けの文書

このディレクトリは、リポジトリを変更するための文書を置く。決定は [実装方針](policy.md)、API の形は [API契約](api-contract.md)、実装状況は各 arch の `README.md`、観測は [実装可否](feasibility.md) と [ゼロコストの確認](zero-cost.md)、手順は [開発環境](development.md) に置く。命令の動作と、利用者が extension を有効にする条件は rustdoc に書く。

文書は、決定（`policy.md`、`scalable-state.md`）、契約（`api-contract.md`、`scalable-mapping.md`、`x86/masking.md`）、実装状況（arch の `README.md`、`coverage.md`）、観測（`feasibility.md`、`zero-cost.md`）、テスト（`testing.md`、`doctest-coverage.md`、`fixture-matrix.md`、`native-testing.md`）、手順（`development.md`、`documentation.md`、`source-layout.md`、`dedup.md`）に分かれる。

## 共通

- [開発環境](development.md)
- [実装方針](policy.md)
- [API契約](api-contract.md)
- [実装状況の集計](coverage.md)
- [実装可否](feasibility.md)
- [ゼロコストの確認](zero-cost.md)
- [スケーラブルベクタと状態](scalable-state.md)
- [スケーラブル・状態付き extension の API 対応](scalable-mapping.md)
- [テストの役割](testing.md)
- [doctest の網羅状況](doctest-coverage.md)
- [native の値テスト](native-testing.md)
- [値テストの fixture](fixture-matrix.md)
- [重複する命令名](dedup.md)
- [ソースとテストの置き方](source-layout.md)
- [文書の位置](documentation.md)

## arch ごと

各 arch のディレクトリには、対象・実装方式・命令ごとの実装状況を書いた `README.md` と、`cfg(target_feature)` の観測表を書いた `feasibility.md` を置く。リンクの条件が一般則と異なる arch だけ、`linking.md` に分ける（例: [x86](x86/linking.md)、[wasm32](wasm32/linking.md)）。

- [x86](x86/README.md)
- [AArch64](aarch64/README.md)
- [32-bit ARM](arm/README.md)
- [PowerPC](powerpc/README.md)
- [RISC-V](riscv/README.md)
- [LoongArch](loongarch/README.md)
- [s390x](s390x/README.md)
- [MIPS](mips/README.md)
- [wasm32](wasm32/README.md)
