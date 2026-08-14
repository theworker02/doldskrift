<p align="center">
  <img src="../../assets/brand/logo-mark.svg" alt="Doldskrift" width="72" />
</p>

# `doldskrift-cli`

<p align="center">
  <a href="https://github.com/theworker02/doldskrift/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/doldskrift/doldskrift/ci.yml?branch=main&label=CI" alt="CI" /></a>
  <img src="https://img.shields.io/badge/MSRV-1.75-orange" alt="MSRV" />
</p>

Command-line interface for DOLDSKRIFT/1 (`dold`). Swedish *dold* + *skrift* → concealed script.

```bash
dold                                    # quiet-mode mark fingerprint
dold guide                              # in-CLI onboarding
dold demo -o ./out                      # Open product pack
dold commands                           # curated catalog (alias: topics)
dold self-test                          # encode/decode/validate + protected refuse
dold pipeline --text "Hello" -o msg.dsk
dold convert --from text --to postcard --text "Hello" -o card.svg
echo "Hello" | dold encode > msg.dsk    # alias: dold e
dold inspect msg.dsk
dold decode msg.dsk                     # alias: dold d
dold logo --out logo-mark.svg
dold about
dold museum
dold doctor
dold radio exchange "ping" -o radio-packet
echo "hi" | dold postcard -o postcard.svg && dold postcard --read postcard.svg
echo "graph" | dold mesh --html -o mesh.html
echo "hi" | dold render --tty
```

> Not encryption — representation only. See root README **Unexpected surfaces**.

[Root README](../../README.md) · brand assets in `assets/brand/`
