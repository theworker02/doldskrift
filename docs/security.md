# Security (summary)

**Open Doldskrift is not encryption.** Visual obfuscation ≠ confidentiality.

**Neural (DSK/3 Neural)** is also **not encryption** — learned representation + compatible reader. See [Neural security](neural/security.md) and [ADR-0007](adr/ADR-0007-dsk3-neural-learned-machine-grammar.md).

**Protected mode (DSK/3 Protected, Phase V sibling — not shipping)** *is* AEAD encryption of semantics, with glyphs encoding ciphertext. See [ADR-0006](adr/ADR-0006-gate-protected-visual-objects.md) and [Protected mode](product/protected-mode.md).

Full threat model, checksum limits, seed handling, and vulnerability reporting live in the repository root:

→ **[`SECURITY.md`](../SECURITY.md)**

### Quick facts

| Mechanism | Provides | Does not provide |
|-----------|----------|------------------|
| Open visual / encoded / session display | Harder casual reading | Secrecy against DOM or tooling |
| CRC32C on `.dsk` | Corruption detection | Authenticity / integrity against attackers |
| Open session seed | Deterministic permutation | Cryptographic key material |
| `DSK?` / `DSK!` | Mode compatibility | Secure channel / Gate auth |
| MGE/2 + GRE/1 | Machine-readable glyphs | Steganographic security |
| Protected AEAD + Gate (target) | Confidentiality + authz for semantics | Immunity if the Gate / capability is compromised |
| Neural reader compatibility | Fail-closed reconstruction | Confidentiality / “AI-only” secrecy |

**Today:** encrypt first (TLS / AEAD), then optionally wrap or display with Open-mode Doldskrift.  
**Phase V:** use named Protected objects + `dold open` when Gate ships.
