# Example: Agent exchange (Open surfaces)

Two agents exchange a tiny visual packet: `.dsk` + SVG + SHA-256 manifest.

Honesty: digests are **integrity** aids, not signatures. Open mode is not encryption.

```bash
# from repository root
cargo run -q -p doldskrift-cli -- radio exchange "Deploy worker seven." -o examples/agent-exchange/packet

# Inspect the container
cargo run -q -p doldskrift-cli -- validate examples/agent-exchange/packet.dsk
cargo run -q -p doldskrift-cli -- inspect --json examples/agent-exchange/packet.dsk
cargo run -q -p doldskrift-cli -- decode examples/agent-exchange/packet.dsk

# Or regenerate via the font crate helper (same semantics)
cargo run -q -p doldskrift-font --example agent_exchange
```

Also try related surfaces:

```bash
echo "ping" | cargo run -q -p doldskrift-cli -- echo -o examples/agent-exchange/echo.svg
cargo run -q -p doldskrift-cli -- echo --verify examples/agent-exchange/echo.svg
cargo run -q -p doldskrift-cli -- handshake visual -o examples/agent-exchange/handshake.svg
```

Site companion: [Agent Radio](https://doldskrift.github.io/doldskrift/radio.html) (static simulation).
