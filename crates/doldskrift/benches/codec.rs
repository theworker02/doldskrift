//! Criterion benches for encode/decode/stream/container/mapping.
//!
//! Glyph generation lives in `doldskrift-font` — bench that crate separately if needed.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use doldskrift::{decode, encode, DskDocument, Mapping, Mode, StreamingDecoder, StreamingEncoder};

fn payload(size: usize) -> String {
    "Hello from Doldskrift — agent channel 0123456789\n"
        .chars()
        .cycle()
        .take(size)
        .collect()
}

fn bench_codec(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_decode");
    for &size in &[1024usize, 100 * 1024, 1024 * 1024] {
        let text = payload(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("encode", size), &text, |b, t| {
            b.iter(|| encode(black_box(t)).unwrap())
        });
        let encoded = encode(&text).unwrap();
        group.bench_with_input(BenchmarkId::new("decode", size), &encoded, |b, e| {
            b.iter(|| decode(black_box(e)).unwrap())
        });
    }
    group.finish();
}

fn bench_stream(c: &mut Criterion) {
    let text = payload(100 * 1024);
    c.bench_function("stream_encode_100kb", |b| {
        b.iter(|| {
            let mut enc = StreamingEncoder::new();
            let mut out = String::new();
            for chunk in text.as_bytes().chunks(1024) {
                out.push_str(&enc.push(black_box(chunk)));
            }
            out.push_str(&enc.finish().unwrap());
            out
        })
    });
    let encoded = encode(&text).unwrap();
    c.bench_function("stream_decode_100kb", |b| {
        b.iter(|| {
            let mut dec = StreamingDecoder::new();
            let mut out = String::new();
            for chunk in encoded.as_bytes().chunks(1024) {
                let s = std::str::from_utf8(chunk).unwrap();
                out.push_str(&dec.push(black_box(s)).unwrap());
            }
            out.push_str(&dec.finish().unwrap());
            out
        })
    });
}

fn bench_container(c: &mut Criterion) {
    let text = payload(100 * 1024);
    let doc = DskDocument::encode_text(&text, Mode::Encoded).unwrap();
    let bytes = doc.to_bytes().unwrap();
    c.bench_function("container_parse_100kb", |b| {
        b.iter(|| DskDocument::parse(black_box(&bytes)).unwrap())
    });
}

fn bench_mapping(c: &mut Criterion) {
    c.bench_function("mapping_from_seed", |b| {
        b.iter(|| Mapping::from_seed(black_box(b"benchmark-seed-001")).unwrap())
    });
}

criterion_group!(
    benches,
    bench_codec,
    bench_stream,
    bench_container,
    bench_mapping
);
criterion_main!(benches);
