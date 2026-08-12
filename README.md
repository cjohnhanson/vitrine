# 🪟 vitrine

> A vitrine is a glass display case: finished work, on view, behind
> glass — you look at it; you don't reach in and smudge it.

Interactive artifacts for plaintext repos. An artifact is an HTML page
that *transcludes* sections of canonical markdown. The markdown is a
tisket issue, a zettel note, or any repo file. You write the content
once, and the artifact renders it where you need it. An artifact can
also carry a response form. The answers go back into the repo as files
that an agent reads.

## The idea

The plaintext is the source of truth. Some deliverables must be visual
and interactive: a plan with diagrams, review findings with accept and
reject controls, a side-by-side comparison. If you copy the markdown
into HTML, you make two versions, and the two versions drift. The
vitrine `<md-section>` element transcludes the markdown instead:

```html
<md-section ref="../../.tisket/v0.1.0/ab12-the-issue.md#goal"></md-section>
```

The artifact keeps no copy of the content. Thus the content cannot
drift.

## Portability ladder

1. **file:// with no JS**: `vitrine sync` bakes the rendered sections
   into the page as plain HTML. The baked content is derived. Do not
   edit it by hand.
2. **Any static server**: the runtime renders the markdown live. Your
   edits show after a page refresh. A ref is a relative path, so the
   artifact needs no resolver and no backend. Make the relative paths
   at authoring time with `vitrine resolve`.
3. **`vitrine serve`**: adds the response inbox. A
   `<form data-respond>` submission goes to
   `.vitrine/<slug>/responses/` and to `latest.json`. The server writes
   one `response saved: <slug>` line to stdout for each submission. A
   watcher reads that line.

## Commands

```
vitrine new <slug>          # make .vitrine/<slug>/ (page + runtime)
vitrine resolve tisket:ab12#goal
vitrine sync [slug]         # bake the transclusions
vitrine serve [--port N]    # static files + response inbox (default 4114)
vitrine extract <md> [anchor]
vitrine render <md>         # the fixed rendering profile, for parity checks
vitrine docs [topic]
```

## Rendering contract

Both renderers use plain CommonMark: comrak in the CLI, and the
vendored commonmark.js reference implementation in the browser. Both
renderers strip the front matter. Neither renderer emits heading ids.
The two renderers agree byte for byte. A test holds them to it: the
missouri suite compares the output of both renderers on one input. An
anchor is a heading slug. Duplicate slugs get a `-1`, `-2`, … suffix.

## Status

These parts work: transclusion, baked and live; scaffolding; resolve
and extract; the response inbox; and the bundled docs. A missouri
state-graph suite and the cargo unit tests cover them.
