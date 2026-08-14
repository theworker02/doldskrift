## Summary

<!-- What changed and why. Profiles touched: Open / Neural / Protected? -->

## Checklist

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `npm test` (golden vectors)
- [ ] `dold conformance` / `dold doctor` when CLI touched
- [ ] `npm run site:build` + `npm run links` if site/docs links touched
- [ ] Spec / DEP / ADR updated if protocol behavior changed
- [ ] `docs/reference/cli.md` + `CHANGELOG.md` updated when CLI/API changes
- [ ] No claim of encryption / confidentiality (Neural ≠ encryption; Protected AEAD not shipping)
- [ ] Workspace still **5 crates**

## Test plan

<!-- How you verified -->
