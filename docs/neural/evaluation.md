# Neural evaluation

## What is measured today

- DeterministicBaseline semantic round-trip: **exact SHA-256** match (`cargo test -p doldskrift --test neural_pipeline`)
- Wrong-reader → `Incompatible`
- Non-Doldskrift → `NotDoldskrift`

## Harness stubs

```text
dold evaluate baseline
```

Prints honest TBD for adversarial OCR and trained-reader scores.

## Planned metrics (empty until measured)

- Symbol / structure recovery under blur, downsample, JPEG, print-scan
- Familiarity / separability under MGE5-NEURAL (Forge 3)
- Cross-epoch retention

Do not publish invented numbers.
