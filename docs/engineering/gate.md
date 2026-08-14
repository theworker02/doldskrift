# Gate API and `dold open` (Phase V stub)

**Status:** Milestone 1–2 CLI stubs landed (`inspect` Protected fields, `decode` refuse, `open` → `AccessDenied`). **No AEAD / runtime key material.** See [ADR-0006](../adr/ADR-0006-gate-protected-visual-objects.md). Shipping helpers: `doldskrift::gate_open`, `DskDocument::encode_protected_stub`.

## Purpose

The **Doldskrift Gate** is the authorized runtime surface for **Protected Visual Objects** (DSK/3). It decrypts ciphertext recovered from glyphs and returns capability-scoped semantic results. Untrusted callers (including LLMs) never receive raw keys.

## CLI contract (target)

| Command | Authorization | Result |
|---------|---------------|--------|
| `dold scan` | None | Frame / visual recovery + Protected metadata; `Readable: No` |
| `dold inspect` | None | Protocol/visual/cipher/session/policy/integrity fields; no plaintext |
| `dold open` | Required | Gate-mediated semantic access under a capability |
| `dold decode` | N/A for Protected | Refuse semantic recovery; explain ciphertext-only reconstruction |

Open-mode documents continue to use `encode` / `decode` as today.

## Rust API sketch (non-normative)

```rust
// Illustrative — do not treat as shipping API.
pub struct Gate { /* runtime-held capability material; never logged */ }

impl Gate {
    pub fn open(
        &self,
        visual: &ProtectedVisual,
        capability: &Capability,
    ) -> Result<SemanticView, DoldError> {
        // AccessDenied must not include plaintext or key material.
        todo!("Phase V: AEAD decrypt + policy apply")
    }
}
```

Rules:

- `Err(DoldError::AccessDenied)` — **no** fallback to open plaintext decode
- Capability checks before decrypt where possible; always before exporting plaintext
- Semantic views may be restricted (`summarize`, `extract_field`, …) without granting `dsk.capability.export.plaintext`

## Capability examples (illustrative)

| Id | Intent |
|----|--------|
| `dsk.capability.read.task` | Read task-shaped fields for agent execution |
| `dsk.capability.export.plaintext` | Full semantic export (highest privilege) |
| `dsk.capability.summarize` | Condensed view only |
| `dsk.capability.classify` | Labels / routing only |

Where K lives (OS keystore, TPM/enclave, remote auth, ephemeral session, KMS) is an integration choice. The font and public glyph map are never K.

## Honest limits

Authorized Gate owners can still extract content by instrumenting the runtime or capturing permitted outputs. Protected mode defends against “RE the font / scan the glyphs / read the repo” — not against a compromised trusted environment.
