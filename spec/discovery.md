# Discovery & Handshake

## HTML

```html
<meta name="doldskrift" content="version=1;mode=encoded">
```

```html
<div data-doldskrift="1" data-doldskrift-mode="session">...</div>
```

## Programmatic

`detectDoldskrift(...)` returns:

```ts
{
  detected: boolean,
  version?: number,
  mode?: "visual" | "encoded" | "session",
  source?: "html_meta" | "data_attributes" | "container_magic" | "pua_heuristic",
  raw?: string
}
```

## Handshake (optional)

Agent A:

```text
DSK?
versions=1
modes=visual,encoded,session
```

Agent B:

```text
DSK!
version=1
mode=session
```

This negotiates representation only. Use TLS (or equivalent) when confidentiality or authenticity is required.
