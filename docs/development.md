# 開発環境

このリポジトリのコマンドは `Containerfile` の Arch イメージの中で動かす。native doctest の実行を含む全テストは、cross GNU toolchain を入れた `Containerfile.debian` の Debian イメージで動かす。cargo、cargo-nextest、clang、llvm-ar、wasm-opt、wasm2wat、wasmtime、qemu-user-static はホストに置かない。

```sh
# イメージ
make image
make image-debian

# 整形
make fmt
make fmt-check

# テスト
make check
make test
make test-debian
make test-host
make test-x86_64
make test-cross
make test-doc
make test-doc-debian
make test-check-static
make test-check-wasm-link
make test-wasip1
make test-wasip1-wide-arithmetic

# 文書と在庫
make docs
make docs-rustdoc-check
make docs-serve
make check-docs
make inventory
make inventory-check
make manifest-pilot-check
make doctest-coverage
make doctest-inventory-check
make zero-cost-check
make docs-check

# fixtureと重複
make fixtures-check
make check-mnemonics
make duplicates
```

`make test` が既定で、`fmt-check`、`inventory-check`、`manifest-pilot-check`、`docs-check`、`docs-rustdoc-check`、`fixtures-check`、`check-mnemonics`、`check-docs`、`test-check-wasm-link`、`test-host`、`test-doc`、`test-wasip1`、`test-wasip1-wide-arithmetic` を実行する。すべてのテストはイメージ内で、対象 triple を明示して実行する。各テストの役割は [テストの役割](testing.md) にある。

`make check` は SDE・QEMU を使わない静的検査で、`fmt-check`、`inventory-check`、`manifest-pilot-check`、`docs-check`、`fixtures-check`、`check-mnemonics`、`test-check-static`、`test-check-wasm-link` を実行する。反復中は `make check` と変更範囲の `test-<arch>` を使い、最後に `make test` を通す。`check-docs` と `docs-rustdoc-check` の rustdoc ビルドは `make check` に含まない。

`make test-debian` は Debian イメージで `make test` を実行し、native doctest を `DOC_TEST=run` で実行する。既定の Arch イメージでは、native doctest は `--no-run` のコンパイル確認だけを行う。

- `make docs-check`: Markdown の相対リンク、見出しアンカー、`docs/README.md` からの到達性、`scalable-mapping.md` のAPI名の存在、文書中の`make`ターゲット名を検査する
- `make fixtures-check`: `fixtures/` と `target-nested` の実ディレクトリが `fixture-matrix.md` と対応するか検査する
- `make check-docs`: 全ターゲットの rustdoc を `-D warnings` で検査する。クロスターゲットは `-Z build-std=core` で core をビルドし、wasm32 は `simd128` / `relaxed-simd` と `wide-arithmetic` を有効にして検査する
- `make test-host`: x86_64 のテストを SDE で実行し、cross-arch の値テストも実行する。`test-x86_64` と `test-cross` の合成である
- `make test-x86_64`: x86_64 の値テストと doctest を SDE で実行し、x86 のゼロコスト検査と静的検査はホストで実行する。SDE の既知不一致は skip する
- `make test-cross`: i686 を SDE、その他の native arch を QEMU で実行する
- `make test-doc`: x86_64 以外の native target の doctest をコンパイルする。`-Z build-std=std` と `RUSTDOCFLAGS=--no-run` を使う。native target は i686 を含む 8 件である。Arch イメージではコンパイルのみを確認し、`DOC_TEST=run` を付けた Debian イメージで cross GCC と QEMU により実行する。x86_64 と wasm32 の doctest は `test-x86_64` と `test-wasip1` が実行する
- `make test-doc-debian`: Debian イメージで `DOC_TEST=run` の doctest を実行する
- `make test-i686`: i686 fixture を SDE で実行する。テストドライバは x86_64 で動かし、ターゲットコードは SDE に限定する
- `make doctest-coverage`: instruction inventory と網羅表を検査し、arch ごとの件数を表示する
- `make manifest-pilot-check` は ACLE の SVE/SVE2 instruction inventory（824 命令）が `sve.toml` / `sve2.toml` にすべて収録されていることも検査する
- `make doctest-inventory-check`: extension ごとの instruction inventory doctest が最新かを検査する
- `make test-check-static`: シンボル一致、rustdoc の言語、レーン即値の検査
- `make test-check-wasm-link`: `wasm32-unknown-unknown` のリンクと simd128 無効時の検査。`make test` に含まれる
- `make test-wasip1`: `wasm32-wasip1` の nextest と doctest
- `make test-wasip1-wide-arithmetic`: `wide-arithmetic` feature を有効にした wasm32-wasip1 の nextest、doctest、リンク検査
- `make check-mnemonics`: rustdoc の命令名を `llvm-mc` で検証する。全 native arch が対象。単一の命令名ではない名前は `tools/mnemonics.toml` の `[allow.<arch>]` に、既知の未修正名は `[baseline.<arch>]` に理由付きで書く。baseline は負債で、新規の無効名は失敗する

