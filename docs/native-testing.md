# native の値テスト

この文書は、native arch の値テストを `tests/` に追加するときの設計である。i686 の SSE2 / AVX2 値テストを `fixtures/i686-values` で実行している。

## 前提

- 開発イメージには `qemu-user-static` と、arch ごとの runner が入っている。
- QEMU の runner は、静的にリンクしたテストバイナリを前提にする。
- テストは `no_std` のバイナリとしてビルドし、終了コードで合否を返す。
- 2026-10-20 の調査では、Rust の `_start` から C を呼ぶ形で、AArch64、ARMv7、RISC-V、LoongArch、s390x、PowerPC、MIPS、i686 の値テストが通った。

## 置き場所

```text
fixtures/i686-values/
  Cargo.toml
  src/main.rs
```

`fixtures/i686-values` は workspace のメンバーにしない。`tests/<arch>.rs` から `cargo build --target <triple>` でビルドし、QEMU で実行する。wasm の `fixtures/wasm-simd-link` と同じ形にする。

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

`tests/<arch>.rs` には、`cargo build`、QEMU の実行、終了コードの確認を追加する。失敗時は build log と終了コードを表示する。

## 現在の状態

- `fixtures/i686-values` は `no_std` の `_start` を持ち、終了コードで合否を返す。
- i686 は `cargo build -Z build-std=core --target i686-unknown-linux-gnu` でビルドし、QEMU-i386 で SSE2 と AVX2 の値テストを実行する。
- AArch64 は同じ形でビルドし、QEMU-AArch64 で NEON と dotprod の値テストを実行する。
- RISC-V、LoongArch、s390x、ARMv7、PowerPC、MIPS も同じ形でビルドし、それぞれの QEMU で主要な SIMD 命令を実行する。
- wasm32 は `fixtures/wide-arithmetic-link` をビルドし、`wasm2wat` で `wide-arithmetic` の命令を確認する。
- 追加した extension の値テストは、NEON / fp16 / dotprod / i8mm / fcma / fhm / AES / SHA-2 / SHA-3 / SM4 / CRC / SVE を AArch64、`zba` から `zbs` / `zkn` / `zks` / Zfh / Zfa / V / スライス API を RISC-V、LSX / LASX を LoongArch、vector を s390x、 NEON / dotprod / fp16 / 暗号を ARM、Altivec / VSX / Power8 を PowerPC、 MSA を MIPS に追加した。
- QEMU が実行しない命令（AArch64 の RDM / BF16、AArch64 SME の `__arm_get_current_vg`）は、API のコンパイルだけを確認する。
