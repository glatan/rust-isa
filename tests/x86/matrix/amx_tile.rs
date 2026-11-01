fn amx_tile_available() -> bool {
    use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv};

    let leaf1 = __cpuid(1);
    if leaf1.ecx & (1 << 27) == 0 {
        return false;
    }
    let leaf = __cpuid_count(7, 0);
    if leaf.edx & (1 << 24) == 0 {
        return false;
    }

    // XTILEDATA is a dynamically enabled xstate feature on Linux. Requesting
    // permission is required even when the XCR0 bit is already set.
    #[cfg(target_os = "linux")]
    {
        if !request_amx_tile_data_permission() {
            return false;
        }
        // TILECFG is required by ldtilecfg. TILEDATA may remain disabled until
        // the first tile-data instruction, so only check TILECFG here.
        let xcr0 = unsafe { _xgetbv(0) };
        xcr0 & (1 << 17) != 0
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

#[cfg(target_os = "linux")]
fn request_amx_tile_data_permission() -> bool {
    const SYS_ARCH_PRCTL: i64 = 158;
    const ARCH_REQ_XCOMP_PERM: i64 = 0x1023;
    const XFEATURE_XTILEDATA: i64 = 18;

    let result: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_ARCH_PRCTL => result,
            in("rdi") ARCH_REQ_XCOMP_PERM,
            in("rsi") XFEATURE_XTILEDATA,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack),
        );
    }
    result == 0
}

#[test]
fn amx_api_is_gated() {
    if !amx_tile_available() {
        return;
    }
    use rust_isa::x86::matrix::amx_tile::{
        AmxGuard, TileConfig, ldtilecfg, sttilecfg, tileloadd, tilestored, tilezero,
    };
    assert_eq!(core::mem::align_of::<TileConfig>(), 64);
    let config_a = TileConfig::builder().start_row(0).tile::<0>(16, 64).build();
    let config_b = TileConfig::builder().start_row(2).tile::<0>(16, 64).build();
    let mut readback = TileConfig::from_bytes([0; 64]);
    let data = core::array::from_fn::<u8, 1024, _>(|index| (index as u8).wrapping_mul(7));
    let mut out = [0u8; 1024];
    unsafe {
        let guard = AmxGuard::enter(&config_a);
        ldtilecfg(&config_b);
        sttilecfg(&guard, &mut readback);

        tilezero::<0>(&guard);
        tilestored::<0>(&guard, out.as_mut_ptr(), 64);
        assert_eq!(out, [0u8; 1024]);
        tileloadd::<0>(&guard, data.as_ptr(), 64);
        tilestored::<0>(&guard, out.as_mut_ptr(), 64);
    }
    assert_eq!(readback.to_bytes(), config_b.to_bytes());
    assert_eq!(out, data);
}

#[test]
fn tile_config_builder_layout() {
    use rust_isa::x86::matrix::amx_tile::TileConfig;

    const CONFIG: TileConfig = TileConfig::builder()
        .start_row(2)
        .tile::<0>(16, 64)
        .tile::<7>(4, 16)
        .build();
    let bytes = CONFIG.to_bytes();
    assert_eq!(bytes[0], 1);
    assert_eq!(bytes[1], 2);
    assert_eq!(&bytes[16..18], &64u16.to_le_bytes());
    assert_eq!(&bytes[30..32], &16u16.to_le_bytes());
    assert_eq!(bytes[48], 16);
    assert_eq!(bytes[55], 4);
    assert!(bytes[2..16].iter().all(|byte| *byte == 0));
    assert!(bytes[18..30].iter().all(|byte| *byte == 0));
    assert!(bytes[32..48].iter().all(|byte| *byte == 0));
    assert!(bytes[49..55].iter().all(|byte| *byte == 0));
    assert!(bytes[56..64].iter().all(|byte| *byte == 0));
}

