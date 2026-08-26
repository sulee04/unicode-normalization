use std::fmt::Write as _;
use std::panic::{self, AssertUnwindSafe};

use unicode_normalization::{is_nfc, is_nfd, is_nfkc, is_nfkd, to_nfc, to_nfd, to_nfkc, to_nfkd};

mod data {
    pub mod normalization_tests;
}
use crate::data::normalization_tests::{NormalizationTest, NORMALIZATION_TESTS};

/// Checks one line of the official Unicode `NormalizationTest.txt` against
/// every invariant from its CONFORMANCE section
/// (<http://www.unicode.org/Public/UNIDATA/NormalizationTest.txt>), plus the
/// corresponding `is_*` "already normalized" checks. Returns a description
/// of every mismatch found (empty if the case passes).
fn check_case(test: &NormalizationTest) -> Vec<String> {
    let mut fails = Vec::new();

    macro_rules! expect {
        ($got:expr, $want:expr, $what:expr) => {{
            let got = $got;
            if got != $want {
                fails.push(format!("{}: got {:?}, want {:?}", $what, got, $want));
            }
        }};
    }

    // c2 ==  toNFC(c1) ==  toNFC(c2) ==  toNFC(c3)
    // c4 ==  toNFC(c4) ==  toNFC(c5)
    expect!(to_nfc(test.source), test.nfc, "to_nfc(source)");
    expect!(to_nfc(test.nfc), test.nfc, "to_nfc(nfc)");
    expect!(to_nfc(test.nfd), test.nfc, "to_nfc(nfd)");
    expect!(to_nfc(test.nfkc), test.nfkc, "to_nfc(nfkc)");
    expect!(to_nfc(test.nfkd), test.nfkc, "to_nfc(nfkd)");

    // c3 ==  toNFD(c1) ==  toNFD(c2) ==  toNFD(c3)
    // c5 ==  toNFD(c4) ==  toNFD(c5)
    expect!(to_nfd(test.source), test.nfd, "to_nfd(source)");
    expect!(to_nfd(test.nfc), test.nfd, "to_nfd(nfc)");
    expect!(to_nfd(test.nfd), test.nfd, "to_nfd(nfd)");
    expect!(to_nfd(test.nfkc), test.nfkd, "to_nfd(nfkc)");
    expect!(to_nfd(test.nfkd), test.nfkd, "to_nfd(nfkd)");

    // c4 == toNFKC(c1) == toNFKC(c2) == toNFKC(c3) == toNFKC(c4) == toNFKC(c5)
    expect!(to_nfkc(test.source), test.nfkc, "to_nfkc(source)");
    expect!(to_nfkc(test.nfc), test.nfkc, "to_nfkc(nfc)");
    expect!(to_nfkc(test.nfd), test.nfkc, "to_nfkc(nfd)");
    expect!(to_nfkc(test.nfkc), test.nfkc, "to_nfkc(nfkc)");
    expect!(to_nfkc(test.nfkd), test.nfkc, "to_nfkc(nfkd)");

    // c5 == toNFKD(c1) == toNFKD(c2) == toNFKD(c3) == toNFKD(c4) == toNFKD(c5)
    expect!(to_nfkd(test.source), test.nfkd, "to_nfkd(source)");
    expect!(to_nfkd(test.nfc), test.nfkd, "to_nfkd(nfc)");
    expect!(to_nfkd(test.nfd), test.nfkd, "to_nfkd(nfd)");
    expect!(to_nfkd(test.nfkc), test.nfkd, "to_nfkd(nfkc)");
    expect!(to_nfkd(test.nfkd), test.nfkd, "to_nfkd(nfkd)");

    macro_rules! expect_bool {
        ($got:expr, $want:expr, $what:expr) => {{
            let got = $got;
            if got != $want {
                fails.push(format!("{}: got {}, want {}", $what, got, $want));
            }
        }};
    }

    expect_bool!(is_nfc(test.nfc), true, "is_nfc(nfc)");
    expect_bool!(is_nfd(test.nfd), true, "is_nfd(nfd)");
    expect_bool!(is_nfkc(test.nfkc), true, "is_nfkc(nfkc)");
    expect_bool!(is_nfkd(test.nfkd), true, "is_nfkd(nfkd)");
    // NFKC/NFKD are also valid NFC/NFD respectively.
    expect_bool!(is_nfc(test.nfkc), true, "is_nfc(nfkc)");
    expect_bool!(is_nfd(test.nfkd), true, "is_nfd(nfkd)");
    if test.nfc != test.nfd {
        expect_bool!(is_nfc(test.nfd), false, "is_nfc(nfd)");
        expect_bool!(is_nfd(test.nfc), false, "is_nfd(nfc)");
    }
    if test.nfkc != test.nfc {
        expect_bool!(is_nfkc(test.nfc), false, "is_nfkc(nfc)");
    }
    if test.nfkd != test.nfd {
        expect_bool!(is_nfkd(test.nfd), false, "is_nfkd(nfd)");
    }

    fails
}

/// Runs the full official conformance suite. Each line of
/// `NormalizationTest.txt` is checked independently: a panic (e.g. from an
/// unimplemented `todo!()`) in one case is caught and recorded as a failure
/// rather than aborting the whole run, so a partial or incorrect
/// implementation still gets a complete, graceful report of exactly which
/// cases pass and fail instead of losing everything after the first crash.
#[test]
fn test_official_conformance() {
    // Suppress the default panic hook for the duration of the loop so an
    // unimplemented `todo!()` doesn't print a full backtrace for every one
    // of the (many thousands of) conformance cases; we print our own
    // summary below instead.
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    let mut failures: Vec<(usize, String)> = Vec::new();
    for (i, test) in NORMALIZATION_TESTS.iter().enumerate() {
        match panic::catch_unwind(AssertUnwindSafe(|| check_case(test))) {
            Ok(fails) if fails.is_empty() => {}
            Ok(fails) => failures.push((i, fails.join("; "))),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "panicked".to_string());
                failures.push((i, format!("panicked: {msg}")));
            }
        }
    }

    panic::set_hook(prev_hook);

    let total = NORMALIZATION_TESTS.len();
    let passed = total - failures.len();
    // Printed unconditionally (run with `cargo test -- --nocapture` to see
    // it even when every case passes; on failure it's included in the
    // panic message below regardless of capturing).
    eprintln!(
        "pass rate: {passed}/{total} ({:.2}%)",
        100.0 * passed as f64 / total as f64
    );

    if !failures.is_empty() {
        let mut msg = format!(
            "{}/{total} conformance cases failed ({passed}/{total} passed, {:.2}%):\n",
            failures.len(),
            100.0 * passed as f64 / total as f64
        );
        for (i, why) in failures.iter().take(20) {
            let test = &NORMALIZATION_TESTS[*i];
            writeln!(msg, "  [{i}] source={:?}: {why}", test.source).unwrap();
        }
        if failures.len() > 20 {
            writeln!(msg, "  ... and {} more", failures.len() - 20).unwrap();
        }
        panic!("{}", msg);
    }
}
