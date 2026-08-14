import { Mapping } from "./mapping.js";
import { isSymbol, type Mode } from "./types.js";

/** Encode `text` with the fixed DOLDSKRIFT/1 identity mapping. */
export function encode(text: string): string {
  return Mapping.identity().encodeStr(text);
}

/** Decode PUA-encoded `text` with the fixed DOLDSKRIFT/1 identity mapping. */
export function decode(encoded: string): string {
  return Mapping.identity().decodeStr(encoded);
}

/** Idiomatic encoder bound to a mode (and optional mapping). */
export class Encoder {
  readonly mode: Mode;
  readonly mapping: Mapping;

  constructor(mode: Mode, mapping: Mapping = Mapping.identity()) {
    this.mode = mode;
    this.mapping = mapping;
  }

  static withMapping(mode: Mode, mapping: Mapping): Encoder {
    return new Encoder(mode, mapping);
  }

  encode(text: string): string {
    switch (this.mode) {
      case "visual":
        return text;
      case "encoded":
      case "session":
        return this.mapping.encodeStr(text);
    }
  }
}

/** Idiomatic decoder. */
export class Decoder {
  readonly mapping: Mapping;

  constructor(mapping: Mapping = Mapping.identity()) {
    this.mapping = mapping;
  }

  static withMapping(mapping: Mapping): Decoder {
    return new Decoder(mapping);
  }

  /**
   * Decode representation text back to semantic UTF-8.
   * If the input contains no PUA symbols, it is returned unchanged.
   */
  decode(payload: string): string {
    let hasSymbol = false;
    for (const ch of payload) {
      if (isSymbol(ch)) {
        hasSymbol = true;
        break;
      }
    }
    if (!hasSymbol) return payload;
    return this.mapping.decodeStr(payload);
  }
}
