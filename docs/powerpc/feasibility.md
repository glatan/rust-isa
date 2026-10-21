# PowerPC の実装可否

対象は `powerpc64le-unknown-linux-gnu` である。CPU 名を変えたときの属性は、「属性には入るが cfg には出ない指定」にある。

不安定と LLVM のみの feature は cfg が立たない。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [PowerPC](README.md) に書く。

| feature            | 種別   | 判定      | 実装   |
| ------------------ | ------ | --------- | ------ |
| `altivec`          | SIMD   | 不安定    | 一部   |
| `vsx`              | SIMD   | 不安定    | 一部   |
| `power8-altivec`   | SIMD   | 不安定    | 一部   |
| `power8-vector`    | SIMD   | 不安定    | 一部   |
| `power9-altivec`   | SIMD   | 不安定    | 一部   |
| `power9-vector`    | SIMD   | 不安定    | 一部   |
| `power10-vector`   | SIMD   | 不安定    | 一部   |
| `power8-crypto`    | 暗号   | 不安定    | 一部   |
| `quadword-atomics` | その他 | 不安定    | 対象外 |
| `partword-atomics` | その他 | 不安定    | 対象外 |
| `mma`              | 行列   | LLVM のみ | 未     |
| `htm`              | その他 | LLVM のみ | 対象外 |

## 属性には入るが cfg には出ない指定

| 指定                                      | rustc で確認したこと                                                                         | 確認していないこと                                                                                                  | `cfg`  |
| ----------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | ------ |
| `powerpc64le` の既定 `target-cpu=ppc64le` | IR は `target-cpu=ppc64le` だけで、feature 列は無い                                          | clang 23.1.3 の `ppc64le` の定義には `altivec`、`vsx`、`power8-vector` がある。rustc がこの定義のまま命令を選ぶこと | 出ない |
| `-C target-cpu=pwr10`                     | IR は `target-cpu=pwr10` だけで、feature 列は無い。警告は出ない                              | clang 23.1.3 の `pwr10` の定義には `power10-vector` と `mma` がある。rustc がこの定義のまま命令を選ぶこと           | 出ない |
| `-C target-feature=+power10-vector`       | 警告が出る。属性は `+altivec`、`+vsx`、`+power8-vector`、`+power9-vector`、`+power10-vector` | —                                                                                                                   | 出ない |

## 確認したこと

- `-C target-feature=+altivec` は警告を出し、`altivec` の cfg は立たなかった。
- `-maltivec -mvsx` の C は `vadduwm` を出した。
- `__attribute__((target("altivec")))` の関数も `vadduwm` を出した。
