# MIPS

対象は `mips64-unknown-linux-gnuabi64` である。この rustc には、このターゲットの標準ライブラリが配布されていない。

family は命令の種類に合わせて決める（`simd`、`system` など）。公開パスは `mips::{family}::{extension}::{instruction}` とする。

## 実装状況

対象は各 extension の命令である。次の表は、現在公開している命令と、
まだ公開していない命令の一覧である。値テストの列は、その extension の
値テストを実行する環境を示す。未実装命令と対象外の命令も同じ表で管理する。

### SIMD

| extension | 実装済み命令                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | 未実装命令                                    | 値テスト |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- | -------- |
| `msa`     | `add`, `add_a`, `add_i16`, `add_i64`, `add_i8`, `adds_a`, `adds_s`, `adds_u`, `and`, `bclr`, `bneg`, `binsl`, `binsr`, `bset`, `cmpeq_i32`, `cmpgt_i32`, `div_s`, `div_u`, `fadd`, `fadd_f64`, `fclass`, `fcmp`, `fdiv`, `fdiv_f64`, `fexp2`, `ffint`, `ffint_s`, `ffint_u`, `flog2`, `fmax`, `fmin`, `fmul`, `fmul_f64`, `frcp`, `frsqrt`, `fsub`, `fsub_f64`, `fsqrt`, `ftint`, `ftint_s`, `ftint_u`, `max_s`, `max_u`, `min_s`, `min_u`, `mod_s`, `mod_u`, `mul`, `mul_i16`, `mul_i32`, `nloc`, `nlzc`, `nor`, `nor_v`, `or`, `pcnt`, `sat_s`, `sat_u`, `seq`, `shf`, `sle_s`, `sle_u`, `sld`, `sll`, `slli`, `slt_s`, `slt_u`, `splat_i32`, `sra`, `srai`, `srl`, `srli`, `sub_i16`, `sub_i32`, `sub_i64`, `sub_i8`, `subs_s`, `subs_u`, `v128_load`, `v128_store`, `vshf`, `xor` | なし                                          | QEMU     |
| `fp64`    | `fadd_f64`, `fdiv_f64`, `fmul_f64`, `fsub_f64`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | なし                                          | QEMU     |
| `virt`    | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | なし（特権・TLB・待機の制御命令のため対象外） | 対象外   |

`msa` の引数の補足:

- `sll`、`srl`、`sra` は、シフト量を各 32-bit lane に持つ `V128` を第 2 引数に取る。
- `slli`、`srli`、`srai` は `const SHIFT: u32` を取り、`0..=31` に制限する。
- `sat_s`、`sat_u` は `const BITS: u32` を取り、`1..=32` に制限する。
- `shf` は `const IMM: u8` を取る。
- `ftint` と `ffint` は `const SIGNED: bool` を取り、`true` なら符号付き、`false` なら符号なしの変換になる。
- `fcmp` は `Fcmp` で条件を選ぶ。真の lane はすべてのビットが 1 になる。
- `sld` の index は `index % 4` を使う。
- `sld`、`vshf`、`shf` は、現在スカラー C で実装している。

`fp64` は `msa` の FP64 サブセットであり、公開パスは `mips::simd::msa` である。

## 実装方式

- `msa` は不安定であり、cfg ゲート方式を使えない。関数を無条件に置き、`unsafe fn` として公開する。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。`<msa.h>` を使う。
- 値は `V128` をポインタで渡す。
- C は extension ごとに分ける。`-mmsa` と `-march=mips64r2` を使う。clang は MSA に r5 以上を要求する警告を出す。
- `asm!` は wasm32 と同様に experimental であり、stable では使えない。

## ゼロコスト

- ゼロコストを確認した。`addv.w`、`mulv.w` が並び、`call` と命令間の store/load が無かった。
- C は `-march=mips64r2 -mmsa -mabicalls -mxgot`、Rust は `-C target-feature=+msa,+fp64` を使う。`-mxgot` と `-mabicalls` が無いと feature 集合が一致しなかった。
- `llvm-objdump` は MSA 命令を `<unknown>` と表示する。バイト列を `llvm-mc --disassemble --triple=mips64-unknown-linux-gnuabi64 --mattr=+msa` でデコードする。
- リンクでは `-Wl,-e,_start` を付ける。付けないと entry が 0 になり、`--gc-sections` でコードが消える。
- 値テストの `_start` は `-mno-abicalls -G0` でコンパイルする。`-mabicalls -mxgot` のままだと、コードサイズによって gp 相対の GOT ロードが未マップのアドレスを指し、QEMU で segfault する。
- 値テストは、lane ごとに独立した C 関数（`c_lane0` など）を使うと通った。共有の静的配列や lane を引数で選ぶ関数は、MIPS の codegen で一致しなかった。

## ツールチェーン

- clang、llvm-ar、llvm-objdump。
- clang 23.1.1 で、`-mmsa` の C がコンパイルできることを確認した。`msa` は MIPS64 revision 5 以上を要求するという警告が出た。
- ゼロコストの確認では、Rust の target-cpu に合わせて C を `-march=mips64r2 -mmsa -mabicalls -mxgot` でコンパイルした。Rust は `-C target-feature=+msa,+fp64` を使った。
- 値テストは `qemu-mips64-static -cpu Loongson-3A4000` で実行する。既定の `MIPS64R2-generic` は MSA の値を正しく返さない。
- rustc の標準ライブラリは配布されていないが、`-Z build-std=core` で `core` をビルドできた。Rust のバイナリはリンクできるが、MIPS64 を読める disassembler が無い。

## 実装の順序

1. `msa`

## 対象外

- `virt`。`dvp`、`evp`、`hypcall`、`mfc0`、`mtc0`、`dmfc0`、`dmtc0`、`tlbp`、`tlbr`、`tlbwi`、`tlbwr`、`eret`、`wait` は、特権・TLB・待機の制御命令のため、SIMD の値 API に含めない。

## 未対応

- 追加した MSA 命令のゼロコスト確認。現在の zero-cost は `addv.w`、`mulv.w` だけである。
- rustc の標準ライブラリが無い状態でのテスト方法は、`-Z build-std=core` と QEMU の終了コードで運用している。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
