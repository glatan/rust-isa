# テストの役割

この文書は、リポジトリのテストを「何を保証するか」で分類し、現在のテスト資産との対応をまとめる。fixture と runner は [値テストの fixture](fixture-matrix.md)、ゼロコストの条件は [ゼロコストの確認](zero-cost.md)、実行コマンドは [開発環境](development.md) に置く。

## 分類

### 値テスト

命令の結果を独立した期待値と比較する。レーンの順序、エンディアン、NaN、飽和、メモリの境界など、命令の意味論を保証する。コード生成と API の在庫は保証しない。

- x86_64: `tests/x86`。SDE で実行する。
- その他の native: `fixtures/<arch>-values` を `tests/<arch>.rs` からビルドし、QEMU または SDE で実行する。
- wasm32: `tests/wasm32`。wasmtime で実行する。

### 公開契約

公開パス、関数の形、cfg と Cargo feature の可用性、即値のコンパイル拒否を保証する。

- `make check-docs`: 全ターゲットで rustdoc を `-D warnings` 付きでビルドする。
- `tests/x86/simd/api_surface.rs` と `tests/x86/matrix/*`: 代表的な API の関数型を検査する。
- `tests/wasm_link.rs`: `simd128` を無効にしたビルドが失敗することを検査する。
- `tests/wasm32/wide_arithmetic.rs`: `wide-arithmetic` feature が有効なときだけビルドされる。
- wasm32 の 2 つの `compile_fail,E0080` doctest: レーン即値の範囲外をコンパイルエラーにする。

値テストは有効な即値だけを呼ぶ。native arch の即値の範囲外は、コンパイル拒否を自動で検査していない。

### コード生成とゼロコスト

LTO で C の関数が Rust のカーネルへインラインされ、`call` と命令間の store/load が残らないことを保証する。

- `tests/x86_link.rs`: x86_64 の link fixture をビルドし、disassembly を検査する。
- `tests/wasm_link.rs`: wasm32 の WAT を検査する。
- `tests/wide_arithmetic_link.rs`: wasm32 の `wide-arithmetic` の opcode を検査する。
- `tests/<arch>.rs`: 値テストの fixture をビルドしたあと、同じバイナリの `kernel_*` を disassembly する。
- `docs/zero-cost.md` の結果表: 測定の記録である。`make zero-cost-check` は fixture と kernel の実在、表の値の整合を検査する。測定結果そのものを再実行しない。`manual` の行は再現手順だけを残す。

値テストとコード生成は、同じ fixture とビルドを使う。`tests/<arch>.rs` では 1 つのテスト関数が両方を順に実行する。失敗が値テストかコード生成かは、テスト名ではなく出力で判断する。32-bit ARM は値テストだけを実行し、ゼロコストは `docs/zero-cost.md` の `manual` 行に記録する。

### ソースと文書の整合性

実装と文書の対応を検査する。命令の意味論とコード生成は保証しない。

- `tests/consistency.rs`: Rust の extern 宣言と C の定義、rustdoc の言語、wasm32 のレーン即値の範囲。
- `make coverage-check`: arch の README の表の形式、表の名前と公開ラッパーの対応、ゼロコスト表の fixture と kernel。
- `make check-mnemonics`: rustdoc の命令名が `llvm-mc` に受理されること。
- `make docs-check`: Markdown のリンク、アンカー、索引からの到達性。
- `make fixtures-check`: `fixtures/` と `fixture-matrix.md` の対応、`nested_dir` の名前。

### 利用例（doctest）

rustdoc の例が、公開パスと基本的な使い方に対してコンパイル・実行できることを保証する。

- wasm32: 72 件の実行例と 2 件の `compile_fail`。`wasm32-wasip1` で wasmtime が実行する。
- x86_64: 1 件。SDE が実行する。
- i686: 1 件。Debian イメージの QEMU が実行する。
- その他の native arch: 各 arch 1 件、合計 7 件。Arch イメージでは `--no-run` でコンパイルだけを確認し、Debian イメージでは cross GCC と QEMU で実行する。

