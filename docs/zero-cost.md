# ゼロコストの確認

この文書は、native のゼロコストの条件、確認方法、arch ごとの結果をまとめる。実装方針は [実装方針](policy.md) に置く。`make zero-cost-check` は、結果表の fixture と kernel の実在、`fixtures/*-link` の網羅、LTO / call / store-load / 結果の値、ゼロコスト行の整合を検査する。

確認した環境は rustc 1.99.1、clang/lld 23.1.1 である。ARM、AArch64、s390x、PowerPC の一部は clang 23.1.3 で再確認した。

## 条件

次の条件を満たすビルドで、C の関数が Rust の関数へインラインされ、命令と命令の間の store/load が消える。

1. Rust と C の LTO モードを一致させる。
   - Rust `-C lto=thin` と C `-flto=thin`
   - Rust `-C lto=fat` と C `-flto`
2. `-C linker-plugin-lto` を付ける。これが無いと C への `call` が残る。
3. C の target-features を Rust の target-features の部分集合にする。
   - x86 では clang が `mmx`、`x87`、`popcnt`、`xsave` を暗黙に付けることがある。`-mno-mmx -mno-x87 -mno-popcnt -mno-xsave` で外す。
   - `cmov` と `cx8` は残ってもインラインされた。
4. C の関数を `always_inline` にする。
5. 呼び出し側の関数に `#[target_feature]` を付けるか、`-C target-feature=+...` を付ける。
6. LTO を扱えるリンカを使う。x86_64 では clang + lld で確認した。環境の既定リンカでも動いたが、再現性のため clang + lld を推奨する。

Cargo では、`-C lto` を `RUSTFLAGS` に入れない。build script の `embed-bitcode=no` と衝突する。最終バイナリの profile に設定する。

```toml
[profile.release]
lto = "thin"

[build]
rustflags = ["-C", "linker-plugin-lto", "-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
```

C は `-flto=thin` でコンパイルする。`lto = "fat"` の場合は C も `-flto` にする。`lto = "off"` と C `-flto=thin` の組でもインラインされたが、明示的な thin または fat を推奨する。LTO モードが一致しない場合は、C への `call` が残る。`thin` と `fat` の両方を、x86_64、i686、AArch64、ARMv7、RISC-V、LoongArch、s390x、PowerPC、MIPS で確認した。

確認した命令は、x86_64 の SSE2 `paddd`、AVX2 `vpaddd` と `vpmulld`、AVX-512F `vpaddd`、AES `aesenc`、FMA `vfmadd`、AArch64 の NEON `add v0.4s` と `mul v0.4s`、RISC-V の `vadd.vv` と `vmul.vv`、LoongArch の `xvadd.w` と `xvmul.w`、s390x の `vaf` と `vmlf`、32-bit ARM の `vadd.i32` と `vmul.i32` である。AVX2 の確認では、`vpaddd` の結果が `ymm1` のまま `vpmulld` に入り、間の store/load が無かった。入力のロードと出力のストアは関数境界にだけ残る。

RISC-V と LoongArch では、intrinsics を使うと C がインラインされても一時値の store/load が残った。GCC の vector 拡張と通常のロード・ストアに置き換えると消えた。C は、可能なら vector 拡張で書く。

条件を満たさないビルドでは、C 関数への `call` が残る。native の既定は呼び出しベースであり、ゼロコストは文書化した条件を満たすビルドだけに適用する。C の feature 集合を Rust の部分集合にできない extension は、ゼロコストを主張しない。

### unstable feature

`#[target_feature]` を付けられない feature は、クレート側のラッパーをゼロコスト領域にできない。利用者がクレート全体を `-C target-feature=+name` でコンパイルし、C の feature 集合を合わせた場合だけ、LTO でインラインできる。cfg は立たないため、API の公開条件には使わない。

x86 の `avx10.1` で確認した。`-C target-feature=+avx10.1` は警告を出すが、LLVM の属性には入る。C を `-mavx10.1 -mno-mmx -mno-x87 -mno-popcnt -mno-xsave` でコンパイルし、`#[target_feature]` を付けないカーネルから呼ぶと、`vpaddd` と `vpmulld` が `zmm` のまま並び、`call` と命令間の store/load が消えた。

