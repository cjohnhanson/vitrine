---
title: "SECURITY: vitrine serve publishes the whole repo root; inbox POST has no Origin check"
status: done
priority: 1
assignee:
labels: [security, bug]
depends_on: []
created: 2026-08-13T02:06:20Z
updated: "2026-08-13T17:25:25Z"
---

serve is not confined to .vitrine/ and the referenced markdown. A checker read .env and .git/HEAD over HTTP. The response inbox accepts a cross-origin simple POST with no Origin check and overwrote latest.json with a text/plain request. Neither is documented. Fix: serve only .vitrine/<slug>/ plus resolved referenced files; require an Origin/same-site check on /respond.
