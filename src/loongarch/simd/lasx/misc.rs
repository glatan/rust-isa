use super::V256;

// Miscellaneous vector instructions.

/// `xvbsll.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbsll(a: V256, shift: u32) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let shift = (shift & 15) as usize;
    for half in 0..2 {
        for lane in shift..16 {
            out[half * 16 + lane] = bytes[half * 16 + lane - shift];
        }
    }
    V256::from_bytes(out)
}

/// `xvbsrl.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbsrl(a: V256, shift: u32) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let shift = (shift & 15) as usize;
    for half in 0..2 {
        for lane in 0..16 - shift {
            out[half * 16 + lane] = bytes[half * 16 + lane + shift];
        }
    }
    V256::from_bytes(out)
}

/// `xvseteqz.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvseteqz(a: V256) -> bool {
    let bytes = a.to_bytes();
    bytes.iter().all(|&b| b == 0)
}

/// `xvsetnez.v`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetnez(a: V256) -> bool {
    let bytes = a.to_bytes();
    bytes.iter().any(|&b| b != 0)
}

/// `xvsetanyeqz.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetanyeqz_b(a: V256) -> bool {
    let bytes = a.to_bytes();
    for lane in 0..32 {
        let off = lane * 1;
        if u8::from_le_bytes(bytes[off..off + 1].try_into().unwrap()) == 0 {
            return true;
        }
    }
    false
}

/// `xvsetanyeqz.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetanyeqz_h(a: V256) -> bool {
    let bytes = a.to_bytes();
    for lane in 0..16 {
        let off = lane * 2;
        if u16::from_le_bytes(bytes[off..off + 2].try_into().unwrap()) == 0 {
            return true;
        }
    }
    false
}

/// `xvsetanyeqz.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetanyeqz_w(a: V256) -> bool {
    let bytes = a.to_bytes();
    for lane in 0..8 {
        let off = lane * 4;
        if u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap()) == 0 {
            return true;
        }
    }
    false
}

/// `xvsetanyeqz.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetanyeqz_d(a: V256) -> bool {
    let bytes = a.to_bytes();
    for lane in 0..4 {
        let off = lane * 8;
        if u64::from_le_bytes(bytes[off..off + 8].try_into().unwrap()) == 0 {
            return true;
        }
    }
    false
}

/// `xvsetallnez.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetallnez_b(a: V256) -> bool {
    let bytes = a.to_bytes();
    for lane in 0..32 {
        let off = lane * 1;
        if u8::from_le_bytes(bytes[off..off + 1].try_into().unwrap()) == 0 {
            return false;
        }
    }
    true
}

/// `xvsetallnez.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetallnez_h(a: V256) -> bool {
    let bytes = a.to_bytes();
    for lane in 0..16 {
        let off = lane * 2;
        if u16::from_le_bytes(bytes[off..off + 2].try_into().unwrap()) == 0 {
            return false;
        }
    }
    true
}

/// `xvsetallnez.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetallnez_w(a: V256) -> bool {
    let bytes = a.to_bytes();
    for lane in 0..8 {
        let off = lane * 4;
        if u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap()) == 0 {
            return false;
        }
    }
    true
}

/// `xvsetallnez.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsetallnez_d(a: V256) -> bool {
    let bytes = a.to_bytes();
    for lane in 0..4 {
        let off = lane * 8;
        if u64::from_le_bytes(bytes[off..off + 8].try_into().unwrap()) == 0 {
            return false;
        }
    }
    true
}

