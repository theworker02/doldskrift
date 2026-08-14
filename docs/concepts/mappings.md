# Mappings

A **Mapping** is a bijection between the 256 UTF-8 byte values and the 256 PUA offsets.

```text
encode_table[byte] = offset
symbol = U+E000 + offset
decode_table[offset] = byte
```

## Identity — `static-v1`

Used by **encoded** mode.

```text
mapping_id = "static-v1"
encode_table[b] = b
```

Example: ASCII `Hi` → bytes `48 69` → symbols `U+E048 U+E069`.

## Session — seeded permutation

1. `state = mix_seed(seed)` (π-based IV, XOR little-endian chunks, SplitMix64)
2. Fisher–Yates shuffle of `[0…255]` using SplitMix64
3. `mapping_id = "session-v1-" + hex(CRC32C(seed))`

Full normative algorithm: [`spec/DOLDSKRIFT-1.md`](../../spec/DOLDSKRIFT-1.md) §6.

**Not a KDF.** Identical seeds exist so agents can interoperate, not so secrets stay secret.

## API sketch

### Rust

```rust
use doldskrift::Mapping;

let id = Mapping::identity();
let session = Mapping::from_seed(b"agent-alpha")?;
assert_eq!(session.encode_str("ok").len(), id.encode_str("ok").chars().count());
```

### JavaScript

```ts
import { Mapping } from "@doldskrift/core";

const id = Mapping.identity();
const session = Mapping.fromSeed(new TextEncoder().encode("agent-alpha"));
```

## Invariants

- Tables MUST be permutations (no collisions).
- `decode(encode(x)) == x` for valid UTF-8 `x`.
- Rust and JS MUST match golden vectors for the same seed.

## Debugging

CLI Phase II `dold explain` surfaces mapping id, mode, and (for sessions) manifest fields without treating the seed as sensitive key material — because it is not.
