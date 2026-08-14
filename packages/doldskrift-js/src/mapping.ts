import { checksumBytes } from "./crc32c.js";
import {
  ALPHABET_SIZE,
  DoldskriftError,
  PUA_BASE,
  PROTOCOL_VERSION,
  byteToSymbol,
  symbolToByte,
} from "./types.js";

const MASK64 = 0xffffffffffffffffn;

/** SplitMix64 — must match Rust doldskrift-codec exactly (wrapping u64). */
export function splitmix64(x: bigint): bigint {
  x = (x + 0x9e3779b97f4a7c15n) & MASK64;
  let z = x;
  z = ((z ^ (z >> 30n)) * 0xbf58476d1ce4e5b9n) & MASK64;
  z = ((z ^ (z >> 27n)) * 0x94d049bb133111ebn) & MASK64;
  return (z ^ (z >> 31n)) & MASK64;
}

/** Mix arbitrary seed bytes into a u64 state (little-endian chunks). */
export function mixSeed(seed: Uint8Array): bigint {
  let state = 0x243f6a8885a308d3n;
  if (seed.length === 0) {
    return splitmix64(state);
  }
  for (let offset = 0; offset < seed.length; offset += 8) {
    const end = Math.min(offset + 8, seed.length);
    let n = 0n;
    for (let i = 0; i < end - offset; i++) {
      n |= BigInt(seed[offset + i]!) << BigInt(8 * i);
    }
    state = (state ^ n) & MASK64;
    state = splitmix64(state);
  }
  return state;
}

export function mappingIdStaticV1(): string {
  return `static-v${PROTOCOL_VERSION}`;
}

export function mappingIdFromSeed(seed: Uint8Array): string {
  const c = checksumBytes(seed);
  return `session-v${PROTOCOL_VERSION}-${c.toString(16).padStart(8, "0")}`;
}

/** Reversible 256-entry byte permutation used for encoding. */
export class Mapping {
  readonly encodeTable: Uint8Array;
  readonly decodeTable: Uint8Array;
  readonly id: string;

  private constructor(encodeTable: Uint8Array, decodeTable: Uint8Array, id: string) {
    this.encodeTable = encodeTable;
    this.decodeTable = decodeTable;
    this.id = id;
  }

  /** Identity mapping: byte `b` → `U+E000 + b`. */
  static identity(): Mapping {
    const encodeTable = new Uint8Array(ALPHABET_SIZE);
    const decodeTable = new Uint8Array(ALPHABET_SIZE);
    for (let i = 0; i < ALPHABET_SIZE; i++) {
      encodeTable[i] = i;
      decodeTable[i] = i;
    }
    return new Mapping(encodeTable, decodeTable, mappingIdStaticV1());
  }

  /** Build from an explicit encode table (must be a permutation). */
  static fromEncodeTable(encodeTable: Uint8Array, id: string): Mapping {
    if (encodeTable.length !== ALPHABET_SIZE) {
      throw new DoldskriftError("encode table must have 256 entries");
    }
    const decodeTable = new Uint8Array(ALPHABET_SIZE);
    const seen = new Uint8Array(ALPHABET_SIZE);
    for (let byte = 0; byte < ALPHABET_SIZE; byte++) {
      const sym = encodeTable[byte]!;
      if (seen[sym]) {
        throw new DoldskriftError(`symbol offset ${sym} assigned twice`);
      }
      seen[sym] = 1;
      decodeTable[sym] = byte;
    }
    for (let i = 0; i < ALPHABET_SIZE; i++) {
      if (!seen[i]) {
        throw new DoldskriftError("encode table is not a complete permutation");
      }
    }
    return new Mapping(new Uint8Array(encodeTable), decodeTable, id);
  }

  /**
   * Deterministic Fisher–Yates shuffle of 0..255 using SplitMix64.
   * Not a cryptographic KDF — representation / obfuscation only.
   */
  static fromSeed(seed: Uint8Array): Mapping {
    const id = mappingIdFromSeed(seed);
    let state = mixSeed(seed);
    const encodeTable = new Uint8Array(ALPHABET_SIZE);
    for (let i = 0; i < ALPHABET_SIZE; i++) {
      encodeTable[i] = i;
    }
    for (let i = ALPHABET_SIZE - 1; i >= 1; i--) {
      state = splitmix64(state);
      const j = Number(state % BigInt(i + 1));
      const tmp = encodeTable[i]!;
      encodeTable[i] = encodeTable[j]!;
      encodeTable[j] = tmp;
    }
    return Mapping.fromEncodeTable(encodeTable, id);
  }

  encodeByte(byte: number): string {
    const offset = this.encodeTable[byte & 0xff]!;
    return String.fromCodePoint(PUA_BASE + offset);
  }

  decodeChar(ch: string): number {
    const offset = symbolToByte(ch);
    if (offset === null) {
      const cp = ch.codePointAt(0) ?? 0;
      throw new DoldskriftError(`invalid codepoint U+${cp.toString(16)}`);
    }
    return this.decodeTable[offset]!;
  }

  encodeStr(text: string): string {
    const bytes = new TextEncoder().encode(text);
    let out = "";
    for (const b of bytes) {
      out += this.encodeByte(b);
    }
    return out;
  }

  decodeStr(encoded: string): string {
    const bytes: number[] = [];
    for (const ch of encoded) {
      bytes.push(this.decodeChar(ch));
    }
    return new TextDecoder("utf-8", { fatal: true }).decode(Uint8Array.from(bytes));
  }
}

/** Identity symbol for a byte (font tooling / tests). */
export function identitySymbol(byte: number): string {
  return byteToSymbol(byte);
}

export type StaticMapping = Mapping;
