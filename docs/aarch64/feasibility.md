# AArch64 の実装可否

対象は `aarch64-unknown-linux-gnu` である。`-neon` を付けると、このターゲットの ABI を実装するには `neon` が必要だという警告が出る。

`neon` は既定であり、安定の feature は cfg が立つ。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [AArch64](README.md) に書く。

> 実装状況は [README](README.md) を参照する。この表は cfg の観測だけを扱う。

| feature        | 種別   | 判定   | 備考                                               |
| -------------- | ------ | ------ | -------------------------------------------------- |
| `neon`         | SIMD   | 既定   |                                                    |
| `fp16`         | SIMD   | 安定   |                                                    |
| `bf16`         | SIMD   | 安定   |                                                    |
| `dotprod`      | SIMD   | 安定   |                                                    |
| `i8mm`         | SIMD   | 安定   |                                                    |
| `f32mm`        | SIMD   | 安定   |                                                    |
| `f64mm`        | SIMD   | 安定   |                                                    |
| `fcma`         | SIMD   | 安定   |                                                    |
| `fhm`          | SIMD   | 安定   |                                                    |
| `rdm`          | SIMD   | 安定   |                                                    |
| `jsconv`       | SIMD   | 安定   |                                                    |
| `sve`          | SIMD   | 安定   | スケーラブルベクタ。スライス API を使う            |
| `sve2`         | SIMD   | 安定   | SVE2 用の API は提供しないため対象外               |
| `aes`          | 暗号   | 安定   |                                                    |
| `sha2`         | 暗号   | 安定   |                                                    |
| `sha3`         | 暗号   | 安定   |                                                    |
| `sm4`          | 暗号   | 安定   |                                                    |
| `crc`          | 暗号   | 安定   |                                                    |
| `lse`          | その他 | 安定   |                                                    |
| `rcpc`         | その他 | 安定   |                                                    |
| `rcpc2`        | その他 | 安定   |                                                    |
| `dit`          | その他 | 安定   |                                                    |
| `flagm`        | その他 | 安定   |                                                    |
| `bti`          | その他 | 安定   | 分岐保護。命令ごとの API にはしない                |
| `paca`         | その他 | 安定   | ポインタ認証。命令ごとの API にはしない            |
| `pacg`         | その他 | 安定   | ポインタ認証。命令ごとの API にはしない            |
| `mte`          | その他 | 安定   | メモリタグ。命令ごとの API にはしない              |
| `faminmax`     | SIMD   | 不安定 |                                                    |
| `sve2p1`       | SIMD   | 不安定 | SVE2.1 も SVE2 と同じく対象外                      |
| `sve-b16b16`   | SIMD   | 不安定 |                                                    |
| `fp8`          | SIMD   | 不安定 |                                                    |
| `fp8fma`       | SIMD   | 不安定 |                                                    |
| `fp8dot2`      | SIMD   | 不安定 |                                                    |
| `fp8dot4`      | SIMD   | 不安定 |                                                    |
| `lut`          | SIMD   | 不安定 |                                                    |
| `lse2`         | その他 | 不安定 |                                                    |
| `lse128`       | その他 | 不安定 |                                                    |
| `rcpc3`        | その他 | 不安定 |                                                    |
| `sme`          | 行列   | 不安定 | ストリーミング状態は guard で管理する。ZA は対象外 |
| `sme2`         | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme2p1`       | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme-f64f64`   | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme-i16i64`   | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme-fa64`     | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme-f16f16`   | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme-b16b16`   | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme-f8f32`    | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme-f8f16`    | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `sme-lutv2`    | 行列   | 不安定 | ZA タイルを要求するため対象外                      |
| `ssve-fp8fma`  | 行列   | 不安定 | 状態管理の設計が必要                               |
| `ssve-fp8dot2` | 行列   | 不安定 | 状態管理の設計が必要                               |
| `ssve-fp8dot4` | 行列   | 不安定 | 状態管理の設計が必要                               |

## 確認したこと

- `-C target-feature=+sve` は警告を出さず、`sve` の cfg が立った。`+sve` で `fp16` の cfg も立った。
- `-C target-feature=+sme` は警告を出し、`sme` の cfg は立たなかった。
- SME は `-march=armv9-a+sme` と `__arm_streaming` で C がコンパイルできた。`-C target-feature=+sme` の cfg は立たない。
