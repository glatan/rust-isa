# 32-bit ARM

対象は `armv7-unknown-linux-gnueabihf` である。`thumbv7neon-unknown-linux-gnueabihf` も比較に使う。

family は命令の種類に合わせて決める（`simd`、`crypto`、`integer` など）。公開パスは `arm::{family}::{extension}::{instruction}` とする。

## 実装状況

対象は各 extension の全命令である。次の表は、現在公開している命令と、
まだ公開していない命令の一覧である。値テストの列は、その extension の
値テストを実行する環境を示す。未実装命令も同じ表で管理する。

### SIMD

| extension                             | 実装済み命令                                                                                                                                                                                                                                                                                                                    | 未実装命令                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 値テスト        |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------- |
| `neon`                                | `add`, `add_i16`, `add_i64`, `add_i8`, `and`, `cmpeq_i16`, `cmpeq_i32`, `cmpeq_i8`, `cmpgt_i16`, `cmpgt_i32`, `cmpgt_i8`, `eor`, `fadd`, `fmul`, `fsub`, `mul`, `mul_i16`, `mul_i32`, `orr`, `splat_i16`, `splat_i32`, `splat_i8`, `sub_i16`, `sub_i32`, `sub_i64`, `sub_i8`, `trn1`, `uzp1`, `v128_load`, `v128_store`, `zip1` | `vaba`, `vabal`, `vabd`, `vabdl`, `vabs`, `vaddhn`, `vaddl`, `vaddw`, `vbic`, `vbsl`, `vceq`, `vcge`, `vcgt`, `vcle`, `vclt`, `vcls`, `vclz`, `vcnt`, `vcvt`, `vext`, `vfma`, `vfms`, `vfnma`, `vfnms`, `vhadd`, `vhsub`, `vld1`, `vld2`, `vld3`, `vld4`, `vmax`, `vmin`, `vmla`, `vmlal`, `vmls`, `vmlsl`, `vmov`, `vmovn`, `vmull`, `vmvn`, `vneg`, `vorn`, `vpadal`, `vpadd`, `vpmax`, `vpmin`, `vqabs`, `vqadd`, `vqdmlal`, `vqdmlsl`, `vqdmulh`, `vqdmull`, `vqmovn`, `vqmovun`, `vqneg`, `vqrdmulh`, `vqrshl`, `vqrshrn`, `vqrshrun`, `vqshl`, `vqshlu`, `vqshrn`, `vqshrun`, `vqsub`, `vraddhn`, `vrecpe`, `vrecps`, `vrev16`, `vrev32`, `vrev64`, `vrshl`, `vrshr`, `vrshrn`, `vrsqrte`, `vrsqrts`, `vrsra`, `vrsubhn`, `vshl`, `vshll`, `vshr`, `vshrn`, `vsli`, `vsra`, `vsri`, `vsubhn`, `vsubl`, `vsubw`, `vswp`, `vtbl`, `vtbx`, `vtrn`, `vtst`, `vuzp`, `vzip` | QEMU            |
| `dotprod`                             | `sdot`, `udot`                                                                                                                                                                                                                                                                                                                  | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | QEMU            |
| `fp16`                                | `add`, `mul`                                                                                                                                                                                                                                                                                                                    | `vabs`, `vadd`, `vcmp`, `vcvt`, `vdiv`, `vfma`, `vfms`, `vmax`, `vmin`, `vmul`, `vneg`, `vsqrt`, `vsub`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | QEMU            |
| `i8mm`                                | `smmla`, `usmmla`                                                                                                                                                                                                                                                                                                               | `vummla`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | QEMU では未実行 |
| `mve`                                 | なし                                                                                                                                                                                                                                                                                                                            | `vabs`, `vadd`, `vaddlv`, `vaddv`, `vand`, `vabd`, `vabav`, `vbic`, `vbrsr`, `vclz`, `vcmp`, `vctp`, `vdup`, `veor`, `vfma`, `vfms`, `vhadd`, `vhsub`, `vidup`, `viwdup`, `vldr`, `vldrb`, `vmax`, `vmaxv`, `vmin`, `vminv`, `vmla`, `vmlal`, `vmls`, `vmlsl`, `vmov`, `vmul`, `vmull`, `vmvn`, `vneg`, `vorn`, `vorr`, `vqabs`, `vqadd`, `vqdmulh`, `vqdmull`, `vqmovn`, `vqneg`, `vqrdmulh`, `vqshl`, `vqsub`, `vrev`, `vrmulh`, `vshl`, `vshr`, `vsli`, `vsri`, `vstr`, `vstrb`, `vsub`, `vsubn`, `vsubw`, `vuzp`, `vzip`                                                                                                                                                                                                                                                                                                                                                 | 対象外          |
| `mve.fp`                              | なし                                                                                                                                                                                                                                                                                                                            | `vabs`, `vadd`, `vcmp`, `vcvt`, `vdiv`, `vfma`, `vfms`, `vmax`, `vmin`, `vmul`, `vneg`, `vsqrt`, `vsub`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | 対象外          |
| `vfp2` / `vfp3` / `vfp4` / `fp-armv8` | なし                                                                                                                                                                                                                                                                                                                            | スカラー VFP は命令 API にしない                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | 対象外          |

### 暗号

| extension | 実装済み命令                                    | 未実装命令 | 値テスト |
| --------- | ----------------------------------------------- | ---------- | -------- |
| `aes`     | `aesd`, `aese`, `aesimc`, `aesmc`               | なし       | QEMU     |
| `sha2`    | `sha256h`, `sha256h2`, `sha256su0`, `sha256su1` | なし       | QEMU     |
| `crc`     | `crc32b`, `crc32h`, `crc32w`                    | なし       | QEMU     |

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

## 未対応

- `mve` / `mve.fp`。M プロファイル向けなので、ARMv7 の family には置かない。
- `vfp2`、`vfp3`、`vfp4`、`fp-armv8`、`d32`、`fpregs`。スカラー VFP と ABI の
  指定であり、SIMD の値 API には含めない。
- `dsp`。スカラー DSP 命令であり、SIMD の値 API には含めない。
- 追加した NEON 以降の命令のゼロコスト確認。現在の zero-cost は `vadd.i32` と `vmul.i32` だけである。
- NEON の hard-float ABI と値型の対応は、C の境界ではポインタだけを使う現在の
  方式で回避している。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
