# Authoring reference

## References

Write a reference as `<md-section ref="PATH[#ANCHOR]">`. PATH is
relative to the artifact directory, which is `.vitrine/<slug>/`. Thus a
repo file is `../../<path>`.

Use `vitrine resolve` to make a relative path from a `tisket:<id>`,
`zettel:<id>`, or `file:<path>` scheme. A committed artifact holds
relative paths only, so it needs no resolver.

An anchor is a heading slug. The slug is lowercase. It keeps the
alphanumerics, and it collapses each run of other characters to one
hyphen. For example, `## Design constraints (v2)` gives
`design-constraints-v2`. Duplicate headings get a `-1`, `-2`, … suffix
in document order. vitrine reads ATX headings only. A section starts at
its heading and stops at the next heading of the same level or a higher
level. To list the anchors of a file, run `vitrine extract <file>`.

Rules:

- Do not nest one `<md-section>` element in another.
- Do not edit the content inside the element. `vitrine sync` bakes that
  content, and each sync and each live render replaces it.
- The canonical markdown always wins. If an artifact shows the wrong
  content, correct the markdown and run `vitrine sync` again.

## Response forms

A `<form data-respond>` sends its fields as JSON to `/respond/<slug>`
on submit. This works under `vitrine serve` only. The server writes
each submission to `.vitrine/<slug>/responses/<stamp>.json` and to
`latest.json`. The server also writes one `response saved: <slug>` line
to stdout for each submission. A watcher reads that line. Name each
field with a stable reference, such as `ab12#constraint-3`. A recorded
decision then survives a change to the order of the content.

## Rendering profile

Both renderers use plain CommonMark: comrak in the CLI, and the
vendored commonmark.js reference implementation in the browser. Both
renderers strip the YAML front matter before they render. There are no
extensions, and neither renderer emits heading ids. This parity keeps
the baked snapshot and the live render the same.
