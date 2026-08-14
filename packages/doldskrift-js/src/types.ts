/** DOLDSKRIFT/1 operating mode. */
export type Mode = "visual" | "encoded" | "session";

/** Where Doldskrift metadata was found. */
export type DetectionSource =
  | "html_meta"
  | "data_attributes"
  | "container_magic"
  | "pua_heuristic";

/** Structured detection result. */
export interface Detection {
  detected: boolean;
  version: number | null;
  mode: Mode | null;
  source: DetectionSource | null;
  raw: string | null;
}

/** Machine-readable session manifest. */
export interface SessionManifest {
  protocol: string;
  version: number;
  mode: string;
  session_id: string;
  mapping_id: string;
  alphabet: string;
  checksum: string;
  seed_hex?: string;
}

/** JSON header embedded in a `.dsk` container. */
export interface DocumentHeader {
  version: number;
  mode: Mode;
  flags: number;
  checksum: number;
  mapping_id?: string;
  session?: SessionManifest;
  char_count?: number;
  byte_count?: number;
  font_version: number;
  meta?: Record<string, unknown>;
}

/** Diagnostic summary for document inspection. */
export interface InspectReport {
  protocol: string;
  mode: string;
  payload_bytes: number;
  characters: number | null;
  mapping: string;
  checksum_valid: boolean;
  checksum: number;
  font_version: number;
  session_id: string | null;
}

// Keep in sync with spec/protocol-constants.json (and packages/.../protocol-constants.json).
export const PROTOCOL_NAME = "doldskrift";
export const PROTOCOL_VERSION = 1;
/** Protected stub container version (AEAD not shipping). */
export const PROTOCOL_VERSION_PROTECTED = 3;
export const PUA_BASE = 0xe000;
export const ALPHABET_SIZE = 256;
/** Align with Rust `FONT_VERSION` / MGE/2. */
export const FONT_VERSION = 2;
export const GLYPH_ENGINE_VERSION = 2;
export const MAGIC = new TextEncoder().encode("DSK1");
export const MAGIC_DSK2 = new TextEncoder().encode("DSK2");
export const HANDSHAKE_QUERY = "DSK?";
export const HANDSHAKE_REPLY = "DSK!";
/** Project mark codepoint — logo is a real machine glyph. */
export const DSK_PROJECT_MARK = 0xe1f0;
export const MIME_BINARY = "application/vnd.doldskrift";
export const MIME_TEXT = "text/doldskrift";
export const CHECKSUM_ALGORITHM = "crc32c";

export function isProjectMark(cp: number): boolean {
  return cp === DSK_PROJECT_MARK;
}

/** Official brand color tokens (keep in sync with `docs/brand.md` / `dold brand`). */
export const BRAND_TOKENS = {
  ink: "#12161A",
  teal: "#0F6B5C",
  paper: "#F7F4EF",
  paperAlt: "#F7F5F2",
} as const;

/** Maintainer thanks.dev URL (GitHub login path). */
export const THANKS_DEV_URL = "https://thanks.dev/u/gh/theworker02";

/** GitHub Pages funding page. */
export const FUNDING_PAGE_URL = "https://doldskrift.github.io/doldskrift/funding.html";

/** Agent-friendly funding payload (`doldskrift.funding/1`). */
export function fundingInfo(): {
  schema: string;
  thanksDev: string;
  fundingPage: string;
  githubLogin: string;
  note: string;
} {
  return {
    schema: "doldskrift.funding/1",
    thanksDev: THANKS_DEV_URL,
    fundingPage: FUNDING_PAGE_URL,
    githubLogin: "theworker02",
    note: "Open-source sustainability — Open/Neural ≠ encryption.",
  };
}

export function modeToU8(mode: Mode): number {
  switch (mode) {
    case "visual":
      return 0;
    case "encoded":
      return 1;
    case "session":
      return 2;
  }
}

export function modeFromU8(value: number): Mode | null {
  switch (value) {
    case 0:
      return "visual";
    case 1:
      return "encoded";
    case 2:
      return "session";
    default:
      return null;
  }
}

export function parseMode(s: string): Mode | null {
  switch (s.trim().toLowerCase()) {
    case "visual":
    case "v":
      return "visual";
    case "encoded":
    case "e":
      return "encoded";
    case "session":
    case "s":
      return "session";
    default:
      return null;
  }
}

export function isSymbol(ch: string): boolean {
  const cp = ch.codePointAt(0);
  if (cp === undefined) return false;
  return cp >= PUA_BASE && cp < PUA_BASE + ALPHABET_SIZE;
}

export function byteToSymbol(byte: number): string {
  return String.fromCodePoint(PUA_BASE + (byte & 0xff));
}

export function symbolToByte(ch: string): number | null {
  const cp = ch.codePointAt(0);
  if (cp === undefined) return null;
  if (cp >= PUA_BASE && cp < PUA_BASE + ALPHABET_SIZE) {
    return cp - PUA_BASE;
  }
  return null;
}

export class DoldskriftError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "DoldskriftError";
  }
}
