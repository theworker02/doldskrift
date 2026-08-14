# Containers (`.dsk`)

## Layout

```text
Offset  Size  Field
0       4     Magic "DSK1"
4       1     Version (=1)
5       1     Mode
6       1     Flags
7       1     Reserved (0)
8       4     Header length N (BE)
12      4     Payload length M (BE)
16      4     CRC32C(header∥payload) (BE)
20      N     Header JSON (UTF-8)
20+N    M     Payload
```

Trailing junk → `InvalidLength`. Short files → `TruncatedPayload`. Bad magic → `InvalidHeader`.

## Flags

| Bit | Name | DSK/1 |
|-----|------|-------|
| 0 | `COMPRESSED` | Reserved; write 0 |
| 1 | `ECC` | Reserved; write 0 |

## Header JSON

Includes `version`, `mode`, `flags`, optional `mapping_id`, `session`, `char_count`, `byte_count`, `font_version`, `meta`.

Session mode SHOULD embed a full `SessionManifest` with `seed_hex` for recoverability in the reference stack.

## Payload

| Mode | Bytes |
|------|-------|
| visual | UTF-8 semantic text |
| encoded / session | UTF-8 encoding of PUA symbol string |

## APIs

- Rust: `DskDocument::{encode_text, encode_session, parse, to_bytes, decode_text, inspect, verify}`
- JS: `DskDocument` in `@doldskrift/core`
- CLI: `dold encode|decode|inspect|verify`

## Inspect policy

Inspect metadata freely; decode only on explicit request. This prevents accidental shoulder-surfing of payloads in logs — it is **not** an access-control mechanism.