#[test]
fn amx_stateful_api_takes_guard() {
    use rust_isa::x86::matrix::amx_bf16::tdpbf16ps;
    use rust_isa::x86::matrix::amx_complex::tcmmimfp16ps;
    use rust_isa::x86::matrix::amx_tile::{
        AmxGuard, TileConfig, sttilecfg, tileloadd, tileloaddt1, tilestored, tilezero,
    };

    let _: unsafe fn(&AmxGuard) = tilezero::<0>;
    let _: unsafe fn(&AmxGuard, *const u8, usize) = tileloadd::<0>;
    let _: unsafe fn(&AmxGuard, *const u8, usize) = tileloaddt1::<0>;
    let _: unsafe fn(&AmxGuard, *mut u8, usize) = tilestored::<0>;
    let _: unsafe fn(&AmxGuard, &mut TileConfig) = sttilecfg;
    let _: unsafe fn(&AmxGuard) = tdpbf16ps::<0, 0, 0>;
    let _: unsafe fn(&AmxGuard) = tcmmimfp16ps::<0, 0, 0>;
}

fn amx_int8_available() -> bool {
    use core::arch::x86_64::__cpuid_count;

    let leaf = __cpuid_count(7, 0);
    leaf.edx & (1 << 25) != 0
}

fn amx_bf16_available() -> bool {
    use core::arch::x86_64::__cpuid_count;

    let leaf = __cpuid_count(7, 0);
    leaf.edx & (1 << 22) != 0
}

fn amx_fp16_available() -> bool {
    use core::arch::x86_64::__cpuid_count;

    let leaf = __cpuid_count(7, 1);
    leaf.eax & (1 << 21) != 0
}

fn amx_fp8_available() -> bool {
    use core::arch::x86_64::{__cpuid, __cpuid_count};

    let max = __cpuid(0).eax;
    if max < 0x1e {
        return false;
    }
    let leaf = __cpuid_count(0x1e, 1);
    leaf.eax & (1 << 4) != 0
}

fn amx_complex_available() -> bool {
    use core::arch::x86_64::__cpuid_count;

    let leaf = __cpuid_count(7, 1);
    leaf.edx & (1 << 8) != 0
}

#[test]
fn amx_int8_ones_accumulate() {
    if !amx_tile_available() || !amx_int8_available() {
        return;
    }
    use rust_isa::x86::matrix::amx_int8::tdpbssd;
    use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig, tileloadd, tilestored, tilezero};

    // 16x64 int8 A and 64x16 int8 B, represented as 16x64 byte tiles.
    let config = TileConfig::builder()
        .tile::<0>(16, 64)
        .tile::<1>(16, 64)
        .tile::<2>(16, 64)
        .build();
    let a = [1u8; 1024];
    let b = [1u8; 1024];
    let mut out = [0u8; 1024];
    unsafe {
        let guard = AmxGuard::enter(&config);
        tileloadd::<0>(&guard, a.as_ptr(), 64);
        tileloadd::<1>(&guard, b.as_ptr(), 64);
        tilezero::<2>(&guard);
        tdpbssd::<2, 0, 1>(&guard);
        tilestored::<2>(&guard, out.as_mut_ptr(), 64);
    }
    for lane in out.chunks_exact(4) {
        assert_eq!(i32::from_le_bytes(lane.try_into().unwrap()), 64);
    }
}

#[test]
fn amx_int8_mixed_ones_accumulate() {
    if !amx_tile_available() || !amx_int8_available() {
        return;
    }
    use rust_isa::x86::matrix::amx_int8::{tdpbsud, tdpbuud};
    use rust_isa::x86::matrix::amx_tile::{
        AmxGuard, TileConfig, tileloadd, tileloaddt1, tilestored, tilezero,
    };

    let config = TileConfig::builder()
        .tile::<0>(16, 64)
        .tile::<1>(16, 64)
        .tile::<2>(16, 64)
        .build();
    let a = [1u8; 1024];
    let b = [1u8; 1024];
    let mut out = [0u8; 1024];
    unsafe {
        let guard = AmxGuard::enter(&config);
        tileloadd::<0>(&guard, a.as_ptr(), 64);
        tileloaddt1::<1>(&guard, b.as_ptr(), 64);
        tilezero::<2>(&guard);
        tdpbsud::<2, 0, 1>(&guard);
        tilestored::<2>(&guard, out.as_mut_ptr(), 64);
    }
    for lane in out.chunks_exact(4) {
        assert_eq!(i32::from_le_bytes(lane.try_into().unwrap()), 64);
    }

    out = [0u8; 1024];
    unsafe {
        let guard = AmxGuard::enter(&config);
        tileloadd::<0>(&guard, a.as_ptr(), 64);
        tileloadd::<1>(&guard, b.as_ptr(), 64);
        tilezero::<2>(&guard);
        tdpbuud::<2, 0, 1>(&guard);
        tilestored::<2>(&guard, out.as_mut_ptr(), 64);
    }
    for lane in out.chunks_exact(4) {
        assert_eq!(i32::from_le_bytes(lane.try_into().unwrap()), 64);
    }
}

