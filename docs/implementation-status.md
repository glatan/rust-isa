# 実装状況の集計

`make inventory` は、`docs/<arch>/README.md` の実装状況の表を読み、arch / extension ごとの実装済み件数と未実装件数を表示する。

```sh
make inventory
make inventory INVENTORY_ARGS='--arch aarch64 --todo'
make inventory INVENTORY_ARGS='--json'
make inventory-check
```

`--manifest PATH` を付けると、表から作った在庫を JSON で書き出せる。マニフェストは命令の実装対象を確定するための作業用であり、`build.rs` は読まない。C と Rust はリポジトリにコミットする。

## 表の形式

各 arch の README の表は、次の列を持つ。

| 列           | 内容                                                                        |
| ------------ | --------------------------------------------------------------------------- |
| extension    | 拡張名を1つだけ書き、バッククォートで囲む。                                 |
| 実装済み命令 | 公開している命令名。複数はカンマで区切る。                                  |
| 未実装命令   | 延期している命令名。対象外はここに書かず、README の「対象外」に理由を書く。 |
| 値テスト     | ホスト、QEMU、wasmtime、未実行のいずれか。                                  |

セル内のバッククォート付きの名前を1件として数える。`なし` は0件とする。`236 命令（...）` のように先頭が件数の場合は、その数を使う。括弧の中は説明として扱い、件数に含めない。命令 API を持たない feature 行は `なし` と書く。

`make inventory-check` は、各表が4列であること、extensionが1つの名前でバッククォートで囲まれていること、実装済みと未実装のセルが空でないこと、`なし` のセルに名前が混ざっていないこと、同じarchでextensionが重複しないことを検査する。続けて `doc_names.py` が表の名前と `src` の公開ラッパーの対応を、`zero_cost.py` がゼロコスト結果表の fixture と kernel を検査する。`make test` は `fmt-check` の後にこの検査を実行する。

## 対象範囲

固定長 extension の対象は、その extension の仕様のうち、即値・状態・ABI の制約で API にできる命令である。API にできない命令は、README の「対象外」に理由を書く。表の「実装済み」と「未実装（延期）」が現在の在庫である。

SVE、RVV、SME、AMX、PowerPC MMA は、命令ごとの網羅ではなく、[API 対応](scalable-mapping.md) の操作を対象とする。ISA 命令の全体を数えない。

全 ISA 命令の一覧は外部仕様に依存するため、このリポジトリでは列挙しない。表にある名前の実在は `doc_names.py` と `mnemonics.py` で検査する。

## inventoryの実行方法

- 実装を追加したら、その arch の README の「実装済み命令」と「未実装命令」を同じ変更で更新する。
- 未実装件数は作業の在庫であり、期間や優先順位の見積もりには使わない。
- 同一命令でもレーン幅ごとに API を持つため、件数は API の一意な命令数ではない。
- スケーラブル・状態付き extension も、実装状況は arch の README に置く。ISA 命令と API の対応は [API 対応](scalable-mapping.md) に置き、状態は持たせない。
