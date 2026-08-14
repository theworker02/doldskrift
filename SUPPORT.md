# Support

| Need | Where |
|------|--------|
| Bugs | [GitHub Issues](https://github.com/doldskrift/doldskrift/issues) |
| Ideas / research | GitHub Discussions (when enabled) |
| Security | See [`SECURITY.md`](SECURITY.md) — private reporting channel |
| Docs gaps | Documentation issue form |
| Contributing | [`CONTRIBUTING.md`](CONTRIBUTING.md) |
| Financial support | [thanks.dev/u/gh/theworker02](https://thanks.dev/u/gh/theworker02) · [`.github/FUNDING.yml`](.github/FUNDING.yml) · site `/funding.html` · `dold funding` |
| Commercial / partnership inquiries | Open a Discussion tagged “partnership” or contact listed on `/partners` when published |

Please include Doldskrift version (`dold --version`), OS, alphabet/profile (Open / Neural / Protected), and a minimal reproduction.

Quick local checks before filing:

```bash
dold doctor
dold version --verbose
dold about
cargo test --workspace
# if site/docs: npm test && npm run site:build && npm run links
```

Doldskrift is open source. There is no SLA unless a separate commercial agreement exists.

**Name note:** *Doldskrift* is Swedish (*dold* + *skrift*, “concealed script”). Open and Neural modes are not encryption — see [`SECURITY.md`](SECURITY.md).
