# 開発コマンドはイメージの中で動かす。ホストに cargo や clang は置かない。
# テストはすべてイメージ内で、対象 triple を明示して実行する。

# Arch イメージのタグは Containerfile のベースイメージ日付と改訂番号に合わせる。
IMAGE_TAG ?= 20261018.5
IMAGE ?= localhost/rust-isa-dev:$(IMAGE_TAG)
# Debian テストイメージのタグは Containerfile.debian の改訂番号に合わせる。
DEBIAN_IMAGE_TAG ?= 20261025.2
DEBIAN_IMAGE ?= localhost/rust-isa-debian:$(DEBIAN_IMAGE_TAG)
DEBIAN_CONTAINERFILE ?= Containerfile.debian
PODMAN ?= podman
NEXTEST ?= cargo nextest run
NEXTEST_FLAGS ?= --no-tests=fail
COVERAGE_ARGS ?= --todo
INVENTORY_ARGS ?= $(COVERAGE_ARGS)
DUPLICATES_ARGS ?=
TEST_GAPS_ARGS ?=
DOC_TEST ?= compile
TARGET_VOLUME ?= rust-isa-target
CARGO_REGISTRY_VOLUME ?= rust-isa-cargo-registry
CARGO_GIT_VOLUME ?= rust-isa-cargo-git

X86_64_TARGET ?= x86_64-unknown-linux-gnu
WASIP1_TARGET ?= wasm32-wasip1
WASM_UNKNOWN_TARGET ?= wasm32-unknown-unknown
DOC_TARGETS ?= i686-unknown-linux-gnu aarch64-unknown-linux-gnu armv7-unknown-linux-gnueabihf powerpc64le-unknown-linux-gnu riscv64gc-unknown-linux-gnu s390x-unknown-linux-gnu loongarch64-unknown-linux-gnu mips64-unknown-linux-gnuabi64
NESTED_DIR ?= target-nested
DOC_TARGET_DIR ?= $(NESTED_DIR)/doc
DOC_TEST_TARGET_DIR ?= $(NESTED_DIR)/doc-test
WASM_DOC_TARGET_DIR ?= $(NESTED_DIR)/doc-wasm

SDE64 ?= /opt/sde/sde64
SDE32 ?= /opt/sde/sde
SDE_FLAGS ?= -future
SDE_SKIP ?= --skip sm4_instruction_reference --skip sm4_key_schedule_and_rounds --skip avx_vnni
SDE_SKIP_ARGS := $(if $(SDE_SKIP),-- $(SDE_SKIP),)

PODMAN_RUN_BASE := $(PODMAN) run --rm \
	-v "$(CURDIR)":/src \
	-v $(TARGET_VOLUME):/src/target \
	-v $(CARGO_REGISTRY_VOLUME):/root/.cargo/registry \
	-v $(CARGO_GIT_VOLUME):/root/.cargo/git \
	-e RUST_ISA_NESTED=/src/$(NESTED_DIR) \
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
		'image-debian                 Debian ベースのテストイメージを作る' \
		'test                         フォーマット検査・rustdoc・全 arch のテスト（QEMU / SDE / wasmtime）' \
		'test-debian                  Debian イメージで全テストを実行する' \
		'test-doc-debian              Debian イメージで native doctest を実行する' \
		'check-docs                   全ターゲットの rustdoc を -D warnings で検査する' \
		'fmt                          dprint と cargo fmt で整形する' \
		'fmt-check                    dprint と cargo fmt の整形を検査する' \
		'test-host                    x86_64 を SDE で実行するテストと cross-arch テスト' \
		'test-x86_64                  x86_64 の SDE 値テスト・doctest とホスト側のゼロコスト検査' \
		'test-check-static            シンボル・rustdoc・レーン即値の検査' \
		'test-check-wasm-link         wasm32-unknown-unknown のリンク検査' \
		'test-i686                    i686 の値テストとゼロコスト検査（SDE）' \
		'test-aarch64                 AArch64 の値テストとゼロコスト検査' \
		'test-armv7                   ARMv7 の値テスト' \
		'test-riscv64                 RISC-V の値テストとゼロコスト検査' \
		'test-loongarch64             LoongArch の値テストとゼロコスト検査' \
		'test-s390x                   s390x の値テストとゼロコスト検査' \
		'test-powerpc64le             PowerPC の値テストとゼロコスト検査' \
		'test-mips64                  MIPS64 の値テストとゼロコスト検査' \
		'test-doc                     native target の doctest（DOC_TEST=run で実行）' \
		'test-cross                   i686 と cross-arch の値テストとゼロコスト検査' \
		'test-wasip1                  wasm32-wasip1 の nextest と doctest' \
		'test-wasip1-wide-arithmetic  wide-arithmetic 有効時の nextest と doctest' \
		'inventory                    arch/extension ごとの実装状況を表示する' \
		'inventory-check              README・ゼロコスト結果表・doctest 在庫を検査する' \
		'coverage                     inventory の互換alias' \
		'coverage-check               inventory-check の互換alias' \
		'doctest-coverage             doctest 網羅を検査して arch 別件数を表示する' \
		'doctest-inventory-check      全公開命令の doctest 在庫を検査する' \
		'test-gaps                    値テストから参照されていない公開関数を表示する' \
		'test-gaps-check              未参照の公開関数があれば失敗する' \
		'zero-cost-check              ゼロコスト結果表の fixture と kernel を検査する' \
		'docs-check                   文書間リンク・アンカー・索引・API対応表・makeターゲット名を検査する' \
		'fixtures-check               fixtures/・target-nested と fixture-matrix.md の対応を検査する' \
		'duplicates                   同じ arch で重複する命令名を表示する'

