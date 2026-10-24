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
make test-cross
make test-doc
make test-check-static
make test-check-wasm-link
make test-wasip1
make test-wasip1-wide-arithmetic
make coverage
make coverage-check
make zero-cost-check
make docs-check
make fixtures-check
make check-mnemonics
make duplicates
```

`make test` が既定で、`fmt-check`、`coverage-check`、`docs-check`、`fixtures-check`、`check-mnemonics`、`check-docs`、`test-host`、`test-doc`、`test-wasip1`、`test-wasip1-wide-arithmetic` を実行する。すべてのテストはイメージ内で、対象 triple を明示して実行する。

- `make docs-check`: Markdown の相対リンク、見出しアンカー、`docs/README.md` からの到達性を検査する
- `make fixtures-check`: `fixtures/` と `target-nested` の実ディレクトリが `fixture-matrix.md` と対応するか検査する
- `make check-docs`: 全ターゲットの rustdoc を `-D warnings` で検査する。クロスターゲットは `-Z build-std=core` で core をビルドし、wasm32 は `simd128` / `relaxed-simd` と `wide-arithmetic` を有効にして検査する
- `make test-host`: x86_64 のテストを SDE で実行し、cross-arch の値テストも実行する。`test-x86_64` と `test-cross` の合成である
- `make test-x86_64`: x86_64 の値テスト、doctest、x86 のゼロコスト検査、静的検査を SDE で実行する。SDE の既知不一致は skip する
- `make test-cross`: i686 を SDE、その他の native arch を QEMU で実行する
- `make test-doc`: x86_64 以外の arch の doctest を QEMU、wasm32 を wasmtime で実行する。クロスターゲットは `-Z build-std=std` を使う
- `make test-i686`: i686 fixture を SDE で実行する。テストドライバは x86_64 で動かし、ターゲットコードは SDE に限定する
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

すべての値テストはイメージ内で実行する。x86_64 と i686 は SDE、wasm32 は wasmtime、その他の native arch は QEMU の runner を使う。runner と必要なツールは `docs/<arch>/README.md` に書く。

SDE はイメージ内の `/opt/sde` に固定してある。ホストからマウントしない。32-bit SDE のため、Arch Linux の `lib32-glibc` と `lib32-gcc-libs` もイメージに入れてある。SDE のテスト runner は次のとおり。

- `x86_64-unknown-linux-gnu`: `/opt/sde/sde64 -future --`
- `i686-unknown-linux-gnu`: `/opt/sde/sde -future --`

QEMU の runner は次のとおり。`tests/<arch>.rs` は、native arch の値を QEMU で実行するときにこれらを使う。

- `aarch64-unknown-linux-gnu`: `qemu-aarch64-static -cpu max`
- `armv7-unknown-linux-gnueabihf`: NEON/FP16 は `qemu-arm-static -cpu max`、MVE/MVE-FP は `qemu-arm-static -cpu cortex-m55`
- `riscv64gc-unknown-linux-gnu`: `qemu-riscv64-static -cpu max`
- `loongarch64-unknown-linux-gnu`: `qemu-loongarch64-static -cpu max`
- `s390x-unknown-linux-gnu`: `qemu-s390x-static -cpu max`（VE3 は実行しない）
- `powerpc64le-unknown-linux-gnu`: `qemu-ppc64le-static -cpu max`
- `mips64-unknown-linux-gnuabi64`: `qemu-mips64-static -cpu max`

ターゲットコードをホストで直接実行しない。テストドライバ（`tests/<arch>.rs` の x86_64 バイナリ）はホストで動かすが、値テストの対象バイナリは SDE / QEMU で実行する。

SDE の SM4 と AVX-VNNI は既知の不一致がある。`make test-x86_64` は `sm4_instruction_reference`、`sm4_key_schedule_and_rounds`、`avx_vnni` を skip する。SDE を更新したら skip を外して再確認する。

QEMU の runner は、静的にリンクしたテストバイナリを前提にする。動的リンクのバイナリは、ターゲットの loader と libc が無いため動かない。

- 値テスト: イメージ内で、対象 triple ごとの runner で実行する。
- 命令の存在: `llvm-objdump` で確認する。
- ゼロコスト: Rust と C の LTO モードを一致させ、C の feature 集合を Rust の部分集合にする。手順と arch ごとの結果は [ゼロコストの確認](zero-cost.md) にある。
- ELF のリンクが通らない arch は、Rust と C の bitcode を `llvm-link` し、`opt -passes="default<O3>"` で最適化して IR を確認する。PowerPC はこの方法でもインラインを確認した。
- cfg: `rustc --print cfg` の結果を `docs/<arch>/feasibility.md` に追記する。

値テストの設計と arch 別の注意は [native の値テスト](native-testing.md) にある。

`make` は次の名前付きボリュームを使う。ホストの rustc と、このイメージのコンパイラで同じ `target/` を共有すると、中間成果物が混ざる。

- `rust-isa-target` を `/src/target` にマウントする
- `rust-isa-cargo-registry` を `/root/.cargo/registry` にマウントする
- `rust-isa-cargo-git` を `/root/.cargo/git` にマウントする

## `target-nested`

`cargo test` は、x86_64 向けのテストを `target/` に書きながら、別の `cargo` を起動する。起動される側の成果物は、リポジトリ直下の `target-nested` に置かれる。`CARGO_TARGET_DIR` の設定は `tests/common/mod.rs` にある。

外側の `cargo test` は `target/.cargo-lock` を保持する。内側が同じ `target/` へ書くと、このロックが外れるまで待つ。成果物のディレクトリを分けるのは、この待ちを避けるためである。

親の `cargo` は子プロセスへ `CARGO_ENCODED_RUSTFLAGS` を渡す。この変数があると `.cargo/config.toml` の rustflags は使われない。内側は起動前にこの変数と `RUSTFLAGS` を外す。

`target-nested/no-simd` のビルドだけは、外したあとに空の `RUSTFLAGS` を付ける。空の `RUSTFLAGS` は config の rustflags を置き換えるので、`simd128` も `relaxed-simd` も付かない。

`target-nested` に作られるビルドの一覧と確認内容は、[値テストの fixture](fixture-matrix.md) の「`target-nested` のビルド」に置く。

`target-nested` は `.gitignore` と `.dockerignore` に入っている。消しても次の `cargo test` が作り直す。

名前付きボリュームが覆うのは `/src/target` だけである。`target-nested` はソースをマウントしたディレクトリに作られる。
