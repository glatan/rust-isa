# 開発

このリポジトリを変更する手順は [docs/](docs/README.md) に置く。共通の実装方針は [docs/policy.md](docs/policy.md) にある。

arch を足すときは、`docs/<arch>/README.md` と `docs/<arch>/feasibility.md` を追加し、[docs/README.md](docs/README.md) の索引に足す。

命令の動作と、利用者が extension を有効にする条件は rustdoc に書く。

変更したら `make fmt` で整形し、`make fmt-check` と `make test` を通す。
