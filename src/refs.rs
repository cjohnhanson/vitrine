//! Reference resolution: `tisket:<id>#<anchor>` → a relative path an
//! artifact can fetch with no resolver service.
//!
//! Resolution happens at authoring time; committed artifacts hold only
//! relative paths. Artifacts live at `.vitrine/<slug>/`, a fixed depth,
//! so every repo file is `../../<repo-relative-path>`.

use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct Resolved {
    /// Path relative to the artifact directory (`../../…`).
    pub relative: String,
    pub anchor: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RefError {
    BadScheme(String),
    NotFound(String),
    Ambiguous(String, Vec<String>),
}

impl std::fmt::Display for RefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadScheme(s) => write!(
                f,
                "unrecognized reference `{s}` (expected tisket:/zettel:/file:)"
            ),
            Self::NotFound(s) => write!(f, "no document found for `{s}`"),
            Self::Ambiguous(s, hits) => write!(
                f,
                "`{s}` matches more than one document: {}",
                hits.join(", ")
            ),
        }
    }
}

/// Resolve a scheme reference against a repo root.
pub fn resolve(repo: &Path, reference: &str) -> Result<Resolved, RefError> {
    let (target, anchor) = match reference.split_once('#') {
        Some((t, a)) => (t, Some(a.to_string())),
        None => (reference, None),
    };
    let (scheme, id) = target
        .split_once(':')
        .ok_or_else(|| RefError::BadScheme(reference.to_string()))?;

    let repo_relative = match scheme {
        "file" => {
            let p = repo.join(id);
            if p.is_file() {
                PathBuf::from(id)
            } else {
                return Err(RefError::NotFound(reference.to_string()));
            }
        }
        "tisket" => find_by_id(repo, ".tisket", id, reference)?,
        "zettel" => find_by_id(repo, ".zettel", id, reference)?,
        _ => return Err(RefError::BadScheme(reference.to_string())),
    };

    Ok(Resolved {
        relative: format!("../../{}", repo_relative.display()),
        anchor,
    })
}

/// Find `<id>.md` or `<id>-*.md` anywhere under `root/<store>/`.
fn find_by_id(repo: &Path, store: &str, id: &str, reference: &str) -> Result<PathBuf, RefError> {
    let mut hits = Vec::new();
    let base = repo.join(store);
    walk(&base, &mut |p| {
        let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
            return;
        };
        if (name == format!("{id}.md")
            || name.starts_with(&format!("{id}-")) && std::path::Path::new(name).extension().is_some_and(|e| e.eq_ignore_ascii_case("md")))
            && let Ok(rel) = p.strip_prefix(repo) {
                hits.push(rel.to_path_buf());
            }
    });
    hits.sort();
    match hits.len() {
        0 => Err(RefError::NotFound(reference.to_string())),
        1 => Ok(hits.remove(0)),
        _ => Err(RefError::Ambiguous(
            reference.to_string(),
            hits.iter().map(|p| p.display().to_string()).collect(),
        )),
    }
}

fn walk(dir: &Path, f: &mut impl FnMut(&Path)) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let p = entry.path();
        if p.is_dir() {
            walk(&p, f);
        } else {
            f(&p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("vitrine-refs-{tag}-{}", std::process::id()));
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(root.join(".tisket/v1/.closed")).unwrap();
        std::fs::create_dir_all(root.join(".zettel")).unwrap();
        std::fs::write(root.join(".tisket/v1/ab12-fix-the-thing.md"), "# T\n").unwrap();
        std::fs::write(root.join(".tisket/v1/.closed/cd34-old.md"), "# T\n").unwrap();
        std::fs::write(root.join(".zettel/note-one.md"), "# N\n").unwrap();
        std::fs::write(root.join("docs.md"), "# D\n").unwrap();
        root
    }

    #[test]
    fn resolves_tisket_by_id_prefix_including_closed() {
        let r = repo("tisket");
        assert_eq!(
            resolve(&r, "tisket:ab12#goal").unwrap(),
            Resolved {
                relative: "../../.tisket/v1/ab12-fix-the-thing.md".into(),
                anchor: Some("goal".into())
            }
        );
        assert!(resolve(&r, "tisket:cd34").is_ok());
    }

    #[test]
    fn resolves_zettel_and_file() {
        let r = repo("zf");
        assert_eq!(
            resolve(&r, "zettel:note-one").unwrap().relative,
            "../../.zettel/note-one.md"
        );
        assert_eq!(
            resolve(&r, "file:docs.md").unwrap().relative,
            "../../docs.md"
        );
    }

    #[test]
    fn missing_ambiguous_and_bad_scheme_error() {
        let r = repo("err");
        assert!(matches!(
            resolve(&r, "tisket:zz99"),
            Err(RefError::NotFound(_))
        ));
        std::fs::write(r.join(".tisket/v1/ab12-other-match.md"), "# T\n").unwrap();
        assert!(matches!(
            resolve(&r, "tisket:ab12"),
            Err(RefError::Ambiguous(..))
        ));
        assert!(matches!(
            resolve(&r, "gopher:x"),
            Err(RefError::BadScheme(_))
        ));
        assert!(matches!(
            resolve(&r, "no-scheme"),
            Err(RefError::BadScheme(_))
        ));
    }
}