#[test]
fn amx_bf16_ones_accumulate() {
    if !amx_tile_available() || !amx_bf16_available() {
        return;
    }
    use rust_isa::x86::matrix::amx_bf16::tdpbf16ps;
    use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig, tileloadd, tilestored, tilezero};

    let config = TileConfig::builder()
        .tile::<0>(16, 64)
        .tile::<1>(16, 64)
        .tile::<2>(16, 64)
        .build();
    // bfloat16 1.0 is 0x3f80.
    let a: [u8; 1024] = core::array::from_fn(|index| if index % 2 == 0 { 0x80 } else { 0x3f });
    let b = a;
    let mut out = [0u8; 1024];
    unsafe {
        let guard = AmxGuard::enter(&config);
        tileloadd::<0>(&guard, a.as_ptr(), 64);
        tileloadd::<1>(&guard, b.as_ptr(), 64);
        tilezero::<2>(&guard);
        tdpbf16ps::<2, 0, 1>(&guard);
        tdpbf16ps::<2, 0, 1>(&guard);
        tilestored::<2>(&guard, out.as_mut_ptr(), 64);
    }
    for lane in out.chunks_exact(4) {
        assert_eq!(f32::from_le_bytes(lane.try_into().unwrap()), 64.0);
    }
}

#[test]
fn amx_fp16_ones_accumulate() {
    if !amx_tile_available() || !amx_fp16_available() {
        return;
    }
    use rust_isa::x86::matrix::amx_fp16::tdpfp16ps;
    use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig, tileloadd, tilestored, tilezero};

    let config = TileConfig::builder()
        .tile::<0>(16, 64)
        .tile::<1>(16, 64)
        .tile::<2>(16, 64)
        .build();
    // binary16 1.0 is 0x3c00.
    let a: [u8; 1024] = core::array::from_fn(|index| if index % 2 == 0 { 0x00 } else { 0x3c });
    let b = a;
    let mut out = [0u8; 1024];
    unsafe {
        let guard = AmxGuard::enter(&config);
        tileloadd::<0>(&guard, a.as_ptr(), 64);
        tileloadd::<1>(&guard, b.as_ptr(), 64);
        tilezero::<2>(&guard);
        tdpfp16ps::<2, 0, 1>(&guard);
        tdpfp16ps::<2, 0, 1>(&guard);
        tilestored::<2>(&guard, out.as_mut_ptr(), 64);
    }
    for lane in out.chunks_exact(4) {
        assert_eq!(f32::from_le_bytes(lane.try_into().unwrap()), 64.0);
    }
}

