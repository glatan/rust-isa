# x86 の実装可否

対象は `x86_64-unknown-linux-gnu` と `i686-unknown-linux-gnu` である。`sse` と `sse2` は両方で既定である。次の表のほかの行は `x86_64-unknown-linux-gnu` で測った。`i686` のほかの feature は測っていない。

既定と安定の feature は cfg が立つ。不安定の feature は立たない。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [x86](README.md) に、LTO とゼロコストは [リンクとゼロコスト](linking.md) に書く。

> 実装状況は [README](README.md) を参照する。この表は cfg の観測だけを扱う。

| feature              | 種別   | 判定   |
| -------------------- | ------ | ------ |
| `sse`                | SIMD   | 既定   |
| `sse2`               | SIMD   | 既定   |
| `sse3`               | SIMD   | 安定   |
| `ssse3`              | SIMD   | 安定   |
| `sse4.1`             | SIMD   | 安定   |
| `sse4.2`             | SIMD   | 安定   |
| `sse4a`              | SIMD   | 安定   |
| `avx`                | SIMD   | 安定   |
| `avx2`               | SIMD   | 安定   |
| `f16c`               | SIMD   | 安定   |
| `fma`                | SIMD   | 安定   |
| `gfni`               | SIMD   | 安定   |
| `avxvnni`            | SIMD   | 安定   |
| `avxvnniint8`        | SIMD   | 安定   |
| `avxvnniint16`       | SIMD   | 安定   |
| `avxifma`            | SIMD   | 安定   |
| `avxneconvert`       | SIMD   | 安定   |
| `avx512f`            | SIMD   | 安定   |
| `avx512dq`           | SIMD   | 安定   |
| `avx512cd`           | SIMD   | 安定   |
| `avx512bw`           | SIMD   | 安定   |
| `avx512vl`           | SIMD   | 安定   |
| `avx512ifma`         | SIMD   | 安定   |
| `avx512vbmi`         | SIMD   | 安定   |
| `avx512vbmi2`        | SIMD   | 安定   |
| `avx512vnni`         | SIMD   | 安定   |
| `avx512bitalg`       | SIMD   | 安定   |
| `avx512vpopcntdq`    | SIMD   | 安定   |
| `avx512vp2intersect` | SIMD   | 安定   |
| `avx512bf16`         | SIMD   | 安定   |
| `avx512fp16`         | SIMD   | 安定   |
| `aes`                | 暗号   | 安定   |
| `sha`                | 暗号   | 安定   |
| `sha512`             | 暗号   | 安定   |
| `sm3`                | 暗号   | 安定   |
| `sm4`                | 暗号   | 安定   |
| `pclmulqdq`          | 暗号   | 安定   |
| `vaes`               | 暗号   | 安定   |
| `vpclmulqdq`         | 暗号   | 安定   |
| `kl`                 | 暗号   | 安定   |
| `widekl`             | 暗号   | 安定   |
| `bmi1`               | 整数   | 安定   |
| `bmi2`               | 整数   | 安定   |
| `adx`                | 整数   | 安定   |
| `lzcnt`              | 整数   | 安定   |
| `popcnt`             | 整数   | 安定   |
| `rdrand`             | 整数   | 安定   |
| `rdseed`             | 整数   | 安定   |
| `movbe`              | 整数   | 安定   |
| `fma4`               | SIMD   | 不安定 |
| `xop`                | SIMD   | 不安定 |
| `avx10.1`            | SIMD   | 不安定 |
| `avx10.2`            | SIMD   | 不安定 |
| `rtm`                | その他 | 不安定 |
| `amx-tile`           | 行列   | 不安定 |
| `amx-int8`           | 行列   | 不安定 |
| `amx-bf16`           | 行列   | 不安定 |
| `amx-fp16`           | 行列   | 不安定 |
| `amx-fp8`            | 行列   | 不安定 |
| `amx-complex`        | 行列   | 不安定 |
| `amx-avx512`         | 行列   | 不安定 |
| `amx-movrs`          | 行列   | 不安定 |

## 確認したこと

- `#[target_feature(enable = "avx10.1")]` と `#[target_feature(enable = "fma4")]` は `E0658` で拒否された。
- `-C target-feature=+avx10.1` は警告を出し、`avx10.1` の cfg は立たなかった。
- `__attribute__((target("avx10.1")))` の C は clang 23.1.1 でコンパイルできた。
- `__attribute__((target("avx2")))` の C は `-mavx2` なしでコンパイルでき、`vpaddd ymm` を出した。
- clang 23.1.3 の `amxavx512intrin.h` / `amxmovrsintrin.h` は `-mamx-avx512 -mavx10.2` / `-mamx-movrs` でコンパイルでき、`tilemovrow`、`tcvtrowd2ps`、`tileloaddrs` を出した。Rust の `asm!` でも同じ命令を出力できた。
