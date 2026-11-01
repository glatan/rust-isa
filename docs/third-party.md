# 第三者ソースとライセンス

この文書は、命令 manifest の候補生成に使う上流ソースと、その配布条件をまとめる。

> ライセンス解釈は法務確認が必要。ここでは技術的な整理と、同梱可否の初期判断を示す。

## 方針

- build 時にネットワークへ取りに行かない。
- 上流ソースは `tools/upstream/SOURCE.toml` に版・commit・license・SHA256 を記録する。
- 同梱するのは、再配布が許可された抽出データだけにする。
- 仕様書本体は、参照のみのものを含む。
- `docs/third-party.md` と `SOURCE.toml` の内容を一致させる。

## 主ソース

| arch          | source             | license                                                             | 同梱       | 備考                                 |
| ------------- | ------------------ | ------------------------------------------------------------------- | ---------- | ------------------------------------ |
| x86           | Intel XED          | Apache-2.0                                                          | 可         | AMX-FP8 を含む。AVX512BMM は含まない |
| x86           | LLVM X86           | Apache-2.0 WITH LLVM-exception                                      | 可         | AVX512BMM を含む                     |
| AArch64 / ARM | ACLE               | 文書 CC-BY-SA-4.0、tools Apache-2.0                                 | 条件付き可 | SVE2.1〜2.3 / SME2.1〜2.3            |
| AArch64 / ARM | LLVM AArch64 / ARM | Apache-2.0 WITH LLVM-exception                                      | 可         | instruction definition               |
| RISC-V        | riscv-opcodes      | BSD-3-Clause                                                        | 可         | opcode / extension                   |
| RISC-V        | riscv-isa-manual   | CC-BY-4.0                                                           | 条件付き可 | 意味論の参照                         |
| RISC-V        | riscv-unified-db   | BSD-3-Clause-Clear / CC-BY / CC0 / MIT                              | 要監査     | 機械可読 database                    |
| LoongArch     | LLVM LoongArch     | Apache-2.0 WITH LLVM-exception                                      | 可         | instruction definition               |
| LoongArch     | loongarch-opcodes  | CC-BY-4.0 / Mulan OWL（data）、Apache-2.0 / MIT / Mulan PSL（code） | 条件付き可 | 非公式                               |
| s390x         | LLVM SystemZ       | Apache-2.0 WITH LLVM-exception                                      | 可         | IBM PoP は参照のみ                   |
| PowerPC       | PowerISA JSON      | Apache-2.0                                                          | 可         | ISA v3.1 / Power10 MMA               |
| PowerPC       | LLVM PowerPC       | Apache-2.0 WITH LLVM-exception                                      | 可         | 新しい form                          |
| MIPS          | LLVM Mips          | Apache-2.0 WITH LLVM-exception                                      | 可         | MSA を含む。legacy                   |
| wasm32        | LLVM WebAssembly   | Apache-2.0 WITH LLVM-exception                                      | 可         | atomics / simd128 / fp16             |
| wasm32        | WebAssembly spec   | W3C / Apache-2.0 / CC0 / CC-BY                                      | 条件付き可 | directory ごとに license が違う      |

## 参照のみ

次の仕様書は同梱しない。意味論の確認にだけ使う。

- Intel SDM / Intel Intrinsics Guide
- Arm Architecture Reference Manual
- IBM z/Architecture Principles of Operation
- MIPS Architecture Manual
- LoongArch Reference Manual / la-asm-manual
- Power ISA PDF

## 条件付き同梱

### ACLE

ACLE の文書は CC-BY-SA-4.0 + patent grant。抽出表を同梱する場合は、`tools/upstream/acle/` に別 license を置き、attribution を付け、share-alike を Rust コード本体に広げない。

### riscv-isa-manual

CC-BY-4.0。抽出物には attribution を付ける。

### loongarch-opcodes

data は CC-BY-4.0 または Mulan OWL-BY-1.0、code は Apache-2.0 / MIT / Mulan PSL-2.0。使う license を選び、README と license を保持する。GitHub の license 表示だけを根拠にしない。

### WebAssembly spec

`document/` / `specification/` は W3C、`spectec/` / `interpreter/` / `test/` は Apache-2.0、`proposals/` は CC0、`papers/` は CC-BY-4.0。wasm の命令一覧は LLVM を主に使う。

## 更新手順

1. `SOURCE.toml` の版・commit・license・SHA256 を更新する。
2. `make manifest-pilot-check` を通す。
3. 抽出データの diff をレビューする。
4. `docs/third-party.md` と `SOURCE.toml` の license を一致させる。
