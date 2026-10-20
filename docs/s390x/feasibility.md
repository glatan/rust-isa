# s390x の実装可否

対象は `s390x-unknown-linux-gnu` である。既定の cfg に、ベクトル feature は無い。

安定の feature は cfg が立つ。不安定の feature は立たない。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [s390x](README.md) に書く。

| feature | 種別 | 判定 | 実装 |
| --- | --- | --- | --- |
| `vector` | SIMD | 安定 | 一部 |
| `vector-enhancements-1` | SIMD | 安定 | 未 |
| `vector-enhancements-2` | SIMD | 安定 | 未 |
| `vector-enhancements-3` | SIMD | 安定 | 未 |
| `vector-packed-decimal` | SIMD | 安定 | 未 |
| `vector-packed-decimal-enhancement` | SIMD | 安定 | 未 |
| `vector-packed-decimal-enhancement-2` | SIMD | 安定 | 未 |
| `vector-packed-decimal-enhancement-3` | SIMD | 安定 | 未 |
| `nnp-assist` | その他 | 安定 | 未 |
| `message-security-assist-extension3` | 暗号 | 不安定 | 未 |
| `message-security-assist-extension4` | 暗号 | 不安定 | 未 |
| `message-security-assist-extension5` | 暗号 | 不安定 | 未 |
| `message-security-assist-extension8` | 暗号 | 不安定 | 未 |
| `message-security-assist-extension9` | 暗号 | 不安定 | 未 |
| `message-security-assist-extension12` | 暗号 | 不安定 | 未 |
| `transactional-execution` | その他 | 不安定 | 未 |
| `guarded-storage` | その他 | 不安定 | 未 |

## 確認したこと

- `-C target-feature=+vector` は警告を出さず、`vector` の cfg が立った。
- `-march=z14 -mzvector` の C はコンパイルできた。`llvm-objdump` は命令名を表示しなかった。