/// `xvsat.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat_b<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 7, "xvsat immediate is out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let max = (1i128 << IMM) - 1;
    let min = -max - 1;
    for lane in 0..32 {
        let off = lane * 1;
        let value = i8::from_le_bytes(bytes[off..off + 1].try_into().unwrap()) as i128;
        let clamped = value.clamp(min, max) as i8;
        out[off..off + 1].copy_from_slice(&clamped.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsat.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat_h<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 15, "xvsat immediate is out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let max = (1i128 << IMM) - 1;
    let min = -max - 1;
    for lane in 0..16 {
        let off = lane * 2;
        let value = i16::from_le_bytes(bytes[off..off + 2].try_into().unwrap()) as i128;
        let clamped = value.clamp(min, max) as i16;
        out[off..off + 2].copy_from_slice(&clamped.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsat.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat_w<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "xvsat immediate is out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let max = (1i128 << IMM) - 1;
    let min = -max - 1;
    for lane in 0..8 {
        let off = lane * 4;
        let value = i32::from_le_bytes(bytes[off..off + 4].try_into().unwrap()) as i128;
        let clamped = value.clamp(min, max) as i32;
        out[off..off + 4].copy_from_slice(&clamped.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsat.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat_d<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 63, "xvsat immediate is out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let max = (1i128 << IMM) - 1;
    let min = -max - 1;
    for lane in 0..4 {
        let off = lane * 8;
        let value = i64::from_le_bytes(bytes[off..off + 8].try_into().unwrap()) as i128;
        let clamped = value.clamp(min, max) as i64;
        out[off..off + 8].copy_from_slice(&clamped.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsat.bu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat_bu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 7, "xvsat immediate is out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let max = (1i128 << (IMM + 1)) - 1;
    for lane in 0..32 {
        let off = lane * 1;
        let value = u8::from_le_bytes(bytes[off..off + 1].try_into().unwrap()) as i128;
        let clamped = value.clamp(0, max) as u8;
        out[off..off + 1].copy_from_slice(&clamped.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsat.hu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat_hu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 15, "xvsat immediate is out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let max = (1i128 << (IMM + 1)) - 1;
    for lane in 0..16 {
        let off = lane * 2;
        let value = u16::from_le_bytes(bytes[off..off + 2].try_into().unwrap()) as i128;
        let clamped = value.clamp(0, max) as u16;
        out[off..off + 2].copy_from_slice(&clamped.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsat.wu`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat_wu<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 31, "xvsat immediate is out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let max = (1i128 << (IMM + 1)) - 1;
    for lane in 0..8 {
        let off = lane * 4;
        let value = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap()) as i128;
        let clamped = value.clamp(0, max) as u32;
        out[off..off + 4].copy_from_slice(&clamped.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvsat.du`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvsat_du<const IMM: u32>(a: V256) -> V256 {
    const { assert!(IMM <= 63, "xvsat immediate is out of range") };
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    let max = (1i128 << (IMM + 1)) - 1;
    for lane in 0..4 {
        let off = lane * 8;
        let value = u64::from_le_bytes(bytes[off..off + 8].try_into().unwrap()) as i128;
        let clamped = value.clamp(0, max) as u64;
        out[off..off + 8].copy_from_slice(&clamped.to_le_bytes());
    }
    V256::from_bytes(out)
}

/// `xvbitseli.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvbitseli_b<const IMM: u8>(a: V256, b: V256) -> V256 {
    const { assert!(IMM <= 255, "immediate must fit in u8") };
    let bytes = a.to_bytes();
    let other = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..32 {
        out[lane] = (bytes[lane] & IMM) | (!bytes[lane] & other[lane]);
    }
    V256::from_bytes(out)
}

/// `xvshuf4i.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf4i_b<const IMM: u8>(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for block in 0..8 {
        for lane in 0..4 {
            let sel = ((IMM >> (2 * lane)) & 3) as usize;
            let dst = (block * 4 + lane) * 1;
            let src = (block * 4 + sel) * 1;
            out[dst..dst + 1].copy_from_slice(&bytes[src..src + 1]);
        }
    }
    V256::from_bytes(out)
}

/// `xvshuf4i.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf4i_h<const IMM: u8>(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for block in 0..4 {
        for lane in 0..4 {
            let sel = ((IMM >> (2 * lane)) & 3) as usize;
            let dst = (block * 4 + lane) * 2;
            let src = (block * 4 + sel) * 2;
            out[dst..dst + 2].copy_from_slice(&bytes[src..src + 2]);
        }
    }
    V256::from_bytes(out)
}

/// `xvshuf4i.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf4i_w<const IMM: u8>(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for block in 0..2 {
        for lane in 0..4 {
            let sel = ((IMM >> (2 * lane)) & 3) as usize;
            let dst = (block * 4 + lane) * 4;
            let src = (block * 4 + sel) * 4;
            out[dst..dst + 4].copy_from_slice(&bytes[src..src + 4]);
        }
    }
    V256::from_bytes(out)
}

/// `xvshuf4i.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvshuf4i_d<const IMM: u8>(a: V256, b: V256) -> V256 {
    let left = a.to_bytes();
    let right = b.to_bytes();
    let mut out = [0u8; 32];
    for half in 0..2 {
        for lane in 0..2 {
            let field = (IMM >> (2 * lane)) & 3;
            let source = if field & 2 != 0 { &right } else { &left };
            let index = half * 2 + (field & 1) as usize;
            let dst = (half * 2 + lane) * 8;
            out[dst..dst + 8].copy_from_slice(&source[index * 8..index * 8 + 8]);
        }
    }
    V256::from_bytes(out)
}

/// `xvpermi.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpermi_w<const IMM: u8>(a: V256, b: V256) -> V256 {
    let left = a.to_bytes();
    let right = b.to_bytes();
    let mut out = [0u8; 32];
    for half in 0..2 {
        for lane in 0..4 {
            let index = half * 4 + ((IMM >> (2 * lane)) & 3) as usize;
            let source = if lane < 2 { &right } else { &left };
            let dst = (half * 4 + lane) * 4;
            out[dst..dst + 4].copy_from_slice(&source[index * 4..index * 4 + 4]);
        }
    }
    V256::from_bytes(out)
}

/// `xvpermi.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpermi_d<const IMM: u8>(a: V256) -> V256 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..4 {
        let index = ((IMM >> (2 * lane)) & 3) as usize;
        out[lane * 8..lane * 8 + 8].copy_from_slice(&bytes[index * 8..index * 8 + 8]);
    }
    V256::from_bytes(out)
}

/// `xvpermi.q`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvpermi_q<const IMM: u8>(a: V256, b: V256) -> V256 {
    let left = a.to_bytes();
    let right = b.to_bytes();
    let mut out = [0u8; 32];
    for lane in 0..2 {
        let field = (IMM >> (2 * lane)) & 3;
        let source = if field & 2 != 0 { &left } else { &right };
        let index = (field & 1) as usize;
        out[lane * 16..lane * 16 + 16].copy_from_slice(&source[index * 16..index * 16 + 16]);
    }
    V256::from_bytes(out)
}

