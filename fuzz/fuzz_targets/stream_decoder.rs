#![no_main]
use libfuzzer_sys::fuzz_target;
use doldskrift::StreamingDecoder;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut dec = StreamingDecoder::new();
        let _ = dec.push(s);
        let _ = dec.finish();
    }
});
