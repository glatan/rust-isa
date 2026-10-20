# 開発環境

このリポジトリのコマンドは `Containerfile` のイメージの中で動かす。cargo、clang、llvm-ar、wasm-opt、wasm2wat、wasmtime、qemu-user-static はホストに置かない。

```sh
make image
make test
make test-wasip1
```

`make test` が既定である。イメージの中の `cargo test` が、rustdoc の言語と、`tests/driver.rs` に書いた wasm32 の確認を実行し、続けて `wasm32-wasip1` の値テストと doctest を実行する。値テストのランナーはイメージの `CARGO_TARGET_WASM32_WASIP1_RUNNER=wasmtime` である。`make test-wasip1` は `wasm32-wasip1` の値テストと doctest だけを実行する。

## arch ごとの確認

現在のテストは wasm32 とホスト向けである。native の他 arch は、実装を足すときに値テストと命令の確認を追加する。runner と必要なツールは `docs/<arch>/README.md` に書く。

イメージには `qemu-user-static` が入っている。次の runner を `Containerfile` で設定してある。Cargo は `cargo test --target <triple>` のときにこれを使う。

- `aarch64-unknown-linux-gnu`: `qemu-aarch64-static`
- `armv7-unknown-linux-gnueabihf`: `qemu-arm-static`
- `riscv64gc-unknown-linux-gnu`: `qemu-riscv64-static`
- `loongarch64-unknown-linux-gnu`: `qemu-loongarch64-static`
- `s390x-unknown-linux-gnu`: `qemu-s390x-static`
- `powerpc64le-unknown-linux-gnu`: `qemu-ppc64le-static`
- `mips64-unknown-linux-gnuabi64`: `qemu-mips64-static`
- `i686-unknown-linux-gnu`: `qemu-i386-static`

QEMU の runner は、静的にリンクしたテストバイナリを前提にする。動的リンクのバイナリは、ターゲットの loader と libc が無いため動かない。

- 値テスト: ホスト、QEMU、または実機で実行する。
- 命令の存在: `llvm-objdump` で確認する。
- ゼロコスト: Rust と C の LTO モードを一致させ、C の feature 集合を Rust の部分集合にする。手順と arch ごとの結果は [ゼロコストの確認](zero-cost.md) にある。
- ELF のリンクが通らない arch は、Rust と C の bitcode を `llvm-link` し、`opt -passes="default<O3>"` で最適化して IR を確認する。PowerPC はこの方法でもインラインを確認した。
- cfg: `rustc --print cfg` の結果を `docs/<arch>/feasibility.md` に追記する。

### QEMU での確認

調査では、`no_std` の静的バイナリを作り、SIMD の値テストを QEMU で実行した。Rust の `_start` から C の関数を呼び、結果を Rust で比較し、終了コードで判定した。

- AArch64、ARMv7、RISC-V、LoongArch、s390x、PowerPC、MIPS、i686: Rust から C を呼ぶ値テストが通った。
- MIPS は、共有の静的配列や lane を引数で選ぶ関数を使うと、codegen の都合で一致しなかった。lane ごとに独立した関数（`c_lane0` など）にすると通った。

終了コードのインライン asm は、レジスタを明示する。`in(reg)` と `li` を組み合わせると、宛先レジスタとオペランドが同じになって値が壊れることがある（PowerPC で発生した）。

この値テストはリポジトリのテストにはまだ入っていない。native の実装を足すときに、`tests/` へ追加する。設計は [native の値テスト](native-testing.md) にある。

`make` は次の名前付きボリュームを使う。ホストの rustc と、このイメージのコンパイラで同じ `target/` を共有すると、中間成果物が混ざる。

- `rust-isa-target` を `/src/target` にマウントする
- `rust-isa-cargo-registry` を `/root/.cargo/registry` にマウントする
- `rust-isa-cargo-git` を `/root/.cargo/git` にマウントする

## `target-nested`

`cargo test` は、ホスト向けのテストを `target/` に書きながら、別の `cargo` を起動する。起動される側の成果物は、リポジトリ直下の `target-nested` に置かれる。`CARGO_TARGET_DIR` の設定は `tests/driver.rs` にある。

外側の `cargo test` は `target/.cargo-lock` を保持する。内側が同じ `target/` へ書くと、このロックが外れるまで待つ。成果物のディレクトリを分けるのは、この待ちを避けるためである。

親の `cargo` は子プロセスへ `CARGO_ENCODED_RUSTFLAGS` を渡す。この変数があると `.cargo/config.toml` の rustflags は使われない。内側は起動前にこの変数と `RUSTFLAGS` を外す。

`target-nested/no-simd` のビルドだけは、外したあとに空の `RUSTFLAGS` を付ける。空の `RUSTFLAGS` は config の rustflags を置き換えるので、`simd128` も `relaxed-simd` も付かない。

現在のテストが使うディレクトリは四つある。`target-nested/wide-link` は過去の `wide-arithmetic` 実験の残骸で、現在のテストは使わない。

- `target-nested/link` は `fixtures/simd128-link` の `wasm32-unknown-unknown` release ビルドである。確認する内容は [wasm32 のリンク](wasm32/linking.md) に書く。
- `target-nested/no-simd` は、`simd128` も `relaxed-simd` も付けない wasm32 ビルドと、その状態で `fixtures/simd128-link` がコンパイルに失敗することの確認である。
- `target-nested/wasip1` は `cargo test --target wasm32-wasip1` の値テストと doctest である。
- `target-nested/sse2-link` は `fixtures/sse2-link` の release ビルドである。`kernel` に `call` が無く、`paddd` が 2 つ残ることを確認する。

`target-nested` は `.gitignore` と `.dockerignore` に入っている。消しても次の `cargo test` が作り直す。

名前付きボリュームが覆うのは `/src/target` だけである。`target-nested` はソースをマウントしたディレクトリに作られる。
