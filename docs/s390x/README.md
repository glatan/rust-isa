# s390x

対象は `s390x-unknown-linux-gnu` である。既定の cfg に、ベクトル feature は無い。`vector` は安定である。

family は命令の種類に合わせて決める（`simd`、`crypto`、`integer` など）。公開パスは `s390x::{family}::{extension}::{instruction}` とする。

## 実装状況

なし。

## 実装方式

- native なので無条件に公開する。任意の feature を要求する命令は `unsafe fn` とする。
- 安定の feature は cfg が立つので、呼び出し側のカーネルに `#[target_feature]` を付けられる。ラッパーは `#[inline(always)]` だけにする。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。`<vecintrin.h>` を使う。
- 値は `V128` をポインタで渡す。
- C は extension ごとに分ける。`-march=z14` と `-mzvector` を使う。
- `asm!` は補助として使える。vector operand の型と ABI は、実装前に確認する。
- `vec_perm` は z13 の feature 集合で `call` が残り、zero-cost は未確認である。
- s390x はビッグエンディアンである。レーンの並びは命令ごとに rustdoc へ書く。

## ゼロコスト

- 条件付きでゼロコストを確認した。`vaf` と `vmlf` が並び、`call` と命令間の store/load が無かった。
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

## 未決定

- ビッグエンディアンのレーン順序と値型の表現。
- vector-packed-decimal の値型。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
