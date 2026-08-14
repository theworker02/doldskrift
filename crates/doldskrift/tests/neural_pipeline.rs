//! Neural foundation pipeline tests (Mock / DeterministicBaseline).
//!
//! No multi-GB weights. Exact SHA-256 match on semantic round-trip.

use doldskrift::neural::{
    compose_deterministic, DeterministicBaseline, EpochLockedReader, GrammarEpoch, MockReader,
    ObservationGraph, ReaderBackend, ReaderStatus,
};
use doldskrift::semantic::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn sample_value() -> Value {
    let mut map = BTreeMap::new();
    map.insert("msg".into(), Value::String("hello neural".into()));
    map.insert("n".into(), Value::Integer(42));
    map.insert("ok".into(), Value::Bool(true));
    map.insert(
        "tags".into(),
        Value::Array(vec![Value::String("a".into()), Value::String("a".into())]),
    );
    Value::Map(map)
}

#[test]
fn deterministic_pipeline_exact_sha256() {
    let value = sample_value();
    let canonical = doldskrift::semantic::encode_value(&value, true).unwrap();
    let expected = sha256_hex(&canonical);

    let latent = compose_deterministic(&value).unwrap();
    assert_eq!(latent.meta.epoch, GrammarEpoch::E0001);

    // Foundation "render → observe" stand-in: carrier observation (not camera).
    let obs = ObservationGraph::from_latent_carrier(latent);
    assert!(obs.is_doldskrift);

    let reader = DeterministicBaseline::new();
    let out = reader.reconstruct(&obs).unwrap();
    assert_eq!(out.status, ReaderStatus::Reconstructed);
    let got = out.value.expect("value");
    let got_bytes = doldskrift::semantic::encode_value(&got, true).unwrap();
    assert_eq!(sha256_hex(&got_bytes), expected);
    assert_eq!(got, value);
}

#[test]
fn mock_reader_round_trip() {
    let value = Value::String("mock-path".into());
    let latent = compose_deterministic(&value).unwrap();
    let obs = ObservationGraph::from_latent_carrier(latent);
    let out = MockReader::new().reconstruct(&obs).unwrap();
    assert!(out.ok());
    assert_eq!(out.value, Some(value));
}

#[test]
fn wrong_reader_incompatible() {
    let value = Value::Integer(1);
    let latent = compose_deterministic(&value).unwrap();
    let obs = ObservationGraph::from_latent_carrier(latent);
    let reader = EpochLockedReader::only(GrammarEpoch { number: 99 });
    let out = reader.reconstruct(&obs).unwrap();
    assert_eq!(out.status, ReaderStatus::Incompatible);
    assert!(out.value.is_none());
}

#[test]
fn random_input_not_doldskrift() {
    let obs = ObservationGraph::not_doldskrift();
    let out = DeterministicBaseline::new().reconstruct(&obs).unwrap();
    assert_eq!(out.status, ReaderStatus::NotDoldskrift);
    assert!(out.value.is_none());
}

#[test]
fn grammar_epoch_parse_label() {
    let e = GrammarEpoch::parse("LSG/1-E0001").unwrap();
    assert_eq!(e.number, 1);
    assert_eq!(e.label(), "LSG/1-E0001");
}

#[test]
fn golden_fixture_round_trip() {
    let raw = include_str!("goldens/neural_e0001_hello.json");
    let v: serde_json::Value = serde_json::from_str(raw).unwrap();
    let latent: doldskrift::neural::LatentGraph =
        serde_json::from_value(v["latent"].clone()).unwrap();
    assert_eq!(latent.meta.epoch, GrammarEpoch::E0001);
    let obs = ObservationGraph::from_latent_carrier(latent);
    let out = DeterministicBaseline::new().reconstruct(&obs).unwrap();
    assert_eq!(out.status, ReaderStatus::Reconstructed);
    let got = out.value.expect("value");
    // Fixture message was "hello-neural:1"
    assert_eq!(got, Value::String("hello-neural:1".into()));
}
