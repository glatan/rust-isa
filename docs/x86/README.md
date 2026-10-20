# x86

対象は `x86_64-unknown-linux-gnu` と `i686-unknown-linux-gnu` である。x86_64 を先に扱う。i686 は `sse2` と `avx2` のゼロコストを確認した。ほかの extension はまだ測っていない。

family は命令の種類に合わせて決める（`simd`、`crypto`、`integer` など）。公開パスは `x86::{family}::{extension}::{instruction}` とする。extension の名前は rustc の feature 名の `.` と `-` を `_` に置き換える（`sse4.1` は `sse4_1`、`avx10.1` は `avx10_1`）。

## 実装状況

`sse2` の `v128_load` / `v128_store`、`paddb` / `psubb` / `pcmpeqb`、`paddw` / `psubw` / `pcmpeqw` / `pmullw`、`paddd` / `psubd` / `pcmpeqd`、`paddq` / `psubq`、`pand` / `por` / `pxor` を実装してある。ほかの命令は未実装である。

## 実装方式

- native なので無条件に公開する。任意の feature を要求する命令は `unsafe fn` とする。
- C + clang + LTO を主方式とする。LTO は wasm32 と native のゼロコストに使う。C は `<immintrin.h>` を使う。
- 値は `V128`、`V256`、`V512` をポインタで渡す。C の関数はベクタ型を内部で使い、境界ではポインタを受け取る。
- C は extension ごとに分ける。`-mavx2` のようなファイル全体のフラグを使うか、`__attribute__((target("avx2")))` を関数単位で付ける。
- stable な feature は、呼び出し側のカーネルに `#[target_feature(enable = "...")]` を付ける。ラッパーは `#[inline(always)]` だけにし、`#[target_feature]` を付けない。unstable な feature は付けない。
- `asm!` は補助として使える。`xmm_reg` は `u128` を受け付ける。`ymm_reg` は `avx` を要求し、256-bit の値型を `core::arch` なしでは表せないため、256-bit 以上は C を主とする。

## ゼロコスト

x86_64 で条件付きのゼロコストを確認した。

- Rust と C の LTO モードを一致させる（Rust `-C lto=thin` と C `-flto=thin`、または fat/fat）。
- `-C linker-plugin-lto` を付ける。
- C の target-features を Rust の部分集合にする。`-mno-mmx -mno-x87 -mno-popcnt -mno-xsave` を付ける。
- C の関数を `__attribute__((always_inline))` にする。
- 呼び出し側の関数に `#[target_feature]` を付けるか、`-C target-feature=+...` を付ける。
- LTO を扱えるリンカを使う。clang + lld で確認した。

確認した命令は、SSE2 の `paddd`、AVX2 の `vpaddd`、`vpmulld`、`vpshufb`、AVX-512F の `vpaddd`、AES の `aesenc`、FMA の `vfmadd`、AVX10.1 の `vpaddd` と `vpmulld`、SHA の `sha256rnds2`、AVX-512BW の `vpaddb`、AVX-512VL の `vpaddb`、AVX-512DQ の `vpmullq`、AVX-512IFMA の `vpmadd52luq`、AVX-512VBMI の `vpermb`、AVX-512VNNI の `vpdpbusd`、GFNI の `gf2p8affineqb`、VAES の `vaesenc`、VPCLMULQDQ の `vpclmulqdq` である。AVX2 の確認では、`vpaddd` の結果が `ymm1` のまま `vpmulld` に入り、間の store/load が無かった。

Cargo では `-C lto` を `RUSTFLAGS` に入れない。build script の `embed-bitcode=no` と衝突する。最終バイナリの profile に設定する。

```toml
[profile.release]
lto = "thin"

[build]
rustflags = ["-C", "linker-plugin-lto", "-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
```

C は `-flto=thin` でコンパイルする。`lto = "fat"` の場合は C も `-flto` にする。

条件を満たさないビルドでは、C 関数への `call` が残る。x86 の既定は呼び出しベースである。

128-bit は `asm!` と `u128` でもゼロコストにできる。`xmm_reg` に `u128` を渡し、2 命令を並べると、`paddd` と `pmulld` の間の store/load が消えることを確認した。256/512-bit は stable な値型が無いため、C + LTO を使う。

## ツールチェーン

- clang、llvm-ar、llvm-objdump。
- clang 23.1.1 で、`-mavx2` と `__attribute__((target("avx2")))` が `vpaddd ymm` を出すことを確認した。
- C の target-features を Rust の部分集合にするため、`-mno-mmx -mno-x87 -mno-popcnt -mno-xsave` を付ける。`cmov` と `cx8` は残ってもインラインされた。
- ホストの x86_64 では、clang の既定 triple は `x86_64-pc-linux-gnu` である。`--target=x86_64-unknown-linux-gnu` を明示すると triple の不一致警告が出る。ホストでは `--target` を省く。
- ゼロコストの確認は rustc 1.99.1、clang/lld 23.1.1 で行った。クロス LTO は LLVM のバージョンに依存する。テストした組み合わせをここに追記する。
- AMX は `-mamx-tile -mamx-bf16` で C がコンパイルでき、`ldtilecfg`、`tileloadd`、`tilestored`、`tilerelease` を出した。タイル状態の guard が必要である。

## 実装の順序

1. `sse2`（x86_64 の既定）
2. `ssse3`、`sse4.1`、`sse4.2`
3. `avx`、`avx2`、`fma`
4. `avx512f` とその拡張
5. `aes`、`sha`、`pclmulqdq`
6. `bmi1`、`bmi2`、`popcnt` など

## 未決定

- native のゼロコストをどう実現するか（LTO の設定、`asm!` の範囲）。
- AMX のタイル状態を API に出すか。
- i686 の値テストと命令の確認。

## 文書

- [実装可否](feasibility.md)
- [リンクとゼロコスト](linking.md)
- [実装方針](../policy.md)
