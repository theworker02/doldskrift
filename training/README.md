# Training experiments (Python)

Python is for **training / evaluation experiments only**.

Production codec, protocol, vision runtime, and CLI remain **Rust-first**. Do not create a second Doldskrift protocol implementation here.

## Layout

| Path | Purpose |
|------|---------|
| `datasets/` | Fixtures (use `dold dataset generate`); no multi-GB weights in git |
| `generation/` | Experiment scripts for synthetic expansion |
| `configs/` | Training / eval YAML stubs |
| `evaluation/` | Metric harness stubs |
| `adapters/` | Future LoRA / adapter export notes |
| `scripts/` | Entrypoints |

## Quick fixture

```bash
dold dataset generate --seed 1 --out training/datasets/synthetic
```

## Status

No Gemma/LoRA training code ships in this foundation pass. See `docs/neural/training.md`.
