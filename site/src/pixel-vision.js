/**
 * Pixel-true glyph recognition for Lens.
 *
 * Builds a catalog by rasterizing each alphabet glyph off-screen, then matches
 * canvas cells by binary downsampled signatures — no retained symbol IDs.
 *
 * Rotation: cells may be drawn rotate-within-cell. Matching deskews modest
 * angles (±ROT_TRY) so modest rotation can still round-trip; large rotation
 * may fail honestly.
 */

import { glyphSvgMge2, PUA_BASE } from "./mge2.js";

const GRID = 12;
/** Deskew candidates tried around the declared cell rotation (degrees). */
const ROT_TRY = [0, -2, 2, -4, 4, -6, 6, -8, 8, -10, 10, -12, 12, -15, 15];
let catalog = null; // Float32Array[256 * GRID*GRID] or null
let deskewCanvas = null;
let deskewCtx = null;

function svgToImage(svgMarkup, size = 64) {
  const blob = new Blob([svgMarkup], { type: "image/svg+xml" });
  const url = URL.createObjectURL(blob);
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => {
      URL.revokeObjectURL(url);
      resolve(img);
    };
    img.onerror = reject;
    img.src = url;
  });
}

function wrapSvg(inner) {
  const body = inner.replace(/<svg[^>]*>/, "").replace(/<\/svg>/, "");
  return `<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 100 100">${body}</svg>`;
}

/** Binary ink signature: GRID×GRID floats in 0..1 */
export function signatureFromImageData(data, width, height) {
  const sig = new Float32Array(GRID * GRID);
  const cellW = width / GRID;
  const cellH = height / GRID;
  for (let gy = 0; gy < GRID; gy++) {
    for (let gx = 0; gx < GRID; gx++) {
      let ink = 0;
      let tot = 0;
      const x0 = Math.floor(gx * cellW);
      const y0 = Math.floor(gy * cellH);
      const x1 = Math.floor((gx + 1) * cellW);
      const y1 = Math.floor((gy + 1) * cellH);
      for (let y = y0; y < y1; y++) {
        for (let x = x0; x < x1; x++) {
          const i = (y * width + x) * 4;
          const r = data[i];
          const g = data[i + 1];
          const b = data[i + 2];
          const a = data[i + 3];
          const lum = (r + g + b) / 3;
          // Prefer detecting non-background: away from near-black bg
          const lit = a > 20 && lum > 55;
          if (lit) ink += 1;
          tot += 1;
        }
      }
      sig[gy * GRID + gx] = tot ? ink / tot : 0;
    }
  }
  return sig;
}

function signatureDistance(a, b) {
  let sum = 0;
  for (let i = 0; i < a.length; i++) {
    const d = a[i] - b[i];
    sum += d * d;
  }
  return Math.sqrt(sum / a.length);
}

function ensureDeskew() {
  if (!deskewCanvas) {
    deskewCanvas = document.createElement("canvas");
    deskewCanvas.width = 64;
    deskewCanvas.height = 64;
    deskewCtx = deskewCanvas.getContext("2d", { willReadFrequently: true });
  }
  return { canvas: deskewCanvas, ctx: deskewCtx };
}

/**
 * Sample a cell and optionally deskew by `deskewDeg` (inverse of draw rotation).
 * Pads the crop so rotated content stays inside the sample window.
 */
export function signatureFromCell(ctx, x, y, w, h, deskewDeg = 0) {
  const pad = Math.ceil(Math.max(w, h) * 0.35);
  const sx = Math.max(0, Math.floor(x - pad));
  const sy = Math.max(0, Math.floor(y - pad));
  const sw = Math.min(ctx.canvas.width - sx, Math.ceil(w + pad * 2));
  const sh = Math.min(ctx.canvas.height - sy, Math.ceil(h + pad * 2));
  if (sw <= 0 || sh <= 0) {
    return signatureFromImageData(new Uint8ClampedArray(4), 1, 1);
  }

  if (Math.abs(deskewDeg) < 0.01) {
    const img = ctx.getImageData(Math.floor(x), Math.floor(y), Math.max(1, Math.floor(w)), Math.max(1, Math.floor(h)));
    return signatureFromImageData(img.data, img.width, img.height);
  }

  const { canvas, ctx: dctx } = ensureDeskew();
  const size = 64;
  canvas.width = size;
  canvas.height = size;
  dctx.setTransform(1, 0, 0, 1, 0, 0);
  dctx.fillStyle = "#101412";
  dctx.fillRect(0, 0, size, size);
  dctx.save();
  dctx.translate(size / 2, size / 2);
  dctx.rotate((-deskewDeg * Math.PI) / 180);
  // Map padded source so the cell center lands at origin
  const cx = x + w / 2 - sx;
  const cy = y + h / 2 - sy;
  const scale = size / Math.max(w, h);
  dctx.scale(scale, scale);
  dctx.drawImage(ctx.canvas, sx, sy, sw, sh, -cx, -cy, sw, sh);
  dctx.restore();
  const { data } = dctx.getImageData(0, 0, size, size);
  return signatureFromImageData(data, size, size);
}

