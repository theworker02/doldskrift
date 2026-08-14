# JavaScript guide

Package: **`@doldskrift/core`** — primary JS SDK (codec + browser helpers + CSS).  
Thin adapters: `@doldskrift/web` (re-export), `@doldskrift/react` (React peer deps).  
WASM: generated `doldskrift-bindings` (Rust remains canonical).

## Install

```bash
npm install @doldskrift/core
```

In this monorepo, workspace packages live under `packages/doldskrift-js` (published name `@doldskrift/core`). Prefer core over `@doldskrift/web` for new code.

## Encode / decode

```ts
import { encode, decode, Encoder, Decoder } from "@doldskrift/core";

const opaque = encode("hello agent");
const clear = decode(opaque);

const enc = new Encoder("encoded");
const dec = new Decoder();
```

## Mapping & session

```ts
import { Mapping, Session } from "@doldskrift/core";

const m = Mapping.fromSeed(new TextEncoder().encode("alpha"));
const session = Session.builder().seed("alpha").sessionId("001").build();
const opaque = session.encode("ping");
```

SplitMix64 / `mixSeed` are exported for conformance debugging — keep them synchronized with Rust.

## Documents

```ts
import { DskDocument } from "@doldskrift/core";

const doc = DskDocument.encodeText("hello", "encoded");
const bytes = doc.toBytes(); // Uint8Array
const again = DskDocument.parse(bytes);
console.log(again.inspect()); // does not decode unless you call decodeText()
```

## Streaming

```ts
import { StreamingEncoder, StreamingDecoder } from "@doldskrift/core";

const enc = new StreamingEncoder();
let out = enc.push("Hello, ") + enc.push("world");
out += enc.finish();

const dec = new StreamingDecoder();
let clear = dec.push(out);
clear += dec.finish();
```

## Discovery

```ts
import {
  detectDoldskrift,
  detectFromHtmlMeta,
  MIME_BINARY,
  MIME_TEXT,
} from "@doldskrift/core";

detectFromHtmlMeta("version=1;mode=session");
// MIME_BINARY === "application/vnd.doldskrift"
// MIME_TEXT === "text/doldskrift"
```

## Errors

Failures throw `DoldskriftError` with a stable `code` string aligned to the Rust error taxonomy where applicable.

## Tests

```bash
npm test
```

Vector tests under `packages/doldskrift-js/test/vectors.test.ts` MUST stay green with `spec/vectors`.
