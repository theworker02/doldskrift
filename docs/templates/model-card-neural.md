# Model card template (Neural reader)

**Status:** Template only — fill when a trained reader ships.

## Model details

- Name / version:
- Architecture:
- Grammar epochs supported:
- Training framework:
- Checkpoint size / hash:
- License:

## Intended use

- Reconstruct DSK/3 Neural observations for declared epochs
- **Not** a confidentiality mechanism

## Out of scope

- Decoding Protected objects without Gate
- Silent decode of unsupported epochs

## Evaluation data

- Link to dataset card:
- Metrics (only measured values):

## Ethical / safety notes

- Fail closed on unknown input
- No fabricated plaintext on low confidence