## ゼロコストの測定方法

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

## arch別の測定結果

`fixture` は測定に使ったビルド、`kernel` は disassembly した関数である。`manual` はこの文書の方法で作った一時バイナリで、リポジトリの fixture ではない。`(arch, fixture, kernel, extension, 確認した命令)` を測定のキーとする。どの行も Rust と C の LTO モードを一致させ、`lto = "thin"` で確認した。fat は「LTO」の節で arch ごとに確認する。

`manual` の行は、[ゼロコストの測定方法](#ゼロコストの測定方法) の `RUSTC_BOOTSTRAP=1 cargo build -Z build-std=core --target <target> --release` で作った `no_std` / `no_main` のバイナリを `llvm-objdump` で見る。再現手順は [再現の例](#再現の例) に従う。`make zero-cost-check` は `manual` の行を fixture / kernel の検査対象にしないが、各行に理由があることは検査する。

`manual` が残るのは、RISC-V の `vrgather` / `vcompress`、LoongArch の `xvshuf.b`、32-bit ARM、MIPS の `vshf.w` である。値テスト fixture にゼロコスト用カーネルがない、ビルド条件がゼロコスト条件と一致しない、または `llvm-mc` でバイト列をデコードする必要があるため、個別の link fixture にしていない。

| arch       | fixture                         | kernel                                                     | LTO  | extension                  | 確認した命令                      | `call` | 命令間の store/load | 結果                      |
| ---------- | ------------------------------- | ---------------------------------------------------------- | ---- | -------------------------- | --------------------------------- | ------ | ------------------- | ------------------------- |
| x86_64     | `fixtures/sse2-link`            | `kernel`                                                   | thin | `sse2`                     | `paddd`                           | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_sse3`                                              | thin | `sse3`                     | `addsubps`                        | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_ssse3`                                             | thin | `ssse3`                    | `pshufb`                          | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_sse41`                                             | thin | `sse4.1`                   | `pmulld`                          | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_sse42`                                             | thin | `sse4.2`                   | `pcmpgtq`                         | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_sse4a`                                             | thin | `sse4a`                    | `insertq`                         | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_avx`                                               | thin | `avx`                      | `vaddps`、`vmulps`                | なし   | なし                | ゼロコスト                |
| i686       | `fixtures/i686-values`          | `kernel_sse2`                                              | thin | `sse2`                     | `paddd`、`pmuludq`                | なし   | なし                | ゼロコスト                |
| i686       | `fixtures/i686-values`          | `kernel_avx2`                                              | thin | `avx2`                     | `vpaddd`、`vpmulld`               | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/avx2-link`            | `kernel`                                                   | thin | `avx2`                     | `vpaddd`、`vpmulld`               | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_pshufb`                                            | thin | `avx2`                     | `vpshufb`                         | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/avx512f-link`         | `kernel`                                                   | thin | `avx512f`                  | `vpaddd`                          | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/aes-link`             | `kernel_aes`                                               | thin | `aes`                      | `aesenc`                          | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/fma-link`             | `kernel_fma`                                               | thin | `fma`                      | `vfmadd`                          | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_avx10_1`                                           | thin | `avx10.1`                  | `vpaddd`、`vpmulld`               | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_sha`                                               | thin | `sha`                      | `sha256rnds2`                     | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_bw`                                                | thin | `avx512bw`                 | `vpaddb`                          | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_cd`                                                | thin | `avx512cd`                 | `vplzcntd`                        | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_vbmi`                                              | thin | `avx512vbmi`               | `vpermb`                          | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_vpopcnt`                                           | thin | `avx512vpopcntdq`          | `vpopcntd`                        | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_bitalg`                                            | thin | `avx512bitalg`             | `vpopcntb`                        | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_vbmi2`                                             | thin | `avx512vbmi2`              | `vpcompressb`                     | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_bf16`                                              | thin | `avx512bf16`               | `vdpbf16ps`                       | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_fp16`                                              | thin | `avx512fp16`               | `vaddph`                          | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_vp2intersect`                                      | thin | `avx512vp2intersect`       | `vp2intersectd`                   | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/avx512vl-link`        | `kernel_add`、`kernel_perm`                                | thin | `avx512vl` + `avx512vbmi`  | `vpaddb`、`vpermb`                | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_dq`                                                | thin | `avx512dq`                 | `vpmullq`                         | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_ifma`                                              | thin | `avx512ifma`               | `vpmadd52luq`                     | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_vnni`                                              | thin | `avx512vnni`               | `vpdpbusd`                        | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_gfni`                                              | thin | `gfni`                     | `gf2p8affineqb`                   | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_vaes`                                              | thin | `vaes`                     | `vaesenc`                         | なし   | なし                | ゼロコスト                |
| x86_64     | `fixtures/x86-zero-cost-link`   | `kernel_vpclmul`                                           | thin | `vpclmulqdq`               | `vpclmulqdq`                      | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_add`                                               | thin | `neon`                     | `add v0.4s`                       | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_mul`                                               | thin | `neon`                     | `mul v0.4s`                       | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_tbl`                                               | thin | `neon`                     | `tbl`                             | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_dot`                                               | thin | `dotprod`                  | `sdot`                            | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_aes`                                               | thin | `aes`                      | `aese`                            | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_sha2`                                              | thin | `sha2`                     | `sha256h`                         | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_crc`                                               | thin | `crc`                      | `crc32b`                          | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_fp16`                                              | thin | `fp16` + `fhm`             | `fadd v0.8h`                      | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_sha3`                                              | thin | `sha3`                     | `eor3`                            | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_sm4`                                               | thin | `sm4`                      | `sm4e`                            | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_i8mm`                                              | thin | `i8mm`                     | `smmla`                           | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_bf16`                                              | thin | `bf16`                     | `bfdot`                           | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_fhm`                                               | thin | `fp16` + `fhm`             | `fmla v0.8h`                      | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_jsconv`                                            | thin | `jsconv`                   | `fjcvtzs`                         | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_sve_add`                                           | thin | `sve`                      | `add z0.s`                        | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_sve_mul`                                           | thin | `sve`                      | `fmul z0.s`                       | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_sve_sub`                                           | thin | `sve`                      | `fsub z0.s`                       | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_f32mm`                                             | thin | `sve` + `f32mm`            | `fmmla`                           | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_f64mm`                                             | thin | `sve` + `f64mm`            | `fmmla`                           | なし   | なし                | ゼロコスト                |
| AArch64    | `fixtures/aarch64-values`       | `kernel_fcma`                                              | thin | `fcma`                     | `fcmla`                           | なし   | なし                | ゼロコスト                |
| RISC-V     | `fixtures/riscv-values`         | `kernel_add`                                               | thin | `v`                        | `vadd.vv`                         | なし   | なし                | ゼロコスト                |
| RISC-V     | `fixtures/riscv-values`         | `kernel_mul`                                               | thin | `v`                        | `vmul.vv`                         | なし   | なし                | ゼロコスト                |
| RISC-V     | manual                          | —                                                          | thin | `v`                        | `vrgather`                        | なし   | あり                | 往復が残る                |
| RISC-V     | manual                          | —                                                          | thin | `v`                        | `vcompress`                       | なし   | あり                | 往復が残る                |
| LoongArch  | `fixtures/loongarch-values`     | `kernel_add`                                               | thin | `lsx`                      | `vadd.w`                          | なし   | なし                | ゼロコスト                |
| LoongArch  | `fixtures/loongarch-values`     | `kernel_mul`                                               | thin | `lsx`                      | `vmul.w`                          | なし   | なし                | ゼロコスト                |
| LoongArch  | `fixtures/loongarch-values`     | `kernel_xvadd`                                             | thin | `lasx`                     | `xvadd.w`                         | なし   | なし                | ゼロコスト                |
| LoongArch  | `fixtures/loongarch-values`     | `kernel_xvmul`                                             | thin | `lasx`                     | `xvmul.w`                         | なし   | なし                | ゼロコスト                |
| LoongArch  | manual                          | —                                                          | thin | `lasx`                     | `xvshuf.b`                        | なし   | あり                | 往復が残る                |
| s390x      | `fixtures/s390x-values`         | `kernel_add`                                               | thin | `vector`                   | `vaf`                             | なし   | なし                | ゼロコスト                |
| s390x      | `fixtures/s390x-values`         | `kernel_mul`                                               | thin | `vector`                   | `vmlf`                            | なし   | なし                | ゼロコスト                |
| s390x      | `fixtures/s390x-values`         | `kernel_perm`                                              | thin | `vector`                   | `vperm`                           | なし   | なし                | ゼロコスト                |
| 32-bit ARM | manual                          | —                                                          | thin | `neon`                     | `vadd.i32`、`vmul.i32`            | なし   | なし                | ゼロコスト                |
| 32-bit ARM | manual                          | —                                                          | thin | `neon`                     | `vtbl1_s8`、`vtbl2_s8`            | —      | —                   | 8-byte table のみ         |
| PowerPC    | `fixtures/powerpc-values`       | `kernel_add`                                               | thin | `altivec`                  | `vadduwm`                         | なし   | なし                | ゼロコスト                |
| PowerPC    | `fixtures/powerpc-values`       | `kernel_mul`                                               | thin | `altivec`                  | `vmuluwm`                         | なし   | なし                | ゼロコスト                |
| PowerPC    | `fixtures/powerpc-values`       | `kernel_perm`                                              | thin | `altivec`                  | `vperm`                           | なし   | なし                | ゼロコスト                |
| MIPS       | `fixtures/mips-values`          | `kernel_add`                                               | thin | `msa`                      | `addv.w`                          | なし   | なし                | ゼロコスト                |
| MIPS       | `fixtures/mips-values`          | `kernel_mul`                                               | thin | `msa`                      | `mulv.w`                          | なし   | なし                | ゼロコスト                |
| MIPS       | manual                          | —                                                          | thin | `msa`                      | `vshf.w`                          | —      | —                   | スカラー C フォールバック |
| wasm32     | `fixtures/wasm-simd-link`       | `kernel_one`、`kernel_two`、`kernel_relaxed`               | thin | `simd128` + `relaxed-simd` | `f32x4.add`、`f32x4.relaxed_madd` | なし   | なし                | wat で確認                |
| wasm32     | `fixtures/wide-arithmetic-link` | `kernel_add`、`kernel_sub`、`kernel_mul_u`、`kernel_mul_s` | thin | `wide-arithmetic`          | `i64.add128` など                 | なし   | なし                | wat で確認                |

## IR レベルの確認

PowerPC は ELF のリンクが `.TOC.` で失敗したため、最初は Rust と C の bitcode を `llvm-link` し、`opt -passes="default<O3>"` で最適化して IR を見た。

- PowerPC: カーネルに `add <4 x i32>` と `mul <4 x i32>` が並び、`c_add` / `c_mul` への `call` と命令間の store/load が無かった。その後、`.TOC.` を参照する非 LTO オブジェクトをリンクに加えることで、バイナリでも `vadduwm` と `vmuluwm` を確認した。
- MIPS: feature 集合を合わせると、カーネルに `addv.w` と `mulv.w` が並び、`c_add` / `c_mul` への `call` と命令間の store/load が無かった。C は `-march=mips64r2 -mmsa -mabicalls -mxgot`、Rust は `-C target-feature=+msa,+fp64` を使う。

## 未確認

arch 単位では、ゼロコストを確認していない arch はない。extension や命令単位の未確認は、各 arch の README と上の結果表にある。

## fixture 化できない測定

### 32-bit ARM

文書の `-Xclang -target-feature` 調整を `build.rs` に入れると、LTO リンクで LLVM の ARM Instruction Selection が `Do not know how to split this operator's operand!` でクラッシュする。thin / fat の両方の LTO モードで、専用 link fixture でも同じクラッシュを再現した。LLVM の修正版を待つか、C を分割して feature 集合を合わせる必要がある。

## スケーラブルベクタ

SVE のスライス API を AArch64 で確認した。C のループを `#[target_feature(enable = "sve")]` の Rust カーネルへインラインすると、`cntw`、`whilelt`、`ld1w`、`add z0.s`、`st1w` がカーネルに並び、`call` が無かった。SVE / F32MM / F64MM の行はスライス API のロード・ストアを含むが、対象命令の間に追加の store/load はない。

RVV のスライス API も RISC-V で確認した。C のループを Rust のカーネルへインラインすると、`vsetvli`、`vle32.v`、`vadd.vv`、`vse32.v` がカーネルに並び、`call` が無かった。

どちらも命令ごとの API ではなく、スライスに対するループになる。固定長 API の「命令間の store/load」とは別に扱う。

## shuffle と compress

shuffle と compress は、intrinsics が必要な場合に SROA が一時値を消せず、往復が残ることがある。結果は上の表の `vpshufb`、`tbl`、`vperm`（PowerPC と s390x）、`vrgather`、`xvshuf.b`、`vshf.w`、`vcompress` の行を見る。`expand` など未確認の命令は、実装時に個別に確認する。

## Rust の属性

rustc 1.99.1 では、`#[inline(always)]` と `#[target_feature]` を同じ関数に付けられない（E0658）。

- ラッパーは `#[inline(always)]` だけにし、呼び出し側のカーネルに `#[target_feature]` を付ける。
- ラッパーに `#[target_feature]` を付ける場合は、`#[inline(always)]` を外す。`#[inline]` は併用できる。C の関数が LTO でラッパーへインラインされることを確認する。x86_64 では、この形でも `call` が消え、`vpaddd` と `vpmulld` が並ぶことを確認した。

## 128-bit の `asm!`

x86_64 では、`asm!` の `xmm_reg` に `u128` を渡せる。`paddd` と `pmulld` を並べると、間の store/load が消えた。

AArch64 では、`asm!` の `vreg` が `u128` を受け付けない。`vreg` が受け付ける `i64x2` や `f128` は unstable である。このため、AArch64 の NEON は C + clang を使う。

ARMv7 では、`asm!` の `qreg` が `u128` を受け付けず、`neon` を要求する。`qreg` が受け付ける `i32x4` などは unstable である。ARMv7 の NEON も C + clang を使う。

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

- RISC-V: `__riscv_vle32_v_i32m1` / `__riscv_vse32_v_i32m1`、`__riscv_vrgather_vv_i32m1`
- LoongArch: `__lasx_xvld` / `__lasx_xvst`

GCC の vector 拡張に置き換えると、どちらも store/load が消えた。一方、`vrgather` は intrinsics が必要で、2 命令を合成すると `vrgather` の間に store/load が残った。LoongArch の `xvshuf.b` も同じだった。intrinsics が必要な命令は、命令ごとに disassembly を確認する。

### arch別のフラグ

archごとのCフラグとfeature集合は、各archの `README.md` の「ゼロコスト」に置く。結果は上の表を見る。

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

| 条件                             | `CARGO_CFG_TARGET_FEATURE`                        |
| -------------------------------- | ------------------------------------------------- |
| x86_64 の既定                    | `fxsr,sse,sse2`                                   |
| x86_64 `-C target-feature=+avx2` | `avx,avx2,fxsr,sse,sse2,sse3,sse4.1,sse4.2,ssse3` |
| AArch64 の既定                   | `neon,outline-atomics`                            |
| AArch64 `-C target-feature=+sve` | `fp16,neon,outline-atomics,sve`                   |

## 再現の例

x86_64 の例は [x86 のリンク](x86/linking-and-zero-cost.md) にある。ほかの arch は、`no_std` と `no_main` のバイナリを作り、`build-std` と clang/lld でリンクする。

```toml
[profile.release]
lto = "thin"
panic = "abort"
```

```toml
[build]
rustflags = [
  "-C",
  "linker-plugin-lto",
  "-C",
  "linker=clang",
  "-C",
  "link-arg=--target=<clang-target>",
  "-C",
  "link-arg=-nostdlib",
  "-C",
  "link-arg=-fuse-ld=lld",
]
```
