# 実装方式

この文書は、C、`asm!`、純Rustの使い分けとツールチェーンをまとめる。決定は [実装方針](policy.md)、ゼロコストの条件は [ゼロコストの確認](zero-cost.md) に置く。

## 方式

| 方式            | 用途                             | 長所                                                                                                          | 制約                                                                                                                                               |
| --------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| C + clang + LTO | ベクタ命令の主方式               | `core::arch` なしで、LLVM の feature と vector 型を使える。rustc が unstable とする feature も clang は扱える | clang が必要。値の受け渡しはポインタ。native のゼロコストは LTO の条件付き                                                                         |
| `asm!`          | 補助                             | cfg に依存しない。C ツールチェーンが不要                                                                      | 256/512-bit の値型がない。wasm と MIPS では unstable。vector operand の型が限られる。AArch64 の `vreg` と ARMv7 の `qreg` は `u128` を受け付けない |
| 純 Rust         | 値の構築、ビット操作、値の意味論 | ツールチェーンが不要                                                                                          | 単独では対象CPUが命令を持つことを保証しない。自動ベクトル化に依存しない                                                                            |

semantic helperは、命令の存在を主張せず、値の意味だけを提供する関数である。semantic helperはsafeにでき、命令の在庫には数えない。

- C は extension ごとにリポジトリへコミットする。生成物は同梱しない。
- C は、可能なら GCC の vector 拡張と通常のロード・ストアで書く。intrinsics は SROA を妨げ、一時値の store/load を残すことがある。wasm32 は `<wasm_simd128.h>` の intrinsics を使う。
- C の関数は、可能なら `__attribute__((target("...")))` を関数単位で付ける。ファイル全体の `-m` フラグが必要な場合は extension ごとに分ける。
- shuffle のように intrinsics が必要な命令は、intrinsics が SROA を妨げることがある。命令ごとに zero-cost を disassembly で確認する。
- Rust 側は `unsafe extern "C"` で宣言し、`#[inline(always)]` のラッパーから呼ぶ。ラッパーに `#[target_feature]` は付けない。
- レーン即値は const generics と `match` で、即値ごとの C 関数に振り分ける。
- LTO は、wasm32 と native のゼロコストのために使う。native は LTO モードと feature 集合を一致させる必要がある。

## 検討した代替案

- `core::arch` のラッパー: API を `core::arch` に依存させるため却下する。このクレートの目的に反する。
- `asm!` だけを使う: 256/512-bit の値型が stable に無く、wasm と MIPS では `asm!` が unstable である。vector operand の型も限られる。AArch64 の `vreg` は `i64x2` や `f128` を要求するが、どちらも unstable である。128-bit の x86 では、`u128` と `xmm_reg` でゼロコストを確認した。
- 全 arch で cfg ゲート: native にはモジュール全体の feature 要件が無く、API が rustflags に依存する。不安定な feature は cfg が立たない。wasm32 だけに使う。
- Cargo feature で opt-in: additive でない組み合わせと docs.rs の扱いが未解決である。wasm32 の `wide-arithmetic` のような例外を除いて保留する。
- 実行時ディスパッチ: std の feature 検出マクロが必要で、`no_std` の既定にできない。保留する。
- 純 Rust のスカラー実装: 命令の存在を保証できない。値の構築、ビット操作、値の意味論に使う。optional extensionのAPIはfeatureを要求し、呼び出し側が保証する。

## ツールチェーン

- `build.rs` は arch ごとに対象 triple とフラグを選ぶ。
- clang の triple は rustc の triple と一致しないことがある。`riscv64gc-unknown-linux-gnu` は clang では `riscv64-unknown-linux-gnu` と `-march=rv64gc` にする。ほかの triple は rustc と同じ名前を clang が受け付けることを確認した。
- x86_64 のホストビルドでは、clang の既定 triple が `x86_64-pc-linux-gnu`、rustc の LLVM triple も同じになる。`--target=x86_64-unknown-linux-gnu` を明示すると triple の不一致警告が出る。ホストでは `--target` を省くか、同じ triple を使う。
- clang、llvm-ar、llvm-objdump を使う。クロス LTO は LLVM のバージョンに依存する。テストした組み合わせを arch 文書に書く。
- wasm32 は clang の wasm32 backend と `wasm_simd128.h` が必要である。
- native では、その arch の全 extension の C をコンパイルする。参照されない関数はリンク時に落ちる。clang が feature を知らない場合は、ビルドを失敗させる。
- native で C をコンパイルするときは、ファイル全体の `-m` フラグか、関数単位の `target` 属性を使う。
- native の C は、Rust の target-features の部分集合になるようにする。x86 では `-mno-mmx -mno-x87 -mno-popcnt -mno-xsave` を付ける。
- `build.rs` は `CARGO_CFG_TARGET_FEATURE` を読み、Rust が有効にしている feature を C のフラグに反映する。Rust が有効にしていない feature は `-mno-*` で外す。ただし cfg に出ない baseline feature もあるため、最終的には IR か disassembly で確認する。
- LTO を使うときは、Rust と C の LTO モードを一致させる。Rust `-C lto=thin` と C `-flto=thin`、または Rust `-C lto=fat` と C `-flto` の組にする。
- `-ffreestanding`、`-fno-builtin`、`-fno-vectorize`、`-fno-slp-vectorize` を使い、意図しない命令を出さない。
- `build.rs` は C のパスと環境変数を `cargo:rerun-if-changed` と `cargo:rerun-if-env-changed` に登録する。ネットワークから取得しない。