上記に加えて、公開命令を持つ 115 extension の rustdoc に **instruction inventory** の doctest を置く。各 extension の公開命令を `use ... as _;` で列挙し、rustdoc が全パスをコンパイルする。現在は 4,528 命令の import を覆っている。`make doctest-inventory-check` が在庫の更新漏れと、arch の README に載る命令の欠落を検査する。

native arch の doctest は GNU target でビルドする。Debian イメージの `DOC_TEST=run` では、cross GCC と glibc をリンクして QEMU で実行する。Arch イメージではターゲット用の libc とリンカを用意しないため、コンパイルだけを確認する。

doctest は命令の意味論を網羅しない。inventory は公開パスのコンパイルを保証し、値の実行は値テストが担う。利用例は、公開パスや値の扱いが自明でない命令に置く。

## 実行単位

| make target                   | 役割                                                | 実行するもの                                                                                         |
| ----------------------------- | --------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `test-x86_64`                 | x86_64 の値テスト、利用例、コード生成、ソース整合性 | `tests/x86`、x86_64 の doctest、`tests/x86_link`、`tests/consistency`（値テストと doctest だけ SDE） |
| `test-cross`                  | native の値テストとコード生成                       | `tests/<arch>` と `fixtures/<arch>-values`                                                           |
| `test-doc`                    | native の利用例（コンパイルのみ）                   | 各 native target の doctest（8 件）                                                                  |
| `test-doc-debian`             | native の利用例（実行）                             | Debian イメージの各 native target の doctest（8 件）                                                 |
| `test-debian`                 | Debian イメージの全テスト                           | `test` 全体と native doctest の実行                                                                  |
| `test-wasip1`                 | wasm32 の値テストと利用例                           | `tests/wasm32`、wasm32 の doctest                                                                    |
| `test-wasip1-wide-arithmetic` | `wide-arithmetic` 有効時の値テスト、利用例、リンク  | feature 有効時の wasm32 のテストと `tests/wide_arithmetic_link`                                      |
| `test-check-wasm-link`        | wasm32 のリンクと `simd128` 無効時のビルド拒否      | `tests/wasm_link`                                                                                    |
| `test-check-static`           | ソース整合性だけを実行する                          | `tests/consistency`                                                                                  |

`test-doc` は、native arch に doctest を追加すると自動的に対象にする。Arch イメージでは `--no-run` でコンパイルだけを確認する。実行まで行う場合は `make test-doc-debian` または `make test-debian` を使う。x86_64 と wasm32 の doctest は、それぞれ `test-x86_64` と `test-wasip1` が実行する。

## テストを追加するとき

1. 追加するテストがどの役割を担うかを決める。
2. 命令の意味論には値テストを追加する。native は fixture、wasm32 は `tests/wasm32`、x86_64 は `tests/x86` に置く。
3. ゼロコストを主張する場合は、`kernel_*` と disassembly の検査を追加する。fixture 化できない場合は `docs/zero-cost.md` の `manual` 行に理由を書く。
4. 即値の範囲を追加する場合は、可能なら `compile_fail` doctest も追加する。
5. arch の README、`fixture-matrix.md`、`zero-cost.md` を同じ変更で更新する。
6. 既定の `make test` で実行する必要があるテストは、`test` の依存に含める。

## 既知の欠落

- native arch の即値の範囲外は、コンパイル拒否を自動で検査していない。wasm32 だけが `compile_fail` doctest を持つ。
- instruction inventory は公開パスの import だけを検査する。命令の実行とリンクは値テストが担う。
- Arch イメージの native doctest はコンパイルだけを確認する。実行は Debian イメージが担う。値テストは doctest の例を実行しない。
- SVE と RVV のスライス API は、長さの不一致で panic する。panic 条件を検査するテストはない。
- `docs/zero-cost.md` の `manual` 行は、測定結果を再実行しない。再現手順と理由だけを検査する。
- x86 の任意 feature の値テストは、CPU が feature を持たない場合に早期 return する。`make test` は SDE で実行するため、既定の経路では feature を有効にできる。SDE を使わない実行では、テストが成功しても命令を実行していないことがある。
