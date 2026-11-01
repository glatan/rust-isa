# ソースの置き方

公開パス、extension 名、値型、rustdoc の契約は [API契約](api-contract.md) に置く。この文書は、ソースとテストの置き場所だけを扱う。

extension の中のモジュールは非公開にし、ファイル名は命令の接頭辞または命令カテゴリに合わせる。公開パスにはそのファイル名を出さない。命令は extension から再公開する。

extension の実装ファイル（C や `asm!` を含む Rust）は、extension のディレクトリに置く。extension ごとの文書は `docs/<arch>/README.md` に置く。wasm32 の `simd128` と `relaxed-simd` のように、build 固有の説明を分ける場合だけ `docs/<arch>/` に個別の文書を置く。

## テスト

テストの役割は [テストの役割](testing.md) に従う。

- 統合テストは `tests/<arch>.rs` または `tests/<arch>/main.rs` を起点にする。family と extension の名前は `src/<arch>/` に合わせる。
- extension のテストが1ファイルで足りる場合は `tests/x86/simd/sse2.rs` に置く。
- 複数ファイルに分かれる場合は `tests/x86/simd/avx2/` のように extension 名のディレクトリに置く。
- 複数の extension をまとめる場合は `tests/x86/simd/avx512/` のように family 内のまとまりの名前のディレクトリに置く。
- extension 内の命令カテゴリで分ける場合は、そのディレクトリ内のカテゴリ名のファイルに置く（例: `tests/x86/simd/avx/compare.rs`）。
- QEMU で実行する native の値テストは `fixtures/<arch>-values` に置き、`tests/<arch>.rs` からビルドして実行する。
- 共通処理は `tests/common/mod.rs` に置く。

## 上流ソースと manifest

- 上流ソースの版・commit・license は `tools/upstream/SOURCE.toml` に置く。
- extension ごとの命令 form は `tools/isa/<arch>/<extension>.toml` に置く。
- manifest の形式は `tools/isa/README.md` に従う。
- 上流の抽出データは `tools/upstream/` に置き、build 時にネットワークへ取りに行かない。
- 第三者ソースの license と同梱可否は [第三者ソースとライセンス](third-party.md) に置く。

## ビルド

C はリポジトリにコミットする。コンパイル済みのオブジェクトは同梱しない。`build.rs` は、arch ごとに対象 triple とフラグを選び、extension ごとの静的ライブラリとしてリンクする。命令の一覧は生成しない。

実装方式は [実装方式](implementation.md) に従う。
