# 開発

このリポジトリを変更する手順は [docs/](docs/README.md) に置く。共通の実装方針は [docs/policy.md](docs/policy.md) にある。

arch を足すときは、`docs/<arch>/README.md` と `docs/<arch>/feasibility.md` を追加し、[docs/README.md](docs/README.md) の索引に足す。

命令の動作と、利用者が extension を有効にする条件は rustdoc に書く。

変更したら `make fmt` で整形し、まず `make check` を通す。変更範囲の `test-<arch>` を実行し、最後に `make test` を通す。

## MR

- 1 MR = 1 extension、または1つの命令ファミリとする。設計MRと実装MRは分ける。
- MRテンプレートは `.gitlab/merge_request_templates/default.md` を使う。
- `Depends on:` と `Blocks:` を書き、前提MRが未マージならDraftにする。
- 対象arch / extension、テスト環境、zero-costの確認結果、docsの更新を書く。
- 実装MRは、そのMR内で値テスト、シンボル検査、rustdoc、arch READMEの更新まで終える。
- `make inventory` で未実装件数を確認できる。件数は作業の在庫であり、期間の見積もりには使わない。
