/**
 * Pure JS CRC32C (Castagnoli) — matches Rust `crc32c` / `doldskrift_core::checksum_bytes`.
 * Polynomial (reflected): 0x82F63B78
 */

const TABLE = (() => {
  const table = new Uint32Array(256);
  for (let i = 0; i < 256; i++) {
    let crc = i;
    for (let j = 0; j < 8; j++) {
      crc = crc & 1 ? (0x82f63b78 ^ (crc >>> 1)) : crc >>> 1;
    }
    table[i] = crc >>> 0;
  }
  return table;
})();

/** Compute CRC32C over `data`. Empty input returns 0. */
export function crc32c(data: Uint8Array): number {
  let crc = 0xffffffff;
  for (let i = 0; i < data.length; i++) {
    crc = TABLE[(crc ^ data[i]!) & 0xff]! ^ (crc >>> 8);
  }
  return (crc ^ 0xffffffff) >>> 0;
}

/** Alias matching Rust `checksum_bytes`. */
export function checksumBytes(data: Uint8Array): number {
  return crc32c(data);
}

/** Verify expected CRC32C. */
export function verifyChecksum(data: Uint8Array, expected: number): void {
  const actual = crc32c(data);
  if (actual !== expected) {
    throw new Error(
      `checksum mismatch: expected 0x${expected.toString(16).padStart(8, "0")}, got 0x${actual.toString(16).padStart(8, "0")}`,
    );
  }
}

/** Format as 8 lowercase hex digits. */
export function checksumHex(data: Uint8Array): string {
  return crc32c(data).toString(16).padStart(8, "0");
}
