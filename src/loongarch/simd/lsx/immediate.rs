//! LSX immediate-form instructions.

use super::V128;

/// `vseqi.b`
#[inline(always)]
pub fn vseqi_b<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        if value == IMM as i8 {
            *out_elem = 0xff;
        }
    }
    V128::from_bytes(out)
}

/// `vseqi.h`
#[inline(always)]
pub fn vseqi_h<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        if value == IMM as i16 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vseqi.w`
#[inline(always)]
pub fn vseqi_w<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        if value == IMM as i32 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vseqi.d`
#[inline(always)]
pub fn vseqi_d<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        if value == IMM as i64 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslei.b`
#[inline(always)]
pub fn vslei_b<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        if value <= IMM as i8 {
            *out_elem = 0xff;
        }
    }
    V128::from_bytes(out)
}

/// `vslei.bu`
#[inline(always)]
pub fn vslei_bu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        if value <= IMM as u8 {
            *out_elem = 0xff;
        }
    }
    V128::from_bytes(out)
}

/// `vslei.h`
#[inline(always)]
pub fn vslei_h<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        if value <= IMM as i16 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslei.hu`
#[inline(always)]
pub fn vslei_hu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        if value <= IMM as u16 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslei.w`
#[inline(always)]
pub fn vslei_w<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        if value <= IMM as i32 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslei.wu`
#[inline(always)]
pub fn vslei_wu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        if value <= IMM as u32 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslei.d`
#[inline(always)]
pub fn vslei_d<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        if value <= IMM as i64 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslei.du`
#[inline(always)]
pub fn vslei_du<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        if value <= IMM as u64 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslti.b`
#[inline(always)]
pub fn vslti_b<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        if value < IMM as i8 {
            *out_elem = 0xff;
        }
    }
    V128::from_bytes(out)
}

/// `vslti.bu`
#[inline(always)]
pub fn vslti_bu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        if value < IMM as u8 {
            *out_elem = 0xff;
        }
    }
    V128::from_bytes(out)
}

/// `vslti.h`
#[inline(always)]
pub fn vslti_h<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        if value < IMM as i16 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslti.hu`
#[inline(always)]
pub fn vslti_hu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        if value < IMM as u16 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslti.w`
#[inline(always)]
pub fn vslti_w<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        if value < IMM as i32 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslti.wu`
#[inline(always)]
pub fn vslti_wu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        if value < IMM as u32 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslti.d`
#[inline(always)]
pub fn vslti_d<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        if value < IMM as i64 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslti.du`
#[inline(always)]
pub fn vslti_du<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        if value < IMM as u64 {
            out_chunk.fill(0xff);
        }
    }
    V128::from_bytes(out)
}

/// `vslli.b`
#[inline(always)]
pub fn vslli_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.wrapping_shl(IMM) as u8;
    }
    V128::from_bytes(out)
}

/// `vslli.h`
#[inline(always)]
pub fn vslli_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shl(IMM) as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vslli.w`
#[inline(always)]
pub fn vslli_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shl(IMM) as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vslli.d`
#[inline(always)]
pub fn vslli_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shl(IMM) as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrli.b`
#[inline(always)]
pub fn vsrli_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.wrapping_shr(IMM) as u8;
    }
    V128::from_bytes(out)
}

/// `vsrli.h`
#[inline(always)]
pub fn vsrli_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrli.w`
#[inline(always)]
pub fn vsrli_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrli.d`
#[inline(always)]
pub fn vsrli_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrai.b`
#[inline(always)]
pub fn vsrai_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        *out_elem = value.wrapping_shr(IMM) as u8;
    }
    V128::from_bytes(out)
}

/// `vsrai.h`
#[inline(always)]
pub fn vsrai_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrai.w`
#[inline(always)]
pub fn vsrai_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrai.d`
#[inline(always)]
pub fn vsrai_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmaxi.b`
#[inline(always)]
pub fn vmaxi_b<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        *out_elem = value.max(IMM as i8) as u8;
    }
    V128::from_bytes(out)
}

/// `vmaxi.bu`
#[inline(always)]
pub fn vmaxi_bu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.max(IMM as u8) as u8;
    }
    V128::from_bytes(out)
}

/// `vmaxi.h`
#[inline(always)]
pub fn vmaxi_h<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as i16) as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmaxi.hu`
#[inline(always)]
pub fn vmaxi_hu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as u16) as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmaxi.w`
#[inline(always)]
pub fn vmaxi_w<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as i32) as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmaxi.wu`
#[inline(always)]
pub fn vmaxi_wu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as u32) as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmaxi.d`
#[inline(always)]
pub fn vmaxi_d<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as i64) as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmaxi.du`
#[inline(always)]
pub fn vmaxi_du<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as u64) as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmini.b`
#[inline(always)]
pub fn vmini_b<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        *out_elem = value.min(IMM as i8) as u8;
    }
    V128::from_bytes(out)
}

/// `vmini.bu`
#[inline(always)]
pub fn vmini_bu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.min(IMM as u8) as u8;
    }
    V128::from_bytes(out)
}

/// `vmini.h`
#[inline(always)]
pub fn vmini_h<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as i16) as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmini.hu`
#[inline(always)]
pub fn vmini_hu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as u16) as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmini.w`
#[inline(always)]
pub fn vmini_w<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as i32) as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmini.wu`
#[inline(always)]
pub fn vmini_wu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as u32) as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmini.d`
#[inline(always)]
pub fn vmini_d<const IMM: i32>(a: V128) -> V128 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as i64) as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vmini.du`
#[inline(always)]
pub fn vmini_du<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as u64) as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vrotri.b`
#[inline(always)]
pub fn vrotri_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = (value.rotate_right(IMM)) as u8;
    }
    V128::from_bytes(out)
}

/// `vrotri.h`
#[inline(always)]
pub fn vrotri_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.rotate_right(IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vrotri.w`
#[inline(always)]
pub fn vrotri_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.rotate_right(IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vrotri.d`
#[inline(always)]
pub fn vrotri_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.rotate_right(IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitclri.b`
#[inline(always)]
pub fn vbitclri_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = (value & !(1u8 << IMM)) as u8;
    }
    V128::from_bytes(out)
}

