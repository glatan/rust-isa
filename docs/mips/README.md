# MIPS

対象は `mips64-unknown-linux-gnuabi64` である。この rustc には、このターゲットの標準ライブラリが配布されていない。

family は命令の種類に合わせて決める（`simd`、`system` など）。公開パスは `mips::{family}::{extension}::{instruction}` とする。

## 実装状況

対象は各 extension の全命令である。次の表は、現在公開している命令と、
まだ公開していない命令の一覧である。値テストの列は、その extension の
値テストを実行する環境を示す。未実装命令も同じ表で管理する。

### SIMD

| extension | 実装済み命令                                                                                                                                                                                                                                                                        | 未実装命令                                                                                                                                                                                                                                                                                                                                                                                                                                    | 値テスト |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| `msa`     | `add`, `add_i16`, `add_i64`, `add_i8`, `and`, `cmpeq_i32`, `cmpgt_i32`, `fadd`, `fadd_f64`, `fdiv`, `fdiv_f64`, `fmul`, `fmul_f64`, `fsub`, `fsub_f64`, `mul`, `mul_i16`, `mul_i32`, `or`, `splat_i32`, `sub_i16`, `sub_i32`, `sub_i64`, `sub_i8`, `v128_load`, `v128_store`, `xor` | `div_s`, `div_u`, `mod_s`, `mod_u`, `add_a`, `adds_a`, `adds_s`, `adds_u`, `subs_s`, `subs_u`, `max_s`, `max_u`, `min_s`, `min_u`, `nor.v`, `seq`, `sle_s`, `sle_u`, `slt_s`, `slt_u`, `sll`, `srl`, `sra`, `slli`, `srli`, `srai`, `bclr`, `bset`, `bneg`, `binsl`, `binsr`, `sat_s`, `sat_u`, `sld`, `vshf`, `shf`, `pcnt`, `nlzc`, `nloc`, `fmax`, `fmin`, `fexp2`, `flog2`, `frcp`, `frsqrt`, `fsqrt`, `ftint`, `ffint`, `fclass`, `fcmp` | QEMU     |
| `fp64`    | `fadd_f64`, `fdiv_f64`, `fmul_f64`, `fsub_f64`                                                                                                                                                                                                                                      | なし                                                                                                                                                                                                                                                                                                                                                                                                                                          | QEMU     |
| `virt`    | なし                                                                                                                                                                                                                                                                                | `dvp`, `evp`, `hypcall`, `mfc0`, `mtc0`, `dmfc0`, `dmtc0`, `tlbp`, `tlbr`, `tlbwi`, `tlbwr`, `eret`, `wait`                                                                                                                                                                                                                                                                                                                                   | 対象外   |

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

## 未対応

- `virt`。仮想化命令のため、SIMD の値 API には含めない。
- 追加した MSA 命令のゼロコスト確認。現在の zero-cost は `addv.w`、`mulv.w`、 `vshf.w` だけである。
- rustc の標準ライブラリが無い状態でのテスト方法は、`-Z build-std=core` と QEMU の終了コードで運用している。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
