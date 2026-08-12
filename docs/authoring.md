# Authoring reference

## References

`<md-section ref="PATH[#ANCHOR]">` — PATH is relative to the artifact
directory (`.vitrine/<slug>/`), so repo files are `../../<path>`.
Resolve `tisket:<id>`, `zettel:<id>`, and `file:<path>` schemes to
relative paths with `vitrine resolve`; committed artifacts contain only
relative paths and depend on no resolver.

Anchors are heading slugs: lowercase, alphanumerics kept, everything
else collapsing to single hyphens (`## Design constraints (v2)` →
`design-constraints-v2`). Duplicate headings get `-1`, `-2`… suffixes
in document order. ATX headings only; a section spans from its heading
to the next heading of the same or higher level. List a file's anchors
with `vitrine extract <file>`.

Rules:

- `<md-section>` elements must not nest.
- Content inside the element is derived (baked by `vitrine sync`) —
  never hand-edit it; it is replaced on every sync and by live render.
- The canonical markdown always wins. If an artifact seems wrong, fix
  the markdown and re-sync.

## Response forms

A `<form data-respond>` posts its fields as JSON to
`/respond/<slug>` on submit (only under `vitrine serve`). Submissions
are written to `.vitrine/<slug>/responses/<stamp>.json` and
`latest.json`, and the server prints `response saved: <slug>` per
submission for watchers. Name fields with stable references
(`ab12#constraint-3`) so recorded decisions survive content
reordering.

## Rendering profile

Plain CommonMark on both sides — comrak in the CLI, the vendored
commonmark.js reference implementation in the browser — with YAML
front matter stripped before rendering. No extensions, no emitted
heading ids: renderer parity is what keeps the baked snapshot and the
live render identical.
