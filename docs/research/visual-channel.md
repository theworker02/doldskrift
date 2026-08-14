# Research: visual channel

The **visual channel** means recovering DSK symbols from pixels (screenshots, print, camera) rather than from Unicode strings.

## Why it is interesting

Agents sometimes only share a framebuffer or a photo of a terminal. A writing system that is *structurally* machine-readable could close that loop without Latin OCR.

## Current stack

```text
glyph → feature map / PNG
     → zone estimates
     → DSKG-1 candidate
     → classify vs catalog
```

GRE/1 adds echo marks so a single missing stroke is less fatal. True ECC (`DocumentFlags::ECC`) is reserved.

## Status

**Experimental.** Not part of DSK/1 conformance. Errors are `VisionError`. CLI: `dold vision`, Phase II `dold bench vision`.

## Threat-model clarity

A working visual decoder strengthens the claim that glyphs are for machines — and simultaneously proves that “looks unreadable” is not security. Document both facts whenever publishing results.

## Next measurements

- Recovery rate vs downscale / blur / JPEG quality
- Orientation bucket stability under rotation
- Whether fingerprint Hamming predicts vision failures
