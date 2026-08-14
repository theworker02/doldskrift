# ADR-0006: Gate and Protected Visual Objects (Phase V sibling profile)

## Status

Proposed (design target). **Milestone 1–2 stubs landed:** Protected container metadata / `inspect` (`Readable: No`), `dold decode` refuse path, stub `dold open` → `AccessDenied`. **No AEAD / real Gate crypto yet.** Phase V’s research centerpiece is **Neural** ([ADR-0007](ADR-0007-dsk3-neural-learned-machine-grammar.md)); Protected remains the encryption sibling profile.

## Context

Through DSK/1–2 and MGE/2–4 research, Doldskrift’s open visual codec remains an honest **representation** stack: glyphs encode symbols that decode to plaintext (or session-permuted plaintext). Product and security docs correctly state that this is **not encryption**.

That honesty creates a gap. Operators want a first-class product surface where:

1. Visible glyphs encode **ciphertext**, not semantic plaintext.
2. Font reverse-engineering, glyph scanning, and knowing the public spec yield opaque bytes only.
3. Semantic recovery requires an **authorized runtime** holding a capability — not a cleverer decoder.
4. Open/demo modes stay available and remain non-crypto encodings.

Prior roadmap language (“live visual state / runtime”) was too vague. Phase V needs a named architecture: **Protected Visual Objects** behind a **Doldskrift Gate**, with a clear CLI split (`scan` / `inspect` vs `open`) and capability-scoped agent ops.

**Reconciliation (2026-08):** Phase V also introduces **DSK/3 Neural** (learned LSG + trained reader). Neural does **not** replace Protected. Profiles: Open | Neural | Protected. Neural ≠ encryption; Protected is the encryption track.

## Decision

### 1. Two product modes

| Mode | What glyphs encode | Decode without auth |
|------|--------------------|---------------------|
| **Open** (DSK/1–2, demos) | Symbols → plaintext (or session permutation of plaintext) | Full semantic recovery |
| **Protected** (DSK/3 target) | Symbols → ciphertext | Stop at ciphertext; refuse semantic reconstruction |

Open mode keeps the historical claim: *Doldskrift (open codec) is not encryption.*  
Protected mode **is** authenticated encryption plus visual encoding of the ciphertext. It must be named explicitly in UX, inspect metadata, and docs.

### 2. Protocol / visual versioning

| Layer | Version | Role |
|-------|---------|------|
| Protocol | **DSK/3 Protected** (proposed) | Protected payload framing: AEAD ciphertext, integrity, session/policy hints |
| Visual | **MGE/5** (shared Phase V visual target; also used by Neural) | Contextual variants, compound grouping, layout suited to machine objects |
| Recognition | DVE (continuing) | Glyph/symbol recovery only — does not decrypt |

DSK/1 remains the open codec baseline. DSK/2 remains experimental semantics for open payloads. **DSK/3 Protected** MUST be distinguishable from **DSK/3 Neural** and from Open on the wire and in `inspect` output. Fail closed on unknown protected versions.

### 3. Encode / decode pipelines (target design)

**Encode (Protected):**

```text
plaintext
  → semantic serialization
  → compression (optional)
  → AEAD (authenticated encryption)
  → session transformation (symbol permutation of ciphertext bytes)
  → machine glyph encoding
  → Doldskrift visual output
```

**Decode with authorization:**

```text
glyphs → symbols → ciphertext
  → authorized agent / Gate runtime
  → AEAD decrypt
  → semantic reconstruction
  → capability-scoped result
```

**Decode without authorization:**

```text
glyphs → symbols → ciphertext  → STOP
```

Even `dold decode` on a Protected object MUST refuse semantic recovery and explain that the visual payload was reconstructed but semantics require an authorized session. Prefer directing operators to `dold scan` / `dold inspect` for metadata and `dold open` for authorized access.

### 4. CLI separation

| Command | Role |
|---------|------|
| `dold scan` | Recover frame / visual payload; report protected metadata; **Readable: No** for Protected |
| `dold inspect` | Structured metadata (see example fields below); no plaintext |
| `dold open` | Requires authorization / capability; Gate-mediated semantic access |
| `dold decode` | Open-mode path only; on Protected objects → refuse + explain |

Example `inspect` fields for a Protected object:

```text
Protocol: DSK/3
Visual: MGE/5
Payload: Protected
Cipher: AEAD
Session: Required
Agent Policy: Required
Integrity: Valid
Readable: No
```

### 5. Gate and capability model

- **Capability K** is held by a trusted runtime (OS keystore, TPM/enclave, remote auth, ephemeral session, company KMS) — **never** by the font or public glyph map.
- LLMs and untrusted agents **never** receive raw keys. The Gate decrypts internally and returns only **permitted** semantic results.
- Capability-scoped operations (illustrative): `summarize`, `classify`, `execute`, `compare`, `extract_field`.
- Capability ids (illustrative): `dsk.capability.read.task` vs `dsk.capability.export.plaintext`.
- Rust sketch (API shape only; not shipping crypto):

```rust
// Target API — Phase V. Must not silently fall back to open decode.
gate.open(&visual, capability)?; // Err(DoldError::AccessDenied) with no plaintext path
```

There is **no** fallback plaintext path for Protected objects.

### 6. Layer stack (documentation target)

1. Semantic serialization  
2. Compression (optional)  
3. AEAD  
4. Session symbol permutation (of ciphertext)  
5. Contextual glyph variants  
6. Compound glyph grouping  
7. 2D layout  
8. ECC (transport resilience, not secrecy)  
9. Machine visual rendering  

Layers 1–3 establish confidentiality/authenticity for Protected mode. Layers 4–9 are representation and channel robustness. Open mode uses a subset without AEAD.

### 7. Honest security bounds (normative claims)

**Do claim:**

- Reverse-engineering the font ≠ decoding a Protected message.
- Scanning glyphs ≠ semantic decode.
- Knowing the public spec/source ≠ opening a specific Protected object without its capability.

**Do not claim:**

- A mathematical “AI-only / human-impossible” property.
- That authorized runtime owners cannot extract content (instrumentation, screenshots, compromised Gate).
- That open/demo modes are confidential.

## Consequences

- Security and product docs must stop blanket “Doldskrift is not encryption” without the Open vs Protected qualifier.
- Implementation is **docs-first**; AEAD/Gate code lands behind explicit milestones and must not pretend to encrypt until wired to real crypto.
- `scan` / `inspect` / `open` semantics become part of the Lab contract for agents and humans.
- Partners can cite Protected Visual Objects as the encrypted product surface while demos remain honest open encodings.
- Related: [ADR-0004](ADR-0004-dsk2-semantics.md) (open semantics), [ADR-0005](ADR-0005-visual-transport.md) (visual hop ≠ crypto channel for open transport).

## Out of scope (this ADR)

- Shipping AEAD algorithms, KMS integrations, or TPM bindings
- Changing DSK/1 golden vectors or open-mode decode behavior
- Completing Phase IV Lens WASM / pixel-true vision (partially done on site; further hardening tracked on ROADMAP)
