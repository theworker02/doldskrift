# Example: Hello world (Open)

End-to-end Open path: encode → validate → inspect → decode → specimen.

```bash
# from repository root
cargo build -q -p doldskrift-cli

echo "Hello from Doldskrift" | cargo run -q -p doldskrift-cli -- encode -o examples/hello-world/hello.dsk
cargo run -q -p doldskrift-cli -- validate examples/hello-world/hello.dsk
cargo run -q -p doldskrift-cli -- inspect --json examples/hello-world/hello.dsk
cargo run -q -p doldskrift-cli -- decode examples/hello-world/hello.dsk
echo "Hello from Doldskrift" | cargo run -q -p doldskrift-cli -- specimen -o examples/hello-world/hello.svg
```

Expected:

- `validate` prints OK / open profile
- `inspect --json` has `"schema": "doldskrift.inspect/1"` and `"profile": "open"`
- `decode` prints `Hello from Doldskrift`
- `hello.svg` is an MGE glyph strip (visual concealment, **not** encryption)

Health check anytime: `cargo run -q -p doldskrift-cli -- doctor`
