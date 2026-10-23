# wasm32

`wasm32` のソースは `src/wasm32` にある。関数名は、WebAssembly のテキスト命令名の `.` を `_` にしたものである。この規則は `wasm32` の rustdoc にも書いてある。

family は `simd` である。extension は次である。

- [simd128](simd128.md)
- [relaxed-simd](relaxed-simd.md)

## 実装状況

対象は各 extension の全命令である。次の表は、現在公開している命令と、
まだ公開していない命令の一覧である。値テストの列は、その extension の
値テストを実行する環境を示す。未実装命令も同じ表で管理する。

### SIMD と整数

| extension         | 実装済み命令                                                                   | 未実装命令                                      | 値テスト |
| ----------------- | ------------------------------------------------------------------------------ | ----------------------------------------------- | -------- |
| `simd128`         | 212 命令（`i8x16` / `i16x8` / `i32x4` / `i64x2` / `f32x4` / `f64x2` / `v128`） | `i8x16.shuffle` など（API 未整備）              | wasmtime |
| `relaxed-simd`    | 20 命令                                                                        | なし                                            | wasmtime |
| `wide-arithmetic` | `add128` / `sub128` / `mul_wide_u` / `mul_wide_s`                              | なし                                            | wasmtime |
| `atomics`         | なし                                                                           | 共有メモリ・スレッド前提のため命令 API にしない | 対象外   |
| `fp16`            | なし                                                                           | LLVM のみで rustc の cfg が立たないため対象外   | 対象外   |

## 実装方式

実装済みの `simd128` と `relaxed-simd` は、今回の方針整理では変更しない。

- extension の `cfg(target_feature)` が立つときだけ、モジュールと C をコンパイルする。
- C + clang + LTO を使う。C は `<wasm_simd128.h>` を使う。
- 値は `V128` をポインタで渡す。
- 呼び出し側が `-C target-feature=+simd128,+relaxed-simd` と `-C linker-plugin-lto` を付けると、C の bitcode が呼び出し側へインラインされ、`v128` の往復が消える。
- `wide-arithmetic` は cfg ゲート方式では関数を出せない。C を無条件にリンクすると、モジュール全体が `wide-arithmetic` を要求する。Cargo feature で opt-in する（下記）。
- `asm!` は wasm32 では experimental であり、stable では使えない。

## 利用者に必要な設定

`.cargo/config.toml` は、`wasm32-unknown-unknown` と `wasm32-wasip1` の `rustflags` と `rustdocflags` に `-C target-feature=+simd128,+relaxed-simd` と `-C linker-plugin-lto` を付ける。この設定は、このリポジトリのテストが `simd128` と `relaxed_simd` をコンパイルするためのものである。`rust-isa` を依存に追加したパッケージへは渡らない。利用者が付けるフラグは、各 extension の rustdoc に書いてある。

## `wide-arithmetic`

`wide-arithmetic` は rustc の cfg が立たない。cfg ゲート方式では関数を出せず、無条件にリンクするとモジュール全体が feature を要求する。この extension だけは Cargo feature で opt-in する。

- feature 名は `wide-arithmetic` とする。既定では無効にする。
- 有効なとき、`build.rs` は C を `-mwide-arithmetic` でコンパイルし、モジュールを `cfg(feature = "wide-arithmetic")` で公開する。
- `fixtures/wide-link` をビルドし、`wasm2wat` で四つの命令を確認する。`make test-wasip1-wide-arithmetic` では、加算・減算・乗算の値テストを実行する。
- この feature を有効にした wasm モジュールは、`wide-arithmetic` を要求する。利用者はこの条件を受け入れる場合だけ有効にする。
- `docs.rs` のメタデータで all-features を使う場合も、この条件を rustdoc に書く。

## 未対応

- `atomics`。共有メモリとスレッドを要求するため、値 API にしない。
- `fp16`。LLVM のみで rustc の cfg が立たないため、対象外にする。
- Cargo feature を有効にしたときの docs.rs の表示。
- `wide-arithmetic` の命令のうち、API にする範囲は 4 命令に限定している。

## 文書

- [実装可否](feasibility.md)
- [リンク後の確認](linking.md)
- [実装方針](../policy.md)
