# RISC-V

対象は `riscv64gc-unknown-linux-gnu` である。`riscv64a23-unknown-linux-gnu` も比較に使う。

family は命令の種類に合わせて決める（`simd`、`crypto`、`integer` など）。公開パスは `riscv::{family}::{extension}::{instruction}` とする。

## 実装状況

なし。

## 実装方式

- native なので無条件に公開する。任意の feature を要求する命令は `unsafe fn` とする。
- `zba` などの安定 feature は、呼び出し側のカーネルに `#[target_feature]` を付けられる。ラッパーは `#[inline(always)]` だけにする。V などの不安定 feature は付けず、rustdoc に条件を書く。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。V は `<riscv_vector.h>` を使う。
- 値は `V128` などをポインタで渡す。V はスケーラブルベクタなので、固定長の型では表せない。VL と VTYPE を含めた API を別に設計する。
- C は extension ごとに分ける。`-march=rv64gc_zba` や `-march=rv64gcv` を使う。
- `asm!` は補助として使える。`vreg` は上書きするレジスタとしてだけ指定でき、ベクトル値のオペランドにはできない。
- 関数単位の `__attribute__((target("arch=+v")))` でも C がコンパイルできることを確認した。

## ゼロコスト

- 条件付きでゼロコストを確認した。`vadd.vv` と `vmul.vv` が並び、`call` と命令間の store/load が無かった。
- C は `-march=rv64gcv -mabi=lp64d -mno-relax` を使う。`-mno-relax` が無いとインラインされなかった。
- `__riscv_vle32_v_i32m1` / `__riscv_vse32_v_i32m1` の intrinsics では、インライン後も一時値の store/load が残った。GCC の vector 拡張と通常のロード・ストアを使う。
- `-C target-feature=+v` は不安定警告が出る。`#[target_feature]` は使えない。クレート全体を flag 付きでコンパイルする。
- スケーラブルなスライス API も確認した。C のループが Rust のカーネルへインラインされ、`vsetvli`、`vle32.v`、`vadd.vv`、`vse32.v` が並び、`call` が無かった。
- LTO モード、C の feature 集合、呼び出し側の feature を一致させる。条件を満たさない場合は `call` が残る。

## ツールチェーン

- ゼロコストは rustc 1.99.1、clang/lld 23.1.1 で確認した。C は `-march=rv64gcv -mabi=lp64d -mno-relax` と GCC の vector 拡張を使う。
- clang、llvm-ar、llvm-objdump。
- clang は `riscv64gc-unknown-linux-gnu` を受け付けない。`--target=riscv64-unknown-linux-gnu` と `-march=rv64gc`、拡張を使う場合は `-march=rv64gcv` などにする。
- clang 23.1.1 で、`-march=rv64gcv` の `__riscv_vadd_vv_i32m1` が `vsetvli` と `vadd.vv` を出すことを確認した。
- `riscv64a23-unknown-linux-gnu` の既定は、警告なしで `v`、`f`、`d` を属性に入れるが、cfg は立てない。

## 実装の順序

1. `zba`、`zbb`、`zbc`、`zbs`
2. `zk*` の暗号
3. `zfh`、`zfa`
4. `v`、`zve*`、`zv*`（VL と VTYPE の設計を含む）

## 未決定

- RVV の VL と VTYPE、スケーラブルベクタ型。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
