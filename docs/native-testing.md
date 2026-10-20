# native の値テスト

この文書は、native arch の値テストを `tests/` に追加するときの設計である。i686 の SSE2 / AVX2 値テストを `fixtures/native-values` で実行している。

## 前提

- 開発イメージには `qemu-user-static` と、arch ごとの runner が入っている。
- QEMU の runner は、静的にリンクしたテストバイナリを前提にする。
- テストは `no_std` のバイナリとしてビルドし、終了コードで合否を返す。
- 2026-10-20 の調査では、Rust の `_start` から C を呼ぶ形で、AArch64、ARMv7、RISC-V、LoongArch、s390x、PowerPC、MIPS、i686 の値テストが通った。

## 置き場所

```text
fixtures/native-values/
  Cargo.toml
  build.rs
  c/test.c
  src/main.rs
```

`fixtures/native-values` は workspace のメンバーにしない。`tests/driver.rs` から `cargo build --target <triple>` でビルドし、QEMU で実行する。wasm の `fixtures/simd128-link` と同じ形にする。

## テストの形

- `build.rs` が `TARGET` を見て C のフラグと clang の triple を選ぶ。
- `src/main.rs` は `_start` で公開 API を呼び、結果を比較し、終了コードを返す。
- 終了コードは、`0` が成功、`i + 1` が lane `i` の不一致とする。
- Rust と C の LTO モードを一致させる。zero-cost の確認は別途、disassembly で行う。

arch 固有の注意:

- **MIPS**: 共有の静的配列や lane を引数で選ぶ関数は codegen で一致しなかった。lane ごとに独立した関数を使う。
- **PowerPC**: `_start` から C を呼ぶ場合、`.quad .TOC.` を持つ非 LTO オブジェクトをリンクに加える。
- **i686**: C に `-mstackrealign` を付ける。
- **s390x**: `llvm-objdump` がベクタ命令を表示しない。`llvm-mc` でバイト列をデコードする。
- **静的リンク**: 動的リンクのバイナリは、ターゲットの loader と libc が無いため QEMU で動かない。

## 追加するテスト

native の API を実装したら、extension ごとに次を追加する。

1. 値を返す演算（add、mul、compare など）
2. メモリを読む命令（load、store）
3. lane 即値（extract、replace）
4. 値テストが通ること
5. 必要なら zero-cost の disassembly 確認

`tests/driver.rs` には、`cargo build`、QEMU の実行、終了コードの確認を追加する。失敗時は build log と終了コードを表示する。

## 現在の状態

- `fixtures/native-values` は `no_std` の `_start` を持ち、終了コードで合否を返す。
- i686 は `cargo build -Z build-std=core --target i686-unknown-linux-gnu` でビルドし、QEMU-i386 で SSE2 と AVX2 の値テストを実行する。
- ほかの arch は API の実装後に同じ形で追加する。
