/**
 * Thin browser adapter — prefer `@doldskrift/core` directly.
 * Kept for backward-compatible imports and CSS subpath.
 */
export {
  decode,
  encode,
  detectDoldskrift,
  Decoder,
  Encoder,
  Session,
  DskDocument,
  PUA_BASE,
  MIME_BINARY,
  MIME_TEXT,
  BRAND_TOKENS,
  THANKS_DEV_URL,
  FUNDING_PAGE_URL,
  fundingInfo,
  DSK_PROJECT_MARK,
  renderDoldskrift,
  encodeDoldskrift,
  decodeDoldskrift,
} from "@doldskrift/core";
