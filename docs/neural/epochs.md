# Grammar epochs

## Format

`LSG/1-E####` — major LSG version + epoch number.

Example: `LSG/1-E0001` (foundation).

## Compatibility rules

1. Reader manifests list supported epochs explicitly.
2. Encode embeds epoch in `LatentMetadata`.
3. Reconstruct requires exact support — no silent upgrade/downgrade.
4. Mismatch → `Incompatible` or `UnknownGrammar`.

## Evolution

New epochs may change latent topology or visual grammar. Breaking changes bump the epoch number. Document migration in this file when E0002 lands.

## Matrix

A site stub (`site/compatibility.html`) will list reader × epoch support. Generate from manifests when packaging exists.
