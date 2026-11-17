// Integration test for SigmaOS-alpm-rust
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_alpm_rust as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
