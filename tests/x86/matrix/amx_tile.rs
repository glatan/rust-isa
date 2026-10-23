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
