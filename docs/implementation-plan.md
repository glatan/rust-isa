# 実装計画

この文書は、`rust-isa` の native arch 実装を進める順序と、各段階の完了条件を定める。方針は [実装方針](policy.md)、設計は [スケーラブルベクタと状態](scalable-state.md) にある。

## 前提

- 実装済みなのは `wasm32` の `simd128` と `relaxed-simd` だけである。
- native の命令 API はまだない。
- `core::arch` は使わない。C + clang + LTO を主方式とし、`asm!` を補助に使う。
- ゼロコストは命令ごとに確認する。算術・論理の一部は zero-cost、intrinsics 依存の shuffle / compress は往復が残る場合がある。
- 開発イメージには `qemu-user-static` と arch ごとの runner が入っている。
- native の値テストは、API の実装後に `tests/` へ追加する。手順は [native の値テスト](native-testing.md) にある。

## 完了条件

各 extension は、次を満たしたときに完了とする。

1. 公開 API が `{arch}::{family}::{extension}::{instruction}` に置かれている。
2. 値の型と safe / `unsafe` の境界が rustdoc に書かれている。
3. 値テストがホストまたは QEMU で通る。
4. ゼロコストを主張する命令は、fixture の disassembly で `call` と往復が無いことを確認する。
5. C と Rust のシンボルが一致する。
6. `docs/<arch>/README.md` と `feasibility.md` が更新されている。
7. `make test` が通る。

## 実装の順序

| 段階 | 対象 | 内容 | 目安 |
| --- | --- | --- | --- |
| 0 | 共通 | build.rs、値の型、native テスト fixture（一部） | M |
| 1 | x86_64 | SSE2、AVX2、AVX-512 の代表、暗号・整数の一部を実装済み。i686 の SSE2/AVX2 値テスト済み | L |
| 2 | AArch64 | NEON、dotprod、SVE スライス API、暗号 | L |
| 3 | RISC-V | 固定長 V128、zba など、スライス API | L |
| 4 | LoongArch | LSX、LASX | M |
| 5 | s390x | vector | M |
| 6 | 32-bit ARM | NEON | M |
| 7 | PowerPC | Altivec、VSX | M |
| 8 | MIPS | MSA | S |
| 9 | wasm32 | wide-arithmetic、整理 | S |

## 段階 0: 共通基盤

最初に、arch を足すための共通部分を作る。

- `build.rs` を arch ごとの表に変える。
  - rustc の target と clang の target の対応。
  - C のフラグ、`-flto=thin`、静的ライブラリ名。
  - `CARGO_CFG_TARGET_FEATURE` を読み、C の feature 集合を Rust の部分集合に寄せる。
  - 対象外の arch では C をビルドしない。
- 値の型を arch ごとに置く。最初は `V128`、必要になったら `V256`、`V512`。
- `fixtures/native-values` を作る。
  - `no_std` の `_start`、終了コードで合否。
  - QEMU の runner を使う。
  - MIPS は lane ごとの関数、PowerPC は `.TOC.`、i686 は `-mstackrealign`。
- `tests/driver.rs` に native のビルドと実行を追加する。`make test` を重くしすぎる場合は `make test-native` に分ける。
- 文書のテンプレートを `docs/<arch>/` に置く。

完了条件:

- x86_64 の最小の命令 1 つが、API、値テスト、disassembly、文書まで通る。
- native のテストが QEMU で 1 arch 動く。

## 段階 1: x86_64

x86_64 を最初に実装する。ホストで値テストを実行でき、QEMU なしで確認できるためである。

### 1a. SSE2

進捗: `V128`、load / store、整数の加減算・比較・乗算・平均・unpack・shift、浮動小数点の加減乗除・比較・平方根、bitwise、lane shuffle を実装した。`paddd` のゼロコストを確認した。i686 の値テストはまだない。

- `V128`、load / store、整数・浮動小数点の加減乗除、比較、shuffle。
- SSE2 は既定なので、演算は safe、メモリ操作は `unsafe`。
- `paddd`、`pmuludq`、`pshufb` などの zero-cost を fixture で確認する。
- i686 でも同じ API を提供する。値テストは QEMU-i386 で行う。

### 1b. AVX2

進捗: `V256`、load / store、整数の加減算・乗算・比較、bitwise、`vpshufb`、`vpermd`、`vpbroadcastd`、shift を実装した。`vpaddd` と `vpmulld` のゼロコストを確認した。

- `V256`、`#[target_feature(enable = "avx2")]`、`unsafe fn`。
- `vpaddd`、`vpmulld`、`vpshufb` の zero-cost。
- 呼び出し側のカーネルに feature を付ける形を rustdoc に書く。

### 1c. AVX-512

進捗: `V512`、load / store、AVX-512F の整数加減算・乗算・bitwise・`vpermd`、AVX-512BW の byte 加減算と 16-bit 乗算、AVX-512DQ の `vpmullq`、AVX-512IFMA、AVX-512VBMI、AVX-512VNNI、AVX-512VL の 128/256-bit `vpmullq` を実装した。AVX-512F のゼロコストを確認した。

- `V512`、AVX-512F / BW / VL / DQ / IFMA / VBMI / VNNI。
- Rust の feature 名と C のフラグを一致させる。VPCLMULQDQ は AVX2 / AVX-512F / AVX-512VL も必要。
- AVX10.1 は global flag で提供する。`#[target_feature]` は使えない。

### 1d. 暗号・整数

進捗: AES、SHA、GFNI、PCLMULQDQ、VAES、VPCLMULQDQ、FMA、BMI1、BMI2、POPCNT の代表命令を実装した。`crypto-link` で `aesenc` と `vfmadd` のゼロコストを確認した。

