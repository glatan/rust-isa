#include <stdint.h>

extern uint32_t run(void);

__attribute__((noreturn))
static void fixture_exit(uint32_t code) {
    register long v0 __asm__("$2") = 5058;
    register long a0 __asm__("$4") = (long)code;
    __asm__ volatile("syscall" : : "r"(v0), "r"(a0) : "memory");
    __builtin_unreachable();
}

__attribute__((noreturn))
void _start(void) {
    fixture_exit(run());
}

__attribute__((noreturn))
void fixture_exit_rust(uint32_t code) {
    fixture_exit(code);
}
