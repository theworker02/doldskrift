# Neural training

## Boundary

| Layer | Language |
|-------|----------|
| Protocol, codec, vision runtime, CLI | **Rust** |
| Dataset generation experiments, adapters, eval notebooks | **Python** under `training/` |

Do **not** reimplement DSK framing or LSG wire formats in Python. Export fixtures from Rust (`dold dataset generate`) and consume them.

## Layout

```text
training/
├── README.md
├── datasets/      # fixtures + .gitkeep (no multi-GB weights in git)
├── generation/
├── configs/
├── evaluation/
├── adapters/
└── scripts/
```

## Deferred

Gemma/LoRA training, checkpoint packaging, and published bake-off numbers. See [evaluation](evaluation.md).
