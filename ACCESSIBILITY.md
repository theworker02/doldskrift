# Accessibility

Doldskrift creates an intentional tension: glyphs are hard for humans to read, while machines decode them easily. Accessibility policy exists so that tension never becomes an excuse to withhold information from people who need it.

## Normative rules

1. **Visual mode** MUST keep semantic Unicode in the underlying text / DOM. Assistive technology that reads the accessibility tree SHOULD observe the same characters as the semantic authoring string.
2. Applications SHOULD provide a path to expose semantic text to AT when content is meant for humans — for example `aria-label`, `aria-describedby`, a parallel plaintext channel, an “accessible transcript,” or a toggle that swaps to a readable font.
3. **Encoded / session modes** replace characters with Private Use Area symbols. When those modes are shown in UI meant for people, authors SHOULD attach the semantic string via accessible naming (`aria-label` or equivalent).
4. Doldskrift MUST NOT be used to hide legally or functionally required accessibility information (forms, errors, consent, safety warnings, primary navigation labels, and similar).
5. Decorative agent-only specimens MAY omit human-readable alternatives when the surrounding page states that the content is non-essential ornament / research demo — still prefer a caption explaining what is shown.

## React reference behavior

The `@doldskrift/react` `<Doldskrift>` component:

| Mode | DOM text | Accessible name |
|------|----------|-----------------|
| `visual` | Semantic children | Native text (no override) |
| `encoded` | PUA-encoded string | `aria-label` = semantic text |
| `session` | Session-encoded string | `aria-label` = semantic text |

Discovery attributes (`data-doldskrift`, `data-doldskrift-mode`) are for machines; they do not replace accessible naming.

## Web helper behavior

`@doldskrift/web` `renderDoldskrift` sets `aria-label` to the semantic string for encoded/session and leaves visual-mode `textContent` as the semantic string.

## Fonts and AT

Custom fonts (`DoldskriftText`, `DoldskriftMono`, `DoldskriftMicro`) change **appearance**. In visual mode they must not change the characters exposed to AT. If a user stylesheet or OS setting disables webfonts, semantic text must remain usable.

`DSK_PROJECT_MARK` (`U+E1F0`) used as a logo MUST have an accessible name such as “Doldskrift” (see brand docs). Do not rely on the Private Use codepoint alone.

## Testing checklist

- [ ] Screen reader announces semantic meaning for encoded UI demos.
- [ ] Keyboard focus order unchanged by Doldskrift wrappers.
- [ ] Contrast of strokes against background meets WCAG for any human-facing chrome (logo, buttons); glyph specimens may be exempt when labeled as research visuals.
- [ ] `prefers-reduced-motion` respected if the specimen site animates glyphs.
- [ ] Failures / errors in apps are never only shown as encoded PUA text.

## Related

- Longer guide: [`docs/accessibility.md`](docs/accessibility.md)
- Security honesty: [`SECURITY.md`](SECURITY.md)
- Brand / logo: [`docs/brand.md`](docs/brand.md)
