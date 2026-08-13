//! Baking: resolve each `<md-section>` element in an artifact, then
//! write the rendered section into the light DOM children.
//!
//! The baked copy is derived. Never edit it by hand. `vitrine sync`
//! makes it again, and the runtime replaces it with a live render when
//! a server is present. The content goes in the light DOM, not the
//! shadow DOM, so it gets the stylesheet of the artifact.
//!
//! `<md-section>` elements must not nest. This module applies the rule,
//! and the authoring guide documents it.

use std::path::Path;

use crate::extract::extract;
use crate::render::{render, strip_front_matter};

#[derive(Debug, PartialEq, Eq)]
pub enum BakeError {
    Nested,
    Unclosed,
    MissingRef(String),
    Unreadable(String),
    AnchorNotFound { file: String, anchor: String },
}

impl std::fmt::Display for BakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nested => write!(f, "<md-section> elements must not nest"),
            Self::Unclosed => write!(f, "unclosed <md-section> element"),
            Self::MissingRef(tag) => {
                write!(f, "the md-section element has no ref attribute: {tag}")
            }
            Self::Unreadable(p) => write!(f, "cannot read the referenced file {p}"),
            Self::AnchorNotFound { file, anchor } => {
                write!(f, "anchor `{anchor}` not found in {file}")
            }
        }
    }
}

/// Rewrite `html`. Fill each `<md-section ref="path[#anchor]">` element
/// with its rendered content. `artifact_dir` is the base directory for
/// a relative ref.
pub fn bake(html: &str, artifact_dir: &Path) -> Result<String, BakeError> {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(start) = rest.find("<md-section") {
        let (before, from_tag) = rest.split_at(start);
        out.push_str(before);

        let open_end = from_tag.find('>').ok_or(BakeError::Unclosed)?;
        let open_tag = &from_tag[..=open_end];
        let after_open = &from_tag[open_end + 1..];

        let close = after_open
            .find("</md-section>")
            .ok_or(BakeError::Unclosed)?;
        let inner = &after_open[..close];
        if inner.contains("<md-section") {
            return Err(BakeError::Nested);
        }
        let after_close = &after_open[close + "</md-section>".len()..];

        let reference =
            attr(open_tag, "ref").ok_or_else(|| BakeError::MissingRef(open_tag.to_string()))?;
        let (path, anchor) = match reference.split_once('#') {
            Some((p, a)) => (p, Some(a)),
            None => (reference.as_str(), None),
        };

        let file = artifact_dir.join(path);
        let markdown =
            std::fs::read_to_string(&file).map_err(|_| BakeError::Unreadable(path.to_string()))?;
        let span = match anchor {
            Some(a) => extract(&markdown, a).ok_or_else(|| BakeError::AnchorNotFound {
                file: path.to_string(),
                anchor: a.to_string(),
            })?,
            None => format!("{}\n", strip_front_matter(&markdown).trim_end()),
        };
        let rendered = render(&span);

        out.push_str(open_tag);
        out.push('\n');
        out.push_str(rendered.trim_end());
        out.push('\n');
        out.push_str("</md-section>");
        rest = after_close;
    }
    out.push_str(rest);
    Ok(out)
}

/// Get one `name="value"` attribute from an open tag. The attribute
/// name must sit at a boundary, so `ref` does not match `data-ref`.
fn attr(open_tag: &str, name: &str) -> Option<String> {
    let key = format!("{name}=\"");
    let mut from = 0;
    let key_at = loop {
        let rel = open_tag[from..].find(&key)?;
        let at = from + rel;
        // The char before the name must be whitespace, so `data-ref="`
        // does not satisfy a request for `ref`.
        let boundary = at == 0
            || open_tag[..at]
                .chars()
                .next_back()
                .is_some_and(char::is_whitespace);
        if boundary {
            break at;
        }
        from = at + key.len();
    };
    let start = key_at + key.len();
    let end = open_tag[start..].find('"')?;
    Some(open_tag[start..start + end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_ref_does_not_shadow_ref() {
        let d = dir("dataref");
        let html = r#"<md-section data-x="1" ref="doc.md#b" data-ref="wrong.md"></md-section>"#;
        let baked = bake(html, &d).unwrap();
        assert!(baked.contains("<h2>B</h2>"), "the real ref was baked: {baked}");
    }


    fn dir(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("vitrine-bake-{tag}-{}", std::process::id()));
        std::fs::remove_dir_all(&d).ok();
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(d.join("doc.md"), "# A\none\n\n## B\ntwo\n").unwrap();
        d
    }

    #[test]
    fn bake_fills_section_and_is_idempotent() {
        let d = dir("fill");
        let html = r#"<p>x</p><md-section ref="doc.md#b"></md-section><p>y</p>"#;
        let baked = bake(html, &d).unwrap();
        assert!(baked.contains("<h2>B</h2>"));
        assert!(baked.contains("<p>two</p>"));
        assert!(baked.starts_with("<p>x</p>"));
        assert!(baked.ends_with("<p>y</p>"));
        let again = bake(&baked, &d).unwrap();
        assert_eq!(baked, again, "a second sync replaces; it does not add");
    }

    #[test]
    fn whole_file_when_no_anchor() {
        let d = dir("whole");
        let baked = bake(r#"<md-section ref="doc.md"></md-section>"#, &d).unwrap();
        assert!(baked.contains("<h1>A</h1>"));
        assert!(baked.contains("<h2>B</h2>"));
    }

    #[test]
    fn errors_are_specific() {
        let d = dir("err");
        assert_eq!(
            bake(r#"<md-section ref="doc.md#zz"></md-section>"#, &d),
            Err(BakeError::AnchorNotFound {
                file: "doc.md".into(),
                anchor: "zz".into()
            })
        );
        assert_eq!(
            bake(r#"<md-section ref="gone.md"></md-section>"#, &d),
            Err(BakeError::Unreadable("gone.md".into()))
        );
        assert_eq!(
            bake("<md-section ref=\"doc.md\">", &d),
            Err(BakeError::Unclosed)
        );
        assert_eq!(
            bake(
                r#"<md-section ref="doc.md"><md-section ref="doc.md"></md-section></md-section>"#,
                &d
            ),
            Err(BakeError::Nested)
        );
        assert!(matches!(
            bake("<md-section></md-section>", &d),
            Err(BakeError::MissingRef(_))
        ));
    }
}
