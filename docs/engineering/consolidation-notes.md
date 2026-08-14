# Consolidation notes

Audit of package and narrative consolidation for the unified platform product. No mass deletion of historical ADRs.

## Platform narrative

| Before | After |
|--------|-------|
| “Phase I… Phase V bolted on” as the story | Present-tense **Open · Neural · Protected** platform |
| ROADMAP as phase changelog | ROADMAP = current product + deepen tracks |
| Product docs speaking in future phase tense | Product docs describe today’s architecture |

ADRs remain historical decision records under `docs/adr/`.

## JavaScript packages (after)

| Package | Role |
|---------|------|
| `@doldskrift/core` (`packages/doldskrift-js`) | **Primary SDK** — codec, constants, browser helpers, CSS |
| `@doldskrift/web` | Thin re-export adapter (backward compatible) |
| `@doldskrift/react` | React peer-dep adapter |
| `doldskrift-bindings` | Generated WASM from Rust |

Browser helpers (`renderDoldskrift`, CSS) live in **core**; web is intentionally thin.

## Rust crates (≤5 — unchanged)

`doldskrift` · `doldskrift-cli` · `doldskrift-vision` · `doldskrift-font` · `doldskrift-bindings`

## Constants

- Canonical JSON: `spec/protocol-constants.json`
- JS mirror: `packages/doldskrift-js/src/protocol-constants.json`
- `dold doctor` reports Rust ↔ spec ↔ JS sync when run from repo root

## Funding / brand

- [`.github/FUNDING.yml`](../../.github/FUNDING.yml) — `github: [theworker02]`, `thanks_dev: u/gh/theworker02`
- Brand mark: `assets/brand/logo-mark.svg` synced to `site/public/` + `packages/*/logo.svg`
- Export: `dold brand` / `dold logo`

## Still open

- Replacing `site/src/doldskrift.js` encode/decode with WASM / `@doldskrift/core` (site still has a local subset for demos)
- Deduplicating every docs cross-link
- Publishing crates.io / npm
