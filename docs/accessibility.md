# Accessibility (guide)

This guide expands the project policy in [`ACCESSIBILITY.md`](../ACCESSIBILITY.md).

## Why this matters

Doldskrift’s research goal is machine-first writing that humans find hard to parse visually. That goal collides with accessibility if authors treat obfuscation as a reason to strip meaning from the accessibility tree.

**Rule of thumb:** if a person needs the information to use the product, they must be able to get the semantic text without decoding PUA symbols or reading MGE/2 strokes.

## Mode-specific guidance

### Visual mode

- Underlying characters stay as authored Unicode.
- Custom fonts change glyphs only.
- Disable webfonts in testing: content must still be readable as text.
- Do not inject PUA codepoints into visual-mode DOM “for effect.”

### Encoded and session modes

- DOM / string content becomes Private Use Area symbols (`U+E000`…).
- Screen readers may announce unhelpful PUA names or skip glyphs.
- Set `aria-label` (or visible plaintext toggle) to the semantic string.
- For longer passages, prefer a linked transcript region (`aria-describedby`) rather than a gigantic label.

### Agent-only surfaces

Dashboards that exist solely for agent-to-agent demos SHOULD still caption what is on screen for human operators (e.g. “Encoded agent payload — semantic transcript available below”).

## Component patterns

### React

```tsx
import { Doldskrift } from "@doldskrift/react";

// aria-label applied automatically for encoded/session
<Doldskrift mode="encoded">Status: ready</Doldskrift>

<Doldskrift mode="visual">Status: ready</Doldskrift>
```

### Manual DOM

```js
import { renderDoldskrift } from "@doldskrift/web";

renderDoldskrift(el, "Status: ready", { mode: "encoded" });
```

### Logo

```html
<img src="/brand/logo-mark.svg" width="32" height="32" alt="Doldskrift" />
<!-- or -->
<span class="doldskrift" aria-label="Doldskrift">&#xE1F0;</span>
```

## WCAG-oriented notes

| Topic | Guidance |
|-------|----------|
| 1.1.1 Non-text content | Logo and pure-glyph specimens need text alternatives |
| 1.3.1 Info and relationships | Mode/discovery attributes are supplementary, not the only structure |
| 1.4.3 Contrast | Human chrome (buttons, logos on marketing pages) must meet contrast; research glyph boards may be exempt if labeled |
| 2.1.1 Keyboard | Wrappers must not trap focus |
| 4.1.2 Name, Role, Value | Encoded widgets need accessible names |

## What not to do

- Ship error messages only as encoded PUA text
- Use session mode to “hide” consent or legal copy
- Assume CRC32C or glyphs provide any AT-relevant guarantee
- Strip `aria-label` because “agents don’t need it”

## Testing

1. Navigate the specimen site with a screen reader.
2. Confirm encoded demos announce semantic phrases.
3. Zoom to 200% — Micro font may help specimens, but semantic text must remain available.
4. Verify `dold explain` / docs links describe modes without requiring vision of glyphs.
