# Getting started

vitrine gives a plaintext repo interactive artifacts. An artifact is an
HTML page that *transcludes* sections of your canonical markdown. The
markdown is a tisket issue, a zettel note, or any repo file. You write
the content once, and the artifact renders it where you need it. An
artifact can also carry a response form. The submissions go back into
the repo as JSON files that an agent reads.

## Make an artifact

    vitrine new q4-plan

This makes `.vitrine/q4-plan/` with a styled `index.html` and the
runtime. The runtime file is `vitrine-runtime.js`. It is
self-contained, so it needs no CDN. Edit the page. Where you want
canonical content, add an element:

    <md-section ref="../../.tisket/v0.1.0/ab12-the-issue.md#goal"></md-section>

Get the relative path from a scheme reference:

    vitrine resolve tisket:ab12#goal

## Bake and serve

    vitrine sync          # bake the transcluded content into the page
    vitrine serve         # http://127.0.0.1:4114/.vitrine/q4-plan/

The portability ladder:

1. **file:// with no JS**: the baked copy shows as plain HTML.
2. **Any static server**: the runtime gets the markdown live, so your
   edits to the canonical file show after a page refresh.
3. **vitrine serve**: adds the response inbox for round-trip forms.

The baked content is derived. Do not edit it by hand. Run `vitrine
sync` again after each markdown edit to refresh the snapshots. Under a
server, the live rendering does this for you.

## Examine the markdown

    vitrine extract file.md            # list the anchors
    vitrine extract file.md the-anchor # print one section
    vitrine render file.md             # the rendered HTML, for parity checks