#[test]
fn amx_fp8_ones_accumulate() {
    if !amx_tile_available() || !amx_fp8_available() {
        return;
    }
    use rust_isa::x86::matrix::amx_fp8::{tdpbf8ps, tdpbhf8ps, tdphbf8ps, tdphf8ps};
    use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig, tileloadd, tilestored, tilezero};

    const BF8_ONE: u8 = 0x3c;
    const BF8_TWO: u8 = 0x40;
    const HF8_ONE: u8 = 0x38;
    const HF8_TWO: u8 = 0x40;

    let config = TileConfig::builder()
        .tile::<0>(16, 64)
        .tile::<1>(16, 64)
        .tile::<2>(16, 64)
        .build();
    let mut out = [0u8; 1024];
    for (a_value, b_value, run) in [
        (
            BF8_ONE,
            BF8_TWO,
            tdpbf8ps::<2, 0, 1> as unsafe fn(&AmxGuard),
        ),
        (BF8_ONE, HF8_TWO, tdpbhf8ps::<2, 0, 1>),
        (HF8_ONE, BF8_TWO, tdphbf8ps::<2, 0, 1>),
        (HF8_ONE, HF8_TWO, tdphf8ps::<2, 0, 1>),
    ] {
        let a = [a_value; 1024];
        let b = [b_value; 1024];
        unsafe {
            let guard = AmxGuard::enter(&config);
            tileloadd::<0>(&guard, a.as_ptr(), 64);
            tileloadd::<1>(&guard, b.as_ptr(), 64);
            tilezero::<2>(&guard);
            run(&guard);
            run(&guard);
            tilestored::<2>(&guard, out.as_mut_ptr(), 64);
        }
        for lane in out.chunks_exact(4) {
            assert_eq!(f32::from_le_bytes(lane.try_into().unwrap()), 256.0);
        }
        out = [0u8; 1024];
    }
}

#[test]
fn amx_complex_values_accumulate() {
    if !amx_tile_available() || !amx_complex_available() {
        return;
    }
    use rust_isa::x86::matrix::amx_complex::{tcmmimfp16ps, tcmmrlfp16ps};
    use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig, tileloadd, tilestored, tilezero};

    let config = TileConfig::builder()
        .tile::<0>(16, 64)
        .tile::<1>(16, 64)
        .tile::<2>(16, 64)
        .tile::<3>(16, 64)
        .build();
    // Each dword of A is complex binary16 1.0 + 2.0i.
    let a: [u8; 1024] = core::array::from_fn(|index| match index % 4 {
        1 => 0x3c,
        3 => 0x40,
        _ => 0,
    });
    // Each dword of B is complex binary16 1.0 + 0.0i.
    let b: [u8; 1024] = core::array::from_fn(|index| match index % 4 {
        1 => 0x3c,
        _ => 0,
    });
    let mut real = [0u8; 1024];
    let mut imaginary = [0u8; 1024];
    unsafe {
        let guard = AmxGuard::enter(&config);
        tileloadd::<0>(&guard, a.as_ptr(), 64);
        tileloadd::<1>(&guard, b.as_ptr(), 64);
        tilezero::<2>(&guard);
        tcmmrlfp16ps::<2, 0, 1>(&guard);
        tilestored::<2>(&guard, real.as_mut_ptr(), 64);
        tilezero::<3>(&guard);
        tcmmimfp16ps::<3, 0, 1>(&guard);
        tilestored::<3>(&guard, imaginary.as_mut_ptr(), 64);
    }
    for lane in real.chunks_exact(4) {
        assert_eq!(f32::from_le_bytes(lane.try_into().unwrap()), 16.0);
    }
    for lane in imaginary.chunks_exact(4) {
        assert_eq!(f32::from_le_bytes(lane.try_into().unwrap()), 32.0);
    }
}

#[test]
#[ignore = "requires an AMX-FP8 CPU and OS tile-state support"]
fn amx_fp8_smoke() {
    use rust_isa::x86::matrix::amx_fp8::{tdpbf8ps, tdpbhf8ps, tdphbf8ps, tdphf8ps};
    use rust_isa::x86::matrix::amx_tile::{AmxGuard, TileConfig};

    let config = TileConfig::builder().tile::<0>(1, 64).build();
    unsafe {
        let guard = AmxGuard::enter(&config);
        core::hint::black_box(tdpbf8ps::<0, 0, 0>(&guard));
        core::hint::black_box(tdpbhf8ps::<0, 0, 0>(&guard));
        core::hint::black_box(tdphbf8ps::<0, 0, 0>(&guard));
        core::hint::black_box(tdphf8ps::<0, 0, 0>(&guard));
    }
}