.PHONY: image
image:
	$(PODMAN) build -t $(IMAGE) -f Containerfile .

.PHONY: image-debian
image-debian:
	$(PODMAN) build -t $(DEBIAN_IMAGE) -f $(DEBIAN_CONTAINERFILE) .

.PHONY: test-debian
test-debian: image-debian
	$(MAKE) IMAGE=$(DEBIAN_IMAGE) DOC_TEST=run NESTED_DIR=target-nested-debian \
		TARGET_VOLUME=$(TARGET_VOLUME)-debian \
		CARGO_REGISTRY_VOLUME=$(CARGO_REGISTRY_VOLUME)-debian \
		CARGO_GIT_VOLUME=$(CARGO_GIT_VOLUME)-debian \
		test

.PHONY: test-doc-debian
test-doc-debian: image-debian
	$(MAKE) IMAGE=$(DEBIAN_IMAGE) DOC_TEST=run NESTED_DIR=target-nested-debian \
		TARGET_VOLUME=$(TARGET_VOLUME)-debian \
		CARGO_REGISTRY_VOLUME=$(CARGO_REGISTRY_VOLUME)-debian \
		CARGO_GIT_VOLUME=$(CARGO_GIT_VOLUME)-debian \
		test-doc

.PHONY: inventory
inventory:
	$(PODMAN_RUN) python3 tools/inventory.py $(INVENTORY_ARGS)

.PHONY: coverage
coverage: inventory

.PHONY: zero-cost-check
zero-cost-check:
	$(PODMAN_RUN) python3 tools/zero_cost.py --check

.PHONY: doctest-coverage
doctest-coverage:
	$(PODMAN_RUN) python3 tools/doctest_inventory.py --summary

.PHONY: doctest-inventory-check
doctest-inventory-check:
	$(PODMAN_RUN) python3 tools/doctest_inventory.py --check

.PHONY: inventory-check
inventory-check: zero-cost-check doctest-inventory-check test-gaps-check
	$(PODMAN_RUN) python3 tools/inventory.py --check
	$(PODMAN_RUN) python3 tools/doc_names.py --check

.PHONY: coverage-check
coverage-check: inventory-check

.PHONY: docs-check
docs-check:
	$(PODMAN_RUN) python3 tools/doc_links.py --check
	$(PODMAN_RUN) python3 tools/mapping.py --check
	$(PODMAN_RUN) python3 tools/commands.py --check

.PHONY: fixtures-check
fixtures-check:
	$(PODMAN_RUN) python3 tools/fixtures.py --check

.PHONY: check-mnemonics
check-mnemonics:
	$(PODMAN_RUN) python3 tools/mnemonics.py

.PHONY: duplicates
duplicates:
	$(PODMAN_RUN) python3 tools/duplicates.py $(DUPLICATES_ARGS)

.PHONY: test-gaps
test-gaps:
	$(PODMAN_RUN) python3 tools/test_gaps.py $(TEST_GAPS_ARGS)

