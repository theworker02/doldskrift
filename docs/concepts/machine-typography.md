# Machine typography

Traditional typography assumes a human reader at the end of the pipeline:

```text
characters → glyphs → human vision → language understanding
```

Doldskrift inverts the priority. Semantic information is for **software agents**; human vision is an optional, often frustrated, side channel.

```text
semantic UTF-8
    → tokenizer (bytes)
    → symbol encoder (Mapping)
    → container and/or glyph engine
    → agent decoder → semantic UTF-8
```

## Three modes

### Visual

Unicode codepoints stay as authored. A Doldskrift font remaps their **appearance** to abstract MGE/2 forms. Clipboard, DOM, and AT still see “Hello”.

Use when you want strange specimens without changing interoperability with ordinary text tools.

### Encoded

Each UTF-8 **byte** becomes a Private Use Area symbol via identity mapping `static-v1`:

```text
0x48 'H' → U+E048
```

Humans see abstract glyphs (with the font) or raw PUA (without). Machines decode losslessly without a session seed.

### Session

Same byte→symbol pipeline, but the offset table is a seeded permutation (SplitMix64 + Fisher–Yates). A manifest documents `mapping_id`, optional `seed_hex`, and table checksum. Obfuscation against casual reading improves slightly; **secrecy does not**.

## Design tensions

| Tension | Resolution in DSK/1 + MGE/2 |
|---------|------------------------------|
| Human familiarity vs machine separability | Prefer low Latin resemblance; enforce `MINIMUM_DISTANCE` |
| Streamability vs grapheme aesthetics | Encode bytes, not grapheme clusters |
| Specimens vs accessibility | Visual mode + `aria-label` on encoded UI |
| Research vision vs protocol stability | Vision is experimental; containers stay DSK/1 |

## What “machine-native” means here

1. **Determinism** — same seed and version → same bytes and glyphs everywhere.
2. **Typed failure** — garbage in → `Error`, never UB.
3. **Discoverability** — HTML meta, data attributes, handshakes.
4. **Structural glyphs** — zones and fingerprints, not merely pretty vectors.
5. **Honest security story** — representation ≠ encryption.

## Related

- [Glyphs](glyphs.md) · [Symbols](symbols.md) · [Mappings](mappings.md) · [Sessions](sessions.md)
- Research: [machine-readable writing](../research/machine-readable-writing.md)
