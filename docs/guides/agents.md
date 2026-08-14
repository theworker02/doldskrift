# Agents guide

Doldskrift is aimed at **software agents** that need a shared, deterministic text representation — optionally displayed as alien typography to human operators.

## Capability tokens

| Capability | Meaning |
|------------|---------|
| `DSK_VISUAL` | Can use visual mode |
| `DSK_ENCODED` | Understands `static-v1` |
| `DSK_SESSION` | Can build/consume session manifests |
| `DSK_STREAM` | Streaming encode/decode |
| `DSK_CONTAINER` | `.dsk` documents |
| `DSK_VISION` | Experimental pixel recognition |

Reference agents advertise the first five by default.

## Handshake

```text
DSK?
versions=1
modes=session,encoded,visual
capabilities=DSK_VISUAL,DSK_ENCODED,DSK_SESSION,DSK_STREAM,DSK_CONTAINER

DSK!
version=1
mode=session
```

CLI:

```bash
dold handshake offer > offer.txt
dold handshake negotiate offer.txt
```

Negotiation picks representation only. Put TLS (or your agent bus auth) underneath. **Protected** objects (Phase V) use a separate Gate capability model — see [Gate stub](../engineering/gate.md); handshake `DSK_*` tokens do not grant decrypt rights.

## Protected capabilities (Phase V target)

Illustrative Gate-scoped ops (not shipping):

| Capability id | Intent |
|---------------|--------|
| `dsk.capability.read.task` | Task-shaped fields for execution |
| `dsk.capability.summarize` | Condensed view |
| `dsk.capability.classify` | Labels / routing |
| `dsk.capability.export.plaintext` | Full semantic export |

LLMs should call Lab/Gate APIs and receive permitted results — never raw keys.

## Exchange patterns

### 1. Raw encoded stream

Agent A sends PUA text over an existing channel; Agent B runs `decode`.

### 2. `.dsk` document

Atomic message with checksum and optional session manifest — good for files, attachments, object stores.

### 3. HTML-embedded

Page advertises meta/data attributes; browser agent or scraper uses discovery APIs, then decodes.

## Recommended flow

```text
1. Optional DSK? / DSK!
2. Agree mode (prefer session if both support)
3. If session: share seed out-of-band or embed seed_hex in .dsk
4. Exchange payload
5. verify / decode
```

## Examples

- `examples/agent/`
- `examples/agent-exchange/`
- `examples/session/`
- `examples/streaming/`

## Ethics / safety

Do not use Doldskrift to conceal content that humans are legally entitled to read (accessibility, consent, safety). Agents SHOULD log semantic transcripts for human audit when operating in regulated environments.
