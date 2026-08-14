# Sessions

A **Session** binds a seed to a Mapping and publishes a **SessionManifest** so another agent can decode.

## Lifecycle

```text
Session::builder()
  .seed(bytes)
  .session_id("opaque-id")
  .build()
    → Mapping::from_seed(seed)
    → SessionManifest { protocol, version, mode, session_id, mapping_id, alphabet, checksum, seed_hex? }
```

Encode/decode call through the session’s mapping.

## Manifest fields

| Field | Purpose |
|-------|---------|
| `protocol` | Must be `doldskrift` |
| `version` | `1` |
| `mode` | `session` |
| `session_id` | Caller-chosen conversation / channel id |
| `mapping_id` | `session-v1-XXXXXXXX` from seed CRC |
| `alphabet` | `pua-byte-256` |
| `checksum` | Hex CRC32C of the 256-byte encode table |
| `seed_hex` | Optional but used by reference `.dsk` recovery |

## Containers

`DskDocument::encode_session(text, &session)` stores the manifest in header JSON and the encoded PUA string as payload. On open, `recover_session()` rebuilds the mapping from `seed_hex`.

## Security reminder

Publishing `seed_hex` is normal in **Open** session mode. Anyone with the `.dsk` file can decode. If that is a problem, you needed encryption **before** Open-mode Doldskrift — or **Protected** mode (DSK/3 / Gate) when it ships. Session seeds are not AEAD keys.

## Negotiation

Agents often select session mode via `DSK?` / `DSK!` when both advertise `DSK_SESSION`. Preference order: session > encoded > visual. See [negotiation](../protocol/negotiation.md).

## Example

```rust
use doldskrift::{DskDocument, Session};

let session = Session::builder()
    .seed(b"alpha")
    .session_id("001")
    .build()?;
let doc = DskDocument::encode_session("ping", &session)?;
assert_eq!(doc.decode_text()?, "ping");
```
