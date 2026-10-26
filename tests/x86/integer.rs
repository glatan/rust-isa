use rust_isa::x86::integer::bmi1::{andn, bextr, blsi, blsmsk, blsr};
use rust_isa::x86::integer::bmi2::{bzhi, pdep, pext, rorx, sarx, shlx, shrx};
use rust_isa::x86::integer::popcnt::{popcnt_u32, popcnt_u64};

fn bmi1_enabled() -> bool {
    is_x86_feature_detected!("bmi1")
}

fn bmi2_enabled() -> bool {
    is_x86_feature_detected!("bmi2")
}

fn popcnt_enabled() -> bool {
    is_x86_feature_detected!("popcnt")
}

#[test]
fn bmi1_operations() {
    if !bmi1_enabled() {
        return;
    }
    assert_eq!(unsafe { andn(0b1010, 0b1100) }, 0b0100);
    assert_eq!(unsafe { bextr(0b1111_0000, 4, 4) }, 0b1111);
    assert_eq!(unsafe { bextr(0xffff_ffff, 4, 4) }, 0xf);
    assert_eq!(unsafe { blsi(0b1010_1000) }, 0b1000);
    assert_eq!(unsafe { blsmsk(0b1010_1000) }, 0b1111);
    assert_eq!(unsafe { blsr(0b1010_1000) }, 0b1010_0000);
}

#[test]
fn bmi2_operations() {
    if !bmi2_enabled() {
        return;
    }
    assert_eq!(unsafe { bzhi(u32::MAX, 8) }, 0xff);
    assert_eq!(unsafe { bzhi(u32::MAX, 0) }, 0);
    assert_eq!(unsafe { bzhi(u32::MAX, 32) }, u32::MAX);
    assert_eq!(unsafe { pdep(0b101, 0b1010_1010) }, 0b0010_0010);
    assert_eq!(unsafe { pext(0b1010_1010, 0b1010_1010) }, 0b1111);
    assert_eq!(unsafe { pext(0b1000, 0b1010) }, 0b10);
    assert_eq!(unsafe { sarx(0x8000_0000, 4) }, 0xf800_0000);
    assert_eq!(unsafe { shlx(1, 4) }, 16);
    assert_eq!(unsafe { shrx(0x8000_0000, 4) }, 0x0800_0000);
    assert_eq!(unsafe { rorx::<4>(0x1234_5678) }, 0x8123_4567);
}

#[test]
fn popcnt() {
    if !popcnt_enabled() {
        return;
    }
    assert_eq!(unsafe { popcnt_u32(0b1011) }, 3);
    assert_eq!(unsafe { popcnt_u64(0xffff_ffff_ffff_ffff) }, 64);
}

#[test]
fn lzcnt_counts_leading_zeros() {
    if !is_x86_feature_detected!("lzcnt") {
        return;
    }
    use rust_isa::x86::integer::lzcnt::{lzcnt_u32, lzcnt_u64};
    assert_eq!(unsafe { lzcnt_u32(1) }, 31);
    assert_eq!(unsafe { lzcnt_u32(0) }, 32);
    assert_eq!(unsafe { lzcnt_u64(1) }, 63);
    assert_eq!(unsafe { lzcnt_u64(0) }, 64);
}

#[test]
fn movbe_loads_and_stores_big_endian() {
    if !is_x86_feature_detected!("movbe") {
        return;
    }
    use rust_isa::x86::integer::movbe::{
        movbe_load_u16, movbe_load_u32, movbe_load_u64, movbe_store_u16, movbe_store_u32,
        movbe_store_u64,
    };
    let mut bytes = [0u8; 8];
    unsafe { movbe_store_u16(bytes.as_mut_ptr(), 0x1234) };
    assert_eq!(&bytes[..2], &[0x12, 0x34]);
    assert_eq!(unsafe { movbe_load_u16(bytes.as_ptr()) }, 0x1234);

    unsafe { movbe_store_u32(bytes.as_mut_ptr(), 0x1234_5678) };
    assert_eq!(&bytes[..4], &[0x12, 0x34, 0x56, 0x78]);
    assert_eq!(unsafe { movbe_load_u32(bytes.as_ptr()) }, 0x1234_5678);

    unsafe { movbe_store_u64(bytes.as_mut_ptr(), 0x0123_4567_89ab_cdef) };
    assert_eq!(&bytes, &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
    assert_eq!(
        unsafe { movbe_load_u64(bytes.as_ptr()) },
        0x0123_4567_89ab_cdef
    );
}

#[test]
fn adx_carry_chains() {
    if !is_x86_feature_detected!("adx") {
        return;
    }
    use rust_isa::x86::integer::adx::{adcx_u32, adcx_u64, adox_u32, adox_u64};
    assert_eq!(unsafe { adcx_u32(u32::MAX, 1, false) }, (0, true));
    assert_eq!(unsafe { adcx_u32(1, 1, true) }, (3, false));
    assert_eq!(unsafe { adox_u32(u32::MAX, 1, false) }, (0, true));
    assert_eq!(unsafe { adox_u32(1, 1, true) }, (3, false));
    assert_eq!(unsafe { adcx_u64(u64::MAX, 1, false) }, (0, true));
    assert_eq!(unsafe { adcx_u64(u64::MAX, 1, true) }, (1, true));
    assert_eq!(unsafe { adox_u64(u64::MAX, 1, false) }, (0, true));
    assert_eq!(unsafe { adox_u64(u64::MAX, 0, true) }, (0, true));
    assert_eq!(unsafe { adox_u64(0, 0, true) }, (1, false));
}

#[test]
fn rdrand_and_rdseed_do_not_panic() {
    if is_x86_feature_detected!("rdrand") {
        use rust_isa::x86::integer::rdrand::{rdrand_u32, rdrand_u64};
        let _ = unsafe { rdrand_u32() };
        let _ = unsafe { rdrand_u64() };
    }
    if is_x86_feature_detected!("rdseed") {
        use rust_isa::x86::integer::rdseed::{rdseed_u32, rdseed_u64};
        let _ = unsafe { rdseed_u32() };
        let _ = unsafe { rdseed_u64() };
    }
}
