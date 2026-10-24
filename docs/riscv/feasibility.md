# RISC-V の実装可否

次の表は `riscv64gc-unknown-linux-gnu` で測った。`v` の不安定は、このターゲットで `+v` が警告を出して cfg が立たない、という意味である。`riscv64a23` の既定は警告が無く、`v` の cfg も立たないので、この不安定には入らない。

`f` と `d` はこの表に行が無い。`riscv64a23` では feature 列に入り、cfg には出ない。`b` はそこでは cfg に出る。属性の文字列は「属性には入るが cfg には出ない指定」にある。

安定の feature は cfg が立つ。不安定の feature は立たない。native では公開条件に cfg を使わない（[実装方針](../policy.md)）。実装方式は [RISC-V](README.md) に書く。

> 実装状況は [README](README.md) を参照する。この表は cfg の観測だけを扱う。

| feature                | 種別   | 判定   |
| ---------------------- | ------ | ------ |
| `zba`                  | 整数   | 安定   |
| `zbb`                  | 整数   | 安定   |
| `zbc`                  | 整数   | 安定   |
| `zbs`                  | 整数   | 安定   |
| `zk`                   | 暗号   | 安定   |
| `zkn`                  | 暗号   | 安定   |
| `zknd`                 | 暗号   | 安定   |
| `zkne`                 | 暗号   | 安定   |
| `zknh`                 | 暗号   | 安定   |
| `zkr`                  | 暗号   | 安定   |
| `zks`                  | 暗号   | 安定   |
| `zksed`                | 暗号   | 安定   |
| `zksh`                 | 暗号   | 安定   |
| `zkt`                  | 暗号   | 安定   |
| `zbkb`                 | 暗号   | 安定   |
| `zbkc`                 | 暗号   | 安定   |
| `zbkx`                 | 暗号   | 安定   |
| `v`                    | SIMD   | 不安定 |
| `zve32x`               | SIMD   | 不安定 |
| `zve32f`               | SIMD   | 不安定 |
| `zve64x`               | SIMD   | 不安定 |
| `zve64f`               | SIMD   | 不安定 |
| `zve64d`               | SIMD   | 不安定 |
| `zvfh`                 | SIMD   | 不安定 |
| `zvfhmin`              | SIMD   | 不安定 |
| `zvfbfmin`             | SIMD   | 不安定 |
| `zvfbfwma`             | SIMD   | 不安定 |
| `zvl32b`               | SIMD   | 不安定 |
| `zvl64b`               | SIMD   | 不安定 |
| `zvl128b`              | SIMD   | 不安定 |
| `zvl256b`              | SIMD   | 不安定 |
| `zvl512b`              | SIMD   | 不安定 |
| `zvl1024b`             | SIMD   | 不安定 |
| `zfh`                  | その他 | 不安定 |
| `zfhmin`               | その他 | 不安定 |
| `zfa`                  | その他 | 不安定 |
| `unaligned-vector-mem` | SIMD   | 不安定 |
| `zvkn`                 | 暗号   | 不安定 |
| `zvknc`                | 暗号   | 不安定 |
| `zvkned`               | 暗号   | 不安定 |
| `zvkng`                | 暗号   | 不安定 |
| `zvknha`               | 暗号   | 不安定 |
| `zvknhb`               | 暗号   | 不安定 |
| `zvks`                 | 暗号   | 不安定 |
| `zvksc`                | 暗号   | 不安定 |
| `zvksed`               | 暗号   | 不安定 |
| `zvksg`                | 暗号   | 不安定 |
| `zvksh`                | 暗号   | 不安定 |
| `zvkb`                 | 暗号   | 不安定 |
| `zvkg`                 | 暗号   | 不安定 |
| `zvkt`                 | 暗号   | 不安定 |
| `zvbc`                 | 暗号   | 不安定 |

## 属性には入るが cfg には出ない指定

| 指定                                                              | rustc で確認したこと                                                    | 確認していないこと | `cfg`                                           |
| ----------------------------------------------------------------- | ----------------------------------------------------------------------- | ------------------ | ----------------------------------------------- |
| `riscv64gc` で `-C target-cpu=sifive-x280` または `andes-ax45mpv` | feature 列が CPU 定義より優先され、`+v` は付かない                      | —                  | 出ない                                          |
| `riscv64gc` で `-C target-feature=+v`                             | 警告が出る。属性に `+v` と `zve64d` などが付く                          | —                  | 出ない                                          |
| `riscv64a23-unknown-linux-gnu` の既定                             | 警告なしで feature 列に `+v`、`+f`、`+d` がある。四つの判定には入らない | —                  | `v`、`f`、`d` は出ない。`b` や `zba` などは出る |

## 確認したこと

- `-C target-feature=+v` は警告を出し、`v` の cfg は立たなかった。
- `-march=rv64gcv` の C は `vsetvli` と `vadd.vv` を出した。
- `__attribute__((target("arch=+v")))` の C もコンパイルできた。
