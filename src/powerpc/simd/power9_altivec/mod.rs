//! PowerPC Power9 Altivec aliases.
//!
//! The Power9 vector instructions are exposed in [`power9_vector`](super::power9_vector). This
//! module provides the Altivec-named entry points for the same operations.
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).

pub use super::power9_vector::{
    vabsd, vabsduw, vctzb, vctzd, vctzh, vctzlsbb, vctzw, vextsb2d, vextsb2w, vextsh2d, vextsh2w,
    vextsw2d, vnegd, vnegw,
};
