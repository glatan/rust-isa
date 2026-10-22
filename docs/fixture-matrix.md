# 値テストの fixture

この文書は、`fixtures/` と `tests/` の対応をまとめる。値テストの設計は [native の値テスト](native-testing.md) に書く。

## native

| fixture                     | triple                          | runner                             | 主な対象                                                                       | 実行できないもの             |
| --------------------------- | ------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------ | ---------------------------- |
| `fixtures/i686-values`      | `i686-unknown-linux-gnu`        | `qemu-i386-static`                 | SSE2、AVX2                                                                     | CPU 非対応の命令             |
| `fixtures/aarch64-values`   | `aarch64-unknown-linux-gnu`     | `qemu-aarch64-static -cpu max`     | NEON、dotprod、fp16、i8mm、fcma、fhm、AES、SHA-2、SHA-3、SM4、CRC、SVE、JSCONV | RDM、BF16、SME の一部        |
| `fixtures/arm-values`       | `armv7-unknown-linux-gnueabihf` | `qemu-arm-static -cpu max`         | NEON、dotprod、fp16、i8mm、AES、SHA-2、CRC                                     | MVE                          |
| `fixtures/riscv-values`     | `riscv64gc-unknown-linux-gnu`   | `qemu-riscv64-static -cpu max`     | zba、zbb、zbc、zbs、zkn、zks、V、スライス API                                  | 将来の `zfh` / `zfa` / `zv*` |
| `fixtures/loongarch-values` | `loongarch64-unknown-linux-gnu` | `qemu-loongarch64-static -cpu max` | LSX、LASX                                                                      | LBT                          |
| `fixtures/s390x-values`     | `s390x-unknown-linux-gnu`       | `qemu-s390x-static`                | vector                                                                         | enhancements の一部          |
| `fixtures/powerpc-values`   | `powerpc64le-unknown-linux-gnu` | `qemu-ppc64le-static -cpu power9`  | Altivec、VSX、Power8 vector、Power8 crypto                                     | Power10、MMA                 |
| `fixtures/mips-values`      | `mips64-unknown-linux-gnuabi64` | `qemu-mips64-static`               | MSA                                                                            | virt                         |

QEMU が命令を実行しない場合は、ビルドと API の存在確認だけを行う。その場合は `docs/<arch>/README.md` の値テスト列に理由を書く。

## wasm32

| fixture                 | target                   | runner     | 内容                                             |
| ----------------------- | ------------------------ | ---------- | ------------------------------------------------ |
| `fixtures/simd128-link` | `wasm32-unknown-unknown` | `wasm2wat` | `simd128` と `relaxed-simd` のリンクと zero-cost |
| `fixtures/wide-link`    | `wasm32-unknown-unknown` | `wasm2wat` | `wide-arithmetic` のリンク                       |

## x86_64 のリンク検査

| fixture                | 内容                                       |
| ---------------------- | ------------------------------------------ |
| `fixtures/sse2-link`   | `paddd` が隣接し、C への `call` が無いこと |
| `fixtures/avx2-link`   | `vpaddd` と `vpmulld` が隣接すること       |
| `fixtures/avx512-link` | `vpaddd` が隣接すること                    |
| `fixtures/crypto-link` | `aesenc` と `vfmadd` が隣接すること        |

## 追加するとき

1. `fixtures/<arch>-values/src/main.rs` に値テストを追加する。
2. `tests/<arch>.rs` のビルドと QEMU 実行に反映する。
3. 値テスト列と、この表を更新する。
4. 実行できない命令は、コンパイル確認だけにした理由を書く。
