---
name: vitrine
description: Build interactive HTML artifacts that transclude canonical markdown with vitrine — plans, reports, and review pages in .vitrine/<slug>/ that reference tisket issues and zettel notes instead of copying them, plus round-trip forms whose responses land as JSON files. Use when creating a plan or report artifact in a repo with a .vitrine/ or .tisket/ directory, or when a deliverable should stay in sync with its markdown source.
---

# vitrine

Artifacts transclude; they never copy. If a page needs content that
exists in a tisket issue, zettel note, or repo markdown file, reference
it — the markdown stays canonical and the page can never drift.

## Create and author

    vitrine new my-plan        # scaffolds .vitrine/my-plan/

Edit `.vitrine/my-plan/index.html`. For canonical content, resolve a
reference and drop in an element:

    vitrine resolve tisket:ab12#goal
    → ../../.tisket/v0.1.0/ab12-the-issue.md#goal

    <md-section ref="../../.tisket/v0.1.0/ab12-the-issue.md#goal"></md-section>

Anchors are heading slugs; list a file's anchors with
`vitrine extract <file>`. Rules that matter:

- Never write content inside an `<md-section>` — it is derived,
  replaced on every sync and by live render. If the content is wrong,
  fix the markdown, not the artifact.
- `<md-section>` elements must not nest.
- Always run `vitrine sync <slug>` after authoring or after markdown
  edits, so the baked (file://-portable) copy is current.

## Round-trip forms

    <form data-respond>
      <select name="ab12#constraint-3"><option>accept</option><option>reject</option></select>
      <button>Submit</button>
    </form>

Under `vitrine serve`, submissions land in
`.vitrine/<slug>/responses/` and `latest.json`, and the server prints
`response saved: <slug>` per submission — watch for that line, then
read `latest.json`. Name fields with stable `issue#anchor` references
so decisions survive content reordering.

## Serve

    vitrine serve              # http://127.0.0.1:4114/.vitrine/<slug>/

Any static server renders artifacts (live transclusion needs no
backend); only the response inbox requires `vitrine serve`.

## Full reference

    vitrine docs getting-started
    vitrine docs authoring
