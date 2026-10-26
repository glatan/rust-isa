# native の値テスト

この文書は、native arch の値テストを `tests/` に追加するときの設計である。実行状況と runner は [値テストの fixture](fixture-matrix.md) と各 arch の `README.md` を参照する。値テストとゼロコスト検査の役割の分類は [テストの役割](testing.md) にある。

## 前提

- 開発イメージには `qemu-user-static`、SDE、wasmtime と、arch ごとの runner が入っている。
- x86_64 と i686 は SDE、その他の native arch は QEMU の runner を使う。
- QEMU / SDE の runner は、静的にリンクしたテストバイナリを前提にする。
- テストは `no_std` のバイナリとしてビルドし、終了コードで合否を返す。

## 置き場所

```text
fixtures/i686-values/
  Cargo.toml
  src/main.rs
```

`fixtures/i686-values` は workspace のメンバーにしない。`tests/<arch>.rs` から `cargo build --target <triple>` でビルドし、SDE / QEMU で実行する。wasm の `fixtures/wasm-simd-link` と同じ形にする。

## テストの形

- `build.rs` が `TARGET` を見て C のフラグと clang の triple を選ぶ。
- `src/main.rs` は `_start` で公開 API を呼び、結果を比較し、終了コードを返す。
- 終了コードは、`0` が成功、`1..=254` が失敗したチェックの識別子、`255` が panic とする。同じ `run()` の中で失敗コードを重複させない。`tests/consistency.rs` の `check_fixture_exit_codes` が範囲と重複を検査する。
- Rust と C の LTO モードを一致させる。zero-cost の確認は別途、disassembly で行う。

arch 固有の注意は各archの `README.md` に置く。値テストの fixture と runner は [値テストの fixture](fixture-matrix.md) を見る。

- **静的リンク**: 動的リンクのバイナリは、ターゲットの loader と libc が無いため SDE / QEMU で動かない。

## 追加するテスト

native の API を実装したら、extension ごとに次を追加する。

1. 値を返す演算（add、mul、compare など）
2. メモリを読む命令（load、store）
3. lane 即値（extract、replace）
4. 値テストが通ること
5. 必要なら zero-cost の disassembly 確認

`tests/<arch>.rs` には、`cargo build`、SDE / QEMU の実行、終了コードの確認を追加する。失敗時は build log と終了コードを表示する。

## 実行できない命令

SDE / QEMU が実行しない命令は、API のコンパイルだけを確認する。arch ごとの対象は [値テストの fixture](fixture-matrix.md) の「実行できないもの」に置く。
