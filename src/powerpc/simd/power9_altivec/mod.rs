//! PowerPC Power9 Altivec aliases.
//!
//! The Power9 vector instructions are exposed in [`power9_vector`](super::power9_vector). This
//! module provides the Altivec-named entry points for the same operations.
//!
//! Power9 Altivec is not part of the PowerPC baseline. The re-exported
//! operations are `unsafe` and require the Power9 feature set. The values are
//! [`V128`](super::altivec::V128).
//!
//! Instruction behavior follows the [Power ISA](https://openpowerfoundation.org/specifications/isa/).
//!
//! This module re-exports existing wrappers and adds no build steps.
//!
//! Instructions not listed in this module are not exposed.

pub use super::power9_vector::{
    vabsd, vabsduw, vctzb, vctzd, vctzh, vctzlsbb, vctzw, vextsb2d, vextsb2w, vextsh2d, vextsh2w,
    vextsw2d, vnegd, vnegw,
};
