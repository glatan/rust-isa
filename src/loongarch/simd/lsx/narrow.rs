use super::V128;

// Narrowing and widening shift instructions.

#[inline(always)]
fn srl_u16(v: u16, imm: u32) -> u16 {
    v >> imm
}

#[inline(always)]
fn srlr_u16(v: u16, imm: u32) -> u16 {
    if imm == 0 {
        v
    } else {
        (v >> imm) + ((v >> (imm - 1)) & 1)
    }
}

#[inline(always)]
fn ssrl_signed_u16(v: u16, imm: u32) -> u16 {
    (v >> imm).min(i8::MAX as u16)
}

#[inline(always)]
fn ssrl_unsigned_u16(v: u16, imm: u32) -> u16 {
    (v >> imm).min(u8::MAX as u16)
}

#[inline(always)]
fn ssrlr_signed_u16(v: u16, imm: u32) -> u16 {
    srlr_u16(v, imm).min(i8::MAX as u16)
}

#[inline(always)]
fn ssrlr_unsigned_u16(v: u16, imm: u32) -> u16 {
    srlr_u16(v, imm).min(u8::MAX as u16)
}

#[inline(always)]
fn sra_i16(v: i16, imm: u32) -> i16 {
    v >> imm
}

#[inline(always)]
fn srar_i16(v: i16, imm: u32) -> i16 {
    if imm == 0 {
        v
    } else {
        (v >> imm).wrapping_add((v >> (imm - 1)) & 1)
    }
}

#[inline(always)]
fn ssra_signed_i16(v: i16, imm: u32) -> i16 {
    (v >> imm).clamp(i8::MIN as i16, i8::MAX as i16)
}

#[inline(always)]
fn ssra_unsigned_i16(v: i16, imm: u32) -> i16 {
    (v >> imm).clamp(0, u8::MAX as i16)
}

#[inline(always)]
fn ssrar_signed_i16(v: i16, imm: u32) -> i16 {
    srar_i16(v, imm).clamp(i8::MIN as i16, i8::MAX as i16)
}

#[inline(always)]
fn ssrar_unsigned_i16(v: i16, imm: u32) -> i16 {
    srar_i16(v, imm).clamp(0, u8::MAX as i16)
}

macro_rules! narrow_logical_b_h {
    ($(#[$meta:meta])* $name:ident, $op:path) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128, b: V128) -> V128 {
            const { assert!(IMM <= 15, "immediate out of range") };
            let a = a.to_bytes();
            let b = b.to_bytes();
            let mut out = [0u8; 16];
            for (lane, (a_chunk, b_chunk)) in a.as_chunks::<2>().0.iter().zip(b.as_chunks::<2>().0.iter()).enumerate() {
                let av = u16::from_le_bytes(
                    *a_chunk,
                );
                let bv = u16::from_le_bytes(
                    *b_chunk,
                );
                let a_shifted = $op(av, IMM) as u16;
                let b_shifted = $op(bv, IMM) as u16;
                out[lane * 1..lane * 1 + 1]
                    .copy_from_slice(&a_shifted.to_le_bytes()[..1]);
                out[0 + lane * 1 + 8
                    ..0 + lane * 1 + 8 + 1]
                    .copy_from_slice(&b_shifted.to_le_bytes()[..1]);}
            V128::from_bytes(out)
        }
    };
}

macro_rules! narrow_arith_b_h {
    ($(#[$meta:meta])* $name:ident, $op:path) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128, b: V128) -> V128 {
            const { assert!(IMM <= 15, "immediate out of range") };
            let a = a.to_bytes();
            let b = b.to_bytes();
            let mut out = [0u8; 16];
            for (lane, (a_chunk, b_chunk)) in a.as_chunks::<2>().0.iter().zip(b.as_chunks::<2>().0.iter()).enumerate() {
                let av = i16::from_le_bytes(
                    *a_chunk,
                );
                let bv = i16::from_le_bytes(
                    *b_chunk,
                );
                let a_shifted = $op(av, IMM) as u16;
                let b_shifted = $op(bv, IMM) as u16;
                out[lane * 1..lane * 1 + 1]
                    .copy_from_slice(&a_shifted.to_le_bytes()[..1]);
                out[0 + lane * 1 + 8
                    ..0 + lane * 1 + 8 + 1]
                    .copy_from_slice(&b_shifted.to_le_bytes()[..1]);}
            V128::from_bytes(out)
        }
    };
}

