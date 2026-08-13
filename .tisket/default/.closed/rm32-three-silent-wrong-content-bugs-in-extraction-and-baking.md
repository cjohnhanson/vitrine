---
title: "three silent wrong-content bugs in extraction and baking"
status: done
priority: 2
assignee:
labels: [bug]
depends_on: []
created: 2026-08-13T02:06:20Z
updated: "2026-08-13T17:53:29Z"
---

1) A # line inside a 4-space indented code block reads as a heading and truncates the section (src/extract.rs:25) — this project's docs use indented code. 2) The bake attribute reader matches data-ref as ref and bakes the wrong file (src/bake.rs:97). 3) Duplicate heading slugs collide: # Alpha / # Alpha 1 / # Alpha emits alpha-1 twice and resolves the wrong heading (src/extract.rs:44). Each exits 0. Also: no test loads the shipped JS runtime; the parity test writes its own stripper and uses a fixture with no raw HTML, so it cannot fail.