/** Precompute 256 glyph pixel signatures (lazy, once). */
export async function buildPixelCatalog() {
  if (catalog) return catalog;
  const off = document.createElement("canvas");
  off.width = 64;
  off.height = 64;
  const octx = off.getContext("2d", { willReadFrequently: true });
  const out = new Float32Array(256 * GRID * GRID);

  for (let id = 0; id < 256; id++) {
    const cp = PUA_BASE + id;
    octx.fillStyle = "#101412";
    octx.fillRect(0, 0, 64, 64);
    const svg = wrapSvg(glyphSvgMge2(cp, { redundancy: 2 }));
    const img = await svgToImage(svg);
    octx.drawImage(img, 0, 0, 64, 64);
    const { data } = octx.getImageData(0, 0, 64, 64);
    const sig = signatureFromImageData(data, 64, 64);
    out.set(sig, id * GRID * GRID);
  }
  catalog = out;
  return catalog;
}

/**
 * Recognize one canvas cell against the pixel catalog.
 * Tries deskew angles around `cell.rotation` (rotate-within-cell draw angle).
 * @returns {{ id: number, confidence: number, distance: number, margin: number, deskewUsed: number }}
 */
export function matchCell(ctx, x, y, w, h, cat, rotation = 0) {
  let bestId = 0;
  let bestDist = Infinity;
  let second = Infinity;
  let deskewUsed = 0;

  const angles = new Set();
  for (const d of ROT_TRY) {
    angles.add(rotation + d);
  }
  // Always try upright (0) in case rotation metadata is wrong
  angles.add(0);

  for (const ang of angles) {
    const sig = signatureFromCell(ctx, x, y, w, h, ang);
    for (let id = 0; id < 256; id++) {
      const slice = cat.subarray(id * GRID * GRID, (id + 1) * GRID * GRID);
      const d = signatureDistance(sig, slice);
      if (d < bestDist) {
        second = bestDist;
        bestDist = d;
        bestId = id;
        deskewUsed = ang;
      } else if (d < second) {
        second = d;
      }
    }
  }

  const confidence = Math.max(0, Math.min(1, 1 - bestDist * 2.2));
  return {
    id: bestId,
    confidence,
    distance: bestDist,
    margin: second - bestDist,
    deskewUsed,
  };
}

/**
 * Scan a layout of glyph cells (skip mark cells).
 * @param {CanvasRenderingContext2D} ctx
 * @param {{x:number,y:number,w:number,h:number,kind:'mark'|'glyph',rotation?:number}[]} cells
 */
export async function scanCanvasCells(ctx, cells) {
  const cat = await buildPixelCatalog();
  const glyphs = [];
  const ids = [];
  let deskewSum = 0;
  let deskewN = 0;
  for (const cell of cells) {
    if (cell.kind === "mark") continue;
    const rot = cell.rotation ?? 0;
    const m = matchCell(ctx, cell.x, cell.y, cell.w, cell.h, cat, rot);
    glyphs.push(m);
    ids.push(m.id);
    deskewSum += m.deskewUsed;
    deskewN += 1;
  }
  const meanConf =
    glyphs.length === 0
      ? 0
      : glyphs.reduce((s, g) => s + g.confidence, 0) / glyphs.length;
  const meanDeskew = deskewN ? deskewSum / deskewN : 0;
  return {
    ids,
    glyphs,
    meanConfidence: meanConf,
    meanDeskew,
    strategy: "rotate-within-cell + multi-angle deskew",
  };
}

/** Reconstruct UTF-8 text from recovered symbol byte ids. */
export function bytesFromSymbolIds(ids) {
  return new TextDecoder().decode(Uint8Array.from(ids));
}
