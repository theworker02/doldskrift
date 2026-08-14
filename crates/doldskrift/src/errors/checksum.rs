//! Checksum helpers (CRC32C).
//!
//! Used for **accidental corruption detection**, not authenticity.

/// Algorithm identifier recorded in manifests / docs.
pub const CHECKSUM_ALGORITHM: &str = "crc32c";

/// Compute CRC32C (Castagnoli) over `data`.
#[inline]
pub fn checksum_bytes(data: &[u8]) -> u32 {
    crc32c::crc32c(data)
}

/// Verify that `expected` matches CRC32C(`data`).
pub fn verify_checksum(data: &[u8], expected: u32) -> Result<(), crate::Error> {
    let actual = checksum_bytes(data);
    if actual == expected {
        Ok(())
    } else {
        Err(crate::Error::ChecksumMismatch { expected, actual })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_checksum_stable() {
        assert_eq!(checksum_bytes(b""), 0);
    }

    #[test]
    fn known_vector() {
        // CRC32C("123456789") == 0xE3069283
        assert_eq!(checksum_bytes(b"123456789"), 0xE306_9283);
    }
}
