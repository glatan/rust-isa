# MIPS の実装可否

対象は `mips64-unknown-linux-gnuabi64` である。この rustc には、このターゲットの標準ライブラリが配布されていない。

不安定の feature は cfg ゲート方式を使えない。実装方式は [MIPS](README.md) に書く。

> 実装状況は [README](README.md) を参照する。この表は cfg の観測だけを扱う。

| feature | 種別   | 判定   |
| ------- | ------ | ------ |
| `msa`   | SIMD   | 不安定 |
| `virt`  | その他 | 不安定 |
| `fp64`  | その他 | 不安定 |

`virt` は特権・TLB・待機の制御命令のため、値 API にしない。

## 確認したこと

- `-mmsa` の C はコンパイルできた。`msa` は MIPS64 revision 5 以上を要求するという警告が出た。
- `asm!` は stable では使えない。
