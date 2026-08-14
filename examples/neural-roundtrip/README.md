# Example: Neural round-trip (foundation)

DeterministicBaseline path only — **no trained Gemma weights**. Neural ≠ encryption.

```bash
# from repository root
mkdir -p examples/neural-roundtrip/out

echo '{"msg":"hello neural"}' | cargo run -q -p doldskrift-cli -- neural encode -o examples/neural-roundtrip/out/plan.json
cargo run -q -p doldskrift-cli -- neural render examples/neural-roundtrip/out/plan.json -o examples/neural-roundtrip/out/carrier.svg --format svg
cargo run -q -p doldskrift-cli -- neural reconstruct examples/neural-roundtrip/out/plan.json
cargo run -q -p doldskrift-cli -- neural inspect examples/neural-roundtrip/out/plan.json
```

Expected honesty:

- Reconstruct uses DeterministicBaseline / Mock — exact match in CI under `crates/doldskrift/tests/neural_pipeline.rs`
- Fail closed on unknown grammar (see Unknown Page on the site)
- `dold doctor` reports `neural: ready (deterministic-baseline/1, no trained weights)`

Site: [Neural](https://doldskrift.github.io/doldskrift/neural.html) · [Neural Lens](https://doldskrift.github.io/doldskrift/neural-lens.html)

Protected contrast (refuse path):

```bash
echo "secret-label" | cargo run -q -p doldskrift-cli -- encode --mode protected -o examples/neural-roundtrip/out/protected.dsk
cargo run -q -p doldskrift-cli -- inspect examples/neural-roundtrip/out/protected.dsk
cargo run -q -p doldskrift-cli -- decode examples/neural-roundtrip/out/protected.dsk || true
```
