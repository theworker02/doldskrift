<p align="center">
  <img src="../../assets/brand/logo-mark.svg" alt="Doldskrift" width="72" />
</p>

# `doldskrift`

<p align="center">
  <a href="https://github.com/doldskrift/doldskrift/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/doldskrift/doldskrift/ci.yml?branch=main&label=CI" alt="CI" /></a>
  <img src="https://img.shields.io/badge/MSRV-1.75-orange" alt="MSRV" />
</p>

Umbrella Rust crate: machine-native typography, encoding, and agent communication (DOLDSKRIFT/1).

**Name:** Swedish *dold* (hidden) + *skrift* (writing) → concealed script.

> Not encryption — representation only. The project mark **is** glyph `DSK_PROJECT_MARK` (`U+E1F0`).

```rust
use doldskrift::{Decoder, Encoder, Mode};

let payload = Encoder::new(Mode::Encoded).encode("Hello, agent!")?;
assert_eq!(Decoder::new().decode(&payload)?, "Hello, agent!");
```

See the [root README](../../README.md) for visuals, CLI, and site demos.
