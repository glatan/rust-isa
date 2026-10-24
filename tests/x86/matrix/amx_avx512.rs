use rust_isa::x86::matrix::amx_avx512::{
    tcvtrowd2ps, tcvtrowd2ps_imm, tcvtrowps2bf16h, tcvtrowps2bf16h_imm, tcvtrowps2bf16l,
    tcvtrowps2bf16l_imm, tcvtrowps2phh, tcvtrowps2phh_imm, tcvtrowps2phl, tcvtrowps2phl_imm,
    tilemovrow, tilemovrow_imm,
};
use rust_isa::x86::matrix::amx_tile::AmxGuard;
use rust_isa::x86::simd::avx512f::V512;

#[test]
fn amx_avx512_api_takes_guard() {
    let _: unsafe fn(&AmxGuard, u32) -> V512 = tilemovrow::<0>;
    let _: unsafe fn(&AmxGuard) -> V512 = tilemovrow_imm::<0, 0>;
    let _: unsafe fn(&AmxGuard, u32) -> V512 = tcvtrowd2ps::<0>;
    let _: unsafe fn(&AmxGuard) -> V512 = tcvtrowd2ps_imm::<0, 0>;
    let _: unsafe fn(&AmxGuard, u32) -> V512 = tcvtrowps2bf16h::<0>;
    let _: unsafe fn(&AmxGuard) -> V512 = tcvtrowps2bf16h_imm::<0, 0>;
    let _: unsafe fn(&AmxGuard, u32) -> V512 = tcvtrowps2bf16l::<0>;
    let _: unsafe fn(&AmxGuard) -> V512 = tcvtrowps2bf16l_imm::<0, 0>;
    let _: unsafe fn(&AmxGuard, u32) -> V512 = tcvtrowps2phh::<0>;
    let _: unsafe fn(&AmxGuard) -> V512 = tcvtrowps2phh_imm::<0, 0>;
    let _: unsafe fn(&AmxGuard, u32) -> V512 = tcvtrowps2phl::<0>;
    let _: unsafe fn(&AmxGuard) -> V512 = tcvtrowps2phl_imm::<0, 0>;
}
