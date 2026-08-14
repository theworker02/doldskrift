//! Property tests for DOLDSKRIFT/1 invariants.

use doldskrift::{decode, encode, Mapping, Session, StreamingDecoder, StreamingEncoder};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    #[test]
    fn decode_encode_identity(s in "\\PC*") {
        // Any UTF-8 string (proptest String strategy) round-trips.
        let encoded = encode(&s).unwrap();
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, s);
    }

    #[test]
    fn stream_roundtrip(s in "\\PC{0,200}") {
        let mut enc = StreamingEncoder::new();
        let mut encoded = String::new();
        for chunk in s.as_bytes().chunks(5) {
            encoded.push_str(&enc.push(chunk));
        }
        encoded.push_str(&enc.finish().unwrap());

        let mut dec = StreamingDecoder::new();
        let mut decoded = String::new();
        for ch in encoded.chars() {
            let mut tmp = String::new();
            tmp.push(ch);
            decoded.push_str(&dec.push(&tmp).unwrap());
        }
        decoded.push_str(&dec.finish().unwrap());
        prop_assert_eq!(decoded, s);
    }

    #[test]
    fn session_seed_deterministic(seed in prop::collection::vec(any::<u8>(), 0..64)) {
        let a = Mapping::from_seed(&seed).unwrap();
        let b = Mapping::from_seed(&seed).unwrap();
        prop_assert_eq!(a.encode_table(), b.encode_table());
        prop_assert_eq!(a.id(), b.id());
    }

    #[test]
    fn session_roundtrip(seed in prop::collection::vec(any::<u8>(), 0..32), s in "\\PC{0,120}") {
        let session = Session::from_seed(&seed, "prop").unwrap();
        let encoded = session.encode(&s).unwrap();
        prop_assert_eq!(session.decode(&encoded).unwrap(), s);
    }
}
