# 値テストの fixture

この文書は、`fixtures/` と `tests/` の対応をまとめる。値テストの設計は [native の値テスト](native-testing.md)、ゼロコストの測定キーは [ゼロコストの確認](zero-cost.md) に書く。`make fixtures-check` は、この表と `fixtures/` の実ディレクトリ、および `target-nested` とテストの `nested_dir(...)` の対応を検査する。

## native

| fixture                     | triple                          | runner                             | 主な対象                                                                        | 実行できないもの             |
| --------------------------- | ------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------- | ---------------------------- |
| `fixtures/i686-values`      | `i686-unknown-linux-gnu`        | `qemu-i386-static`                 | SSE2、AVX2（`kernel_sse2` / `kernel_avx2` の zero-cost を含む）                 | CPU 非対応の命令             |
| `fixtures/aarch64-values`   | `aarch64-unknown-linux-gnu`     | `qemu-aarch64-static -cpu max`     | NEON、dotprod、fp16、i8mm、fcma、fhm、AES、SHA-2、SHA-3、SM4、CRC、SVE、JSCONV  | RDM、BF16、SME の一部        |
| `fixtures/arm-values`       | `armv7-unknown-linux-gnueabihf` | `qemu-arm-static -cpu max`         | NEON、dotprod、fp16、i8mm、AES、SHA-2、CRC                                      | MVE                          |
| `fixtures/riscv-values`     | `riscv64gc-unknown-linux-gnu`   | `qemu-riscv64-static -cpu max`     | zba、zbb、zbc、zbs、zkn、zks、V、スライス API                                   | 将来の `zfh` / `zfa` / `zv*` |
| `fixtures/loongarch-values` | `loongarch64-unknown-linux-gnu` | `qemu-loongarch64-static -cpu max` | LSX、LASX（`kernel_xvadd` / `kernel_xvmul` の zero-cost を含む）                | LBT                          |
| `fixtures/s390x-values`     | `s390x-unknown-linux-gnu`       | `qemu-s390x-static`                | vector                                                                          | enhancements の一部          |
| `fixtures/powerpc-values`   | `powerpc64le-unknown-linux-gnu` | `qemu-ppc64le-static -cpu power9`  | Altivec、VSX、Power8 vector、Power8 crypto（`kernel_perm` の zero-cost を含む） | Power10、MMA                 |
| `fixtures/mips-values`      | `mips64-unknown-linux-gnuabi64` | `qemu-mips64-static`               | MSA（`kernel_add` / `kernel_mul` の zero-cost を含む）                          | virt                         |

QEMU が命令を実行しない場合は、ビルドと API の存在確認だけを行う。その場合は `docs/<arch>/README.md` の値テスト列に理由を書く。

## wasm32

| fixture                         | target                   | runner     | 内容                                             |
| ------------------------------- | ------------------------ | ---------- | ------------------------------------------------ |
| `fixtures/wasm-simd-link`       | `wasm32-unknown-unknown` | `wasm2wat` | `simd128` と `relaxed-simd` のリンクと zero-cost |
| `fixtures/wide-arithmetic-link` | `wasm32-unknown-unknown` | `wasm2wat` | `wide-arithmetic` のリンク                       |

## x86_64 のリンク検査