.PHONY: test-gaps-check
test-gaps-check:
	$(PODMAN_RUN) python3 tools/test_gaps.py --check

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
test: fmt-check inventory-check docs-check fixtures-check check-mnemonics check-docs test-check-wasm-link test-host test-doc test-wasip1 test-wasip1-wide-arithmetic

.PHONY: test-host
test-host: test-x86_64 test-cross

.PHONY: test-cross
test-cross: test-i686 test-aarch64 test-armv7 test-riscv64 test-loongarch64 test-s390x test-powerpc64le test-mips64

.PHONY: test-x86_64
# x86_link and consistency only build and inspect files. Running their test
# drivers under SDE makes the nested cargo and llvm-objdump calls extremely
# slow, so only the tests that execute x86 instructions use SDE.
test-x86_64:
	$(PODMAN_RUN_SDE) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test x86 $(CARGO_ARGS) $(SDE_SKIP_ARGS)
	$(PODMAN_RUN_SDE) cargo test --target $(X86_64_TARGET) --doc $(CARGO_ARGS)
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test x86_link $(CARGO_ARGS)
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test consistency $(CARGO_ARGS)

.PHONY: test-check-static
test-check-static:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test consistency $(CARGO_ARGS)

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
ifeq ($(DOC_TEST),run)
test-doc:
	$(PODMAN_RUN) bash -lc 'set -e; \
		for t in $(DOC_TARGETS); do \
			case "$$t" in \
				i686-*) cc=i686-linux-gnu-gcc; r="qemu-i386-static -cpu max"; l=/usr/i686-linux-gnu ;; \
				aarch64-*) cc=aarch64-linux-gnu-gcc; r="qemu-aarch64-static -cpu max"; l=/usr/aarch64-linux-gnu ;; \
				armv7-*) cc=arm-linux-gnueabihf-gcc; r="qemu-arm-static -cpu max"; l=/usr/arm-linux-gnueabihf ;; \
				powerpc64le-*) cc=powerpc64le-linux-gnu-gcc; r="qemu-ppc64le-static -cpu power9"; l=/usr/powerpc64le-linux-gnu ;; \
				riscv64gc-*) cc=riscv64-linux-gnu-gcc; r="qemu-riscv64-static -cpu max"; l=/usr/riscv64-linux-gnu ;; \
				s390x-*) cc=s390x-linux-gnu-gcc; r="qemu-s390x-static -cpu max"; l=/usr/s390x-linux-gnu ;; \
				loongarch64-*) cc=loongarch64-linux-gnu-gcc; r="qemu-loongarch64-static -cpu max"; l=/usr/loongarch64-linux-gnu ;; \
				mips64-*) cc=mips64-linux-gnuabi64-gcc; r="qemu-mips64-static -cpu Loongson-3A4000"; l=/usr/mips64-linux-gnuabi64 ;; \
			esac; \
			key=$$(echo "$$t" | tr "a-z-" "A-Z_"); \
			flags="-C linker=$$cc -C panic=abort"; \
			echo "===== doctest $$t"; \
			RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR=$(DOC_TEST_TARGET_DIR) RUST_ISA_DOCTEST=1 \
				env "CARGO_TARGET_$${key}_RUNNER=$$r -L $$l" \
					RUSTFLAGS="$$flags" RUSTDOCFLAGS="$$flags" \
				cargo test --doc --target "$$t" -Z build-std=std,panic_abort; \
		done'
else
test-doc:
	$(PODMAN_RUN) bash -lc 'set -e; \
		for t in $(DOC_TARGETS); do \
			echo "===== doctest compile $$t"; \
			RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="--no-run" CARGO_TARGET_DIR=$(DOC_TEST_TARGET_DIR) \
				cargo test --doc --target "$$t" -Z build-std=std; \
		done'
endif

.PHONY: test-wasip1
test-wasip1:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(WASIP1_TARGET) $(CARGO_ARGS)
	$(PODMAN_RUN) cargo test --target $(WASIP1_TARGET) --doc $(CARGO_ARGS)

.PHONY: test-wasip1-wide-arithmetic
test-wasip1-wide-arithmetic:
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --features wide-arithmetic --target $(WASIP1_TARGET) $(CARGO_ARGS)
	$(PODMAN_RUN) cargo test --features wide-arithmetic --target $(WASIP1_TARGET) --doc $(CARGO_ARGS)
	$(PODMAN_RUN) $(NEXTEST) $(NEXTEST_FLAGS) --target $(X86_64_TARGET) --test wide_arithmetic_link $(CARGO_ARGS)
