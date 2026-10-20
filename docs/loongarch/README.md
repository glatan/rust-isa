# LoongArch

対象は `loongarch64-unknown-linux-gnu` である。`lsx` は既定、`lasx` は安定である。

family は命令の種類に合わせて決める（`simd`、`system` など）。公開パスは `loongarch::{family}::{extension}::{instruction}` とする。

## 実装状況

LSX の `V128`、load / store、`vadd.w`、`vmul.w` を実装してある。`fixtures/loongarch-values` を QEMU-LoongArch で実行し、`vadd.w` と `vmul.w` の zero-cost を確認する。

## 実装方式

- native なので無条件に公開する。任意の feature を要求する命令は `unsafe fn` とする。
- `lsx` と `lasx` は cfg が立つので、呼び出し側のカーネルに `#[target_feature]` を付けられる。ラッパーは `#[inline(always)]` だけにする。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。LASX は `<lasxintrin.h>`、LSX は `<lsxintrin.h>` を使う。
- 値は `V128`（LSX）と `V256`（LASX）をポインタで渡す。
- C は extension ごとに分ける。`-mlsx` と `-mlasx` を使う。
- `asm!` は補助として使える。vector operand の型と ABI は、実装前に確認する。

## ゼロコスト

- 条件付きでゼロコストを確認した。`xvadd.w` と `xvmul.w` が並び、`call` と命令間の store/load が無かった。
- `xvshuf.b` は intrinsics（`__lasx_xvshuf_b`）が必要で、2 命令を合成すると `xvshuf.b` の間に store/load が残った。shuffle 系は命令ごとに zero-cost を確認する。
- C は `-mlasx` を使う。`__lasx_xvld` / `__lasx_xvst` の intrinsics では、インライン後も一時値の store/load が残った。GCC の vector 拡張と通常のロード・ストアを使う。
- `#[target_feature(enable = "lasx")]` をカーネルに付ける。
- LTO モード、C の feature 集合、呼び出し側の feature を一致させる。条件を満たさない場合は `call` が残る。

## ツールチェーン

- ゼロコストは rustc 1.99.1、clang/lld 23.1.1 で確認した。C は `-mlasx` と GCC の vector 拡張を使う。intrinsics では store/load が残った。
- clang、llvm-ar、llvm-objdump。
- clang 23.1.1 で、`-mlasx` の `__lasx_xvadd_w` が `xvld` と `xvadd.w` を出すことを確認した。
- rustc の feature 一覧に、暗号の名前は無かった。

## 実装の順序

1. `lsx`（既定）
2. `lasx`
3. `lbt`、`lamcas`、`scq`

## 未決定

- LASX のベクタ ABI。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
