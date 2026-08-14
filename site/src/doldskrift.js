/** Minimal DOLDSKRIFT/1 codec for the documentation site (matches Rust). */

import { glyphSvgMge2, fingerprint, projectMarkSvg, DSK_PROJECT_MARK } from "./mge2.js";

export const PUA_BASE = 0xe000;
export { glyphSvgMge2 as glyphSvg, fingerprint, projectMarkSvg, DSK_PROJECT_MARK };

const U64 = {
  mask: (n) => BigInt.asUintN(64, n),
  add: (a, b) => BigInt.asUintN(64, a + b),
  mul: (a, b) => BigInt.asUintN(64, a * b),
  xor: (a, b) => BigInt.asUintN(64, a ^ b),
  shr: (a, n) => BigInt.asUintN(64, a >> BigInt(n)),
};

function splitmix64(x) {
  x = U64.add(x, 0x9e3779b97f4a7c15n);
  let z = x;
  z = U64.mul(U64.xor(z, U64.shr(z, 30)), 0xbf58476d1ce4e5b9n);
  z = U64.mul(U64.xor(z, U64.shr(z, 27)), 0x94d049bb133111ebn);
  return U64.xor(z, U64.shr(z, 31));
}

function mixSeed(seedBytes) {
  let state = 0x243f6a8885a308d3n;
  if (seedBytes.length === 0) return splitmix64(state);
  for (let i = 0; i < seedBytes.length; i += 8) {
    let v = 0n;
    for (let j = 0; j < 8 && i + j < seedBytes.length; j++) {
      v |= BigInt(seedBytes[i + j]) << BigInt(8 * j);
    }
    state = splitmix64(U64.xor(state, v));
  }
  return state;
}

export function identityTable() {
  return Uint8Array.from({ length: 256 }, (_, i) => i);
}

export function mappingFromSeed(seed) {
  const bytes =
    typeof seed === "string" ? new TextEncoder().encode(seed) : seed;
  let state = mixSeed(bytes);
  const table = identityTable();
  for (let i = 255; i >= 1; i--) {
    state = splitmix64(state);
    const j = Number(state % BigInt(i + 1));
    const tmp = table[i];
    table[i] = table[j];
    table[j] = tmp;
  }
  return table;
}

export function encodeWithTable(text, table) {
  const bytes = new TextEncoder().encode(text);
  let out = "";
  for (const b of bytes) {
    out += String.fromCodePoint(PUA_BASE + table[b]);
  }
  return out;
}

export function decodeWithTable(encoded, table) {
  const inv = new Uint8Array(256);
  for (let i = 0; i < 256; i++) inv[table[i]] = i;
  const bytes = [];
  for (const ch of encoded) {
    const cp = ch.codePointAt(0);
    if (cp < PUA_BASE || cp >= PUA_BASE + 256) {
      throw new Error(`invalid codepoint U+${cp.toString(16)}`);
    }
    bytes.push(inv[cp - PUA_BASE]);
  }
  return new TextDecoder().decode(Uint8Array.from(bytes));
}

export function encode(text) {
  return encodeWithTable(text, identityTable());
}

export function decode(text) {
  return decodeWithTable(text, identityTable());
}

export function encodeSession(text, seed) {
  return encodeWithTable(text, mappingFromSeed(seed));
}

export function decodeSession(text, seed) {
  return decodeWithTable(text, mappingFromSeed(seed));
}

export function renderGlyphRun(text, mode, seed, opts = {}) {
  const redundancy = opts.redundancy ?? 2;
  const animate = opts.animate ?? false;
  if (mode === "visual") {
    return [...text]
      .map((ch, i) => {
        if (ch === " " || ch === "\n") return ch === "\n" ? "<br/>" : `<span class="sp"> </span>`;
        const cp = ch.codePointAt(0);
        const svg = glyphSvgMge2(cp, { seed, redundancy, animate });
        return `<span class="g" style="animation-delay:${i * 18}ms" title="U+${cp.toString(16).toUpperCase()}">${svg}</span>`;
      })
      .join("");
  }
  const encoded =
    mode === "session" ? encodeSession(text, String(seed)) : encode(text);
  return [...encoded]
    .map((ch, i) => {
      const cp = ch.codePointAt(0);
      const svg = glyphSvgMge2(cp, { seed, redundancy, animate });
      return `<span class="g" data-cp="${cp}" style="animation-delay:${i * 18}ms" title="U+${cp.toString(16).toUpperCase()} · ${fingerprint(cp - PUA_BASE, redundancy)}">${svg}</span>`;
    })
    .join("");
}
