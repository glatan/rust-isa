# doctest の網羅状況

この文書は、命令単位の doctest 網羅を「どの範囲で、何を保証しているか」とともに記録する。テストの役割は [テストの役割](testing.md)、実行方法は [開発環境](development.md) にある。

## 網羅の定義

公開命令のパスを、その extension の rustdoc にある instruction inventory の doctest に含める。free function は `use ... as _;`、associated function は `let _ = Type::fn;`、型は `use ...::Type;` で列挙する。rustdoc が全パスをコンパイルする。命令の動作と期待値は値テストが担う。

在庫は `tools/doctest_inventory.py` で生成する。arch の README に載る命令名との対応も同じツールで検査する。

## 集計

`python3 tools/doctest_inventory.py --summary` の結果である。`--summary` は集計前に在庫の鮮度とこの表の一致を検査する。

| arch      | extension | 命令関数 | 型 |
| --------- | --------: | -------: | -: |
| x86       |        54 |    1,589 |  4 |
| aarch64   |        18 |    1,214 |  1 |
| arm       |         9 |      229 |  0 |
| riscv     |        13 |      418 |  0 |
| loongarch |         3 |      300 |  0 |
| s390x     |         6 |      204 |  0 |
| powerpc   |         8 |      227 |  1 |
| mips      |         1 |       82 |  0 |
| wasm32    |         3 |      258 |  1 |
| 合計      |       115 |    4,521 |  7 |

「命令関数」は free function と associated function を合わせた数である。「型」は `V128`、`SmeGuard` などの import で、命令数には含めない。x86 には x86_64 専用命令の cfg 付き import を含む。wasm32 の 3 extension は `simd128`、`relaxed-simd`、`wide-arithmetic` である。

## 検査

```sh
make doctest-coverage
make doctest-inventory-check
```

- `make doctest-coverage`: 在庫の鮮度と上の表を検査し、集計を表示する。
- `make doctest-inventory-check`: inventory の更新漏れと、arch の README に載る命令名の欠落を検査する。`make coverage-check` にも含まれる。

inventory を実際に rustdoc でコンパイルするのは、次のテストである。`doctest-coverage` と `doctest-inventory-check` はコンパイルを行わない。

- `make test-doc`: native target。Arch では `--no-run`、Debian では `DOC_TEST=run`。
- `make test-x86_64`: x86_64。x86_64 専用命令の cfg を含む。
- `make test-wasip1`: wasm32 の `simd128` / `relaxed-simd`。
- `make test-wasip1-wide-arithmetic`: `wide-arithmetic` を有効にした wasm32。

## 範囲外

inventory は公開パスがコンパイルできることを保証する。命令の実行結果、NaN、飽和、エンディアン、ゼロコストは値テストと disassembly が担当する。利用例は、公開パスや値の扱いが自明でない命令に代表例を置く。
