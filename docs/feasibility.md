# 実装可否

この文書は、`cfg(target_feature)` の観測用語と、arch ごとの表への索引である。観測表は `docs/<arch>/feasibility.md` に置く。実装方式と可用性の決定は [実装方針](policy.md) に置く。

ここでいう feature は、rustc の `-C target-feature` に渡す名前である。このクレートの Cargo feature ではない。

## cfg の判定

| 判定 | 意味 |
| --- | --- |
| 既定 | フラグ無しで `cfg(target_feature)` が立つ |
| 安定 | `+name` で警告なく cfg が立つ |
| 不安定 | `+name` は警告を出し、cfg は立たない |
| LLVM のみ | rustc の feature 名ではない。cfg に書けない |

「ゼロコスト」の列は、確認した条件で命令間の store/load が消えるかを分ける。確認していない arch は未確認とする。条件は [実装方針](policy.md) に書く。

警告が無く cfg も立たない指定は、上の四語に入れない。「属性には入るが cfg には出ない指定」として arch ごとの表に書く。

## cfg と実装可否は別

- cfg が立つ feature は、`cfg` の中に関数を置く方式（cfg ゲート）を使える。ただし native の公開条件は [実装方針](policy.md) に従い、cfg ゲートにしない。
- cfg が立たない feature は、cfg ゲートを使えない。しかし、関数を無条件に置き、C の `__attribute__((target(...)))` や `asm!` で実装できる場合がある。
- cfg の中に書いた関数が含まれないのは、その cfg が立たないときである。関数を cfg の外に置けば、そのコンパイルにも含まれる。
- したがって、この文書は「実装できるか」を判定しない。判定するのは cfg が立つかだけである。
- ゼロコストで実装できるかは、さらに別である。[実装方針](policy.md) の「ゼロコスト」を参照する。

wasm32 では、モジュールが含む命令の feature を全体として要求する。このため、cfg ゲートを外して関数を無条件に置くと、feature が無効なビルドにも feature を要求する。wasm32 の cfg ゲートは、この性質のためのものである。native には同じ制約はない。

## 観測環境

- rustc 1.99.1（58cf8f9c3、2026-10-13）
- clang 23.1.1
- 表の「実装」は、このリポジトリにその関数があるかを分ける。済みはあり、未は無い。未は、実装できない、という意味ではない。

## arch ごとの表

| arch | 対象 | 既定の SIMD | 安定の例 | 実装 | ゼロコスト | 表 |
| --- | --- | --- | --- | --- | --- | --- |
| x86 | `x86_64-unknown-linux-gnu`、`i686-unknown-linux-gnu` | `sse`、`sse2` | `avx2`、`avx512f`、`aes`、`bmi2` | `sse2`、`avx2`、`avx512f`、暗号・整数の一部 | x86_64、i686 で条件付き | [x86](x86/feasibility.md) |
| AArch64 | `aarch64-unknown-linux-gnu` | `neon` | `sve`、`sve2`、`dotprod`、`i8mm` | なし | `neon` で条件付き | [AArch64](aarch64/feasibility.md) |
| 32-bit ARM | `armv7-unknown-linux-gnueabihf` | なし | 今回測った範囲ではなし | なし | `neon` で条件付き | [32-bit ARM](arm/feasibility.md) |
| PowerPC | `powerpc64le-unknown-linux-gnu` | なし | 今回測った範囲ではなし | なし | 条件付き | [PowerPC](powerpc/feasibility.md) |
| RISC-V | `riscv64gc-unknown-linux-gnu` | なし | `zba`、`zbb`、`zbc`、`zbs`、`zk*` | なし | `v` で条件付き | [RISC-V](riscv/feasibility.md) |
| LoongArch | `loongarch64-unknown-linux-gnu` | `lsx` | `lasx`、`lbt` | なし | `lasx` で条件付き | [LoongArch](loongarch/feasibility.md) |
| s390x | `s390x-unknown-linux-gnu` | なし | `vector`、`vector-enhancements-*` | なし | `vector` で条件付き | [s390x](s390x/feasibility.md) |
| MIPS | `mips64-unknown-linux-gnuabi64` | なし | 今回測った範囲ではなし | なし | 条件付き | [MIPS](mips/feasibility.md) |
| wasm32 | `wasm32-unknown-unknown` | なし | `simd128`、`relaxed-simd` | `simd128`、`relaxed-simd` | 確認済み | [wasm32](wasm32/feasibility.md) |

## ゼロコストの状態

wasm32 の `simd128` と `relaxed-simd` は、往復が消えることを確認してある。native は x86_64、AArch64、RISC-V、LoongArch、s390x、32-bit ARM、PowerPC、MIPS で条件付きのゼロコストを確認した。LTO モードを Rust と C で一致させ、C の target-features を Rust の部分集合にすると、C の関数がインラインされる。128-bit は x86_64 で `asm!` と `u128` でも確認した。詳しくは [ゼロコストの確認](zero-cost.md) を参照する。

## 表の限界

- 表は、調べた feature だけを含む。rustc がそのターゲットに持つ feature の全体ではない。
- cfg は、そのクレートをコンパイルするときのフラグを見る。呼び出し側の `#[target_feature]` や `cargo rustc -- -C target-feature` は、依存クレートの cfg を変えない。
- cfg が立つかは、rustc のバージョンで変わる。観測したバージョンを各 arch の表に書く。
