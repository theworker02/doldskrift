/** Load Rust/WASM DSK codec — canonical encode/decode for the static site. */

let ready = null;
let usingWasm = false;

/** Resolve a path under the static site root (works with GitHub Pages project base). */
function siteUrl(rel) {
  return new URL(rel.replace(/^\//, ""), document.baseURI || window.location.href).href;
}

/** Initialize WASM (idempotent). Falls back to JS codec on failure. */
export async function initWasmCodec() {
  if (ready) return ready;
  ready = (async () => {
    try {
      // public/wasm → dist/wasm (not bundled); path must be relative to the page URL
      const mod = await import(/* @vite-ignore */ siteUrl("wasm/doldskrift.js"));
      await mod.default();
      usingWasm = true;
      return {
        encode: (t) => mod.wasm_encode(t),
        decode: (t) => mod.wasm_decode(t),
        puaBase: mod.wasm_pua_base(),
        protocolVersion: mod.wasm_protocol_version(),
        crateVersion: mod.wasm_crate_version(),
        backend: "wasm",
      };
    } catch (err) {
      console.warn("WASM codec unavailable, using JS fallback:", err);
      const js = await import("./doldskrift.js");
      usingWasm = false;
      return {
        encode: (t) => js.encode(t),
        decode: (t) => js.decode(t),
        puaBase: js.PUA_BASE,
        protocolVersion: 1,
        crateVersion: "js-fallback",
        backend: "js",
      };
    }
  })();
  return ready;
}

export function isWasmBackend() {
  return usingWasm;
}
