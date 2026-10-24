# スケーラブル・状態付き extension の API 対応

この文書は、SVE、RVV、SME、AMX、PowerPC MMA の ISA 命令と API の対応を示す。実装状況の正は `docs/<arch>/README.md` の表である。この文書は対応関係だけを持ち、`provided` / `deferred` などの状態は持たない。

## SVE

SVE は固定長の値型ではなく、スライス操作として API を出す。関数名は命令名に対応し、`a`、`b`、`out` のスライスを受け取る。

| API の例             | カバーする命令 |
| -------------------- | -------------- |
| `add_f32`、`add_i32` | `FADD`、`ADD`  |
| `mul_f32`、`mul_i32` | `FMUL`、`MUL`  |
| `abs_f32`、`abs_i32` | `FABS`、`ABS`  |
| `fmla_f32`           | `FMLA`         |
| `ld1_f32`、`st1_f32` | `LD1W`、`ST1W` |

全 API は [AArch64 の実装状況](aarch64/README.md#実装状況) を見る。SVE2 固有命令は対象外である。

## RVV

### 固定長 `V128` API

`V128` を受ける関数は、1命令または同じ形の命令列に対応する。

| API の例                                  | カバーする命令                                 |
| ----------------------------------------- | ---------------------------------------------- |
| `add`、`add_i8`、`add_i16`、`add_i64`     | `vadd.vv`                                      |
| `sub_i8`、`sub_i16`、`sub_i32`、`sub_i64` | `vsub.vv`                                      |
| `mul`、`mul_i8`、`mul_i16`                | `vmul.vv`                                      |
| `and`、`or`、`xor`                        | `vand.vv`、`vor.vv`、`vxor.vv`                 |
| `cmpeq_i32`、`cmpgt_i32`                  | `vmseq.vv`、`vmsgt.vv`                         |
| `fadd`、`fsub`、`fmul`、`fdiv`            | `vfadd.vv`、`vfsub.vv`、`vfmul.vv`、`vfdiv.vv` |
| `splat_i32`                               | `vmv.v.x`                                      |

全 API は [RISC-V の実装状況](riscv/README.md#実装状況) を見る。

### スライス API

スライス API は、C のループが `vsetvli` で VL を決め、load、演算、store を並べる。

| API の例             | カバーする命令                                          |
| -------------------- | ------------------------------------------------------- |
| `add_f32`、`mul_f32` | `vsetvli`、`vle32.v`、`vfadd.vv`、`vfmul.vv`、`vse32.v` |
| `add_i32`、`mul_i32` | `vsetvli`、`vle32.v`、`vadd.vv`、`vmul.vv`、`vse32.v`   |

LMUL は 32-bit 要素で 1 とする。VL は C の `vsetvl` が決める。

## SME

`SmeGuard` がストリーミングモードを管理し、状態を要求する関数は `&SmeGuard` を引数に取る。

| API               | カバーする命令   |
| ----------------- | ---------------- |
| `SmeGuard::enter` | `SMSTART`        |
| `SmeGuard::drop`  | `SMSTOP`         |
| `add_f32`         | streaming `FADD` |
| `mul_f32`         | streaming `FMUL` |

ZA タイル、SME2、`smstart` / `smstop` の直接 API は対象外である。理由は [AArch64 の対象外](aarch64/README.md#対象外) を見る。

## AMX

`AmxGuard` がタイル状態を管理し、状態を要求する関数は `&AmxGuard` を引数に取る。`TileConfig` は `ldtilecfg` の入力を保持し、`TileConfig::builder()` で構築する。

| API                                                       | カバーする命令                 |
| --------------------------------------------------------- | ------------------------------ |
| `ldtilecfg`、`AmxGuard::enter`                            | `LDTILECFG`                    |
| `sttilecfg`                                               | `STTILECFG`                    |
| `tilezero::<TILE>`                                        | `TILEZERO`                     |
| `tileloadd::<TILE>`、`tileloaddt1::<TILE>`                | `TILELOADD`、`TILELOADDT1`     |
| `tileloaddrs::<TILE>`、`tileloaddrst1::<TILE>`            | `TILELOADDRS`、`TILELOADDRST1` |
| `tilestored::<TILE>`                                      | `TILESTORED`                   |
| `tilemovrow::<TILE>`、`tilemovrow_imm::<TILE, ROW>`       | `TILEMOVROW`                   |
| `tcvtrowd2ps`、`tcvtrowps2bf16h/l`、`tcvtrowps2phh/l`     | `TCVTROW*`                     |
| `tdpbf16ps`、`tdpbssd`、`tdpbsud`、`tdpbuud`、`tdpfp16ps` | 行列積                         |
| `tcmmimfp16ps`、`tcmmrlfp16ps`                            | 複素行列積                     |

`amx-avx512` の変換命令はランタイム行と即値行の両方を提供する。全 API は [x86 の実装状況](x86/README.md#実装状況) を見る。

## PowerPC MMA

`MmaAccumulator` がアキュムレータを保持し、命令は `&mut MmaAccumulator` を引数に取る。

| API                                                      | カバーする命令                                           |
| -------------------------------------------------------- | -------------------------------------------------------- |
| `MmaAccumulator::new`                                    | `xxsetaccz`                                              |
| `xxmfacc`、`xxmtacc`                                     | `xxmfacc`、`xxmtacc`                                     |
| `xvi4ger8`、`xvi4ger8pp`                                 | `xvi4ger8`、`xvi4ger8pp`                                 |
| `xvi8ger4`、`xvi8ger4pp`、`xvi8ger4spp`                  | `xvi8ger4`、`xvi8ger4pp`、`xvi8ger4spp`                  |
| `xvi16ger2`、`xvi16ger2s`、`xvi16ger2pp`、`xvi16ger2spp` | `xvi16ger2`、`xvi16ger2s`、`xvi16ger2pp`、`xvi16ger2spp` |

全 API は [PowerPC の実装状況](powerpc/README.md#実装状況) を見る。

## 対象外

- SVE / SME / AMX の制御命令、状態命令、メモリ順序命令は、値 API にしない。
- RVV の `vsetvl` は API にしない。VL は C が決める。
- システム命令、割り込み、仮想化、タグ、認証命令は対象外とする。
