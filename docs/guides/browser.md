# Browser guide

## Goals in the browser

1. Render agent text in visual or encoded form.
2. Advertise Doldskrift via HTML discovery hooks.
3. Keep accessibility labels for humans.
4. Optionally ship `.dsk` as `application/vnd.doldskrift` downloads.

## Quick DOM helper

```bash
npm install @doldskrift/web
```

```js
import { renderDoldskrift, encodeDoldskrift, decodeDoldskrift } from "@doldskrift/web";

const el = document.querySelector("#agent");
renderDoldskrift(el, "hello agent", { mode: "encoded" });
```

This adds classes `doldskrift agent-text`, sets `data-doldskrift="1"` / `data-doldskrift-mode`, encodes when needed, and sets `aria-label` for encoded/session.

## Discovery markup

```html
<meta name="doldskrift" content="version=1;mode=encoded" />

<div
  data-doldskrift="1"
  data-doldskrift-mode="session"
  data-doldskrift-version="1"
  class="doldskrift agent-text"
></div>
```

## Fonts

```css
@font-face {
  font-family: "DoldskriftText";
  src: url("/fonts/Doldskrift-Regular.svg") format("svg");
  font-display: block;
}

.agent-text {
  font-family: "DoldskriftText", sans-serif;
  font-size: 1.25rem;
  letter-spacing: 0.02em;
}
```

See [font installation](font-installation.md). Generated CSS may already exist at `fonts/generated/doldskrift.css`.

## Encoded vs visual in the DOM

| Mode | `textContent` | AT |
|------|---------------|----|
| visual | Semantic Unicode | Reads text directly |
| encoded/session | PUA symbols | Needs `aria-label` |

Remember: anyone can read `textContent` or disable the font. Obfuscation is visual only.

## Containers in the browser

```js
import { DskDocument } from "@doldskrift/core";

const doc = DskDocument.encodeText("payload", "encoded");
const blob = new Blob([doc.toBytes()], { type: "application/vnd.doldskrift" });
const url = URL.createObjectURL(blob);
```

## Examples

- `examples/browser/`
- `examples/browser-encoded/`
- `examples/browser-visual/`
- Specimen site under `site/`
