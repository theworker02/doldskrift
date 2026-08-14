# `.dsk` Containers

File extension: `.dsk`  
Experimental MIME: `application/vnd.doldskrift`

## Layout

See [DOLDSKRIFT-1.md](./DOLDSKRIFT-1.md) §7.

## Payload by mode

| Mode | Payload |
|------|---------|
| visual | Semantic UTF-8 text |
| encoded | UTF-8 of PUA symbol string (identity mapping) |
| session | UTF-8 of PUA symbol string (session mapping) + manifest in header |

## APIs

Rust:

```rust
DskDocument::open(path)?;
DskDocument::parse(bytes)?;
doc.write(path)?;
doc.decode_text()?;
doc.inspect(); // never reveals payload by default
```

JavaScript mirrors these names on `DskDocument`.
