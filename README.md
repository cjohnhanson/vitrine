# 🪟 vitrine

> A vitrine is a glass display case: finished work, on view, behind
> glass — you look at it; you don't reach in and smudge it.

Interactive artifacts for plaintext repos. HTML pages that *transclude*
sections of canonical markdown — tisket issues, zettel notes, any repo
file — so content is written once and rendered wherever it's needed,
plus a response inbox so artifacts can carry forms whose answers land
back in the repo as files agents can read.

## The idea

Plaintext is the source of truth; some deliverables want to be visual
and interactive anyway (plans with diagrams, review findings with
accept/reject toggles, side-by-side comparisons). Copying markdown into
HTML creates two versions that drift. vitrine's `<md-section>` element
transcludes instead:

```html
<md-section ref="../../.tisket/v0.1.0/ab12-the-issue.md#goal"></md-section>
```

The artifact never holds its own copy of content — drift is
structurally impossible.

## Portability ladder

1. **file:// with no JS** — `vitrine sync` bakes rendered sections into
   the page as plain HTML (derived, regenerated, never hand-edited).
2. **Any static server** — the runtime re-renders live from the
   markdown; edits appear on refresh. No resolver, no backend: refs are
   relative paths, resolved at authoring time by `vitrine resolve`.
3. **`vitrine serve`** — adds the response inbox:
   `<form data-respond>` submissions land in
   `.vitrine/<slug>/responses/` and `latest.json`, with a
   `response saved: <slug>` stdout line per submission for watchers.

## Commands

```
vitrine new <slug>          # scaffold .vitrine/<slug>/ (page + runtime)
vitrine resolve tisket:ab12#goal
vitrine sync [slug]         # bake transclusions
vitrine serve [--port N]    # static files + response inbox (default 4114)
vitrine extract <md> [anchor]
vitrine render <md>         # the fixed rendering profile, for parity checks
vitrine docs [topic]
```

## Rendering contract

Plain CommonMark on both sides — comrak in the CLI, the vendored
commonmark.js reference implementation in the browser — front matter
stripped, no emitted heading ids. Renderer parity is byte-for-byte and
tested (the missouri suite diffs both renderers' output on the same
input). Anchors are heading slugs; duplicates get `-1`, `-2`… suffixes.

## Status

Working: transclusion (bake + live), scaffolding, resolve/extract,
response inbox, bundled docs. Tested by a missouri state-graph suite
and cargo unit tests.
