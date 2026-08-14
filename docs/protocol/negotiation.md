# Negotiation

Optional ASCII handshake selects a mutually supported **representation** mode.

## Wire format

Offer (`DSK?`):

```text
DSK?
versions=1
modes=session,encoded,visual
capabilities=DSK_VISUAL,DSK_ENCODED,DSK_SESSION,DSK_STREAM,DSK_CONTAINER
```

Reply (`DSK!`):

```text
DSK!
version=1
mode=session
```

Constants: `HANDSHAKE_QUERY = "DSK?"`, `HANDSHAKE_REPLY = "DSK!"`.

## Preference

When intersecting capability sets, preferred mode order is:

1. `session`
2. `encoded`
3. `visual`

If the intersection is empty → `NegotiationFailed`.

## What negotiation is not

- Not key exchange
- Not authentication
- Not authorization
- Not a substitute for TLS

## CLI

```bash
dold handshake offer
dold handshake negotiate path/to/offer.txt
```

## Rust

```rust
use doldskrift::{negotiate, CapabilitySet, HandshakeOffer};

let offer = HandshakeOffer::reference();
let reply = negotiate(&offer, &CapabilitySet::reference())?;
```
