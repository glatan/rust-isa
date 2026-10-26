# 重複する命令名

同じ arch の中で、同じ命令名が複数の extension に現れることがある。これはエラーではない。公開APIは extension ごとに分け、内部の C 関数だけを共有できるか検討する。

`make duplicates` で、同じ arch の複数 extension に現れる命令名を表示する。

```sh
make duplicates
make duplicates DUPLICATES_ARGS='--arch powerpc'
```

## 判定

次の条件をすべて満たす場合だけ、内部 C 関数を共有する。

- レーンの意味と要素型が同じ。
- 飽和、丸め、NaN の扱いが同じ。
- 必要な feature の集合が同じ、または共有する側がより強い feature を要求する。
- 値型が同じ。`V128` と `V256` のように幅が違う場合は共有しない。

一つでも違う場合は、extension ごとに C 関数を分ける。公開ラッパーは常に extension ごとに置く。

## arch ごとの重複

現在の重複と扱いは、各 arch の `README.md` の実装方式と `make duplicates` の出力を見る。この文書は判定規則だけを置き、arch ごとのスナップショットは持たない。

## 内部Cシンボル

Cシンボルは公開ABIではない。接頭辞はモジュール名に合わせる。

- RISC-V `simd/zv/zvbc`、`zvkg`、`zvkn`、`zvks` は、それぞれ `rust_isa_zvbc_`、`rust_isa_zvkg_`、`rust_isa_zvkn_`、`rust_isa_zvks_` を使う。
- s390x の vector-enhancements-1 は `rust_isa_s390x_ve1_`、vector-enhancements-3 は `rust_isa_s390x_ve3_` を使う。

C関数を共有する場合だけ、共有先の接頭辞を使い、その理由をこの節に書く。
