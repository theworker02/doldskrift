# DOLDSKRIFT/1

**Status:** Stable protocol requirements for version 1  
**MIME (experimental):** `application/vnd.doldskrift`, `text/doldskrift`  
**Companion engines (non-breaking):** MGE/2 (glyphs), GRE/1 (redundancy), DSKG-1 (fingerprints)

## 1. Introduction

Doldskrift is a machine-native typography, encoding, and agent-communication representation.

Doldskrift is **NOT** encryption, authentication, authorization, DRM, or a secure channel.
It MUST NOT be marketed as providing confidentiality against determined inspection.
**Visual obfuscation is not confidentiality.**

Normative keywords follow [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119):
**MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, **MAY**.

When this document and `spec/vectors/*.json` disagree, **vectors are authoritative** until this text is corrected.

## 2. Terminology

| Term | Meaning |
|------|---------|
| Semantic text | The original UTF-8 textual information |
| Symbol | A codepoint in the DOLDSKRIFT/1 alphabet (`U+E000`…`U+E0FF`) |
| Mapping | A permutation of byte→symbol offsets |
| Mode | `visual` (0), `encoded` (1), or `session` (2) |
| Container | A `.dsk` binary document |
| Manifest | Machine-readable session metadata |
| Header JSON | UTF-8 JSON embedded after the 20-byte fixed preamble |

## 3. Versioning

The protocol family name is `doldskrift`.
This document specifies major version **1** (`DSK/1`).

Parsers MUST reject unsupported major versions with a typed error and MUST NOT panic.
Future versions (`DSK/2`, …) MUST remain distinguishable by the version byte / field.

## 4. Operating Modes

### 4.1 Visual

Underlying Unicode MUST remain unchanged.
A Doldskrift font MAY substitute abstract glyphs for human display.
Software reading DOM text, accessibility trees, clipboard text, or APIs MUST still observe the semantic characters.
No decoder is required.

### 4.2 Encoded

Semantic UTF-8 bytes MUST be transformed into Doldskrift symbols using the versioned static mapping (`static-v1`).
Decoding MUST be lossless for any valid UTF-8 input.

### 4.3 Session

A session MUST produce a deterministic mapping from a seed.
The session MUST publish a manifest describing the mapping.
Session mappings provide representation/obfuscation only — not cryptographic secrecy.

## 5. Character Representation

### 5.1 Decision: UTF-8 bytes

Encoded and session modes operate on **UTF-8 bytes**, not Unicode scalar values.

**Rationale:**
- Fixed 256-symbol alphabet fits cleanly in the BMP Private Use Area.
- Emoji, CJK, combining marks, and arbitrary scripts round-trip losslessly.
- Streaming is byte-oriented and allocation-friendly.

**Tradeoff:** Glyph count follows UTF-8 length, not perceived character count.

### 5.2 Alphabet

| Property | Value |
|----------|-------|
| Size | 256 |
| Base | `U+E000` (`PUA_BASE`) |
| Symbol for offset `o` | `U+E000 + o` |

Identity mapping (`static-v1`): byte `b` → `U+E000 + b`.

### 5.3 UTF-8 handling (normative)

1. **Encode input:** Implementations MUST accept a sequence of Unicode scalar values already encoded as UTF-8, or a language string type that is valid UTF-8. Encoding walks the **raw UTF-8 bytes** in order.
2. **Encode output (textual):** Each mapped symbol is a BMP PUA scalar. When serialized as a Unicode string / UTF-8 text, each symbol occupies **three UTF-8 bytes** (`EE 80 80` … pattern for the PUA range).
3. **Decode input:** Implementations MUST iterate Unicode scalars (not UTF-8 bytes of the encoded form). Each scalar MUST be in `U+E000`…`U+E0FF` or the decoder MUST return `InvalidCodepoint`.
4. **Decode output:** Collected bytes MUST form valid UTF-8 or the decoder MUST return `InvalidUtf8` with the byte offset of failure (`valid_up_to` semantics).
5. **Empty string:** Encodes to empty payload; round-trip MUST hold.
6. **Streaming:** Encoders MAY emit symbols per complete input byte. Decoders MUST buffer decoded bytes until a valid UTF-8 boundary can be flushed (or report `InvalidUtf8` at finish if trailing bytes remain incomplete). Invariant: `stream_decode(stream_encode(x)) == x` for valid UTF-8 `x`.

### 5.4 Control symbols and project mark (outside alphabet)

These codepoints are **not** part of the 256-byte mapping alphabet:

| Name | Codepoint | Notes |
|------|-----------|-------|
| `DSK_START` | `U+E100` | Control base + 0 |
| `DSK_END` | `U+E101` | |
| `DSK_TOKEN` | `U+E102` | |
| `DSK_BREAK` | `U+E103` | |
| `DSK_META` | `U+E104` | |
| `DSK_ESCAPE` | `U+E105` | |
| `DSK_PROJECT_MARK` | `U+E1F0` | Official logo; valid MGE/2 glyph |

Control range check used by implementations: `U+E100` ≤ cp < `U+E100 + 16`.
Payload alphabet decode MUST reject these as `InvalidCodepoint` when expecting data symbols.