/// `xvextrins.b`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvextrins_b<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = a.to_bytes();
    let other = b.to_bytes();
    let index = IMM as usize;
    let dst = ((index >> 4) & 15) * 1;
    let src = (index & 15) * 1;
    for half in 0..2 {
        let base = half * 16;
        out[base + dst..base + dst + 1].copy_from_slice(&other[base + src..base + src + 1]);
    }
    V256::from_bytes(out)
}

/// `xvextrins.h`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvextrins_h<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = a.to_bytes();
    let other = b.to_bytes();
    let index = IMM as usize;
    let dst = ((index >> 4) & 7) * 2;
    let src = (index & 7) * 2;
    for half in 0..2 {
        let base = half * 16;
        out[base + dst..base + dst + 2].copy_from_slice(&other[base + src..base + src + 2]);
    }
    V256::from_bytes(out)
}

/// `xvextrins.w`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvextrins_w<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = a.to_bytes();
    let other = b.to_bytes();
    let index = IMM as usize;
    let dst = ((index >> 4) & 3) * 4;
    let src = (index & 3) * 4;
    for half in 0..2 {
        let base = half * 16;
        out[base + dst..base + dst + 4].copy_from_slice(&other[base + src..base + src + 4]);
    }
    V256::from_bytes(out)
}

/// `xvextrins.d`
///
/// # Safety
///
/// The caller must enable LASX.
#[target_feature(enable = "lasx")]
#[inline]
pub unsafe fn xvextrins_d<const IMM: u8>(a: V256, b: V256) -> V256 {
    let mut out = a.to_bytes();
    let other = b.to_bytes();
    let index = IMM as usize;
    let dst = ((index >> 4) & 1) * 8;
    let src = (index & 1) * 8;
    for half in 0..2 {
        let base = half * 16;
        out[base + dst..base + dst + 8].copy_from_slice(&other[base + src..base + src + 8]);
    }
    V256::from_bytes(out)
}
