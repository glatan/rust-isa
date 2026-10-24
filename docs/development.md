# 開発環境

このリポジトリのコマンドは `Containerfile` のイメージの中で動かす。cargo、cargo-nextest、clang、llvm-ar、wasm-opt、wasm2wat、wasmtime、qemu-user-static はホストに置かない。

```sh
make image
make fmt
make fmt-check
make test
make check-docs
make test-host
make test-x86_64
make test-check-static
make test-check-wasm-link
make test-wasip1
make test-wasip1-wide-arithmetic
make coverage
make coverage-check
make check-mnemonics
make duplicates
```

`make test` が既定で、`fmt-check`、`coverage-check`、`check-mnemonics`、`check-docs`、`test-host`、`test-wasip1`、`test-wasip1-wide-arithmetic` を実行する。すべてのテストはイメージ内で、対象 triple を明示して実行する。

- `make check-docs`: 全ターゲットの rustdoc を `-D warnings` で検査する。クロスターゲットは `-Z build-std=core` で core をビルドし、wasm32 は `simd128` / `relaxed-simd` と `wide-arithmetic` を有効にして検査する
- `make test-host`: `x86_64-unknown-linux-gnu` で実行する全テストと doctest。新しい統合テストも自動的に対象になる
- `make test-x86_64`: x86_64 の値テスト、doctest、x86 のゼロコスト検査だけを実行する
- `make test-check-static`: シンボル一致、rustdoc の言語、レーン即値の検査
- `make test-check-wasm-link`: `wasm32-unknown-unknown` のリンクと simd128 無効時の検査
- `make test-wasip1`: `wasm32-wasip1` の nextest と doctest
- `make test-wasip1-wide-arithmetic`: `wide-arithmetic` feature を有効にした wasm32-wasip1 の nextest、doctest、リンク検査
- `make check-mnemonics`: rustdoc の命令名を `llvm-mc` で検証する。全 native arch が対象。単一の命令名ではない名前は `tools/mnemonics.toml` の `[allow.<arch>]` に、既知の未修正名は `[baseline.<arch>]` に理由付きで書く。baseline は負債で、新規の無効名は失敗する

arch ごとの `test-i686`、`test-aarch64`、`test-armv7`、`test-riscv64`、`test-loongarch64`、`test-s390x`、`test-powerpc64le`、`test-mips64` は、対応する `tests/<arch>.rs` のテストだけを実行する。`make test` は `test-host` 経由でこれらも実行する。

テストの実行には `cargo nextest run` を使い、`--no-tests=fail` を付ける。テストターゲットにテストが 0 件の場合は失敗する。doctest は nextest の対象外なので、`cargo test --doc` で別に実行する。

## 整形

整形はイメージ内の dprint と cargo fmt で行う。dprint の設定はリポジトリ直下の `dprint.json` にあり、プラグインのバージョンも固定している。イメージをビルドすると
プラグインをキャッシュするため、整形時にネットワークへ接続しない。

```sh
make fmt
make fmt-check
```

`make fmt` は次を実行する。

- `dprint fmt`: Markdown、JSON、TOML、Dockerfile
- `cargo fmt --all`: ルートワークスペースの Rust
- `fixtures/*-values` の `cargo fmt`

Rust は dprint ではなく rustfmt を使う。`make test` は最初に `fmt-check` を実行する。

## arch ごとの確認

すべての値テストはイメージ内で実行する。x86_64 は `cargo test --target x86_64-unknown-linux-gnu`、wasm32 は wasmtime、その他の native arch は QEMU の runner を使う。runner と必要なツールは `docs/<arch>/README.md` に書く。

イメージには `qemu-user-static` が入っている。次の runner を `Containerfile` で設定してある。`tests/<arch>.rs` は、native arch の値を QEMU で実行するときにこれらを使う。

