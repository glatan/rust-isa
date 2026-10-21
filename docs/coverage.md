# 実装状況の集計

`make coverage` は、`docs/<arch>/README.md` の実装状況の表を読み、arch / extension ごとの実装済み件数と未実装件数を表示する。

```sh
make coverage
make coverage COVERAGE_ARGS='--arch aarch64 --todo'
make coverage COVERAGE_ARGS='--json'
make coverage-check
```

`--manifest PATH` を付けると、表から作った在庫を JSON で書き出せる。マニフェストは命令の実装対象を確定するための作業用であり、`build.rs` は読まない。C と Rust はリポジトリにコミットする。

## 表の形式

各 arch の README の表は、次の列を持つ。

| 列           | 内容                                                       |
| ------------ | ---------------------------------------------------------- |
| extension    | 拡張名。バッククォートで囲む。                             |
| 実装済み命令 | 公開している命令名。複数はカンマで区切る。                 |
| 未実装命令   | まだ公開していない命令名。対象外・延期もここに理由を書く。 |
| 値テスト     | ホスト、QEMU、wasmtime、未実行のいずれか。                 |

セル内のバッククォート付きの名前を1件として数える。`なし` は0件とする。`236 命令（...）` のように先頭が件数の場合は、その数を使う。

`make coverage-check` は、各表が4列であること、extensionがバッククォートで囲まれていること、実装済みと未実装のセルが空でないこと、同じarchでextensionが重複しないことを検査する。`make test` はこの検査を最初に実行する。

## 使い方

- 実装を追加したら、その arch の README の「実装済み命令」と「未実装命令」を同じ変更で更新する。
- 未実装件数は作業の在庫であり、期間や優先順位の見積もりには使わない。
- スケーラブル・状態付き extension は、命令名ではなく API の対応表を別に持つ。対応表の作り方は [実装方針](policy.md) と [スケーラブルベクタと状態](scalable-state.md) に書く。
