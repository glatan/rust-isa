# 開発コマンドはイメージの中で動かす。ホストに cargo や clang は置かない。
# テストはすべてイメージ内で、対象 triple を明示して実行する。

# Containerfile のベースイメージ日付とイメージ改訂番号に合わせる。
IMAGE_TAG ?= 20261018.5
IMAGE ?= localhost/rust-isa-dev:$(IMAGE_TAG)
PODMAN ?= podman
NEXTEST ?= cargo nextest run
NEXTEST_FLAGS ?= --no-tests=fail
COVERAGE_ARGS ?= --todo
DUPLICATES_ARGS ?=

X86_64_TARGET ?= x86_64-unknown-linux-gnu
WASIP1_TARGET ?= wasm32-wasip1
WASM_UNKNOWN_TARGET ?= wasm32-unknown-unknown
DOC_TARGETS ?= aarch64-unknown-linux-gnu armv7-unknown-linux-gnueabihf powerpc64le-unknown-linux-gnu riscv64gc-unknown-linux-gnu s390x-unknown-linux-gnu loongarch64-unknown-linux-gnu mips64-unknown-linux-gnuabi64
DOC_TARGET_DIR ?= target-nested/doc
DOC_TEST_TARGET_DIR ?= target-nested/doc-test
WASM_DOC_TARGET_DIR ?= target-nested/doc-wasm

SDE64 ?= /opt/sde/sde64
SDE32 ?= /opt/sde/sde
SDE_FLAGS ?= -future
SDE_SKIP ?= --skip sm4_instruction_reference --skip sm4_key_schedule_and_rounds --skip avx_vnni
SDE_SKIP_ARGS := $(if $(SDE_SKIP),-- $(SDE_SKIP),)

PODMAN_RUN_BASE := $(PODMAN) run --rm \
	-v "$(CURDIR)":/src \
	-v rust-isa-target:/src/target \
	-v rust-isa-cargo-registry:/root/.cargo/registry \
	-v rust-isa-cargo-git:/root/.cargo/git \
	-w /src
PODMAN_SDE_ENV := -e CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER='$(SDE64) $(SDE_FLAGS) --' \
	-e CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_RUNNER='$(SDE32) $(SDE_FLAGS) --' \
	-e RUST_ISA_SDE32='$(SDE32)' \
	-e RUST_ISA_SDE_FLAGS='$(SDE_FLAGS)'
PODMAN_I686_ENV := -e RUST_ISA_SDE32='$(SDE32)' -e RUST_ISA_SDE_FLAGS='$(SDE_FLAGS)'
PODMAN_RUN := $(PODMAN_RUN_BASE) $(IMAGE)
PODMAN_RUN_SDE := $(PODMAN_RUN_BASE) $(PODMAN_SDE_ENV) $(IMAGE)
PODMAN_RUN_I686 := $(PODMAN_RUN_BASE) $(PODMAN_I686_ENV) $(IMAGE)

.DEFAULT_GOAL := test

.PHONY: help
help:
	@printf '%s\n' \
		'image                        開発イメージを作る' \
		'test                         フォーマット検査・rustdoc・全 arch のテスト（QEMU / SDE / wasmtime）' \
		'check-docs                   全ターゲットの rustdoc を -D warnings で検査する' \
		'fmt                          dprint と cargo fmt で整形する' \
		'fmt-check                    dprint と cargo fmt の整形を検査する' \
		'test-host                    x86_64 を SDE で実行するテストと cross-arch テスト' \
		'test-x86_64                  x86_64 の SDE 値テスト・doctest・ゼロコスト検査' \
		'test-check-static            シンボル・rustdoc・レーン即値の検査' \
		'test-check-wasm-link         wasm32-unknown-unknown のリンク検査' \
		'test-i686                    i686 の値テスト（SDE）' \
		'test-aarch64                 AArch64 の値テスト' \
		'test-armv7                   ARMv7 の値テスト' \
		'test-riscv64                 RISC-V の値テスト' \
		'test-loongarch64             LoongArch の値テスト' \
		'test-s390x                   s390x の値テスト' \
		'test-powerpc64le             PowerPC の値テスト' \
		'test-mips64                  MIPS64 の値テスト' \
		'test-doc                     全 arch の doctest（QEMU / wasmtime）' \
		'test-cross                   i686 と cross-arch の値テスト' \
		'test-wasip1                  wasm32-wasip1 の nextest と doctest' \
		'test-wasip1-wide-arithmetic  wide-arithmetic 有効時の nextest と doctest' \
		'coverage                     arch/extension ごとの実装状況を表示する' \
		'coverage-check               README とゼロコスト結果表の形式を検査する' \
		'zero-cost-check              ゼロコスト結果表の fixture と kernel を検査する' \
		'docs-check                   文書間リンク・アンカー・索引からの到達性を検査する' \
		'fixtures-check               fixtures/・target-nested と fixture-matrix.md の対応を検査する' \
		'duplicates                   同じ arch で重複する命令名を表示する'

.PHONY: image
image:
	$(PODMAN) build -t $(IMAGE) -f Containerfile .

.PHONY: coverage
coverage:
	$(PODMAN_RUN) python3 tools/coverage.py $(COVERAGE_ARGS)

.PHONY: zero-cost-check
zero-cost-check:
	$(PODMAN_RUN) python3 tools/zero_cost.py --check

