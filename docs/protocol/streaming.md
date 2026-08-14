# Streaming

Streaming APIs encode/decode **symbol strings** incrementally so agents can process large messages without holding entire documents.

## Invariant

```text
stream_decode(stream_encode(x)) == x
```

for every valid UTF-8 string `x`.

## Encoder

- Input: UTF-8 byte chunks (may split mid-codepoint if you push raw bytes carefully; `push_str` always passes complete scalars’ bytes).
- Output: PUA characters for each consumed byte via the Mapping.
- Mapping: identity by default; session mapping via `with_mapping`.

## Decoder

- Input: encoded PUA text chunks.
- Output: UTF-8 byte runs flushed on valid UTF-8 boundaries.
- Incomplete trailing bytes at `finish` → `InvalidUtf8` (or equivalent).

## Containers

`.dsk` parse/serialize in v1 is document-oriented (full buffer). Stream the **payload symbol text** inside your own framing if you need chunked network send, or send successive `.dsk` messages.

## APIs

| Language | Types |
|----------|-------|
| Rust | `StreamingEncoder`, `StreamingDecoder` |
| JS | `StreamingEncoder`, `StreamingDecoder` |

## Example (Rust)

```rust
use doldskrift::StreamingEncoder;

let mut enc = StreamingEncoder::new();
let a = enc.push_str("partial ");
let b = enc.push_str("message");
let c = enc.finish()?;
let opaque = format!("{a}{b}{c}");
```

Capability flag: `DSK_STREAM`.
