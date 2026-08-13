---
title: "SECURITY: live render assigns unsanitized commonmark.js output to innerHTML (XSS)"
status: todo
priority: 1
assignee:
labels: [security, bug]
depends_on: []
created: "2026-08-13T02:06:20Z"
updated: "2026-08-13T02:06:20Z"
---

assets/vitrine.js:64 uses commonmark.js defaults (raw HTML + dangerous URLs emitted), then :83 assigns to innerHTML. Markup inside a referenced tisket issue runs in the served page, same origin as the response inbox. Also breaks the parity guarantee: comrak (bake) strips raw HTML, commonmark.js (live) does not, so baked and served pages differ — the exact drift the product prevents. README:60-61 and docs/authoring.md:41-45 state parity as fact. Fix: enable safe/sanitizing render on the JS side to match comrak; use textContent or a sanitizer.
