//! Markdown rendering with a fixed, deterministic profile.
//!
//! The profile is plain `CommonMark`. It uses no extension that changes
//! the structure of the output. The browser-side renderer is the
//! vendored commonmark.js, and it must make the same DOM. A test holds
//! the two renderers to that parity. This module computes a heading
//! anchor internally, for extraction. It never emits an anchor into the
//! HTML.

use comrak::{markdown_to_html, Options};

/// Render markdown to HTML under the fixed profile.
#[must_use]
pub fn render(markdown: &str) -> String {
    let options = Options::default();
    markdown_to_html(markdown, &options)
}

/// Strip a leading YAML front matter block (`---\n...\n---\n`).
/// commonmark.js does not know about front matter. Thus both renderers
/// strip the block before they render.
#[must_use]
pub fn strip_front_matter(markdown: &str) -> &str {
    let Some(rest) = markdown.strip_prefix("---\n") else {
        return markdown;
    };
    rest.find("\n---\n")
        .map_or(markdown, |end| &rest[end + "\n---\n".len()..])
}

/// Make the slug for a heading. The JS runtime shares this contract.
///
/// The slug is lowercase. It keeps the alphanumerics. It collapses each
/// run of other characters to one hyphen. It has no hyphen at the start
/// or at the end. Duplicate slugs get a `-1`, `-2`, … suffix. The
/// caller adds that suffix, in document order.
#[must_use]
pub fn slugify(heading: &str) -> String {
    let mut out = String::with_capacity(heading.len());
    let mut last_hyphen = true; // suppress leading hyphen
    for c in heading.trim().chars() {
        if c.is_alphanumeric() {
            for lc in c.to_lowercase() {
                out.push(lc);
            }
            last_hyphen = false;
        } else if !last_hyphen {
            out.push('-');
            last_hyphen = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_is_plain_commonmark() {
        assert_eq!(render("# Hi\n"), "<h1>Hi</h1>\n");
        assert_eq!(render("a `b` c\n"), "<p>a <code>b</code> c</p>\n");
    }

    #[test]
    fn front_matter_stripped_only_when_complete() {
        assert_eq!(strip_front_matter("---\na: 1\n---\nbody\n"), "body\n");
        assert_eq!(strip_front_matter("no front matter\n"), "no front matter\n");
        assert_eq!(
            strip_front_matter("---\nunterminated\n"),
            "---\nunterminated\n"
        );
    }

    #[test]
    fn slugs_collapse_and_trim() {
        assert_eq!(
            slugify("Design constraints (from review)"),
            "design-constraints-from-review"
        );
        assert_eq!(slugify("  What -- next?  "), "what-next");
        assert_eq!(slugify("Ünïcode Héadings"), "ünïcode-héadings");
        assert_eq!(slugify("---"), "");
    }
}
