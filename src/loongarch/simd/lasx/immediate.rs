//! LASX immediate-form instructions.

use super::V256;

/// `xvseqi.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseqi_b<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        if value == IMM as i8 {
            *out_elem = 0xff;
        }
    }
    V256::from_bytes(out)
}

/// `xvseqi.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseqi_h<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvseqi.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseqi_w<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvseqi.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseqi_d<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslei.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei_b<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        if value <= IMM as i8 {
            *out_elem = 0xff;
        }
    }
    V256::from_bytes(out)
}

/// `xvslei.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei_bu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        if value <= IMM as u8 {
            *out_elem = 0xff;
        }
    }
    V256::from_bytes(out)
}

/// `xvslei.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei_h<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslei.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei_hu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslei.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei_w<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslei.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei_wu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslei.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei_d<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslei.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslei_du<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslti.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti_b<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        if value < IMM as i8 {
            *out_elem = 0xff;
        }
    }
    V256::from_bytes(out)
}

/// `xvslti.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti_bu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        if value < IMM as u8 {
            *out_elem = 0xff;
        }
    }
    V256::from_bytes(out)
}

/// `xvslti.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti_h<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslti.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti_hu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslti.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti_w<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslti.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti_wu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslti.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti_d<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslti.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslti_du<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvslli.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslli_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.wrapping_shl(IMM) as u8;
    }
    V256::from_bytes(out)
}

/// `xvslli.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslli_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shl(IMM) as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslli.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslli_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shl(IMM) as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvslli.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvslli_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shl(IMM) as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrli.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrli_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.wrapping_shr(IMM) as u8;
    }
    V256::from_bytes(out)
}

/// `xvsrli.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrli_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrli.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrli_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrli.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrli_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrai.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrai_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        *out_elem = value.wrapping_shr(IMM) as u8;
    }
    V256::from_bytes(out)
}

/// `xvsrai.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrai_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrai.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrai_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrai.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrai_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.wrapping_shr(IMM) as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmaxi.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaxi_b<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        *out_elem = value.max(IMM as i8) as u8;
    }
    V256::from_bytes(out)
}

/// `xvmaxi.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaxi_bu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.max(IMM as u8) as u8;
    }
    V256::from_bytes(out)
}

/// `xvmaxi.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaxi_h<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as i16) as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmaxi.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaxi_hu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as u16) as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmaxi.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaxi_w<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as i32) as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmaxi.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaxi_wu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as u32) as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmaxi.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaxi_d<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as i64) as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmaxi.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmaxi_du<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.max(IMM as u64) as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmini.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmini_b<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        *out_elem = value.min(IMM as i8) as u8;
    }
    V256::from_bytes(out)
}

/// `xvmini.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmini_bu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.min(IMM as u8) as u8;
    }
    V256::from_bytes(out)
}

/// `xvmini.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmini_h<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = i16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as i16) as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmini.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmini_hu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as u16) as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmini.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmini_w<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = i32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as i32) as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmini.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmini_wu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as u32) as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmini.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmini_d<const IMM: i32>(a: V256) -> V256 {
    const { assert!(IMM >= -16 && IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = i64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as i64) as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvmini.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvmini_du<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM >= 0 && IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.min(IMM as u64) as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvrotri.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrotri_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = (value.rotate_right(IMM)) as u8;
    }
    V256::from_bytes(out)
}

/// `xvrotri.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrotri_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.rotate_right(IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvrotri.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrotri_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.rotate_right(IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvrotri.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvrotri_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value.rotate_right(IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitclri.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitclri_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = (value & !(1u8 << IMM)) as u8;
    }
    V256::from_bytes(out)
}

/// `xvbitclri.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitclri_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value & !(1u16 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitclri.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitclri_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value & !(1u32 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitclri.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitclri_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value & !(1u64 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitseti.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitseti_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = (value | (1u8 << IMM)) as u8;
    }
    V256::from_bytes(out)
}

/// `xvbitseti.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitseti_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value | (1u16 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitseti.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitseti_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value | (1u32 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitseti.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitseti_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value | (1u64 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitrevi.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitrevi_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = (value ^ (1u8 << IMM)) as u8;
    }
    V256::from_bytes(out)
}

/// `xvbitrevi.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitrevi_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value ^ (1u16 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitrevi.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitrevi_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value ^ (1u32 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitrevi.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitrevi_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&(value ^ (1u64 << IMM)).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvandi.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvandi_b<const IMM: u8>(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (out_elem, bytes_elem) in out.iter_mut().zip(bytes.iter()) {
        *out_elem = *bytes_elem & IMM;
    }
    V256::from_bytes(out)
}

/// `xvori.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvori_b<const IMM: u8>(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (out_elem, bytes_elem) in out.iter_mut().zip(bytes.iter()) {
        *out_elem = *bytes_elem | IMM;
    }
    V256::from_bytes(out)
}

/// `xvxori.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvxori_b<const IMM: u8>(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (out_elem, bytes_elem) in out.iter_mut().zip(bytes.iter()) {
        *out_elem = *bytes_elem ^ IMM;
    }
    V256::from_bytes(out)
}

/// `xvnori.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvnori_b<const IMM: u8>(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (out_elem, bytes_elem) in out.iter_mut().zip(bytes.iter()) {
        *out_elem = !(*bytes_elem | IMM);
    }
    V256::from_bytes(out)
}

/// `xvaddi.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddi_bu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.saturating_add(IMM as u8);
    }
    V256::from_bytes(out)
}

/// `xvaddi.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddi_hu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_add(IMM as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvaddi.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddi_wu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_add(IMM as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvaddi.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvaddi_du<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_add(IMM as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsubi.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubi_bu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        *out_elem = value.saturating_sub(IMM as u8);
    }
    V256::from_bytes(out)
}

/// `xvsubi.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubi_hu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<2>().0.iter_mut())
    {
        let value = u16::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_sub(IMM as u16).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsubi.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubi_wu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0.iter_mut())
    {
        let value = u32::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_sub(IMM as u32).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsubi.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsubi_du<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (chunk, out_chunk) in bytes
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<8>().0.iter_mut())
    {
        let value = u64::from_le_bytes(*chunk);
        out_chunk.copy_from_slice(&value.saturating_sub(IMM as u64).to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsrlri.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlri_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as u8;
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        *out_elem = rounded as u8;
    }
    V256::from_bytes(out)
}

/// `xvsrlri.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlri_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvsrlri.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlri_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvsrlri.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrlri_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvsrari.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrari_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 7, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for (bytes_elem, out_elem) in bytes.iter().zip(out.iter_mut()) {
        let value = *bytes_elem as i8;
        let rounded = if IMM == 0 {
            value
        } else {
            (value >> IMM) + ((value >> (IMM - 1)) & 1)
        };
        *out_elem = rounded as u8;
    }
    V256::from_bytes(out)
}

/// `xvsrari.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrari_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 15, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvsrari.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrari_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}

/// `xvsrari.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsrari_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 63, "immediate out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
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
    V256::from_bytes(out)
}
