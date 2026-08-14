# Encoding Notes (DOLDSKRIFT/1)

## Unit of encoding

**UTF-8 bytes.**

```text
semantic UTF-8 bytes
    → map each byte through Mapping[256]
    → emit U+E000 + offset as Unicode scalars
    → serialize those scalars as UTF-8 for textual transport
```

## Why not scalar values?

Mapping every Unicode scalar would require either:

1. A huge alphabet, or
2. Escape sequences for out-of-range characters

Byte encoding keeps a 256-symbol machine alphabet, preserves all Unicode, and streams cleanly.

## Identity mapping (`static-v1`)

```text
byte 0x48 ('H') → U+E048
byte 0x65 ('e') → U+E065
```

## Session mapping

1. Mix seed bytes with SplitMix64 (`mix_seed`)
2. Fisher–Yates shuffle of `[0, 255]`
3. Resulting permutation is the encode table

Identical seeds MUST produce identical tables across Rust and JavaScript.

## Round-trip invariant

```text
decode(encode(x)) == x
```

for every valid UTF-8 string `x`.
