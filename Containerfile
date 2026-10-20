FROM docker.io/library/archlinux:base-devel-20261011.0.609700@sha256:061679925e5d5028f2df9395d441ce6c4b9accf0157ff3486c06b6962e014ef1

# The mirror date matches the base image tag. A live mirror would drift
# away from the digest-pinned rootfs.
RUN printf 'Server = https://archive.archlinux.org/repos/2026/10/11/$repo/os/$arch\n' > /etc/pacman.d/mirrorlist \
    && pacman -Syu --noconfirm \
        rust \
        rust-wasm \
        rust-analyzer \
        clang \
        llvm \
        binaryen \
        wabt \
        wasmtime \
        qemu-user-static \
    && pacman -Scc --noconfirm

# The 2026-10-11 snapshot has rustc 1.99.0. The crate MSRV is 1.99.1.
# rust-analyzer in that snapshot is tied to 1.99.0, so it is removed before the upgrade.
RUN pacman -Rdd --noconfirm rust-analyzer \
    && pacman -U --noconfirm \
        https://archive.archlinux.org/packages/r/rust/rust-1:1.99.1-1-x86_64.pkg.tar.zst \
        https://archive.archlinux.org/packages/r/rust-wasm/rust-wasm-1:1.99.1-1-x86_64.pkg.tar.zst \
    && pacman -Udd --noconfirm \
        https://archive.archlinux.org/packages/r/rust-analyzer/rust-analyzer-20260907-1-x86_64.pkg.tar.zst

ENV CARGO_TARGET_WASM32_WASIP1_RUNNER=wasmtime
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER=qemu-aarch64-static
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUNNER=qemu-arm-static
ENV CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER=qemu-riscv64-static
ENV CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_RUNNER=qemu-loongarch64-static
ENV CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_RUNNER=qemu-s390x-static
ENV CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_GNU_RUNNER=qemu-ppc64le-static
ENV CARGO_TARGET_MIPS64_UNKNOWN_LINUX_GNUABI64_RUNNER=qemu-mips64-static
ENV CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_RUNNER=qemu-i386-static
WORKDIR /src

RUN rustc --version \
    && cargo --version \
    && test -x /usr/bin/rust-analyzer \
    && clang --version \
    && llvm-ar --version \
    && wasm-opt --version \
    && wasm2wat --version \
    && wasmtime --version \
    && test -x /usr/bin/qemu-aarch64-static \
    && test -x /usr/bin/qemu-arm-static \
    && test -x /usr/bin/qemu-riscv64-static \
    && test -x /usr/bin/qemu-loongarch64-static \
    && test -x /usr/bin/qemu-s390x-static \
    && test -x /usr/bin/qemu-ppc64le-static \
    && test -x /usr/bin/qemu-mips64-static \
    && test -x /usr/bin/qemu-i386-static
