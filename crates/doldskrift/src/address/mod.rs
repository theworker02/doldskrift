//! Content addressing for canonical Doldskrift payloads.

use sha2::{Digest, Sha256};

/// Compute a stable content id: `dsk-sha256:<hex>`.
pub fn dsk_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("dsk-sha256:{}", hex::encode(digest))
}

/// Raw SHA-256 digest (32 bytes).
pub fn sha256_bytes(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_id_stable() {
        let a = dsk_sha256(b"hello");
        let b = dsk_sha256(b"hello");
        assert_eq!(a, b);
        assert!(a.starts_with("dsk-sha256:"));
    }
}
