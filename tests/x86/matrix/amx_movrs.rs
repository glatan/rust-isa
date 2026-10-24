use rust_isa::x86::matrix::amx_movrs::{tileloaddrs, tileloaddrst1};
use rust_isa::x86::matrix::amx_tile::AmxGuard;

#[test]
fn amx_movrs_api_takes_guard() {
    let _: unsafe fn(&AmxGuard, *const u8, usize) = tileloaddrs::<0>;
    let _: unsafe fn(&AmxGuard, *const u8, usize) = tileloaddrst1::<0>;
}
