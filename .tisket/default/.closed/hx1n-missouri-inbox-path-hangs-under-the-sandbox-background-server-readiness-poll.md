---
title: "missouri inbox path hangs under the sandbox (background server + readiness poll)"
status: done
priority: 3
assignee:
labels: [tests]
depends_on: []
created: 2026-08-13T17:25:25Z
updated: "2026-08-14T19:24:37Z"
---

The inbox state starts 'vitrine serve &' then loops 'until curl -sf .../index.html'. Under the missouri sandbox this hangs and never returns a result line; confirmed pre-existing (hangs with the security fix stashed too), so it is a harness fragility, not a serve defect. serve is verified correct by hand: index 200, .env 404, same-origin POST saved, cross-origin POST 403. Fix direction: give the readiness poll a bounded timeout and a hard server kill, or drive the server from a fixed ephemeral port with a health check.

## Scratch Notes

RESOLVED: the hang does not reproduce with the fixed missouri (--no-use-registries, current main build); it was observed with the stale April system missouri. Six consecutive green runs in full nix mode. The fixture is also hardened: bounded readiness poll (5s, prints server.log on failure) and a PID-derived port instead of fixed 4199, so a dead server or a parallel port collision cannot hang the suite again.
