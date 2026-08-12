---
name: vitrine
description: Make interactive HTML artifacts with vitrine. An artifact is a plan, report, or review page in .vitrine/<slug>/. It transcludes canonical markdown, so it references tisket issues and zettel notes instead of copying them. It can also carry round-trip forms that write the responses as JSON files. Use this skill to make a plan or report artifact in a repo that has a .vitrine/ or .tisket/ directory. Also use it when a deliverable must stay in sync with its markdown source.
---

# vitrine

An artifact transcludes content. An artifact never copies content. If a
page needs content that is in a tisket issue, a zettel note, or a repo
markdown file, reference that file. The markdown stays canonical, and
the page cannot drift.

## Make and author

    vitrine new my-plan        # makes .vitrine/my-plan/

Edit `.vitrine/my-plan/index.html`. For canonical content, resolve a
reference and add an element:

    vitrine resolve tisket:ab12#goal
    → ../../.tisket/v0.1.0/ab12-the-issue.md#goal

    <md-section ref="../../.tisket/v0.1.0/ab12-the-issue.md#goal"></md-section>

An anchor is a heading slug. To list the anchors of a file, run
`vitrine extract <file>`. These rules are important:

- Never write content inside an `<md-section>` element. That content is
  derived, and each sync and each live render replaces it. If the
  content is wrong, correct the markdown, not the artifact.
- Do not nest one `<md-section>` element in another.
- Always run `vitrine sync <slug>` after you author the page, and after
  each markdown edit. The baked copy is then current, and the page
  works from `file://`.

## Round-trip forms

    <form data-respond>
      <select name="ab12#constraint-3"><option>accept</option><option>reject</option></select>
      <button>Submit</button>
    </form>

Under `vitrine serve`, each submission goes to
`.vitrine/<slug>/responses/` and to `latest.json`. The server also
writes one `response saved: <slug>` line for each submission. Watch for
that line, then read `latest.json`. Name each field with a stable
`issue#anchor` reference. A decision then survives a change to the
order of the content.

## Serve

    vitrine serve              # http://127.0.0.1:4114/.vitrine/<slug>/

Any static server shows an artifact, because live transclusion needs no
backend. The response inbox needs `vitrine serve`.

## Full reference

    vitrine docs getting-started
    vitrine docs authoring
