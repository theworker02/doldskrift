# Rust guide

## Add the crate

From this workspace:

```toml
[dependencies]
doldskrift = { path = "crates/doldskrift", features = ["font"] } # font optional
```

The facade re-exports codec, document, protocol, and (with `font`) glyph APIs.

## Encode / decode

```rust
use doldskrift::{Decoder, Encoder, Mode};

let enc = Encoder::new(Mode::Encoded);
let payload = enc.encode("Hello, agent!")?;
let text = Decoder::new().decode(&payload)?;
```

Helpers `encode` / `decode` use identity mapping directly.

## Sessions

```rust
use doldskrift::Session;

let session = Session::builder()
    .seed(b"demo")
    .session_id("run-42")
    .build()?;
let opaque = session.encode("status=ok")?;
let clear = session.decode(&opaque)?;
```

## Containers

```rust
use doldskrift::{DskDocument, Mode};

let doc = DskDocument::encode_text("ping", Mode::Encoded)?;
let bytes = doc.to_bytes()?;
let parsed = DskDocument::parse(&bytes)?;
assert!(parsed.inspect().checksum_valid);
assert_eq!(parsed.decode_text()?, "ping");
```

Session documents: `DskDocument::encode_session`.

## Streaming

```rust
use doldskrift::{StreamingDecoder, StreamingEncoder};

let mut enc = StreamingEncoder::new();
let mut out = String::new();
out.push_str(&enc.push_str("Hello, "));
out.push_str(&enc.push_str("world"));
out.push_str(&enc.finish()?);

let mut dec = StreamingDecoder::new();
let mut clear = String::new();
clear.push_str(&dec.push_str(&out)?);
clear.push_str(&dec.finish()?);
```

## Discovery & handshake

```rust
use doldskrift::{
    detect_from_html_meta, negotiate, CapabilitySet, HandshakeOffer,
};

let det = detect_from_html_meta("version=1;mode=encoded");
assert!(det.detected);

let offer = HandshakeOffer::reference();
let reply = negotiate(&offer, &CapabilitySet::reference())?;
```

## Fonts (feature `font`)

```rust
use doldskrift::{generate_glyph, PUA_BASE, FONT_VERSION};

let g = generate_glyph(PUA_BASE + 65, FONT_VERSION, 1)?;
println!("{}", g.fingerprint());
```

## Errors

All fallible APIs return `doldskrift::Result<T>` with typed `Error` variants — see [errors](../reference/errors.md).

## Examples in-repo

- `examples/rust/roundtrip.rs`
- `crates/doldskrift/examples/gen_vectors.rs`
- `cargo test -p doldskrift`
- `cargo bench -p doldskrift`
