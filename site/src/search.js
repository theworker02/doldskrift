/** Tiny client-side docs search index. */
export const SEARCH_INDEX = [
  { title: "Session mapping", href: "./docs.html#session", blurb: "Seeded SplitMix64 permutations" },
  { title: "Checksum CRC32C", href: "./spec.html", blurb: "Corruption detection, not authenticity" },
  { title: "Glyph fingerprint", href: "./glyphs.html", blurb: "DSKG-1 zone fingerprints" },
  { title: "JS decoder", href: "./docs.html", blurb: "@doldskrift/core decode()" },
  { title: "DSK header", href: "./protocol.html", blurb: "Magic, version, mode, lengths, CRC" },
  { title: "Machine Reader", href: "./machine-reader.html", blurb: "Vision-only reconstruction demo" },
  { title: "Studio", href: "./studio.html", blurb: "Morph modes, fingerprints, SVG export" },
  { title: "Agent Radio", href: "./radio.html", blurb: "Live session-encoded agent packets" },
  { title: "Security model", href: "./security.html", blurb: "Not encryption" },
  { title: "MGE/2", href: "./glyphs.html", blurb: "Zone anatomy and GRE/1" },
  { title: "Project mark", href: "./studio.html", blurb: "DSK_PROJECT_MARK U+E1F0 logo glyph" },
  { title: "Specimen export", href: "./studio.html", blurb: "Export MGE/2 glyph strips as SVG" },
];

export function searchDocs(q) {
  const needle = q.trim().toLowerCase();
  if (!needle) return [];
  return SEARCH_INDEX.filter(
    (e) =>
      e.title.toLowerCase().includes(needle) ||
      e.blurb.toLowerCase().includes(needle)
  );
}
