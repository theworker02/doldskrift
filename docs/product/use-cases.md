# Use cases

Grounded scenarios where a machine writing system that survives pixels is useful. These are **not** customer testimonials or market-size claims.

## Good fits

### Machine-readable displays

Agent dashboards, build walls, and status panes that must be recoverable from a screenshot or camera without relying on Latin OCR of human UI chrome.

### Agent debugging surfaces

Opaque-to-humans glyph streams that still round-trip exactly for tools — useful when operators should not casually read payload content over a shoulder, without treating Open mode as encryption. For real confidentiality, use external AEAD today or Protected mode (Phase V) when Gate ships.

### Camera-readable device coordination (experimental)

Optical handoff between devices when RF is unavailable or undesirable. DVE maturity is experimental; treat this as research and Lab demos until measured vision benches and field tests say otherwise.

### Visual telemetry / HCI research

Studying how much human familiarity can be removed while keeping machine separability. See [`docs/research/`](../research/).

### Air-gap *demonstrations*

Showing that structured data can cross an optical channel. Demonstration ≠ production air-gap security product.

### Printed machine-oriented media

Specimens, labels, and print layouts meant for scanners/agents first. Provide a human semantic channel whenever people must understand the content.

## Poor fits

| Need | Prefer instead |
|------|----------------|
| Ubiquitous public scanning | QR / existing barcode ecosystems |
| Confidentiality / authenticity | Real AEAD, signatures, TLS |
| Accessible human UI | Clear text + ARIA; see [`accessibility.md`](../accessibility.md) |
| Drop-in “secure font” marketing | Do not — see [`SECURITY.md`](../../SECURITY.md) |

## Related

- [Positioning](positioning.md)
- [Product site](../../site/product.html)
- [Security model](../security.md)