- `aarch64-unknown-linux-gnu`: `qemu-aarch64-static`
- `armv7-unknown-linux-gnueabihf`: `qemu-arm-static`
- `riscv64gc-unknown-linux-gnu`: `qemu-riscv64-static`
- `loongarch64-unknown-linux-gnu`: `qemu-loongarch64-static`
- `s390x-unknown-linux-gnu`: `qemu-s390x-static`
- `powerpc64le-unknown-linux-gnu`: `qemu-ppc64le-static`
- `mips64-unknown-linux-gnuabi64`: `qemu-mips64-static`
- `i686-unknown-linux-gnu`: `qemu-i386-static`

QEMU の runner は、静的にリンクしたテストバイナリを前提にする。動的リンクのバイナリは、ターゲットの loader と libc が無いため動かない。

- 値テスト: イメージ内で、対象 triple ごとの runner で実行する。
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

`cargo test` は、x86_64 向けのテストを `target/` に書きながら、別の `cargo` を起動する。起動される側の成果物は、リポジトリ直下の `target-nested` に置かれる。`CARGO_TARGET_DIR` の設定は `tests/common/mod.rs` にある。

外側の `cargo test` は `target/.cargo-lock` を保持する。内側が同じ `target/` へ書くと、このロックが外れるまで待つ。成果物のディレクトリを分けるのは、この待ちを避けるためである。

親の `cargo` は子プロセスへ `CARGO_ENCODED_RUSTFLAGS` を渡す。この変数があると `.cargo/config.toml` の rustflags は使われない。内側は起動前にこの変数と `RUSTFLAGS` を外す。

`target-nested/no-simd` のビルドだけは、外したあとに空の `RUSTFLAGS` を付ける。空の `RUSTFLAGS` は config の rustflags を置き換えるので、`simd128` も `relaxed-simd` も付かない。

現在のテストが使うディレクトリは十五ある。

- `target-nested/link` は `fixtures/wasm-simd-link` の `wasm32-unknown-unknown` release ビルドである。確認する内容は [wasm32 のリンク](wasm32/linking.md) に書く。
- `target-nested/no-simd` は、`simd128` も `relaxed-simd` も付けない wasm32 ビルドと、その状態で `fixtures/wasm-simd-link` がコンパイルに失敗することの確認である。
- `target-nested/sse2-link` は `fixtures/sse2-link` の release ビルドである。`kernel` に `call` が無く、`paddd` が 2 つ残ることを確認する。
- `target-nested/avx2-link` は `fixtures/avx2-link` の release ビルドである。`kernel` に `call` が無く、`vpaddd` の直後に `vpmulld` が残ることを確認する。
- `target-nested/avx512f-link` は `fixtures/avx512f-link` の release ビルドである。`kernel` に `call` が無く、`vpaddd` が 2 つ隣接することを確認する。
- `target-nested/aes-link` は `fixtures/aes-link` の release ビルドである。`target-nested/fma-link` は `fixtures/fma-link` の release ビルドである。それぞれの kernel に `call` が無く、`aes-link` は `aesenc`、`fma-link` は `vfmadd` が隣接することを確認する。
- `target-nested/i686-values` は `fixtures/i686-values` の i686 release ビルドである。QEMU-i386 で SSE2 と AVX2 の値テストを実行する。
- `target-nested/aarch64-values` は `fixtures/aarch64-values` の AArch64 release ビルドである。QEMU-AArch64 で NEON と dotprod の値テストを実行し、`add` / `tbl` / `sdot` の disassembly を確認する。
- `target-nested/riscv-values` は RISC-V の固定長 V128 値テストである。
- `target-nested/loongarch-values` は LoongArch LSX の値テストである。
- `target-nested/s390x-values` は s390x vector の値テストである。
- `target-nested/arm-values` は ARMv7 NEON の値テストである。
- `target-nested/powerpc-values` は PowerPC Altivec の値テストである。
- `target-nested/mips-values` は MIPS MSA の値テストである。
- `target-nested/wide-arithmetic-link` は wasm32 の `wide-arithmetic` リンク検査に使うビルドである。

`target-nested` は `.gitignore` と `.dockerignore` に入っている。消しても次の `cargo test` が作り直す。

名前付きボリュームが覆うのは `/src/target` だけである。`target-nested` はソースをマウントしたディレクトリに作られる。
