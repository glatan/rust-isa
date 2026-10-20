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
    assert_eq!(unsafe { pdep(0b101, 0b1010_1010) }, 0b0010_0010);
    assert_eq!(unsafe { pext(0b1010_1010, 0b1010_1010) }, 0b1111);
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
