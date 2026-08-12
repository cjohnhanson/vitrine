//! `vitrine new <slug>`: make `.vitrine/<slug>/` with the page template
//! and the runtime. The runtime is the vendored commonmark.js plus the
//! vitrine component code. The directory is self-contained.

use std::path::{Path, PathBuf};

const TEMPLATE: &str = include_str!("../assets/template.html");
const COMMONMARK_JS: &str = include_str!("../assets/commonmark.min.js");
const VITRINE_JS: &str = include_str!("../assets/vitrine.js");

#[derive(Debug, PartialEq, Eq)]
pub enum ScaffoldError {
    BadSlug(String),
    Exists(PathBuf),
    Io(String),
}

impl std::fmt::Display for ScaffoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadSlug(s) => write!(
                f,
                "the slug `{s}` must use lowercase alphanumerics and hyphens only"
            ),
            Self::Exists(p) => write!(f, "{} already exists", p.display()),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

#[must_use]
pub fn slug_ok(slug: &str) -> bool {
    !slug.is_empty()
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && slug
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Make the page heading from the slug: `q4-plan` gives `Q4 plan`.
fn title_from(slug: &str) -> String {
    let mut words = slug.split('-');
    let first = words.next().unwrap_or_default();
    let mut title: String = first
        .char_indices()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect();
    for w in words {
        title.push(' ');
        title.push_str(w);
    }
    title
}

pub fn new(repo: &Path, slug: &str) -> Result<PathBuf, ScaffoldError> {
    if !slug_ok(slug) {
        return Err(ScaffoldError::BadSlug(slug.to_string()));
    }
    let dir = repo.join(".vitrine").join(slug);
    if dir.exists() {
        return Err(ScaffoldError::Exists(dir));
    }
    let io = |e: std::io::Error| ScaffoldError::Io(e.to_string());
    std::fs::create_dir_all(&dir).map_err(io)?;
    std::fs::write(
        dir.join("index.html"),
        TEMPLATE.replace("__TITLE__", &title_from(slug)),
    )
    .map_err(io)?;
    let mut runtime = String::with_capacity(COMMONMARK_JS.len() + VITRINE_JS.len() + 1);
    runtime.push_str(COMMONMARK_JS);
    runtime.push('\n');
    runtime.push_str(VITRINE_JS);
    std::fs::write(dir.join("vitrine-runtime.js"), runtime).map_err(io)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffolds_and_refuses_repeats_and_bad_slugs() {
        let repo = std::env::temp_dir().join(format!("vitrine-scaffold-{}", std::process::id()));
        std::fs::remove_dir_all(&repo).ok();
        std::fs::create_dir_all(&repo).unwrap();

        let dir = new(&repo, "q4-plan").unwrap();
        let html = std::fs::read_to_string(dir.join("index.html")).unwrap();
        assert!(html.contains("<title>Q4 plan</title>"));
        assert!(dir.join("vitrine-runtime.js").exists());

        assert!(matches!(
            new(&repo, "q4-plan"),
            Err(ScaffoldError::Exists(_))
        ));
        for bad in ["", "Q4", "a/b", "-x", "x-", "a b"] {
            assert!(
                matches!(new(&repo, bad), Err(ScaffoldError::BadSlug(_))),
                "{bad}"
            );
        }
    }
}
