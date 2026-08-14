<p align="center">
  <img src="../assets/brand/logo-mark.svg" alt="Doldskrift" width="72" />
</p>

# Doldskrift static site (GitHub Pages)

This folder is the **only** public web surface for the project: a Vite-built **static** multi-page site deployed with GitHub Pages. There is no separate hosted web app, Vercel/Netlify app, or alternate website product.

Brand marks in `public/` (`logo-mark.svg`, `favicon.svg`, …) mirror `assets/brand/` (regenerate with `dold logo`, then sync). Nav/footer show the Swedish etymology (*dold* + *skrift* → concealed script).

## Canonical URL

After Pages is enabled on the repo:

`https://doldskrift.github.io/doldskrift/`

Build uses `base: "./"` so asset URLs stay relative under that project path.

## Deploy

[`.github/workflows/pages.yml`](../.github/workflows/pages.yml) builds `doldskrift-site` and publishes `site/dist` via `actions/deploy-pages`.

**Enable once** in the GitHub repo: **Settings → Pages → Source: GitHub Actions**  
(Do not use “Deploy from a branch” for this workflow.)

After the first successful Actions run, the site is at `https://doldskrift.github.io/doldskrift/`.

Sponsor / thanks.dev: [`funding.html`](./funding.html) · footer CTA · [`.github/FUNDING.yml`](../.github/FUNDING.yml).

## Local preview

```bash
# from repo root
npm install
npm run build -w doldskrift-site
npm run preview -w doldskrift-site
```

For editing with hot reload:

```bash
npm run site
```

(`npm run site` starts the Vite dev server only — production remains the static Pages build.)

## Layout

| Path | Role |
|------|------|
| `*.html` | Multi-page entries (Lens, Neural, docs portal, etc.) |
| `src/` | Shared JS/CSS (bundled into `dist/assets`) |
| `public/` | Copied as-is into `dist/` (logos, `wasm/`, `.nojekyll`) |
| `dist/` | Static output (gitignored; CI artifact for Pages) |
