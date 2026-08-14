# Protected mode (product stub)

**Status:** Platform **Protected** profile — milestone 1–2 stubs shipping (`Readable: No` inspect, `decode` refuse, `open` → AccessDenied). **No AEAD yet.** Architecture: [ADR-0006](../adr/ADR-0006-gate-protected-visual-objects.md). Gate stub: [Gate](../engineering/gate.md). Neural profile: [ADR-0007](../adr/ADR-0007-dsk3-neural-learned-machine-grammar.md).

## One-line distinction

| Mode | Product claim |
|------|----------------|
| **Open** (DSK/1–2, Lens demos) | Machine-native **encoding**. Not encryption. Glyphs → symbols → plaintext. |
| **Neural** (DSK/3 Neural) | Learned latent visuals + compatible reader. **Not encryption.** See [Neural overview](../neural/overview.md). |
| **Protected** (DSK/3 Protected) | Machine-native **encoding of ciphertext**. Authenticated encryption + Gate authorization. |

## Operator story

1. Agent or service seals a semantic object → Protected visual artifact.
2. Humans and unauthenticated tools see abstract glyphs; `scan` / `inspect` report **Readable: No**.
3. An authorized runtime presents a capability to `dold open` / `gate.open` and receives only permitted results.

## What partners may say

- Font reverse-engineering does not yield message semantics for Protected objects.
- Recovering symbols from pixels stops at ciphertext without a capability.
- Open demos remain fully decodable by design — do not market demos as Protected.

## What partners must not say

- “AI-only / human-impossible” cryptography
- That screenshots or a compromised Gate cannot leak content
- That all Doldskrift visuals are encrypted

## Related

- [`SECURITY.md`](../../SECURITY.md)
- [Positioning](positioning.md)
- [`ROADMAP.md`](../../ROADMAP.md) — Protected track
