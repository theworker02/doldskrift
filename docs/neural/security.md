# Neural security language

## Mandatory framing

**Neural ≠ encryption.**

A learned visual representation may require a compatible trained reader. That is a **compatibility** property, not a confidentiality guarantee. Parties with access to pixels, models, instrumentation, or side channels may still recover meaning.

## Correct vs incorrect claims

| Incorrect | Correct |
|-----------|---------|
| Impossible to decode | May require a compatible reader / epoch |
| Unbreakable / AI-only encryption | Representation; use Protected/AEAD for secrecy |
| Humans cannot reverse engineer | Humans and tools may still reverse engineer |
| Scan recovers plaintext always | Wrong/unknown reader must fail closed |

## Protected relationship

Confidentiality = **DSK/3 Protected** (AEAD + Gate). See [ADR-0006](../adr/ADR-0006-gate-protected-visual-objects.md) and [`SECURITY.md`](../../SECURITY.md).

Neural + Protected composition (e.g. Neural visual of ciphertext) is deferred.

## Fail closed

Readers must return `Incompatible` / `UnknownGrammar` / `NotDoldskrift` / `ReconstructionFailed` rather than inventing plausible plaintext.
