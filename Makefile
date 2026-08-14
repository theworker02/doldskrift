# Doldskrift quality bar — Unix make / Git Bash / WSL.
# On Windows PowerShell without make, use the npm scripts or cargo commands below.

.PHONY: check fmt clippy test conformance doctor site:build links fixtures clean-doc doc

check: fmt clippy test
	@echo "check OK"

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- --deny warnings

test:
	cargo test --workspace
	npm test

conformance:
	cargo run -q -p doldskrift-cli -- conformance

doctor:
	cargo run -q -p doldskrift-cli -- doctor

site\:build:
	npm run site:build

links:
	npm run links

fixtures:
	npm run surfaces:fixtures

doc:
	cargo doc -p doldskrift --no-deps
	@echo "Open target/doc/doldskrift/index.html"

clean-doc:
	rm -rf target/doc
