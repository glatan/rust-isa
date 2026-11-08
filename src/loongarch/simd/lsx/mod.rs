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
//! # Compile-time immediate check
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsat, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsat::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vseqi_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vseqi_b::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vseqi_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vseqi_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vseqi_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vseqi_w::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vseqi_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vseqi_d::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslei_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslei_b::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslei_bu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslei_bu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslei_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslei_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslei_hu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslei_hu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslei_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslei_w::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslei_wu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslei_wu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslei_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslei_d::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslei_du, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslei_du::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslti_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslti_b::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslti_bu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslti_bu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslti_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslti_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslti_hu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslti_hu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslti_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslti_w::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslti_wu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslti_wu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslti_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslti_d::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslti_du, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslti_du::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslli_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslli_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslli_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslli_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslli_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslli_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vslli_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vslli_d::<64>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrli_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrli_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrli_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrli_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrli_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrli_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrli_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrli_d::<64>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrai_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrai_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrai_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrai_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrai_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrai_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrai_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrai_d::<64>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmaxi_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmaxi_b::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmaxi_bu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmaxi_bu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmaxi_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmaxi_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmaxi_hu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmaxi_hu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmaxi_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmaxi_w::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmaxi_wu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmaxi_wu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmaxi_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmaxi_d::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmaxi_du, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmaxi_du::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmini_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmini_b::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmini_bu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmini_bu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmini_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmini_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmini_hu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmini_hu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmini_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmini_w::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmini_wu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmini_wu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmini_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmini_d::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vmini_du, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vmini_du::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vrotri_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vrotri_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vrotri_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vrotri_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vrotri_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vrotri_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vrotri_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vrotri_d::<64>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitclri_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitclri_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitclri_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitclri_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitclri_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitclri_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitclri_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitclri_d::<64>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitseti_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitseti_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitseti_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitseti_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitseti_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitseti_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitseti_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitseti_d::<64>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitrevi_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitrevi_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitrevi_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitrevi_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitrevi_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitrevi_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vbitrevi_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vbitrevi_d::<64>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vaddi_bu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vaddi_bu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vaddi_hu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vaddi_hu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vaddi_wu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vaddi_wu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vaddi_du, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vaddi_du::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsubi_bu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsubi_bu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsubi_hu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsubi_hu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsubi_wu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsubi_wu::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsubi_du, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsubi_du::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrlri_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrlri_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrlri_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrlri_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrlri_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrlri_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrlri_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrlri_d::<64>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrari_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrari_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrari_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrari_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrari_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrari_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsrari_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsrari_d::<64>(v);
//! ```
//!
//!

