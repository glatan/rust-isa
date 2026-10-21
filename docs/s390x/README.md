# s390x

対象は `s390x-unknown-linux-gnu` である。既定の cfg に、ベクトル feature は無い。`vector` は安定である。

family は命令の種類に合わせて決める（`simd`、`crypto`、`integer` など）。公開パスは `s390x::{family}::{extension}::{instruction}` とする。

## 実装状況

対象は各 extension の全命令である。次の表は、現在公開している命令と、
まだ公開していない命令の一覧である。値テストの列は、その extension の
値テストを実行する環境を示す。未実装命令も同じ表で管理する。

### SIMD

| extension                     | 実装済み命令                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | 未実装命令                                                                                                                  | 値テスト |
| ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | -------- |
| `vector`                      | `add`, `add_i16`, `add_i64`, `add_i8`, `and`, `cmpeq_i32`, `cmpgt_i32`, `fadd`, `fdiv`, `fmul`, `fsub`, `mul`, `mul_i16`, `mul_i32`, `or`, `permute`, `splat_i32`, `sub_i16`, `sub_i32`, `sub_i64`, `sub_i8`, `v128_load`, `v128_store`, `vab`, `vaf`, `vag`, `vah`, `vavg`, `vavgl`, `vceq`, `vch`, `vchl`, `vcksm`, `vcl`, `vclf`, `vclg`, `vclh`, `vfa`, `vfce`, `vfch`, `vfche`, `vfd`, `vfm`, `vfs`, `vfsq`, `vgbm`, `vle`, `vlef`, `vmax`, `vme`, `vmh`, `vmin`, `vml`, `vmo`, `vperm`, `vpkls`, `vpks`, `vsb`, `vsel`, `vsf`, `vsg`, `vsh`, `vsl`, `vsr`, `vst`, `vstef`, `xor` | `vftci`, `vistr`, `vmle`, `vmlh`, `vmlo`, `vrep`, `vseg`, `vsld`, `vsrd`, `vstr`, `vstrl`, `vuph`, `vupl`, `vuplh`, `vupll` | QEMU     |
| `vector-enhancements-1`       | `cntlz_i32`, `fma`, `popcnt_i32`, `popcnt_i8`, `vclz`, `vctz`, `vfa`, `vfce`, `vfch`, `vfche`, `vfd`, `vfm`, `vfma`, `vfms`, `vfnma`, `vfnms`, `vfs`, `vfsq`, `vpopct`                                                                                                                                                                                                                                                                                                                                                                                                                 | `vftci`, `vistr`, `vstrl`, `vstr`                                                                                           | QEMU     |
| `vector-enhancements-2` / `3` | `vclzd`, `vclzg`, `vctzd`, `vctzg`, `vpopcntd`, `vpopcntg`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `vfae`, `vfee`, `vfene`, `vgef`, `vgeg`, `vistr`, `vlgv`, `vscbi`, `vstr`, `vstrl`                                          | 未実行   |
| `vector-packed-decimal` 系    | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `vpab`, `vpsrp`, `vpk`, `vupk`, `vcsd`, `vcsb`, `vcvd`, `vcvb`, `vistr`, `vstr`                                             | 未実行   |
| `nnp-assist`                  | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `vnn`, `vflr`, `vfli`, `vfsq`, `vistr`, `vstr`, `vuph`, `vupl`, `vfll`, `vflr`                                              | 未実行   |
| `message-security-assist-*`   | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 状態命令のため命令 API にしない                                                                                             | 対象外   |

## 実装方式

- native なので無条件に公開する。任意の feature を要求する命令は `unsafe fn` とする。
- 安定の feature は cfg が立つので、呼び出し側のカーネルに `#[target_feature]` を付けられる。ラッパーは `#[inline(always)]` だけにする。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。`<vecintrin.h>` を使う。
- 値は `V128` をポインタで渡す。
- C は extension ごとに分ける。`-march=z14` と `-mzvector` を使う。
- `asm!` は補助として使える。vector operand の型と ABI は、実装前に確認する。
- `vec_perm` も zero-cost を確認した。C は target-cpu を `z10` にし、`+vector` を明示すると Rust の feature 集合と一致する。
- s390x はビッグエンディアンである。レーンの並びは命令ごとに rustdoc へ書く。

## ゼロコスト

- 条件付きでゼロコストを確認した。`vaf`、`vmlf`、`vperm` が並び、`call` と命令間の store/load が無かった。
- C は `-march=z10 -mzvector` を使う。Rust の target-cpu は `z10` である。`-march=z13 -mzvector` では `+vector-enhancements-1` と `+transactional-execution` が付き、インラインされなかった。
- C は GCC の vector 拡張と通常のロード・ストアで書く。
- `llvm-objdump` は s390x のベクタ命令を表示しない。バイト列を `llvm-mc --disassemble --triple=s390x-unknown-linux-gnu --mattr=+vector` でデコードする。
- LTO モード、C の feature 集合、呼び出し側の feature を一致させる。条件を満たさない場合は `call` が残る。

## ツールチェーン

- ゼロコストは rustc 1.99.1、clang/lld 23.1.1 で確認した。C は `-march=z10 -mzvector` と GCC の vector 拡張を使う。`llvm-objdump` はベクタ命令を表示しないため、`llvm-mc` でデコードする。
- clang、llvm-ar、llvm-objdump。
- clang 23.1.1 で、`-march=z14 -mzvector` の C がコンパイルできることを確認した。`llvm-objdump` は命令名を表示しなかったため、命令の確認は `llvm-mc` か実機で行う。
- `llvm-objdump` のバージョンと s390x の対応を確認する。

## 実装の順序

1. `vector`
2. `vector-enhancements-1`、`2`、`3`
3. `vector-packed-decimal` 系
4. `nnp-assist`
5. `message-security-assist-extension*`（不安定）

## 未対応

- `vector-enhancements-2` / `3`。
- `vector-packed-decimal` とその enhancement。
- `nnp-assist`。
- `message-security-assist-extension*` は状態を持つため、値 API にしない。
- `transactional-execution` と `guarded-storage` は制御・状態の命令のため、
  値 API にしない。
- 追加した vector 命令のゼロコスト確認。現在の zero-cost は `vaf`、`vmlf`、 `vperm` だけである。
- ビッグエンディアンのレーン順序は rustdoc に書いた。値型は `V128` の
  バイト列のままである。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
