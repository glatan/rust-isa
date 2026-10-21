# スケーラブル・状態付き extension の対応表

この文書は、SVE、RVV、SME、AMX、PowerPC MMA の ISA 命令と API の対応を管理する。固定長 extension のように、命令ごとに1関数を公開しないため、対応表を実装状況の正とする。

## 状態

| 状態     | 意味                                  |
| -------- | ------------------------------------- |
| provided | API として公開している。              |
| covered  | 別の API または操作でカバーしている。 |
| excluded | 対象外。理由を書く。                  |
| deferred | 延期。API をまだ公開していない。      |

## SVE

| API       | カバーする命令          | 状態     |
| --------- | ----------------------- | -------- |
| `add_f32` | `FADD`（32-bit float）  | provided |
| `add_f64` | `FADD`（64-bit float）  | provided |
| `add_i32` | `ADD`（32-bit integer） | provided |
| `mul_f32` | `FMUL`（32-bit float）  | provided |
| `mul_f64` | `FMUL`（64-bit float）  | provided |
| `mul_i32` | `MUL`（32-bit integer） | provided |

`docs/aarch64/README.md` の SVE 行に残る命令は、スライス API の操作として未提供であり、`deferred` とする。SVE2 固有命令も `deferred` とする。

## RVV

### 固定長 `V128` API

| API                                       | カバーする命令                                 | 状態     |
| ----------------------------------------- | ---------------------------------------------- | -------- |
| `add`, `add_i8`, `add_i16`, `add_i64`     | `vadd.vv`                                      | provided |
| `sub_i8`, `sub_i16`, `sub_i32`, `sub_i64` | `vsub.vv`                                      | provided |
| `mul`, `mul_i8`, `mul_i16`                | `vmul.vv`                                      | provided |
| `and`, `or`, `xor`                        | `vand.vv`, `vor.vv`, `vxor.vv`                 | provided |
| `cmpeq_i32`                               | `vmseq.vv`                                     | provided |
| `cmpgt_i32`                               | `vmsgt.vv`                                     | provided |
| `fadd`, `fsub`, `fmul`, `fdiv`            | `vfadd.vv`, `vfsub.vv`, `vfmul.vv`, `vfdiv.vv` | provided |
| `splat_i32`                               | `vmv.v.x`                                      | provided |

`docs/riscv/README.md` の V 行に残る命令は `deferred` とする。メモリ、segment、strided、reduction はスライス API の拡張として扱う。

### スライス API

| API                  | カバーする命令                                          | 状態     |
| -------------------- | ------------------------------------------------------- | -------- |
| `add_f32`, `mul_f32` | `vsetvli`, `vle32.v`, `vfadd.vv`, `vfmul.vv`, `vse32.v` | provided |
| `add_i32`, `mul_i32` | `vsetvli`, `vle32.v`, `vadd.vv`, `vmul.vv`, `vse32.v`   | provided |

LMUL は 32-bit 要素で 1 とする。VL は C の `vsetvl` が決める。

## SME

| API               | カバーする命令                                                                                                                                                      | 状態     |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| `SmeGuard::enter` | `SMSTART`                                                                                                                                                           | provided |
| `SmeGuard::drop`  | `SMSTOP`                                                                                                                                                            | provided |
| `add_f32`         | streaming `FADD`                                                                                                                                                    | provided |
| `mul_f32`         | streaming `FMUL`                                                                                                                                                    | provided |
| ZA タイル操作     | `ldr`, `str`, `zero`, `mova`, `add`, `sub`, `mul`, `fmopa`, `fmops`, `bfmopa`, `bfmops`, `smopa`, `smops`, `umopa`, `umops`, `usmopa`, `usmops`, `sumopa`, `sumops` | deferred |

ZA の型と操作は未決定である。SME2 は `deferred` とする。

## AMX

| API                | カバーする命令                                                                            | 状態     |
| ------------------ | ----------------------------------------------------------------------------------------- | -------- |
| `TileConfig`       | `LDTILECFG` の入力                                                                        | provided |
| `AmxGuard::enter`  | `LDTILECFG`                                                                               | provided |
| `AmxGuard::drop`   | `TILERELEASE`                                                                             | provided |
| `tilezero::<TILE>` | `TILEZERO`                                                                                | provided |
| tile config store  | `STTILECFG`                                                                               | deferred |
| tile load / store  | `TILELOADD`, `TILELOADDT1`, `TILESTORED`                                                  | deferred |
| matrix multiply    | `TDPBF16PS`, `TDPBSSD`, `TDPBSUD`, `TDPBUUD`, `TDPFP16PS`, `TCMMIMFP16PS`, `TCMMRLFP16PS` | deferred |

AMX の追加拡張（`amx-int8`、`amx-bf16`、`amx-fp16`、`amx-fp8`、`amx-complex`、`amx-avx512`、`amx-movrs`）は `deferred` とする。

## PowerPC MMA

`MmaAccumulator` の設計はあるが、API は未公開である。`xvi4ger8`、`xvi8ger4`、`xvi4ger8pp`、`xvi8ger4pp`、`xvi8ger4spp`、`xvi16ger2`、`xvi16ger2s`、`xvi16ger2pp`、`xvi16ger2spp`、`xxmfacc`、`xxmtacc`、`xxsetaccz` は `deferred` とする。

## 対象外

- SVE / SME / AMX の制御命令、状態命令、メモリ順序命令は、値 API にしない。
- RVV の `vsetvl` は API にしない。VL は C が決める。
- システム命令、割り込み、仮想化、タグ、認証命令は対象外とする。

## 次の作業

- SVE のスライス API を増やしたら、この表の `provided` を更新する。
- RVV の固定長 API とスライス API の追加分をこの表に追加する。
- SME ZA、AMX タイル、MMA の設計が決まったら、`deferred` を `provided` に変える。
- 全 ISA 命令が `provided`、`covered`、`excluded`、`deferred` のいずれかになるまで、`F4.2b` の coverage blocking を有効にしない。
