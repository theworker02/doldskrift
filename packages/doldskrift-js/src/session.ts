import { checksumBytes, checksumHex } from "./crc32c.js";
import { Mapping } from "./mapping.js";
import {
  ALPHABET_SIZE,
  DoldskriftError,
  PROTOCOL_NAME,
  PROTOCOL_VERSION,
  type SessionManifest,
} from "./types.js";

function toBytes(seed: string | Uint8Array): Uint8Array {
  return typeof seed === "string" ? new TextEncoder().encode(seed) : seed;
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
}

function hexToBytes(hex: string): Uint8Array {
  if (hex.length % 2 !== 0) {
    throw new DoldskriftError(`bad seed_hex: odd length`);
  }
  const out = new Uint8Array(hex.length / 2);
  for (let i = 0; i < out.length; i++) {
    const byte = Number.parseInt(hex.slice(i * 2, i * 2 + 2), 16);
    if (Number.isNaN(byte)) {
      throw new DoldskriftError(`bad seed_hex: invalid hex`);
    }
    out[i] = byte;
  }
  return out;
}

export function validateSessionManifest(m: SessionManifest): void {
  if (m.protocol !== PROTOCOL_NAME) {
    throw new DoldskriftError(`unexpected protocol '${m.protocol}'`);
  }
  if (m.version !== PROTOCOL_VERSION) {
    throw new DoldskriftError(`unsupported version ${m.version}`);
  }
  if (m.mode !== "session") {
    throw new DoldskriftError(`expected mode 'session', got '${m.mode}'`);
  }
  if (!m.session_id || !m.mapping_id) {
    throw new DoldskriftError("session_id and mapping_id must be non-empty");
  }
}

export function sessionManifestToJson(m: SessionManifest): string {
  validateSessionManifest(m);
  return JSON.stringify(m);
}

export function sessionManifestFromJson(s: string): SessionManifest {
  const m = JSON.parse(s) as SessionManifest;
  validateSessionManifest(m);
  return m;
}

/** Builder for Session. */
export class SessionBuilder {
  private _seed: Uint8Array | null = null;
  private _sessionId: string | null = null;

  seed(seed: string | Uint8Array): this {
    this._seed = toBytes(seed);
    return this;
  }

  sessionId(id: string): this {
    this._sessionId = id;
    return this;
  }

  build(): Session {
    const seed = this._seed ?? new Uint8Array(0);
    const mapping = Mapping.fromSeed(seed);
    const sessionId =
      this._sessionId ?? `sess-${checksumBytes(seed).toString(16).padStart(8, "0")}`;
    const tableChecksum = checksumHex(mapping.encodeTable);
    const manifest: SessionManifest = {
      protocol: PROTOCOL_NAME,
      version: PROTOCOL_VERSION,
      mode: "session",
      session_id: sessionId,
      mapping_id: mapping.id,
      alphabet: `pua-byte-${ALPHABET_SIZE}`,
      checksum: tableChecksum,
      seed_hex: bytesToHex(seed),
    };
    validateSessionManifest(manifest);
    return Session.create(seed, sessionId, mapping, manifest);
  }
}

/** A live session binding a seed to a mapping. */
export class Session {
  private constructor(
    readonly seed: Uint8Array,
    readonly sessionId: string,
    readonly mapping: Mapping,
    readonly manifest: SessionManifest,
  ) {}

  /** @internal */
  static create(
    seed: Uint8Array,
    sessionId: string,
    mapping: Mapping,
    manifest: SessionManifest,
  ): Session {
    return new Session(seed, sessionId, mapping, manifest);
  }

  static builder(): SessionBuilder {
    return new SessionBuilder();
  }

  static fromSeed(seed: string | Uint8Array, sessionId: string): Session {
    return Session.builder().seed(seed).sessionId(sessionId).build();
  }

  static fromManifest(manifest: SessionManifest): Session {
    validateSessionManifest(manifest);
    if (!manifest.seed_hex) {
      throw new DoldskriftError("missing seed_hex in manifest");
    }
    return Session.fromSeed(hexToBytes(manifest.seed_hex), manifest.session_id);
  }

  mappingId(): string {
    return this.mapping.id;
  }

  encode(text: string): string {
    return this.mapping.encodeStr(text);
  }

  decode(encoded: string): string {
    return this.mapping.decodeStr(encoded);
  }
}

export type { SessionManifest };
