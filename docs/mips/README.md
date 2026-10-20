# MIPS

対象は `mips64-unknown-linux-gnuabi64` である。この rustc には、このターゲットの標準ライブラリが配布されていない。

family は命令の種類に合わせて決める（`simd`、`system` など）。公開パスは `mips::{family}::{extension}::{instruction}` とする。

## 実装状況

MSA の `V128`、load / store、`addv.w`、`mulv.w` を実装してある。`fixtures/mips-values` を QEMU-MIPS64 で実行する。`_start` は C で書き、終了コードで合否を返す。

## 実装方式

- `msa` は不安定であり、cfg ゲート方式を使えない。関数を無条件に置き、`unsafe fn` として公開する。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。`<msa.h>` を使う。
- 値は `V128` をポインタで渡す。
- C は extension ごとに分ける。`-mmsa` と `-march=mips64r5` を使う。
- `asm!` は wasm32 と同様に experimental であり、stable では使えない。

## ゼロコスト

- ゼロコストを確認した。`addv.w`、`mulv.w`、`vshf.w` が並び、`call` と命令間の store/load が無かった。
- C は `-march=mips64r2 -mmsa -mabicalls -mxgot`、Rust は `-C target-feature=+msa,+fp64` を使う。`-mxgot` と `-mabicalls` が無いと feature 集合が一致しなかった。
- `llvm-objdump` は MSA 命令を `<unknown>` と表示する。バイト列を `llvm-mc --disassemble --triple=mips64-unknown-linux-gnuabi64 --mattr=+msa` でデコードする。
- リンクでは `-Wl,-e,_start` を付ける。付けないと entry が 0 になり、`--gc-sections` でコードが消える。
- 値テストは、lane ごとに独立した C 関数（`c_lane0` など）を使うと通った。共有の静的配列や lane を引数で選ぶ関数は、MIPS の codegen で一致しなかった。
- shuffle の `vshf.w` も zero-cost を確認した。

## ツールチェーン

- clang、llvm-ar、llvm-objdump。
- clang 23.1.1 で、`-mmsa` の C がコンパイルできることを確認した。`msa` は MIPS64 revision 5 以上を要求するという警告が出た。
- ゼロコストの確認では、Rust の target-cpu に合わせて C を `-march=mips64r2 -mmsa -mabicalls -mxgot` でコンパイルした。Rust は `-C target-feature=+msa,+fp64` を使った。
- rustc の標準ライブラリは配布されていないが、`-Z build-std=core` で `core` をビルドできた。Rust のバイナリはリンクできるが、MIPS64 を読める disassembler が無い。

## 実装の順序

1. `msa`
2. `virt`

## 未決定

- rustc の標準ライブラリが無い状態でのテスト方法。
- native のゼロコスト。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
