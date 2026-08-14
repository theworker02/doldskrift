# Neural reproducibility

## CI path (no weights)

```bash
cargo test -p doldskrift --test neural_pipeline
cargo test -p doldskrift-font mge5 --lib
cargo test -p doldskrift-vision dve2 --lib
```

## Deterministic fixtures

```bash
dold dataset generate --seed 1 --out training/datasets/synthetic
```

Same seed → same JSON structure (1×1 PNG stub is fixed placeholder).

## Training reproducibility (future)

Record: config hash, dataset card, adapter card, commit SHA, epoch id, eval command. Templates: [`docs/templates/`](../templates/).