/// `vsllwil.h.b`
#[inline(always)]
pub fn vsllwil_h_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_chunk, out_chunk) in bytes.iter().zip(out.as_chunks_mut::<2>().0.iter_mut()) {
        let raw = *bytes_chunk;
        let value = (raw as i8) as i128;
        let widened = value << IMM;
        out_chunk.copy_from_slice(&(widened as i16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsllwil.hu.bu`
#[inline(always)]
pub fn vsllwil_hu_bu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_chunk, out_chunk) in bytes.iter().zip(out.as_chunks_mut::<2>().0.iter_mut()) {
        let raw = *bytes_chunk;
        let value = (raw as u8) as i128;
        let widened = value << IMM;
        out_chunk.copy_from_slice(&(widened as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

narrow_logical_b_h!(
    /// `vsrlni.b.h`
    vsrlni_b_h,
    srl_u16
);

narrow_logical_b_h!(
    /// `vsrlrni.b.h`
    vsrlrni_b_h,
    srlr_u16
);

narrow_arith_b_h!(
    /// `vsrani.b.h`
    vsrani_b_h,
    sra_i16
);

narrow_arith_b_h!(
    /// `vsrarni.b.h`
    vsrarni_b_h,
    srar_i16
);

narrow_logical_b_h!(
    /// `vssrlni.b.h`
    vssrlni_b_h,
    ssrl_signed_u16
);

narrow_logical_b_h!(
    /// `vssrlni.bu.h`
    vssrlni_bu_h,
    ssrl_unsigned_u16
);

narrow_logical_b_h!(
    /// `vssrlrni.b.h`
    vssrlrni_b_h,
    ssrlr_signed_u16
);

narrow_logical_b_h!(
    /// `vssrlrni.bu.h`
    vssrlrni_bu_h,
    ssrlr_unsigned_u16
);

narrow_arith_b_h!(
    /// `vssrani.b.h`
    vssrani_b_h,
    ssra_signed_i16
);

narrow_arith_b_h!(
    /// `vssrani.bu.h`
    vssrani_bu_h,
    ssra_unsigned_i16
);

narrow_arith_b_h!(
    /// `vssrarni.b.h`
    vssrarni_b_h,
    ssrar_signed_i16
);

narrow_arith_b_h!(
    /// `vssrarni.bu.h`
    vssrarni_bu_h,
    ssrar_unsigned_i16
);

#[inline(always)]
fn srl_u32(v: u32, imm: u32) -> u32 {
    v >> imm
}

#[inline(always)]
fn srlr_u32(v: u32, imm: u32) -> u32 {
    if imm == 0 {
        v
    } else {
        (v >> imm) + ((v >> (imm - 1)) & 1)
    }
}

#[inline(always)]
fn ssrl_signed_u32(v: u32, imm: u32) -> u32 {
    (v >> imm).min(i16::MAX as u32)
}

#[inline(always)]
fn ssrl_unsigned_u32(v: u32, imm: u32) -> u32 {
    (v >> imm).min(u16::MAX as u32)
}

#[inline(always)]
fn ssrlr_signed_u32(v: u32, imm: u32) -> u32 {
    srlr_u32(v, imm).min(i16::MAX as u32)
}

#[inline(always)]
fn ssrlr_unsigned_u32(v: u32, imm: u32) -> u32 {
    srlr_u32(v, imm).min(u16::MAX as u32)
}

#[inline(always)]
fn sra_i32(v: i32, imm: u32) -> i32 {
    v >> imm
}

#[inline(always)]
fn srar_i32(v: i32, imm: u32) -> i32 {
    if imm == 0 {
        v
    } else {
        (v >> imm).wrapping_add((v >> (imm - 1)) & 1)
    }
}

#[inline(always)]
fn ssra_signed_i32(v: i32, imm: u32) -> i32 {
    (v >> imm).clamp(i16::MIN as i32, i16::MAX as i32)
}

#[inline(always)]
fn ssra_unsigned_i32(v: i32, imm: u32) -> i32 {
    (v >> imm).clamp(0, u16::MAX as i32)
}

#[inline(always)]
fn ssrar_signed_i32(v: i32, imm: u32) -> i32 {
    srar_i32(v, imm).clamp(i16::MIN as i32, i16::MAX as i32)
}

#[inline(always)]
fn ssrar_unsigned_i32(v: i32, imm: u32) -> i32 {
    srar_i32(v, imm).clamp(0, u16::MAX as i32)
}

macro_rules! narrow_logical_h_w {
    ($(#[$meta:meta])* $name:ident, $op:path) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128, b: V128) -> V128 {
            const { assert!(IMM <= 31, "immediate out of range") };
            let a = a.to_bytes();
            let b = b.to_bytes();
            let mut out = [0u8; 16];
            for (lane, (a_chunk, b_chunk)) in a.as_chunks::<4>().0.iter().zip(b.as_chunks::<4>().0.iter()).enumerate() {
                let av = u32::from_le_bytes(
                    *a_chunk,
                );
                let bv = u32::from_le_bytes(
                    *b_chunk,
                );
                let a_shifted = $op(av, IMM) as u32;
                let b_shifted = $op(bv, IMM) as u32;
                out[lane * 2..lane * 2 + 2]
                    .copy_from_slice(&a_shifted.to_le_bytes()[..2]);
                out[0 + lane * 2 + 8
                    ..0 + lane * 2 + 8 + 2]
                    .copy_from_slice(&b_shifted.to_le_bytes()[..2]);}
            V128::from_bytes(out)
        }
    };
}

macro_rules! narrow_arith_h_w {
    ($(#[$meta:meta])* $name:ident, $op:path) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128, b: V128) -> V128 {
            const { assert!(IMM <= 31, "immediate out of range") };
            let a = a.to_bytes();
            let b = b.to_bytes();
            let mut out = [0u8; 16];
            for (lane, (a_chunk, b_chunk)) in a.as_chunks::<4>().0.iter().zip(b.as_chunks::<4>().0.iter()).enumerate() {
                let av = i32::from_le_bytes(
                    *a_chunk,
                );
                let bv = i32::from_le_bytes(
                    *b_chunk,
                );
                let a_shifted = $op(av, IMM) as u32;
                let b_shifted = $op(bv, IMM) as u32;
                out[lane * 2..lane * 2 + 2]
                    .copy_from_slice(&a_shifted.to_le_bytes()[..2]);
                out[0 + lane * 2 + 8
                    ..0 + lane * 2 + 8 + 2]
                    .copy_from_slice(&b_shifted.to_le_bytes()[..2]);}
            V128::from_bytes(out)
        }
    };
}

/// `vsllwil.w.h`
#[inline(always)]
pub fn vsllwil_w_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let raw = u16::from_le_bytes(*bytes_chunk);
        let value = (raw as i16) as i128;
        let widened = value << IMM;
        out_chunk.copy_from_slice(&(widened as i32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsllwil.wu.hu`
#[inline(always)]
pub fn vsllwil_wu_hu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let raw = u16::from_le_bytes(*bytes_chunk);
        let value = (raw as u16) as i128;
        let widened = value << IMM;
        out_chunk.copy_from_slice(&(widened as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

narrow_logical_h_w!(
    /// `vsrlni.h.w`
    vsrlni_h_w,
    srl_u32
);

narrow_logical_h_w!(
    /// `vsrlrni.h.w`
    vsrlrni_h_w,
    srlr_u32
);

narrow_arith_h_w!(
    /// `vsrani.h.w`
    vsrani_h_w,
    sra_i32
);

narrow_arith_h_w!(
    /// `vsrarni.h.w`
    vsrarni_h_w,
    srar_i32
);

narrow_logical_h_w!(
    /// `vssrlni.h.w`
    vssrlni_h_w,
    ssrl_signed_u32
);

narrow_logical_h_w!(
    /// `vssrlni.hu.w`
    vssrlni_hu_w,
    ssrl_unsigned_u32
);

narrow_logical_h_w!(
    /// `vssrlrni.h.w`
    vssrlrni_h_w,
    ssrlr_signed_u32
);

narrow_logical_h_w!(
    /// `vssrlrni.hu.w`
    vssrlrni_hu_w,
    ssrlr_unsigned_u32
);

narrow_arith_h_w!(
    /// `vssrani.h.w`
    vssrani_h_w,
    ssra_signed_i32
);

narrow_arith_h_w!(
    /// `vssrani.hu.w`
    vssrani_hu_w,
    ssra_unsigned_i32
);

narrow_arith_h_w!(
    /// `vssrarni.h.w`
    vssrarni_h_w,
    ssrar_signed_i32
);

narrow_arith_h_w!(
    /// `vssrarni.hu.w`
    vssrarni_hu_w,
    ssrar_unsigned_i32
);

#[inline(always)]
fn srl_u64(v: u64, imm: u32) -> u64 {
    v >> imm
}

#[inline(always)]
fn srlr_u64(v: u64, imm: u32) -> u64 {
    if imm == 0 {
        v
    } else {
        (v >> imm) + ((v >> (imm - 1)) & 1)
    }
}

#[inline(always)]
fn ssrl_signed_u64(v: u64, imm: u32) -> u64 {
    (v >> imm).min(i32::MAX as u64)
}

#[inline(always)]
fn ssrl_unsigned_u64(v: u64, imm: u32) -> u64 {
    (v >> imm).min(u32::MAX as u64)
}

#[inline(always)]
fn ssrlr_signed_u64(v: u64, imm: u32) -> u64 {
    srlr_u64(v, imm).min(i32::MAX as u64)
}

#[inline(always)]
fn ssrlr_unsigned_u64(v: u64, imm: u32) -> u64 {
    srlr_u64(v, imm).min(u32::MAX as u64)
}

#[inline(always)]
fn sra_i64(v: i64, imm: u32) -> i64 {
    v >> imm
}

#[inline(always)]
fn srar_i64(v: i64, imm: u32) -> i64 {
    if imm == 0 {
        v
    } else {
        (v >> imm).wrapping_add((v >> (imm - 1)) & 1)
    }
}

#[inline(always)]
fn ssra_signed_i64(v: i64, imm: u32) -> i64 {
    (v >> imm).clamp(i32::MIN as i64, i32::MAX as i64)
}

#[inline(always)]
fn ssra_unsigned_i64(v: i64, imm: u32) -> i64 {
    (v >> imm).clamp(0, u32::MAX as i64)
}

#[inline(always)]
fn ssrar_signed_i64(v: i64, imm: u32) -> i64 {
    srar_i64(v, imm).clamp(i32::MIN as i64, i32::MAX as i64)
}

#[inline(always)]
fn ssrar_unsigned_i64(v: i64, imm: u32) -> i64 {
    srar_i64(v, imm).clamp(0, u32::MAX as i64)
}

macro_rules! narrow_logical_w_d {
    ($(#[$meta:meta])* $name:ident, $op:path) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128, b: V128) -> V128 {
            const { assert!(IMM <= 63, "immediate out of range") };
            let a = a.to_bytes();
            let b = b.to_bytes();
            let mut out = [0u8; 16];
            for (lane, (a_chunk, b_chunk)) in a.as_chunks::<8>().0.iter().zip(b.as_chunks::<8>().0.iter()).enumerate() {
                let av = u64::from_le_bytes(
                    *a_chunk,
                );
                let bv = u64::from_le_bytes(
                    *b_chunk,
                );
                let a_shifted = $op(av, IMM) as u64;
                let b_shifted = $op(bv, IMM) as u64;
                out[lane * 4..lane * 4 + 4]
                    .copy_from_slice(&a_shifted.to_le_bytes()[..4]);
                out[0 + lane * 4 + 8
                    ..0 + lane * 4 + 8 + 4]
                    .copy_from_slice(&b_shifted.to_le_bytes()[..4]);}
            V128::from_bytes(out)
        }
    };
}

macro_rules! narrow_arith_w_d {
    ($(#[$meta:meta])* $name:ident, $op:path) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128, b: V128) -> V128 {
            const { assert!(IMM <= 63, "immediate out of range") };
            let a = a.to_bytes();
            let b = b.to_bytes();
            let mut out = [0u8; 16];
            for (lane, (a_chunk, b_chunk)) in a.as_chunks::<8>().0.iter().zip(b.as_chunks::<8>().0.iter()).enumerate() {
                let av = i64::from_le_bytes(
                    *a_chunk,
                );
                let bv = i64::from_le_bytes(
                    *b_chunk,
                );
                let a_shifted = $op(av, IMM) as u64;
                let b_shifted = $op(bv, IMM) as u64;
                out[lane * 4..lane * 4 + 4]
                    .copy_from_slice(&a_shifted.to_le_bytes()[..4]);
                out[0 + lane * 4 + 8
                    ..0 + lane * 4 + 8 + 4]
                    .copy_from_slice(&b_shifted.to_le_bytes()[..4]);}
            V128::from_bytes(out)
        }
    };
}

/// `vsllwil.d.w`
#[inline(always)]
pub fn vsllwil_d_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let raw = u32::from_le_bytes(*bytes_chunk);
        let value = (raw as i32) as i128;
        let widened = value << IMM;
        out_chunk.copy_from_slice(&(widened as i64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsllwil.du.wu`
#[inline(always)]
pub fn vsllwil_du_wu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let raw = u32::from_le_bytes(*bytes_chunk);
        let value = (raw as u32) as i128;
        let widened = value << IMM;
        out_chunk.copy_from_slice(&(widened as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

narrow_logical_w_d!(
    /// `vsrlni.w.d`
    vsrlni_w_d,
    srl_u64
);

narrow_logical_w_d!(
    /// `vsrlrni.w.d`
    vsrlrni_w_d,
    srlr_u64
);

narrow_arith_w_d!(
    /// `vsrani.w.d`
    vsrani_w_d,
    sra_i64
);

narrow_arith_w_d!(
    /// `vsrarni.w.d`
    vsrarni_w_d,
    srar_i64
);

narrow_logical_w_d!(
    /// `vssrlni.w.d`
    vssrlni_w_d,
    ssrl_signed_u64
);

narrow_logical_w_d!(
    /// `vssrlni.wu.d`
    vssrlni_wu_d,
    ssrl_unsigned_u64
);

narrow_logical_w_d!(
    /// `vssrlrni.w.d`
    vssrlrni_w_d,
    ssrlr_signed_u64
);

narrow_logical_w_d!(
    /// `vssrlrni.wu.d`
    vssrlrni_wu_d,
    ssrlr_unsigned_u64
);

narrow_arith_w_d!(
    /// `vssrani.w.d`
    vssrani_w_d,
    ssra_signed_i64
);

narrow_arith_w_d!(
    /// `vssrani.wu.d`
    vssrani_wu_d,
    ssra_unsigned_i64
);

narrow_arith_w_d!(
    /// `vssrarni.w.d`
    vssrarni_w_d,
    ssrar_signed_i64
);

narrow_arith_w_d!(
    /// `vssrarni.wu.d`
    vssrarni_wu_d,
    ssrar_unsigned_i64
);

#[inline(always)]
fn srl_u128(v: u128, imm: u32) -> u128 {
    v >> imm
}

#[inline(always)]
fn srlr_u128(v: u128, imm: u32) -> u128 {
    if imm == 0 {
        v
    } else {
        (v >> imm) + ((v >> (imm - 1)) & 1)
    }
}

#[inline(always)]
fn ssrl_signed_u128(v: u128, imm: u32) -> u128 {
    (v >> imm).min(i64::MAX as u128)
}

#[inline(always)]
fn ssrl_unsigned_u128(v: u128, imm: u32) -> u128 {
    (v >> imm).min(u64::MAX as u128)
}

#[inline(always)]
fn ssrlr_signed_u128(v: u128, imm: u32) -> u128 {
    srlr_u128(v, imm).min(i64::MAX as u128)
}

#[inline(always)]
fn ssrlr_unsigned_u128(v: u128, imm: u32) -> u128 {
    srlr_u128(v, imm).min(u64::MAX as u128)
}

#[inline(always)]
fn sra_i128(v: i128, imm: u32) -> i128 {
    v >> imm
}

#[inline(always)]
fn srar_i128(v: i128, imm: u32) -> i128 {
    if imm == 0 {
        v
    } else {
        (v >> imm).wrapping_add((v >> (imm - 1)) & 1)
    }
}

#[inline(always)]
fn ssra_signed_i128(v: i128, imm: u32) -> i128 {
    (v >> imm).clamp(i64::MIN as i128, i64::MAX as i128)
}

#[inline(always)]
fn ssra_unsigned_i128(v: i128, imm: u32) -> i128 {
    (v >> imm).clamp(0, u64::MAX as i128)
}

#[inline(always)]
fn ssrar_signed_i128(v: i128, imm: u32) -> i128 {
    srar_i128(v, imm).clamp(i64::MIN as i128, i64::MAX as i128)
}

#[inline(always)]
fn ssrar_unsigned_i128(v: i128, imm: u32) -> i128 {
    srar_i128(v, imm).clamp(0, u64::MAX as i128)
}

macro_rules! narrow_logical_d_q {
    ($(#[$meta:meta])* $name:ident, $op:path) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128, b: V128) -> V128 {
            const { assert!(IMM <= 127, "immediate out of range") };
            let a = a.to_bytes();
            let b = b.to_bytes();
            let mut out = [0u8; 16];
        let av = u128::from_le_bytes(a[0..16].try_into().unwrap());
        let bv = u128::from_le_bytes(b[0..16].try_into().unwrap());
        let a_shifted = $op(av, IMM) as u128;
        let b_shifted = $op(bv, IMM) as u128;
        out[0..8].copy_from_slice(&a_shifted.to_le_bytes()[..8]);
        out[8..16].copy_from_slice(&b_shifted.to_le_bytes()[..8]);
            V128::from_bytes(out)
        }
    };
}

macro_rules! narrow_arith_d_q {
    ($(#[$meta:meta])* $name:ident, $op:path) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128, b: V128) -> V128 {
            const { assert!(IMM <= 127, "immediate out of range") };
            let a = a.to_bytes();
            let b = b.to_bytes();
            let mut out = [0u8; 16];
        let av = i128::from_le_bytes(a[0..16].try_into().unwrap());
        let bv = i128::from_le_bytes(b[0..16].try_into().unwrap());
        let a_shifted = $op(av, IMM) as u128;
        let b_shifted = $op(bv, IMM) as u128;
        out[0..8].copy_from_slice(&a_shifted.to_le_bytes()[..8]);
        out[8..16].copy_from_slice(&b_shifted.to_le_bytes()[..8]);
            V128::from_bytes(out)
        }
    };
}

narrow_logical_d_q!(
    /// `vsrlni.d.q`
    vsrlni_d_q,
    srl_u128
);

narrow_logical_d_q!(
    /// `vsrlrni.d.q`
    vsrlrni_d_q,
    srlr_u128
);

narrow_arith_d_q!(
    /// `vsrani.d.q`
    vsrani_d_q,
    sra_i128
);

narrow_arith_d_q!(
    /// `vsrarni.d.q`
    vsrarni_d_q,
    srar_i128
);

narrow_logical_d_q!(
    /// `vssrlni.d.q`
    vssrlni_d_q,
    ssrl_signed_u128
);

narrow_logical_d_q!(
    /// `vssrlni.du.q`
    vssrlni_du_q,
    ssrl_unsigned_u128
);

narrow_logical_d_q!(
    /// `vssrlrni.d.q`
    vssrlrni_d_q,
    ssrlr_signed_u128
);

narrow_logical_d_q!(
    /// `vssrlrni.du.q`
    vssrlrni_du_q,
    ssrlr_unsigned_u128
);

narrow_arith_d_q!(
    /// `vssrani.d.q`
    vssrani_d_q,
    ssra_signed_i128
);

narrow_arith_d_q!(
    /// `vssrani.du.q`
    vssrani_du_q,
    ssra_unsigned_i128
);

narrow_arith_d_q!(
    /// `vssrarni.d.q`
    vssrarni_d_q,
    ssrar_signed_i128
);

narrow_arith_d_q!(
    /// `vssrarni.du.q`
    vssrarni_du_q,
    ssrar_unsigned_i128
);
