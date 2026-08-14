<p align="center">
  <img src="../../assets/brand/logo-mark.svg" alt="Doldskrift" width="72" />
</p>

# Product documentation

Honest product framing for **Doldskrift** — a machine-native visual information platform (independent OSS company product), not a QR clone and not a crypto theater.

**Open** and **Neural** are representation profiles. **Protected** is the named encrypted surface (AEAD + Gate) — refuse stubs today, not shipping crypto.

## Documents

| Doc | Purpose |
|-----|---------|
| [Positioning](positioning.md) | What it is / is not; QR differentiation |
| [Protected mode](protected-mode.md) | Open vs Protected claims |
| [Use cases](use-cases.md) | Grounded applications (no invented customers) |
| [Project brief](../project-brief.md) | Source material for talks and partners |

## Platform architecture (today)

| Name | Role |
|------|------|
| **DSK** | Protocol (DSK/1 prototype-stable; DSK/2 experimental; DSK/3 Neural + Protected) |
| **MGE** | Machine Glyph Engine |
| **DVE** | Vision Engine |
| **LSG** | Latent Structure Grammar (Neural) |
| **Gate** | Authorized runtime for Protected objects |
| **Forge** | Alphabet / glyph optimization (research) |
| **Lens** | Flagship visual demonstration (Open) |
| **Lab** | CLI and developer tooling (`dold`) |

```text
# Open
DATA → DSK → MGE → VISUAL → DVE → DSK → DATA

# Neural (foundation)
semantic → LSG → MGE/5 → visual → DVE → DSK-R

# Protected (target)
DATA → AEAD → DSK/3 → MGE → VISUAL → DVE → ciphertext → Gate(K) → DATA
```

Site surfaces (static GitHub Pages): [`site/`](../../site/) · live [https://doldskrift.github.io/doldskrift/](https://doldskrift.github.io/doldskrift/).

## Maturity

See the status table in the root [`README.md`](../../README.md). Do not publish numeric performance claims without reproducible `dold bench` / Criterion output.
