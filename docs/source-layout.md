# ソースの置き方

公開パスは `{arch}::{family}::{extension}::{instruction}` である。`family` は名前空間である。extension の中のモジュールは非公開で、ファイル名は命令の接頭辞または命令カテゴリに合わせる。公開パスにはそのファイル名は出ない。命令は extension から再公開する。

extension の名前は、rustc の feature 名の `.` と `-` を `_` に置き換える。たとえば `sse4.1` は `sse4_1`、`relaxed-simd` は `relaxed_simd` である。仕様名をそのまま使う arch は、その arch の `README.md` に規則を書く。

テストも同じ階層に置く。統合テストは `tests/<arch>.rs` または `tests/<arch>/main.rs` を起点にし、family と extension の名前を `src/<arch>/` に合わせる。extension のテストが1ファイルで足りる場合は `tests/x86/simd/sse2.rs`、複数ファイルに分かれる場合は `tests/x86/simd/avx2/` のように extension 名のディレクトリに置く。複数の extension をまとめる場合は `tests/x86/simd/avx512/` のように family 内のまとまりの名前のディレクトリに置く。extension 内の命令カテゴリで分ける場合は、そのディレクトリ内のカテゴリ名のファイルに置く（例: `tests/x86/simd/avx/compare.rs`）。QEMU で実行する native の値テストは `fixtures/<arch>-values` に置き、`tests/<arch>.rs` からビルドして実行する。共通処理は `tests/common/mod.rs` に置く。

extension ごとのファイル一覧は、`docs/<arch>/` のその extension の文書に書く。extension の実装ファイル（C や `asm!` を含む Rust）は、extension のディレクトリに置く。

実装方式は arch ごとに決める。C + clang を主方式とし、`asm!` を補助に使う。LTO は wasm32 と native のゼロコストに使う。native は LTO モードと feature 集合を一致させる。詳しくは [実装方針](policy.md) と、その arch の `README.md` に書く。

C はリポジトリにコミットする。コンパイル済みのオブジェクトは同梱しない。`build.rs` は、arch ごとに対象 triple とフラグを選び、extension ごとの静的ライブラリとしてリンクする。命令の一覧は生成しない。

値の型は arch ごとに定義する。`core::arch` の型は公開しない。C との境界ではポインタを使う。

rustdoc には、命令の動作と、利用者が extension を有効にする条件を書く。ゼロコストの条件は [実装方針](policy.md) に書く。