//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vstelm_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//! let _ = unsafe { vstelm_b::<16>(v, storage.0.as_mut_ptr(), 0) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vreplvei_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vreplvei_b::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vinsgr2vr_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vinsgr2vr_b::<16>(v, 1);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vpickve2gr_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vpickve2gr_b::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vpickve2gr_bu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vpickve2gr_bu::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vstelm_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//! let _ = unsafe { vstelm_h::<8>(v, storage.0.as_mut_ptr(), 0) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vreplvei_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vreplvei_h::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vinsgr2vr_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vinsgr2vr_h::<8>(v, 1);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vpickve2gr_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vpickve2gr_h::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vpickve2gr_hu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vpickve2gr_hu::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vstelm_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//! let _ = unsafe { vstelm_w::<4>(v, storage.0.as_mut_ptr(), 0) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vreplvei_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vreplvei_w::<4>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vinsgr2vr_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vinsgr2vr_w::<4>(v, 1);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vpickve2gr_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vpickve2gr_w::<4>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vpickve2gr_wu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vpickve2gr_wu::<4>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vstelm_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! #[repr(align(64))]
//! struct Aligned([u8; 4096]);
//! let mut storage = Aligned([0; 4096]);
//! let _ = unsafe { vstelm_d::<2>(v, storage.0.as_mut_ptr(), 0) };
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vreplvei_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vreplvei_d::<2>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vinsgr2vr_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vinsgr2vr_d::<2>(v, 1i64);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vpickve2gr_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vpickve2gr_d::<2>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vpickve2gr_du, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vpickve2gr_du::<2>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vfrstpi_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vfrstpi_b::<32>(v, v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vfrstpi_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vfrstpi_h::<32>(v, v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vssrarn_w_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vssrarn_w_d::<64>(v, v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vssrarn_wu_du, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vssrarn_wu_du::<64>(v, v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vfcmp, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vfcmp::<18>(v, v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vfcmp_d, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vfcmp_d::<18>(v, v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsllwil_h_b, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsllwil_h_b::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsllwil_hu_bu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsllwil_hu_bu::<8>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsllwil_w_h, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsllwil_w_h::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsllwil_wu_hu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsllwil_wu_hu::<16>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsllwil_d_w, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsllwil_d_w::<32>(v);
//! ```
//!
//! ```compile_fail
//! use rust_isa::loongarch::simd::lsx::{vsllwil_du_wu, V128};
//!
//! let v = V128::from_bytes([0; 16]);
//! let _ = vsllwil_du_wu::<32>(v);
//! ```
//!
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
//! let _ = vfcmp_caf_s(v128, v128);
//! let _ = vfmadd_s(v128, v128, v128);
//! let _ = vfmadd_d(v128, v128, v128);
//! let _ = vfmsub_s(v128, v128, v128);
//! let _ = vfmsub_d(v128, v128, v128);
//! let _ = vfnmadd_s(v128, v128, v128);
//! let _ = vfnmadd_d(v128, v128, v128);
//! let _ = vfnmsub_s(v128, v128, v128);
//! let _ = vfnmsub_d(v128, v128, v128);
//! let _ = vfcmp_saf_s(v128, v128);
//! let _ = vfcmp_clt_s(v128, v128);
//! let _ = vfcmp_slt_s(v128, v128);
//! let _ = vfcmp_ceq_s(v128, v128);
//! let _ = vfcmp_seq_s(v128, v128);
//! let _ = vfcmp_cle_s(v128, v128);
//! let _ = vfcmp_sle_s(v128, v128);
//! let _ = vfcmp_cun_s(v128, v128);
//! let _ = vfcmp_sun_s(v128, v128);
//! let _ = vfcmp_cult_s(v128, v128);
//! let _ = vfcmp_sult_s(v128, v128);
//! let _ = vfcmp_cueq_s(v128, v128);
//! let _ = vfcmp_sueq_s(v128, v128);
//! let _ = vfcmp_cule_s(v128, v128);
//! let _ = vfcmp_sule_s(v128, v128);
//! let _ = vfcmp_cne_s(v128, v128);
//! let _ = vfcmp_sne_s(v128, v128);
//! let _ = vfcmp_cor_s(v128, v128);
//! let _ = vfcmp_sor_s(v128, v128);
//! let _ = vfcmp_cune_s(v128, v128);
//! let _ = vfcmp_sune_s(v128, v128);
//! let _ = vfcmp_caf_d(v128, v128);
//! let _ = vfcmp_saf_d(v128, v128);
//! let _ = vfcmp_clt_d(v128, v128);
//! let _ = vfcmp_slt_d(v128, v128);
//! let _ = vfcmp_ceq_d(v128, v128);
//! let _ = vfcmp_seq_d(v128, v128);
//! let _ = vfcmp_cle_d(v128, v128);
//! let _ = vfcmp_sle_d(v128, v128);
//! let _ = vfcmp_cun_d(v128, v128);
//! let _ = vfcmp_sun_d(v128, v128);
//! let _ = vfcmp_cult_d(v128, v128);
//! let _ = vfcmp_sult_d(v128, v128);
//! let _ = vfcmp_cueq_d(v128, v128);
//! let _ = vfcmp_sueq_d(v128, v128);
//! let _ = vfcmp_cule_d(v128, v128);
//! let _ = vfcmp_sule_d(v128, v128);
//! let _ = vfcmp_cne_d(v128, v128);
//! let _ = vfcmp_sne_d(v128, v128);
//! let _ = vfcmp_cor_d(v128, v128);
//! let _ = vfcmp_sor_d(v128, v128);
//! let _ = vfcmp_cune_d(v128, v128);
//! let _ = vfcmp_sune_d(v128, v128);
//! let _ = vfcmp_d::<0>(v128, v128);
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

//! let _ = vbitsel_v(v128, v128, v128);
//! let _ = vshuf_b(v128, v128, v128);
//! let _ = unsafe { vldx(storage.0.as_ptr(), 0) };
//! let _ = unsafe { vstx(v128, storage.0.as_mut_ptr(), 0) };
//! let _ = vseq_b(v128, v128);
//! let _ = vseq_h(v128, v128);
//! let _ = vseq_w(v128, v128);
//! let _ = vseq_d(v128, v128);
//! let _ = vsle_b(v128, v128);
//! let _ = vsle_h(v128, v128);
//! let _ = vsle_w(v128, v128);
//! let _ = vsle_d(v128, v128);
//! let _ = vsle_bu(v128, v128);
//! let _ = vsle_hu(v128, v128);
//! let _ = vsle_wu(v128, v128);
//! let _ = vsle_du(v128, v128);
//! let _ = vslt_b(v128, v128);
//! let _ = vslt_h(v128, v128);
//! let _ = vslt_w(v128, v128);
//! let _ = vslt_d(v128, v128);
//! let _ = vslt_bu(v128, v128);
//! let _ = vslt_hu(v128, v128);
//! let _ = vslt_wu(v128, v128);
//! let _ = vslt_du(v128, v128);
//! let _ = vadd_b(v128, v128);
//! let _ = vadd_h(v128, v128);
//! let _ = vadd_w(v128, v128);
//! let _ = vadd_d(v128, v128);
//! let _ = vsub_b(v128, v128);
//! let _ = vsub_h(v128, v128);
//! let _ = vsub_w(v128, v128);
//! let _ = vsub_d(v128, v128);
//! let _ = vaddwev_h_b(v128, v128);
//! let _ = vaddwev_w_h(v128, v128);
//! let _ = vaddwev_d_w(v128, v128);
//! let _ = vaddwev_q_d(v128, v128);
//! let _ = vsubwev_h_b(v128, v128);
//! let _ = vsubwev_w_h(v128, v128);
//! let _ = vsubwev_d_w(v128, v128);
//! let _ = vsubwev_q_d(v128, v128);
//! let _ = vaddwod_h_b(v128, v128);
//! let _ = vaddwod_w_h(v128, v128);
//! let _ = vaddwod_d_w(v128, v128);
//! let _ = vaddwod_q_d(v128, v128);
//! let _ = vsubwod_h_b(v128, v128);
//! let _ = vsubwod_w_h(v128, v128);
//! let _ = vsubwod_d_w(v128, v128);
//! let _ = vsubwod_q_d(v128, v128);
//! let _ = vaddwev_h_bu(v128, v128);
//! let _ = vaddwev_w_hu(v128, v128);
//! let _ = vaddwev_d_wu(v128, v128);
//! let _ = vaddwev_q_du(v128, v128);
//! let _ = vsubwev_h_bu(v128, v128);
//! let _ = vsubwev_w_hu(v128, v128);
//! let _ = vsubwev_d_wu(v128, v128);
//! let _ = vsubwev_q_du(v128, v128);
//! let _ = vaddwod_h_bu(v128, v128);
//! let _ = vaddwod_w_hu(v128, v128);
//! let _ = vaddwod_d_wu(v128, v128);
//! let _ = vaddwod_q_du(v128, v128);
//! let _ = vsubwod_h_bu(v128, v128);
//! let _ = vsubwod_w_hu(v128, v128);
//! let _ = vsubwod_d_wu(v128, v128);
//! let _ = vsubwod_q_du(v128, v128);
//! let _ = vaddwev_h_bu_b(v128, v128);
//! let _ = vaddwev_w_hu_h(v128, v128);
//! let _ = vaddwev_d_wu_w(v128, v128);
//! let _ = vaddwev_q_du_d(v128, v128);
//! let _ = vaddwod_h_bu_b(v128, v128);
//! let _ = vaddwod_w_hu_h(v128, v128);
//! let _ = vaddwod_d_wu_w(v128, v128);
//! let _ = vaddwod_q_du_d(v128, v128);
//! let _ = vsadd_b(v128, v128);
//! let _ = vsadd_h(v128, v128);
//! let _ = vsadd_w(v128, v128);
//! let _ = vsadd_d(v128, v128);
//! let _ = vssub_b(v128, v128);
//! let _ = vssub_h(v128, v128);
//! let _ = vssub_w(v128, v128);
//! let _ = vssub_d(v128, v128);
//! let _ = vsadd_bu(v128, v128);
//! let _ = vsadd_hu(v128, v128);
//! let _ = vsadd_wu(v128, v128);
//! let _ = vsadd_du(v128, v128);
//! let _ = vssub_bu(v128, v128);
//! let _ = vssub_hu(v128, v128);
//! let _ = vssub_wu(v128, v128);
//! let _ = vssub_du(v128, v128);
//! let _ = vhaddw_h_b(v128, v128);
//! let _ = vhaddw_w_h(v128, v128);
//! let _ = vhaddw_d_w(v128, v128);
//! let _ = vhaddw_q_d(v128, v128);
//! let _ = vhsubw_h_b(v128, v128);
//! let _ = vhsubw_w_h(v128, v128);
//! let _ = vhsubw_d_w(v128, v128);
//! let _ = vhsubw_q_d(v128, v128);
//! let _ = vhaddw_hu_bu(v128, v128);
//! let _ = vhaddw_wu_hu(v128, v128);
//! let _ = vhaddw_du_wu(v128, v128);
//! let _ = vhaddw_qu_du(v128, v128);
//! let _ = vhsubw_hu_bu(v128, v128);
//! let _ = vhsubw_wu_hu(v128, v128);
//! let _ = vhsubw_du_wu(v128, v128);
//! let _ = vhsubw_qu_du(v128, v128);
//! let _ = vadda_b(v128, v128);
//! let _ = vadda_h(v128, v128);
//! let _ = vadda_w(v128, v128);
//! let _ = vadda_d(v128, v128);
//! let _ = vabsd_b(v128, v128);
//! let _ = vabsd_h(v128, v128);
//! let _ = vabsd_w(v128, v128);
//! let _ = vabsd_d(v128, v128);
//! let _ = vabsd_bu(v128, v128);
//! let _ = vabsd_hu(v128, v128);
//! let _ = vabsd_wu(v128, v128);
//! let _ = vabsd_du(v128, v128);
//! let _ = vavg_b(v128, v128);
//! let _ = vavg_h(v128, v128);
//! let _ = vavg_w(v128, v128);
//! let _ = vavg_d(v128, v128);
//! let _ = vavg_bu(v128, v128);
//! let _ = vavg_hu(v128, v128);
//! let _ = vavg_wu(v128, v128);
//! let _ = vavg_du(v128, v128);
//! let _ = vmax_b(v128, v128);
//! let _ = vmax_h(v128, v128);
//! let _ = vmax_w(v128, v128);
//! let _ = vmax_d(v128, v128);
//! let _ = vmin_b(v128, v128);
//! let _ = vmin_h(v128, v128);
//! let _ = vmin_w(v128, v128);
//! let _ = vmin_d(v128, v128);
//! let _ = vmax_bu(v128, v128);
//! let _ = vmax_hu(v128, v128);
//! let _ = vmax_wu(v128, v128);
//! let _ = vmax_du(v128, v128);
//! let _ = vmin_bu(v128, v128);
//! let _ = vmin_hu(v128, v128);
//! let _ = vmin_wu(v128, v128);
//! let _ = vmin_du(v128, v128);
//! let _ = vmul_b(v128, v128);
//! let _ = vmul_h(v128, v128);
//! let _ = vmul_w(v128, v128);
//! let _ = vmul_d(v128, v128);
//! let _ = vmuh_b(v128, v128);
//! let _ = vmuh_h(v128, v128);
//! let _ = vmuh_w(v128, v128);
//! let _ = vmuh_d(v128, v128);
//! let _ = vmuh_bu(v128, v128);
//! let _ = vmuh_hu(v128, v128);
//! let _ = vmuh_wu(v128, v128);
//! let _ = vmuh_du(v128, v128);
//! let _ = vmulwev_h_b(v128, v128);
//! let _ = vmulwev_w_h(v128, v128);
//! let _ = vmulwev_d_w(v128, v128);
//! let _ = vmulwev_q_d(v128, v128);
//! let _ = vmulwod_h_b(v128, v128);
//! let _ = vmulwod_w_h(v128, v128);
//! let _ = vmulwod_d_w(v128, v128);
//! let _ = vmulwod_q_d(v128, v128);
//! let _ = vmulwev_h_bu(v128, v128);
//! let _ = vmulwev_w_hu(v128, v128);
//! let _ = vmulwev_d_wu(v128, v128);
//! let _ = vmulwev_q_du(v128, v128);
//! let _ = vmulwod_h_bu(v128, v128);
//! let _ = vmulwod_w_hu(v128, v128);
//! let _ = vmulwod_d_wu(v128, v128);
//! let _ = vmulwod_q_du(v128, v128);
//! let _ = vmulwev_h_bu_b(v128, v128);
//! let _ = vmulwev_w_hu_h(v128, v128);
//! let _ = vmulwev_d_wu_w(v128, v128);
//! let _ = vmulwev_q_du_d(v128, v128);
//! let _ = vmulwod_h_bu_b(v128, v128);
//! let _ = vmulwod_w_hu_h(v128, v128);
//! let _ = vmulwod_d_wu_w(v128, v128);
//! let _ = vmulwod_q_du_d(v128, v128);
//! let _ = vmadd_b(v128, v128, v128);
//! let _ = vmadd_h(v128, v128, v128);
//! let _ = vmadd_w(v128, v128, v128);
//! let _ = vmadd_d(v128, v128, v128);
//! let _ = vmsub_b(v128, v128, v128);
//! let _ = vmsub_h(v128, v128, v128);
//! let _ = vmsub_w(v128, v128, v128);
//! let _ = vmsub_d(v128, v128, v128);
//! let _ = vmaddwev_h_b(v128, v128, v128);
//! let _ = vmaddwev_w_h(v128, v128, v128);
//! let _ = vmaddwev_d_w(v128, v128, v128);
//! let _ = vmaddwev_q_d(v128, v128, v128);
//! let _ = vmaddwod_h_b(v128, v128, v128);
//! let _ = vmaddwod_w_h(v128, v128, v128);
//! let _ = vmaddwod_d_w(v128, v128, v128);
//! let _ = vmaddwod_q_d(v128, v128, v128);
//! let _ = vmaddwev_h_bu(v128, v128, v128);
//! let _ = vmaddwev_w_hu(v128, v128, v128);
//! let _ = vmaddwev_d_wu(v128, v128, v128);
//! let _ = vmaddwev_q_du(v128, v128, v128);
//! let _ = vmaddwod_h_bu(v128, v128, v128);
//! let _ = vmaddwod_w_hu(v128, v128, v128);
//! let _ = vmaddwod_d_wu(v128, v128, v128);
//! let _ = vmaddwod_q_du(v128, v128, v128);
//! let _ = vmaddwev_h_bu_b(v128, v128, v128);
//! let _ = vmaddwev_w_hu_h(v128, v128, v128);
//! let _ = vmaddwev_d_wu_w(v128, v128, v128);
//! let _ = vmaddwev_q_du_d(v128, v128, v128);
//! let _ = vmaddwod_h_bu_b(v128, v128, v128);
//! let _ = vmaddwod_w_hu_h(v128, v128, v128);
//! let _ = vmaddwod_d_wu_w(v128, v128, v128);
//! let _ = vmaddwod_q_du_d(v128, v128, v128);
//! let _ = vdiv_b(v128, v128);
//! let _ = vdiv_h(v128, v128);
//! let _ = vdiv_w(v128, v128);
//! let _ = vdiv_d(v128, v128);
//! let _ = vmod_b(v128, v128);
//! let _ = vmod_h(v128, v128);
//! let _ = vmod_w(v128, v128);
//! let _ = vmod_d(v128, v128);
//! let _ = vdiv_bu(v128, v128);
//! let _ = vdiv_hu(v128, v128);
//! let _ = vdiv_wu(v128, v128);
//! let _ = vdiv_du(v128, v128);
//! let _ = vmod_bu(v128, v128);
//! let _ = vmod_hu(v128, v128);
//! let _ = vmod_wu(v128, v128);
//! let _ = vmod_du(v128, v128);
//! let _ = vsll_b(v128, v128);
//! let _ = vsll_h(v128, v128);
//! let _ = vsll_w(v128, v128);
//! let _ = vsll_d(v128, v128);
//! let _ = vsrl_b(v128, v128);
//! let _ = vsrl_h(v128, v128);
//! let _ = vsrl_w(v128, v128);
//! let _ = vsrl_d(v128, v128);
//! let _ = vsra_b(v128, v128);
//! let _ = vsra_h(v128, v128);
//! let _ = vsra_w(v128, v128);
//! let _ = vsra_d(v128, v128);
//! let _ = vrotr_b(v128, v128);
//! let _ = vrotr_h(v128, v128);
//! let _ = vrotr_w(v128, v128);
//! let _ = vrotr_d(v128, v128);
//! let _ = vsrlr_b(v128, v128);
//! let _ = vsrlr_h(v128, v128);
//! let _ = vsrlr_w(v128, v128);
//! let _ = vsrlr_d(v128, v128);
//! let _ = vsrar_b(v128, v128);
//! let _ = vsrar_h(v128, v128);
//! let _ = vsrar_w(v128, v128);
//! let _ = vsrar_d(v128, v128);
//! let _ = vsrln_b_h(v128, v128);
//! let _ = vsrln_h_w(v128, v128);
//! let _ = vsrln_w_d(v128, v128);
//! let _ = vsran_b_h(v128, v128);
//! let _ = vsran_h_w(v128, v128);
//! let _ = vsran_w_d(v128, v128);
//! let _ = vsrlrn_b_h(v128, v128);
//! let _ = vsrlrn_h_w(v128, v128);
//! let _ = vsrlrn_w_d(v128, v128);
//! let _ = vsrarn_b_h(v128, v128);
//! let _ = vsrarn_h_w(v128, v128);
//! let _ = vsrarn_w_d(v128, v128);
//! let _ = vssrln_b_h(v128, v128);
//! let _ = vssrln_h_w(v128, v128);
//! let _ = vssran_b_h(v128, v128);
//! let _ = vssran_h_w(v128, v128);
//! let _ = vssran_w_d(v128, v128);
//! let _ = vssrlrn_b_h(v128, v128);
//! let _ = vssrlrn_h_w(v128, v128);
//! let _ = vssrarn_b_h(v128, v128);
//! let _ = vssrarn_h_w(v128, v128);
//! let _ = vssrln_bu_h(v128, v128);
//! let _ = vssrln_hu_w(v128, v128);
//! let _ = vssrln_wu_d(v128, v128);
//! let _ = vssran_bu_h(v128, v128);
//! let _ = vssran_hu_w(v128, v128);
//! let _ = vssran_wu_d(v128, v128);
//! let _ = vssrlrn_bu_h(v128, v128);
//! let _ = vssrlrn_hu_w(v128, v128);
//! let _ = vssrlrn_wu_d(v128, v128);
//! let _ = vssrarn_bu_h(v128, v128);
//! let _ = vssrarn_hu_w(v128, v128);
//! let _ = vssrarn_wu_d(v128, v128);
//! let _ = vbitclr_b(v128, v128);
//! let _ = vbitclr_h(v128, v128);
//! let _ = vbitclr_w(v128, v128);
//! let _ = vbitclr_d(v128, v128);
//! let _ = vbitset_b(v128, v128);
//! let _ = vbitset_h(v128, v128);
//! let _ = vbitset_w(v128, v128);
//! let _ = vbitset_d(v128, v128);
//! let _ = vbitrev_b(v128, v128);
//! let _ = vbitrev_h(v128, v128);
//! let _ = vbitrev_w(v128, v128);
//! let _ = vbitrev_d(v128, v128);
//! let _ = vpackev_b(v128, v128);
//! let _ = vpackev_h(v128, v128);
//! let _ = vpackev_w(v128, v128);
//! let _ = vpackev_d(v128, v128);
//! let _ = vpackod_b(v128, v128);
//! let _ = vpackod_h(v128, v128);
//! let _ = vpackod_w(v128, v128);
//! let _ = vpackod_d(v128, v128);
//! let _ = vilvl_b(v128, v128);
//! let _ = vilvl_h(v128, v128);
//! let _ = vilvl_w(v128, v128);
//! let _ = vilvl_d(v128, v128);
//! let _ = vilvh_b(v128, v128);
//! let _ = vilvh_h(v128, v128);
//! let _ = vilvh_w(v128, v128);
//! let _ = vilvh_d(v128, v128);
//! let _ = vpickev_b(v128, v128);
//! let _ = vpickev_h(v128, v128);
//! let _ = vpickev_w(v128, v128);
//! let _ = vpickev_d(v128, v128);
//! let _ = vpickod_b(v128, v128);
//! let _ = vpickod_h(v128, v128);
//! let _ = vpickod_w(v128, v128);
//! let _ = vpickod_d(v128, v128);
//! let _ = vand_v(v128, v128);
//! let _ = vor_v(v128, v128);
//! let _ = vxor_v(v128, v128);
//! let _ = vnor_v(v128, v128);
//! let _ = vandn_v(v128, v128);
//! let _ = vorn_v(v128, v128);
//! let _ = vfrstp_b(v128, v128, v128);
//! let _ = vfrstp_h(v128, v128, v128);
//! let _ = vadd_q(v128, v128);
//! let _ = vsub_q(v128, v128);
//! let _ = vsigncov_b(v128, v128);
//! let _ = vsigncov_h(v128, v128);
//! let _ = vsigncov_w(v128, v128);
//! let _ = vsigncov_d(v128, v128);
//! let _ = vshuf_h(v128, v128, v128);
//! let _ = vshuf_w(v128, v128, v128);
//! let _ = vshuf_d(v128, v128, v128);
//! let _ = vclo_b(v128);
//! let _ = vclo_h(v128);
//! let _ = vclo_w(v128);
//! let _ = vclo_d(v128);
//! let _ = vclz_b(v128);
//! let _ = vclz_h(v128);
//! let _ = vclz_w(v128);
//! let _ = vclz_d(v128);
//! let _ = vpcnt_b(v128);
//! let _ = vpcnt_h(v128);
//! let _ = vpcnt_w(v128);
//! let _ = vpcnt_d(v128);
//! let _ = vneg_b(v128);
//! let _ = vneg_h(v128);
//! let _ = vneg_w(v128);
//! let _ = vneg_d(v128);
//! let _ = vmskltz_b(v128);
//! let _ = vmskltz_h(v128);
//! let _ = vmskltz_w(v128);
//! let _ = vmskltz_d(v128);
//! let _ = vmskgez_b(v128);
//! let _ = vmsknz_b(v128);
//! let _ = vfclass_s(v128);
//! let _ = vfclass_d(v128);
//! let _ = vftintl_l_s(v128);
//! let _ = vftinth_l_s(v128);
//! let _ = vftintrml_l_s(v128);
//! let _ = vftintrmh_l_s(v128);
//! let _ = vftintrpl_l_s(v128);
//! let _ = vftintrph_l_s(v128);
//! let _ = vftintrzl_l_s(v128);
//! let _ = vftintrzh_l_s(v128);
//! let _ = vftintrnel_l_s(v128);
//! let _ = vftintrneh_l_s(v128);
//! let _ = vextl_q_d(v128);
//! let _ = vslei_bu::<1>(v128);
//! let _ = vslti_bu::<1>(v128);
//! let _ = vmaxi_bu::<1>(v128);
//! let _ = vmini_bu::<1>(v128);
//! let _ = vslei_hu::<1>(v128);
//! let _ = vslti_hu::<1>(v128);
//! let _ = vmaxi_hu::<1>(v128);
//! let _ = vmini_hu::<1>(v128);
//! let _ = vslei_wu::<1>(v128);
//! let _ = vslti_wu::<1>(v128);
//! let _ = vmaxi_wu::<1>(v128);
//! let _ = vmini_wu::<1>(v128);
//! let _ = vslei_du::<1>(v128);
//! let _ = vslti_du::<1>(v128);
//! let _ = vmaxi_du::<1>(v128);
//! let _ = vmini_du::<1>(v128);
//! let _ = vseqi_b::<1>(v128);
//! let _ = vslei_b::<1>(v128);
//! let _ = vslei_bu::<1>(v128);
//! let _ = vslti_b::<1>(v128);
//! let _ = vslti_bu::<1>(v128);
//! let _ = vslli_b::<1>(v128);
//! let _ = vsrli_b::<1>(v128);
//! let _ = vsrai_b::<1>(v128);
//! let _ = vmaxi_b::<1>(v128);
//! let _ = vmaxi_bu::<1>(v128);
//! let _ = vmini_b::<1>(v128);
//! let _ = vmini_bu::<1>(v128);
//! let _ = vseqi_h::<1>(v128);
//! let _ = vslei_h::<1>(v128);
//! let _ = vslei_hu::<1>(v128);
//! let _ = vslti_h::<1>(v128);
//! let _ = vslti_hu::<1>(v128);
//! let _ = vslli_h::<1>(v128);
//! let _ = vsrli_h::<1>(v128);
//! let _ = vsrai_h::<1>(v128);
//! let _ = vmaxi_h::<1>(v128);
//! let _ = vmaxi_hu::<1>(v128);
//! let _ = vmini_h::<1>(v128);
//! let _ = vmini_hu::<1>(v128);
//! let _ = vseqi_w::<1>(v128);
//! let _ = vslei_w::<1>(v128);
//! let _ = vslei_wu::<1>(v128);
//! let _ = vslti_w::<1>(v128);
//! let _ = vslti_wu::<1>(v128);
//! let _ = vslli_w::<1>(v128);
//! let _ = vsrli_w::<1>(v128);
//! let _ = vsrai_w::<1>(v128);
//! let _ = vmaxi_w::<1>(v128);
//! let _ = vmaxi_wu::<1>(v128);
//! let _ = vmini_w::<1>(v128);
//! let _ = vmini_wu::<1>(v128);
//! let _ = vseqi_d::<1>(v128);
//! let _ = vslei_d::<1>(v128);
//! let _ = vslei_du::<1>(v128);
//! let _ = vslti_d::<1>(v128);
//! let _ = vslti_du::<1>(v128);
//! let _ = vslli_d::<1>(v128);
//! let _ = vsrli_d::<1>(v128);
//! let _ = vsrai_d::<1>(v128);
//! let _ = vmaxi_d::<1>(v128);
//! let _ = vmaxi_du::<1>(v128);
//! let _ = vmini_d::<1>(v128);
//! let _ = vmini_du::<1>(v128);
//! let _ = vrotri_b::<1>(v128);
//! let _ = vbitclri_b::<1>(v128);
//! let _ = vbitseti_b::<1>(v128);
//! let _ = vbitrevi_b::<1>(v128);
//! let _ = vrotri_h::<1>(v128);
//! let _ = vbitclri_h::<1>(v128);
//! let _ = vbitseti_h::<1>(v128);
//! let _ = vbitrevi_h::<1>(v128);
//! let _ = vrotri_w::<1>(v128);
//! let _ = vbitclri_w::<1>(v128);
//! let _ = vbitseti_w::<1>(v128);
//! let _ = vbitrevi_w::<1>(v128);
//! let _ = vrotri_d::<1>(v128);
//! let _ = vbitclri_d::<1>(v128);
//! let _ = vbitseti_d::<1>(v128);
//! let _ = vbitrevi_d::<1>(v128);
//! let _ = vandi_b::<1>(v128);
//! let _ = vori_b::<1>(v128);
//! let _ = vxori_b::<1>(v128);
//! let _ = vnori_b::<1>(v128);
//! let _ = vaddi_bu::<1>(v128);
//! let _ = vsubi_bu::<1>(v128);
//! let _ = vsrlri_b::<1>(v128);
//! let _ = vsrari_b::<1>(v128);
//! let _ = vaddi_hu::<1>(v128);
//! let _ = vsubi_hu::<1>(v128);
//! let _ = vsrlri_h::<1>(v128);
//! let _ = vsrari_h::<1>(v128);
//! let _ = vaddi_wu::<1>(v128);
//! let _ = vsubi_wu::<1>(v128);
//! let _ = vsrlri_w::<1>(v128);
//! let _ = vsrari_w::<1>(v128);
//! let _ = vaddi_du::<1>(v128);
//! let _ = vsubi_du::<1>(v128);
//! let _ = vsrlri_d::<1>(v128);
//! let _ = vsrari_d::<1>(v128);
//! let _ = vfmaxa_s(v128, v128);
//! let _ = vfmaxa_d(v128, v128);
//! let _ = vfmina_s(v128, v128);
//! let _ = vfmina_d(v128, v128);
//! let _ = vflogb_s(v128);
//! let _ = vflogb_d(v128);
//! let _ = vfsqrt_s(v128);
//! let _ = vfsqrt_d(v128);
//! let _ = vfrecip_s(v128);
//! let _ = vfrecip_d(v128);
//! let _ = vfrsqrt_s(v128);
//! let _ = vfrsqrt_d(v128);
//! let _ = vfrint_s(v128);
//! let _ = vfrint_d(v128);
//! let _ = vfrintrm_s(v128);
//! let _ = vfrintrm_d(v128);
//! let _ = vfrintrp_s(v128);
//! let _ = vfrintrp_d(v128);
//! let _ = vfrintrz_s(v128);
//! let _ = vfrintrz_d(v128);
//! let _ = vfrintrne_s(v128);
//! let _ = vfrintrne_d(v128);
//! let _ = unsafe { vldrepl_b(storage.0.as_ptr(), 0) };
//! let _ = unsafe { vldrepl_h(storage.0.as_ptr(), 0) };
//! let _ = unsafe { vldrepl_w(storage.0.as_ptr(), 0) };
//! let _ = unsafe { vldrepl_d(storage.0.as_ptr(), 0) };
//! let _ = unsafe { vstelm_b::<0>(v128, storage.0.as_mut_ptr(), 0) };
//! let _ = unsafe { vstelm_h::<0>(v128, storage.0.as_mut_ptr(), 0) };
//! let _ = unsafe { vstelm_w::<0>(v128, storage.0.as_mut_ptr(), 0) };
//! let _ = unsafe { vstelm_d::<0>(v128, storage.0.as_mut_ptr(), 0) };
//! let _ = vinsgr2vr_b::<0>(v128, 1);
//! let _ = vinsgr2vr_h::<0>(v128, 1);
//! let _ = vinsgr2vr_w::<0>(v128, 1);
//! let _ = vinsgr2vr_d::<0>(v128, 1);
//! let _ = vpickve2gr_b::<0>(v128);
//! let _ = vpickve2gr_h::<0>(v128);
//! let _ = vpickve2gr_w::<0>(v128);
//! let _ = vpickve2gr_d::<0>(v128);
//! let _ = vpickve2gr_bu::<0>(v128);
//! let _ = vpickve2gr_hu::<0>(v128);
//! let _ = vpickve2gr_wu::<0>(v128);
//! let _ = vpickve2gr_du::<0>(v128);
//! let _ = vsllwil_h_b::<1>(v128);
//! let _ = vsllwil_hu_bu::<1>(v128);
//! let _ = vsrlni_b_h::<1>(v128, v128);
//! let _ = vsrlrni_b_h::<1>(v128, v128);
//! let _ = vssrlni_b_h::<1>(v128, v128);
//! let _ = vssrlni_bu_h::<1>(v128, v128);
//! let _ = vssrlrni_b_h::<1>(v128, v128);
//! let _ = vssrlrni_bu_h::<1>(v128, v128);
//! let _ = vsrani_b_h::<1>(v128, v128);
//! let _ = vsrarni_b_h::<1>(v128, v128);
//! let _ = vssrani_b_h::<1>(v128, v128);
//! let _ = vssrani_bu_h::<1>(v128, v128);
//! let _ = vssrarni_b_h::<1>(v128, v128);
//! let _ = vssrarni_bu_h::<1>(v128, v128);
//! let _ = vsllwil_w_h::<1>(v128);
//! let _ = vsllwil_wu_hu::<1>(v128);
//! let _ = vsrlni_h_w::<1>(v128, v128);
//! let _ = vsrlrni_h_w::<1>(v128, v128);
//! let _ = vssrlni_h_w::<1>(v128, v128);
//! let _ = vssrlni_hu_w::<1>(v128, v128);
//! let _ = vssrlrni_h_w::<1>(v128, v128);
//! let _ = vssrlrni_hu_w::<1>(v128, v128);
//! let _ = vsrani_h_w::<1>(v128, v128);
//! let _ = vsrarni_h_w::<1>(v128, v128);
//! let _ = vssrani_h_w::<1>(v128, v128);
//! let _ = vssrani_hu_w::<1>(v128, v128);
//! let _ = vssrarni_h_w::<1>(v128, v128);
//! let _ = vssrarni_hu_w::<1>(v128, v128);
//! let _ = vsllwil_d_w::<1>(v128);
//! let _ = vsllwil_du_wu::<1>(v128);
//! let _ = vsrlni_w_d::<1>(v128, v128);
//! let _ = vsrlrni_w_d::<1>(v128, v128);
//! let _ = vssrlni_w_d::<1>(v128, v128);
//! let _ = vssrlni_wu_d::<1>(v128, v128);
//! let _ = vssrlrni_w_d::<1>(v128, v128);
//! let _ = vssrlrni_wu_d::<1>(v128, v128);
//! let _ = vsrani_w_d::<1>(v128, v128);
//! let _ = vsrarni_w_d::<1>(v128, v128);
//! let _ = vssrani_w_d::<1>(v128, v128);
//! let _ = vssrani_wu_d::<1>(v128, v128);
//! let _ = vssrarni_w_d::<1>(v128, v128);
//! let _ = vssrarni_wu_d::<1>(v128, v128);
//! let _ = vsrlni_d_q::<1>(v128, v128);
//! let _ = vsrlrni_d_q::<1>(v128, v128);
//! let _ = vssrlni_d_q::<1>(v128, v128);
//! let _ = vssrlni_du_q::<1>(v128, v128);
//! let _ = vssrlrni_d_q::<1>(v128, v128);
//! let _ = vssrlrni_du_q::<1>(v128, v128);
//! let _ = vsrani_d_q::<1>(v128, v128);
//! let _ = vsrarni_d_q::<1>(v128, v128);
//! let _ = vssrani_d_q::<1>(v128, v128);
//! let _ = vssrani_du_q::<1>(v128, v128);
//! let _ = vssrarni_d_q::<1>(v128, v128);
//! let _ = vssrarni_du_q::<1>(v128, v128);
//! let _ = vbsll(v128, 1);
//! let _ = vbsrl(v128, 1);
//! let _ = vseteqz(v128);
//! let _ = vsetnez(v128);
//! let _ = vsetanyeqz_b(v128);
//! let _ = vsetanyeqz_h(v128);
//! let _ = vsetanyeqz_w(v128);
//! let _ = vsetanyeqz_d(v128);
//! let _ = vsetallnez_b(v128);
//! let _ = vsetallnez_h(v128);
//! let _ = vsetallnez_w(v128);
//! let _ = vsetallnez_d(v128);
//! let _ = vbitseli_b::<0x0f>(v128, v128);
//! let _ = vshuf4i_b::<0>(v128);
//! let _ = vshuf4i_h::<0>(v128);
//! let _ = vshuf4i_w::<0>(v128);
//! let _ = vshuf4i_d::<0>(v128, v128);
//! let _ = vpermi_w::<0>(v128, v128);
//! let _ = vfrecipe_s(v128);
//! let _ = vfrecipe_d(v128);
//! let _ = vfrsqrte_s(v128);
//! let _ = vfrsqrte_d(v128);
//! let _ = vextrins_b::<0>(v128, v128);
//! let _ = vextrins_h::<0>(v128, v128);
//! let _ = vextrins_w::<0>(v128, v128);
//! let _ = vextrins_d::<0>(v128, v128);
//! let _ = vfrstpi_b::<0>(v128, v128);
//! let _ = vfrstpi_h::<0>(v128, v128);
//! let _ = vfcvt_s_d(v128, v128);
//! let _ = vfcvtl_s_h(v128);
//! let _ = vfcvth_s_h(v128);
//! let _ = vfcvtl_d_s(v128);
//! let _ = vfcvth_d_s(v128);
//! let _ = vfcvt_h_s(v128, v128);
//! let _ = vrepl_b(0);
//! let _ = vrepl_h(0);
//! let _ = vrepl_w(0);
//! let _ = vrepl_d(0);
//! let _ = vreplgr2vr_b(0);
//! let _ = vreplgr2vr_h(0);
//! let _ = vreplgr2vr_w(0);
//! let _ = vreplgr2vr_d(0);
//! let _ = vreplve_b::<0>(v128);
//! let _ = vreplve_h::<0>(v128);
//! let _ = vreplve_w::<0>(v128);
//! let _ = vreplve_d::<0>(v128);
//! let _ = vreplvei_b::<0>(v128);
//! let _ = vreplvei_h::<0>(v128);
//! let _ = vreplvei_w::<0>(v128);
//! let _ = vreplvei_d::<0>(v128);
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
//! use rust_isa::loongarch::simd::lsx::vabsd_b as _;
//! use rust_isa::loongarch::simd::lsx::vabsd_bu as _;
//! use rust_isa::loongarch::simd::lsx::vabsd_d as _;
//! use rust_isa::loongarch::simd::lsx::vabsd_du as _;
//! use rust_isa::loongarch::simd::lsx::vabsd_h as _;
//! use rust_isa::loongarch::simd::lsx::vabsd_hu as _;
//! use rust_isa::loongarch::simd::lsx::vabsd_w as _;
//! use rust_isa::loongarch::simd::lsx::vabsd_wu as _;
//! use rust_isa::loongarch::simd::lsx::vadd as _;
//! use rust_isa::loongarch::simd::lsx::vadd_b as _;
//! use rust_isa::loongarch::simd::lsx::vadd_d as _;
//! use rust_isa::loongarch::simd::lsx::vadd_h as _;
//! use rust_isa::loongarch::simd::lsx::vadd_q as _;
//! use rust_isa::loongarch::simd::lsx::vadd_w as _;
//! use rust_isa::loongarch::simd::lsx::vadda_b as _;
//! use rust_isa::loongarch::simd::lsx::vadda_d as _;
//! use rust_isa::loongarch::simd::lsx::vadda_h as _;
//! use rust_isa::loongarch::simd::lsx::vadda_w as _;
//! use rust_isa::loongarch::simd::lsx::vaddi_bu as _;
//! use rust_isa::loongarch::simd::lsx::vaddi_du as _;
//! use rust_isa::loongarch::simd::lsx::vaddi_hu as _;
//! use rust_isa::loongarch::simd::lsx::vaddi_wu as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_d_wu as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_d_wu_w as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_h_bu as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_h_bu_b as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_q_du as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_q_du_d as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_w_hu as _;
//! use rust_isa::loongarch::simd::lsx::vaddwev_w_hu_h as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_d_wu as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_d_wu_w as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_h_bu as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_h_bu_b as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_q_du as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_q_du_d as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_w_hu as _;
//! use rust_isa::loongarch::simd::lsx::vaddwod_w_hu_h as _;
//! use rust_isa::loongarch::simd::lsx::vand as _;
//! use rust_isa::loongarch::simd::lsx::vand_v as _;
//! use rust_isa::loongarch::simd::lsx::vandi_b as _;
//! use rust_isa::loongarch::simd::lsx::vandn_v as _;
//! use rust_isa::loongarch::simd::lsx::vavg_b as _;
//! use rust_isa::loongarch::simd::lsx::vavg_bu as _;
//! use rust_isa::loongarch::simd::lsx::vavg_d as _;
//! use rust_isa::loongarch::simd::lsx::vavg_du as _;
//! use rust_isa::loongarch::simd::lsx::vavg_h as _;
//! use rust_isa::loongarch::simd::lsx::vavg_hu as _;
//! use rust_isa::loongarch::simd::lsx::vavg_w as _;
//! use rust_isa::loongarch::simd::lsx::vavg_wu as _;
//! use rust_isa::loongarch::simd::lsx::vavgr as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_b as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_bu as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_d as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_du as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_h as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_hu as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_w as _;
//! use rust_isa::loongarch::simd::lsx::vavgr_wu as _;
//! use rust_isa::loongarch::simd::lsx::vbitclr_b as _;
//! use rust_isa::loongarch::simd::lsx::vbitclr_d as _;
//! use rust_isa::loongarch::simd::lsx::vbitclr_h as _;
//! use rust_isa::loongarch::simd::lsx::vbitclr_w as _;
//! use rust_isa::loongarch::simd::lsx::vbitclri_b as _;
//! use rust_isa::loongarch::simd::lsx::vbitclri_d as _;
//! use rust_isa::loongarch::simd::lsx::vbitclri_h as _;
//! use rust_isa::loongarch::simd::lsx::vbitclri_w as _;
//! use rust_isa::loongarch::simd::lsx::vbitrev_b as _;
//! use rust_isa::loongarch::simd::lsx::vbitrev_d as _;
//! use rust_isa::loongarch::simd::lsx::vbitrev_h as _;
//! use rust_isa::loongarch::simd::lsx::vbitrev_w as _;
//! use rust_isa::loongarch::simd::lsx::vbitrevi_b as _;
//! use rust_isa::loongarch::simd::lsx::vbitrevi_d as _;
//! use rust_isa::loongarch::simd::lsx::vbitrevi_h as _;
//! use rust_isa::loongarch::simd::lsx::vbitrevi_w as _;
//! use rust_isa::loongarch::simd::lsx::vbitsel as _;
//! use rust_isa::loongarch::simd::lsx::vbitsel_v as _;
//! use rust_isa::loongarch::simd::lsx::vbitseli_b as _;
//! use rust_isa::loongarch::simd::lsx::vbitset_b as _;
//! use rust_isa::loongarch::simd::lsx::vbitset_d as _;
//! use rust_isa::loongarch::simd::lsx::vbitset_h as _;
//! use rust_isa::loongarch::simd::lsx::vbitset_w as _;
//! use rust_isa::loongarch::simd::lsx::vbitseti_b as _;
//! use rust_isa::loongarch::simd::lsx::vbitseti_d as _;
//! use rust_isa::loongarch::simd::lsx::vbitseti_h as _;
//! use rust_isa::loongarch::simd::lsx::vbitseti_w as _;
//! use rust_isa::loongarch::simd::lsx::vbsll as _;
//! use rust_isa::loongarch::simd::lsx::vbsrl as _;
//! use rust_isa::loongarch::simd::lsx::vclo_b as _;
//! use rust_isa::loongarch::simd::lsx::vclo_d as _;
//! use rust_isa::loongarch::simd::lsx::vclo_h as _;
//! use rust_isa::loongarch::simd::lsx::vclo_w as _;
//! use rust_isa::loongarch::simd::lsx::vclz_b as _;
//! use rust_isa::loongarch::simd::lsx::vclz_d as _;
//! use rust_isa::loongarch::simd::lsx::vclz_h as _;
//! use rust_isa::loongarch::simd::lsx::vclz_w as _;
//! use rust_isa::loongarch::simd::lsx::vdiv_b as _;
//! use rust_isa::loongarch::simd::lsx::vdiv_bu as _;
//! use rust_isa::loongarch::simd::lsx::vdiv_d as _;
//! use rust_isa::loongarch::simd::lsx::vdiv_du as _;
//! use rust_isa::loongarch::simd::lsx::vdiv_h as _;
//! use rust_isa::loongarch::simd::lsx::vdiv_hu as _;
//! use rust_isa::loongarch::simd::lsx::vdiv_w as _;
//! use rust_isa::loongarch::simd::lsx::vdiv_wu as _;
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
//! use rust_isa::loongarch::simd::lsx::vextl_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vextl_qu_du as _;
//! use rust_isa::loongarch::simd::lsx::vextrins_b as _;
//! use rust_isa::loongarch::simd::lsx::vextrins_d as _;
//! use rust_isa::loongarch::simd::lsx::vextrins_h as _;
//! use rust_isa::loongarch::simd::lsx::vextrins_w as _;
//! use rust_isa::loongarch::simd::lsx::vfadd as _;
//! use rust_isa::loongarch::simd::lsx::vfadd_d as _;
//! use rust_isa::loongarch::simd::lsx::vfadd_s as _;
//! use rust_isa::loongarch::simd::lsx::vfclass_d as _;
//! use rust_isa::loongarch::simd::lsx::vfclass_s as _;
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
//! use rust_isa::loongarch::simd::lsx::vfcvt_h_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcvt_s_d as _;
//! use rust_isa::loongarch::simd::lsx::vfcvth_d_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcvth_s_h as _;
//! use rust_isa::loongarch::simd::lsx::vfcvtl_d_s as _;
//! use rust_isa::loongarch::simd::lsx::vfcvtl_s_h as _;
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
//! use rust_isa::loongarch::simd::lsx::vflogb_d as _;
//! use rust_isa::loongarch::simd::lsx::vflogb_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmadd_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmadd_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmax as _;
//! use rust_isa::loongarch::simd::lsx::vfmax_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmax_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmaxa_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmaxa_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmin as _;
//! use rust_isa::loongarch::simd::lsx::vfmin_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmin_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmina_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmina_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmsub_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmsub_s as _;
//! use rust_isa::loongarch::simd::lsx::vfmul as _;
//! use rust_isa::loongarch::simd::lsx::vfmul_d as _;
//! use rust_isa::loongarch::simd::lsx::vfmul_s as _;
//! use rust_isa::loongarch::simd::lsx::vfnmadd_d as _;
//! use rust_isa::loongarch::simd::lsx::vfnmadd_s as _;
//! use rust_isa::loongarch::simd::lsx::vfnmsub_d as _;
//! use rust_isa::loongarch::simd::lsx::vfnmsub_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrecip_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrecip_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrecipe_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrecipe_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrint_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrint_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrintrm_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrintrm_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrintrne_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrintrne_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrintrp_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrintrp_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrintrz_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrintrz_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrsqrt_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrsqrt_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrsqrte_d as _;
//! use rust_isa::loongarch::simd::lsx::vfrsqrte_s as _;
//! use rust_isa::loongarch::simd::lsx::vfrstp_b as _;
//! use rust_isa::loongarch::simd::lsx::vfrstp_h as _;
//! use rust_isa::loongarch::simd::lsx::vfrstpi_b as _;
//! use rust_isa::loongarch::simd::lsx::vfrstpi_h as _;
//! use rust_isa::loongarch::simd::lsx::vfsqrt_d as _;
//! use rust_isa::loongarch::simd::lsx::vfsqrt_s as _;
//! use rust_isa::loongarch::simd::lsx::vfsub as _;
//! use rust_isa::loongarch::simd::lsx::vfsub_d as _;
//! use rust_isa::loongarch::simd::lsx::vfsub_s as _;
//! use rust_isa::loongarch::simd::lsx::vftint as _;
//! use rust_isa::loongarch::simd::lsx::vftint_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftint_lu_d as _;
//! use rust_isa::loongarch::simd::lsx::vftint_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftint_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftint_wu_s as _;
//! use rust_isa::loongarch::simd::lsx::vftinth_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintl_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrm_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrm_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrm_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrmh_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrml_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrne_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrne_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrne_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrneh_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrnel_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrp_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrp_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrp_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrph_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrpl_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_l_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_lu_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_w_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrz_wu_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrzh_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vftintrzl_l_s as _;
//! use rust_isa::loongarch::simd::lsx::vhaddw_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vhaddw_du_wu as _;
//! use rust_isa::loongarch::simd::lsx::vhaddw_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vhaddw_hu_bu as _;
//! use rust_isa::loongarch::simd::lsx::vhaddw_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vhaddw_qu_du as _;
//! use rust_isa::loongarch::simd::lsx::vhaddw_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vhaddw_wu_hu as _;
//! use rust_isa::loongarch::simd::lsx::vhsubw_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vhsubw_du_wu as _;
//! use rust_isa::loongarch::simd::lsx::vhsubw_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vhsubw_hu_bu as _;
//! use rust_isa::loongarch::simd::lsx::vhsubw_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vhsubw_qu_du as _;
//! use rust_isa::loongarch::simd::lsx::vhsubw_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vhsubw_wu_hu as _;
//! use rust_isa::loongarch::simd::lsx::vilvh_b as _;
//! use rust_isa::loongarch::simd::lsx::vilvh_d as _;
//! use rust_isa::loongarch::simd::lsx::vilvh_h as _;
//! use rust_isa::loongarch::simd::lsx::vilvh_w as _;
//! use rust_isa::loongarch::simd::lsx::vilvl_b as _;
//! use rust_isa::loongarch::simd::lsx::vilvl_d as _;
//! use rust_isa::loongarch::simd::lsx::vilvl_h as _;
//! use rust_isa::loongarch::simd::lsx::vilvl_w as _;
//! use rust_isa::loongarch::simd::lsx::vinsgr2vr_b as _;
//! use rust_isa::loongarch::simd::lsx::vinsgr2vr_d as _;
//! use rust_isa::loongarch::simd::lsx::vinsgr2vr_h as _;
//! use rust_isa::loongarch::simd::lsx::vinsgr2vr_w as _;
//! use rust_isa::loongarch::simd::lsx::vld as _;
//! use rust_isa::loongarch::simd::lsx::vldi as _;
//! use rust_isa::loongarch::simd::lsx::vldrepl_b as _;
//! use rust_isa::loongarch::simd::lsx::vldrepl_d as _;
//! use rust_isa::loongarch::simd::lsx::vldrepl_h as _;
//! use rust_isa::loongarch::simd::lsx::vldrepl_w as _;
//! use rust_isa::loongarch::simd::lsx::vldx as _;
//! use rust_isa::loongarch::simd::lsx::vmadd_b as _;
//! use rust_isa::loongarch::simd::lsx::vmadd_d as _;
//! use rust_isa::loongarch::simd::lsx::vmadd_h as _;
//! use rust_isa::loongarch::simd::lsx::vmadd_w as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_d_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_d_wu_w as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_h_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_h_bu_b as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_q_du as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_q_du_d as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_w_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwev_w_hu_h as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_d_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_d_wu_w as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_h_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_h_bu_b as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_q_du as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_q_du_d as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_w_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmaddwod_w_hu_h as _;
//! use rust_isa::loongarch::simd::lsx::vmax as _;
//! use rust_isa::loongarch::simd::lsx::vmax_b as _;
//! use rust_isa::loongarch::simd::lsx::vmax_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmax_d as _;
//! use rust_isa::loongarch::simd::lsx::vmax_du as _;
//! use rust_isa::loongarch::simd::lsx::vmax_h as _;
//! use rust_isa::loongarch::simd::lsx::vmax_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmax_w as _;
//! use rust_isa::loongarch::simd::lsx::vmax_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmaxi_b as _;
//! use rust_isa::loongarch::simd::lsx::vmaxi_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmaxi_d as _;
//! use rust_isa::loongarch::simd::lsx::vmaxi_du as _;
//! use rust_isa::loongarch::simd::lsx::vmaxi_h as _;
//! use rust_isa::loongarch::simd::lsx::vmaxi_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmaxi_w as _;
//! use rust_isa::loongarch::simd::lsx::vmaxi_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmin as _;
//! use rust_isa::loongarch::simd::lsx::vmin_b as _;
//! use rust_isa::loongarch::simd::lsx::vmin_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmin_d as _;
//! use rust_isa::loongarch::simd::lsx::vmin_du as _;
//! use rust_isa::loongarch::simd::lsx::vmin_h as _;
//! use rust_isa::loongarch::simd::lsx::vmin_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmin_w as _;
//! use rust_isa::loongarch::simd::lsx::vmin_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmini_b as _;
//! use rust_isa::loongarch::simd::lsx::vmini_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmini_d as _;
//! use rust_isa::loongarch::simd::lsx::vmini_du as _;
//! use rust_isa::loongarch::simd::lsx::vmini_h as _;
//! use rust_isa::loongarch::simd::lsx::vmini_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmini_w as _;
//! use rust_isa::loongarch::simd::lsx::vmini_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmod_b as _;
//! use rust_isa::loongarch::simd::lsx::vmod_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmod_d as _;
//! use rust_isa::loongarch::simd::lsx::vmod_du as _;
//! use rust_isa::loongarch::simd::lsx::vmod_h as _;
//! use rust_isa::loongarch::simd::lsx::vmod_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmod_w as _;
//! use rust_isa::loongarch::simd::lsx::vmod_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmskgez_b as _;
//! use rust_isa::loongarch::simd::lsx::vmskltz_b as _;
//! use rust_isa::loongarch::simd::lsx::vmskltz_d as _;
//! use rust_isa::loongarch::simd::lsx::vmskltz_h as _;
//! use rust_isa::loongarch::simd::lsx::vmskltz_w as _;
//! use rust_isa::loongarch::simd::lsx::vmsknz_b as _;
//! use rust_isa::loongarch::simd::lsx::vmsub_b as _;
//! use rust_isa::loongarch::simd::lsx::vmsub_d as _;
//! use rust_isa::loongarch::simd::lsx::vmsub_h as _;
//! use rust_isa::loongarch::simd::lsx::vmsub_w as _;
//! use rust_isa::loongarch::simd::lsx::vmuh_b as _;
//! use rust_isa::loongarch::simd::lsx::vmuh_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmuh_d as _;
//! use rust_isa::loongarch::simd::lsx::vmuh_du as _;
//! use rust_isa::loongarch::simd::lsx::vmuh_h as _;
//! use rust_isa::loongarch::simd::lsx::vmuh_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmuh_w as _;
//! use rust_isa::loongarch::simd::lsx::vmuh_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmul as _;
//! use rust_isa::loongarch::simd::lsx::vmul_b as _;
//! use rust_isa::loongarch::simd::lsx::vmul_d as _;
//! use rust_isa::loongarch::simd::lsx::vmul_h as _;
//! use rust_isa::loongarch::simd::lsx::vmul_w as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_d_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_d_wu_w as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_h_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_h_bu_b as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_q_du as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_q_du_d as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_w_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmulwev_w_hu_h as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_d_wu as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_d_wu_w as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_h_bu as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_h_bu_b as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_q_du as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_q_du_d as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_w_hu as _;
//! use rust_isa::loongarch::simd::lsx::vmulwod_w_hu_h as _;
//! use rust_isa::loongarch::simd::lsx::vneg as _;
//! use rust_isa::loongarch::simd::lsx::vneg_b as _;
//! use rust_isa::loongarch::simd::lsx::vneg_d as _;
//! use rust_isa::loongarch::simd::lsx::vneg_h as _;
//! use rust_isa::loongarch::simd::lsx::vneg_w as _;
//! use rust_isa::loongarch::simd::lsx::vnor as _;
//! use rust_isa::loongarch::simd::lsx::vnor_v as _;
//! use rust_isa::loongarch::simd::lsx::vnori_b as _;
//! use rust_isa::loongarch::simd::lsx::vor as _;
//! use rust_isa::loongarch::simd::lsx::vor_v as _;
//! use rust_isa::loongarch::simd::lsx::vori_b as _;
//! use rust_isa::loongarch::simd::lsx::vorn_v as _;
//! use rust_isa::loongarch::simd::lsx::vpackev as _;
//! use rust_isa::loongarch::simd::lsx::vpackev_b as _;
//! use rust_isa::loongarch::simd::lsx::vpackev_d as _;
//! use rust_isa::loongarch::simd::lsx::vpackev_h as _;
//! use rust_isa::loongarch::simd::lsx::vpackev_w as _;
//! use rust_isa::loongarch::simd::lsx::vpackod as _;
//! use rust_isa::loongarch::simd::lsx::vpackod_b as _;
//! use rust_isa::loongarch::simd::lsx::vpackod_d as _;
//! use rust_isa::loongarch::simd::lsx::vpackod_h as _;
//! use rust_isa::loongarch::simd::lsx::vpackod_w as _;
//! use rust_isa::loongarch::simd::lsx::vpcnt_b as _;
//! use rust_isa::loongarch::simd::lsx::vpcnt_d as _;
//! use rust_isa::loongarch::simd::lsx::vpcnt_h as _;
//! use rust_isa::loongarch::simd::lsx::vpcnt_w as _;
//! use rust_isa::loongarch::simd::lsx::vpermi_w as _;
//! use rust_isa::loongarch::simd::lsx::vpickev_b as _;
//! use rust_isa::loongarch::simd::lsx::vpickev_d as _;
//! use rust_isa::loongarch::simd::lsx::vpickev_h as _;
//! use rust_isa::loongarch::simd::lsx::vpickev_w as _;
//! use rust_isa::loongarch::simd::lsx::vpickod_b as _;
//! use rust_isa::loongarch::simd::lsx::vpickod_d as _;
//! use rust_isa::loongarch::simd::lsx::vpickod_h as _;
//! use rust_isa::loongarch::simd::lsx::vpickod_w as _;
//! use rust_isa::loongarch::simd::lsx::vpickve2gr_b as _;
//! use rust_isa::loongarch::simd::lsx::vpickve2gr_bu as _;
//! use rust_isa::loongarch::simd::lsx::vpickve2gr_d as _;
//! use rust_isa::loongarch::simd::lsx::vpickve2gr_du as _;
//! use rust_isa::loongarch::simd::lsx::vpickve2gr_h as _;
//! use rust_isa::loongarch::simd::lsx::vpickve2gr_hu as _;
//! use rust_isa::loongarch::simd::lsx::vpickve2gr_w as _;
//! use rust_isa::loongarch::simd::lsx::vpickve2gr_wu as _;
//! use rust_isa::loongarch::simd::lsx::vrepl_b as _;
//! use rust_isa::loongarch::simd::lsx::vrepl_d as _;
//! use rust_isa::loongarch::simd::lsx::vrepl_h as _;
//! use rust_isa::loongarch::simd::lsx::vrepl_w as _;
//! use rust_isa::loongarch::simd::lsx::vreplgr2vr_b as _;
//! use rust_isa::loongarch::simd::lsx::vreplgr2vr_d as _;
//! use rust_isa::loongarch::simd::lsx::vreplgr2vr_h as _;
//! use rust_isa::loongarch::simd::lsx::vreplgr2vr_w as _;
//! use rust_isa::loongarch::simd::lsx::vreplve_b as _;
//! use rust_isa::loongarch::simd::lsx::vreplve_d as _;
//! use rust_isa::loongarch::simd::lsx::vreplve_h as _;
//! use rust_isa::loongarch::simd::lsx::vreplve_w as _;
//! use rust_isa::loongarch::simd::lsx::vreplvei_b as _;
//! use rust_isa::loongarch::simd::lsx::vreplvei_d as _;
//! use rust_isa::loongarch::simd::lsx::vreplvei_h as _;
//! use rust_isa::loongarch::simd::lsx::vreplvei_w as _;
//! use rust_isa::loongarch::simd::lsx::vrotr_b as _;
//! use rust_isa::loongarch::simd::lsx::vrotr_d as _;
//! use rust_isa::loongarch::simd::lsx::vrotr_h as _;
//! use rust_isa::loongarch::simd::lsx::vrotr_w as _;
//! use rust_isa::loongarch::simd::lsx::vrotri_b as _;
//! use rust_isa::loongarch::simd::lsx::vrotri_d as _;
//! use rust_isa::loongarch::simd::lsx::vrotri_h as _;
//! use rust_isa::loongarch::simd::lsx::vrotri_w as _;
//! use rust_isa::loongarch::simd::lsx::vsadd_b as _;
//! use rust_isa::loongarch::simd::lsx::vsadd_bu as _;
//! use rust_isa::loongarch::simd::lsx::vsadd_d as _;
//! use rust_isa::loongarch::simd::lsx::vsadd_du as _;
//! use rust_isa::loongarch::simd::lsx::vsadd_h as _;
//! use rust_isa::loongarch::simd::lsx::vsadd_hu as _;
//! use rust_isa::loongarch::simd::lsx::vsadd_w as _;
//! use rust_isa::loongarch::simd::lsx::vsadd_wu as _;
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
//! use rust_isa::loongarch::simd::lsx::vseq_b as _;
//! use rust_isa::loongarch::simd::lsx::vseq_d as _;
//! use rust_isa::loongarch::simd::lsx::vseq_h as _;
//! use rust_isa::loongarch::simd::lsx::vseq_w as _;
//! use rust_isa::loongarch::simd::lsx::vseqi as _;
//! use rust_isa::loongarch::simd::lsx::vseqi_b as _;
//! use rust_isa::loongarch::simd::lsx::vseqi_d as _;
//! use rust_isa::loongarch::simd::lsx::vseqi_h as _;
//! use rust_isa::loongarch::simd::lsx::vseqi_w as _;
//! use rust_isa::loongarch::simd::lsx::vsetallnez_b as _;
//! use rust_isa::loongarch::simd::lsx::vsetallnez_d as _;
//! use rust_isa::loongarch::simd::lsx::vsetallnez_h as _;
//! use rust_isa::loongarch::simd::lsx::vsetallnez_w as _;
//! use rust_isa::loongarch::simd::lsx::vsetanyeqz_b as _;
//! use rust_isa::loongarch::simd::lsx::vsetanyeqz_d as _;
//! use rust_isa::loongarch::simd::lsx::vsetanyeqz_h as _;
//! use rust_isa::loongarch::simd::lsx::vsetanyeqz_w as _;
//! use rust_isa::loongarch::simd::lsx::vseteqz as _;
//! use rust_isa::loongarch::simd::lsx::vsetnez as _;
//! use rust_isa::loongarch::simd::lsx::vshuf as _;
//! use rust_isa::loongarch::simd::lsx::vshuf4i as _;
//! use rust_isa::loongarch::simd::lsx::vshuf4i_b as _;
//! use rust_isa::loongarch::simd::lsx::vshuf4i_d as _;
//! use rust_isa::loongarch::simd::lsx::vshuf4i_h as _;
//! use rust_isa::loongarch::simd::lsx::vshuf4i_w as _;
//! use rust_isa::loongarch::simd::lsx::vshuf_b as _;
//! use rust_isa::loongarch::simd::lsx::vshuf_d as _;
//! use rust_isa::loongarch::simd::lsx::vshuf_h as _;
//! use rust_isa::loongarch::simd::lsx::vshuf_w as _;
//! use rust_isa::loongarch::simd::lsx::vsigncov_b as _;
//! use rust_isa::loongarch::simd::lsx::vsigncov_d as _;
//! use rust_isa::loongarch::simd::lsx::vsigncov_h as _;
//! use rust_isa::loongarch::simd::lsx::vsigncov_w as _;
//! use rust_isa::loongarch::simd::lsx::vsle as _;
//! use rust_isa::loongarch::simd::lsx::vsle_b as _;
//! use rust_isa::loongarch::simd::lsx::vsle_bu as _;
//! use rust_isa::loongarch::simd::lsx::vsle_d as _;
//! use rust_isa::loongarch::simd::lsx::vsle_du as _;
//! use rust_isa::loongarch::simd::lsx::vsle_h as _;
//! use rust_isa::loongarch::simd::lsx::vsle_hu as _;
//! use rust_isa::loongarch::simd::lsx::vsle_w as _;
//! use rust_isa::loongarch::simd::lsx::vsle_wu as _;
//! use rust_isa::loongarch::simd::lsx::vslei as _;
//! use rust_isa::loongarch::simd::lsx::vslei_b as _;
//! use rust_isa::loongarch::simd::lsx::vslei_bu as _;
//! use rust_isa::loongarch::simd::lsx::vslei_d as _;
//! use rust_isa::loongarch::simd::lsx::vslei_du as _;
//! use rust_isa::loongarch::simd::lsx::vslei_h as _;
//! use rust_isa::loongarch::simd::lsx::vslei_hu as _;
//! use rust_isa::loongarch::simd::lsx::vslei_w as _;
//! use rust_isa::loongarch::simd::lsx::vslei_wu as _;
//! use rust_isa::loongarch::simd::lsx::vsll as _;
//! use rust_isa::loongarch::simd::lsx::vsll_b as _;
//! use rust_isa::loongarch::simd::lsx::vsll_d as _;
//! use rust_isa::loongarch::simd::lsx::vsll_h as _;
//! use rust_isa::loongarch::simd::lsx::vsll_w as _;
//! use rust_isa::loongarch::simd::lsx::vslli as _;
//! use rust_isa::loongarch::simd::lsx::vslli_b as _;
//! use rust_isa::loongarch::simd::lsx::vslli_d as _;
//! use rust_isa::loongarch::simd::lsx::vslli_h as _;
//! use rust_isa::loongarch::simd::lsx::vslli_w as _;
//! use rust_isa::loongarch::simd::lsx::vsllwil_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vsllwil_du_wu as _;
//! use rust_isa::loongarch::simd::lsx::vsllwil_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vsllwil_hu_bu as _;
//! use rust_isa::loongarch::simd::lsx::vsllwil_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vsllwil_wu_hu as _;
//! use rust_isa::loongarch::simd::lsx::vslt as _;
//! use rust_isa::loongarch::simd::lsx::vslt_b as _;
//! use rust_isa::loongarch::simd::lsx::vslt_bu as _;
//! use rust_isa::loongarch::simd::lsx::vslt_d as _;
//! use rust_isa::loongarch::simd::lsx::vslt_du as _;
//! use rust_isa::loongarch::simd::lsx::vslt_h as _;
//! use rust_isa::loongarch::simd::lsx::vslt_hu as _;
//! use rust_isa::loongarch::simd::lsx::vslt_w as _;
//! use rust_isa::loongarch::simd::lsx::vslt_wu as _;
//! use rust_isa::loongarch::simd::lsx::vslti as _;
//! use rust_isa::loongarch::simd::lsx::vslti_b as _;
//! use rust_isa::loongarch::simd::lsx::vslti_bu as _;
//! use rust_isa::loongarch::simd::lsx::vslti_d as _;
//! use rust_isa::loongarch::simd::lsx::vslti_du as _;
//! use rust_isa::loongarch::simd::lsx::vslti_h as _;
//! use rust_isa::loongarch::simd::lsx::vslti_hu as _;
//! use rust_isa::loongarch::simd::lsx::vslti_w as _;
//! use rust_isa::loongarch::simd::lsx::vslti_wu as _;
//! use rust_isa::loongarch::simd::lsx::vsra as _;
//! use rust_isa::loongarch::simd::lsx::vsra_b as _;
//! use rust_isa::loongarch::simd::lsx::vsra_d as _;
//! use rust_isa::loongarch::simd::lsx::vsra_h as _;
//! use rust_isa::loongarch::simd::lsx::vsra_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrai as _;
//! use rust_isa::loongarch::simd::lsx::vsrai_b as _;
//! use rust_isa::loongarch::simd::lsx::vsrai_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrai_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrai_w as _;
//! use rust_isa::loongarch::simd::lsx::vsran_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vsran_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vsran_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrani_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrani_d_q as _;
//! use rust_isa::loongarch::simd::lsx::vsrani_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrani_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrar_b as _;
//! use rust_isa::loongarch::simd::lsx::vsrar_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrar_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrar_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrari_b as _;
//! use rust_isa::loongarch::simd::lsx::vsrari_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrari_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrari_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrarn_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrarn_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrarn_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrarni_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrarni_d_q as _;
//! use rust_isa::loongarch::simd::lsx::vsrarni_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrarni_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrl as _;
//! use rust_isa::loongarch::simd::lsx::vsrl_b as _;
//! use rust_isa::loongarch::simd::lsx::vsrl_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrl_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrl_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrli as _;
//! use rust_isa::loongarch::simd::lsx::vsrli_b as _;
//! use rust_isa::loongarch::simd::lsx::vsrli_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrli_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrli_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrln_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrln_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrln_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrlni_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrlni_d_q as _;
//! use rust_isa::loongarch::simd::lsx::vsrlni_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrlni_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrlr_b as _;
//! use rust_isa::loongarch::simd::lsx::vsrlr_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrlr_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrlr_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrlri_b as _;
//! use rust_isa::loongarch::simd::lsx::vsrlri_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrlri_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrlri_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrlrn_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrlrn_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrlrn_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vsrlrni_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vsrlrni_d_q as _;
//! use rust_isa::loongarch::simd::lsx::vsrlrni_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vsrlrni_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssr as _;
//! use rust_isa::loongarch::simd::lsx::vssran_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vssran_bu_h as _;
//! use rust_isa::loongarch::simd::lsx::vssran_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vssran_hu_w as _;
//! use rust_isa::loongarch::simd::lsx::vssran_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssran_wu_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrani_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrani_bu_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrani_d_q as _;
//! use rust_isa::loongarch::simd::lsx::vssrani_du_q as _;
//! use rust_isa::loongarch::simd::lsx::vssrani_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrani_hu_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrani_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrani_wu_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_bu_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_hu_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_wu_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrarn_wu_du as _;
//! use rust_isa::loongarch::simd::lsx::vssrarni_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrarni_bu_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrarni_d_q as _;
//! use rust_isa::loongarch::simd::lsx::vssrarni_du_q as _;
//! use rust_isa::loongarch::simd::lsx::vssrarni_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrarni_hu_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrarni_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrarni_wu_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_bu_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_hu_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_wu_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrln_wu_du as _;
//! use rust_isa::loongarch::simd::lsx::vssrlni_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrlni_bu_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrlni_d_q as _;
//! use rust_isa::loongarch::simd::lsx::vssrlni_du_q as _;
//! use rust_isa::loongarch::simd::lsx::vssrlni_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrlni_hu_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrlni_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrlni_wu_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_bu_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_hu_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_wu_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrn_wu_du as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrni_b_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrni_bu_h as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrni_d_q as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrni_du_q as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrni_h_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrni_hu_w as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrni_w_d as _;
//! use rust_isa::loongarch::simd::lsx::vssrlrni_wu_d as _;
//! use rust_isa::loongarch::simd::lsx::vssub_b as _;
//! use rust_isa::loongarch::simd::lsx::vssub_bu as _;
//! use rust_isa::loongarch::simd::lsx::vssub_d as _;
//! use rust_isa::loongarch::simd::lsx::vssub_du as _;
//! use rust_isa::loongarch::simd::lsx::vssub_h as _;
//! use rust_isa::loongarch::simd::lsx::vssub_hu as _;
//! use rust_isa::loongarch::simd::lsx::vssub_w as _;
//! use rust_isa::loongarch::simd::lsx::vssub_wu as _;
//! use rust_isa::loongarch::simd::lsx::vst as _;
//! use rust_isa::loongarch::simd::lsx::vstelm_b as _;
//! use rust_isa::loongarch::simd::lsx::vstelm_d as _;
//! use rust_isa::loongarch::simd::lsx::vstelm_h as _;
//! use rust_isa::loongarch::simd::lsx::vstelm_w as _;
//! use rust_isa::loongarch::simd::lsx::vstx as _;
//! use rust_isa::loongarch::simd::lsx::vsub as _;
//! use rust_isa::loongarch::simd::lsx::vsub_b as _;
//! use rust_isa::loongarch::simd::lsx::vsub_d as _;
//! use rust_isa::loongarch::simd::lsx::vsub_h as _;
//! use rust_isa::loongarch::simd::lsx::vsub_q as _;
//! use rust_isa::loongarch::simd::lsx::vsub_w as _;
//! use rust_isa::loongarch::simd::lsx::vsubi_bu as _;
//! use rust_isa::loongarch::simd::lsx::vsubi_du as _;
//! use rust_isa::loongarch::simd::lsx::vsubi_hu as _;
//! use rust_isa::loongarch::simd::lsx::vsubi_wu as _;
//! use rust_isa::loongarch::simd::lsx::vsubwev_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vsubwev_d_wu as _;
//! use rust_isa::loongarch::simd::lsx::vsubwev_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vsubwev_h_bu as _;
//! use rust_isa::loongarch::simd::lsx::vsubwev_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vsubwev_q_du as _;
//! use rust_isa::loongarch::simd::lsx::vsubwev_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vsubwev_w_hu as _;
//! use rust_isa::loongarch::simd::lsx::vsubwod_d_w as _;
//! use rust_isa::loongarch::simd::lsx::vsubwod_d_wu as _;
//! use rust_isa::loongarch::simd::lsx::vsubwod_h_b as _;
//! use rust_isa::loongarch::simd::lsx::vsubwod_h_bu as _;
//! use rust_isa::loongarch::simd::lsx::vsubwod_q_d as _;
//! use rust_isa::loongarch::simd::lsx::vsubwod_q_du as _;
//! use rust_isa::loongarch::simd::lsx::vsubwod_w_h as _;
//! use rust_isa::loongarch::simd::lsx::vsubwod_w_hu as _;
//! use rust_isa::loongarch::simd::lsx::vxor as _;
//! use rust_isa::loongarch::simd::lsx::vxor_v as _;
//! use rust_isa::loongarch::simd::lsx::vxori_b as _;
//! use rust_isa::loongarch::simd::lsx::xor as _;
//! ```
//! <!-- doctest-inventory:end -->
//!
//! </details>

mod float_extra;
mod immediate;
mod lane;
mod misc;
mod narrow;
mod v128;

pub use float_extra::*;
pub use immediate::*;
pub use lane::*;
pub use misc::*;
pub use narrow::*;
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

unsafe extern "C" {
    fn rust_isa_lsx_vfmadd_s(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `vfmadd.s`
#[inline(always)]
pub fn vfmadd_s(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfmadd_s(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfmadd_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `vfmadd.d`
#[inline(always)]
pub fn vfmadd_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfmadd_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfmsub_s(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `vfmsub.s`
#[inline(always)]
pub fn vfmsub_s(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfmsub_s(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfmsub_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `vfmsub.d`
#[inline(always)]
pub fn vfmsub_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfmsub_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfnmadd_s(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `vfnmadd.s`
#[inline(always)]
pub fn vfnmadd_s(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfnmadd_s(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfnmadd_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `vfnmadd.d`
#[inline(always)]
pub fn vfnmadd_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfnmadd_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfnmsub_s(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `vfnmsub.s`
#[inline(always)]
pub fn vfnmsub_s(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfnmsub_s(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vfnmsub_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
}
/// `vfnmsub.d`
#[inline(always)]
pub fn vfnmsub_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfnmsub_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

unsafe extern "C" {
    fn rust_isa_lsx_vbitsel_v(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vshuf_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vldx(p: *const u8, offset: i64, out: *mut u8);
    fn rust_isa_lsx_vstx(value: *const u8, p: *mut u8, offset: i64);
    fn rust_isa_lsx_vseq_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vseq_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vseq_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vseq_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsle_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsle_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsle_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsle_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsle_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsle_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsle_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsle_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vslt_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vslt_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vslt_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vslt_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vslt_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vslt_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vslt_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vslt_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwev_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwev_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwev_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwev_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwod_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwod_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwod_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwod_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwev_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwev_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwev_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwev_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwod_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwod_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwod_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsubwod_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_h_bu_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_w_hu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_d_wu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwev_q_du_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_h_bu_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_w_hu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_d_wu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vaddwod_q_du_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsadd_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsadd_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsadd_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsadd_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssub_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssub_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssub_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssub_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsadd_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsadd_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsadd_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsadd_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssub_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssub_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssub_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssub_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhaddw_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhaddw_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhaddw_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhaddw_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhsubw_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhsubw_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhsubw_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhsubw_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhaddw_hu_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhaddw_wu_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhaddw_du_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhaddw_qu_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhsubw_hu_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhsubw_wu_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhsubw_du_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vhsubw_qu_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vadda_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vadda_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vadda_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vadda_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vabsd_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vabsd_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vabsd_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vabsd_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vabsd_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vabsd_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vabsd_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vabsd_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavg_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavg_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavg_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavg_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavg_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavg_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavg_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vavg_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmax_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmax_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmax_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmax_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmin_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmin_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmin_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmin_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmax_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmax_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmax_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmax_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmin_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmin_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmin_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmin_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmul_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmul_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmuh_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmuh_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmuh_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmuh_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmuh_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmuh_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmuh_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmuh_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_h_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_w_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_d_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_q_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_h_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_w_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_d_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_q_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_h_bu_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_w_hu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_d_wu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwev_q_du_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_h_bu_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_w_hu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_d_wu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmulwod_q_du_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmadd_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmadd_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmadd_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmadd_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmsub_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmsub_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmsub_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmsub_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_h_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_w_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_d_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_q_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_h_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_w_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_d_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_q_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_h_bu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_w_hu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_d_wu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_q_du(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_h_bu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_w_hu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_d_wu(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_q_du(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_h_bu_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_w_hu_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_d_wu_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwev_q_du_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_h_bu_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_w_hu_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_d_wu_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmaddwod_q_du_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vdiv_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vdiv_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vdiv_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vdiv_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmod_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmod_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmod_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmod_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vdiv_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vdiv_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vdiv_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vdiv_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmod_bu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmod_hu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmod_wu(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmod_du(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsll_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsll_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsll_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsll_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrl_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrl_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrl_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrl_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsra_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsra_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsra_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsra_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vrotr_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vrotr_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vrotr_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vrotr_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrlr_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrlr_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrlr_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrlr_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrar_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrar_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrar_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrar_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrln_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrln_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrln_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsran_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsran_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsran_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrlrn_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrlrn_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrlrn_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrarn_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrarn_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsrarn_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrln_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrln_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssran_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssran_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssran_w_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrlrn_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrlrn_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrarn_b_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrarn_h_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrln_bu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrln_hu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrln_wu_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssran_bu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssran_hu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssran_wu_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrlrn_bu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrlrn_hu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrlrn_wu_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrarn_bu_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrarn_hu_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vssrarn_wu_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitclr_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitclr_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitclr_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitclr_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitset_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitset_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitset_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitset_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitrev_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitrev_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitrev_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vbitrev_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpackev_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpackev_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpackev_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpackev_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpackod_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpackod_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpackod_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpackod_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vilvl_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vilvl_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vilvl_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vilvl_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vilvh_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vilvh_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vilvh_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vilvh_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpickev_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpickev_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpickev_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpickev_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpickod_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpickod_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpickod_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpickod_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vand_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vor_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vxor_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vnor_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vandn_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vorn_v(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstp_b(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfrstp_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vadd_q(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsub_q(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsigncov_b(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsigncov_h(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsigncov_w(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vsigncov_d(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfcvt_h_s(a: *const u8, b: *const u8, out: *mut u8);
    fn rust_isa_lsx_vshuf_h(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vshuf_w(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vshuf_d(a: *const u8, b: *const u8, c: *const u8, out: *mut u8);
    fn rust_isa_lsx_vclo_b(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vclo_h(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vclo_w(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vclo_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vclz_b(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vclz_h(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vclz_w(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vclz_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpcnt_b(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpcnt_h(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpcnt_w(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vpcnt_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmskltz_b(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmskltz_h(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmskltz_w(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmskltz_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmskgez_b(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vmsknz_b(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfclass_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vfclass_d(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintl_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftinth_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrml_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrmh_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrpl_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrph_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrzl_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrzh_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrnel_l_s(a: *const u8, out: *mut u8);
    fn rust_isa_lsx_vftintrneh_l_s(a: *const u8, out: *mut u8);
}

/// `vbitsel.v`
#[inline(always)]
pub fn vbitsel_v(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitsel_v(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vshuf.b`
#[inline(always)]
pub fn vshuf_b(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vshuf_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vldx`
///
/// Loads 16 bytes from `p + offset`.
///
/// # Safety
///
/// The caller must ensure the range is valid for reading.
#[inline(always)]
pub unsafe fn vldx(p: *const u8, offset: i64) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vldx(p, offset, out.as_mut_ptr());
    }
    out
}

/// `vstx`
///
/// Stores 16 bytes to `p + offset`.
///
/// # Safety
///
/// The caller must ensure the range is valid for writing.
#[inline(always)]
pub unsafe fn vstx(value: V128, p: *mut u8, offset: i64) {
    unsafe {
        rust_isa_lsx_vstx(value.as_ptr(), p, offset);
    }
}

/// `vseq.b`
#[inline(always)]
pub fn vseq_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vseq_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vseq.h`
#[inline(always)]
pub fn vseq_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vseq_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vseq.w`
#[inline(always)]
pub fn vseq_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vseq_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vseq.d`
#[inline(always)]
pub fn vseq_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vseq_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsle.b`
#[inline(always)]
pub fn vsle_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsle_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsle.h`
#[inline(always)]
pub fn vsle_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsle_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsle.w`
#[inline(always)]
pub fn vsle_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsle_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsle.d`
#[inline(always)]
pub fn vsle_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsle_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsle.bu`
#[inline(always)]
pub fn vsle_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsle_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsle.hu`
#[inline(always)]
pub fn vsle_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsle_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsle.wu`
#[inline(always)]
pub fn vsle_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsle_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsle.du`
#[inline(always)]
pub fn vsle_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsle_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vslt.b`
#[inline(always)]
pub fn vslt_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vslt_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vslt.h`
#[inline(always)]
pub fn vslt_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vslt_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vslt.w`
#[inline(always)]
pub fn vslt_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vslt_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vslt.d`
#[inline(always)]
pub fn vslt_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vslt_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vslt.bu`
#[inline(always)]
pub fn vslt_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vslt_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vslt.hu`
#[inline(always)]
pub fn vslt_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vslt_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vslt.wu`
#[inline(always)]
pub fn vslt_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vslt_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vslt.du`
#[inline(always)]
pub fn vslt_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vslt_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.h.b`
#[inline(always)]
pub fn vaddwev_h_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.w.h`
#[inline(always)]
pub fn vaddwev_w_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.d.w`
#[inline(always)]
pub fn vaddwev_d_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.q.d`
#[inline(always)]
pub fn vaddwev_q_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwev.h.b`
#[inline(always)]
pub fn vsubwev_h_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwev_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwev.w.h`
#[inline(always)]
pub fn vsubwev_w_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwev_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwev.d.w`
#[inline(always)]
pub fn vsubwev_d_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwev_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwev.q.d`
#[inline(always)]
pub fn vsubwev_q_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwev_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.h.b`
#[inline(always)]
pub fn vaddwod_h_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.w.h`
#[inline(always)]
pub fn vaddwod_w_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.d.w`
#[inline(always)]
pub fn vaddwod_d_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.q.d`
#[inline(always)]
pub fn vaddwod_q_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwod.h.b`
#[inline(always)]
pub fn vsubwod_h_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwod_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwod.w.h`
#[inline(always)]
pub fn vsubwod_w_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwod_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwod.d.w`
#[inline(always)]
pub fn vsubwod_d_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwod_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwod.q.d`
#[inline(always)]
pub fn vsubwod_q_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwod_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.h.bu`
#[inline(always)]
pub fn vaddwev_h_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.w.hu`
#[inline(always)]
pub fn vaddwev_w_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.d.wu`
#[inline(always)]
pub fn vaddwev_d_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.q.du`
#[inline(always)]
pub fn vaddwev_q_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwev.h.bu`
#[inline(always)]
pub fn vsubwev_h_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwev_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwev.w.hu`
#[inline(always)]
pub fn vsubwev_w_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwev_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwev.d.wu`
#[inline(always)]
pub fn vsubwev_d_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwev_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwev.q.du`
#[inline(always)]
pub fn vsubwev_q_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwev_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.h.bu`
#[inline(always)]
pub fn vaddwod_h_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.w.hu`
#[inline(always)]
pub fn vaddwod_w_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.d.wu`
#[inline(always)]
pub fn vaddwod_d_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.q.du`
#[inline(always)]
pub fn vaddwod_q_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwod.h.bu`
#[inline(always)]
pub fn vsubwod_h_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwod_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwod.w.hu`
#[inline(always)]
pub fn vsubwod_w_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwod_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwod.d.wu`
#[inline(always)]
pub fn vsubwod_d_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwod_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsubwod.q.du`
#[inline(always)]
pub fn vsubwod_q_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsubwod_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.h.bu.b`
#[inline(always)]
pub fn vaddwev_h_bu_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_h_bu_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.w.hu.h`
#[inline(always)]
pub fn vaddwev_w_hu_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_w_hu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.d.wu.w`
#[inline(always)]
pub fn vaddwev_d_wu_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_d_wu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwev.q.du.d`
#[inline(always)]
pub fn vaddwev_q_du_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwev_q_du_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.h.bu.b`
#[inline(always)]
pub fn vaddwod_h_bu_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_h_bu_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.w.hu.h`
#[inline(always)]
pub fn vaddwod_w_hu_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_w_hu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.d.wu.w`
#[inline(always)]
pub fn vaddwod_d_wu_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_d_wu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vaddwod.q.du.d`
#[inline(always)]
pub fn vaddwod_q_du_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vaddwod_q_du_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsadd.b`
#[inline(always)]
pub fn vsadd_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsadd_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsadd.h`
#[inline(always)]
pub fn vsadd_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsadd_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsadd.w`
#[inline(always)]
pub fn vsadd_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsadd_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsadd.d`
#[inline(always)]
pub fn vsadd_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsadd_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssub.b`
#[inline(always)]
pub fn vssub_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssub_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssub.h`
#[inline(always)]
pub fn vssub_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssub_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssub.w`
#[inline(always)]
pub fn vssub_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssub_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssub.d`
#[inline(always)]
pub fn vssub_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssub_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsadd.bu`
#[inline(always)]
pub fn vsadd_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsadd_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsadd.hu`
#[inline(always)]
pub fn vsadd_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsadd_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsadd.wu`
#[inline(always)]
pub fn vsadd_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsadd_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsadd.du`
#[inline(always)]
pub fn vsadd_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsadd_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssub.bu`
#[inline(always)]
pub fn vssub_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssub_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssub.hu`
#[inline(always)]
pub fn vssub_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssub_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssub.wu`
#[inline(always)]
pub fn vssub_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssub_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssub.du`
#[inline(always)]
pub fn vssub_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssub_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhaddw.h.b`
#[inline(always)]
pub fn vhaddw_h_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhaddw_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhaddw.w.h`
#[inline(always)]
pub fn vhaddw_w_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhaddw_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhaddw.d.w`
#[inline(always)]
pub fn vhaddw_d_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhaddw_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhaddw.q.d`
#[inline(always)]
pub fn vhaddw_q_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhaddw_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhsubw.h.b`
#[inline(always)]
pub fn vhsubw_h_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhsubw_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhsubw.w.h`
#[inline(always)]
pub fn vhsubw_w_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhsubw_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhsubw.d.w`
#[inline(always)]
pub fn vhsubw_d_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhsubw_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhsubw.q.d`
#[inline(always)]
pub fn vhsubw_q_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhsubw_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhaddw.hu.bu`
#[inline(always)]
pub fn vhaddw_hu_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhaddw_hu_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhaddw.wu.hu`
#[inline(always)]
pub fn vhaddw_wu_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhaddw_wu_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhaddw.du.wu`
#[inline(always)]
pub fn vhaddw_du_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhaddw_du_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhaddw.qu.du`
#[inline(always)]
pub fn vhaddw_qu_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhaddw_qu_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhsubw.hu.bu`
#[inline(always)]
pub fn vhsubw_hu_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhsubw_hu_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhsubw.wu.hu`
#[inline(always)]
pub fn vhsubw_wu_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhsubw_wu_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhsubw.du.wu`
#[inline(always)]
pub fn vhsubw_du_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhsubw_du_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vhsubw.qu.du`
#[inline(always)]
pub fn vhsubw_qu_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vhsubw_qu_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vadda.b`
#[inline(always)]
pub fn vadda_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vadda_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vadda.h`
#[inline(always)]
pub fn vadda_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vadda_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vadda.w`
#[inline(always)]
pub fn vadda_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vadda_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vadda.d`
#[inline(always)]
pub fn vadda_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vadda_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsd.b`
#[inline(always)]
pub fn vabsd_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vabsd_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsd.h`
#[inline(always)]
pub fn vabsd_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vabsd_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsd.w`
#[inline(always)]
pub fn vabsd_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vabsd_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsd.d`
#[inline(always)]
pub fn vabsd_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vabsd_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsd.bu`
#[inline(always)]
pub fn vabsd_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vabsd_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsd.hu`
#[inline(always)]
pub fn vabsd_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vabsd_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsd.wu`
#[inline(always)]
pub fn vabsd_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vabsd_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vabsd.du`
#[inline(always)]
pub fn vabsd_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vabsd_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vavg.b`
#[inline(always)]
pub fn vavg_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vavg_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vavg.h`
#[inline(always)]
pub fn vavg_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vavg_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vavg.w`
#[inline(always)]
pub fn vavg_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vavg_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vavg.d`
#[inline(always)]
pub fn vavg_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vavg_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vavg.bu`
#[inline(always)]
pub fn vavg_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vavg_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vavg.hu`
#[inline(always)]
pub fn vavg_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vavg_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vavg.wu`
#[inline(always)]
pub fn vavg_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vavg_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vavg.du`
#[inline(always)]
pub fn vavg_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vavg_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmax.b`
#[inline(always)]
pub fn vmax_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmax_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmax.h`
#[inline(always)]
pub fn vmax_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmax_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmax.w`
#[inline(always)]
pub fn vmax_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmax_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmax.d`
#[inline(always)]
pub fn vmax_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmax_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmin.b`
#[inline(always)]
pub fn vmin_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmin_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmin.h`
#[inline(always)]
pub fn vmin_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmin_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmin.w`
#[inline(always)]
pub fn vmin_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmin_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmin.d`
#[inline(always)]
pub fn vmin_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmin_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmax.bu`
#[inline(always)]
pub fn vmax_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmax_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmax.hu`
#[inline(always)]
pub fn vmax_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmax_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmax.wu`
#[inline(always)]
pub fn vmax_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmax_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmax.du`
#[inline(always)]
pub fn vmax_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmax_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmin.bu`
#[inline(always)]
pub fn vmin_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmin_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmin.hu`
#[inline(always)]
pub fn vmin_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmin_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmin.wu`
#[inline(always)]
pub fn vmin_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmin_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmin.du`
#[inline(always)]
pub fn vmin_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmin_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.b`
#[inline(always)]
pub fn vmul_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmul_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmul.d`
#[inline(always)]
pub fn vmul_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmul_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuh.b`
#[inline(always)]
pub fn vmuh_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmuh_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuh.h`
#[inline(always)]
pub fn vmuh_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmuh_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuh.w`
#[inline(always)]
pub fn vmuh_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmuh_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuh.d`
#[inline(always)]
pub fn vmuh_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmuh_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuh.bu`
#[inline(always)]
pub fn vmuh_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmuh_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuh.hu`
#[inline(always)]
pub fn vmuh_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmuh_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuh.wu`
#[inline(always)]
pub fn vmuh_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmuh_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmuh.du`
#[inline(always)]
pub fn vmuh_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmuh_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.h.b`
#[inline(always)]
pub fn vmulwev_h_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.w.h`
#[inline(always)]
pub fn vmulwev_w_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.d.w`
#[inline(always)]
pub fn vmulwev_d_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.q.d`
#[inline(always)]
pub fn vmulwev_q_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.h.b`
#[inline(always)]
pub fn vmulwod_h_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_h_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.w.h`
#[inline(always)]
pub fn vmulwod_w_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_w_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.d.w`
#[inline(always)]
pub fn vmulwod_d_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_d_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.q.d`
#[inline(always)]
pub fn vmulwod_q_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_q_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.h.bu`
#[inline(always)]
pub fn vmulwev_h_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.w.hu`
#[inline(always)]
pub fn vmulwev_w_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.d.wu`
#[inline(always)]
pub fn vmulwev_d_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.q.du`
#[inline(always)]
pub fn vmulwev_q_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.h.bu`
#[inline(always)]
pub fn vmulwod_h_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_h_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.w.hu`
#[inline(always)]
pub fn vmulwod_w_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_w_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.d.wu`
#[inline(always)]
pub fn vmulwod_d_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_d_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.q.du`
#[inline(always)]
pub fn vmulwod_q_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_q_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.h.bu.b`
#[inline(always)]
pub fn vmulwev_h_bu_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_h_bu_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.w.hu.h`
#[inline(always)]
pub fn vmulwev_w_hu_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_w_hu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.d.wu.w`
#[inline(always)]
pub fn vmulwev_d_wu_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_d_wu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwev.q.du.d`
#[inline(always)]
pub fn vmulwev_q_du_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwev_q_du_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.h.bu.b`
#[inline(always)]
pub fn vmulwod_h_bu_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_h_bu_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.w.hu.h`
#[inline(always)]
pub fn vmulwod_w_hu_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_w_hu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.d.wu.w`
#[inline(always)]
pub fn vmulwod_d_wu_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_d_wu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmulwod.q.du.d`
#[inline(always)]
pub fn vmulwod_q_du_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmulwod_q_du_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmadd.b`
#[inline(always)]
pub fn vmadd_b(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmadd_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmadd.h`
#[inline(always)]
pub fn vmadd_h(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmadd_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmadd.w`
#[inline(always)]
pub fn vmadd_w(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmadd_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmadd.d`
#[inline(always)]
pub fn vmadd_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmadd_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmsub.b`
#[inline(always)]
pub fn vmsub_b(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmsub_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmsub.h`
#[inline(always)]
pub fn vmsub_h(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmsub_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmsub.w`
#[inline(always)]
pub fn vmsub_w(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmsub_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmsub.d`
#[inline(always)]
pub fn vmsub_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmsub_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.h.b`
#[inline(always)]
pub fn vmaddwev_h_b(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_h_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.w.h`
#[inline(always)]
pub fn vmaddwev_w_h(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_w_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.d.w`
#[inline(always)]
pub fn vmaddwev_d_w(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_d_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.q.d`
#[inline(always)]
pub fn vmaddwev_q_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_q_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.h.b`
#[inline(always)]
pub fn vmaddwod_h_b(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_h_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.w.h`
#[inline(always)]
pub fn vmaddwod_w_h(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_w_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.d.w`
#[inline(always)]
pub fn vmaddwod_d_w(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_d_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.q.d`
#[inline(always)]
pub fn vmaddwod_q_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_q_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.h.bu`
#[inline(always)]
pub fn vmaddwev_h_bu(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_h_bu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.w.hu`
#[inline(always)]
pub fn vmaddwev_w_hu(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_w_hu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.d.wu`
#[inline(always)]
pub fn vmaddwev_d_wu(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_d_wu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.q.du`
#[inline(always)]
pub fn vmaddwev_q_du(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_q_du(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.h.bu`
#[inline(always)]
pub fn vmaddwod_h_bu(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_h_bu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.w.hu`
#[inline(always)]
pub fn vmaddwod_w_hu(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_w_hu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.d.wu`
#[inline(always)]
pub fn vmaddwod_d_wu(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_d_wu(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.q.du`
#[inline(always)]
pub fn vmaddwod_q_du(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_q_du(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.h.bu.b`
#[inline(always)]
pub fn vmaddwev_h_bu_b(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_h_bu_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.w.hu.h`
#[inline(always)]
pub fn vmaddwev_w_hu_h(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_w_hu_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.d.wu.w`
#[inline(always)]
pub fn vmaddwev_d_wu_w(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_d_wu_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwev.q.du.d`
#[inline(always)]
pub fn vmaddwev_q_du_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwev_q_du_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.h.bu.b`
#[inline(always)]
pub fn vmaddwod_h_bu_b(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_h_bu_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.w.hu.h`
#[inline(always)]
pub fn vmaddwod_w_hu_h(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_w_hu_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.d.wu.w`
#[inline(always)]
pub fn vmaddwod_d_wu_w(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_d_wu_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmaddwod.q.du.d`
#[inline(always)]
pub fn vmaddwod_q_du_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmaddwod_q_du_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdiv.b`
#[inline(always)]
pub fn vdiv_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vdiv_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdiv.h`
#[inline(always)]
pub fn vdiv_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vdiv_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdiv.w`
#[inline(always)]
pub fn vdiv_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vdiv_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdiv.d`
#[inline(always)]
pub fn vdiv_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vdiv_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmod.b`
#[inline(always)]
pub fn vmod_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmod_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmod.h`
#[inline(always)]
pub fn vmod_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmod_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmod.w`
#[inline(always)]
pub fn vmod_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmod_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmod.d`
#[inline(always)]
pub fn vmod_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmod_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdiv.bu`
#[inline(always)]
pub fn vdiv_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vdiv_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdiv.hu`
#[inline(always)]
pub fn vdiv_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vdiv_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdiv.wu`
#[inline(always)]
pub fn vdiv_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vdiv_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vdiv.du`
#[inline(always)]
pub fn vdiv_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vdiv_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmod.bu`
#[inline(always)]
pub fn vmod_bu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmod_bu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmod.hu`
#[inline(always)]
pub fn vmod_hu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmod_hu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmod.wu`
#[inline(always)]
pub fn vmod_wu(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmod_wu(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmod.du`
#[inline(always)]
pub fn vmod_du(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmod_du(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsll.b`
#[inline(always)]
pub fn vsll_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsll_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsll.h`
#[inline(always)]
pub fn vsll_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsll_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsll.w`
#[inline(always)]
pub fn vsll_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsll_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsll.d`
#[inline(always)]
pub fn vsll_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsll_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrl.b`
#[inline(always)]
pub fn vsrl_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrl_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrl.h`
#[inline(always)]
pub fn vsrl_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrl_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrl.w`
#[inline(always)]
pub fn vsrl_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrl_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrl.d`
#[inline(always)]
pub fn vsrl_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrl_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsra.b`
#[inline(always)]
pub fn vsra_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsra_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsra.h`
#[inline(always)]
pub fn vsra_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsra_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsra.w`
#[inline(always)]
pub fn vsra_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsra_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsra.d`
#[inline(always)]
pub fn vsra_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsra_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrotr.b`
#[inline(always)]
pub fn vrotr_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vrotr_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrotr.h`
#[inline(always)]
pub fn vrotr_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vrotr_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrotr.w`
#[inline(always)]
pub fn vrotr_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vrotr_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vrotr.d`
#[inline(always)]
pub fn vrotr_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vrotr_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrlr.b`
#[inline(always)]
pub fn vsrlr_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrlr_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrlr.h`
#[inline(always)]
pub fn vsrlr_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrlr_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrlr.w`
#[inline(always)]
pub fn vsrlr_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrlr_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrlr.d`
#[inline(always)]
pub fn vsrlr_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrlr_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrar.b`
#[inline(always)]
pub fn vsrar_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrar_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrar.h`
#[inline(always)]
pub fn vsrar_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrar_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrar.w`
#[inline(always)]
pub fn vsrar_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrar_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrar.d`
#[inline(always)]
pub fn vsrar_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrar_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrln.b.h`
#[inline(always)]
pub fn vsrln_b_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrln_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrln.h.w`
#[inline(always)]
pub fn vsrln_h_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrln_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrln.w.d`
#[inline(always)]
pub fn vsrln_w_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrln_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsran.b.h`
#[inline(always)]
pub fn vsran_b_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsran_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsran.h.w`
#[inline(always)]
pub fn vsran_h_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsran_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsran.w.d`
#[inline(always)]
pub fn vsran_w_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsran_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrlrn.b.h`
#[inline(always)]
pub fn vsrlrn_b_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrlrn_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrlrn.h.w`
#[inline(always)]
pub fn vsrlrn_h_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrlrn_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrlrn.w.d`
#[inline(always)]
pub fn vsrlrn_w_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrlrn_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrarn.b.h`
#[inline(always)]
pub fn vsrarn_b_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrarn_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrarn.h.w`
#[inline(always)]
pub fn vsrarn_h_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrarn_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsrarn.w.d`
#[inline(always)]
pub fn vsrarn_w_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsrarn_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrln.b.h`
#[inline(always)]
pub fn vssrln_b_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrln_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrln.h.w`
#[inline(always)]
pub fn vssrln_h_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrln_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssran.b.h`
#[inline(always)]
pub fn vssran_b_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssran_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssran.h.w`
#[inline(always)]
pub fn vssran_h_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssran_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssran.w.d`
#[inline(always)]
pub fn vssran_w_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssran_w_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrlrn.b.h`
#[inline(always)]
pub fn vssrlrn_b_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrlrn_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrlrn.h.w`
#[inline(always)]
pub fn vssrlrn_h_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrlrn_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrarn.b.h`
#[inline(always)]
pub fn vssrarn_b_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrarn_b_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrarn.h.w`
#[inline(always)]
pub fn vssrarn_h_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrarn_h_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrln.bu.h`
#[inline(always)]
pub fn vssrln_bu_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrln_bu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrln.hu.w`
#[inline(always)]
pub fn vssrln_hu_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrln_hu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrln.wu.d`
#[inline(always)]
pub fn vssrln_wu_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrln_wu_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssran.bu.h`
#[inline(always)]
pub fn vssran_bu_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssran_bu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssran.hu.w`
#[inline(always)]
pub fn vssran_hu_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssran_hu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssran.wu.d`
#[inline(always)]
pub fn vssran_wu_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssran_wu_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrlrn.bu.h`
#[inline(always)]
pub fn vssrlrn_bu_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrlrn_bu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrlrn.hu.w`
#[inline(always)]
pub fn vssrlrn_hu_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrlrn_hu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrlrn.wu.d`
#[inline(always)]
pub fn vssrlrn_wu_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrlrn_wu_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrarn.bu.h`
#[inline(always)]
pub fn vssrarn_bu_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrarn_bu_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrarn.hu.w`
#[inline(always)]
pub fn vssrarn_hu_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrarn_hu_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vssrarn.wu.d`
#[inline(always)]
pub fn vssrarn_wu_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vssrarn_wu_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitclr.b`
#[inline(always)]
pub fn vbitclr_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitclr_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitclr.h`
#[inline(always)]
pub fn vbitclr_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitclr_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitclr.w`
#[inline(always)]
pub fn vbitclr_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitclr_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitclr.d`
#[inline(always)]
pub fn vbitclr_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitclr_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitset.b`
#[inline(always)]
pub fn vbitset_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitset_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitset.h`
#[inline(always)]
pub fn vbitset_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitset_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitset.w`
#[inline(always)]
pub fn vbitset_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitset_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitset.d`
#[inline(always)]
pub fn vbitset_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitset_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitrev.b`
#[inline(always)]
pub fn vbitrev_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitrev_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitrev.h`
#[inline(always)]
pub fn vbitrev_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitrev_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitrev.w`
#[inline(always)]
pub fn vbitrev_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitrev_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vbitrev.d`
#[inline(always)]
pub fn vbitrev_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vbitrev_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackev.b`
#[inline(always)]
pub fn vpackev_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpackev_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackev.h`
#[inline(always)]
pub fn vpackev_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpackev_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackev.w`
#[inline(always)]
pub fn vpackev_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpackev_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackev.d`
#[inline(always)]
pub fn vpackev_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpackev_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackod.b`
#[inline(always)]
pub fn vpackod_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpackod_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackod.h`
#[inline(always)]
pub fn vpackod_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpackod_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackod.w`
#[inline(always)]
pub fn vpackod_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpackod_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpackod.d`
#[inline(always)]
pub fn vpackod_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpackod_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vilvl.b`
#[inline(always)]
pub fn vilvl_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vilvl_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vilvl.h`
#[inline(always)]
pub fn vilvl_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vilvl_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vilvl.w`
#[inline(always)]
pub fn vilvl_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vilvl_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vilvl.d`
#[inline(always)]
pub fn vilvl_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vilvl_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vilvh.b`
#[inline(always)]
pub fn vilvh_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vilvh_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vilvh.h`
#[inline(always)]
pub fn vilvh_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vilvh_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vilvh.w`
#[inline(always)]
pub fn vilvh_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vilvh_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vilvh.d`
#[inline(always)]
pub fn vilvh_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vilvh_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpickev.b`
#[inline(always)]
pub fn vpickev_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpickev_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpickev.h`
#[inline(always)]
pub fn vpickev_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpickev_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpickev.w`
#[inline(always)]
pub fn vpickev_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpickev_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpickev.d`
#[inline(always)]
pub fn vpickev_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpickev_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpickod.b`
#[inline(always)]
pub fn vpickod_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpickod_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpickod.h`
#[inline(always)]
pub fn vpickod_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpickod_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpickod.w`
#[inline(always)]
pub fn vpickod_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpickod_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpickod.d`
#[inline(always)]
pub fn vpickod_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpickod_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vand.v`
#[inline(always)]
pub fn vand_v(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vand_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vor.v`
#[inline(always)]
pub fn vor_v(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vor_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vxor.v`
#[inline(always)]
pub fn vxor_v(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vxor_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vnor.v`
#[inline(always)]
pub fn vnor_v(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vnor_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vandn.v`
#[inline(always)]
pub fn vandn_v(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vandn_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vorn.v`
#[inline(always)]
pub fn vorn_v(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vorn_v(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrstp.b`
#[inline(always)]
pub fn vfrstp_b(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrstp_b(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfrstp.h`
#[inline(always)]
pub fn vfrstp_h(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfrstp_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vadd.q`
#[inline(always)]
pub fn vadd_q(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vadd_q(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsub.q`
#[inline(always)]
pub fn vsub_q(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsub_q(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsigncov.b`
#[inline(always)]
pub fn vsigncov_b(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsigncov_b(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsigncov.h`
#[inline(always)]
pub fn vsigncov_h(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsigncov_h(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsigncov.w`
#[inline(always)]
pub fn vsigncov_w(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsigncov_w(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vsigncov.d`
#[inline(always)]
pub fn vsigncov_d(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vsigncov_d(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfcvt.h.s`
#[inline(always)]
pub fn vfcvt_h_s(a: V128, b: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfcvt_h_s(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vshuf.h`
#[inline(always)]
pub fn vshuf_h(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vshuf_h(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vshuf.w`
#[inline(always)]
pub fn vshuf_w(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vshuf_w(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vshuf.d`
#[inline(always)]
pub fn vshuf_d(a: V128, b: V128, c: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vshuf_d(a.as_ptr(), b.as_ptr(), c.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclo.b`
#[inline(always)]
pub fn vclo_b(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vclo_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclo.h`
#[inline(always)]
pub fn vclo_h(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vclo_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclo.w`
#[inline(always)]
pub fn vclo_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vclo_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclo.d`
#[inline(always)]
pub fn vclo_d(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vclo_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclz.b`
#[inline(always)]
pub fn vclz_b(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vclz_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclz.h`
#[inline(always)]
pub fn vclz_h(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vclz_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclz.w`
#[inline(always)]
pub fn vclz_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vclz_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vclz.d`
#[inline(always)]
pub fn vclz_d(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vclz_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpcnt.b`
#[inline(always)]
pub fn vpcnt_b(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpcnt_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpcnt.h`
#[inline(always)]
pub fn vpcnt_h(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpcnt_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpcnt.w`
#[inline(always)]
pub fn vpcnt_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpcnt_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vpcnt.d`
#[inline(always)]
pub fn vpcnt_d(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vpcnt_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmskltz.b`
#[inline(always)]
pub fn vmskltz_b(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmskltz_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmskltz.h`
#[inline(always)]
pub fn vmskltz_h(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmskltz_h(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmskltz.w`
#[inline(always)]
pub fn vmskltz_w(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmskltz_w(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmskltz.d`
#[inline(always)]
pub fn vmskltz_d(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmskltz_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmskgez.b`
#[inline(always)]
pub fn vmskgez_b(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmskgez_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vmsknz.b`
#[inline(always)]
pub fn vmsknz_b(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vmsknz_b(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfclass.s`
#[inline(always)]
pub fn vfclass_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfclass_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vfclass.d`
#[inline(always)]
pub fn vfclass_d(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vfclass_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintl.l.s`
#[inline(always)]
pub fn vftintl_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintl_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftinth.l.s`
#[inline(always)]
pub fn vftinth_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftinth_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintrml.l.s`
#[inline(always)]
pub fn vftintrml_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintrml_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintrmh.l.s`
#[inline(always)]
pub fn vftintrmh_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintrmh_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintrpl.l.s`
#[inline(always)]
pub fn vftintrpl_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintrpl_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintrph.l.s`
#[inline(always)]
pub fn vftintrph_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintrph_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintrzl.l.s`
#[inline(always)]
pub fn vftintrzl_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintrzl_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintrzh.l.s`
#[inline(always)]
pub fn vftintrzh_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintrzh_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintrnel.l.s`
#[inline(always)]
pub fn vftintrnel_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintrnel_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vftintrneh.l.s`
#[inline(always)]
pub fn vftintrneh_l_s(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vftintrneh_l_s(a.as_ptr(), out.as_mut_ptr());
    }
    out
}

/// `vreplgr2vr.b`
#[inline(always)]
pub fn vreplgr2vr_b(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vreplgr2vr_b(value, out.as_mut_ptr());
    }
    out
}

/// `vreplgr2vr.h`
#[inline(always)]
pub fn vreplgr2vr_h(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vreplgr2vr_h(value, out.as_mut_ptr());
    }
    out
}

/// `vreplgr2vr.w`
#[inline(always)]
pub fn vreplgr2vr_w(value: i32) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vreplgr2vr_w(value, out.as_mut_ptr());
    }
    out
}

/// `vreplgr2vr.d`
#[inline(always)]
pub fn vreplgr2vr_d(value: i64) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vreplgr2vr_d(value, out.as_mut_ptr());
    }
    out
}

/// `vextl.q.d`
#[inline(always)]
pub fn vextl_q_d(a: V128) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_lsx_vextl_q_d(a.as_ptr(), out.as_mut_ptr());
    }
    out
}