| fixture                       | 内容                                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fixtures/sse2-link`          | `paddd` が隣接し、C への `call` が無いこと                                                                                                              |
| `fixtures/avx2-link`          | `vpaddd` と `vpmulld` が隣接すること                                                                                                                    |
| `fixtures/avx512f-link`       | `vpaddd` が隣接すること                                                                                                                                 |
| `fixtures/avx512vl-link`      | `vpaddb` と `vpermb` が隣接すること                                                                                                                     |
| `fixtures/aes-link`           | `aesenc` が隣接すること                                                                                                                                 |
| `fixtures/fma-link`           | `vfmadd` が隣接すること                                                                                                                                 |
| `fixtures/x86-zero-cost-link` | `vpshufb`、`sha256rnds2`、`vpaddb`、`vpmullq`、`vpmadd52luq`、`vpdpbusd`、`gf2p8affineqb`、`vaesenc`、`vpclmulqdq`、`vpaddd` / `vpmulld` が隣接すること |

## `target-nested` のビルド

`target-nested` は、外側の `cargo test` と内側の `cargo` が同じ `target/` を奪い合わないようにするための成果物ディレクトリである。設定は `tests/common/mod.rs` にある。

| ディレクトリ                         | fixture                                | 確認する内容                                                                                                                                          |
| ------------------------------------ | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `target-nested/link`                 | `fixtures/wasm-simd-link`              | wasm32-unknown-unknown のリンクと zero-cost                                                                                                           |
| `target-nested/no-simd`              | `fixtures/wasm-simd-link`              | `simd128` / `relaxed-simd` なしのビルドが失敗すること                                                                                                 |
| `target-nested/sse2-link`            | `fixtures/sse2-link`                   | `kernel` に `call` が無く、`paddd` が2つ隣接する                                                                                                      |
| `target-nested/avx2-link`            | `fixtures/avx2-link`                   | `kernel` に `call` が無く、`vpaddd` の直後に `vpmulld` が残る                                                                                         |
| `target-nested/avx512f-link`         | `fixtures/avx512f-link`                | `kernel` に `call` が無く、`vpaddd` が2つ隣接する                                                                                                     |
| `target-nested/avx512vl-link`        | `fixtures/avx512vl-link`               | `kernel_add` / `kernel_perm` に `call` が無く、`vpaddb` / `vpermb` が隣接する                                                                         |
| `target-nested/x86-zero-cost-link`   | `fixtures/x86-zero-cost-link`          | `sha`、`avx512bw`、`avx512dq`、`avx512ifma`、`avx512vnni`、`gfni`、`vaes`、`vpclmulqdq`、`avx10.1` の各 kernel に `call` と命令間の store/load が無い |
| `target-nested/aes-link`             | `fixtures/aes-link`                    | `kernel_aes` に `call` が無く、`aesenc` が隣接する                                                                                                    |
| `target-nested/fma-link`             | `fixtures/fma-link`                    | `kernel_fma` に `call` が無く、`vfmadd` が隣接する                                                                                                    |
| `target-nested/i686-values`          | `fixtures/i686-values`                 | QEMU-i386 で SSE2 / AVX2 の値テストと `kernel_sse2` / `kernel_avx2` の disassembly                                                                    |
| `target-nested/aarch64-values`       | `fixtures/aarch64-values`              | QEMU-AArch64 で NEON / dotprod の値テストと `add` / `tbl` / `sdot` の disassembly                                                                     |
| `target-nested/riscv-values`         | `fixtures/riscv-values`                | RISC-V の固定長 V128 値テストと `vadd.vv` / `vmul.vv` の disassembly                                                                                  |
| `target-nested/loongarch-values`     | `fixtures/loongarch-values`            | LoongArch LSX / LASX の値テストと `vadd.w` / `vmul.w` / `xvadd.w` / `xvmul.w` の disassembly                                                          |
| `target-nested/s390x-values`         | `fixtures/s390x-values`                | s390x vector の値テスト                                                                                                                               |
| `target-nested/arm-values`           | `fixtures/arm-values`                  | ARMv7 NEON の値テスト                                                                                                                                 |
| `target-nested/arm-mve-values`       | `fixtures/arm-values`（`mve` feature） | QEMU `cortex-m55` で MVE / MVE-FP の値テスト                                                                                                          |
| `target-nested/powerpc-values`       | `fixtures/powerpc-values`              | PowerPC Altivec の値テストと `vadduwm` / `vmuluwm` / `vperm` の disassembly                                                                           |
| `target-nested/mips-values`          | `fixtures/mips-values`                 | MIPS MSA の値テストと `kernel_add` / `kernel_mul` の `llvm-mc` デコード                                                                               |
| `target-nested/wide-arithmetic-link` | `fixtures/wide-arithmetic-link`        | wasm32 の `wide-arithmetic` リンク検査                                                                                                                |

## 追加するとき

1. `fixtures/<arch>-values/src/main.rs` に値テストを追加する。
2. `tests/<arch>.rs` のビルドと QEMU 実行に反映する。
3. 値テスト列と、この表を更新する。
4. 実行できない命令は、コンパイル確認だけにした理由を書く。
