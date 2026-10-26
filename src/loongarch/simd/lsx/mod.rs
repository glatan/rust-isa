//! LSX.
//!
//! LSX is part of the LoongArch baseline. The arithmetic operations are safe;
//! loads and stores take raw pointers and are `unsafe`. The values are
//! [`V128`]. Immediate operands are const generics unless documented
//! otherwise.
//!
//! Instruction behavior follows the [LoongArch Documentation](https://loongson.github.io/LoongArch-Documentation/).
//!
//! The build compiles the bundled C with clang.
//!
//! Instructions not listed in this module are not exposed.

//! <details>
//! <summary>Instruction examples</summary>
//!
//! # Behavior example
//!
//! ```no_run
//! use rust_isa::loongarch::simd::lsx::*;
//!
//! let v = V128::from_bytes([0; 16]);
//! assert_eq!(add(v, v).to_bytes(), [0; 16]);
//! ```
//!
//! # Instruction examples
//!
//! ```
//! use rust_isa::loongarch::simd::lsx::*;
//! use rust_isa::loongarch::simd::lsx::V128;
//! use rust_isa::loongarch::simd::lasx::V256;
//!
//! let v128 = V128::from_bytes([0; 16]);
//! let v256 = V256::from_bytes([0; 32]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//!
//! let _ = add(v128, v128);
//! let _ = add_i16(v128, v128);
//! let _ = add_i64(v128, v128);
//! let _ = add_i8(v128, v128);
//! let _ = and(v128, v128);
//! let _ = cmpeq_i32(v128, v128);
//! let _ = cmpgt_i32(v128, v128);
//! let _ = fadd(v128, v128);
//! let _ = fdiv(v128, v128);
//! let _ = fmul(v128, v128);
//! let _ = fsub(v128, v128);
//! let _ = mul(v128, v128);
//! let _ = mul_i16(v128, v128);
//! let _ = mul_i32(v128, v128);
//! let _ = or(v128, v128);
//! let _ = splat_i32(0);
//! let _ = sub_i16(v128, v128);
//! let _ = sub_i32(v128, v128);
//! let _ = sub_i64(v128, v128);
//! let _ = sub_i8(v128, v128);
//! let _ = unsafe { v128_load(storage.0.as_ptr().cast()) };
//! let _ = unsafe { v128_store(storage.0.as_mut_ptr().cast(), v128) };
//! let _ = vabs(v128);
//! let _ = vabs_b(v128);
//! let _ = vabs_d(v128);
//! let _ = vabs_h(v128);
//! let _ = vadd(v128, v128);
//! let _ = vadd_b(v128, v128);
//! let _ = vadd_d(v128, v128);
//! let _ = vadd_h(v128, v128);
//! let _ = vadd_w(v128, v128);
//! let _ = vand(v128, v128);
//! let _ = vavgr(v128, v128);
//! let _ = vavgr_b(v128, v128);
//! let _ = vavgr_bu(v128, v128);
//! let _ = vavgr_d(v128, v128);
//! let _ = vavgr_du(v128, v128);
//! let _ = vavgr_h(v128, v128);
//! let _ = vavgr_hu(v128, v128);
//! let _ = vavgr_w(v128, v128);
//! let _ = vavgr_wu(v128, v128);
//! let _ = vbitsel(v128, v128, v128);
//! let _ = vext(v128);
//! let _ = vexth(v128);
//! let _ = vexth_d_w(v128);
//! let _ = vexth_du_wu(v128);
//! let _ = vexth_h_b(v128);
//! let _ = vexth_hu_bu(v128);
//! let _ = vexth_q_d(v128);
//! let _ = vexth_qu_du(v128);
//! let _ = vexth_w_h(v128);
//! let _ = vexth_wu_hu(v128);
//! let _ = vextl(v128);
//! let _ = vextl_qu_du(v128);
//! let _ = vfadd(v128, v128);
//! let _ = vfadd_d(v128, v128);
//! let _ = vfadd_s(v128, v128);
//! let _ = vfcmp::<0>(v128, v128);
//! let _ = vfcmp_caf_d(v128, v128);
//! let _ = vfcmp_caf_s(v128, v128);
//! let _ = vfcmp_ceq_d(v128, v128);
//! let _ = vfcmp_ceq_s(v128, v128);
//! let _ = vfcmp_cle_d(v128, v128);
//! let _ = vfcmp_cle_s(v128, v128);
//! let _ = vfcmp_clt_d(v128, v128);
//! let _ = vfcmp_clt_s(v128, v128);
//! let _ = vfcmp_cne_d(v128, v128);
//! let _ = vfcmp_cne_s(v128, v128);
//! let _ = vfcmp_cor_d(v128, v128);
//! let _ = vfcmp_cor_s(v128, v128);
//! let _ = vfcmp_cueq_d(v128, v128);
//! let _ = vfcmp_cueq_s(v128, v128);
//! let _ = vfcmp_cule_d(v128, v128);
//! let _ = vfcmp_cule_s(v128, v128);
//! let _ = vfcmp_cult_d(v128, v128);
//! let _ = vfcmp_cult_s(v128, v128);
//! let _ = vfcmp_cun_d(v128, v128);
//! let _ = vfcmp_cun_s(v128, v128);
//! let _ = vfcmp_cune_d(v128, v128);
//! let _ = vfcmp_cune_s(v128, v128);
//! let _ = vfcmp_d::<0>(v128, v128);
//! let _ = vfcmp_saf_d(v128, v128);
//! let _ = vfcmp_saf_s(v128, v128);
//! let _ = vfcmp_seq_d(v128, v128);
//! let _ = vfcmp_seq_s(v128, v128);
//! let _ = vfcmp_sle_d(v128, v128);
//! let _ = vfcmp_sle_s(v128, v128);
//! let _ = vfcmp_slt_d(v128, v128);
//! let _ = vfcmp_slt_s(v128, v128);
//! let _ = vfcmp_sne_d(v128, v128);
//! let _ = vfcmp_sne_s(v128, v128);
//! let _ = vfcmp_sor_d(v128, v128);
//! let _ = vfcmp_sor_s(v128, v128);
//! let _ = vfcmp_sueq_d(v128, v128);
//! let _ = vfcmp_sueq_s(v128, v128);
//! let _ = vfcmp_sule_d(v128, v128);
//! let _ = vfcmp_sule_s(v128, v128);
//! let _ = vfcmp_sult_d(v128, v128);
//! let _ = vfcmp_sult_s(v128, v128);
//! let _ = vfcmp_sun_d(v128, v128);
//! let _ = vfcmp_sun_s(v128, v128);
//! let _ = vfcmp_sune_d(v128, v128);
//! let _ = vfcmp_sune_s(v128, v128);
//! let _ = vfcvt(v128, v128);
//! let _ = vfcvt_s_d(v128, v128);
//! let _ = vfcvth_d_s(v128);
//! let _ = vfcvtl_d_s(v128);
//! let _ = vfdiv(v128, v128);
//! let _ = vfdiv_d(v128, v128);
//! let _ = vfdiv_s(v128, v128);
//! let _ = vffint(v128);
//! let _ = vffint_d_l(v128);
//! let _ = vffint_d_lu(v128);
//! let _ = vffint_s_l(v128, v128);
//! let _ = vffint_s_w(v128);
//! let _ = vffint_s_wu(v128);
//! let _ = vffinth_d_w(v128);
//! let _ = vffintl_d_w(v128);
//! let _ = vfmax(v128, v128);
//! let _ = vfmax_d(v128, v128);
//! let _ = vfmax_s(v128, v128);
//! let _ = vfmin(v128, v128);
//! let _ = vfmin_d(v128, v128);
//! let _ = vfmin_s(v128, v128);
//! let _ = vfmul(v128, v128);
//! let _ = vfmul_d(v128, v128);
//! let _ = vfmul_s(v128, v128);
//! let _ = vfsub(v128, v128);
//! let _ = vfsub_d(v128, v128);
//! let _ = vfsub_s(v128, v128);
//! let _ = vftint(v128);
//! let _ = vftint_l_d(v128);
//! let _ = vftint_lu_d(v128);
//! let _ = vftint_w_d(v128, v128);
//! let _ = vftint_w_s(v128);
//! let _ = vftint_wu_s(v128);
//! let _ = vftintrm_l_d(v128);
//! let _ = vftintrm_w_d(v128, v128);
//! let _ = vftintrm_w_s(v128);
//! let _ = vftintrne_l_d(v128);
//! let _ = vftintrne_w_d(v128, v128);
//! let _ = vftintrne_w_s(v128);
//! let _ = vftintrp_l_d(v128);
//! let _ = vftintrp_w_d(v128, v128);
//! let _ = vftintrp_w_s(v128);
//! let _ = vftintrz_l_d(v128);
//! let _ = vftintrz_lu_d(v128);
//! let _ = vftintrz_w_d(v128, v128);
//! let _ = vftintrz_w_s(v128);
//! let _ = vftintrz_wu_s(v128);
//! let _ = unsafe { vld(storage.0.as_ptr().cast(), 0) };
//! let _ = vldi::<0>();
//! let _ = vmax(v128, v128);
//! let _ = vmin(v128, v128);
//! let _ = vmul(v128, v128);
//! let _ = vmul_h(v128, v128);
//! let _ = vmul_w(v128, v128);
//! let _ = vneg(v128);
//! let _ = vneg_b(v128);
//! let _ = vneg_d(v128);
//! let _ = vneg_h(v128);
//! let _ = vneg_w(v128);
//! let _ = vnor(v128, v128);
//! let _ = vor(v128, v128);
//! let _ = vpackev(v128, v128);
//! let _ = vpackod(v128, v128);
//! let _ = vrepl(0);
//! let _ = vrepl_b(0);
//! let _ = vrepl_d(0);
//! let _ = vrepl_h(0);
//! let _ = vrepl_w(0);
//! let _ = vreplve::<0>(v128);
//! let _ = vreplve_b::<0>(v128);
//! let _ = vreplve_d::<0>(v128);
//! let _ = vreplve_h::<0>(v128);
//! let _ = vreplve_w::<0>(v128);
//! let _ = vsat::<0>(v128);
//! let _ = vsat_b::<0>(v128);
//! let _ = vsat_bu::<0>(v128);
//! let _ = vsat_d::<0>(v128);
//! let _ = vsat_du::<0>(v128);
//! let _ = vsat_h::<0>(v128);
//! let _ = vsat_hu::<0>(v128);
//! let _ = vsat_w::<0>(v128);
//! let _ = vsat_wu::<0>(v128);
//! let _ = vseq(v128, v128);
//! let _ = vseqi(v128, 0);
//! let _ = vshuf(v128, v128, v128);
//! let _ = vshuf4i::<0>(v128);
//! let _ = vsle(v128, v128);
//! let _ = vslei(v128, 0);
//! let _ = vsll(v128, 0);
//! let _ = vslli::<0>(v128);
//! let _ = vslt(v128, v128);
//! let _ = vslti(v128, 0);
//! let _ = vsra(v128, 0);
//! let _ = vsrai::<0>(v128);
//! let _ = vsrl(v128, 0);
//! let _ = vsrli::<0>(v128);
//! let _ = vssr(v128, v128);
//! let _ = vssrarn_w_d::<0>(v128, v128);
//! let _ = vssrarn_wu_du::<0>(v128, v128);
//! let _ = vssrln_w_d::<0>(v128, v128);
//! let _ = vssrln_wu_du::<0>(v128, v128);
//! let _ = vssrlrn_w_d::<0>(v128, v128);
//! let _ = vssrlrn_wu_du::<0>(v128, v128);
//! let _ = unsafe { vst(storage.0.as_mut_ptr().cast(), 0, v128) };
//! let _ = vsub(v128, v128);
//! let _ = vsub_b(v128, v128);
//! let _ = vsub_d(v128, v128);
//! let _ = vsub_h(v128, v128);
//! let _ = vsub_w(v128, v128);
//! let _ = vxor(v128, v128);
//! let _ = xor(v128, v128);
//! ```
//!
//! <!-- doctest-inventory:start -->
//!
//! # Instruction inventory
//!
//! ```
//! use rust_isa::loongarch::simd::lsx::add as _;
//! use rust_isa::loongarch::simd::lsx::add_i16 as _;
//! use rust_isa::loongarch::simd::lsx::add_i64 as _;
//! use rust_isa::loongarch::simd::lsx::add_i8 as _;
//! use rust_isa::loongarch::simd::lsx::and as _;
//! use rust_isa::loongarch::simd::lsx::cmpeq_i32 as _;
//! use rust_isa::loongarch::simd::lsx::cmpgt_i32 as _;
//! use rust_isa::loongarch::simd::lsx::fadd as _;
//! use rust_isa::loongarch::simd::lsx::fdiv as _;
//! use rust_isa::loongarch::simd::lsx::fmul as _;
//! use rust_isa::loongarch::simd::lsx::fsub as _;
//! use rust_isa::loongarch::simd::lsx::mul as _;
//! use rust_isa::loongarch::simd::lsx::mul_i16 as _;
//! use rust_isa::loongarch::simd::lsx::mul_i32 as _;
//! use rust_isa::loongarch::simd::lsx::or as _;
//! use rust_isa::loongarch::simd::lsx::splat_i32 as _;
//! use rust_isa::loongarch::simd::lsx::sub_i16 as _;
//! use rust_isa::loongarch::simd::lsx::sub_i32 as _;
//! use rust_isa::loongarch::simd::lsx::sub_i64 as _;
//! use rust_isa::loongarch::simd::lsx::sub_i8 as _;
//! use rust_isa::loongarch::simd::lsx::v128_load as _;
//! use rust_isa::loongarch::simd::lsx::v128_store as _;
//! use rust_isa::loongarch::simd::lsx::vabs as _;
//! use rust_isa::loongarch::simd::lsx::vabs_b as _;
//! use rust_isa::loongarch::simd::lsx::vabs_d as _;
//! use rust_isa::loongarch::simd::lsx::vabs_h as _;
//! use rust_isa::loongarch::simd::lsx::vadd as _;
//! use rust_isa::loongarch::simd::lsx::vadd_b as _;
//! use rust_isa::loongarch::simd::lsx::vadd_d as _;
//! use rust_isa::loongarch::simd::lsx::vadd_h as _;
//! use rust_isa::loongarch::simd::lsx::vadd_w as _;
//! use rust_isa::loongarch::simd::lsx::vand as _;
//! use rust_isa::loongarch::simd::lsx::vavgr as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_b as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_bu as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_d as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_du as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_h as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_hu as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_w as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_wu as _;
//! use rust_isa::loongarch::simd::lsx::vbitsel as _;
//! use rust_isa::loongarch::simd::lsx::vext as _;
//! use rust_isa::loongarch::simd::lsx::vexth as _;
//! use rust_isa::loongarch::simd::lsx::vexth_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vexth_du_wu as _;
//! use rust_isa::loongarch::simd::lsx::vexth_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vexth_hu_bu as _;
//! use rust_isa::loongarch::simd::lsx::vexth_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vexth_qu_du as _;
//! use rust_isa::loongarch::simd::lsx::vexth_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vexth_wu_hu as _;
//! use rust_isa::loongarch::simd::lsx::vextl as _;
//! use rust_isa::loongarch::simd::lsx::vextl_qu_du as _;
//! use rust_isa::loongarch::simd::lsx::vfadd as _;
//! use rust_isa::loongarch::simd::lsx::vfadd_d as _;
//! use rust_isa::loongarch::simd::lsx::vfadd_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_caf_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_caf_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_ceq_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_ceq_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cle_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cle_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_clt_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_clt_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cne_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cne_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cor_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cor_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cueq_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cueq_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cule_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cule_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cult_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cult_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cun_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cun_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cune_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_cune_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_saf_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_saf_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_seq_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_seq_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sle_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sle_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_slt_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_slt_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sne_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sne_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sor_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sor_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sueq_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sueq_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sule_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sule_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sult_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sult_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sun_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sun_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sune_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcmp_sune_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcvt as _;
//! use rust_isa::loongarch::simd::lsx::vfcvt_s_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcvth_d_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcvtl_d_s as _;
//! use rust_isa::loongarch::simd::lsx::vfdiv as _;
//! use rust_isa::loongarch::simd::lsx::vfdiv_d as _;
//! use rust_isa::loongarch::simd::lsx::vfdiv_s as _;
//! use rust_isa::loongarch::simd::lsx::vffint as _;
//! use rust_isa::loongarch::simd::lsx::vffint_d_l as _;
//! use rust_isa::loongarch::simd::lsx::vffint_d_lu as _;
//! use rust_isa::loongarch::simd::lsx::vffint_s_l as _;
//! use rust_isa::loongarch::simd::lsx::vffint_s_w as _;
//! use rust_isa::loongarch::simd::lsx::vffint_s_wu as _;
//! use rust_isa::loongarch::simd::lsx::vffinth_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vffintl_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vfmax as _;
//! use rust_isa::loongarch::simd::lsx::vfmax_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmax_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmin as _;
//! use rust_isa::loongarch::simd::lsx::vfmin_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmin_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmul as _;
//! use rust_isa::loongarch::simd::lsx::vfmul_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmul_s as _;
//! use rust_isa::loongarch::simd::lsx::vfsub as _;
//! use rust_isa::loongarch::simd::lsx::vfsub_d as _;
//! use rust_isa::loongarch::simd::lsx::vfsub_s as _;
//! use rust_isa::loongarch::simd::lsx::vftint as _;
//! use rust_isa::loongarch::simd::lsx::vftint_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftint_lu_d as _;
//! use rust_isa::loongarch::simd::lsx::vftint_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftint_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftint_wu_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrm_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrm_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrm_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrne_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrne_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrne_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrp_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrp_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrp_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_lu_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_wu_s as _;
//! use rust_isa::loongarch::simd::lsx::vld as _;
//! use rust_isa::loongarch::simd::lsx::vldi as _;
//! use rust_isa::loongarch::simd::lsx::vmax as _;
//! use rust_isa::loongarch::simd::lsx::vmin as _;
//! use rust_isa::loongarch::simd::lsx::vmul as _;
//! use rust_isa::loongarch::simd::lsx::vmul_h as _;
//! use rust_isa::loongarch::simd::lsx::vmul_w as _;
//! use rust_isa::loongarch::simd::lsx::vneg as _;
//! use rust_isa::loongarch::simd::lsx::vneg_b as _;
//! use rust_isa::loongarch::simd::lsx::vneg_d as _;
//! use rust_isa::loongarch::simd::lsx::vneg_h as _;
//! use rust_isa::loongarch::simd::lsx::vneg_w as _;
//! use rust_isa::loongarch::simd::lsx::vnor as _;
//! use rust_isa::loongarch::simd::lsx::vor as _;
//! use rust_isa::loongarch::simd::lsx::vpackev as _;
//! use rust_isa::loongarch::simd::lsx::vpackod as _;
//! use rust_isa::loongarch::simd::lsx::vrepl as _;
//! use rust_isa::loongarch::simd::lsx::vrepl_b as _;
//! use rust_isa::loongarch::simd::lsx::vrepl_d as _;
//! use rust_isa::loongarch::simd::lsx::vrepl_h as _;
//! use rust_isa::loongarch::simd::lsx::vrepl_w as _;
//! use rust_isa::loongarch::simd::lsx::vreplve as _;
//! use rust_isa::loongarch::simd::lsx::vreplve_b as _;
//! use rust_isa::loongarch::simd::lsx::vreplve_d as _;
//! use rust_isa::loongarch::simd::lsx::vreplve_h as _;
//! use rust_isa::loongarch::simd::lsx::vreplve_w as _;
//! use rust_isa::loongarch::simd::lsx::vsat as _;
//! use rust_isa::loongarch::simd::lsx::vsat_b as _;
//! use rust_isa::loongarch::simd::lsx::vsat_bu as _;
//! use rust_isa::loongarch::simd::lsx::vsat_d as _;
//! use rust_isa::loongarch::simd::lsx::vsat_du as _;
//! use rust_isa::loongarch::simd::lsx::vsat_h as _;
//! use rust_isa::loongarch::simd::lsx::vsat_hu as _;
//! use rust_isa::loongarch::simd::lsx::vsat_w as _;
//! use rust_isa::loongarch::simd::lsx::vsat_wu as _;
//! use rust_isa::loongarch::simd::lsx::vseq as _;
//! use rust_isa::loongarch::simd::lsx::vseqi as _;
//! use rust_isa::loongarch::simd::lsx::vshuf as _;
//! use rust_isa::loongarch::simd::lsx::vshuf4i as _;
//! use rust_isa::loongarch::simd::lsx::vsle as _;
//! use rust_isa::loongarch::simd::lsx::vslei as _;
//! use rust_isa::loongarch::simd::lsx::vsll as _;
//! use rust_isa::loongarch::simd::lsx::vslli as _;
//! use rust_isa::loongarch::simd::lsx::vslt as _;
//! use rust_isa::loongarch::simd::lsx::vslti as _;
//! use rust_isa::loongarch::simd::lsx::vsra as _;
//! use rust_isa::loongarch::simd::lsx::vsrai as _;
//! use rust_isa::loongarch::simd::lsx::vsrl as _;
//! use rust_isa::loongarch::simd::lsx::vsrli as _;
//! use rust_isa::loongarch::simd::lsx::vssr as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_wu_du as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_wu_du as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_wu_du as _;
//! use rust_isa::loongarch::simd::lsx::vst as _;
//! use rust_isa::loongarch::simd::lsx::vsub as _;
//! use rust_isa::loongarch::simd::lsx::vsub_b as _;
//! use rust_isa::loongarch::simd::lsx::vsub_d as _;
//! use rust_isa::loongarch::simd::lsx::vsub_h as _;
//! use rust_isa::loongarch::simd::lsx::vsub_w as _;
//! use rust_isa::loongarch::simd::lsx::vxor as _;
//! use rust_isa::loongarch::simd::lsx::xor as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

