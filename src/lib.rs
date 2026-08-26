// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode Normalization Forms conformance workspace.
//!
//! This crate defines the minimal public API for Unicode Normalization
//! Forms as described in
//! [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/):
//! Canonical/Compatibility Decomposition (NFD/NFKD), Canonical/Compatibility
//! Composition (NFC/NFKC), and the corresponding "is already normalized"
//! checks.
//!
//! ```rust
//! use unicode_normalization::to_nfc;
//!
//! assert_eq!(to_nfc("A\u{30a}"), "\u{c5}");
//! ```

#![deny(missing_docs, unsafe_code)]

mod lookups;
mod perfect_hash;
mod quick_check;
mod tables;

pub use crate::tables::UNICODE_VERSION;

/// Returns the Canonical Decomposition (NFD) of `s`.
pub fn to_nfd(s: &str) -> String {
    todo!()
}

/// Returns the Compatibility Decomposition (NFKD) of `s`.
pub fn to_nfkd(s: &str) -> String {
    todo!()
}

/// Returns the Canonical Decomposition followed by Canonical Composition
/// (NFC) of `s`.
pub fn to_nfc(s: &str) -> String {
    todo!()
}

/// Returns the Compatibility Decomposition followed by Canonical
/// Composition (NFKC) of `s`.
pub fn to_nfkc(s: &str) -> String {
    todo!()
}

/// Returns whether `s` is already in Canonical Decomposition (NFD).
pub fn is_nfd(s: &str) -> bool {
    todo!()
}

/// Returns whether `s` is already in Compatibility Decomposition (NFKD).
pub fn is_nfkd(s: &str) -> bool {
    todo!()
}

/// Returns whether `s` is already in Canonical Composition (NFC).
pub fn is_nfc(s: &str) -> bool {
    todo!()
}

/// Returns whether `s` is already in Compatibility Composition (NFKC).
pub fn is_nfkc(s: &str) -> bool {
    todo!()
}
