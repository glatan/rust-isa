# 文書の位置

README には、クレートの説明、公開パス、ライセンス、[CONTRIBUTING.md](../CONTRIBUTING.md) へのリンクを置く。

rustdoc は英語で書く。`src` の `///` と `//!` にひらがな、カタカナ、漢字があると `cargo test` が失敗する。

クレート直下には、`no_std` であること、`core::arch` を使わないこと、パスの形だけを書く。関数名の規則は arch のモジュールに書く。

extension の `mod.rs` には、次を書く。

- 対応する仕様へのリンク
- 値の型
- そのモジュールの公開条件（無条件、`cfg`、Cargo feature のいずれか）
- 利用者が付けるフラグのうち、必須のものとゼロコストのためのもの
- ビルドに必要な外部ツール
- safe と `unsafe` の境界、即値の規則
- 意図的に含めていない命令

命令ごとの動作と例は、その関数の rustdoc に書く。

## arch ごとの文書

`docs/<arch>/README.md` には、次を書く。

- 対象の triple
- 既定の feature と、実装の順序
- 実装状況（extension ごとの実装済み命令と未実装命令の表）
- 実装方式（C、`asm!`、純 Rust）
- 値の型と C との受け渡し
- ゼロコストの条件と、測定したかどうか
- ツールチェーンと、確認した clang のバージョン
- 未決定の設計（スケーラブルベクタ、状態管理など）
- 対象外とする命令と、その理由

ゼロコストを確認した arch は、リンクの条件と確認手順を `linking.md` に分ける。例は [x86](x86/linking.md) にある。

`docs/<arch>/feasibility.md` には、その arch の `cfg(target_feature)` の観測表を置く。用語と共通の注意は [実装可否](feasibility.md) に置く。

## 共通の文書

- [実装方針](policy.md): 可用性、ゼロコスト、実装方式、安全境界
- [API契約](api-contract.md): 公開パス、値型、即値、状態型の形
- [実装状況の集計](coverage.md): `make coverage` と表の形式
- [実装可否](feasibility.md): cfg の用語と arch ごとの表への索引
- [ゼロコストの確認](zero-cost.md): 確認方法と arch ごとの結果
- [スケーラブルベクタと状態](scalable-state.md): SVE、RVV、SME、AMX、MMA の API 設計
- [native の値テスト](native-testing.md): QEMU を使う値テストの設計
- [ソースの置き方](source-layout.md): ディレクトリと公開パス
- [開発環境](development.md): リポジトリのコマンド

この `docs/` には、リポジトリを変更する手順を置く。arch や extension に固有の手順は、その arch のディレクトリに分ける。