mod v128;

pub use v128::*;

unsafe extern "C" {
    fn rust_isa_lsx_add(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_mul(a: *const u8, b: *const u8, out: *mut u8);
}

/// `vadd.w`
///
/// Add the four 32-bit integer lanes.
///
/// # Examples
///
/// ```
/// use rust_isa::loongarch::simd::lsx::{add, V128};
///
/// let a = V128::from_bytes([1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
/// let b = V128::from_bytes([2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0]);
/// assert_eq!(
///     add(a, b).to_bytes(),
///     [3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0]
/// );
/// ```
#[inline(always)]
pub fn add(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_add(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.w`
///
/// Multiply the four 32-bit integer lanes, keeping the low 32 bits.
#[inline(always)]
pub fn mul(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_mul(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_add_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_add_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_add_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_sub_i8(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_sub_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_sub_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_sub_i64(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_mul_i16(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_mul_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_and(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_or(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_xor(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_cmpeq_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_cmpgt_i32(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_fadd(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_fsub(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_fmul(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_fdiv(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_splat_i32(value: i32, out: *mut u8);
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

binary!(
    /// `vadd.b` on 8-bit lanes.
    add_i8,
    rust_isa_lsx_add_i8
);
binary!(
    /// `vadd.h` on 16-bit lanes.
    add_i16,
    rust_isa_lsx_add_i16
);
binary!(
    /// `vadd.d` on 64-bit lanes.
    add_i64,
    rust_isa_lsx_add_i64
);
binary!(
    /// `vsub.b` on 8-bit lanes.
    sub_i8,
    rust_isa_lsx_sub_i8
);
binary!(
    /// `vsub.h` on 16-bit lanes.
    sub_i16,
    rust_isa_lsx_sub_i16
);
binary!(
    /// `vsub.w` on 32-bit lanes.
    sub_i32,
    rust_isa_lsx_sub_i32
);
binary!(
    /// `vsub.d` on 64-bit lanes.
    sub_i64,
    rust_isa_lsx_sub_i64
);
binary!(
    /// `vmul.h` on 16-bit lanes, keeping the low 16 bits.
    mul_i16,
    rust_isa_lsx_mul_i16
);
binary!(
    /// `vmul.w` on 32-bit lanes, keeping the low 32 bits.
    mul_i32,
    rust_isa_lsx_mul_i32
);
binary!(
    /// Bitwise AND.
    and,
    rust_isa_lsx_and
);
binary!(
    /// Bitwise OR.
    or,
    rust_isa_lsx_or
);
binary!(
    /// Bitwise XOR.
    xor,
    rust_isa_lsx_xor
);
binary!(
    /// `vseq.w` on 32-bit lanes. A true lane is all bits one.
    cmpeq_i32,
    rust_isa_lsx_cmpeq_i32
);
binary!(
    /// `vslt.w` on signed 32-bit lanes. A true lane is all bits one.
    cmpgt_i32,
    rust_isa_lsx_cmpgt_i32
);
binary!(
    /// `vfadd.s` on 32-bit float lanes.
    fadd,
    rust_isa_lsx_fadd
);
binary!(
    /// `vfsub.s` on 32-bit float lanes.
    fsub,
    rust_isa_lsx_fsub
);
binary!(
    /// `vfmul.s` on 32-bit float lanes.
    fmul,
    rust_isa_lsx_fmul
);
binary!(
    /// `vfdiv.s` on 32-bit float lanes.
    fdiv,
    rust_isa_lsx_fdiv
);

/// Broadcast a 32-bit value to all four lanes.
#[inline(always)]
pub fn splat_i32(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_splat_i32(value, out.as_mut_ptr());
    }
    out
}

/// `vadd.w`.
#[inline(always)]
pub fn vadd(a: V128, b: V128) -> V128 {
    add(a, b)
}

/// `vsub.w`.
#[inline(always)]
pub fn vsub(a: V128, b: V128) -> V128 {
    sub_i32(a, b)
}

/// `vmul.w`.
#[inline(always)]
pub fn vmul(a: V128, b: V128) -> V128 {
    mul(a, b)
}

/// `vand.v`.
#[inline(always)]
pub fn vand(a: V128, b: V128) -> V128 {
    and(a, b)
}

/// `vor.v`.
#[inline(always)]
pub fn vor(a: V128, b: V128) -> V128 {
    or(a, b)
}

/// `vxor.v`.
#[inline(always)]
pub fn vxor(a: V128, b: V128) -> V128 {
    xor(a, b)
}

/// `vabs.w`.
#[inline(always)]
pub fn vabs(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for lane in 0..4 {
        let value = i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.wrapping_abs().to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vneg.w`.
#[inline(always)]
pub fn vneg(a: V128) -> V128 {
    let bytes = a.to_bytes();
    let mut out = [0u8; 16];
    for lane in 0..4 {
        let value = i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap());
        out[lane * 4..lane * 4 + 4].copy_from_slice(&value.wrapping_neg().to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vadd.d`.
#[inline(always)]
pub fn vadd_d(a: V128, b: V128) -> V128 {
    add_i64(a, b)
}

/// `vsub.d`.
#[inline(always)]
pub fn vsub_d(a: V128, b: V128) -> V128 {
    sub_i64(a, b)
}

/// `vadd.b`.
#[inline(always)]
pub fn vadd_b(a: V128, b: V128) -> V128 {
    add_i8(a, b)
}

/// `vadd.h`.
#[inline(always)]
pub fn vadd_h(a: V128, b: V128) -> V128 {
    add_i16(a, b)
}

/// `vsub.b`.
#[inline(always)]
pub fn vsub_b(a: V128, b: V128) -> V128 {
    sub_i8(a, b)
}

/// `vsub.h`.
#[inline(always)]
pub fn vsub_h(a: V128, b: V128) -> V128 {
    sub_i16(a, b)
}

/// `vmul.h`.
#[inline(always)]
pub fn vmul_h(a: V128, b: V128) -> V128 {
    mul_i16(a, b)
}

/// `vadd.w`.
#[inline(always)]
pub fn vadd_w(a: V128, b: V128) -> V128 {
    add(a, b)
}

/// `vsub.w`.
#[inline(always)]
pub fn vsub_w(a: V128, b: V128) -> V128 {
    sub_i32(a, b)
}

/// `vmul.w`.
#[inline(always)]
pub fn vmul_w(a: V128, b: V128) -> V128 {
    mul(a, b)
}

/// `vneg.w`.
#[inline(always)]
pub fn vneg_w(a: V128) -> V128 {
    vneg(a)
}

/// `vabs.b`.
#[inline(always)]
pub fn vabs_b(a: V128) -> V128 {
    V128::from_bytes(core::array::from_fn(|i| {
        (a.to_bytes()[i] as i8).wrapping_abs() as u8
    }))
}

/// `vabs.h`.
#[inline(always)]
pub fn vabs_h(a: V128) -> V128 {
    let b = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..8 {
        let v = i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]);
        o[i * 2..i * 2 + 2].copy_from_slice(&v.wrapping_abs().to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vabs.d`.
#[inline(always)]
pub fn vabs_d(a: V128) -> V128 {
    let b = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..2 {
        let v = i64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        o[i * 8..i * 8 + 8].copy_from_slice(&v.wrapping_abs().to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vneg.b`.
#[inline(always)]
pub fn vneg_b(a: V128) -> V128 {
    V128::from_bytes(core::array::from_fn(|i| {
        (a.to_bytes()[i] as i8).wrapping_neg() as u8
    }))
}

/// `vneg.h`.
#[inline(always)]
pub fn vneg_h(a: V128) -> V128 {
    let b = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..8 {
        let v = i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]);
        o[i * 2..i * 2 + 2].copy_from_slice(&v.wrapping_neg().to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vneg.d`.
#[inline(always)]
pub fn vneg_d(a: V128) -> V128 {
    let b = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..2 {
        let v = i64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        o[i * 8..i * 8 + 8].copy_from_slice(&v.wrapping_neg().to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vmax.w`.
#[inline(always)]
pub fn vmax(a: V128, b: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&x.max(y).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vmin.w`.
#[inline(always)]
pub fn vmin(a: V128, b: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&x.min(y).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vseq.w`.
#[inline(always)]
pub fn vseq(a: V128, b: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(if x == y { -1i32 } else { 0 }).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vsle.w`.
#[inline(always)]
pub fn vsle(a: V128, b: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(if x <= y { -1i32 } else { 0 }).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vslt.w`.
#[inline(always)]
pub fn vslt(a: V128, b: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        let y = i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(if x < y { -1i32 } else { 0 }).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vseqi.w`.
#[inline(always)]
pub fn vseqi(a: V128, imm: i32) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(if x == imm { -1i32 } else { 0 }).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vslei.w`.
#[inline(always)]
pub fn vslei(a: V128, imm: i32) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(if x <= imm { -1i32 } else { 0 }).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vslti.w`.
#[inline(always)]
pub fn vslti(a: V128, imm: i32) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(if x < imm { -1i32 } else { 0 }).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vnor.v`.
#[inline(always)]
pub fn vnor(a: V128, b: V128) -> V128 {
    V128::from_bytes(core::array::from_fn(|i| {
        !(a.to_bytes()[i] | b.to_bytes()[i])
    }))
}

/// `vbitsel.v`.
#[inline(always)]
pub fn vbitsel(a: V128, b: V128, mask: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let m = mask.to_bytes();
    V128::from_bytes(core::array::from_fn(|i| (a[i] & m[i]) | (b[i] & !m[i])))
}

/// `vshuf.b`.
#[inline(always)]
pub fn vshuf(a: V128, b: V128, control: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let c = control.to_bytes();
    V128::from_bytes(core::array::from_fn(|i| {
        let idx = (c[i] & 31) as usize;
        if idx < 16 { a[idx] } else { b[idx - 16] }
    }))
}

/// `vshuf4i.b`.
#[inline(always)]
pub fn vshuf4i<const IMM: u8>(a: V128) -> V128 {
    let a = a.to_bytes();
    V128::from_bytes(core::array::from_fn(|i| {
        a[(i / 4) * 4 + (((IMM >> ((i % 4) * 2)) & 3) as usize)]
    }))
}

/// `vsll.w`.
#[inline(always)]
pub fn vsll(a: V128, shift: i32) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(x << ((shift as u32) & 31)).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vsrl.w`.
#[inline(always)]
pub fn vsrl(a: V128, shift: i32) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = u32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(x >> ((shift as u32) & 31)).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vsra.w`.
#[inline(always)]
pub fn vsra(a: V128, shift: i32) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(x >> ((shift as u32) & 31)).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vslli.w`.
#[inline(always)]
pub fn vslli<const SHIFT: u32>(a: V128) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(x << (SHIFT & 31)).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vsrli.w`.
#[inline(always)]
pub fn vsrli<const SHIFT: u32>(a: V128) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = u32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(x >> (SHIFT & 31)).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vsrai.w`.
#[inline(always)]
pub fn vsrai<const SHIFT: u32>(a: V128) -> V128 {
    let a = a.to_bytes();
    let mut o = [0u8; 16];
    for i in 0..4 {
        let x = i32::from_le_bytes(a[i * 4..i * 4 + 4].try_into().unwrap());
        o[i * 4..i * 4 + 4].copy_from_slice(&(x >> (SHIFT & 31)).to_le_bytes());
    }
    V128::from_bytes(o)
}

/// `vpackev.b`.
#[inline(always)]
pub fn vpackev(a: V128, b: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V128::from_bytes(core::array::from_fn(|i| {
        if i % 2 == 0 { a[i] } else { b[i - 1] }
    }))
}

/// `vpackod.b`.
#[inline(always)]
pub fn vpackod(a: V128, b: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    V128::from_bytes(core::array::from_fn(|i| {
        if i % 2 == 0 { a[i + 1] } else { b[i] }
    }))
}

#[inline(always)]
fn i64_lanes(a: V128) -> [i64; 2] {
    let bytes = a.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

#[inline(always)]
fn u64_lanes(a: V128) -> [u64; 2] {
    let bytes = a.to_bytes();
    core::array::from_fn(|lane| {
        u64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

#[inline(always)]
fn from_u32_lanes(lanes: [u32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in lanes.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

/// `vsllwil.h.b` with a zero shift.
///
/// Sign-extend the low eight bytes to eight halfwords.
#[inline(always)]
pub fn vext(a: V128) -> V128 {
    let a = a.to_bytes();
    let mut out = [0u8; 16];
    for lane in 0..8 {
        let value = (a[lane] as i8 as i16).to_le_bytes();
        out[lane * 2..lane * 2 + 2].copy_from_slice(&value);
    }
    V128::from_bytes(out)
}

unsafe extern "C" {
    fn rust_isa_lsx_vexth_h_b(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vexth_w_h(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vexth_d_w(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vexth_q_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vexth_hu_bu(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vexth_wu_hu(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vexth_du_wu(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vexth_qu_du(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vextl_q_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vextl_qu_du(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavgr_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavgr_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavgr_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavgr_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavgr_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavgr_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavgr_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavgr_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfadd_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfadd_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfsub_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfsub_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmul_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmul_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfdiv_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfdiv_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmax_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmax_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmin_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfmin_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfcvt_s_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfcvtl_d_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfcvth_d_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftint_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftint_wu_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftint_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftint_lu_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftint_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrz_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrm_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrp_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrne_w_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrz_wu_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrz_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrm_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrp_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrne_l_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrz_lu_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrz_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrm_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrp_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrne_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vffint_s_w(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vffint_s_wu(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vffint_d_l(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vffint_d_lu(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vffint_s_l(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vffintl_d_w(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vffinth_d_w(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vreplgr2vr_b(value: i32, out: *mut u8);
    fn rust_isa_lsx_vreplgr2vr_h(value: i32, out: *mut u8);
    fn rust_isa_lsx_vreplgr2vr_w(value: i32, out: *mut u8);
    fn rust_isa_lsx_vreplgr2vr_d(value: i64, out: *mut u8);
}

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

unary!(
    /// `vexth.h.b` on the upper eight bytes, sign-extended to halfwords.
    vexth_h_b,
    rust_isa_lsx_vexth_h_b
);
unary!(
    /// `vexth.w.h` on the upper four halfwords, sign-extended to words.
    vexth_w_h,
    rust_isa_lsx_vexth_w_h
);
unary!(
    /// `vexth.d.w` on the upper two words, sign-extended to doublewords.
    vexth_d_w,
    rust_isa_lsx_vexth_d_w
);
unary!(
    /// `vexth.q.d` on the upper doubleword, sign-extended to a quadword.
    vexth_q_d,
    rust_isa_lsx_vexth_q_d
);
unary!(
    /// `vexth.hu.bu` on the upper eight bytes, zero-extended to halfwords.
    vexth_hu_bu,
    rust_isa_lsx_vexth_hu_bu
);
unary!(
    /// `vexth.wu.hu` on the upper four halfwords, zero-extended to words.
    vexth_wu_hu,
    rust_isa_lsx_vexth_wu_hu
);
unary!(
    /// `vexth.du.wu` on the upper two words, zero-extended to doublewords.
    vexth_du_wu,
    rust_isa_lsx_vexth_du_wu
);
unary!(
    /// `vexth.qu.du` on the upper doubleword, zero-extended to a quadword.
    vexth_qu_du,
    rust_isa_lsx_vexth_qu_du
);
unary!(
    /// `vextl.q.d`
    ///
    /// Sign-extend the low doubleword to a quadword.
    vextl,
    rust_isa_lsx_vextl_q_d
);
unary!(
    /// `vextl.qu.du`
    ///
    /// Zero-extend the low doubleword to a quadword.
    vextl_qu_du,
    rust_isa_lsx_vextl_qu_du
);

/// `vexth.h.b` on the upper eight bytes.
#[inline(always)]
pub fn vexth(a: V128) -> V128 {
    vexth_h_b(a)
}

binary!(
    /// `vavgr.b`
    vavgr_b,
    rust_isa_lsx_vavgr_b
);
binary!(
    /// `vavgr.h`
    vavgr_h,
    rust_isa_lsx_vavgr_h
);
binary!(
    /// `vavgr.w`
    vavgr_w,
    rust_isa_lsx_vavgr_w
);
binary!(
    /// `vavgr.d`
    vavgr_d,
    rust_isa_lsx_vavgr_d
);
binary!(
    /// `vavgr.bu`
    vavgr_bu,
    rust_isa_lsx_vavgr_bu
);
binary!(
    /// `vavgr.hu`
    vavgr_hu,
    rust_isa_lsx_vavgr_hu
);
binary!(
    /// `vavgr.wu`
    vavgr_wu,
    rust_isa_lsx_vavgr_wu
);
binary!(
    /// `vavgr.du`
    vavgr_du,
    rust_isa_lsx_vavgr_du
);

/// `vavgr.bu`
///
/// Rounding average of the unsigned byte lanes: `(a + b + 1) >> 1`.
#[inline(always)]
pub fn vavgr(a: V128, b: V128) -> V128 {
    vavgr_bu(a, b)
}

macro_rules! vsat_signed {
    ($(#[$meta:meta])* $name:ident, $ty:ty, $max_imm:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128) -> V128 {
            const {
                assert!(IMM <= $max_imm, "vsat immediate is out of range");
            }
            let width = core::mem::size_of::<$ty>();
            let bytes = a.to_bytes();
            let mut out = [0u8; 16];
            let max = (1i128 << IMM) - 1;
            let min = -max - 1;
            for lane in 0..(16 / width) {
                let value = <$ty>::from_le_bytes(
                    bytes[lane * width..lane * width + width]
                        .try_into()
                        .unwrap(),
                ) as i128;
                let clamped = value.clamp(min, max) as $ty;
                out[lane * width..lane * width + width]
                    .copy_from_slice(&clamped.to_le_bytes());
            }
            V128::from_bytes(out)
        }
    };
}

macro_rules! vsat_unsigned {
    ($(#[$meta:meta])* $name:ident, $ty:ty, $max_imm:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const IMM: u32>(a: V128) -> V128 {
            const {
                assert!(IMM <= $max_imm, "vsat immediate is out of range");
            }
            let width = core::mem::size_of::<$ty>();
            let bytes = a.to_bytes();
            let mut out = [0u8; 16];
            let max = (1i128 << (IMM + 1)) - 1;
            for lane in 0..(16 / width) {
                let value = <$ty>::from_le_bytes(
                    bytes[lane * width..lane * width + width]
                        .try_into()
                        .unwrap(),
                ) as i128;
                let clamped = value.clamp(0, max) as $ty;
                out[lane * width..lane * width + width]
                    .copy_from_slice(&clamped.to_le_bytes());
            }
            V128::from_bytes(out)
        }
    };
}

vsat_signed!(
    /// `vsat.b`
    ///
    /// Saturate each signed byte to `[-2^IMM, 2^IMM - 1]`.
    vsat_b,
    i8,
    7
);
vsat_signed!(
    /// `vsat.h`
    ///
    /// Saturate each signed halfword to `[-2^IMM, 2^IMM - 1]`.
    vsat_h,
    i16,
    15
);
vsat_signed!(
    /// `vsat.w`
    ///
    /// Saturate each signed word to `[-2^IMM, 2^IMM - 1]`.
    vsat_w,
    i32,
    31
);
vsat_signed!(
    /// `vsat.d`
    ///
    /// Saturate each signed doubleword to `[-2^IMM, 2^IMM - 1]`.
    vsat_d,
    i64,
    63
);
vsat_unsigned!(
    /// `vsat.bu`
    ///
    /// Saturate each byte to `[0, 2^(IMM + 1) - 1]`.
    vsat_bu,
    u8,
    7
);
vsat_unsigned!(
    /// `vsat.hu`
    ///
    /// Saturate each halfword to `[0, 2^(IMM + 1) - 1]`.
    vsat_hu,
    u16,
    15
);
vsat_unsigned!(
    /// `vsat.wu`
    ///
    /// Saturate each word to `[0, 2^(IMM + 1) - 1]`.
    vsat_wu,
    u32,
    31
);
vsat_unsigned!(
    /// `vsat.du`
    ///
    /// Saturate each doubleword to `[0, 2^(IMM + 1) - 1]`.
    vsat_du,
    u64,
    63
);

/// `vsat.w`
#[inline(always)]
pub fn vsat<const IMM: u32>(a: V128) -> V128 {
    vsat_w::<IMM>(a)
}

#[inline(always)]
fn ssr_logical(value: u64, shift: u32, round: bool, max: u32) -> u32 {
    let shifted = if shift == 0 {
        value
    } else if round {
        (value >> shift) + ((value >> (shift - 1)) & 1)
    } else {
        value >> shift
    };
    shifted.min(max as u64) as u32
}

#[inline(always)]
fn ssr_arithmetic(value: i64, shift: u32, round: bool) -> i32 {
    let shifted = if shift == 0 {
        value
    } else if round {
        (value >> shift) + ((value >> (shift - 1)) & 1)
    } else {
        value >> shift
    };
    shifted.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

macro_rules! vssr_word_doubleword {
    ($(#[$meta:meta])* $name:ident, $body:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const SHIFT: u32>(a: V128, b: V128) -> V128 {
            const {
                assert!(SHIFT < 64, "vssr shift must be in 0..64");
            }
            let mut lanes = [0u32; 4];
            let sources = u64_lanes(a).into_iter().chain(u64_lanes(b));
            for (lane, value) in sources.enumerate() {
                lanes[lane] = $body(value);
            }
            from_u32_lanes(lanes)
        }
    };
}

vssr_word_doubleword!(
    /// `vssrlni.w.d`
    ///
    /// The low two doublewords come from `a`, the high two from `b`. Each
    /// result is shifted right and saturated to a non-negative signed word.
    vssrln_w_d,
    |value: u64| ssr_logical(value, SHIFT, false, i32::MAX as u32)
);
vssr_word_doubleword!(
    /// `vssrlrni.w.d`
    ///
    /// Like [`vssrln_w_d`], with rounding applied before saturation.
    vssrlrn_w_d,
    |value: u64| ssr_logical(value, SHIFT, true, i32::MAX as u32)
);
vssr_word_doubleword!(
    /// `vssrlni.wu.d`
    ///
    /// The low two doublewords come from `a`, the high two from `b`. Each
    /// result is shifted right and saturated to an unsigned word.
    vssrln_wu_du,
    |value: u64| ssr_logical(value, SHIFT, false, u32::MAX)
);
vssr_word_doubleword!(
    /// `vssrlrni.wu.d`
    ///
    /// Like [`vssrln_wu_du`], with rounding applied before saturation.
    vssrlrn_wu_du,
    |value: u64| ssr_logical(value, SHIFT, true, u32::MAX)
);

/// `vssrani.w.d`
///
/// The low two doublewords come from `a`, the high two from `b`. Each result
/// is arithmetic-shifted right with rounding and saturated to a signed word.
#[inline(always)]
pub fn vssrarn_w_d<const SHIFT: u32>(a: V128, b: V128) -> V128 {
    const {
        assert!(SHIFT < 64, "vssr shift must be in 0..64");
    }
    let sources = i64_lanes(a).into_iter().chain(i64_lanes(b));
    let mut lanes = [0u32; 4];
    for (lane, value) in sources.enumerate() {
        lanes[lane] = ssr_arithmetic(value, SHIFT, true) as u32;
    }
    from_u32_lanes(lanes)
}

/// `vssrarni.wu.d`
///
/// Negative inputs saturate to zero. The low two doublewords come from `a`,
/// the high two from `b`.
#[inline(always)]
pub fn vssrarn_wu_du<const SHIFT: u32>(a: V128, b: V128) -> V128 {
    const {
        assert!(SHIFT < 64, "vssr shift must be in 0..64");
    }
    let mut lanes = [0u32; 4];
    let sources = u64_lanes(a).into_iter().chain(u64_lanes(b));
    for (lane, value) in sources.enumerate() {
        lanes[lane] = if (value as i64) < 0 {
            0
        } else {
            ssr_logical(value, SHIFT, true, u32::MAX)
        };
    }
    from_u32_lanes(lanes)
}

/// `vssrln.b.h`
///
/// Shift each 16-bit lane of `a` right by the low four bits of the
/// corresponding lane of `b` and saturate to an unsigned byte. The result is
/// written to the low eight bytes; the high eight bytes are zero.
#[inline(always)]
pub fn vssr(a: V128, b: V128) -> V128 {
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 16];
    for lane in 0..8 {
        let offset = lane * 2;
        let source = u16::from_le_bytes([a[offset], a[offset + 1]]);
        let shift = u16::from_le_bytes([b[offset], b[offset + 1]]) % 16;
        out[lane] = (source >> shift).min(0xff) as u8;
    }
    V128::from_bytes(out)
}

binary!(
    /// `vfadd.s`
    vfadd_s,
    rust_isa_lsx_vfadd_s
);
binary!(
    /// `vfadd.d`
    vfadd_d,
    rust_isa_lsx_vfadd_d
);
binary!(
    /// `vfsub.s`
    vfsub_s,
    rust_isa_lsx_vfsub_s
);
binary!(
    /// `vfsub.d`
    vfsub_d,
    rust_isa_lsx_vfsub_d
);
binary!(
    /// `vfmul.s`
    vfmul_s,
    rust_isa_lsx_vfmul_s
);
binary!(
    /// `vfmul.d`
    vfmul_d,
    rust_isa_lsx_vfmul_d
);
binary!(
    /// `vfdiv.s`
    vfdiv_s,
    rust_isa_lsx_vfdiv_s
);
binary!(
    /// `vfdiv.d`
    vfdiv_d,
    rust_isa_lsx_vfdiv_d
);
binary!(
    /// `vfmax.s`
    vfmax_s,
    rust_isa_lsx_vfmax_s
);
binary!(
    /// `vfmax.d`
    vfmax_d,
    rust_isa_lsx_vfmax_d
);
binary!(
    /// `vfmin.s`
    vfmin_s,
    rust_isa_lsx_vfmin_s
);
binary!(
    /// `vfmin.d`
    vfmin_d,
    rust_isa_lsx_vfmin_d
);

/// `vfadd.s`
#[inline(always)]
pub fn vfadd(a: V128, b: V128) -> V128 {
    vfadd_s(a, b)
}

/// `vfsub.s`
#[inline(always)]
pub fn vfsub(a: V128, b: V128) -> V128 {
    vfsub_s(a, b)
}

/// `vfmul.s`
#[inline(always)]
pub fn vfmul(a: V128, b: V128) -> V128 {
    vfmul_s(a, b)
}

/// `vfdiv.s`
#[inline(always)]
pub fn vfdiv(a: V128, b: V128) -> V128 {
    vfdiv_s(a, b)
}

/// `vfmax.s`
#[inline(always)]
pub fn vfmax(a: V128, b: V128) -> V128 {
    vfmax_s(a, b)
}

/// `vfmin.s`
#[inline(always)]
pub fn vfmin(a: V128, b: V128) -> V128 {
    vfmin_s(a, b)
}

/// `vfcmp.<COND>.s`
///
/// `COND` is the 5-bit condition field. The low bit selects the signaling
/// form; only the result mask is exposed, so the quiet and signaling forms
/// return the same lanes. Valid values and mnemonics:
///
/// | `COND` | quiet  | signaling |
/// | ------ | ------ | --------- |
/// | 0      | `caf`  | `saf`     |
/// | 2      | `clt`  | `slt`     |
/// | 4      | `ceq`  | `seq`     |
/// | 6      | `cle`  | `sle`     |
/// | 8      | `cun`  | `sun`     |
/// | 10     | `cult` | `sult`    |
/// | 12     | `cueq` | `sueq`    |
/// | 14     | `cule` | `sule`    |
/// | 16     | `cne`  | `sne`     |
/// | 20     | `cor`  | `sor`     |
/// | 24     | `cune` | `sune`    |
///
/// A true lane is all bits one. Unordered comparisons are true when either
/// lane is NaN.
#[inline(always)]
pub fn vfcmp<const COND: u32>(a: V128, b: V128) -> V128 {
    const { assert!(matches!(COND, 0..=17 | 20 | 21 | 24 | 25)) };
    let less = (COND >> 1) & 1 != 0;
    let equal = (COND >> 1) & 2 != 0;
    let unordered = (COND >> 1) & 4 != 0;
    let ordered_ne = (COND >> 1) & 8 != 0;
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 16];
    for lane in 0..4 {
        let offset = lane * 4;
        let x = f32::from_le_bytes(a[offset..offset + 4].try_into().unwrap());
        let y = f32::from_le_bytes(b[offset..offset + 4].try_into().unwrap());
        let hit = (less && x < y)
            || (equal && x == y)
            || (unordered && (x.is_nan() || y.is_nan()))
            || (ordered_ne && (x < y || x > y));
        let value = if hit { -1i32 } else { 0 };
        out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(out)
}

/// `vfcmp.<COND>.d`
///
/// `COND` uses the same encoding as [`vfcmp`]. A true lane is all bits one.
#[inline(always)]
pub fn vfcmp_d<const COND: u32>(a: V128, b: V128) -> V128 {
    const { assert!(matches!(COND, 0..=17 | 20 | 21 | 24 | 25)) };
    let less = (COND >> 1) & 1 != 0;
    let equal = (COND >> 1) & 2 != 0;
    let unordered = (COND >> 1) & 4 != 0;
    let ordered_ne = (COND >> 1) & 8 != 0;
    let a = a.to_bytes();
    let b = b.to_bytes();
    let mut out = [0u8; 16];
    for lane in 0..2 {
        let offset = lane * 8;
        let x = f64::from_le_bytes(a[offset..offset + 8].try_into().unwrap());
        let y = f64::from_le_bytes(b[offset..offset + 8].try_into().unwrap());
        let hit = (less && x < y)
            || (equal && x == y)
            || (unordered && (x.is_nan() || y.is_nan()))
            || (ordered_ne && (x < y || x > y));
        let value = if hit { -1i64 } else { 0 };
        out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(out)
}

macro_rules! vfcmp_conditions {
    ($(($s:ident, $d:ident, $cond:expr)),* $(,)?) => {
        $(
            #[doc = concat!("`", stringify!($s), "`")]
            #[inline(always)]
            pub fn $s(a: V128, b: V128) -> V128 {
                vfcmp::<$cond>(a, b)
            }

            #[doc = concat!("`", stringify!($d), "`")]
            #[inline(always)]
            pub fn $d(a: V128, b: V128) -> V128 {
                vfcmp_d::<$cond>(a, b)
            }
        )*
    };
}

vfcmp_conditions!(
    (vfcmp_caf_s, vfcmp_caf_d, 0),
    (vfcmp_saf_s, vfcmp_saf_d, 1),
    (vfcmp_clt_s, vfcmp_clt_d, 2),
    (vfcmp_slt_s, vfcmp_slt_d, 3),
    (vfcmp_ceq_s, vfcmp_ceq_d, 4),
    (vfcmp_seq_s, vfcmp_seq_d, 5),
    (vfcmp_cle_s, vfcmp_cle_d, 6),
    (vfcmp_sle_s, vfcmp_sle_d, 7),
    (vfcmp_cun_s, vfcmp_cun_d, 8),
    (vfcmp_sun_s, vfcmp_sun_d, 9),
    (vfcmp_cult_s, vfcmp_cult_d, 10),
    (vfcmp_sult_s, vfcmp_sult_d, 11),
    (vfcmp_cueq_s, vfcmp_cueq_d, 12),
    (vfcmp_sueq_s, vfcmp_sueq_d, 13),
    (vfcmp_cule_s, vfcmp_cule_d, 14),
    (vfcmp_sule_s, vfcmp_sule_d, 15),
    (vfcmp_cne_s, vfcmp_cne_d, 16),
    (vfcmp_sne_s, vfcmp_sne_d, 17),
    (vfcmp_cor_s, vfcmp_cor_d, 20),
    (vfcmp_sor_s, vfcmp_sor_d, 21),
    (vfcmp_cune_s, vfcmp_cune_d, 24),
    (vfcmp_sune_s, vfcmp_sune_d, 25),
);

/// `vfcvt.s.d`
///
/// Convert two pairs of double-precision lanes and interleave the results.
#[inline(always)]
pub fn vfcvt_s_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfcvt_s_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfcvtl.d.s`
///
/// Convert the low two single-precision lanes to double precision.
#[inline(always)]
pub fn vfcvtl_d_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfcvtl_d_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfcvth.d.s`
///
/// Convert the high two single-precision lanes to double precision.
#[inline(always)]
pub fn vfcvth_d_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfcvth_d_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfcvt.s.d`
#[inline(always)]
pub fn vfcvt(a: V128, b: V128) -> V128 {
    vfcvt_s_d(a, b)
}

macro_rules! convert_narrow {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! convert_narrow2 {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

convert_narrow!(
    /// `vftint.w.s`
    ///
    /// Convert four single-precision lanes to signed words using the current
    /// floating-point rounding mode.
    vftint_w_s,
    rust_isa_lsx_vftint_w_s
);
convert_narrow!(
    /// `vftint.wu.s`
    ///
    /// Convert four single-precision lanes to unsigned words using the current
    /// floating-point rounding mode.
    vftint_wu_s,
    rust_isa_lsx_vftint_wu_s
);
convert_narrow!(
    /// `vftint.l.d`
    ///
    /// Convert two double-precision lanes to signed doublewords using the
    /// current floating-point rounding mode.
    vftint_l_d,
    rust_isa_lsx_vftint_l_d
);
convert_narrow!(
    /// `vftint.lu.d`
    ///
    /// Convert two double-precision lanes to unsigned doublewords using the
    /// current floating-point rounding mode.
    vftint_lu_d,
    rust_isa_lsx_vftint_lu_d
);
convert_narrow2!(
    /// `vftint.w.d`
    ///
    /// Convert two pairs of double-precision lanes to signed words and
    /// interleave the results.
    vftint_w_d,
    rust_isa_lsx_vftint_w_d
);
convert_narrow!(
    /// `vftintrz.w.s`
    vftintrz_w_s,
    rust_isa_lsx_vftintrz_w_s
);
convert_narrow!(
    /// `vftintrm.w.s`
    vftintrm_w_s,
    rust_isa_lsx_vftintrm_w_s
);
convert_narrow!(
    /// `vftintrp.w.s`
    vftintrp_w_s,
    rust_isa_lsx_vftintrp_w_s
);
convert_narrow!(
    /// `vftintrne.w.s`
    vftintrne_w_s,
    rust_isa_lsx_vftintrne_w_s
);
convert_narrow!(
    /// `vftintrz.wu.s`
    vftintrz_wu_s,
    rust_isa_lsx_vftintrz_wu_s
);
convert_narrow!(
    /// `vftintrz.l.d`
    vftintrz_l_d,
    rust_isa_lsx_vftintrz_l_d
);
convert_narrow!(
    /// `vftintrm.l.d`
    vftintrm_l_d,
    rust_isa_lsx_vftintrm_l_d
);
convert_narrow!(
    /// `vftintrp.l.d`
    vftintrp_l_d,
    rust_isa_lsx_vftintrp_l_d
);
convert_narrow!(
    /// `vftintrne.l.d`
    vftintrne_l_d,
    rust_isa_lsx_vftintrne_l_d
);
convert_narrow!(
    /// `vftintrz.lu.d`
    vftintrz_lu_d,
    rust_isa_lsx_vftintrz_lu_d
);
convert_narrow2!(
    /// `vftintrz.w.d`
    vftintrz_w_d,
    rust_isa_lsx_vftintrz_w_d
);
convert_narrow2!(
    /// `vftintrm.w.d`
    vftintrm_w_d,
    rust_isa_lsx_vftintrm_w_d
);
convert_narrow2!(
    /// `vftintrp.w.d`
    vftintrp_w_d,
    rust_isa_lsx_vftintrp_w_d
);
convert_narrow2!(
    /// `vftintrne.w.d`
    vftintrne_w_d,
    rust_isa_lsx_vftintrne_w_d
);

/// `vftint.w.s`
#[inline(always)]
pub fn vftint(a: V128) -> V128 {
    vftint_w_s(a)
}

convert_narrow!(
    /// `vffint.s.w`
    vffint_s_w,
    rust_isa_lsx_vffint_s_w
);
convert_narrow!(
    /// `vffint.s.wu`
    vffint_s_wu,
    rust_isa_lsx_vffint_s_wu
);
convert_narrow!(
    /// `vffint.d.l`
    vffint_d_l,
    rust_isa_lsx_vffint_d_l
);
convert_narrow!(
    /// `vffint.d.lu`
    vffint_d_lu,
    rust_isa_lsx_vffint_d_lu
);
convert_narrow2!(
    /// `vffint.s.l`
    vffint_s_l,
    rust_isa_lsx_vffint_s_l
);
convert_narrow!(
    /// `vffintl.d.w`
    vffintl_d_w,
    rust_isa_lsx_vffintl_d_w
);
convert_narrow!(
    /// `vffinth.d.w`
    vffinth_d_w,
    rust_isa_lsx_vffinth_d_w
);

/// `vffint.s.w`
#[inline(always)]
pub fn vffint(a: V128) -> V128 {
    vffint_s_w(a)
}

/// `vld`
///
/// Load 16 bytes from `p + offset`.
///
/// # Safety
///
/// `offset` must be in `-2048..=2047` and the range must be valid for reading
/// 16 bytes. The load does not require alignment.
#[inline(always)]
pub unsafe fn vld(p: *const u8, offset: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        core::ptr::copy_nonoverlapping(p.offset(offset as isize), out.as_mut_ptr(), 16);
    }
    out
}

/// `vst`
///
/// Store 16 bytes to `p + offset`.
///
/// # Safety
///
/// `offset` must be in `-2048..=2047` and the range must be valid for writing
/// 16 bytes. The store does not require alignment.
#[inline(always)]
pub unsafe fn vst(p: *mut u8, offset: i32, value: V128) {
    unsafe {
        core::ptr::copy_nonoverlapping(value.as_ptr(), p.offset(offset as isize), 16);
    }
}

/// `vldi`
///
/// Build the immediate value of the instruction. `IMM` is the 13-bit encoding,
/// not the loaded scalar.
///
/// When bit 12 is clear, bits 9 to 0 are sign-extended and replicated as lanes
/// of the width selected by bits 11 and 10 (`0` is 8-bit, `1` is 16-bit, `2` is
/// 32-bit, `3` is 64-bit).
///
/// When bit 12 is set, bits 11 to 8 select one of the 13 constant patterns and
/// bits 7 to 0 supply the pattern data. The 64-bit pattern is replicated twice.
#[inline(always)]
pub fn vldi<const IMM: i32>() -> V128 {
    const { V128::from_bytes(vldi_bytes(IMM as u32)) }
}

const fn vldi_bytes(imm: u32) -> [u8; 16] {
    let mut out = [0u8; 16];
    if imm & 0x1000 == 0 {
        let lane = 1usize << ((imm >> 10) & 3);
        let value = (((imm & 0x3ff) << 22) as i32 >> 22) as i64 as u64;
        let bytes = value.to_le_bytes();
        let mut i = 0;
        while i < 16 {
            out[i] = bytes[i % lane];
            i += 1;
        }
    } else {
        let data = vldi_pattern(imm).to_le_bytes();
        let mut i = 0;
        while i < 16 {
            out[i] = data[i % 8];
            i += 1;
        }
    }
    out
}

const fn vldi_pattern(imm: u32) -> u64 {
    let mode = (imm >> 8) & 0xf;
    let t = (imm & 0xff) as u64;
    match mode {
        0 => (t << 32) | t,
        1 => (t << 40) | (t << 8),
        2 => (t << 48) | (t << 16),
        3 => (t << 56) | (t << 24),
        4 => (t << 48) | (t << 32) | (t << 16) | t,
        5 => (t << 56) | (t << 40) | (t << 24) | (t << 8),
        6 => (t << 40) | (0xff << 32) | (t << 8) | 0xff,
        7 => (t << 48) | (0xffff << 32) | (t << 16) | 0xffff,
        8 => {
            let mut data = 0u64;
            let mut i = 0;
            while i < 8 {
                data |= t << (i * 8);
                i += 1;
            }
            data
        }
        9 => {
            let mut data = 0u64;
            let mut i = 0;
            while i < 8 {
                if (t >> i) & 1 != 0 {
                    data |= 0xffu64 << (i * 8);
                }
                i += 1;
            }
            data
        }
        10 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 6) | (((1 - b6) as u64) << 5) | if b6 != 0 { 0x1f } else { 0 };
            (high << 57) | (low << 51) | (high << 25) | (low << 19)
        }
        11 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 6) | (((1 - b6) as u64) << 5) | if b6 != 0 { 0x1f } else { 0 };
            (high << 25) | (low << 19)
        }
        12 => {
            let b6 = (imm & 0x40) >> 6;
            let b7 = (imm & 0x80) >> 7;
            let low = (imm & 0x3f) as u64;
            let high =
                ((b7 as u64) << 9) | (((1 - b6) as u64) << 8) | if b6 != 0 { 0xff } else { 0 };
            (high << 54) | (low << 48)
        }
        _ => panic!("invalid vldi immediate"),
    }
}

/// `vreplgr2vr.b`
#[inline(always)]
pub fn vrepl_b(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vreplgr2vr_b(value, out.as_mut_ptr());
    }
    out
}

/// `vreplgr2vr.h`
#[inline(always)]
pub fn vrepl_h(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vreplgr2vr_h(value, out.as_mut_ptr());
    }
    out
}

/// `vreplgr2vr.w`
#[inline(always)]
pub fn vrepl_w(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vreplgr2vr_w(value, out.as_mut_ptr());
    }
    out
}

/// `vreplgr2vr.d`
#[inline(always)]
pub fn vrepl_d(value: i64) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vreplgr2vr_d(value, out.as_mut_ptr());
    }
    out
}

/// `vreplgr2vr.w`
#[inline(always)]
pub fn vrepl(value: i32) -> V128 {
    vrepl_w(value)
}

macro_rules! vreplve {
    ($(#[$meta:meta])* $name:ident, $ty:ty, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name<const INDEX: u8>(a: V128) -> V128 {
            const {
                assert!(INDEX < $max, "vreplve index is out of range");
            }
            let width = core::mem::size_of::<$ty>();
            let bytes = a.to_bytes();
            let mut out = [0u8; 16];
            for lane in 0..(16 / width) {
                let start = INDEX as usize * width;
                out[lane * width..lane * width + width]
                    .copy_from_slice(&bytes[start..start + width]);
            }
            V128::from_bytes(out)
        }
    };
}

vreplve!(
    /// `vreplvei.b`
    vreplve_b,
    u8,
    16
);
vreplve!(
    /// `vreplvei.h`
    vreplve_h,
    u16,
    8
);
vreplve!(
    /// `vreplvei.w`
    vreplve_w,
    u32,
    4
);
vreplve!(
    /// `vreplvei.d`
    vreplve_d,
    u64,
    2
);

/// `vreplvei.w`
#[inline(always)]
pub fn vreplve<const INDEX: u8>(a: V128) -> V128 {
    vreplve_w::<INDEX>(a)
}
