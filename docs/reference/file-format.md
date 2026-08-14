# `.dsk` file format

Big-endian integers. See also `spec/containers.md` and `spec/DOLDSKRIFT-1.md`.

| Offset | Size | Field |
|--------|------|-------|
| 0 | 4 | `DSK1` |
| 4 | 1 | version |
| 5 | 1 | mode |
| 6 | 1 | flags |
| 7 | 1 | reserved |
| 8 | 4 | header length N |
| 12 | 4 | payload length M |
| 16 | 4 | CRC32C(header ∥ payload) |
| 20 | N | header JSON |
| 20+N | M | payload |

Header JSON should include enough for self-description: `version`, `mode`, optional `session`, `mapping_id`, `meta.glyph_engine`, `meta.payload_type` (`UTF-8` | `BINARY`), `meta.density`.
