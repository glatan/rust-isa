# 文書の位置

文書の分類は [docs/README.md](README.md) の表に従う。この文書は、文書とrustdocの置き場所だけを扱う。

README には、クレートの説明、公開パス、ライセンス、[CONTRIBUTING.md](../CONTRIBUTING.md) へのリンクを置く。

rustdoc は英語で書く。`src` の `///` と `//!` にひらがな、カタカナ、漢字があると `cargo test` が失敗する。

クレート直下には、`no_std` であること、`core::arch` を使わないこと、パスの形だけを書く。関数名の規則は arch のモジュールに書く。

extension の `mod.rs` には、次を書く。

- 対応する仕様へのリンク
- 値の型
- そのモジュールの公開条件（無条件、`cfg`、Cargo feature のいずれか）
- 利用者が付けるフラグのうち、必須のものとゼロコストのためのもの
- ビルドに必要な外部ツール
- safe と `unsafe` の境界、即値の規則（[API契約](api-contract.md)）
- 意図的に含めていない命令

命令ごとの動作は、その関数の rustdoc に書く。rustdoc の必須項目は [API契約](api-contract.md) の「rustdoc」に従う。

公開命令を持つ extension には、全公開命令を呼ぶ instruction examples と、期待値を持つ代表的な behavior example を置く。instruction examples はSDE・QEMUなど共有ランナーで実行できるものを実行し、実行できないCPU機能・状態は `no_run` にする。命令の意味論の網羅は値テストが担う。役割の分類は [テストの役割](testing.md) にある。

公開命令を持つ各 extension の `mod.rs` には、公開命令を列挙した instruction inventory の doctest を置く。命令の追加・削除時は `python3 tools/doctest_inventory.py --write` で更新し、件数が変わる場合は `docs/doctest-coverage.md` の表も更新する。

## arch ごとの文書

`docs/<arch>/README.md` は、その arch のプロファイルと実装状況を置く。表の形式は [実装状況の集計](implementation-status.md) に従う。

`docs/<arch>/feasibility.md` は、`cfg(target_feature)` の観測だけを置く。用語と共通の注意は [実装可否](feasibility.md) に置く。実装状況は `README.md` を参照し、この文書には置かない。

リンクの条件が一般則と異なる arch だけ、`linking.md` に分ける。例は [x86](x86/linking-and-zero-cost.md) と [wasm32](wasm32/linking.md) にある。

## 索引

文書の一覧は [docs/README.md](README.md) にだけ置く。この文書には索引を置かない。

この `docs/` には、リポジトリを変更する手順と、その判断の記録を置く。arch や extension に固有の手順は、その arch のディレクトリに分ける。
