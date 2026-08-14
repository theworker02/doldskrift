# CLI guide (`dold`)

Build:

```bash
cargo install --path crates/doldskrift-cli
```

## Everyday workflow

```bash
dold guide
dold doctor
dold demo -o ./dold-demo
echo "Hello from Doldskrift" | dold encode > message.dsk
dold inspect message.dsk
dold verify message.dsk
dold decode message.dsk
dold convert --from text --to postcard --text "hi" -o card.svg
```

## Commands (overview)

| Command | Purpose |
|---------|---------|
| `guide` | In-CLI onboarding (honesty, swedish, surfaces, …) |
| `demo` | One-shot Open product pack (`.dsk` + postcard + mark) |
| `convert` | Bridge text/dsk ↔ postcard/ambient/seal |
| `config` | Project defaults (`.doldskrift/config.json`) |
| `encode` | Text → `.dsk` (or `--raw` PUA text) |
| `decode` | `.dsk` / raw → semantic text |
| `inspect` | Metadata without payload (`--reveal` opt-in) |
| `verify` | Checksum + structure |
| `pipeline` / `self-test` / `commands` | Workflows & catalog |
| `render` | Mode transform for specimens (`--tty` braille/blocks) |
| `font` | `build` / `inspect` font catalogs |
| `vision` / `scan` | Structural vision; `--diff` compares maps |
| `open` | Protected Gate stub → AccessDenied until AEAD |
| `doctor` | Install health + constants sync |
| `neural` | DSK/3 Neural encode / render / reconstruct (Mock/Deterministic) |
| `radio` / `echo` / `ambient` / … | Unexpected Open visual surfaces |
| `museum` / `about` / `funding` | Identity & sustainability |

Full flag reference: [reference/cli.md](../reference/cli.md). CLI contract (`scan` / `inspect` vs `open`): [Gate stub](../engineering/gate.md). `decode` remains the Open-mode path; Protected refuses semantic recovery.

## Encode options

```bash
dold encode input.txt -o out.dsk --mode encoded
dold encode --mode session --seed alpha --session-id demo
dold encode --raw   # stdout is PUA text, not DSK1
```

## Render / density

```bash
dold render --mode encoded --density normal
dold render --mode session --seed alpha --debug
echo "tty demo" | dold render --tty
```

`--debug` annotates fingerprints / zones for specimens; `--density` selects GRE/1 level via low/normal/high; `--tty` prints braille/block art (no browser).

## Unexpected surfaces

See root README. Quick try:

```bash
dold radio exchange "ping" -o radio-packet
echo "hi" | dold echo -o echo.svg && dold echo --verify echo.svg
echo "stars" | dold ambient -o ambient.svg && dold ambient --decode ambient.svg
dold museum
```

## Fonts

```bash
dold font build --out fonts/generated --seed 1
dold font inspect --catalog fonts/generated/glyphs.json
```

## Conformance & handshake

```bash
dold conformance --vectors spec/vectors
dold handshake offer
dold handshake negotiate examples/agent/handshake.txt
```

## Exit codes

`0` success; non-zero on typed failure (message on stderr as `dold: …`).
