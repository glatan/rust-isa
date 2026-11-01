//! SVE prefetch and first-fault load instructions.

macro_rules! prefetch {
    ($(#[$meta:meta])* $name:ident, $mnemonic:literal) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The address must be valid for a prefetch hint.
        #[target_feature(enable = "sve")]
        #[inline]
        pub unsafe fn $name(base: *const u8, offset: i64) {
            let addr = base.wrapping_offset(offset as isize);
            unsafe {
                core::arch::asm!(
                    concat!("prfm ", $mnemonic, ", [{addr}]"),
                    addr = in(reg) addr,
                    options(nostack, preserves_flags)
                );
            }
        }
    };
}

prefetch!(
    /// `prfb` scalar plus immediate form.
    prfb_imm,
    "pldl1keep"
);
prefetch!(
    /// `prfb` scalar plus scalar form.
    prfb_scalar,
    "pldl1keep"
);
prefetch!(
    /// `prfd` scalar plus immediate form.
    prfd_imm,
    "pldl1keep"
);
prefetch!(
    /// `prfd` scalar plus scalar form.
    prfd_scalar,
    "pldl1keep"
);
prefetch!(
    /// `prfh` scalar plus immediate form.
    prfh_imm,
    "pldl1keep"
);
prefetch!(
    /// `prfh` scalar plus scalar form.
    prfh_scalar,
    "pldl1keep"
);
prefetch!(
    /// `prfw` scalar plus immediate form.
    prfw_imm,
    "pldl1keep"
);
prefetch!(
    /// `prfw` scalar plus scalar form.
    prfw_scalar,
    "pldl1keep"
);

macro_rules! prefetch_vec {
    ($(#[$meta:meta])* $name:ident, $mnemonic:literal) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The addresses must be valid for a prefetch hint.
        #[target_feature(enable = "sve")]
        #[inline]
        pub unsafe fn $name(base: *const u8, offsets: &[i64]) {
            for &offset in offsets {
                let addr = base.wrapping_offset(offset as isize);
                unsafe {
                    core::arch::asm!(
                        concat!("prfm ", $mnemonic, ", [{addr}]"),
                        addr = in(reg) addr,
                        options(nostack, preserves_flags)
                    );
                }
            }
        }
    };
}

prefetch_vec!(
    /// `prfb` scalar plus vector form.
    prfb_vector,
    "pldl1keep"
);
prefetch_vec!(
    /// `prfd` scalar plus vector form.
    prfd_vector,
    "pldl1keep"
);
prefetch_vec!(
    /// `prfh` scalar plus vector form.
    prfh_vector,
    "pldl1keep"
);
prefetch_vec!(
    /// `prfw` scalar plus vector form.
    prfw_vector,
    "pldl1keep"
);

macro_rules! prefetch_vec_imm {
    ($(#[$meta:meta])* $name:ident, $mnemonic:literal) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The addresses must be valid for a prefetch hint.
        #[target_feature(enable = "sve")]
        #[inline]
        pub unsafe fn $name(offsets: &[i64], imm: i64) {
            for &offset in offsets {
                let addr = (offset as *const u8).wrapping_offset(imm as isize);
                unsafe {
                    core::arch::asm!(
                        concat!("prfm ", $mnemonic, ", [{addr}]"),
                        addr = in(reg) addr,
                        options(nostack, preserves_flags)
                    );
                }
            }
        }
    };
}

prefetch_vec_imm!(
    /// `prfb` vector plus immediate form.
    prfb_vec_imm,
    "pldl1keep"
);
prefetch_vec_imm!(
    /// `prfd` vector plus immediate form.
    prfd_vec_imm,
    "pldl1keep"
);
prefetch_vec_imm!(
    /// `prfh` vector plus immediate form.
    prfh_vec_imm,
    "pldl1keep"
);
prefetch_vec_imm!(
    /// `prfw` vector plus immediate form.
    prfw_vec_imm,
    "pldl1keep"
);

macro_rules! ldff1 {
    ($(#[$meta:meta])* $name:ident, $ty:ty, $out:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable SVE and the accessed range must be readable.
        #[target_feature(enable = "sve")]
        #[inline]
        pub unsafe fn $name(base: *const $ty, offset: i64, out: &mut [$out]) {
            for lane in 0..out.len() {
                out[lane] = unsafe { *base.offset(offset as isize + lane as isize) } as $out;
            }
        }
    };
}

ldff1!(
    /// `ldff1sb` scalar plus scalar form.
    ldff1sb_scalar,
    i8,
    i32
);
ldff1!(
    /// `ldff1sh` scalar plus scalar form.
    ldff1sh_scalar,
    i16,
    i32
);
ldff1!(
    /// `ldff1sw` scalar plus scalar form.
    ldff1sw_scalar,
    i32,
    i32
);

macro_rules! ldff1_vec {
    ($(#[$meta:meta])* $name:ident, $ty:ty, $out:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable SVE and each accessed address must be readable.
        #[target_feature(enable = "sve")]
        #[inline]
        pub unsafe fn $name(base: *const $ty, offsets: &[i64], out: &mut [$out]) {
            assert_eq!(offsets.len(), out.len(), "SVE slices must have the same length");
            for lane in 0..out.len() {
                out[lane] = unsafe { *base.offset(offsets[lane] as isize) } as $out;
            }
        }
    };
}

ldff1_vec!(
    /// `ldff1sb` scalar plus vector form.
    ldff1sb_vector,
    i8,
    i32
);
ldff1_vec!(
    /// `ldff1sh` scalar plus vector form.
    ldff1sh_vector,
    i16,
    i32
);
ldff1_vec!(
    /// `ldff1sw` scalar plus vector form.
    ldff1sw_vector,
    i32,
    i32
);

macro_rules! ldff1_vec_imm {
    ($(#[$meta:meta])* $name:ident, $ty:ty, $out:ty) => {
        $(#[$meta])*
        ///
        /// # Safety
        ///
        /// The caller must enable SVE and each base address plus `imm` must be readable.
        #[target_feature(enable = "sve")]
        #[inline]
        pub unsafe fn $name(offsets: &[i64], imm: i64, out: &mut [$out]) {
            assert_eq!(offsets.len(), out.len(), "SVE slices must have the same length");
            for lane in 0..out.len() {
                let base = offsets[lane] as *const $ty;
                out[lane] = unsafe { *base.offset(imm as isize) } as $out;
            }
        }
    };
}

ldff1_vec_imm!(
    /// `ldff1sb` vector plus immediate form.
    ldff1sb_vec_imm,
    i8,
    i32
);
ldff1_vec_imm!(
    /// `ldff1sh` vector plus immediate form.
    ldff1sh_vec_imm,
    i16,
    i32
);
ldff1_vec_imm!(
    /// `ldff1sw` vector plus immediate form.
    ldff1sw_vec_imm,
    i32,
    i32
);
