#include <stdint.h>
#include <altivec.h>

typedef vector unsigned char v16u8;

/*
 * The accumulator is represented by four consecutive 128-bit VSRs. The
 * assemble and disassemble builtins use opposite register orders, so the
 * values are passed to assemble_acc in reverse.
 */
__attribute__((always_inline))
static void load_acc(__vector_quad *q, const uint8_t *acc) {
    v16u8 x = *(const v16u8 *)(const void *)acc;
    v16u8 y = *(const v16u8 *)(const void *)(acc + 16);
    v16u8 z = *(const v16u8 *)(const void *)(acc + 32);
    v16u8 t = *(const v16u8 *)(const void *)(acc + 48);
    __builtin_mma_assemble_acc(q, t, z, y, x);
}

__attribute__((always_inline))
static void store_acc(uint8_t *acc, __vector_quad *q) {
    __builtin_mma_disassemble_acc(acc, q);
}

__attribute__((always_inline))
void rust_isa_mma_xxsetaccz(uint8_t *acc) {
    __vector_quad q;
    __builtin_mma_xxsetaccz(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xxmfacc(const uint8_t *acc, uint8_t *out) {
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xxmfacc(&q);
    store_acc(out, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xxmtacc(const uint8_t *values, uint8_t *out) {
    __vector_quad q;
    load_acc(&q, values);
    __builtin_mma_xxmfacc(&q);
    store_acc(out, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi4ger8(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi4ger8(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi4ger8pp(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi4ger8pp(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi8ger4(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi8ger4(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi8ger4pp(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi8ger4pp(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi8ger4spp(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi8ger4spp(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi16ger2(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi16ger2(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi16ger2s(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi16ger2s(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi16ger2pp(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi16ger2pp(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}

__attribute__((always_inline))
void rust_isa_mma_xvi16ger2spp(uint8_t *acc, const uint8_t *a, const uint8_t *b) {
    v16u8 va = *(const v16u8 *)(const void *)a;
    v16u8 vb = *(const v16u8 *)(const void *)b;
    __vector_quad q;
    load_acc(&q, acc);
    __builtin_mma_xvi16ger2spp(&q, va, vb);
    __builtin_mma_xxmfacc(&q);
    store_acc(acc, &q);
}
