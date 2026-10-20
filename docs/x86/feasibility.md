# x86 の実装可否

対象は `x86_64-unknown-linux-gnu` と `i686-unknown-linux-gnu` である。`sse` と `sse2` は両方で既定である。`i686` の `sse2` はゼロコストを確認した。次の表のほかの行は `x86_64-unknown-linux-gnu` で測った。`i686` のほかの feature は測っていない。

既定と安定の feature は cfg が立つ。不安定の feature は立たない。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [x86](README.md) に、LTO とゼロコストは [リンクとゼロコスト](linking.md) に書く。

| feature | 種別 | 判定 | 実装 |
| --- | --- | --- | --- |
| `sse` | SIMD | 既定 | 未 |
| `sse2` | SIMD | 既定 | 未 |
| `sse3` | SIMD | 安定 | 未 |
| `ssse3` | SIMD | 安定 | 未 |
| `sse4.1` | SIMD | 安定 | 未 |
| `sse4.2` | SIMD | 安定 | 未 |
| `sse4a` | SIMD | 安定 | 未 |
| `avx` | SIMD | 安定 | 未 |
| `avx2` | SIMD | 安定 | 未 |
| `f16c` | SIMD | 安定 | 未 |
| `fma` | SIMD | 安定 | 未 |
| `gfni` | SIMD | 安定 | 未 |
| `avxvnni` | SIMD | 安定 | 未 |
| `avxvnniint8` | SIMD | 安定 | 未 |
| `avxvnniint16` | SIMD | 安定 | 未 |
| `avxifma` | SIMD | 安定 | 未 |
| `avxneconvert` | SIMD | 安定 | 未 |
| `avx512f` | SIMD | 安定 | 未 |
| `avx512dq` | SIMD | 安定 | 未 |
| `avx512cd` | SIMD | 安定 | 未 |
| `avx512bw` | SIMD | 安定 | 未 |
| `avx512vl` | SIMD | 安定 | 未 |
| `avx512ifma` | SIMD | 安定 | 未 |
| `avx512vbmi` | SIMD | 安定 | 未 |
| `avx512vbmi2` | SIMD | 安定 | 未 |
| `avx512vnni` | SIMD | 安定 | 未 |
| `avx512bitalg` | SIMD | 安定 | 未 |
| `avx512vpopcntdq` | SIMD | 安定 | 未 |
| `avx512vp2intersect` | SIMD | 安定 | 未 |
| `avx512bf16` | SIMD | 安定 | 未 |
| `avx512fp16` | SIMD | 安定 | 未 |
| `aes` | 暗号 | 安定 | 未 |
| `sha` | 暗号 | 安定 | 未 |
| `sha512` | 暗号 | 安定 | 未 |
| `sm3` | 暗号 | 安定 | 未 |
| `sm4` | 暗号 | 安定 | 未 |
| `pclmulqdq` | 暗号 | 安定 | 未 |
| `vaes` | 暗号 | 安定 | 未 |
| `vpclmulqdq` | 暗号 | 安定 | 未 |
| `kl` | 暗号 | 安定 | 未 |
| `widekl` | 暗号 | 安定 | 未 |
| `bmi1` | 整数 | 安定 | 未 |
| `bmi2` | 整数 | 安定 | 未 |
| `adx` | 整数 | 安定 | 未 |
| `lzcnt` | 整数 | 安定 | 未 |
| `popcnt` | 整数 | 安定 | 未 |
| `rdrand` | 整数 | 安定 | 未 |
| `rdseed` | 整数 | 安定 | 未 |
| `movbe` | 整数 | 安定 | 未 |
| `fma4` | SIMD | 不安定 | 未 |
| `xop` | SIMD | 不安定 | 未 |
| `avx10.1` | SIMD | 不安定 | 未 |
| `avx10.2` | SIMD | 不安定 | 未 |
| `rtm` | その他 | 不安定 | 未 |
| `amx-tile` | 行列 | 不安定 | 未 |
| `amx-int8` | 行列 | 不安定 | 未 |
| `amx-bf16` | 行列 | 不安定 | 未 |
| `amx-fp16` | 行列 | 不安定 | 未 |
| `amx-fp8` | 行列 | 不安定 | 未 |
| `amx-complex` | 行列 | 不安定 | 未 |
| `amx-avx512` | 行列 | 不安定 | 未 |
| `amx-movrs` | 行列 | 不安定 | 未 |

## 確認したこと

- `#[target_feature(enable = "avx10.1")]` と `#[target_feature(enable = "fma4")]` は `E0658` で拒否された。
- `-C target-feature=+avx10.1` は警告を出し、`avx10.1` の cfg は立たなかった。
- `__attribute__((target("avx10.1")))` の C は clang 23.1.1 でコンパイルできた。
- `__attribute__((target("avx2")))` の C は `-mavx2` なしでコンパイルでき、`vpaddd ymm` を出した。
