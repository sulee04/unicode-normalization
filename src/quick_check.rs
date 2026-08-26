// Copyright 2019 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The `QC` (QuickCheck) property value from `DerivedNormalizationProps.txt`
//! in the Unicode Character Database.
//!
//! This type only exists because `tables.rs` (generated, not hand-edited)
//! declares its `qc_*` property-lookup functions in terms of it. An
//! implementation of `is_nfc`/`is_nfd`/`is_nfkc`/`is_nfkd` is free to use
//! `crate::tables::qc_nfc` et al. as a fast-path optimization, or to ignore
//! them entirely and check by comparing against the `to_*` output instead.

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum IsNormalized {
    /// The character is allowed unchanged in this normalization form.
    Yes,
    /// The character is not allowed in this normalization form.
    No,
    /// The character may or may not be allowed; a full check is required.
    Maybe,
}
