/** Browser MGE/2-style zone glyphs (aligned with Rust structural encoding). */

export const PUA_BASE = 0xe000;
export const DSK_PROJECT_MARK = 0xe1f0;

const PRIM = ["0", "T", "D", "A", "B", "R", "J", "X"];

export function zoneMapForSymbol(symbolId, redundancy = 2) {
  const b0 = symbolId & 0b11;
  const b1 = (symbolId >> 2) & 0b111;
  const b2 = (symbolId >> 5) & 0b111;
  let n = 1 + (b0 % 7);
  let e = 1 + (b1 % 7);
  let s = 1 + (b2 % 7);
  let w = 1 + ((reverseBits8(symbolId) & 0b111) % 7);
  let c = 1 + ((((symbolId * 3) >> 2) & 0b111) % 7);
  if (redundancy >= 2 && w === 0) w = 2;
  if (redundancy >= 3 && s === 0) s = 1;
  return { n, e, s, w, c, orientation: (symbolId * 5) & 7 };
}

function reverseBits8(v) {
  let x = v & 0xff;
  x = ((x & 0xf0) >> 4) | ((x & 0x0f) << 4);
  x = ((x & 0xcc) >> 2) | ((x & 0x33) << 2);
  x = ((x & 0xaa) >> 1) | ((x & 0x55) << 1);
  return x;
}

export function fingerprint(symbolId, redundancy = 2) {
  const z = zoneMapForSymbol(symbolId, redundancy);
  const integrity = (symbolId ^ (z.n << 1) ^ (z.c << 3) ^ redundancy) & 0xf;
  return `DSKG-1:N${z.n}-E${z.e}-S${z.s}-W${z.w}-C${z.c}-R${redundancy}-I${integrity.toString(16)}-O${z.orientation}`;
}

function zoneXY(zone) {
  switch (zone) {
    case "N":
      return [50, 22];
    case "E":
      return [78, 50];
    case "S":
      return [50, 78];
    case "W":
      return [22, 50];
    default:
      return [50, 50];
  }
}

function drawPrim(code, zx, zy, angle) {
  const rad = (angle * Math.PI) / 180;
  const dx = Math.cos(rad);
  const dy = Math.sin(rad);
  switch (code) {
    case 1: // terminal
      return `<circle cx="${zx}" cy="${zy}" r="4.5" fill="none"/><line x1="${zx - dx * 6}" y1="${zy - dy * 6}" x2="${zx + dx * 6}" y2="${zy + dy * 6}"/>`;
    case 2: // dot
      return `<circle cx="${zx}" cy="${zy}" r="3.2" fill="currentColor" stroke="none"/>`;
    case 3: // arc
      return `<path d="M${zx - 8} ${zy} A8 8 0 0 1 ${zx + 8} ${zy}"/>`;
    case 4: // branch
      return `<path d="M${zx} ${zy} L${zx + dy * 10} ${zy - dx * 10} M${zx} ${zy} L${zx - dy * 10} ${zy + dx * 10}"/>`;
    case 5: // ring
      return `<circle cx="${zx}" cy="${zy}" r="7"/>`;
    case 6: // junction
      return `<circle cx="${zx}" cy="${zy}" r="2.4" fill="currentColor" stroke="none"/>`;
    case 7: // cross
      return `<path d="M${zx - 8} ${zy} L${zx + 8} ${zy} M${zx} ${zy - 8} L${zx} ${zy + 8}"/>`;
    default:
      return "";
  }
}

/** Project mark — matches brand D geometry. */
export function projectMarkSvg(size = "1em") {
  return `<svg viewBox="0 0 100 100" width="${size}" height="${size}" fill="none" stroke="currentColor" stroke-width="6" stroke-linecap="square" stroke-linejoin="miter" class="project-mark" aria-label="DSK_PROJECT_MARK">
    <path d="M28 20 V80"/>
    <path d="M28 20 C58 20 78 36 78 50 C78 64 58 80 28 80"/>
    <path d="M42 36 L58 50 L42 64"/>
    <path d="M38 50 H54"/>
    <circle cx="28" cy="20" r="2.5" fill="currentColor" stroke="none"/>
    <circle cx="28" cy="80" r="2.5" fill="currentColor" stroke="none"/>
    <circle cx="78" cy="50" r="2.5" fill="#0F6B5C" stroke="none"/>
    <circle cx="88" cy="24" r="1.6" fill="currentColor" stroke="none"/>
    <circle cx="94" cy="24" r="1.6" fill="currentColor" stroke="none"/>
  </svg>`;
}

export function glyphSvgMge2(codepoint, { seed = 1, redundancy = 2, animate = false } = {}) {
  if (codepoint === DSK_PROJECT_MARK) return projectMarkSvg();
  const symbolId =
    codepoint >= PUA_BASE && codepoint < PUA_BASE + 256
      ? codepoint - PUA_BASE
      : codepoint & 0xff;
  const z = zoneMapForSymbol(symbolId ^ (seed & 0xff), redundancy);
  const angle = z.orientation * 45;
  const rad = (angle * Math.PI) / 180;
  const dx = Math.cos(rad);
  const dy = Math.sin(rad);
  let body = `<line class="spine" x1="${50 - dx * 28}" y1="${50 - dy * 28}" x2="${50 + dx * 28}" y2="${50 + dy * 28}"/>`;
  for (const [name, code] of [
    ["N", z.n],
    ["E", z.e],
    ["S", z.s],
    ["W", z.w],
    ["C", z.c],
  ]) {
    const [zx, zy] = zoneXY(name);
    body += drawPrim(code, zx, zy, angle + (name === "E" ? 0 : name === "N" ? 90 : name === "S" ? 270 : name === "W" ? 180 : 45));
  }
  // GRE ticks
  if (redundancy >= 2) {
    const ticks = (symbolId & 3) + 1;
    for (let t = 0; t < ticks; t++) {
      body += `<circle cx="${82 + t * 5}" cy="18" r="1.6" fill="currentColor" stroke="none"/>`;
    }
  }
  const anim = animate
    ? `<style>@keyframes ink{from{stroke-dashoffset:120}to{stroke-dashoffset:0}}.spine{stroke-dasharray:120;animation:ink .7s ease both}</style>`
    : "";
  return `<svg viewBox="0 0 100 100" width="1em" height="1em" fill="none" stroke="currentColor" stroke-width="5.5" stroke-linecap="round" stroke-linejoin="round" data-fp="${fingerprint(symbolId, redundancy)}">${anim}${body}</svg>`;
}

export function fingerprintLegend(fp) {
  const m = Object.fromEntries(
    fp
      .replace(/^DSKG-1:/, "")
      .split("-")
      .map((p) => [p[0], p.slice(1)])
  );
  return {
    format: "DSKG-1",
    zones: {
      N: PRIM[Number(m.N) || 0],
      E: PRIM[Number(m.E) || 0],
      S: PRIM[Number(m.S) || 0],
      W: PRIM[Number(m.W) || 0],
      C: PRIM[Number(m.C) || 0],
    },
    redundancy: Number(m.R) || 0,
    integrity: m.I,
    orientation: Number(m.O) || 0,
  };
}
