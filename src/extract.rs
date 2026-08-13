//! Section extraction: a markdown text and an anchor give the markdown
//! span of one section.
//!
//! This module reads ATX headings only (`#` through `######`). It knows
//! about code fences, so a `#` line in a code block is never a heading.
//! A section starts at its heading line. It stops at the next heading
//! of the same level or a higher level. Duplicate heading slugs get a
//! `-1`, `-2`, … suffix in document order. The JS runtime implements
//! the same contract.

use crate::render::{slugify, strip_front_matter};

struct Heading {
    line_idx: usize,
    level: usize,
    slug: String,
}

fn headings(lines: &[&str]) -> Vec<Heading> {
    let mut out = Vec::new();
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut in_fence: Option<String> = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if let Some(fence) = &in_fence {
            if trimmed.starts_with(fence.as_str()) {
                in_fence = None;
            }
            continue;
        }
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = Some(trimmed[..3].to_string());
            continue;
        }
        // Four or more leading spaces make an indented code block, so a
        // `#` there is code, not a heading.
        let indent = line.len() - trimmed.len();
        if indent >= 4 {
            continue;
        }
        let hashes = trimmed.bytes().take_while(|b| *b == b'#').count();
        if hashes == 0 || hashes > 6 {
            continue;
        }
        let rest = &trimmed[hashes..];
        if !rest.is_empty() && !rest.starts_with(' ') {
            continue; // a hashtag, not a heading
        }
        let title = rest.trim().trim_end_matches(['#', ' ']);
        let slug = unique_slug(&slugify(title), &mut used);
        out.push(Heading {
            line_idx: i,
            level: hashes,
            slug,
        });
    }
    out
}

/// A slug not yet used. A duplicate gets `-1`, `-2`, … and the suffix
/// keeps climbing past any slug already taken, so `Alpha` / `Alpha 1` /
/// `Alpha` never collide on `alpha-1`.
fn unique_slug(base: &str, used: &mut std::collections::HashSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }
    let mut n = 1;
    loop {
        let candidate = format!("{base}-{n}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        n += 1;
    }
}

/// Extract the section that `anchor` addresses. The result is the
/// markdown span. It includes the heading line, and it has no trailing
/// blank lines.
#[must_use]
pub fn extract(markdown: &str, anchor: &str) -> Option<String> {
    let body = strip_front_matter(markdown);
    let lines: Vec<&str> = body.lines().collect();
    let heads = headings(&lines);
    let pos = heads.iter().position(|h| h.slug == anchor)?;
    let start = heads[pos].line_idx;
    let level = heads[pos].level;
    let end = heads[pos + 1..]
        .iter()
        .find(|h| h.level <= level)
        .map_or(lines.len(), |h| h.line_idx);
    let section = lines[start..end].join("\n");
    Some(format!("{}\n", section.trim_end()))
}

/// List each anchor in the document, in order. `vitrine extract <file>`
/// prints this list.
#[must_use]
pub fn anchors(markdown: &str) -> Vec<String> {
    let body = strip_front_matter(markdown);
    let lines: Vec<&str> = body.lines().collect();
    headings(&lines).into_iter().map(|h| h.slug).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indented_hash_is_code_not_a_heading() {
        let doc = "# Real
body

    # indented four spaces is code

## Next
x
";
        assert_eq!(anchors(doc), vec!["real", "next"]);
    }

    #[test]
    fn duplicate_slug_does_not_collide_with_explicit() {
        // Alpha -> alpha, "Alpha 1" -> alpha-1, Alpha -> must be alpha-2,
        // not a second alpha-1.
        let doc = "# Alpha
a

# Alpha 1
b

# Alpha
c
";
        assert_eq!(anchors(doc), vec!["alpha", "alpha-1", "alpha-2"]);
        assert!(extract(doc, "alpha-2").unwrap().contains("\nc"));
    }


    const DOC: &str = "---\ntitle: x\n---\n# Top\nintro\n\n## Alpha\na-body\n\n### Deep\nd-body\n\n## Beta\nb-body\n\n```sh\n# not a heading\n```\n\n## Alpha\ndupe-body\n";

    #[test]
    fn section_spans_to_same_or_higher_level() {
        let s = extract(DOC, "alpha").unwrap();
        assert!(s.starts_with("## Alpha"));
        assert!(s.contains("### Deep"), "the subsections are included");
        assert!(!s.contains("## Beta"), "the section stops at the sibling");
    }

    #[test]
    fn fenced_hashes_are_not_headings_and_dupes_get_suffixes() {
        assert_eq!(
            anchors(DOC),
            vec!["top", "alpha", "deep", "beta", "alpha-1"]
        );
        let s = extract(DOC, "alpha-1").unwrap();
        assert!(s.contains("dupe-body"));
    }

    #[test]
    fn last_section_runs_to_eof_and_missing_anchor_is_none() {
        let s = extract(DOC, "alpha-1").unwrap();
        assert!(s.ends_with("dupe-body\n"));
        assert!(extract(DOC, "nope").is_none());
    }

    #[test]
    fn front_matter_does_not_shift_extraction() {
        let s = extract(DOC, "top").unwrap();
        assert!(s.starts_with("# Top"));
        assert!(!s.contains("title: x"));
    }
}
