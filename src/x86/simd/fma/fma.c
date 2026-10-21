#include <stdint.h>
#include <immintrin.h>

__attribute__((always_inline))
void rust_isa_fma_vfmadd132ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmadd132ps %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd132pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmadd132pd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd213ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmadd213ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd213pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmadd213pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd231ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmadd231ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd231pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmadd231pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub132ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsub132ps %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub132pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsub132pd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub213ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsub213ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub213pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsub213pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub231ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsub231ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub231pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsub231pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd132ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmadd132ps %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd132pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmadd132pd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd213ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmadd213ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd213pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmadd213pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd231ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmadd231ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd231pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmadd231pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub132ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmsub132ps %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub132pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmsub132pd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub213ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmsub213ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub213pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmsub213pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub231ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmsub231ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub231pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmsub231pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmaddsub132ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmaddsub132ps %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmaddsub132pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmaddsub132pd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmaddsub213ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmaddsub213ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmaddsub213pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmaddsub213pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmaddsub231ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmaddsub231ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmaddsub231pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmaddsub231pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsubadd132ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsubadd132ps %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsubadd132pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsubadd132pd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsubadd213ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsubadd213ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsubadd213pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsubadd213pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsubadd231ps(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256 va = _mm256_loadu_ps((const float *)(const void *)a);
    __m256 vb = _mm256_loadu_ps((const float *)(const void *)b);
    __m256 vc = _mm256_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsubadd231ps %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsubadd231pd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m256d va = _mm256_loadu_pd((const double *)(const void *)a);
    __m256d vb = _mm256_loadu_pd((const double *)(const void *)b);
    __m256d vc = _mm256_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsubadd231pd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm256_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd132ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfmadd132ss %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd132sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfmadd132sd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd213ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfmadd213ss %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd213sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfmadd213sd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd231ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfmadd231ss %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmadd231sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfmadd231sd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub132ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsub132ss %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub132sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsub132sd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub213ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsub213ss %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub213sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsub213sd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub231ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfmsub231ss %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfmsub231sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfmsub231sd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd132ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmadd132ss %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd132sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmadd132sd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd213ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmadd213ss %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd213sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmadd213sd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd231ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmadd231ss %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmadd231sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmadd231sd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub132ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmsub132ss %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub132sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmsub132sd %1, %2, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub213ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmsub213ss %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub213sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmsub213sd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub231ss(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128 va = _mm_loadu_ps((const float *)(const void *)a);
    __m128 vb = _mm_loadu_ps((const float *)(const void *)b);
    __m128 vc = _mm_loadu_ps((const float *)(const void *)c);
    __asm__("vfnmsub231ss %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_ps((float *)(void *)out, va);
}

__attribute__((always_inline))
void rust_isa_fma_vfnmsub231sd(const uint8_t *a, const uint8_t *b, const uint8_t *c, uint8_t *out) {
    __m128d va = _mm_loadu_pd((const double *)(const void *)a);
    __m128d vb = _mm_loadu_pd((const double *)(const void *)b);
    __m128d vc = _mm_loadu_pd((const double *)(const void *)c);
    __asm__("vfnmsub231sd %2, %1, %0" : "+x"(va) : "x"(vb), "x"(vc));
    _mm_storeu_pd((double *)(void *)out, va);
}
