# LoongArch

対象は `loongarch64-unknown-linux-gnu` である。`lsx` は既定、`lasx` は安定である。

family は命令の種類に合わせて決める（`simd`、`system` など）。公開パスは `loongarch::{family}::{extension}::{instruction}` とする。

## 実装状況

対象は各 extension の全命令である。次の表は、現在公開している命令と、
まだ公開していない命令の一覧である。値テストの列は、その extension の
値テストを実行する環境を示す。未実装命令も同じ表で管理する。

### SIMD

| extension        | 実装済み命令                                                                                                                                                                                                                                                                                                                                                                     | 未実装命令                                                                                                                                                                                                                                                                                                                                                                                              | 値テスト |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| `lsx` | `add`, `add_i16`, `add_i64`, `add_i8`, `and`, `cmpeq_i32`, `cmpgt_i32`, `fadd`, `fdiv`, `fmul`, `fsub`, `mul`, `mul_i16`, `mul_i32`, `or`, `splat_i32`, `sub_i16`, `sub_i32`, `sub_i64`, `sub_i8`, `v128_load`, `v128_store`, `xor`, `vadd`, `vsub`, `vmul`, `vand`, `vor`, `vxor`, `vabs`, `vneg`, `vdiv`, `vadd_d`, `vsub_d`, `vadd_b`, `vadd_h`, `vsub_b`, `vsub_h`, `vmul_h`, `vadd_w`, `vsub_w`, `vmul_w`, `vneg_w`, `vabs_b`, `vabs_h`, `vabs_d`, `vneg_b`, `vneg_h`, `vneg_d` | `vmax`, `vmin`, `vseq`, `vsle`, `vslt`, `vseqi`, `vslei`, `vslti`, `vnor`, `vbitsel`, `vshuf`, `vshuf4i`, `vsll`, `vsrl`, `vsra`, `vslli`, `vsrli`, `vsrai`, `vpackev`, `vpackod`, `vext`, `vexth`, `vextl`, `vsat`, `vssr`, `vavgr`, `vfadd`, `vfsub`, `vfmul`, `vfdiv`, `vfmax`, `vfmin`, `vfcmp`, `vfcvt`, `vftint`, `vffint`, `vld`, `vst`, `vldi`, `vrepl` | QEMU |
| `lasx` | `add`, `add_i64`, `and`, `fadd`, `fdiv`, `fmul`, `fsub`, `mul`, `mul_i32`, `or`, `splat_i32`, `sub`, `sub_i64`, `v256_load`, `v256_store`, `xor`, `xvadd`, `xvsub`, `xvmul`, `xvand`, `xvor`, `xvxor`, `xvabs`, `xvneg`, `xvdiv`, `xvadd_d`, `xvsub_d`, `xvadd_b`, `xvadd_h`, `xvsub_b`, `xvsub_h`, `xvmul_h`, `xvabs_b` | `xvmax`, `xvmin`, `xvseq`, `xvsle`, `xvslt`, `xvseqi`, `xvslei`, `xvslti`, `xvnor`, `xvbitsel`, `xvshuf`, `xvshuf4i`, `xvsll`, `xvsrl`, `xvsra`, `xvslli`, `xvsrli`, `xvsrai`, `xvpackev`, `xvpackod`, `xvext`, `xvexth`, `xvextl`, `xvsat`, `xvssr`, `xvavgr`, `xvfadd`, `xvfsub`, `xvfmul`, `xvfdiv`, `xvfmax`, `xvfmin`, `xvfcmp`, `xvfcvt`, `xvftint`, `xvffint`, `xvld`, `xvst`, `xvldi`, `xvrepl` | QEMU |
| `lbt`            | `bitrev`, `bstrins`, `bstrpick`, `ext`, `ins`, `revb`, `revh`, `revw`                                                                                                                                                                                                                                                                                                            | なし                                                                                                                                                                                                                                                                                                                                                                                                    | 未実行   |
| `lamcas` / `scq` | なし                                                                                                                                                                                                                                                                                                                                                                             | 同期命令のため命令 API にしない                                                                                                                                                                                                                                                                                                                                                                         | 対象外   |

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

## 未対応

- `lbt`、`lamcas`、`scq`。
- LASX の shuffle。`xvshuf.b` は intrinsics で往復が残るため、現在の API には
  含めていない。
- 追加した LSX / LASX の命令のゼロコスト確認。現在の zero-cost は `xvadd.w` と `xvmul.w` だけである。
- LASX のベクタ ABI は、C の境界ではポインタだけを使う現在の方式で回避して
  いる。値渡しの ABI は公開しない。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
