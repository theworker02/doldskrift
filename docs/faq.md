# FAQ

## Is Doldskrift encryption?

**Open mode (shipping):** No. It is a representation and typography system. Anyone with a conforming decoder, the public `static-v1` mapping, or a session seed can recover the text.

**Protected mode (DSK/3, Phase V target):** Yes — AEAD of semantics, glyphs encode ciphertext, Gate + capability required for semantic recovery. Not implemented yet. See [`SECURITY.md`](../SECURITY.md) and [Protected mode](product/protected-mode.md).

## What does “visual obfuscation ≠ confidentiality” mean?

In **Open** mode, rendered glyphs may look unreadable to a casual viewer, but the underlying data (DOM text, `.dsk` payload, or known mapping) still yields the message. Treat it like an odd font / encoding, not a lock. **Protected** mode is a different product surface: scan stops at ciphertext.

## Which mode should I use?

| Goal | Mode |
|------|------|
| Keep selectable/searchable semantic text; weird font only | `visual` |
| Interop with a fixed alphabet; reversible PUA bytes | `encoded` |
| Per-conversation permutation + manifest | `session` |

## Why encode UTF-8 bytes instead of characters?

A 256-symbol alphabet maps cleanly onto `U+E000`–`U+E0FF`, streams easily, and round-trips every script and emoji. Glyph count tracks byte length, not grapheme count.

## What is a `.dsk` file?

A binary container: magic `DSK1`, version/mode/flags, big-endian lengths, CRC32C over header JSON ∥ payload, then header and payload. MIME (experimental): `application/vnd.doldskrift`.

## Why doesn’t `dold inspect` show my message?

By design. Inspect is for metadata. For **Open** documents use `dold decode` or `dold inspect --reveal`. For **Protected** objects (Phase V), plaintext must go through `dold open` with a capability — `--reveal` is not an access-control bypass.

## What is MGE/2?

Machine Glyph Engine v2 — zone-structured procedural glyphs with `DSKG-1` fingerprints and GRE/1 redundancy. See [font architecture](font/architecture.md) and [DEP-0002](../deps/DEP-0002-machine-glyph-engine.md).

## What is `U+E1F0`?

`DSK_PROJECT_MARK` — the official logo glyph. Brand rules: [brand.md](brand.md).

## Do I need the font to decode?

No. Decoding is a text/byte operation. Fonts matter for human display and experimental vision scans.

## Can agents negotiate Doldskrift over the network?

Yes, optionally via `DSK?` / `DSK!` capability messages. That negotiation is **not** TLS. Use a secure transport underneath.

## Which packages exist?

- Rust: `doldskrift` facade + crates under `crates/`
- JS: `@doldskrift/core`, `@doldskrift/react`, `@doldskrift/web`
- CLI: `dold`

## How do I verify my implementation?

```bash
dold conformance --vectors spec/vectors
```

## Where are decisions recorded?

[`deps/`](../deps/) (DEPs) and [`GOVERNANCE.md`](../GOVERNANCE.md).