## 6. Mapping Generation

### 6.1 Identity (`static-v1`)

```text
for b in 0..256:
  encode_table[b] = b
  decode_table[b] = b
mapping_id = "static-v1"   # PROTOCOL_VERSION = 1
```

### 6.2 Session permutation (normative algorithm)

Implementations MUST match this algorithm bit-for-bit with the reference Rust (`doldskrift_codec::mapping`) and JavaScript SDK.

#### SplitMix64

Given state `x: u64`:

```text
x = x + 0x9E3779B97F4A7C15          (wrapping)
z = x
z = (z XOR (z >> 30)) * 0xBF58476D1CE4E5B9   (wrapping)
z = (z XOR (z >> 27)) * 0x94D049BB133111EB   (wrapping)
return z XOR (z >> 31)
```

#### `mix_seed(seed: bytes) -> u64`

```text
state = 0x243F6A8885A308D3          # fractional π bits
if seed is empty:
  return splitmix64(state)

for each 8-byte chunk of seed (final chunk zero-padded on the right):
  word = u64 from chunk as LITTLE-ENDIAN
  state = state XOR word
  state = splitmix64(state)
return state
```

#### Fisher–Yates

```text
encode_table = [0, 1, 2, …, 255]
state = mix_seed(seed)
for i from 255 down to 1:
  state = splitmix64(state)
  j = (state as usize) % (i + 1)    # use the full u64 value, then modulo
  swap encode_table[i], encode_table[j]
```

Build `decode_table` as the inverse permutation. If the table is not a bijection, fail with `MappingCollision` / `InvalidMapping`.

#### Mapping id

```text
c = CRC32C(seed_bytes)             # Castagnoli
mapping_id = format("session-v1-{c:08x}")   # lowercase hex, 8 digits
```

### 6.3 Encode / decode with a mapping

```text
encode_byte(b) = char(U+E000 + encode_table[b])
decode_char(ch) = decode_table[ch - U+E000]   # if ch out of alphabet → InvalidCodepoint
```

## 7. Framing and Checksums

Logical frame (informational):

```text
MAGIC | VERSION | MODE | LENGTH | PAYLOAD | CHECKSUM
```

Concrete containers use the layout in §8.

### 7.1 Checksum algorithm

- Name: **CRC32C** (Castagnoli), algorithm id string `crc32c`.
- Known vector: CRC32C(`123456789`) = `0xE3069283`.
- CRC32C of empty input = `0`.

Checksums detect accidental corruption. They MUST NOT be treated as authenticity proofs.

### 7.2 Coverage for `.dsk`

```text
checksum = CRC32C( header_json_bytes ∥ payload_bytes )
```

Stored as a **big-endian** `u32` at offset 16 of the file.
The checksum field inside header JSON (if present when parsed) is **not** included in a second hash of itself: serializers MUST zero `checksum` in the JSON object before producing `header_json_bytes` used for coverage and for writing.

Malformed input MUST produce typed errors (§12).

## 8. Containers (`.dsk`)

All multi-byte integers are **big-endian**.

### 8.1 Binary layout

| Offset | Size | Field | Notes |
|--------|------|-------|-------|
| 0 | 4 | Magic | ASCII `DSK1` (`44 53 4B 31`) |
| 4 | 1 | Version | `1` for DSK/1 |
| 5 | 1 | Mode | `0` visual, `1` encoded, `2` session |
| 6 | 1 | Flags | bit0 `COMPRESSED` (reserved), bit1 `ECC` (reserved) |
| 7 | 1 | Reserved | MUST be `0` on write; parsers MAY ignore |
| 8 | 4 | Header length `N` | BE `u32` |
| 12 | 4 | Payload length `M` | BE `u32` |
| 16 | 4 | CRC32C | BE `u32`, see §7.2 |
| 20 | N | Header JSON | UTF-8 JSON object |
| 20+N | M | Payload | mode-dependent bytes |

Total size MUST equal `20 + N + M`. Trailing bytes MUST cause `InvalidLength`. Truncation MUST cause `TruncatedPayload`.

MIME type (project-defined / experimental): `application/vnd.doldskrift`.

### 8.2 Header JSON

