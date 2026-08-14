# Positioning

## One sentence

Doldskrift is a **machine-native visual information system** — protocol, writing system, renderer, and recognition engine for structured data that can survive screens, print, and cameras.

## What it is

- A **protocol** (DSK) for framing machine information
- A **glyph grammar** (MGE) designed for recognition, not decoration
- A **vision path** (DVE) aimed at reconstructing symbols from visual media
- Open-source Lab tooling (`dold`) and demos (Lens, Black Page)

## What it is not

- **Not encryption in Open mode.** Visual illegibility frustrates casual reading; DSK/1–2 demos do not provide confidentiality. **Protected mode (DSK/3)** is the named encrypted product surface — design target, not shipping ([Protected mode](protected-mode.md)).
- **Not a QR replacement** for the general public.
- **Not** an accessible human interface by itself — always offer a semantic channel when people must understand content (safety, legal, medical, consent).
- **Not** a claim that browser demos equal production camera DVE.

## QR vs Doldskrift (careful wording)

| Dimension | QR (and similar) | Doldskrift |
|-----------|------------------|------------|
| Ecosystem | Ubiquitous scanners, standards, retail readiness | Research / prototype platform; agent-oriented |
| Visual form | Fixed 2D modules optimized for scanners | Font-like, zone-structured machine glyphs (MGE) |
| Primary reader | Commodity phone cameras + mature libraries | Agents and Lab tools; DVE still experimental |
| Human experience | Often human-scannable intent (“open this URL”) | Intentionally low human familiarity |
| Security role | Encoding + optional crypto wrappers elsewhere | **Open / Neural:** encoding / visual channel only. **Protected:** AEAD ciphertext in glyphs + Gate auth (not shipping) |
| When to choose | Need everyone to scan today | Exploring machine writing, visual channels, HCI, agent media |

**Fair statement:** QR wins today for universal scanability. Doldskrift explores a different design space — structured machine typography and protocolized visual transport — and should be evaluated on that basis, not as “better QR.”

**Unfair / avoid:** “Replaces QR,” “unbreakable visual crypto,” fake accuracy percentages, or invented deployment counts.

## Audience

| Audience | Message |
|----------|---------|
| Researchers / HCI | Machine writing systems, visual channels, recognition under degradation |
| Agent / tooling builders | Deterministic encode/decode, containers, Lab CLI |
| Partners | Open core, honest maturity, no security theater |
| General public | Curious demos (Lens); not a consumer security product |

## Related

- [Use cases](use-cases.md)
- [`SECURITY.md`](../../SECURITY.md)
- [Technology page](../../site/technology.html)
