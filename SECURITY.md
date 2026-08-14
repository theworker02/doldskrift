# Security Model

## Plain statement

**Open Doldskrift (DSK/1–2, demos) is not encryption.**

Visual obfuscation — abstract glyphs, PUA symbols, session permutations of **plaintext** — is a **representation** choice. It is **not** confidentiality. A determined party with DOM access, clipboard access, network capture of cleartext `.dsk` payloads, or knowledge of the public mapping algorithms can recover semantic text.

**Neural (DSK/3 Neural) is not encryption.** A learned visual grammar and compatible reader may make casual decoding harder and must fail closed on unknown epochs — that is still representation. See [ADR-0007](docs/adr/ADR-0007-dsk3-neural-learned-machine-grammar.md).

**Protected mode (DSK/3 Protected, Phase V sibling) is encryption** — authenticated encryption (AEAD) of a semantic payload, then visual encoding of the **ciphertext**. Font reverse-engineering and glyph scanning recover opaque bytes only. Semantic reconstruction requires an authorized **Gate** holding a capability. See [ADR-0006](docs/adr/ADR-0006-gate-protected-visual-objects.md). Protected mode is **not shipping yet**; until it does, treat current artifacts as Open (or Neural foundation stubs without crypto).

```text
# Open mode (shipping)
plaintext → Doldskrift representation → glyphs

# Neural mode (foundation / research)
semantic → LSG → MGE/5 → glyphs → DVE/2 → compatible reader

# Protected mode (Phase V sibling target)
plaintext → semantic serialize → AEAD → Doldskrift representation of ciphertext → glyphs
```

If you need confidentiality **today**, authenticate and encrypt with established cryptography first, then optionally represent the ciphertext with Open-mode Doldskrift — or wait for Gate / Protected objects.

Phase II / Open-mode wording (normative for **open** product claims):

```text
visual obfuscation  ≠  confidentiality
CRC32C              ≠  authenticity (not a MAC)
session seed        ≠  cryptographic key
MGE/2 / GRE/1       ≠  steganographic security
handshake (DSK?/!)  ≠  secure channel negotiation
```

Protected-mode wording (normative for **DSK/3** claims, when implemented):

```text
glyphs encode ciphertext, not plaintext
font RE / glyph scan  ≠  semantic decode
public spec/source    ≠  open without capability K
authorized Gate owner ≠  immune to instrumentation / screenshots
no claim of “AI-only / human-impossible” secrecy
```

## Open mode is NOT

- encryption
- authentication
- authorization
- DRM
- a secure channel
- a replacement for TLS
- a secret-storage system
- a way to hide legally required accessibility information
- proof that “nobody can read this on screen”

## Open mode IS

- a representation system
- a machine-oriented encoding (DSK/1)
- a typography / glyph-engine experiment (MGE/2)
- an agent interoperability protocol
- a visual-obfuscation mechanism that can frustrate **casual** human reading

## Protected mode (target) IS

- AEAD-backed confidentiality and integrity for the semantic payload
- visual encoding of ciphertext (machine glyphs)
- Gate-mediated, capability-scoped disclosure to authorized runtimes
- explicitly named in `inspect` / product UX (`Payload: Protected`, `Readable: No`)

Protected mode is **still not** DRM against a compromised authorized environment, and it is **not** a mathematical “machines only” property.

## Threat model

### Open mode — can frustrate

- casual visual inspection of rendered glyphs
- shoulder surfing of on-screen agent text
- immediate human comprehension without tooling

### Open mode — cannot inherently defeat

- DOM inspection (encoded PUA text is still in the tree)
- copy/paste of underlying or encoded text
- source inspection and public specs
- reverse engineering of `static-v1` or seeded Fisher–Yates mappings
- instrumented browsers and debugger access
- a determined analyst with `dold decode` or any conforming SDK
- vision/OCR research aimed at MGE/2 (the engine is designed to be machine-readable)

Encoded and session modes may slow casual reading; they **do not** create a security boundary.

### Protected mode — intended boundary (when implemented)

- RE of fonts / glyph catalogs → ciphertext bytes only
- `scan` / vision without capability → ciphertext + metadata; no semantics
- Public DSK/3 spec without K → cannot open a specific Protected object

### Protected mode — still out of scope

- Compromised Gate, leaked capability, or operator screenshots of permitted outputs
- Side channels in the trusted runtime
- Claims that humans physically cannot copy pixels an authorized agent already rendered as plaintext

## Checksums

CRC32C (Castagnoli) over `header_json ∥ payload` detects **accidental corruption**. It is not a MAC, signature, or authenticity proof. An attacker who can modify bytes can recompute the checksum. Protected mode’s AEAD tag is the authenticity mechanism for sealed payloads (separate from CRC32C).

## Mapping seeds

Session seeds select deterministic permutations for representation.
In **Open** mode, treat them as protocol parameters, not cryptographic keys. Publishing `seed_hex` in a `.dsk` manifest is intentional for recoverability — not a secret leak.

In **Protected** mode, session transformation applies to **ciphertext** bytes; capability **K** remains separate and must not be published in manifests.

## Containers and transport

- MIME `application/vnd.doldskrift` / `text/doldskrift` are experimental labels, not security controls.
- `dold inspect` does not reveal decoded payload unless `--reveal` is explicitly passed — for Open mode this is a UX footgun guard, not access control. For Protected mode, reveal of plaintext must go through Gate / `dold open` with a capability.
- Capability negotiation (`DSK?` / `DSK!`) selects representation compatibility only (Open handshake). Protected authorization is a separate Gate concern.

## Reporting vulnerabilities

In-scope examples:

- Panic, memory unsafety, or undefined behavior on malformed input
- Checksum verification bypass for **corruption detection** (logic bugs)
- Incorrect decode that silently alters semantic text
- Spec/implementation divergence that breaks interoperability in unsafe ways
- (Future) AEAD / Gate bugs that leak plaintext or key material without a valid capability

Out of scope (please do not file):

- “Open-mode Doldskrift can be decoded”
- “Session mode is not encryption”
- “Glyphs can be recognized by machines”
- “Someone with the open-mode seed can read the message”
- “An authorized Gate can export plaintext” (by design for high-privilege capabilities)

Report implementation bugs via GitHub security advisories or a private channel designated by maintainers.
