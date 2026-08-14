import { decode, encode } from "./codec.js";
import { checksumBytes, verifyChecksum } from "./crc32c.js";
import { Session } from "./session.js";
import {
  DoldskriftError,
  FONT_VERSION,
  MAGIC,
  PROTOCOL_VERSION,
  modeFromU8,
  modeToU8,
  type DocumentHeader,
  type InspectReport,
  type Mode,
  type SessionManifest,
} from "./types.js";

const FIXED_HEADER_LEN = 20;
const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder("utf-8", { fatal: true });

function headerToJsonBytes(header: DocumentHeader): Uint8Array {
  const forSer: Record<string, unknown> = {
    version: header.version,
    mode: header.mode,
    flags: header.flags,
    checksum: 0,
  };
  if (header.mapping_id !== undefined) forSer.mapping_id = header.mapping_id;
  if (header.session !== undefined) forSer.session = header.session;
  if (header.char_count !== undefined) forSer.char_count = header.char_count;
  if (header.byte_count !== undefined) forSer.byte_count = header.byte_count;
  forSer.font_version = header.font_version;
  if (header.meta && Object.keys(header.meta).length > 0) {
    forSer.meta = header.meta;
  }
  return textEncoder.encode(JSON.stringify(forSer));
}

function headerFromJsonBytes(bytes: Uint8Array): DocumentHeader {
  const obj = JSON.parse(textDecoder.decode(bytes)) as DocumentHeader;
  return {
    version: obj.version ?? PROTOCOL_VERSION,
    mode: obj.mode,
    flags: obj.flags ?? 0,
    checksum: obj.checksum ?? 0,
    mapping_id: obj.mapping_id,
    session: obj.session,
    char_count: obj.char_count,
    byte_count: obj.byte_count,
    font_version: obj.font_version ?? FONT_VERSION,
    meta: obj.meta ?? {},
  };
}

function newHeader(mode: Mode): DocumentHeader {
  return {
    version: PROTOCOL_VERSION,
    mode,
    flags: 0,
    checksum: 0,
    font_version: FONT_VERSION,
    meta: {},
  };
}

function concatBytes(a: Uint8Array, b: Uint8Array): Uint8Array {
  const out = new Uint8Array(a.length + b.length);
  out.set(a, 0);
  out.set(b, a.length);
  return out;
}

function readU32BE(bytes: Uint8Array, offset: number): number {
  return (
    ((bytes[offset]! << 24) |
      (bytes[offset + 1]! << 16) |
      (bytes[offset + 2]! << 8) |
      bytes[offset + 3]!) >>>
    0
  );
}

function writeU32BE(value: number): Uint8Array {
  return Uint8Array.of(
    (value >>> 24) & 0xff,
    (value >>> 16) & 0xff,
    (value >>> 8) & 0xff,
    value & 0xff,
  );
}

function inspectFromDocument(doc: DskDocument): InspectReport {
  let mapping: string;
  switch (doc.header.mode) {
    case "visual":
      mapping = "none (visual)";
      break;
    case "encoded":
      mapping = doc.header.mapping_id ?? "static-v1";
      break;
    case "session":
      mapping = "dynamic";
      break;
  }
  return {
    protocol: `DSK/${doc.header.version}`,
    mode: doc.header.mode,
    payload_bytes: doc.payload.length,
    characters: doc.header.char_count ?? null,
    mapping,
    checksum_valid: true,
    checksum: doc.header.checksum,
    font_version: doc.header.font_version,
    session_id: doc.header.session?.session_id ?? null,
  };
}

/** An in-memory Doldskrift document (`.dsk`). */
export class DskDocument {
  header: DocumentHeader;
  payload: Uint8Array;

  constructor(header: DocumentHeader, payload: Uint8Array) {
    this.header = header;
    this.payload = payload;
  }

  /** Build a document by encoding `text` in `mode`. */
  static encodeText(text: string, mode: Mode, session?: Session): DskDocument {
    if (mode === "session") {
      if (!session) {
        throw new DoldskriftError("use DskDocument.encodeText with a Session for session mode");
      }
      return DskDocument.encodeSession(text, session);
    }
    if (mode === "visual") {
      return DskDocument.fromParts(newHeader(mode), textEncoder.encode(text));
    }
    const encoded = encode(text);
    const header = newHeader(mode);
    header.char_count = [...text].length;
    header.byte_count = textEncoder.encode(text).length;
    return DskDocument.fromParts(header, textEncoder.encode(encoded));
  }

