# MIPS の実装可否

対象は `mips64-unknown-linux-gnuabi64` である。この rustc には、このターゲットの標準ライブラリが配布されていない。

不安定の feature は cfg ゲート方式を使えない。実装方式は [MIPS](README.md) に書く。

| feature | 種別   | 判定   | 実装 |
| ------- | ------ | ------ | ---- |
| `msa`   | SIMD   | 不安定 | 一部 |
| `virt`  | その他 | 不安定 | 未   |
| `fp64`  | その他 | 不安定 | 一部 |

## 確認したこと

- `-mmsa` の C はコンパイルできた。`msa` は MIPS64 revision 5 以上を要求するという警告が出た。
- `asm!` は stable では使えない。
