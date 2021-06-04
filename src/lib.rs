//! Implementation of the Pallas / Vesta curve cycle.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unknown_lints)]
#![allow(clippy::op_ref, clippy::same_item_push, clippy::upper_case_acronyms)]
#![deny(broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[macro_use]
mod macros;
mod curves;
mod fields;

pub mod arithmetic;
pub mod pallas;
pub mod vesta;

#[cfg(feature = "alloc")]
mod hashtocurve;

pub use curves::*;
pub use fields::*;

#[cfg(feature = "alloc")]
#[test]
fn test_endo_consistency() {
    use crate::arithmetic::{CurveExt, FieldExt};
    use group::Group;

    let a = pallas::Point::generator();
    assert_eq!(a * pallas::Scalar::ZETA, a.endo());
    let a = vesta::Point::generator();
    assert_eq!(a * vesta::Scalar::ZETA, a.endo());
}
