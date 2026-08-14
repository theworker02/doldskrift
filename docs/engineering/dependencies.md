# Cargo features (Phase IV)

Preferred feature surface for the workspace:

| Crate | Features | Notes |
|-------|----------|--------|
| `doldskrift` | `default=[]`, `ffi` | Core protocol; no vision/image |
| `doldskrift-font` | (none required) | MGE + Forge |
| `doldskrift-vision` | (none) | Pulls `image` |
| `doldskrift-bindings` | `wasm` (default), `ffi` | WASM / C ABI glue |
| `doldskrift-cli` | — | Depends on core+font+vision |

Avoid micro-features. Optional future: `async`, `compression`, `ecc`, `crypto` as **integration hooks** only.

## Dependency audit (runtime)

| Crate | Why |
|-------|-----|
| `thiserror` | Typed errors |
| `serde` / `serde_json` | Headers, manifests, reports |
| `crc32c` | Corruption detection |
| `hex` / `base64` | Armor / digests display |
| `sha2` | Content addressing (`dsk:sha256:…`) |
| `clap` | CLI |
| `image` | Vision only |
| `wasm-bindgen` | Bindings only |

Dev: `proptest`, `criterion`.

Check duplicates:

```bash
cargo tree -d
```

Duplicates observed today are mostly from **dev-dependencies** (`proptest` / `criterion` pulling alternate `getrandom` / `syn` versions) — not shipped in the release CLI codec path.
