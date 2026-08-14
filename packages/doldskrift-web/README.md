<p align="center">
  <img src="../../assets/brand/logo-mark.svg" alt="Doldskrift" width="72" />
</p>

# `@doldskrift/web`

<p align="center">
  <a href="https://github.com/theworker02/doldskrift/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/doldskrift/doldskrift/ci.yml?branch=main&label=CI" alt="CI" /></a>
  <img src="https://img.shields.io/badge/license-Apache--2.0%20%7C%20MIT-blue" alt="License" />
  <a href="https://theworker02.github.io/doldskrift/"><img src="https://img.shields.io/badge/site-GitHub%20Pages-0F6B5C" alt="GitHub Pages" /></a>
</p>

**Thin browser adapter** over [`@doldskrift/core`](../doldskrift-js). Prefer importing from `@doldskrift/core` (codec + `renderDoldskrift` + CSS). This package remains for backward-compatible import paths.

```ts
// Preferred
import { encode, renderDoldskrift } from "@doldskrift/core";
import "@doldskrift/core/doldskrift.css";

// Still supported
import { encode, renderDoldskrift } from "@doldskrift/web";
import "@doldskrift/web/doldskrift.css";
```

**Doldskrift** (*dold* + *skrift*) — concealed script for machines. Not encryption.

Support: [thanks.dev/u/gh/theworker02](https://thanks.dev/u/gh/theworker02).

[Root README](../../README.md) · [Core package](../doldskrift-js/README.md)
