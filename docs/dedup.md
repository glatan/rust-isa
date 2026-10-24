# 重複する命令名

同じ arch の中で、同じ命令名が複数の extension に現れることがある。これはエラーではない。公開APIは extension ごとに分け、内部の C 関数だけを共有できるか検討する。

`make duplicates` で、同じ arch の複数 extension に現れる命令名を表示する。

```sh
make duplicates
make duplicates DUPLICATES_ARGS='--arch powerpc'
```

## 判定

次の条件をすべて満たす場合だけ、内部 C 関数を共有する。

- レーンの意味と要素型が同じ。
- 飽和、丸め、NaN の扱いが同じ。
- 必要な feature の集合が同じ、または共有する側がより強い feature を要求する。
- 値型が同じ。`V128` と `V256` のように幅が違う場合は共有しない。

一つでも違う場合は、extension ごとに C 関数を分ける。公開ラッパーは常に extension ごとに置く。

## arch ごとの重複

現在の重複と扱いは、各 arch の `README.md` の実装方式と `make duplicates` の出力を見る。この文書は判定規則だけを置き、arch ごとのスナップショットは持たない。

## レビュー

重複する命令を実装する MR では、次を確認する。

- 既存の C 関数を再利用する場合、その関数が要求する feature を新しい extension が満たすか。
- 再利用する C 関数のレーン幅と、新しい API の値型が一致するか。
- 新しい extension の方が弱い feature を要求する場合、既存関数をそのまま使わない。
- 公開ラッパーの rustdoc は extension ごとに書く。共有関数の rustdoc を流用しない。
