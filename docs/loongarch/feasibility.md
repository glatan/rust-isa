# LoongArch の実装可否

対象は `loongarch64-unknown-linux-gnu` である。rustc の feature 一覧に、暗号の名前は無かった。

既定と安定の feature は cfg が立つ。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [LoongArch](README.md) に書く。

| feature  | 種別   | 判定 | 実装 |
| -------- | ------ | ---- | ---- |
| `lsx`    | SIMD   | 既定 | 一部 |
| `lasx`   | SIMD   | 安定 | 一部 |
| `lbt`    | その他 | 安定 | 未   |
| `lamcas` | その他 | 安定 | 未   |
| `scq`    | その他 | 安定 | 未   |

## 確認したこと

- `-C target-feature=+lasx` は警告を出さず、`lasx` の cfg が立った。
- `-mlasx` の C は `xvld` と `xvadd.w` を出した。
