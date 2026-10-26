# x86 のリンクとゼロコスト

x86_64 で、C の関数を Rust の関数へインラインし、命令の間の store/load を消す条件を確認した。確認したのは rustc 1.99.1、clang/lld 23.1.1 である。

## x86固有の条件

LTO モード、`-C linker-plugin-lto`、`always_inline`、呼び出し側の feature、リンカという一般条件は [ゼロコストの確認](../zero-cost.md#条件) に従う。x86固有の条件は次のとおりである。

- C の target-features を Rust の target-features の部分集合にする。`avx512vl.c` は f/vl の baseline でコンパイルし、BW / DQ と IFMA / VBMI / VBMI2 / VNNI / BITALG / VPOPCNTDQ / VP2INTERSECT は関数単位の `target` 属性で有効にする。
- `-mno-mmx -mno-x87 -mno-popcnt -mno-xsave` を付ける。
- `cmov` と `cx8` は残ってもインラインされた。

## C のコンパイル

AVX2 の例である。

```sh
clang -O2 -ffreestanding -fno-builtin -fno-vectorize -fno-slp-vectorize \
  -mavx2 -mno-mmx -mno-x87 -mno-popcnt -mno-xsave -flto=thin -c add.c -o add.o
llvm-ar rcs libadd.a add.o
```

`build.rs` は `-flto=thin` のオブジェクトを静的ライブラリにし、`cargo:rustc-link-search` と `cargo:rustc-link-lib=static=...` を出す。

## Rust の関数

```rust
#[target_feature(enable = "avx2")]
unsafe fn kernel(a: &V256, b: &V256) -> V256 {
    unsafe { mul(&add(a, b), b) }
}
```

`add` と `mul` は `#[inline(always)]` のラッパーで、C の関数を呼ぶ。カーネルに `#[target_feature]` を付けると、C の関数がカーネルへインラインされる。

`#[inline(always)]` と `#[target_feature]` は同じ関数に付けられない（rustc 1.99.1、E0658）。SSE2 のラッパーは `#[inline(always)]` だけにする。AVX2 / AVX-512 のラッパーは機能を要求するため `#[target_feature]` を付け、`#[inline]` を使う。この形でも LTO で C がインラインされることを確認した。

## disassemblyによる確認

カーネルのシンボルを `llvm-nm` で探し、その範囲を `llvm-objdump` で見る。C への `call` は `callq *...` の間接呼び出しになることがあるため、シンボル名だけで探さない。

```sh
sym=$(llvm-nm target/release/app | grep -oE '_R[A-Za-z0-9_]*kernel' | head -1)
llvm-objdump -d --no-show-raw-insn target/release/app | sed -n "/<$sym>:/,/^$/p"
```

この範囲に `call` が無く、SIMD 命令が並ぶことを確認する。AVX2 の確認では、`vpaddd` の結果が `ymm1` のまま `vpmulld` に入り、間の store/load が無かった。入力のロードと出力のストアは関数境界にだけ残る。対象の fixture は `tests/x86_link.rs` と [値テストの fixture](../fixture-matrix.md) を正とする。

確認した feature と C のフラグは次のとおりである。すべて `-mno-mmx -mno-x87 -mno-popcnt -mno-xsave` を付けた。命令と結果は [ゼロコストの確認](../zero-cost.md) の結果表に置く。

| feature      | C のフラグ                                 |
| ------------ | ------------------------------------------ |
| `sse2`       | `-msse2`                                   |
| `avx2`       | `-mavx2`                                   |
| `avx512f`    | `-mavx512f`                                |
| `aes`        | `-maes`                                    |
| `fma`        | `-mfma`                                    |
| `avx10.1`    | `-mavx10.1`                                |
| `sha`        | `-msha`                                    |
| `avx512bw`   | `-mavx512bw`                               |
| `avx512vl`   | `-mavx512vl -mavx512bw`                    |
| `avx512dq`   | `-mavx512dq -mavx512f`                     |
| `avx512ifma` | `-mavx512ifma -mavx512f`                   |
| `avx512vbmi` | `-mavx512vbmi -mavx512bw`                  |
| `avx512vnni` | `-mavx512vnni -mavx512f`                   |
| `gfni`       | `-mgfni`                                   |
| `vaes`       | `-mvaes -mavx2`                            |
| `vpclmulqdq` | `-mvpclmulqdq -mavx2 -mavx512f -mavx512vl` |

## unstable feature

`avx10.1` は `#[target_feature]` を付けられない。クレート全体を `-C target-feature=+avx10.1` でコンパイルし、カーネルには `#[target_feature]` を付けない。C は `-mavx10.1 -mno-mmx -mno-x87 -mno-popcnt -mno-xsave` でコンパイルする。`vpaddd` と `vpmulld` が `zmm` のまま並び、`call` と store/load が消えることを確認した。

`-C target-feature=+avx10.1` は警告を出すが、LLVM の属性には入る。cfg は立たない。

`vpclmulqdq` は、256-bit の intrinsic が AVX2 / AVX-512F / AVX-512VL も要求する。Rust のカーネルにこれらの `#[target_feature]` を追加し、C のフラグを合わせると、`call` と store/load が消えることを確認した。

## 128-bit の `asm!`

128-bit は C を使わずに `asm!` と `u128` でもゼロコストにできる。`xmm_reg` に `u128` を渡し、`paddd` と `pmulld` を並べると、間の store/load が消えた。256/512-bit は stable な値型が無いため、C + LTO を使う。

## 環境差と失敗時の挙動

- ホストの x86_64 では、clang の既定 triple は `x86_64-pc-linux-gnu` である。`--target=x86_64-unknown-linux-gnu` を明示すると triple の不一致警告が出る。ホストでは `--target` を省く。
- `lto` が無効なプロファイル、LTO モードの不一致、feature 集合の不一致では、C への `call` が残る。一般条件は [ゼロコストの確認](../zero-cost.md#条件) を見る。
- unstable な feature は `#[target_feature]` を付けられない。クレート全体を `-C target-feature=+name` でコンパイルした場合だけ、LTO でインラインできる。
- この結果は x86_64 のものである。他の arch の結果は [ゼロコストの確認](../zero-cost.md) にある。
