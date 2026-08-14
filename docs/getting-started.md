# Getting started

Install the CLI, encode a message, inspect the container, and decode it back. Then try the same round-trip from Rust or JavaScript.

## Prerequisites

- Rust toolchain (stable) with Cargo
- Optional: Node.js 18+ for `@doldskrift/core`

Clone the repo and build from source (recommended while the project is pre-crates.io stable):

```bash
cargo install --path crates/doldskrift-cli
dold --help
```

## Encode and decode

```bash
echo "Hello from Doldskrift" | dold encode > message.dsk
dold inspect message.dsk
dold verify message.dsk
dold decode message.dsk
```

`inspect` shows mode, lengths, and checksum validity **without** printing the semantic payload. Pass `--reveal` only when you intentionally want the decoded text in the inspect output.

### Raw PUA text (no container)

```bash
echo "hello agent" | dold encode --raw
echo "<paste symbols>" | dold decode --seed ""   # encoded mode needs no seed
```

### Session mode

```bash
echo "deploy ready" | dold encode --mode session --seed alpha --session-id demo > session.dsk
dold decode session.dsk
```

The container header carries the session manifest (including `seed_hex` in the reference implementation) so decode can recover the mapping.

## Rust

```rust
use doldskrift::{Decoder, Encoder, Mode, Session};

fn main() -> doldskrift::Result<()> {
    let encoder = Encoder::new(Mode::Encoded);
    let payload = encoder.encode("Hello, agent!")?;
    let message = Decoder::new().decode(&payload)?;
    assert_eq!(message, "Hello, agent!");

    let session = Session::builder().seed(b"demo").session_id("001").build()?;
    let opaque = session.encode("hello")?;
    assert_eq!(session.decode(&opaque)?, "hello");
    Ok(())
}
```

Container helper:

```rust
use doldskrift::{DskDocument, Mode};

let doc = DskDocument::encode_text("ping", Mode::Encoded)?;
std::fs::write("ping.dsk", doc.to_bytes()?)?;
let again = DskDocument::open("ping.dsk")?;
assert_eq!(again.decode_text()?, "ping");
```

## JavaScript

```bash
npm install @doldskrift/core
```

```ts
import { encode, decode, DskDocument } from "@doldskrift/core";

const opaque = encode("hello agent");
console.log(decode(opaque));

const doc = DskDocument.encodeText("hello agent", "encoded");
const bytes = doc.toBytes();
```

React:

```tsx
import { Doldskrift } from "@doldskrift/react";

<Doldskrift mode="encoded">hello agent</Doldskrift>
```

## Fonts (optional)

```bash
dold font build --out fonts/generated
```

Install or `@font-face` the generated family (see [Font installation](guides/font-installation.md)). Visual mode keeps Unicode underneath; encoded mode displays PUA symbols that the font maps to MGE/2 glyphs.

## Conformance

```bash
dold conformance --vectors spec/vectors
```

Every implementation MUST pass these golden vectors.

## Next steps

- Modes explained: [Machine typography](concepts/machine-typography.md)
- Agent handshake: [Agents guide](guides/agents.md)
- Threat model: [Security](security.md)
