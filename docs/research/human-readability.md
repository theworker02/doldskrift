# Research: human readability

Doldskrift deliberately scores glyphs for **low** resemblance to Latin letters and digits while keeping them separable for machines.

## Heuristic

During generation, `human_readability_score` estimates familiarity. Candidates scoring ≥ ~0.55 are regenerated with spun seeds (except `DSK_PROJECT_MARK`).

This is a **heuristic**, not a psychophysical model. It exists to bias the search away from accidental “looks like E” glyphs.

## Tradeoffs

| More alien | Risk |
|------------|------|
| Heavier abstraction | Harder for humans to proofread agent logs |
| Higher GRE density | Visual clutter |
| Session permutation | Same glyphs, shuffled meaning — little extra human opacity once font is known |

## Accessibility collision

Human-hostile glyphs are fine for agent specimens; they are not fine as the only channel for human-required information. Policy: [`ACCESSIBILITY.md`](../../ACCESSIBILITY.md).

## Open experiments

- Controlled user studies (time-to-comprehend vs plaintext)
- Whether Micro density accidentally increases Latin resemblance at tiny sizes
- Operator UX for “reveal semantic” toggles without breaking agent demos