Canonical fields (serde-compatible with the reference implementation):

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `version` | u8 | yes | Protocol major |
| `mode` | string | yes | `visual` / `encoded` / `session` |
| `flags` | u8 | no | default 0 |
| `checksum` | u32 | no | ignored for coverage; set 0 when serializing for hash |
| `mapping_id` | string | no | e.g. `static-v1` or `session-v1-…` |
| `session` | object | session mode | [`SessionManifest`](#83-session-manifest) |
| `char_count` | u64 | no | semantic Unicode scalar count |
| `byte_count` | u64 | no | semantic UTF-8 byte length |
| `font_version` | u32 | no | defaults to implementation font version (MGE/2 → 2) |
| `meta` | object | no | extension map; unknown keys MUST be preserved by rewriters that claim transparency, and MAY be ignored by readers |

### 8.3 Session manifest

```json
{
  "protocol": "doldskrift",
  "version": 1,
  "mode": "session",
  "session_id": "...",
  "mapping_id": "...",
  "alphabet": "pua-byte-256",
  "checksum": "...",
  "seed_hex": "..."
}
```

- `protocol` MUST be `doldskrift`.
- `version` MUST be `1` for DSK/1.
- `mode` MUST be `session`.
- `checksum` is hex-encoded CRC32C of the **encode table bytes** (256 raw offset bytes), not of the seed.
- `seed_hex` is optional on the wire but REQUIRED for reference recovery of the mapping from the container alone.
- Seed material is representation state, not a cryptographic secret.

Mapping generation MUST be deterministic for equal `(version, seed)` per §6.2.

### 8.4 Payload by mode

| Mode | Payload contents |
|------|------------------|
| visual | Raw UTF-8 bytes of semantic text |
| encoded | UTF-8 encoding of the PUA symbol string (`static-v1`) |
| session | UTF-8 encoding of the PUA symbol string (session mapping) |

### 8.5 Inspect behavior

`dold inspect` and equivalent APIs MUST NOT reveal decoded semantic payload unless explicitly requested (e.g. `--reveal`). Metadata (mode, lengths, mapping id, checksum validity) MAY be shown.

## 9. Streaming

Implementations SHOULD provide streaming encoder/decoder interfaces so agents need not load entire documents into memory.
`stream_decode(stream_encode(x))` MUST equal `x` for valid UTF-8 `x`.

Streaming applies to the **symbol string** path. Container framing MAY still be buffered as a whole document in v1.

## 10. Discovery

HTML MAY advertise Doldskrift:

```html
<meta name="doldskrift" content="version=1;mode=encoded">
<div data-doldskrift="1" data-doldskrift-mode="encoded">...</div>
```

Programmatic detection SHOULD return structured information (`detected`, `version`, `mode`, `source`, `raw`), not only a boolean.
Heuristic PUA density detection MAY be offered; it MUST NOT false-trigger on empty strings.

## 11. Capability Negotiation

Optional handshake:

```text
DSK?
versions=1
modes=visual,encoded,session
capabilities=DSK_VISUAL,DSK_ENCODED,DSK_SESSION,DSK_STREAM,DSK_CONTAINER

DSK!
version=1
mode=session
```

Capabilities: `DSK_VISUAL`, `DSK_ENCODED`, `DSK_SESSION`, `DSK_STREAM`, `DSK_CONTAINER`, `DSK_VISION`.

Preference order when multiple modes match: **session > encoded > visual**.

Negotiation selects representation compatibility only. It MUST NOT be construed as a secure channel.

## 12. Error Handling

Implementations MUST use typed errors such as:

| Error | When |
|-------|------|
| `InvalidHeader` | Magic ≠ `DSK1` |
| `UnsupportedVersion` | Version byte not supported |
| `UnsupportedMode` | Mode byte unknown |
| `InvalidCodepoint` | Symbol outside alphabet when decoding payload |
| `InvalidMapping` | Incomplete / inconsistent mapping |
| `MappingCollision` | Non-bijective encode table |
| `ChecksumMismatch` | CRC32C mismatch (`expected`, `actual`) |
| `TruncatedPayload` | Fewer bytes than `20+N+M` |
| `InvalidLength` | Overflow, trailing junk, inconsistent lengths |
| `InvalidUtf8` | Decoded bytes not UTF-8 |
| `InvalidManifest` | Header/session JSON invalid |
| `NegotiationFailed` | No mutually supported mode |
| `FontGenerationError` | Glyph engine failure |
| `VisionError` | Experimental vision pipeline failure |
| `IoError` | OS I/O |

Arbitrary bytes MAY produce an error. They MUST NOT cause undefined behavior or uncontrolled panic.

Fail-closed: unknown modes/versions MUST error; MUST NOT guess.

## 13. Forward Compatibility

- Unknown modes/versions MUST fail closed with typed errors.
- Flag bits `COMPRESSED` and `ECC` are reserved for future versions; DSK/1 writers MUST set them to 0 unless a future DEP activates them.
- Header `meta` extension fields: readers MUST ignore unknown keys; they MUST NOT reject a document solely for unknown `meta` entries.
- Experimental extensions (vision, machine-optimized alphabet, ECC, MGE/2 catalogs) MUST NOT break DSK/1 parsers when absent.

## 14. Experimental Extensions

The following are **experimental** and not required for DSK/1 text/container conformance:

- Pixel/vision recognition pipeline (`DSK_VISION`)
- Machine-optimized visual bit alphabets
- Error-correcting codes for lossy capture (`ECC` flag)
- Human-readability / machine-separability metrics used during font generation
- Control-symbol semantic channel (`U+E100+`)
- GRE/1 density redundancy and MGE/2 fingerprints (required for **font** Phase II tooling, not for byte codec conformance)

See [`deps/DEP-0002-machine-glyph-engine.md`](../deps/DEP-0002-machine-glyph-engine.md) and [`deps/DEP-0003-glyph-redundancy.md`](../deps/DEP-0003-glyph-redundancy.md).
