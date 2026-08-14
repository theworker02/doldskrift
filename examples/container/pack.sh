#!/usr/bin/env bash
set -euo pipefail
printf 'raw-bytes' > payload.bin
cargo run -q -p doldskrift-cli -- pack payload.bin -o payload.dsk
cargo run -q -p doldskrift-cli -- inspect payload.dsk
cargo run -q -p doldskrift-cli -- unpack payload.dsk -o out.bin
cmp payload.bin out.bin && echo "pack/unpack ok"