- AES、SHA、GFNI、VAES、VPCLMULQDQ、FMA、BMI1/2、POPCNT など。
- C の feature 集合を Rust の部分集合にする。

完了条件:

- SSE2 と AVX2 の主要命令が揃う。
- AVX-512 の代表的な extension が動く。
- x86_64 と i686 の値テストが通る。

## 段階 2: AArch64

- `V128`、NEON の加減乗除、比較、`tbl`。
- NEON は既定なので safe。メモリ操作は `unsafe`。
- `dotprod`、`fp16`、`bf16`、`i8mm` は `unsafe fn`。
- AES、SHA2、SHA3、SM4、CRC。
- SVE / SVE2 はスライス API だけを提供する。VL は C が決める。
- SME は `SmeGuard` の設計に従う。実装は後半。

完了条件:

- NEON の主要命令と `sdot`、`tbl` の zero-cost。
- QEMU の値テスト。
- SVE のスライス API が 1 つ動く。

## 段階 3: RISC-V

- 固定長 `V128` の V 拡張。利用者は `-C target-feature=+v` を付ける。
- `zba`、`zbb`、`zbc`、`zbs`、`zk*` は stable なので cfg ゲートも使える。
- スライス API は C が `vsetvl` で VL を決める。
- `vrgather`、`vcompress` は往復が残るため、zero-cost を主張しない。
- 値テストは QEMU-riscv64。

完了条件:

- 固定長 V128 の算術とスライス API。
- stable な整数・暗号 extension。

## 段階 4: LoongArch

- LSX の `V128` と LASX の `V256`。
- LSX は既定、LASX は安定。
- 算術は GCC vector 拡張で zero-cost。`xvshuf.b` は往復が残る。
- 値テストは QEMU-loongarch64。

## 段階 5: s390x

- `V128` の vector。
- target-cpu は `z10`。C は `-Xclang -target-cpu z10` と `+vector` を明示する。
- `vaf`、`vmlf`、`vperm` の zero-cost。
- ビッグエンディアンのレーン順序を rustdoc に書く。
- 値テストは QEMU-s390x。

## 段階 6: 32-bit ARM

- `V128` の NEON。
- `neon` は不安定なので、利用者はクレート全体を `-C target-feature=+neon` でコンパイルする。
- C の feature 集合を Rust に合わせるため、`-Xclang -target-feature` を使う。
- 128-bit の table lookup は無い。`V128` shuffle は出さず、必要なら `V64` を別に設計する。
- 値テストは QEMU-arm。

## 段階 7: PowerPC

- `V128` の Altivec / VSX。
- `target-cpu` と feature 集合を合わせる。
- `vadduwm`、`vmuluwm`、`vperm` の zero-cost。
- LTO のリンクには `.TOC.` を参照する非 LTO オブジェクトを追加する。
- 値テストは QEMU-ppc64le。
- MMA は `MmaAccumulator` の設計後。

## 段階 8: MIPS

- `V128` の MSA。
- `msa` は不安定なので、global flag で提供する。
- C は `-march=mips64r2 -mmsa -mno-abicalls -G0`、Rust は `+msa,+fp64`。
- lane ごとに独立した C 関数で値テストする。
- `llvm-objdump` は MSA 命令を表示しないため、`llvm-mc` でデコードする。
- 優先度は低い。

## 段階 9: wasm32

- 既存の `simd128` / `relaxed-simd` は変更しない。
- `wide-arithmetic` を Cargo feature で opt-in する。
- 有効時にモジュール全体が `wide-arithmetic` を要求することを rustdoc に書く。

## テスト

| 対象 | ビルド | 値テスト | ゼロコスト |
| --- | --- | --- | --- |
| x86_64 | ホスト | ホスト | ホストの disassembly |
| i686 | `--target i686` | QEMU-i386 | disassembly |
| AArch64 | `--target aarch64` | QEMU-aarch64 | disassembly |
| ARMv7 | `--target armv7` | QEMU-arm | disassembly |
| RISC-V | `--target riscv64gc` | QEMU-riscv64 | disassembly |
| LoongArch | `--target loongarch64` | QEMU-loongarch64 | disassembly |
| s390x | `--target s390x` | QEMU-s390x | `llvm-mc` |
| PowerPC | `--target powerpc64le` | QEMU-ppc64le | disassembly |
| MIPS | `--target mips64` | QEMU-mips64 | `llvm-mc` |
| wasm32 | `--target wasm32-wasip1` | wasmtime | `wasm2wat` |

## リスク

- zero-cost は命令ごとに異なる。shuffle / compress は intrinsics で往復が残る場合がある。
- C の feature 集合を Rust の部分集合に合わせる作業は clang の実装に依存する。
- native の LTO は利用者の `profile.release.lto` と `-C linker-plugin-lto` に依存する。
- `#[inline(always)]` と `#[target_feature]` は併用できない。
- PowerPC の `.TOC.`、MIPS の codegen、ARMv7 の feature 調整は arch 固有の作業になる。
- QEMU は静的リンクのバイナリだけを動かす。テスト fixture は `-static` で作る。

## 最初のマイルストーン

最初は x86_64 の SSE2 で、`paddd` 1 命令を端から端まで通す。

1. `src/x86/simd/sse2/` と C を置く。
2. `V128`、`paddd`、`v128_load` / `v128_store` を実装する。
3. ホストの値テストを追加する。
4. fixture を disassembly し、`paddd` が `call` と往復なしで残ることを確認する。
5. `docs/x86/README.md` と `feasibility.md` を更新する。

この 1 命令で、build.rs、API、テスト、zero-cost 確認、文書の型を固める。その後、SSE2 の全体、AVX2、AVX-512 へ広げる。
