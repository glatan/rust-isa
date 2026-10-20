# x86 のリンクとゼロコスト

x86_64 で、C の関数を Rust の関数へインラインし、命令の間の store/load を消す条件を確認した。確認したのは rustc 1.99.1、clang/lld 23.1.1 である。

## 条件

1. Rust と C の LTO モードを一致させる。
   - Rust `lto = "thin"` と C `-flto=thin`
   - Rust `lto = "fat"` と C `-flto`
2. Rust に `-C linker-plugin-lto` を付ける。
3. C の target-features を Rust の target-features の部分集合にする。
   - `-mno-mmx -mno-x87 -mno-popcnt -mno-xsave` を付ける。
   - `cmov` と `cx8` は残ってもインラインされた。
4. C の関数を `__attribute__((always_inline))` にする。
5. 呼び出し側の関数に `#[target_feature]` を付けるか、`-C target-feature=+...` を付ける。
6. LTO を扱えるリンカを使う。clang + lld で確認した。

## Cargo の設定

`-C lto` を `RUSTFLAGS` に入れない。build script の `embed-bitcode=no` と衝突する。最終バイナリの profile に設定する。

```toml
[profile.release]
lto = "thin"

[build]
rustflags = ["-C", "linker-plugin-lto", "-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
```

C は `-flto=thin` でコンパイルする。`lto = "fat"` の場合は C も `-flto` にする。`-C linker-plugin-lto` が無いと、C への `call` が残る。`lto = "off"` と C `-flto=thin` の組でもインラインされたが、明示的な thin または fat を推奨する。

ライブラリクレートとバイナリクレートに分けた Cargo workspace でも、同じ設定で C の関数がカーネルへインラインされることを確認した。

`-C linker-plugin-lto` を付けたときの LTO モードの組み合わせは次のとおりである。

| Rust の profile | C のフラグ | カーネル内の `call` |
| --- | --- | --- |
| `lto = "off"` | なし | 残る |
| `lto = "off"` | `-flto=thin` | 消える |
| `lto = "thin"` | `-flto=thin` | 消える |
| `lto = "thin"` | `-flto` | 残る |
| `lto = "fat"` | `-flto` | 消える |
| `lto = "fat"` | `-flto=thin` | 残る |

`-C linker-plugin-lto` が無い場合は、C が `-flto=thin` でも `call` が残る。

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

`#[inline(always)]` と `#[target_feature]` は同じ関数に付けられない（rustc 1.99.1、E0658）。ラッパーは `#[inline(always)]` だけにする。

## 確認

カーネルのシンボルを `llvm-nm` で探し、その範囲を `llvm-objdump` で見る。C への `call` は `callq *...` の間接呼び出しになることがあるため、シンボル名だけで探さない。

```sh
sym=$(llvm-nm target/release/app | grep -oE '_R[A-Za-z0-9_]*kernel' | head -1)
llvm-objdump -d --no-show-raw-insn target/release/app | sed -n "/<$sym>:/,/^$/p"
```

この範囲に `call` が無く、SIMD 命令が並ぶことを確認する。AVX2 の確認では、`vpaddd` の結果が `ymm1` のまま `vpmulld` に入り、間の store/load が無かった。入力のロードと出力のストアは関数境界にだけ残る。

確認した feature と命令は次のとおりである。すべて `-mno-mmx -mno-x87 -mno-popcnt -mno-xsave` を付けた。

| feature | C のフラグ | 確認した命令 |
| --- | --- | --- |
| `sse2` | `-msse2` | `paddd` |
| `avx2` | `-mavx2` | `vpaddd`、`vpmulld` |
| `avx512f` | `-mavx512f` | `vpaddd` |
| `aes` | `-maes` | `aesenc` |
| `fma` | `-mfma` | `vfmadd` |
| `avx10.1` | `-mavx10.1` | `vpaddd`、`vpmulld` |
| `sha` | `-msha` | `sha256rnds2` |
| `avx512bw` | `-mavx512bw` | `vpaddb` |
| `gfni` | `-mgfni` | `gf2p8affineqb` |
| `vaes` | `-mvaes -mavx2` | `vaesenc` |

## unstable feature

`avx10.1` は `#[target_feature]` を付けられない。クレート全体を `-C target-feature=+avx10.1` でコンパイルし、カーネルには `#[target_feature]` を付けない。C は `-mavx10.1 -mno-mmx -mno-x87 -mno-popcnt -mno-xsave` でコンパイルする。`vpaddd` と `vpmulld` が `zmm` のまま並び、`call` と store/load が消えることを確認した。

`-C target-feature=+avx10.1` は警告を出すが、LLVM の属性には入る。cfg は立たない。

`vpclmulqdq` は、256-bit の intrinsic が AVX2 / AVX-512VL も要求する。C と Rust の feature 集合を追加で合わせる必要があり、まだ確認していない。

## 128-bit の `asm!`

128-bit は C を使わずに `asm!` と `u128` でもゼロコストにできる。`xmm_reg` に `u128` を渡し、`paddd` と `pmulld` を並べると、間の store/load が消えた。256/512-bit は stable な値型が無いため、C + LTO を使う。

## 注意

- ホストの x86_64 では、clang の既定 triple は `x86_64-pc-linux-gnu` である。`--target=x86_64-unknown-linux-gnu` を明示すると triple の不一致警告が出る。ホストでは `--target` を省く。
- `lto` が無効なプロファイル、LTO モードの不一致、feature 集合の不一致では、C への `call` が残る。feature 集合を部分集合にできない extension は、ゼロコストを主張しない。
- unstable な feature は `#[target_feature]` を付けられない。クレート全体を `-C target-feature=+name` でコンパイルした場合だけ、LTO でインラインできる。
- この結果は x86_64 のものである。他の arch の結果は [ゼロコストの確認](../zero-cost.md) にある。
