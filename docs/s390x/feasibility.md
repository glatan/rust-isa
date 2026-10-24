# s390x の実装可否

対象は `s390x-unknown-linux-gnu` である。既定の cfg に、ベクトル feature は無い。

安定の feature は cfg が立つ。不安定の feature は立たない。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [s390x](README.md) に書く。

> 実装状況は [README](README.md) を参照する。この表は cfg の観測だけを扱う。

| feature                               | 種別   | 判定   |
| ------------------------------------- | ------ | ------ |
| `vector`                              | SIMD   | 安定   |
| `vector-enhancements-1`               | SIMD   | 安定   |
| `vector-enhancements-2`               | SIMD   | 安定   |
| `vector-enhancements-3`               | SIMD   | 安定   |
| `vector-packed-decimal`               | SIMD   | 安定   |
| `vector-packed-decimal-enhancement`   | SIMD   | 安定   |
| `vector-packed-decimal-enhancement-2` | SIMD   | 安定   |
| `vector-packed-decimal-enhancement-3` | SIMD   | 安定   |
| `nnp-assist`                          | その他 | 安定   |
| `message-security-assist-extension3`  | 暗号   | 不安定 |
| `message-security-assist-extension4`  | 暗号   | 不安定 |
| `message-security-assist-extension5`  | 暗号   | 不安定 |
| `message-security-assist-extension8`  | 暗号   | 不安定 |
| `message-security-assist-extension9`  | 暗号   | 不安定 |
| `message-security-assist-extension12` | 暗号   | 不安定 |
| `transactional-execution`             | その他 | 不安定 |
| `guarded-storage`                     | その他 | 不安定 |

## 確認したこと

- `-C target-feature=+vector` は警告を出さず、`vector` の cfg が立った。
- `-march=z14 -mzvector` の C はコンパイルできた。`llvm-objdump` は命令名を表示しなかった。
- `vector-enhancements-2`、`vector-packed-decimal`、`nnp-assist` は `#[target_feature]` と `asm!` で扱える。
- `-march=z17 -fzvector` の C は `vaq`、`vclzq`、`vblendb`、`vgemq`、`vdq` を出した。QEMU は vector-enhancements-3 を実行できないため、値テストは未実行である。
