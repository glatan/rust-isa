use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::sse2::V128;

#[test]
fn optional_extension_api_surface() {
    let _: unsafe fn(V128, V128) -> V128 = rust_isa::x86::simd::sse4a::extrq;
    let _: unsafe fn(V128, V128) -> V128 = rust_isa::x86::simd::sse4a::insertq;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::fma4::vfmaddps;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::fma4::vfmaddpd;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::fma4::vfmsubps;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::fma4::vfnmaddps;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::fma4::vfnmsubps;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::fma4::vfmaddsubps;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::fma4::vfmsubaddps;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::xop::vpcmov;
    let _: unsafe fn(V128, V128) -> V128 = rust_isa::x86::simd::xop::vprotb;
    let _: unsafe fn(V128, V128, u8) -> V128 = rust_isa::x86::simd::xop::vpcomub;
    let _: unsafe fn(V128) -> V128 = rust_isa::x86::simd::xop::vfrczps;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::xop::vpmacssww;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::xop::vpmacssdu;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::xop::vpmacsdu;
    let _: unsafe fn(V128, V128, V128, u8) -> V128 = rust_isa::x86::simd::xop::vpermil2ps;
    let _: unsafe fn(V128, V128, V128, u8) -> V128 = rust_isa::x86::simd::xop::vpermil2pd;
    let _: unsafe fn(V128, V128, V128) -> V128 = rust_isa::x86::simd::xop::vpperm;
    let _: unsafe fn(V256, V256) -> V256 = rust_isa::x86::simd::avx10_1::vaddps_v256;
    let _: unsafe fn(
        rust_isa::x86::simd::avx512f::V512,
        rust_isa::x86::simd::avx512f::V512,
    ) -> rust_isa::x86::simd::avx512f::V512 = rust_isa::x86::simd::avx10_2::vmulps_v512;
    assert_eq!(
        core::mem::size_of::<rust_isa::x86::matrix::amx_tile::TileConfig>(),
        64
    );
    assert_eq!(
        core::mem::align_of::<rust_isa::x86::matrix::amx_tile::TileConfig>(),
        64
    );
}
