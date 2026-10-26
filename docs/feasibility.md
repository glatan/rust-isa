# 実装可否

この文書は、`cfg(target_feature)` の観測用語と、arch ごとの表への索引である。観測表は `docs/<arch>/feasibility.md` に置く。実装方式は [実装方式](implementation.md) に、可用性の決定は [実装方針](policy.md) に置く。

ここでいう feature は、rustc の `-C target-feature` に渡す名前である。このクレートの Cargo feature ではない。

## cfg の判定

| 判定      | 意味                                        |
| --------- | ------------------------------------------- |
| 既定      | フラグ無しで `cfg(target_feature)` が立つ   |
| 安定      | `+name` で警告なく cfg が立つ               |
| 不安定    | `+name` は警告を出し、cfg は立たない        |
| LLVM のみ | rustc の feature 名ではない。cfg に書けない |

ゼロコストはこの表に含めない。確認方法と結果は [ゼロコストの確認](zero-cost.md) に置く。

警告が無く cfg も立たない指定は、上の四語に入れない。「属性には入るが cfg には出ない指定」として arch ごとの表に書く。

## cfg と実装可否は別

- cfg が立つ feature は、`cfg` の中に関数を置く方式（cfg ゲート）を使える。ただし native の公開条件は [実装方針](policy.md) に従い、cfg ゲートにしない。
- cfg が立たない feature は、cfg ゲートを使えない。しかし、関数を無条件に置き、C の `__attribute__((target(...)))` や `asm!` で実装できる場合がある。
- cfg の中に書いた関数が含まれないのは、その cfg が立たないときである。関数を cfg の外に置けば、そのコンパイルにも含まれる。
- したがって、この文書は「実装できるか」を判定しない。判定するのは cfg が立つかだけである。
- ゼロコストで実装できるかは、さらに別である。[ゼロコストの確認](zero-cost.md) を参照する。

wasm32 では、モジュールが含む命令の feature を全体として要求する。このため、cfg ゲートを外して関数を無条件に置くと、feature が無効なビルドにも feature を要求する。wasm32 の cfg ゲートは、この性質のためのものである。native には同じ制約はない。

## 観測環境

- rustc 1.99.1（58cf8f9c3、2026-10-13）
- clang 23.1.1。ARM と PowerPC の一部は clang 23.1.3 で再確認した。

## arch ごとの表

| arch       | 対象                                                 | 既定の SIMD   | 安定の例                          | 表                                    |
| ---------- | ---------------------------------------------------- | ------------- | --------------------------------- | ------------------------------------- |
| x86        | `x86_64-unknown-linux-gnu`、`i686-unknown-linux-gnu` | `sse`、`sse2` | `avx2`、`avx512f`、`aes`、`bmi2`  | [x86](x86/feasibility.md)             |
| AArch64    | `aarch64-unknown-linux-gnu`                          | `neon`        | `sve`、`sve2`、`dotprod`、`i8mm`  | [AArch64](aarch64/feasibility.md)     |
| 32-bit ARM | `armv7-unknown-linux-gnueabihf`                      | なし          | 今回測った範囲ではなし            | [32-bit ARM](arm/feasibility.md)      |
| PowerPC    | `powerpc64le-unknown-linux-gnu`                      | なし          | 今回測った範囲ではなし            | [PowerPC](powerpc/feasibility.md)     |
| RISC-V     | `riscv64gc-unknown-linux-gnu`                        | なし          | `zba`、`zbb`、`zbc`、`zbs`、`zk*` | [RISC-V](riscv/feasibility.md)        |
| LoongArch  | `loongarch64-unknown-linux-gnu`                      | `lsx`         | `lasx`、`lbt`                     | [LoongArch](loongarch/feasibility.md) |
| s390x      | `s390x-unknown-linux-gnu`                            | なし          | `vector`、`vector-enhancements-*` | [s390x](s390x/feasibility.md)         |
| MIPS       | `mips64-unknown-linux-gnuabi64`                      | なし          | 今回測った範囲ではなし            | [MIPS](mips/feasibility.md)           |
| wasm32     | `wasm32-unknown-unknown`                             | なし          | `simd128`、`relaxed-simd`         | [wasm32](wasm32/feasibility.md)       |

実装状況は各 arch の `README.md` を正とする。ゼロコストの方法と結果は [ゼロコストの確認](zero-cost.md) に置く。この文書には状態表を置かない。

## 表の限界

- 表は、調べた feature だけを含む。rustc がそのターゲットに持つ feature の全体ではない。
- cfg は、そのクレートをコンパイルするときのフラグを見る。呼び出し側の `#[target_feature]` や `cargo rustc -- -C target-feature` は、依存クレートの cfg を変えない。
- cfg が立つかは、rustc のバージョンで変わる。観測したバージョンを各 arch の「確認したこと」に書く。
