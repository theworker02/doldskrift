# Latent Structure Grammar (LSG)

## Versioning

- Major: `LSG/1`
- Epochs: `LSG/1-E0001`, `LSG/1-E0002`, …
- Foundation composer emits **E0001** only

Readers declare supported epochs. Silent decode across incompatible epochs is forbidden.

## Types

| Type | Role |
|------|------|
| `LatentNode` | Atom / sequence / record / field / null / opaque |
| `LatentEdge` | Directed relation (`child`, `item:N`, `field`, `value`) |
| `LatentMetadata` | Epoch, content_id, composer id |
| `LatentGraph` | Full intermediate structure |

## Composition

`SemanticComposer` maps `semantic::Value` → `LatentGraph`.  
`DeterministicComposer` is the CI reference implementation.

## Relation to visuals

MGE/5 consumes latent structure (not only PUA symbol IDs). Current code **plans** compounds and contextual variants; path geometry is TODO.