  /** Build a session-mode document. */
  static encodeSession(text: string, session: Session): DskDocument {
    const encoded = session.encode(text);
    const header = newHeader("session");
    header.char_count = [...text].length;
    header.byte_count = textEncoder.encode(text).length;
    header.session = { ...session.manifest };
    header.mapping_id = session.mappingId();
    return DskDocument.fromParts(header, textEncoder.encode(encoded));
  }

  /** Construct from header + payload (computes checksum). */
  static fromParts(header: DocumentHeader, payload: Uint8Array): DskDocument {
    const h: DocumentHeader = { ...header, version: PROTOCOL_VERSION };
    const headerBytes = headerToJsonBytes(h);
    const covered = concatBytes(headerBytes, payload);
    h.checksum = checksumBytes(covered);
    return new DskDocument(h, payload);
  }

  /** Parse a `.dsk` blob. */
  static parse(bytes: Uint8Array): DskDocument {
    if (bytes.length < FIXED_HEADER_LEN) {
      throw new DoldskriftError(
        `truncated payload: needed ${FIXED_HEADER_LEN - bytes.length} more bytes`,
      );
    }
    if (
      bytes[0] !== MAGIC[0] ||
      bytes[1] !== MAGIC[1] ||
      bytes[2] !== MAGIC[2] ||
      bytes[3] !== MAGIC[3]
    ) {
      throw new DoldskriftError("invalid header magic");
    }
    const version = bytes[4]!;
    if (version !== PROTOCOL_VERSION) {
      throw new DoldskriftError(`unsupported version ${version}`);
    }
    const mode = modeFromU8(bytes[5]!);
    if (!mode) {
      throw new DoldskriftError(`unsupported mode ${bytes[5]}`);
    }
    const flags = bytes[6]!;
    const headerLen = readU32BE(bytes, 8);
    const payloadLen = readU32BE(bytes, 12);
    const expectedChecksum = readU32BE(bytes, 16);

    const bodyStart = FIXED_HEADER_LEN;
    const headerEnd = bodyStart + headerLen;
    const payloadEnd = headerEnd + payloadLen;
    if (bytes.length < payloadEnd) {
      throw new DoldskriftError(
        `truncated payload: needed ${payloadEnd - bytes.length} more bytes`,
      );
    }
    if (bytes.length > payloadEnd) {
      throw new DoldskriftError(
        `trailing ${bytes.length - payloadEnd} bytes after payload`,
      );
    }

    const headerBytes = bytes.subarray(bodyStart, headerEnd);
    const payload = bytes.subarray(headerEnd, payloadEnd);
    const covered = concatBytes(headerBytes, payload);
    verifyChecksum(covered, expectedChecksum);

    let header: DocumentHeader =
      headerBytes.length === 0 ? newHeader(mode) : headerFromJsonBytes(headerBytes);
    header.version = version;
    header.mode = mode;
    header.flags = flags;
    header.checksum = expectedChecksum;

    return new DskDocument(header, new Uint8Array(payload));
  }

  /** Serialize to `.dsk` bytes. */
  toBytes(): Uint8Array {
    const headerBytes = headerToJsonBytes(this.header);
    const covered = concatBytes(headerBytes, this.payload);
    const checksum = checksumBytes(covered);

    const out = new Uint8Array(FIXED_HEADER_LEN + covered.length);
    out.set(MAGIC, 0);
    out[4] = this.header.version;
    out[5] = modeToU8(this.header.mode);
    out[6] = this.header.flags;
    out[7] = 0;
    out.set(writeU32BE(headerBytes.length), 8);
    out.set(writeU32BE(this.payload.length), 12);
    out.set(writeU32BE(checksum), 16);
    out.set(headerBytes, FIXED_HEADER_LEN);
    out.set(this.payload, FIXED_HEADER_LEN + headerBytes.length);
    return out;
  }

  /** Decode semantic text from the document. */
  decodeText(): string {
    switch (this.header.mode) {
      case "visual":
        return textDecoder.decode(this.payload);
      case "encoded": {
        const encoded = textDecoder.decode(this.payload);
        return decode(encoded);
      }
      case "session": {
        const encoded = textDecoder.decode(this.payload);
        return this.recoverSession().decode(encoded);
      }
    }
  }

  /** Recover a Session from header metadata when possible. */
  recoverSession(): Session {
    const manifest = this.header.session;
    if (!manifest) {
      throw new DoldskriftError("missing session manifest");
    }
    return Session.fromManifest(manifest as SessionManifest);
  }

  /** Produce an inspect report without decoding the payload. */
  inspect(): InspectReport {
    return inspectFromDocument(this);
  }

  payloadLen(): number {
    return this.payload.length;
  }
}
