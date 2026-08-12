//! vitrine: interactive artifacts for plaintext repos.
//!
//! The canonical content is markdown: tisket issues, zettel notes, repo
//! files. An artifact is an HTML page in `.vitrine/<slug>/`. It
//! *transcludes* sections of that markdown by reference. You write the
//! content once, and each artifact renders it. An artifact can also
//! carry a round-trip form, and the response inbox records the answers.
//!
//! The portability ladder has three steps. Baked light DOM content
//! shows from `file://` with no JS. Any static server gives live
//! transclusion. `vitrine serve` adds the inbox.

pub mod bake;
pub mod extract;
pub mod refs;
pub mod render;
pub mod scaffold;
pub mod serve;
