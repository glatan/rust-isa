# ゼロコストの確認

この文書は、native のゼロコストをどのように確認したかと、arch ごとの結果をまとめる。条件と方針は [実装方針](policy.md) に置く。

確認した環境は rustc 1.99.1、clang/lld 23.1.1 である。

## 方法

ホスト以外の arch でも disassembly だけを確認できるように、`no_std` と `no_main` のバイナリを `build-std` でクロスビルドする。

- Rust は `-Z build-std=core`、`profile.release.lto = "thin"`、`-C linker-plugin-lto`。
- C は clang で `-flto=thin`、`-ffreestanding`、`-fno-builtin`、`-fno-vectorize`、`-fno-slp-vectorize`。
- C の target-features は Rust の部分集合にする。
- clang + lld でリンクする。libc は使わない。
- `_start` は `loop {}` にし、入力は `core::hint::black_box` で隠す。
- 2 命令を合成するカーネルを `llvm-objdump` で見て、`call` と命令間の store/load が無いことを確認する。

Rust のバイナリは次の形で作る。

```sh
RUSTC_BOOTSTRAP=1 cargo build -Z build-std=core --target <target> --release
```

`-Z build-std` は nightly の機能である。`RUSTC_BOOTSTRAP=1` は検証用であり、クレートの利用者に要求しない。

アーキテクチャによっては、`llvm-objdump` がベクタ命令を表示できない。s390x では、`llvm-objdump` が出したバイト列を `llvm-mc --disassemble --triple=s390x-unknown-linux-gnu --mattr=+vector` でデコードした。

## 結果

| arch | extension | 確認した命令 | `call` | 命令間の store/load | 結果 |
| --- | --- | --- | --- | --- | --- |
| x86_64 | `sse2` | `paddd` | なし | なし | ゼロコスト |
| i686 | `sse2` | `paddd`、`pmuludq` | なし | なし | ゼロコスト |
| x86_64 | `avx2` | `vpaddd`、`vpmulld` | なし | なし | ゼロコスト |
| x86_64 | `avx512f` | `vpaddd` | なし | なし | ゼロコスト |
| x86_64 | `aes` | `aesenc` | なし | なし | ゼロコスト |
| x86_64 | `fma` | `vfmadd` | なし | なし | ゼロコスト |
| x86_64 | `avx10.1` | `vpaddd`、`vpmulld` | なし | なし | ゼロコスト |
| x86_64 | `sha` | `sha256rnds2` | なし | なし | ゼロコスト |
| x86_64 | `avx512bw` | `vpaddb` | なし | なし | ゼロコスト |
| AArch64 | `neon` | `add v0.4s`、`mul v0.4s` | なし | なし | ゼロコスト |
| RISC-V | `v` | `vadd.vv`、`vmul.vv` | なし | なし | ゼロコスト |
| LoongArch | `lasx` | `xvadd.w`、`xvmul.w` | なし | なし | ゼロコスト |
| s390x | `vector` | `vaf`、`vmlf` | なし | なし | ゼロコスト |
| 32-bit ARM | `neon` | `vadd.i32`、`vmul.i32` | なし | なし | ゼロコスト |
| PowerPC | `altivec` | `vadduwm`、`vmuluwm` | なし | なし | ゼロコスト |
| MIPS | `msa` | `addv.w`、`mulv.w` | なし | なし | ゼロコスト |

## IR レベルの確認

PowerPC は ELF のリンクが `.TOC.` で失敗したため、最初は Rust と C の bitcode を `llvm-link` し、`opt -passes="default<O3>"` で最適化して IR を見た。

- PowerPC: カーネルに `add <4 x i32>` と `mul <4 x i32>` が並び、`c_add` / `c_mul` への `call` と命令間の store/load が無かった。その後、`.TOC.` を参照する非 LTO オブジェクトをリンクに加えることで、バイナリでも `vadduwm` と `vmuluwm` を確認した。
- MIPS: feature 集合を合わせると、カーネルに `addv.w` と `mulv.w` が並び、`c_add` / `c_mul` への `call` と命令間の store/load が無かった。C は `-march=mips64r2 -mmsa -mabicalls -mxgot`、Rust は `-C target-feature=+msa,+fp64` を使う。