arch ごとの `test-i686`、`test-aarch64`、`test-armv7`、`test-riscv64`、`test-loongarch64`、`test-s390x`、`test-powerpc64le`、`test-mips64` は、対応する `tests/<arch>.rs` のテストだけを実行する。値テストに加え、ARMv7 を除く fixture を持つ arch は同じバイナリのゼロコストを disassembly で検査する。`make test` は `test-host` 経由でこれらも実行する。

テストの実行には `cargo nextest run` を使い、`--no-tests=fail` を付ける。テストターゲットにテストが 0 件の場合は失敗する。doctest は nextest の対象外なので、`cargo test --doc` で別に実行する。native arch の doctest は、ターゲット用の libc を用意せずに `RUSTDOCFLAGS=--no-run` でコンパイルだけを確認する。

## コードと文書の整形

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

## arch ごとの実行確認

すべての値テストはイメージ内で実行する。x86_64 と i686 は SDE、wasm32 は wasmtime、その他の native arch は QEMU の runner を使う。runner の実行ファイルと引数は `tests/<arch>.rs` を正とし、arch 固有の条件は `docs/<arch>/README.md` に書く。

SDE はイメージ内の `/opt/sde` に固定してある。ホストからマウントしない。32-bit SDE のため、Arch イメージには `lib32-glibc` と `lib32-gcc-libs`、Debian イメージには `libc6:i386` も入れてある。

SDE を使うのは、x86 命令を実際に実行するテストだけである。`tests/x86_link` と `tests/consistency` はビルドとファイル検査だけなので、SDE の外で実行する。SDE の下で nested `cargo build` を動かすと、cargo と rustc までエミュレーションされ、cold build が数分単位になるためである。

SDE と QEMU の実行ファイル、CPU、引数は `tests/<arch>.rs` に書く。値テストの fixture と triple の対応は [値テストの fixture](fixture-matrix.md) にある。

ターゲットコードをホストで直接実行しない。テストドライバ（`tests/<arch>.rs` の x86_64 バイナリ）はホストで動かすが、値テストの対象バイナリは SDE / QEMU で実行する。

SDE の SM4 と AVX-VNNI は既知の不一致がある。`make test-x86_64` は `sm4_instruction_reference`、`sm4_key_schedule_and_rounds`、`avx_vnni` を skip する。SDE を更新したら skip を外して再確認する。

QEMU の runner は、値テストでは静的にリンクしたテストバイナリを前提にする。Debian イメージの native doctest は、cross glibc を `-L` で参照する動的リンクのバイナリを実行する。

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

Debian イメージでは、Arch イメージの成果物と混ざらないように次のボリュームを使う。

- `rust-isa-target-debian`
- `rust-isa-cargo-registry-debian`
- `rust-isa-cargo-git-debian`

`target-nested` もイメージごとに分ける。Debian の実行では `RUST_ISA_NESTED=/src/target-nested-debian` を設定し、`tests/common` と `DOC_TEST_TARGET_DIR` などがこのディレクトリを使う。

## AMD 実機

