# 32-bit ARM

対象は `armv7-unknown-linux-gnueabihf` である。`thumbv7neon-unknown-linux-gnueabihf` も比較に使う。

family は命令の種類に合わせて決める（`simd`、`crypto`、`integer` など）。公開パスは `arm::{family}::{extension}::{instruction}` とする。

## 実装状況

NEON の `V128`、load / store、`vadd.i32`、`vmul.i32` を実装してある。`fixtures/arm-values` を QEMU-ARM で実行する。NEON は不安定なので、利用者はクレート全体を `-C target-feature=+neon` でコンパイルする。

## 実装方式

- 今回測った feature はすべて不安定であり、cfg ゲート方式を使えない。関数を無条件に置き、`unsafe fn` として公開する。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。NEON は `<arm_neon.h>` を使う。
- 値は `V128` をポインタで渡す。
- C は extension ごとに分ける。`-mfpu=neon` と `-mfloat-abi=hard` を使う。
- `asm!` の `qreg` は `u128` を受け付けず、`neon` を要求する。`qreg` が受け付ける `i32x4` などは unstable である。NEON は C + clang を使う。
- NEON の table lookup は 8-byte の `vtbl1_s8` / `vtbl2_s8` だけである。128-bit の `V128` を 1 命令で shuffle する命令は無い。`V128` の shuffle API は出さない。必要なら 8-byte の `V64` を別に設計する。

## ゼロコスト

- 条件付きでゼロコストを確認した。`vadd.i32` と `vmul.i32` が並び、`call` と命令間の store/load が無かった。
- C は `-mthumb -march=armv7-a -mfpu=neon -mfloat-abi=hard` を使う。さらに `-Xclang -target-feature` で feature 集合を Rust に合わせる。
  - 外す: `armv7-a`、`dsp`、`fp64`、`vfp2sp`、`vfp3sp`、`vfp3d16sp`、`read-tp-tpidruro`、`thumb-mode`
  - 足す: `v7`、`thumb2`
- `-C target-feature=+neon` は不安定警告が出る。`#[target_feature]` は使えない。クレート全体を flag 付きでコンパイルする。
- この feature 調整は clang の実装に依存する。build.rs に固定し、disassembly で確認する。

## ツールチェーン

- clang、llvm-ar、llvm-objdump。
- clang 23.1.1 で、`-mfpu=neon -mfloat-abi=hard` の `vaddq_s32` が `vadd.i32 q0` を出すことを確認した。
- ゼロコストの確認では、`-Xclang -target-feature` で C の feature 集合を Rust に合わせた。この調整は clang の実装に依存する。

## 実装の順序

1. `neon`
2. `vfp3`、`vfp4`、`fp-armv8`
3. `fp16`、`dotprod`、`i8mm`
4. `aes`、`sha2`、`crc`
5. `mve`、`mve.fp`（M プロファイル向け。対象を分ける）

## 未決定

- NEON の hard-float ABI と値型の対応。
- MVE を ARMv7 と同じ family に置くか、M プロファイル用に分けるか。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
