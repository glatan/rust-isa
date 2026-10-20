# 開発コマンドはイメージの中で動かす。ホストに cargo や clang は置かない。

IMAGE := localhost/rust-isa-dev:20261011

PODMAN_RUN := podman run --rm \
	-v "$(CURDIR)":/src \
	-v rust-isa-target:/src/target \
	-v rust-isa-cargo-registry:/root/.cargo/registry \
	-v rust-isa-cargo-git:/root/.cargo/git \
	-w /src \
	$(IMAGE)

.PHONY: help image test test-wasip1

.DEFAULT_GOAL := test

help:
	@printf '%s\n' \
		'image         開発イメージを作る' \
		'test          cargo test（既定）' \
		'test-wasip1   wasm32-wasip1 の値テストと doctest'

image:
	podman build -t $(IMAGE) -f Containerfile .

test:
	$(PODMAN_RUN) cargo test $(CARGO_ARGS)

test-wasip1:
	$(PODMAN_RUN) cargo test --target wasm32-wasip1 $(CARGO_ARGS)
