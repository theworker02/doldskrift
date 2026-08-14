# Brand

The Doldskrift mark is not decoration bolted onto the project — **the logo is a valid machine glyph**.

## Name (Swedish)

| Part | Gloss |
|------|-------|
| *dold* | hidden / concealed |
| *skrift* | writing / script |

Together: **“hidden writing” / “concealed script.”** Tagline: *Maskinskriven betydelse — inte hemlighet.* (Machine-written meaning — not secrecy.)

The Swedish sense of concealment describes visual opacity for humans, **not** cryptographic confidentiality. Open and Neural modes are representation; only Protected (when shipping) is encryption. See `SECURITY.md` and `dold about`.

## Project mark = `DSK_PROJECT_MARK`

| Property | Value |
|----------|-------|
| Name | `DSK_PROJECT_MARK` |
| Codepoint | `U+E1F0` |
| Engine | MGE/2 |
| Assets | `assets/brand/logo-mark.svg`, `fonts/specimens/logo.svg`, `fonts/specimens/logo.png` |

When you need the logo in UI, prefer rendering `U+E1F0` with a Doldskrift font **or** embedding the SVG specimen that matches engine geometry. Do not invent a unrelated wordmark that pretends to be the protocol mark.

Accessible name for the mark: **“Doldskrift”** (never rely on the PUA codepoint alone for AT).

## Logo construction

The mark is built from MGE/2 primitives consistent with the glyph grammar:

- Vertical stem with terminal caps (north / south)
- Open arc / enclosure on the east
- Branch / junction into the center
- Accent dots in the brand teal (`#0F6B5C`) against ink (`#12161A`)
- GRE integrity ticks in the far north-east (machine cue, part of the mark)

Simplified SVG projection: `assets/brand/logo-mark.svg` (128×128 viewBox). Regenerating the official mark MUST keep these structural roles so fingerprints remain stable for the published seed.

## Clear space

Keep an empty margin around the mark of at least **1/8 of the mark’s height** on all sides. Do not let body text, other logos, or busy photography intrude into that margin.

```text
        ← ⅛H →
      ┌─────────┐
      │  ┌───┐  │
  ⅛H  │  │ ★ │  │  ⅛H
      │  └───┘  │
      └─────────┘
        ← ⅛H →
```

## Minimum size

| Medium | Minimum |
|--------|---------|
| Screen UI / favicon | 16×16 CSS px (use Micro-simplified asset if needed) |
| Body / docs | 24×24 CSS px |
| Print | 8 mm height |
| Hero lockups | 64×64 CSS px or larger |

Below minimum size, prefer the monochrome solid mark without forcing GRE ticks to remain individually visible — but do not replace it with a Latin “D” monogram.

## Backgrounds

| Background | Treatment |
|------------|-----------|
| Light (`#F7F5F2` … white) | Default ink `#12161A` + teal accents |
| Dark charcoal | Invert stem/arc to light ink; keep teal accents if contrast ≥ 3:1 |
| Photography | Place on a flat scrim; do not overlay on high-frequency texture |
| Brand teal field | Use light/knockout version of the stem |

Do not place the mark on saturated red/green pairs that destroy accent contrast, and do not add drop shadows, glows, or gradients to the glyph itself.

## Typography

Product name in prose: **Doldskrift** (capital D only).

Protocol / engine labels in monospaced technical contexts:

- `DSK/1`, `MGE/2`, `GRE/1`, `DSKG-1`
- MIME: `application/vnd.doldskrift`, `text/doldskrift`

Marketing headlines SHOULD NOT overpower the wordmark when the mark is present; the mark is the brand signal (see project design rules for landing surfaces).

Pair UI chrome with a distinctive humanist or grotesque — avoid defaulting to Inter / Roboto / Arial as the brand voice. Specimen pages may use the Doldskrift font for encoded samples only.

## Color tokens

| Token | Hex | Use |
|-------|-----|-----|
| Ink | `#12161A` | Primary strokes |
| Teal | `#0F6B5C` | Accents / integrity cues |
| Paper | `#F7F5F2` | Light grounds (optional) |

Avoid purple-gradient “AI defaults” and fake-security neon glows in official materials.

## Incorrect usage

Do **not**:

1. Stretch, rotate randomly, or add 3D bevels to the mark
2. Recolor zone structure so N/E/S/W/C roles become unrecognizable
3. Replace the mark with a Latin letter or generic robot icon
4. Claim the logo “encrypts” or “secures” content
5. Use `U+E1F0` without an accessible name in interactive UI
6. Crop integrity ticks in official downloads (research crops for papers MUST note they are cropped)
7. Place the mark inside rounded “app icon” cards that imply a different product system unless shipping a real app icon derived from the same geometry

## Sync paths (one mark, many mirrors)

Regenerate the canonical mark, then copy to package/site mirrors:

```bash
dold logo --out assets/brand/logo-mark.svg
# mirrors (keep byte-identical or regenerate):
#   site/public/logo-mark.svg
#   packages/doldskrift-js/logo.svg
#   packages/doldskrift-web/logo.svg
#   packages/doldskrift-react/logo.svg
```

Horizontal lockup: `assets/brand/logo-horizontal.svg` (README + site hero).  
README demo specimen: `cargo run -p doldskrift-font --example gen_readme_demo`.

Paper token used on the site chrome is `#F7F4EF` (near Paper); ink `#12161A`; teal `#0F6B5C`.

### Brand kit CLI

```bash
dold brand --out brand-kit          # logo-mark.svg + tokens.json + brand-manifest.json
dold brand --json                   # manifest only (stdout)
dold logo --out assets/brand/logo-mark.svg
```

Funding (not part of the mark, but linked from kit README): `dold funding` → [thanks.dev/u/gh/theworker02](https://thanks.dev/u/gh/theworker02).

Social / OG: `assets/brand/social-card.svg` → sync to `site/public/social-card.svg` (Pages chrome injects `og:image`).

## File checklist for releases

- [ ] `assets/brand/logo-mark.svg` matches `generate_project_mark` geometry for the release seed
- [ ] Specimens under `fonts/specimens/` updated
- [ ] README hero references the SVG / horizontal lockup
- [ ] Docs state logo ≡ `DSK_PROJECT_MARK`
- [ ] Site + package logos synced
- [ ] `site/public/social-card.svg` synced
- [ ] Swedish etymology present on README + site chrome (`dold` + `skrift`)
- [ ] `.github/FUNDING.yml` still points at the maintainer thanks.dev path
