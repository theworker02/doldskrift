# Framing

DSK/1 uses explicit lengths and a Castagnoli CRC so agents can reject truncated or corrupted messages early.

## Logical frame

```text
MAGIC | VERSION | MODE | LENGTHS | CHECKSUM | HEADER | PAYLOAD
```

On the wire for `.dsk`, this is the 20-byte fixed preamble plus JSON header plus payload (see [containers](containers.md)).

## Integers

All multi-byte integers in the container preamble are **big-endian** `u32`:

- header length `N`
- payload length `M`
- CRC32C value

## Checksum

```text
CRC32C( header_json_bytes || payload_bytes )
```

- Algorithm id: `crc32c`
- Empty input CRC: `0`
- Vector: CRC32C(`123456789`) = `0xE3069283`
- Header JSON serialization MUST store `checksum: 0` when producing bytes covered by the CRC

CRC detects accidents. It is not a MAC.

## Mode byte

| Value | Mode |
|------:|------|
| 0 | visual |
| 1 | encoded |
| 2 | session |

Unknown → `UnsupportedMode`.

## Streaming vs document framing

Symbol streaming (PUA text chunks) has no CRC of its own — integrity is the transport’s job, or wrap chunks in `.dsk`. Document framing is all-or-nothing for v1: parsers require exact `20+N+M` bytes.
