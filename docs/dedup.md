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

## 主な重複

| arch    | 重複の種類                                          | 扱い                                                                               |
| ------- | --------------------------------------------------- | ---------------------------------------------------------------------------------- |
| x86     | SSE2 / AVX / AVX2 / AVX-512 の 128/256/512-bit 形式 | 形式ごとにラッパーを分ける。同じ形式の演算は内部カーネルを共有できる。             |
| x86     | AVX-512F / BW / DQ / VL / AVX10                     | feature と値幅が同じ場合だけ共有する。マスクの有無は別関数にする。                 |
| PowerPC | Altivec / VSX / Power8〜10                          | 同じ mnemonic でも feature と値型が違うことがある。dedup表で確認してから共有する。 |
| AArch64 | NEON / SVE                                          | API の形が違う（`V128` とスライス）。C も共有しない。                              |
| AArch64 | NEON / fp16 / fcma / fhm                            | 同じ mnemonic でも要素型が違う。別関数にする。                                     |
| RISC-V  | V の固定長 API / スライス API                       | API の形が違う。C も共有しない。                                                   |
| ARM     | NEON / MVE                                          | MVE は対象外。共有しない。                                                         |

## レビュー

重複する命令を実装する MR では、次を確認する。

- 既存の C 関数を再利用する場合、その関数が要求する feature を新しい extension が満たすか。
- 再利用する C 関数のレーン幅と、新しい API の値型が一致するか。
- 新しい extension の方が弱い feature を要求する場合、既存関数をそのまま使わない。
- 公開ラッパーの rustdoc は extension ごとに書く。共有関数の rustdoc を流用しない。