x86 の任意 feature の値テストは、CPU が feature を持たない場合に早期 return する。既定の `make test-x86_64` は SDE で実行するため、AMD 専用の `sse4a`、`fma4`、`xop` はスキップされる。AMD 実機では `make test-amd` を使う。SDE を介さずホスト CPU の機能検出で実行するため、これらの extension も対象になる。Intel 実機では同じテストが feature 検出でスキップされる。

## Debian テストイメージ

`Containerfile.debian` は、native arch の doctest を実行するための Debian 13（trixie）イメージである。ベースイメージの digest、SDE、直接ダウンロードするツールは SHA-256 で固定する。Debian の apt パッケージ、apt.llvm.org、rustup はバージョン指定のみで、完全な再現性は保証しない。Arch イメージと同じテストを実行できるように、次のツールを入れてある。

- i686、AArch64、ARMv7、PowerPC64LE、s390x、MIPS64、RISC-V、LoongArch の cross GCC と glibc
- LLVM 23 の clang、lld、`llvm-objdump`、`llvm-mc` など
- Rust 1.99.1、`rust-src`、`wasm32-wasip1`、`wasm32-unknown-unknown`
- cargo-nextest、wasmtime、wasm-opt 130、wabt 1.0.42
- QEMU user-mode と Intel SDE

```sh
make image-debian
make test-debian
make test-doc-debian
```

`make test-debian` は Debian イメージで `make test` を実行する。このとき `DOC_TEST=run` が選ばれ、native doctest は cross GCC でリンクされ、QEMU で実行される。`make test-doc-debian` は native doctest だけを実行する。i686 は QEMU の i386 runner で実行する。

native doctest のビルドでは `RUST_ISA_DOCTEST=1` を設定し、C を ThinLTO bitcode ではなく ELF オブジェクトとしてコンパイルする。cross GCC のリンカは clang の ThinLTO bitcode を読めないためである。ゼロコストの確認は、従来どおり GNU target の値テストと disassembly で行う。

## `target-nested`

`cargo test` は、x86_64 向けのテストを `target/` に書きながら、別の `cargo` を起動する。起動される側の成果物は、既定ではリポジトリ直下の `target-nested` に置かれる。`CARGO_TARGET_DIR` と `RUST_ISA_NESTED` の設定は `tests/common/mod.rs` にある。Debian イメージでは `target-nested-debian` を使う。

外側の `cargo test` は `target/.cargo-lock` を保持する。内側が同じ `target/` へ書くと、このロックが外れるまで待つ。成果物のディレクトリを分けるのは、この待ちを避けるためである。

親の `cargo` は子プロセスへ `CARGO_ENCODED_RUSTFLAGS` を渡す。この変数があると `.cargo/config.toml` の rustflags は使われない。内側は起動前にこの変数と `RUSTFLAGS` を外す。

`target-nested/no-simd` のビルドだけは、外したあとに空の `RUSTFLAGS` を付ける。空の `RUSTFLAGS` は config の rustflags を置き換えるので、`simd128` も `relaxed-simd` も付かない。

`target-nested` に作られる値テストのビルドの一覧と確認内容は、[値テストの fixture](fixture-matrix.md) の「`target-nested` のビルド」に置く。

`make test-doc` の成果物は `target-nested/doc-test` に置く。fixture ではないため `fixture-matrix.md` には載せない。

命令を追加・削除したら `python3 tools/doctest_inventory.py --write` で extension の instruction inventory を更新する。`# Instruction examples` にも新しい命令の呼び出しを追加する。件数が変わった場合は `docs/doctest-coverage.md` の表も `make doctest-coverage` の出力に合わせて更新する。`make inventory-check` は inventory の更新漏れ、instruction examples の欠落、arch の README に載る命令の欠落、網羅表の不一致を失敗にする。

`target-nested` は `.gitignore` と `.dockerignore` に入っている。消しても次の `cargo test` が作り直す。

名前付きボリュームが覆うのは `/src/target` だけである。`target-nested` はソースをマウントしたディレクトリに作られる。
