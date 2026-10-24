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
    let mut bytes = [0u8; 64];
    bytes[0] = 1;
    bytes[16] = 64;
    bytes[17] = 0;
    bytes[48] = 16;
    let config = TileConfig::from_bytes(bytes);
    let mut readback = TileConfig::from_bytes([0; 64]);
    unsafe {
        ldtilecfg(&config);
        let guard = AmxGuard::enter(&config);
        sttilecfg(&guard, &mut readback);
        let mut tile = [0u8; 1024];
        tilezero::<0>(&guard);
        tileloadd::<0>(&guard, tile.as_ptr(), 64);
        tilestored::<0>(&guard, tile.as_mut_ptr(), 64);
    }
    assert_eq!(readback.to_bytes(), config.to_bytes());
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
    use rust_isa::x86::matrix::amx_tile::{
        AmxGuard, TileConfig, sttilecfg, tcmmimfp16ps, tdpbf16ps, tileloadd, tileloaddt1,
        tilestored, tilezero,
    };

    let _: unsafe fn(&AmxGuard) = tilezero::<0>;
    let _: unsafe fn(&AmxGuard, *const u8, usize) = tileloadd::<0>;
    let _: unsafe fn(&AmxGuard, *const u8, usize) = tileloaddt1::<0>;
    let _: unsafe fn(&AmxGuard, *mut u8, usize) = tilestored::<0>;
    let _: unsafe fn(&AmxGuard, &mut TileConfig) = sttilecfg;
    let _: unsafe fn(&AmxGuard) = tdpbf16ps::<0, 0, 0>;
    let _: unsafe fn(&AmxGuard) = tcmmimfp16ps::<0, 0, 0>;
}
