# 32-bit ARM の実装可否

対象は `armv7-unknown-linux-gnueabihf` である。このターゲットで調べた feature はすべて不安定である。`thumbv7neon` の既定は、警告が無く `neon` の cfg も立たないので、この表の不安定には入れていない。その属性は「属性には入るが cfg には出ない指定」にある。

不安定の feature は cfg が立たない。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [32-bit ARM](README.md) に書く。

| feature    | 種別   | 判定   | 実装   |
| ---------- | ------ | ------ | ------ |
| `neon`     | SIMD   | 不安定 | 一部   |
| `vfp2`     | SIMD   | 不安定 | 対象外 |
| `vfp3`     | SIMD   | 不安定 | 対象外 |
| `vfp4`     | SIMD   | 不安定 | 対象外 |
| `fp-armv8` | SIMD   | 不安定 | 対象外 |
| `fp16`     | SIMD   | 不安定 | 一部   |
| `d32`      | SIMD   | 不安定 | 対象外 |
| `fpregs`   | SIMD   | 不安定 | 対象外 |
| `mve`      | SIMD   | 不安定 | 対象外 |
| `mve.fp`   | SIMD   | 不安定 | 対象外 |
| `dotprod`  | SIMD   | 不安定 | 一部   |
| `i8mm`     | SIMD   | 不安定 | 一部   |
| `aes`      | 暗号   | 不安定 | 一部   |
| `sha2`     | 暗号   | 不安定 | 一部   |
| `crc`      | 暗号   | 不安定 | 一部   |
| `dsp`      | その他 | 不安定 | 対象外 |

## 属性には入るが cfg には出ない指定

| 指定                                         | rustc で確認したこと                                                       | 確認していないこと | `cfg`           |
| -------------------------------------------- | -------------------------------------------------------------------------- | ------------------ | --------------- |
| `armv7-unknown-linux-gnueabihf` の既定       | 属性は `-neon`                                                             | —                  | `neon` は出ない |
| `thumbv7neon-unknown-linux-gnueabihf` の既定 | `+neon`。警告は無い。`cfg` の集合は `armv7` と同じ。四つの判定には入らない | —                  | `neon` は出ない |
| `armv7` で `-C target-feature=+neon`         | 警告が出て属性は `+neon`                                                   | —                  | 出ない          |

## 確認したこと

- `-C target-feature=+neon` は警告を出し、`neon` の cfg は立たなかった。
- `-mfpu=neon -mfloat-abi=hard` の C は `vadd.i32 q0` を出した。
