# Neural readers (DSK-R)

## Status enum

| Status | Meaning |
|--------|---------|
| `Reconstructed` | Value trustworthy |
| `Incompatible` | Epoch / reader mismatch |
| `LowConfidence` | Reserved |
| `UnknownGrammar` | Epoch unknown |
| `ReconstructionFailed` | Structural failure |
| `NotDoldskrift` | Input not recognized |

On failure: **no** fabricated convincing plaintext.

## Backends (foundation)

| Reader | Role |
|--------|------|
| `DeterministicBaseline` | Exact reverse of DeterministicComposer (CI) |
| `MockReader` | Same behavior, demo id |
| `GemmaReaderStub` | Feature `gemma-reader` only — refuses inference |
| `EpochLockedReader` | Test helper for wrong-reader cases |

## CLI

```text
dold reader info|list|install|verify|benchmark|remove|doctor|train-adapter
dold neural reconstruct --reader deterministic|mock
```

`install` refuses auto-download of large checkpoints.

## Session isolation

`SessionReader` + `reset()` stubs exist for future multi-tenant runtimes. Not a security boundary today.