.PHONY: coverage-check
coverage-check: zero-cost-check
	$(PODMAN_RUN) python3 tools/coverage.py --check
	$(PODMAN_RUN) python3 tools/doc_names.py --check

.PHONY: docs-check
docs-check:
	$(PODMAN_RUN) python3 tools/doc_links.py --check

.PHONY: fixtures-check
fixtures-check:
	$(PODMAN_RUN) python3 tools/fixtures.py --check

.PHONY: check-mnemonics
check-mnemonics:
	$(PODMAN_RUN) python3 tools/mnemonics.py

.PHONY: duplicates
duplicates:
	$(PODMAN_RUN) python3 tools/duplicates.py $(DUPLICATES_ARGS)

.PHONY: fmt
fmt:
	$(PODMAN_RUN) bash -lc 'dprint fmt && cargo fmt --all && for d in fixtures/*-values; do cargo fmt --manifest-path "$$d/Cargo.toml" || exit 1; done'

.PHONY: fmt-check
fmt-check:
	$(PODMAN_RUN) bash -lc 'dprint check && cargo fmt --all -- --check && for d in fixtures/*-values; do cargo fmt --manifest-path "$$d/Cargo.toml" -- --check || exit 1; done'

.PHONY: check-docs
check-docs:
	$(PODMAN_RUN) bash -lc 'set -e; \
		RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --target $(X86_64_TARGET); \
		for t in $(DOC_TARGETS); do \
			echo "===== doc $$t"; \
			RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-D warnings" CARGO_TARGET_DIR=$(DOC_TARGET_DIR) cargo doc --no-deps -Z build-std=core --target "$$t"; \
		done; \
		CARGO_TARGET_DIR=$(WASM_DOC_TARGET_DIR) RUSTDOCFLAGS="-D warnings -C target-feature=+simd128,+relaxed-simd" cargo doc --no-deps --features wide-arithmetic --target $(WASM_UNKNOWN_TARGET)'

.PHONY: test
test: fmt-check coverage-check docs-check fixtures-check check-mnemonics check-docs test-host test-doc test-wasip1 test-wasip1-wide-arithmetic

.PHONY: test-host
test-host: test-x86_64 test-cross

.PHONY: test-cross
test-cross: test-i686 test-aarch64 test-armv7 test-riscv64 test-loongarch64 test-s390x test-powerpc64le test-mips64

.PHONY: test-x86_64
test-x86_64:
	$(PODMAN_RUN_SDE) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test x86 $(CARGO_ARGS) $(SDE_SKIP_ARGS)
	$(PODMAN_RUN_SDE) cargo test --target $(X86_64_TARGET) --doc $(CARGO_ARGS)
	$(PODMAN_RUN_SDE) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test x86_link $(CARGO_ARGS)
	$(PODMAN_RUN_SDE) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test consistency $(CARGO_ARGS)

.PHONY: test-check-static
test-check-static:
	$(PODMAN_RUN_SDE) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test consistency $(CARGO_ARGS)

.PHONY: test-check-wasm-link
test-check-wasm-link:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test wasm_link $(CARGO_ARGS)

.PHONY: test-i686
test-i686:
	$(PODMAN_RUN_I686) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test i686 $(CARGO_ARGS)

.PHONY: test-aarch64
test-aarch64:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test aarch64 $(CARGO_ARGS)

.PHONY: test-armv7
test-armv7:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test arm $(CARGO_ARGS)

.PHONY: test-riscv64
test-riscv64:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test riscv $(CARGO_ARGS)

.PHONY: test-loongarch64
test-loongarch64:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test loongarch $(CARGO_ARGS)

.PHONY: test-s390x
test-s390x:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test s390x $(CARGO_ARGS)

.PHONY: test-powerpc64le
test-powerpc64le:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test powerpc $(CARGO_ARGS)

.PHONY: test-mips64
test-mips64:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test mips $(CARGO_ARGS)

.PHONY: test-doc
test-doc:
	$(PODMAN_RUN) bash -lc 'set -e; \
		for t in $(DOC_TARGETS); do \
			case "$$t" in \
				aarch64-*) r=qemu-aarch64-static ;; \
				armv7-*) r=qemu-arm-static ;; \
				powerpc64le-*) r=qemu-ppc64le-static ;; \
				riscv64gc-*) r=qemu-riscv64-static ;; \
				s390x-*) r=qemu-s390x-static ;; \
				loongarch64-*) r=qemu-loongarch64-static ;; \
				mips64-*) r=qemu-mips64-static ;; \
			esac; \
			key=$$(echo "$$t" | tr "a-z-" "A-Z_"); \
			echo "===== doctest $$t"; \
			RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR=$(DOC_TEST_TARGET_DIR) \
				env "CARGO_TARGET_$${key}_RUNNER=$$r" \
				cargo test --doc --target "$$t" -Z build-std=std; \
		done'

.PHONY: test-wasip1
test-wasip1:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(WASIP1_TARGET) $(CARGO_ARGS)
	$(PODMAN_RUN) cargo test --target $(WASIP1_TARGET) --doc $(CARGO_ARGS)

.PHONY: test-wasip1-wide-arithmetic
test-wasip1-wide-arithmetic:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --features wide-arithmetic --target $(WASIP1_TARGET) $(CARGO_ARGS)
	$(PODMAN_RUN) cargo test --features wide-arithmetic --target $(WASIP1_TARGET) --doc $(CARGO_ARGS)
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test wide_arithmetic_link $(CARGO_ARGS)
