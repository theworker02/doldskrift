# Build handbook

Short guide for building Doldskrift on a developer machine (including Windows).

## Prerequisites

- Rust toolchain (stable) via [rustup](https://rustup.rs/)
- Node.js + npm (static GitHub Pages site only)
- Git

Optional: `wasm-pack` when working on `doldskrift-bindings`.

## Workspace

From the repository root:

```bash
cargo build --workspace
cargo test --workspace
cargo install --path crates/doldskrift-cli
```

The Lab CLI binary is `dold`.

```bash
dold doctor
echo "hello" | dold encode > message.dsk
dold decode message.dsk
```

## Static site (GitHub Pages)

The public web presence is a **static** multi-page site under `site/`, deployed only via GitHub Pages (`.github/workflows/pages.yml` → `site/dist`). There is no separate Vercel/Netlify/Render website product.

```bash
npm install
npm run site:preview   # vite build + preview of site/dist
# npm run site         # optional: Vite hot-reload while editing
```

- Config: `site/vite.config.js` (`base: "./"` for project Pages under `/doldskrift/`)
- Entries: Lens, Neural, Technology, Demo, examples, etc.
- Details: [`site/README.md`](../../site/README.md)

## Fonts / specimens

```bash
dold font build --out fonts/generated --seed 1
dold logo --out assets/brand/logo-mark.svg
```

## Features

Prefer workspace feature flags documented in [`dependencies.md`](dependencies.md). Vision-heavy deps stay behind the vision crate so the core codec stays lean.

## Quality bar one-liners

```bash
# Makefile / justfile (if available)
make check          # fmt + clippy -D warnings + cargo test + npm test
make doctor
make site:build
make fixtures
make links

# npm
npm run check
npm run site:build
npm run links
npm run fixtures
```

Rustdocs for public APIs (core crate warns on missing docs):

```bash
cargo doc -p doldskrift --no-deps
# open target/doc/doldskrift/index.html
```

## cargo-deny / SBOM notes

- Config: root [`deny.toml`](../../deny.toml) (licenses, advisories, sources).
- CI job runs EmbarkStudios `cargo-deny-action` with **`continue-on-error: true`** until the allowlist is fully hardened — treat as a release blocker when cutting tags.
- Local: `cargo deny check` (install [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)).
- Informal inventory: `cargo tree`. No formal SPDX SBOM artifact is published yet.

## CI

GitHub Actions: `.github/workflows/ci.yml` (rustfmt, clippy `-D warnings`, tests, conformance, font build, surfaces fixtures, JS tests, site build, link check, advisory cargo-deny). Pages deploy: `.github/workflows/pages.yml`.

## Related

- [Testing](testing.md)
- [CLI guide](../guides/cli.md)
- [Contributing](../../CONTRIBUTING.md)