## 未確認

現在、ゼロコストを確認していない arch はない。値テストは native の多くの arch で未整備である。

イメージには `qemu-user-static` を入れ、静的な `no_std` テストを QEMU で実行した。

- AArch64、ARMv7、RISC-V、LoongArch、s390x、i686 は値テストが通った。
- PowerPC は Rust から C を呼ぶ harness で lane 3 が一致しなかった。C 単体の値テストは通った。
- MIPS は Rust から C を呼ぶ harness で lane 3 が一致しなかった。lane ごとの値テストでは、すべて 15 になった。

QEMU の runner と確認方法は [開発環境](development.md) に書く。

## スケーラブルベクタ

SVE のスライス API を AArch64 で確認した。C のループを `#[target_feature(enable = "sve")]` の Rust カーネルへインラインすると、`cntw`、`whilelt`、`ld1w`、`add z0.s`、`st1w` がカーネルに並び、`call` が無かった。

RVV のスライス API も RISC-V で確認した。C のループを Rust のカーネルへインラインすると、`vsetvli`、`vle32.v`、`vadd.vv`、`vse32.v` がカーネルに並び、`call` が無かった。

どちらも命令ごとの API ではなく、スライスに対するループになる。固定長 API の「命令間の store/load」とは別に扱う。

## Rust の属性

rustc 1.99.1 では、`#[inline(always)]` と `#[target_feature]` を同じ関数に付けられない（E0658）。

- ラッパーは `#[inline(always)]` だけにし、呼び出し側のカーネルに `#[target_feature]` を付ける。
- ラッパーに `#[target_feature]` を付ける場合は、`#[inline(always)]` を外す。`#[inline]` は併用できる。C の関数が LTO でラッパーへインラインされることを確認する。x86_64 では、この形でも `call` が消え、`vpaddd` と `vpmulld` が並ぶことを確認した。

## 128-bit の `asm!`

x86_64 では、`asm!` の `xmm_reg` に `u128` を渡せる。`paddd` と `pmulld` を並べると、間の store/load が消えた。

AArch64 では、`asm!` の `vreg` が `u128` を受け付けない。`vreg` が受け付ける `i64x2` や `f128` は unstable である。このため、AArch64 の NEON は C + clang を使う。

## 実装上の注意

### C の書き方

C は、可能なら GCC の vector 拡張と通常のロード・ストアで書く。

```c
typedef int v4i32 __attribute__((vector_size(16)));
__attribute__((always_inline)) void c_add(const int *a, const int *b, int *out) {
  v4i32 x = *(const v4i32 *)a;
  v4i32 y = *(const v4i32 *)b;
  *(v4i32 *)out = x + y;
}
```

RISC-V と LoongArch では、次の intrinsics を使うと、C がインラインされても一時値の store/load が残った。

- RISC-V: `__riscv_vle32_v_i32m1` / `__riscv_vse32_v_i32m1`
- LoongArch: `__lasx_xvld` / `__lasx_xvst`

GCC の vector 拡張に置き換えると、どちらも store/load が消えた。intrinsics が必要な命令は、命令ごとに disassembly を確認する。

### target-cpu と feature 集合

