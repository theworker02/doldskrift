import { Mapping } from "./mapping.js";
import { DoldskriftError } from "./types.js";

/**
 * Streaming encoder: push UTF-8 chunks, pull encoded PUA text.
 * Streaming encode is byte-oriented by design.
 */
export class StreamingEncoder {
  private readonly mapping: Mapping;
  private pending: number[] = [];

  constructor(mapping: Mapping = Mapping.identity()) {
    this.mapping = mapping;
  }

  static withMapping(mapping: Mapping): StreamingEncoder {
    return new StreamingEncoder(mapping);
  }

  /** Push a chunk of UTF-8 bytes (may be mid-character). */
  push(chunk: Uint8Array): string {
    let out = "";
    for (const b of chunk) {
      out += this.mapping.encodeByte(b);
    }
    // Match Rust: extend pending then encode all and clear.
    // The Rust impl clears pending after encoding; pending is unused across calls
    // when encoding immediately. Keep API parity.
    this.pending = [];
    return out;
  }

  pushStr(chunk: string): string {
    return this.push(new TextEncoder().encode(chunk));
  }

  finish(): string {
    if (this.pending.length === 0) return "";
    let out = "";
    for (const b of this.pending) {
      out += this.mapping.encodeByte(b);
    }
    this.pending = [];
    return out;
  }
}

/**
 * Streaming decoder: push encoded PUA text, pull decoded UTF-8.
 */
export class StreamingDecoder {
  private readonly mapping: Mapping;
  private byteBuf: number[] = [];

  constructor(mapping: Mapping = Mapping.identity()) {
    this.mapping = mapping;
  }

  static withMapping(mapping: Mapping): StreamingDecoder {
    return new StreamingDecoder(mapping);
  }

  push(encodedChunk: string): string {
    for (const ch of encodedChunk) {
      this.byteBuf.push(this.mapping.decodeChar(ch));
    }
    return flushUtf8Prefix(this.byteBuf);
  }

  finish(): string {
    if (this.byteBuf.length === 0) return "";
    try {
      const out = new TextDecoder("utf-8", { fatal: true }).decode(
        Uint8Array.from(this.byteBuf),
      );
      this.byteBuf = [];
      return out;
    } catch {
      throw new DoldskriftError("invalid UTF-8 in stream finish");
    }
  }
}

/** Split off the longest valid UTF-8 prefix from `buf` (mutates buf). */
function flushUtf8Prefix(buf: number[]): string {
  const bytes = Uint8Array.from(buf);
  try {
    const out = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
    buf.length = 0;
    return out;
  } catch {
    // Find longest valid prefix by probing.
    let validUpTo = 0;
    for (let i = 1; i <= bytes.length; i++) {
      try {
        new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(0, i));
        validUpTo = i;
      } catch {
        // incomplete or invalid at i — keep last good
        const first = bytes[0]!;
        let need = 1;
        if (first >= 0xf0) need = 4;
        else if (first >= 0xe0) need = 3;
        else if (first >= 0xc0) need = 2;
        if (bytes.length < need) {
          // incomplete sequence — wait for more
          return "";
        }
        // hard invalid
        if (validUpTo === 0) {
          throw new DoldskriftError("invalid UTF-8 in stream");
        }
        break;
      }
    }
    if (validUpTo === 0) {
      // Incomplete multi-byte sequence at start
      const first = bytes[0]!;
      let need = 1;
      if (first >= 0xf0) need = 4;
      else if (first >= 0xe0) need = 3;
      else if (first >= 0xc0) need = 2;
      if (bytes.length < need) return "";
      throw new DoldskriftError("invalid UTF-8 in stream");
    }
    const out = new TextDecoder("utf-8", { fatal: true }).decode(
      bytes.subarray(0, validUpTo),
    );
    const rest = Array.from(bytes.subarray(validUpTo));
    buf.length = 0;
    buf.push(...rest);
    return out;
  }
}
