# 32-bit ARM

対象は `armv7-unknown-linux-gnueabihf` である。`thumbv7neon-unknown-linux-gnueabihf` も比較に使う。

`mve` と `mve.fp` は M プロファイル専用である。値テストは `-C target-feature=+thumb-mode` でビルドし、`qemu-arm-static -cpu cortex-m55` で実行する。ほかの extension は `-cpu max` で実行する。

family は命令の種類に合わせて決める（`simd`、`crypto`、`integer` など）。公開パスは `arm::{family}::{extension}::{instruction}` とする。

## 実装状況

対象は各 extension の全命令である。次の表は、現在公開している命令と、
まだ公開していない命令の一覧である。値テストの列は、その extension の
値テストを実行する環境を示す。未実装命令も同じ表で管理する。

### SIMD

| extension  | 実装済み命令                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | 未実装命令 | 値テスト             |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------------------- |
| `neon`     | `add`, `add_i16`, `add_i64`, `add_i8`, `and`, `cmpeq_i16`, `cmpeq_i32`, `cmpeq_i8`, `cmpgt_i16`, `cmpgt_i32`, `cmpgt_i8`, `eor`, `fadd`, `fmul`, `fsub`, `mul`, `mul_i16`, `mul_i32`, `orr`, `splat_i16`, `splat_i32`, `splat_i8`, `sub_i16`, `sub_i32`, `sub_i64`, `sub_i8`, `v128_load`, `v128_store`, `vaba`, `vabal`, `vabd`, `vabdl`, `vabs`, `vaddhn`, `vaddl`, `vaddw`, `vbic`, `vbsl`, `vceq`, `vcge`, `vcgt`, `vcle`, `vcls`, `vclt`, `vclz`, `vcnt`, `vcvt`, `vext`, `vfma`, `vfms`, `vfnma`, `vfnms`, `vhadd`, `vhsub`, `vld1`, `vld2`, `vld3`, `vld4`, `vmax`, `vmin`, `vmla`, `vmlal`, `vmls`, `vmlsl`, `vmov`, `vmovn`, `vmull`, `vmvn`, `vneg`, `vorn`, `vpadal`, `vpadd`, `vpmax`, `vpmin`, `vqabs`, `vqadd`, `vqdmlal`, `vqdmlsl`, `vqdmulh`, `vqdmull`, `vqmovn`, `vqmovun`, `vqneg`, `vqrdmulh`, `vqrshl`, `vqrshrn`, `vqrshrun`, `vqshl`, `vqshlu`, `vqshrn`, `vqshrun`, `vqsub`, `vraddhn`, `vrecpe`, `vrecps`, `vrev16`, `vrev32`, `vrev64`, `vrshl`, `vrshr`, `vrshrn`, `vrsqrte`, `vrsqrts`, `vrsra`, `vrsubhn`, `vshl`, `vshll`, `vshr`, `vshrn`, `vsli`, `vsra`, `vsri`, `vsubhn`, `vsubl`, `vsubw`, `vswp`, `vtbl`, `vtbx`, `vtrn`, `vtst`, `vuzp`, `vzip` | なし       | QEMU                 |
| `dotprod`  | `sdot`, `udot`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | なし       | QEMU                 |
| `fp16`     | `abs`, `add`, `cmp`, `cvt_f16`, `cvt_f32`, `cvt_high_f16`, `cvt_high_f32`, `div`, `fma`, `fms`, `max`, `min`, `mul`, `neg`, `sqrt`, `sub`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | なし       | QEMU                 |
| `i8mm`     | `smmla`, `vusmmla`, `vummla`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | なし       | QEMU                 |
| `mve`      | `abs`, `abav`, `abd`, `add`, `addlv`, `addv`, `and`, `bic`, `brsr`, `clz`, `cmp`, `ctp`, `dup`, `eor`, `fma`, `fms`, `hadd`, `hsub`, `idup`, `iwdup`, `ldr`, `ldrb`, `max`, `maxv`, `min`, `minv`, `mla`, `mlal`, `mls`, `mlsl`, `mov`, `mul`, `mull`, `mvn`, `neg`, `orn`, `orr`, `qabs`, `qadd`, `qdmulh`, `qdmull`, `qmovn`, `qneg`, `qrdmulh`, `qshl`, `qsub`, `rev16`, `rev32`, `rev64`, `rmulh`, `shl`, `shr`, `sli`, `sri`, `str`, `strb`, `sub`, `subn`, `subw`, `uzp`, `zip`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | なし       | QEMU（`cortex-m55`） |
| `mve.fp`   | `abs`, `add`, `cmp`, `cvt`, `cvt_f32`, `div`, `fma`, `fms`, `max`, `min`, `mul`, `neg`, `sqrt`, `sub`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | なし       | QEMU（`cortex-m55`） |
| `vfp2`     | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | なし       | 対象外               |
| `vfp3`     | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | なし       | 対象外               |
| `vfp4`     | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | なし       | 対象外               |
| `fp-armv8` | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | なし       | 対象外               |

### 暗号・チェックサム

| extension | 実装済み命令                                    | 未実装命令 | 値テスト |
| --------- | ----------------------------------------------- | ---------- | -------- |
| `aes`     | `aesd`, `aese`, `aesimc`, `aesmc`               | なし       | QEMU     |
| `sha2`    | `sha256h`, `sha256h2`, `sha256su0`, `sha256su1` | なし       | QEMU     |
| `crc`     | `crc32b`, `crc32h`, `crc32w`                    | なし       | QEMU     |

## 実装方式

