# AArch64

対象は `aarch64-unknown-linux-gnu` である。`neon` は既定である。

family は命令の種類に合わせて決める（`simd`、`crypto`、`system` など）。公開パスは `aarch64::{family}::{extension}::{instruction}` とする。

## 実装状況

NEON は `V128`、load / store、32-bit 整数の `add` / `mul`、32-bit 浮動小数点の `fadd` / `fmul`、`tbl` を実装してある。`dotprod` は `sdot` を実装してある。

`fixtures/aarch64-values` を `-Z build-std=core` でビルドし、QEMU-AArch64 で値テストを実行する。`kernel_add`、`kernel_tbl`、`kernel_dot` の disassembly で、`add`、`tbl`、`sdot` が隣接し、`call` が無いことを確認する。

## 実装方式

- native なので無条件に公開する。任意の feature を要求する命令は `unsafe fn` とする。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。NEON は `<arm_neon.h>`、SVE は `<arm_sve.h>` を使う。
- 値は `V128` をポインタで渡す。SVE はスケーラブルベクタなので、固定長の `V128` では表せない。SVE の型と API は別に設計する。
- C は extension ごとに分ける。`-march=armv8-a+simd` や `-march=armv8-a+sve` を使う。
- stable な feature は、呼び出し側のカーネルに `#[target_feature(enable = "...")]` を付ける。ラッパーは `#[inline(always)]` だけにし、`#[target_feature]` を付けない。unstable な feature は付けない。
- `asm!` は補助として使えるが、`vreg` は `u128` を受け付けない。`i64x2` や `f128` は unstable である。NEON は C + clang を使う。

## ゼロコスト

- 条件付きでゼロコストを確認した。`#[target_feature(enable = "neon")]` のカーネルで、`add v0.4s`、`mul v0.4s`、`tbl v0.16b`、`sdot v0.4s` が並び、`call` と命令間の store/load が無かった。
- C は `-march=armv8-a+simd -flto=thin`、`__attribute__((always_inline))` を使う。Rust は `profile.release.lto = "thin"` と `-C linker-plugin-lto` を使う。
- LTO モード、C の feature 集合、呼び出し側の feature を一致させる。条件を満たさない場合は `call` が残る。
- カーネルに `#[target_feature]` を付けると、命令間の値がレジスタに残る。ループの外側など、feature の無い関数から呼ぶと、境界の `call` は残る。
- SVE のスライス API では、C のループが `#[target_feature(enable = "sve")]` のカーネルへインラインされ、`cntw`、`whilelt`、`ld1w`、`add z0.s`、`st1w` が並び、`call` が無かった。命令レベルの API ではない。

## ツールチェーン

- ゼロコストは rustc 1.99.1、clang/lld 23.1.1 で確認した。C は `-march=armv8-a+simd -flto=thin`。
- clang、llvm-ar、llvm-objdump。
- clang 23.1.1 で、NEON の `vaddq_s32` が `add v0.4s` を、SVE の `svadd_s32_z` が `add z0.s` を出すことを確認した。
- `-march=armv8-a+neon` は clang では受け付けられない。NEON は `+simd` を使う。
- SVE の関数は `-march=armv8-a+sve` でコンパイルできた。ABI とスケーラブルベクタの受け渡しは未確認である。
- SME は `-march=armv9-a+sme` でコンパイルできた。`__arm_streaming` は宣言子の後ろに置く。関数は `smstart` / `smstop` を出さないため、呼び出し側の guard が必要である。

## 実装の順序

1. `neon`（既定）
2. `fp16`、`bf16`、`dotprod`、`i8mm`
3. `aes`、`sha2`、`sha3`、`sm4`、`crc`
4. `sve`、`sve2`（型と API の設計を含む）
5. `sme`、`sme2`（状態管理の設計を含む）

## 未決定

- SVE / SVE2 のスケーラブルベクタ型と API。
- SME / SME2 のストリーミング状態。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
