# 実装方針

この文書は、arch に共通する実装方針である。arch ごとの対象、実装状況、ツールチェーンは `docs/<arch>/README.md` に置く。

## クレートの目的

- `core::arch` に相当する API を、stable の `no_std` で提供する。
- `no_std` とし、`alloc` を使わない。
- 公開パスは `{arch}::{family}::{extension}::{instruction}` とする。
- 命令の動作は各 arch の仕様に従い、rustdoc に英語で書く。

## 網羅の単位

固定長の extension は、命令ごとに API を公開し、arch の表の命令を網羅する。

SVE、RVV、SME、AMX、PowerPC MMA は、固定長の値型と1命令1関数の形に当てはまらない。これらは操作単位で API を公開する。

- SVE と RVV は、スライス API の操作を単位とする。
- SME、AMX、PowerPC MMA は、guard 型または状態型と、その状態を要求する操作を単位とする。
- ISA 命令と API の対応を [スケーラブル・状態付き extension の API 対応](scalable-mapping.md) に置く。
- 実装状況は各 arch の `README.md` を正とする。対応表は状態を持たず、実装済み・未実装・対象外の理由は arch の表に書く。
- 対応表は代表的な対応を示し、全 API は arch の `README.md` に置く。全 ISA 命令の列挙は外部仕様に依存するため行わず、対象範囲は [実装状況の集計](implementation-status.md) に定義する。
- 固定長 extension の命令を網羅する。即値や状態の都合で出せない命令は、その理由を arch の表と rustdoc に書く。
- スケーラブル・状態付き extension は、操作単位で網羅する。ISA 命令と API の対応を arch の文書に書く。

## 非目標

- 高水準の SIMD ライブラリにはしない。命令ごとの API だけを置く。
- 実行時ディスパッチをクレートの既定にはしない。
- 自動ベクトル化には依存しない。
- C の ABI や C シンボルを公開 API にしない。
- `core::arch` の型や intrinsic を再エクスポートしない。

## `core::arch` を使わない

`core::arch` の型、intrinsic 関数、feature 検出マクロを、公開 API にも内部実装にも使わない。`#[target_feature]` は Rust の属性であり、使ってよい。

`asm!` は `core::arch` から公開されるマクロだが、intrinsic 関数ではない。この文書では、禁止の対象を型と intrinsic 関数に限定し、`asm!` は補助方式として許可する。`asm!` も禁止する場合は、C だけを実装方式にする。

## 可用性とゼロコスト

- **可用性**は、API が存在する条件である。
- **ゼロコスト**は、命令と命令の間の値がメモリへストアされ、直後にロードされないことである。関数呼び出しのコストは別に扱う。
- この二つを同じ条件にしない。ゼロコストでないことは、実装できないことを意味しない。

## 可用性

公開条件の規則は [API契約](api-contract.md) に置く。ここでは決定の理由だけを書く。

- wasm32: モジュールが含む命令の feature を全体として要求するため、cfg ゲートでモジュールを公開する。
- native: API の存在を呼び出し側の rustflags に依存させないため、無条件に公開する。任意の feature を要求する命令は `unsafe fn` とする。
- Cargo feature: extension の組み合わせが additive でない場合があり、wasm の cfg ゲートと二重になるため、原則として採用しない。`wide-arithmetic` だけ例外とする。

## 実装方式

方式、代替案、ツールチェーンは [実装方式](implementation.md) に置く。

## ゼロコスト

ゼロコストは、LTO と feature 集合の条件を満たすビルドだけに適用する。規範条件、測定方法、arch 別の結果は [ゼロコストの確認](zero-cost.md) に置く。この文書には状態表を置かない。

## スケーラブルベクタと状態を持つ拡張

SVE、RVV、SME、AMX、PowerPC の MMA は、固定長の `V128` と 1 命令 1 関数の形にそのまま当てはまらない。API の形は次のように決めた。

- SVE: ベクタ長は実行時に決まる。固定長の値型では表せない。`Vl` を引数に取る API、またはメモリ上のスライスに対する API になる。固定長レジスタ型の命令別関数を並べても、値がレジスタに残る保証はない。スライス API は AArch64 で、C のループが `#[target_feature(enable = "sve")]` のカーネルへインラインされ、`call` が無いことを確認した。
- RVV: 固定長の `V128` 部分集合なら、`vadd.vv` / `vmul.vv` でゼロコストを確認した。VL を実行時にするスライス API も、`vsetvli` / `vle32.v` / `vadd.vv` / `vse32.v` のループがカーネルへインラインされ、`call` が無いことを確認した。
- SME: ストリーミングモードと ZA の状態を持つ。`SMSTART` / `SMSTOP` に対応する guard 型が必要になる。
- AMX: タイル設定とタイル解放の状態を持つ。設定用の guard 型と、OS のタイル状態サポートが必要になる。
- PowerPC MMA: アキュムレータの状態を持つ。設定と解放を含む API になる。

