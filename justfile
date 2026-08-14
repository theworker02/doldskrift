# Doldskrift one-liners (https://github.com/casey/just)
# Fallback: Makefile or npm scripts.

check:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    npm test

test:
    cargo test --workspace
    npm test

site-build:
    npm run site:build

links:
    npm run links

fixtures:
    npm run surfaces:fixtures

doctor:
    cargo run -q -p doldskrift-cli -- doctor

conformance:
    cargo run -q -p doldskrift-cli -- conformance

doc:
    cargo doc -p doldskrift --no-deps
