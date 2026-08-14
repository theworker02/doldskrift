export { encode, decode, Encoder, Decoder } from "./codec.js";
export { crc32c, checksumBytes, checksumHex, verifyChecksum } from "./crc32c.js";
export { Mapping, identitySymbol, mixSeed, splitmix64, mappingIdFromSeed, mappingIdStaticV1 } from "./mapping.js";
export { Session, SessionBuilder, validateSessionManifest, sessionManifestToJson, sessionManifestFromJson } from "./session.js";
export { DskDocument } from "./document.js";
export {
  detectDoldskrift,
  detectFromHtmlMeta,
  detectFromAttrs,
  detectPuaHeuristic,
} from "./discovery.js";
export type { DetectInput } from "./discovery.js";
export { StreamingEncoder, StreamingDecoder } from "./stream.js";
export {
  renderDoldskrift,
  encodeDoldskrift,
  decodeDoldskrift,
} from "./web.js";
export {
  PUA_BASE,
  MIME_BINARY,
  MIME_TEXT,
  PROTOCOL_NAME,
  PROTOCOL_VERSION,
  PROTOCOL_VERSION_PROTECTED,
  ALPHABET_SIZE,
  FONT_VERSION,
  GLYPH_ENGINE_VERSION,
  MAGIC,
  MAGIC_DSK2,
  HANDSHAKE_QUERY,
  HANDSHAKE_REPLY,
  DSK_PROJECT_MARK,
  CHECKSUM_ALGORITHM,
  BRAND_TOKENS,
  THANKS_DEV_URL,
  FUNDING_PAGE_URL,
  DoldskriftError,
  isSymbol,
  isProjectMark,
  fundingInfo,
  byteToSymbol,
  symbolToByte,
  parseMode,
  modeToU8,
  modeFromU8,
} from "./types.js";
export type {
  Mode,
  Detection,
  DetectionSource,
  SessionManifest,
  DocumentHeader,
  InspectReport,
} from "./types.js";
