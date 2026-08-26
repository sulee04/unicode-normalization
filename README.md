# unicode-normalization

Unicode Normalization Forms conformance workspace, implementing
[Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).

`src/lib.rs` declares the minimal public API — `to_nfd`, `to_nfkd`, `to_nfc`,
`to_nfkc`, `is_nfd`, `is_nfkd`, `is_nfc`, `is_nfkc` — with `todo!()` bodies to
be implemented. `tests/tests.rs` checks an implementation against the
official Unicode `NormalizationTest.txt` conformance suite
(`tests/data/normalization_tests.rs`).

```rust
use unicode_normalization::to_nfc;

fn main() {
    assert_eq!(to_nfc("A\u{30a}"), "\u{c5}");
}
```

Character property data (combining classes, decomposition/composition
tables, etc.), generated from the Unicode Character Database by
`scripts/unicode.py`, is available in `src/tables.rs` via the accessors in
`src/lookups.rs`.

```sh
cargo test
```
