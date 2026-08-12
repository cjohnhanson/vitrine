//! vitrine — interactive artifacts for plaintext repos.
//!
//! Canonical content lives in markdown (tisket issues, zettel notes,
//! repo files). Artifacts are HTML pages in `.vitrine/<slug>/` that
//! *transclude* sections of that markdown by reference — written once,
//! rendered wherever — plus a response inbox for round-trip forms.
//! Portability ladder: baked light-DOM content renders from `file://`
//! with no JS; any static server gets live transclusion; `vitrine
//! serve` adds the inbox.

pub mod bake;
pub mod extract;
pub mod refs;
pub mod render;
pub mod scaffold;
pub mod serve;
