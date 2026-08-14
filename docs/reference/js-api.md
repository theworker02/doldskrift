# JavaScript API reference

Package `@doldskrift/core` (TypeScript declarations included).

```ts
encode(text: string): string
decode(encoded: string): string
new Encoder(mode)
new Decoder()
Session.builder().seed(seed).build()
DskDocument.encodeText / parse / decodeText / toBytes / inspect
detectDoldskrift({ metaContent?, dataVersion?, dataMode?, text? })
StreamingEncoder / StreamingDecoder
```

React: `@doldskrift/react` — `<Doldskrift mode="…">`, `useDoldskrift`, `useDoldskriftDecoder`.

Web helpers: `@doldskrift/web` — `renderDoldskrift`, CSS `@font-face`.
