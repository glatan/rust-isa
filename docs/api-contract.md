# API契約

この文書は、公開APIの形を固定する。実装方式は [実装方式](implementation.md) に、可用性の背景は [実装方針](policy.md) に書く。

## パス

公開パスは `{arch}::{family}::{extension}::{instruction}` とする。

- `family` は名前空間である。`simd`、`crypto`、`integer`、`matrix` などを使う。
- `extension` は rustc の feature 名の `.` と `-` を `_` に置き換える。`sse4.1` は `sse4_1`、`relaxed-simd` は `relaxed_simd` である。
- 複数の extension をまとめるグループ（`riscv::simd::zv`、`riscv::float::scalar`）と、API 形状を表すモジュール（`riscv::simd::rvv_slice`）は、arch の `README.md` に規則を書いた場合だけ例外とする。
- 命令は extension から再公開する。extension の中のモジュールは非公開にする。
- family を持たない extension は arch 直下に置く。`wasm32::wide_arithmetic` が該当する。`wide-arithmetic` は cfg ゲートを持てず Cargo feature で opt-in するため、`simd` などの family に属さない。
- 関数名は `{接頭辞}_{命令名}` とする。接頭辞は arch の規則に従う。
- 値型は命令名に含めない。レーンの意味は命令が決める。

## 値型

- 128/256/512-bit の値は、arch ごとに `V128` / `V256` / `V512` を定義する。
- レイアウトは `#[repr(C, align(N))]` のバイト列とする。`N` は 16、32、64 である。
- `core::arch` の型は公開しない。
- 値型に `Add` などの演算子を実装しない。同じビット列でも命令によって意味が変わるためである。
- 値型は `Copy`、`Clone`、`Debug`、`Eq`、`PartialEq` を実装してよい。
- 128-bit は `u128` を内部に持つ形も許す。`asm!` の `xmm_reg` は `u128` を受け付ける（x86 で確認）。
- C との境界ではポインタを使う。値渡しの ABI は環境依存なので公開しない。

## 可用性

### native

- 原則として無条件に公開する。API の存在を呼び出し側の rustflags に依存させない。
- 任意の feature を要求する命令は `unsafe fn` とする。CPU の対応と feature の有効化は呼び出し側の責任である。
- rustc が stable に扱う feature は、呼び出し側のカーネルに `#[target_feature(enable = "...")]` を付ける。
- unstable または LLVM のみの feature は、`#[target_feature]` を付けず、rustdoc に条件を書く。
- ABI や状態を変える拡張（SVE、SME、AMX、RVV）は、無条件公開の前に型と状態を設計する。

### wasm32

- extension の `cfg(target_feature)` でモジュールを公開する。
- wasm のモジュールは、含む命令の feature を全体として要求する。feature が無効なビルドに関数を置くと、モジュールがその feature を要求する。
- `simd128` と `relaxed-simd` はこの方式である。

### Cargo feature

- 原則として採用しない。extension の組み合わせが additive でない場合があり、wasm の cfg ゲートと二重になるため。
- `wide-arithmetic` のように cfg ゲートとモジュール全体の feature 要件が両立しない extension は、extension 名の Cargo feature で opt-in する。既定では無効にし、有効なビルドが feature を要求することを rustdoc に書く。
- 代替として検討する場合は、extension ごとに opt-in し、docs.rs と利用者の設定を arch 文書に書く。

### ディスパッチ

- 実行時ディスパッチはクレートに入れない。

## 安全境界

- 既定の feature に含まれる演算は safe にできる。
- semantic helper は命令 wrapper ではない。値の意味だけを提供し、対象CPUが命令を持つことを保証しない。semantic helper は safe にでき、命令の在庫に数えない。
- 既定の feature を無効化したビルドはサポートしない。safe な API は、そのターゲットの既定の feature を前提にする。
- 任意の feature を要求する命令は `unsafe fn` とする。
- wasm32 の cfg ゲートされた演算は safe にできる。モジュールが feature を要求するため、feature 無効の環境では動かない。
- メモリ命令は常に `unsafe fn` とする。ポインタの有効性、境界、アラインメントは呼び出し側が保証する。
- `#[target_feature]` を付けた関数は、その feature を持つ呼び出し元からだけ呼ぶ。
- ラッパーは panic しない。panic する可能性がある処理を入れない。
- スライス API で長さが一致しない場合の panic は、関数の rustdoc に書く。

## 即値

- レーン、シフト量、回転量などの即値は `const` generic とする。レーンは `const LANE: u32` とする。
- 範囲外の即値はコンパイルエラーにする。`const { assert!(...) }` を使う。
- 即値ごとに C の関数を用意し、Rust 側は `match` で振り分ける。
- 即値の範囲と意味を rustdoc に書く。

## 比較と浮動小数点

- 比較の結果は、真のレーンをすべて 1、偽のレーンをすべて 0 にする。
- NaN、signed zero、丸めの扱いは命令の仕様に従い、rustdoc に書く。
- 飽和演算は、飽和する条件と結果を rustdoc に書く。

## エンディアン

- バイト列の順序とレーンの順序を混同しない。
- 多バイトのレーンは、arch の仕様に従う。リトルエンディアンとビッグエンディアンの別を rustdoc に書く。
- s390x と PowerPC はビッグエンディアンである。値テストでレーン順を確認する。

## スケーラブル・状態付き extension

- SVE と RVV は、固定長の値型ではなくスライス API を提供する。
- SME、AMX、PowerPC MMA は、guard 型または状態型を提供する。状態を要求する命令は、その型への参照を引数に取る。
- guard は `Send` と `Sync` を実装しない。`Drop` で状態を終了し、入れ子を許さない。
- スケーラブル・状態付き extension は、ISA 命令と API の対応を [スケーラブル・状態付き extension の API 対応](scalable-mapping.md) に置く。
- 実装状況は各 arch の `README.md` を正とし、対応表には状態を持たせない。

## rustdoc

各関数の rustdoc には、次を英語で書く。

- 命令名と動作
- 必要なら feature と `#[target_feature]` の条件
- `unsafe` の条件
- 即値の範囲
- エンディアンと NaN の扱い
- 対象外・延期の理由（命令を公開しない場合）

## 変更

- 公開 API は semver に従う。
- wasm32 の cfg ゲートされた API は、feature を有効にしたビルドの API として semver の対象にする。
- extension の追加は minor とする。命令の削除、意味の変更、公開条件の変更は major とする。
- ツールチェーンの最小バージョンを上げる変更は minor とし、arch 文書に書く。
- C のシンボルと値型のメモリ配置は公開 ABI ではない。
