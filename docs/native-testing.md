# native の値テスト

この文書は、native arch の値テストを `tests/` に追加するときの設計である。実行状況と runner は [値テストの fixture](fixture-matrix.md) と各 arch の `README.md` を参照する。

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
- 終了コードは、`0` が成功、`i + 1` が lane `i` の不一致とする。
- Rust と C の LTO モードを一致させる。zero-cost の確認は別途、disassembly で行う。

arch 固有の注意:

- **MIPS**: 共有の静的配列や lane を引数で選ぶ関数は codegen で一致しなかった。lane ごとに独立した関数を使う。
- **PowerPC**: `_start` から C を呼ぶ場合、`.quad .TOC.` を持つ非 LTO オブジェクトをリンクに加える。
- **i686**: C に `-mstackrealign` を付ける。値テストは SDE の 32-bit runner で実行する。
- **s390x**: `llvm-objdump` がベクタ命令を表示しない。`llvm-mc` でバイト列をデコードする。
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
