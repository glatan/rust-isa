#![cfg_attr(all(target_arch = "wasm32", not(target_os = "wasi")), no_std)]

#[cfg(all(
    target_arch = "wasm32",
    target_feature = "simd128",
    target_feature = "relaxed-simd"
))]
mod kernels {
    use rust_isa::wasm32::simd::relaxed_simd::f32x4_relaxed_madd;
    use rust_isa::wasm32::simd::simd128::{V128, f32x4_add};

    #[unsafe(no_mangle)]
    pub extern "C" fn kernel_one(a: V128, b: V128) -> V128 {
        f32x4_add(a, b)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn kernel_two(a: V128, b: V128, c: V128) -> V128 {
        let sum = f32x4_add(a, b);
        f32x4_add(sum, c)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn kernel_relaxed(a: V128, b: V128, c: V128) -> V128 {
        let product = f32x4_relaxed_madd(a, b, c);
        f32x4_relaxed_madd(product, b, c)
    }
}

#[cfg(all(
    target_arch = "wasm32",
    not(all(target_feature = "simd128", target_feature = "relaxed-simd"))
))]
compile_error!("wasm-simd-link requires target_feature simd128 and relaxed-simd");

#[cfg(not(target_arch = "wasm32"))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_one() {}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
