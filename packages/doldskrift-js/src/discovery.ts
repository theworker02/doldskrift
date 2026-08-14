import {
  PROTOCOL_VERSION,
  isSymbol,
  parseMode,
  type Detection,
  type DetectionSource,
  type Mode,
} from "./types.js";

function none(): Detection {
  return {
    detected: false,
    version: null,
    mode: null,
    source: null,
    raw: null,
  };
}

/** Parse `content` from `<meta name="doldskrift" content="version=1;mode=encoded">`. */
export function detectFromHtmlMeta(content: string): Detection {
  let version: number | null = null;
  let mode: Mode | null = null;
  for (const part of content.split(";")) {
    const trimmed = part.trim();
    if (trimmed.startsWith("version=")) {
      const v = Number.parseInt(trimmed.slice("version=".length).trim(), 10);
      if (!Number.isNaN(v)) version = v;
    } else if (trimmed.startsWith("mode=")) {
      mode = parseMode(trimmed.slice("mode=".length).trim());
    }
  }
  if (version === null && mode === null && content.trim() === "") {
    return none();
  }
  return {
    detected: true,
    version: version ?? PROTOCOL_VERSION,
    mode,
    source: "html_meta" satisfies DetectionSource,
    raw: content,
  };
}

/** Detect from `data-doldskrift` and optional `data-doldskrift-mode`. */
export function detectFromAttrs(
  versionAttr: string | null | undefined,
  modeAttr: string | null | undefined,
): Detection {
  if (versionAttr == null) return none();
  const parsed = Number.parseInt(versionAttr, 10);
  const version = Number.isNaN(parsed) ? PROTOCOL_VERSION : parsed;
  const mode = modeAttr != null ? parseMode(modeAttr) : null;
  return {
    detected: true,
    version,
    mode,
    source: "data_attributes",
    raw: `data-doldskrift=${versionAttr};data-doldskrift-mode=${modeAttr ?? ""}`,
  };
}

/** Heuristic: fraction of PUA alphabet characters above threshold. */
export function detectPuaHeuristic(text: string, threshold = 0.5): Detection {
  const chars = [...text];
  const total = chars.length;
  if (total === 0) return none();
  let symbols = 0;
  for (const ch of chars) {
    if (isSymbol(ch)) symbols++;
  }
  const ratio = symbols / total;
  if (ratio >= threshold) {
    return {
      detected: true,
      version: PROTOCOL_VERSION,
      mode: "encoded",
      source: "pua_heuristic",
      raw: `ratio=${ratio.toFixed(3)}`,
    };
  }
  return none();
}

export interface DetectInput {
  metaContent?: string;
  dataVersion?: string;
  dataMode?: string;
  text?: string;
}

/**
 * Detect Doldskrift from HTML meta, data attributes, and/or PUA text heuristics.
 * Preference order: meta → data attributes → PUA heuristic.
 */
export function detectDoldskrift(input: DetectInput): Detection {
  if (input.metaContent != null && input.metaContent !== "") {
    const d = detectFromHtmlMeta(input.metaContent);
    if (d.detected) return d;
  }
  if (input.dataVersion != null) {
    const d = detectFromAttrs(input.dataVersion, input.dataMode);
    if (d.detected) return d;
  }
  if (input.text != null) {
    return detectPuaHeuristic(input.text);
  }
  return none();
}
