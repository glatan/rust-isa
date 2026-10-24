#include <stdint.h>

typedef float v4f32 __attribute__((vector_size(16)));

/*
 * The ACLE intrinsics for FCMLA are guarded by the Armv8.3-A target, but the
 * instructions themselves only need the complex-number feature. Use inline
 * assembly with vector operands so the C unit can be compiled with the
 * baseline target and still match a Rust caller that enables `fcma`.
 */
#define FCMLA_BODY(IMM)                                                        \
    do {                                                                       \
        v4f32 va = *(const v4f32 *)(const void *)a;                            \
        v4f32 vb = *(const v4f32 *)(const void *)b;                            \
        v4f32 vc = *(const v4f32 *)(const void *)acc;                          \
        __asm__("fcmla %0.4s, %1.4s, %2.4s, #" #IMM                           \
                : "+w"(vc)                                                     \
                : "w"(va), "w"(vb));                                           \
        *(v4f32 *)(void *)out = vc;                                            \
    } while (0)

__attribute__((always_inline))
void rust_isa_fcma_vcmlaq_f32(const uint8_t *acc, const uint8_t *a,
                              const uint8_t *b, uint8_t *out) {
    FCMLA_BODY(0);
}

__attribute__((always_inline))
void rust_isa_fcma_vcmlaq_rot90_f32(const uint8_t *acc, const uint8_t *a,
                                    const uint8_t *b, uint8_t *out) {
    FCMLA_BODY(90);
}

__attribute__((always_inline))
void rust_isa_fcma_vcmlaq_rot180_f32(const uint8_t *acc, const uint8_t *a,
                                     const uint8_t *b, uint8_t *out) {
    FCMLA_BODY(180);
}

__attribute__((always_inline))
void rust_isa_fcma_vcmlaq_rot270_f32(const uint8_t *acc, const uint8_t *a,
                                     const uint8_t *b, uint8_t *out) {
    FCMLA_BODY(270);
}