これらは「cfg が立たない」問題ではなく、API の形を決める問題である。固定長レジスタ型の命令別 API は出さず、スライス操作または guard を取る関数として公開する。設計は [スケーラブルベクタと状態](scalable-state.md) に置く。

clang 23.1.1 で確認したこと:

- AMX は `-mamx-tile -mamx-bf16` で C がコンパイルでき、`ldtilecfg`、`tileloadd`、`tilestored`、`tilerelease` を出した。
- SME は `-march=armv9-a+sme` で C がコンパイルできた。`__arm_streaming` は宣言子の後ろに置く。関数は `smstart` / `smstop` を出さないため、呼び出し側がストリーミングモードに入る guard を持つ必要がある。
- PowerPC の MMA は `-mcpu=power10 -mmma` で C がコンパイルでき、`xvi4ger8` と `xxmfacc` を出した。アキュムレータの状態を持つ。

## API の参照

公開APIの形、値型、即値、エンディアン、安全境界は [API契約](api-contract.md) に置く。この文書には重複して書かない。

## 機能検出とディスパッチ

- クレートは実行時検出を提供しない。`no_std` の既定にできないためである。
- native の利用者は、CPU の feature を自分で確認し、`unsafe` な関数を呼ぶ。
- std を使う利用者は、`is_x86_feature_detected!` などで分岐できる。この場合も、クレートの関数は `unsafe` のままである。
- 将来 `std` feature を足す場合も、ディスパッチは別の層にする。命令の API には検出を入れない。

## テスト

実装したら、値テスト、命令の存在、シンボル一致のテストを同じ変更に追加する。

- テストの役割と実行単位: [テストの役割](testing.md)
- 値テストの設計と arch 別の注意: [native の値テスト](native-testing.md)
- fixture と runner: [値テストの fixture](fixture-matrix.md)
- ゼロコストの確認方法: [ゼロコストの確認](zero-cost.md)
- cfg の観測: [実装可否](feasibility.md)
- 実行するコマンド: [開発環境](development.md)

## セキュリティ

- 暗号命令のラッパーは、命令をそのまま呼ぶ。定数時間性は命令と利用者の使い方に依存する。クレートは秘密依存の分岐を追加しない。
- 秘密値のゼロ化、メモリのロック、サイドチャネル対策は提供しない。利用者の責任とする。
- メモリ命令の境界とアラインメントは、呼び出し側が保証する。

## 互換性の参照

semver と互換性の規則は [API契約](api-contract.md) の「変更」に置く。この文書には重複して書かない。

## テストの実行環境

arch ごとの値テストの runner と実行状況は [値テストの fixture](fixture-matrix.md) に置く。実装状況は各 arch の `README.md` を正とし、この文書には置かない。

## arch と extension を追加する手順

実装の順序と命令ごとの状態は、各 arch の `README.md` に置く。ISA 命令と API の対応は [API 対応](scalable-mapping.md) に置く。

1. 対象 triple と既定の feature を決める。
2. `docs/<arch>/feasibility.md` に cfg の観測表を置く。
3. `docs/<arch>/README.md` に可用性、実装方式、ゼロコスト、ツールチェーン、未決定の設計を書く。
4. 値の型と安全境界を決める。
5. C または `asm!` を実装し、extension の `mod.rs` に契約を書く。
6. 値テスト、命令の存在、シンボル一致のテストを追加する。
7. ゼロコストを主張する場合は、wasm32 は `tests/wasm_link.rs`、native は disassembly の確認を追加する。確認できるまでは「未確立」と書く。
8. `docs/README.md` の索引に arch を足す。

## 決定

- `core::arch` を使わない。公開 API にも内部実装にも使わない。
- wasm32 は cfg ゲートで公開する。
- native は無条件に公開し、任意の feature を要求する命令は `unsafe fn` とする。
- C を主方式、`asm!` を補助方式とする。
- C は GCC の vector 拡張と通常のロード・ストアを優先する。intrinsics は必要なら使い、ゼロコストを disassembly で確認する。
- 独自の値型を使う。
- native のゼロコストは、x86_64、AArch64、RISC-V、LoongArch、s390x、32-bit ARM、PowerPC、MIPS で確認した条件を満たすビルドで保証する。条件を満たさないビルドは呼び出しベースとする。
- native の 128-bit は、x86 では `asm!` と `u128` でもゼロコストにできる。AArch64 の `asm!` は stable な値型が無いため、C + LTO を使う。256/512-bit も C + LTO を使う。
- 実行時ディスパッチはクレートに入れない。

## 保留

- unstable feature のゼロコストを利用者が設定する方法。
- Cargo feature による opt-in。
- 実行時ディスパッチを提供する `std` feature。
- native の全命令の網羅。固定長 extension の未実装命令は arch の表で管理し、スケーラブル・状態付き extension は操作単位の対応表で管理する。
- 追加した命令のゼロコスト確認。確認できるまでは zero-cost を主張しない。
