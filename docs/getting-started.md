# Getting started

vitrine gives a plaintext repo interactive artifacts: HTML pages that
*transclude* sections of your canonical markdown — tisket issues,
zettel notes, any repo file — so content is written once and rendered
wherever it's needed. Plus a response inbox, so an artifact can carry
forms whose submissions land back in the repo as JSON for an agent to
read.

## Create an artifact

    vitrine new q4-plan

This scaffolds `.vitrine/q4-plan/` with a styled `index.html` and the
runtime (`vitrine-runtime.js`, self-contained — no CDN). Edit the page;
where you want canonical content, add:

    <md-section ref="../../.tisket/v0.1.0/ab12-the-issue.md#goal"></md-section>

Get the relative path from a scheme reference:

    vitrine resolve tisket:ab12#goal

## Bake and serve

    vitrine sync          # bake transcluded content into the page
    vitrine serve         # http://127.0.0.1:4114/.vitrine/q4-plan/

The portability ladder:

1. **file:// with no JS** — the baked copy renders as plain HTML.
2. **Any static server** — the runtime fetches the markdown live, so
   edits to the canonical file appear on refresh.
3. **vitrine serve** — adds the response inbox for round-trip forms.

Baked content is derived, never hand-edited; re-run `vitrine sync`
after markdown edits to refresh snapshots (or rely on live rendering
when served).

## Inspect markdown

    vitrine extract file.md            # list anchors
    vitrine extract file.md the-anchor # print a section
    vitrine render file.md             # rendered HTML (parity-testable)
