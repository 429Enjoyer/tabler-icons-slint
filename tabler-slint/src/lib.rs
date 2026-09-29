//! Tabler Icons for Slint.
//!
//! The generated Slint library provides outline and filled icon sets.
//! See the repository README for usage and generation instructions.

/// Returns the path to the generated `tabler.slint` library.
///
/// Use this path with `slint_build::CompilerConfiguration::with_library_paths`.
pub fn lib() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tabler.slint").to_string()
}
