# 開発者向けの文書

このディレクトリは、リポジトリを変更するための文書を置く。

命令の動作と、利用者が extension を有効にする条件は rustdoc に書く。

文書の役割は次の表にまとめる。「正とする情報」は、その文書だけが持つ情報である。

| 単位種別 | 文書                                                                                                                                                | 正とする情報                                             | 検査                                        |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | ------------------------------------------- |
| 決定     | [実装方針](policy.md)、[スケーラブルベクタと状態](scalable-state.md)                                                                                | 目的、非目標、可用性、セキュリティ、追加手順、決定、保留 | `docs-check`、レビュー                      |
| 実装     | [実装方式](implementation.md)                                                                                                                       | 方式、代替案、ツールチェーン                             | `docs-check`、レビュー                      |
| 契約     | [API契約](api-contract.md)、[API対応](scalable-mapping.md)、[x86のマスク](x86/masking.md)                                                           | APIの形、安全境界、ISAとAPIの対応                        | `doc_names.py`、`docs-check`                |
| 実装状況 | 各archの`README.md`、[実装状況の集計](implementation-status.md)                                                                                     | 公開命令の在庫、未実装命令                               | `inventory-check`、`doc_names.py`           |
| 観測     | [実装可否](feasibility.md)、[ゼロコストの確認](zero-cost.md)                                                                                        | `cfg`の観測、ゼロコストの測定結果                        | `zero-cost-check`、`docs-check`             |
| テスト   | [テストの役割](testing.md)、[doctestの網羅状況](doctest-coverage.md)、[値テストのfixture](fixture-matrix.md)、[nativeの値テスト](native-testing.md) | テストの分類、網羅、fixture、実行条件                    | `doctest-inventory-check`、`fixtures-check` |
| 手順     | [開発環境](development.md)、[文書の位置](documentation.md)、[ソースとテストの置き方](source-layout.md)、[重複する命令名](duplicates.md)             | コマンド、配置、重複の判定                               | `docs-check`、`fmt-check`                   |

## 共通

- [開発環境](development.md)
- [実装方針](policy.md)
- [実装方式](implementation.md)
- [API契約](api-contract.md)
- [実装状況の集計](implementation-status.md)
- [実装可否](feasibility.md)
- [ゼロコストの確認](zero-cost.md)
- [スケーラブルベクタと状態](scalable-state.md)
- [スケーラブル・状態付き extension の API 対応](scalable-mapping.md)
- [テストの役割](testing.md)
- [doctest の網羅状況](doctest-coverage.md)
- [native の値テスト](native-testing.md)
- [値テストの fixture](fixture-matrix.md)
- [重複する命令名](duplicates.md)
- [第三者ソースとライセンス](third-party.md)
- [ソースとテストの置き方](source-layout.md)
- [文書の位置](documentation.md)

## arch ごと

各 arch のディレクトリには、対象・実装方式・命令ごとの実装状況を書いた `README.md` と、`cfg(target_feature)` の観測表を書いた `feasibility.md` を置く。リンクの条件が一般則と異なる arch だけ、`linking.md` に分ける。x86 はゼロコストも含むため `linking-and-zero-cost.md` に置く（[x86](x86/linking-and-zero-cost.md)、[wasm32](wasm32/linking.md)）。

- [x86](x86/README.md)
- [AArch64](aarch64/README.md)
- [32-bit ARM](arm/README.md)
- [PowerPC](powerpc/README.md)
- [RISC-V](riscv/README.md)
- [LoongArch](loongarch/README.md)
- [s390x](s390x/README.md)
- [MIPS](mips/README.md)
- [wasm32](wasm32/README.md)
