// Integration test for SigmaOS-arch-reflections-kernel
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_reflections_kernel as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
