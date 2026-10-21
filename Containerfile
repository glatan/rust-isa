FROM docker.io/library/archlinux:base-devel-20261018.0.612668@sha256:0ccc8b2e55c0c26a6e501e892cc6b1cdea7cfdf77e432941320148b4dabd2bff

COPY dprint.json /tmp/dprint.json

# The mirror date matches the base image tag. A live mirror would drift
# away from the digest-pinned rootfs.
RUN printf 'Server = https://archive.archlinux.org/repos/2026/10/18/$repo/os/$arch\n' > /etc/pacman.d/mirrorlist \
    && pacman -Syu --noconfirm \
        rust \
        rust-wasm \
        cargo-nextest \
        rust-analyzer \
        dprint \
        clang \
        llvm \
        binaryen \
        wabt \
        wasmtime \
        qemu-user-static \
    && pacman -Scc --noconfirm

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
    && dprint --version \
    && clang --version \
    && llvm-ar --version \
    && cargo nextest --version \
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

# Preload the pinned dprint plugins so formatting does not need network access
# after the image is built.
RUN cd /tmp \
    && dprint output-file-paths > /dev/null \
    && test -d /root/.cache/dprint
