<p align="center">
  <img src="../../assets/brand/logo-mark.svg" alt="Doldskrift" width="72" />
</p>

# `@doldskrift/react`

<p align="center">
  <a href="https://github.com/theworker02/doldskrift/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/doldskrift/doldskrift/ci.yml?branch=main&label=CI" alt="CI" /></a>
  <img src="https://img.shields.io/badge/license-Apache--2.0%20%7C%20MIT-blue" alt="License" />
  <a href="https://theworker02.github.io/doldskrift/"><img src="https://img.shields.io/badge/site-GitHub%20Pages-0F6B5C" alt="GitHub Pages" /></a>
</p>

**React adapter** for Doldskrift Open mode — components and hooks over [`@doldskrift/core`](../doldskrift-js). Kept separate so React peer dependencies do not pollute the core SDK.

**Doldskrift** — Swedish *dold* + *skrift* (concealed script). Not encryption.

```tsx
import { Doldskrift } from "@doldskrift/react";

<Doldskrift mode="encoded">Hello, agent!</Doldskrift>
```

Re-exports `fundingInfo`, `THANKS_DEV_URL`, `BRAND_TOKENS` from `@doldskrift/core`. Support: [thanks.dev/u/gh/theworker02](https://thanks.dev/u/gh/theworker02).

[Root README](../../README.md) · [Core](../doldskrift-js/README.md) · [Guide](../../docs/guides/react.md)