- 今回測った feature はすべて不安定であり、cfg ゲート方式を使えない。関数を無条件に置き、`unsafe fn` として公開する。
- C + clang を主方式とする。NEON は `<arm_neon.h>`、MVE は `<arm_mve.h>` を使う。
- 値は `V128` をポインタで渡す。MVE も NEON と同じ `V128` を使う。
- C は extension ごとに分ける。NEON は `-mfpu=neon-vfpv4` と `-mfloat-abi=hard` を使う。`vfma` / `vfms` には VFPv4 が必要である。
- MVE は M プロファイル専用である。C は `-march=armv8.1-m.main+mve`（整数）と `-march=armv8.1-m.main+mve.fp+fp.dp`（浮動小数点）でコンパイルし、`-mfpu=fp-armv8` を付ける。Rust 側は Thumb でビルドし、QEMU は `-cpu cortex-m55` を使う。
- MVE の predicate は 16-bit の値である。`cmp` は真のレーンをすべて 1、偽のレーンを 0 にした `V128` を返す。`ctp` は tail predicate の生値を `u16` で返す。
- `mve::fma` と `mve::fms` は MVE の整数命令ではなく、MVE-FP の `vfma.f32` / `vfms.f32` を再エクスポートする。呼び出しには MVE-FP が要る。
- MVE の表にある `vuzp`、`vzip`、`vsubn`、`vsubw` は、clang 23 の MVE intrinsic に単体の命令として無い。表の名前を保つため、`vuzp` / `vzip` はバイト操作、`vsubn` / `vsubw` は MVE の移動・減算の組み合わせで同じ値を返す。
- MVE の `mla` / `mls` はベクタ同士の形が ACLE に無いため、`vmul` と `vadd` / `vsub` の組み合わせで実装する。`mlal` / `mlsl` は `vmullb` と `vadd` / `vsub` の組み合わせで実装する。
- MVE-FP の `vdiv` / `vsqrt`、および FP16 の `vdiv` / `vsqrt` も AArch32 の clang intrinsic に無い。スカラー浮動小数点のループで同じ値を返す。
- `asm!` の `qreg` は `u128` を受け付けず、`neon` を要求する。`qreg` が受け付ける `i32x4` などは unstable である。NEON は C + clang を使う。
- NEON の table lookup は 8-byte の `vtbl1_s8` / `vtbl2_s8` だけである。`vtbl` と `vtbx` は 16-byte の table と 8-byte の index・結果を使う `V64` で公開する。`V128` を 1 命令で shuffle する命令は無いため、`V128` の shuffle API は出さない。
- NEON の命令は代表のデータ幅だけを公開する。widening の入力と narrowing の結果は 8-byte であり、widening は入力の下位 8 bytes を読み、narrowing は結果の下位 8 bytes に書いて上位 8 bytes を 0 にする。即値を取る命令は const generic で公開する。
- AArch32 に vector 形の `vfnma` / `vfnms` は無い。`vfnma` は `acc` を符号反転した `vfma`、`vfnms` は `acc` を符号反転した `vfms` で同じ値を返す。

## ゼロコスト

- 結果は [ゼロコストの確認](../zero-cost.md) を見る。
- C は `-mthumb -march=armv7-a -mfpu=neon-vfpv4 -mfloat-abi=hard` を使う。さらに `-Xclang -target-feature` で feature 集合を Rust に合わせる。
  - 外す: `armv7-a`、`dsp`、`fp64`、`vfp2sp`、`vfp3sp`、`vfp3d16sp`、`read-tp-tpidruro`、`thumb-mode`
  - 足す: `v7`、`thumb2`
- `-C target-feature=+neon` は不安定警告が出る。`#[target_feature]` は使えない。クレート全体を flag 付きでコンパイルする。
- この feature 調整は clang の実装に依存する。build.rs に固定し、disassembly で確認する。

## ツールチェーン

- clang、llvm-ar、llvm-objdump、qemu-arm-static。
- clang 23.1.3 で、`-mfpu=neon-vfpv4 -mfloat-abi=hard` の `vaddq_s32` が `vadd.i32 q0` を出すことを確認した。
- clang 23.1.3 で、`-march=armv8.1-m.main+mve` の `vaddq_u8` が `vadd.i8 q0` を、`mve.fp+fp.dp` の `vaddq_f32` が MVE の `vadd.f32` を出すことを確認した。
- QEMU では NEON と FP16 を `-cpu max`、MVE と MVE-FP を `-cpu cortex-m55` で実行する。`-cpu max` は MVE を実行できず、`cortex-m55` は NEON を実行できないため、値テストを分ける。
- ゼロコストの確認では、`-Xclang -target-feature` で C の feature 集合を Rust に合わせた。この調整は clang の実装に依存する。

## 実装の順序

1. `neon`
2. `vfp3`、`vfp4`、`fp-armv8`
3. `fp16`、`dotprod`、`i8mm`
4. `aes`、`sha2`、`crc`
5. `mve`、`mve.fp`（M プロファイル向け。対象を分ける）

## 未対応

- `vfp2`、`vfp3`、`vfp4`、`fp-armv8`、`d32`、`fpregs`。スカラー VFP と ABI の
  指定であり、SIMD の値 API には含めない。
- `dsp`。スカラー DSP 命令であり、SIMD の値 API には含めない。
- MVE の zero-cost は未確認である。MVE の API は M プロファイルでだけ動き、`armv7-unknown-linux-gnueabihf` のビルドでは C と Rust の feature 集合を一致させられない。
- MVE の命令は代表のデータ幅だけを公開する。整数は signed 32-bit、`mull` / `mlal` / `mlsl` / `qdmull` は signed 16-bit、`abav` は signed 8-bit、`addlv` は signed 32-bit の入力を使う。ほかのデータ幅は同じ命令の別形として未公開である。
- NEON の hard-float ABI と値型の対応は、C の境界ではポインタだけを使う現在の
  方式で回避している。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