/// `vbitclri.h`
#[inline(always)]
pub fn vbitclri_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value & !(1u16 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitclri.w`
#[inline(always)]
pub fn vbitclri_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value & !(1u32 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitclri.d`
#[inline(always)]
pub fn vbitclri_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value & !(1u64 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitseti.b`
#[inline(always)]
pub fn vbitseti_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = (value | (1u8 << IMM)) as u8;
    }
    V128::from_bytes(out)
}

/// `vbitseti.h`
#[inline(always)]
pub fn vbitseti_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value | (1u16 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitseti.w`
#[inline(always)]
pub fn vbitseti_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value | (1u32 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitseti.d`
#[inline(always)]
pub fn vbitseti_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value | (1u64 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitrevi.b`
#[inline(always)]
pub fn vbitrevi_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = (value ^ (1u8 << IMM)) as u8;
    }
    V128::from_bytes(out)
}

/// `vbitrevi.h`
#[inline(always)]
pub fn vbitrevi_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value ^ (1u16 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitrevi.w`
#[inline(always)]
pub fn vbitrevi_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value ^ (1u32 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vbitrevi.d`
#[inline(always)]
pub fn vbitrevi_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value ^ (1u64 << IMM)).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vandi.b`
#[inline(always)]
pub fn vandi_b<const IMM: u8>(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (out_elem, bytes_elem) in out.iter_mut().zip(bytes.iter()) {
        *out_elem = *bytes_elem & IMM;
    }
    V128::from_bytes(out)
}

/// `vori.b`
#[inline(always)]
pub fn vori_b<const IMM: u8>(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (out_elem, bytes_elem) in out.iter_mut().zip(bytes.iter()) {
        *out_elem = *bytes_elem | IMM;
    }
    V128::from_bytes(out)
}

/// `vxori.b`
#[inline(always)]
pub fn vxori_b<const IMM: u8>(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (out_elem, bytes_elem) in out.iter_mut().zip(bytes.iter()) {
        *out_elem = *bytes_elem ^ IMM;
    }
    V128::from_bytes(out)
}

/// `vnori.b`
#[inline(always)]
pub fn vnori_b<const IMM: u8>(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (out_elem, bytes_elem) in out.iter_mut().zip(bytes.iter()) {
        *out_elem = !(*bytes_elem | IMM);
    }
    V128::from_bytes(out)
}

/// `vaddi.bu`
#[inline(always)]
pub fn vaddi_bu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.saturating_add(IMM as u8);
    }
    V128::from_bytes(out)
}

/// `vaddi.hu`
#[inline(always)]
pub fn vaddi_hu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_add(IMM as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vaddi.wu`
#[inline(always)]
pub fn vaddi_wu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_add(IMM as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vaddi.du`
#[inline(always)]
pub fn vaddi_du<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_add(IMM as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsubi.bu`
#[inline(always)]
pub fn vsubi_bu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.saturating_sub(IMM as u8);
    }
    V128::from_bytes(out)
}

/// `vsubi.hu`
#[inline(always)]
pub fn vsubi_hu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_sub(IMM as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsubi.wu`
#[inline(always)]
pub fn vsubi_wu<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_sub(IMM as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsubi.du`
#[inline(always)]
pub fn vsubi_du<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_sub(IMM as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrlri.b`
#[inline(always)]
pub fn vsrlri_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        *out_elem = rounded as u8;
    }
    V128::from_bytes(out)
}

/// `vsrlri.h`
#[inline(always)]
pub fn vsrlri_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        out_chunk.copy_from_slice(&(rounded as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrlri.w`
#[inline(always)]
pub fn vsrlri_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        out_chunk.copy_from_slice(&(rounded as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrlri.d`
#[inline(always)]
pub fn vsrlri_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        out_chunk.copy_from_slice(&(rounded as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrari.b`
#[inline(always)]
pub fn vsrari_b<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        *out_elem = rounded as u8;
    }
    V128::from_bytes(out)
}

/// `vsrari.h`
#[inline(always)]
pub fn vsrari_h<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        out_chunk.copy_from_slice(&(rounded as u16).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrari.w`
#[inline(always)]
pub fn vsrari_w<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        out_chunk.copy_from_slice(&(rounded as u32).to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vsrari.d`
#[inline(always)]
pub fn vsrari_d<const IMM: u32>(a: V128) -> V128 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        out_chunk.copy_from_slice(&(rounded as u64).to_le_bytes());
    }
    V128::from_bytes(out)
}