- **RISC-V**: `-mno-relax` を付けないと、C の関数がインラインされなかった。C は `-march=rv64gcv -mabi=lp64d -mno-relax`。
- **s390x**: Rust の target-cpu は `z10` である。C を `-march=z10 -mzvector` でコンパイルすると一致した。`-march=z13 -mzvector` では `+vector-enhancements-1` と `+transactional-execution` が付き、インラインされなかった。
- **AArch64**: `-march=armv8-a+simd` で一致した。
- **LoongArch**: `-mlasx` で一致した。
- **MIPS**: C は `-march=mips64r2 -mmsa -mabicalls -mxgot`、Rust は `-C target-feature=+msa,+fp64` を使う。`llvm-objdump` は MSA 命令を `<unknown>` と表示するため、バイト列を `llvm-mc --disassemble --triple=mips64-unknown-linux-gnuabi64 --mattr=+msa` でデコードする。
- **PowerPC**: `.TOC.` は lld が `addReservedSymbols` で定義するが、LTO 後のオブジェクトにだけ参照があると定義されない。`.TOC.` を参照する非 LTO オブジェクトをリンクに加えると解決する。`.quad .TOC.` を持つ小さなアセンブリオブジェクトで確認した。
- **32-bit ARM**: C に `+armv7-a`、`+dsp`、`+fp64`、`+vfp2sp` などが付き、そのままでは Rust の feature 集合と一致しなかった。`-mthumb -march=armv7-a -mfpu=neon -mfloat-abi=hard` に加え、`-Xclang -target-feature` で次を調整すると一致した。
  - 外す: `armv7-a`、`dsp`、`fp64`、`vfp2sp`、`vfp3sp`、`vfp3d16sp`、`read-tp-tpidruro`、`thumb-mode`
  - 足す: `v7`、`thumb2`
  - この調整は clang の実装に依存する。build.rs に固定し、disassembly で確認する。

### unstable feature

x86 の `avx10.1` は `#[target_feature]` を付けられない。クレート全体を `-C target-feature=+avx10.1` でコンパイルし、C を `-mavx10.1` でコンパイルすると、`#[target_feature]` を付けないカーネルでも `vpaddd` と `vpmulld` が `zmm` のまま並び、`call` と store/load が消えた。`-C target-feature` は警告を出すが、LLVM の属性には入る。cfg は立たない。

### feature 集合の確認方法

C の feature 集合は、LLVM IR を出して確認できる。

```sh
clang -O2 -mavx2 -mno-mmx -mno-x87 -mno-popcnt -mno-xsave -S -emit-llvm c.c -o c.ll
grep -o 'target-features"="[^"]*' c.ll | head -1
```

Rust 側は、`--emit=llvm-ir` か `-C save-temps` で出した bitcode を `llvm-dis` で見る。

```sh
llvm-dis path/to/crate.rcgu.bc -o - | grep -o 'target-features"="[^"]*' | sort -u
```

C の集合が Rust の集合の部分集合になるまで、`-mno-*` や `-Xclang -target-feature` で調整する。`cmov` と `cx8` のような baseline の feature は残ってもインラインされた。`dsp`、`fp64`、`popcnt`、`xsave`、`relax` のような追加 feature はインラインを妨げることがある。

`build.rs` は `CARGO_CFG_TARGET_FEATURE` を読める。Rust が有効にしている feature を C のフラグに反映し、有効にしていない feature を `-mno-*` で外す。ただし `CARGO_CFG_TARGET_FEATURE` に現れない baseline feature もあるため、最終確認は IR か disassembly で行う。

確認した値の例:

| 条件 | `CARGO_CFG_TARGET_FEATURE` |
| --- | --- |
| x86_64 の既定 | `fxsr,sse,sse2` |
| x86_64 `-C target-feature=+avx2` | `avx,avx2,fxsr,sse,sse2,sse3,sse4.1,sse4.2,ssse3` |
| AArch64 の既定 | `neon,outline-atomics` |
| AArch64 `-C target-feature=+sve` | `fp16,neon,outline-atomics,sve` |

### LTO

- Rust と C の LTO モードを一致させる。`thin` と `thin`、または `fat` と `fat`。
- `thin` と `fat` の両方を x86_64、AArch64、RISC-V で確認した。
- `-C linker-plugin-lto` が無いと、C への `call` が残る。
- Cargo では `-C lto` を `RUSTFLAGS` に入れない。build script の `embed-bitcode=no` と衝突する。`profile.release.lto` に設定する。

## 再現の例

x86_64 の例は [x86 のリンク](x86/linking.md) にある。ほかの arch は、`no_std` と `no_main` のバイナリを作り、`build-std` と clang/lld でリンクする。

```toml
[profile.release]
lto = "thin"
panic = "abort"
```

```toml
[build]
rustflags = [
  "-C", "linker-plugin-lto",
  "-C", "linker=clang",
  "-C", "link-arg=--target=<clang-target>",
  "-C", "link-arg=-nostdlib",
  "-C", "link-arg=-fuse-ld=lld",
]
```
