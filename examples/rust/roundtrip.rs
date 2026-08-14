//! Minimal Rust example: encode → inspect metadata → decode.

fn main() -> doldskrift::Result<()> {
    let text = "Hello from Doldskrift";
    let doc = doldskrift::DskDocument::encode_text(text, doldskrift::Mode::Encoded)?;
    println!("{}", doc.inspect().display());
    assert_eq!(doc.decode_text()?, text);
    println!("roundtrip ok");
    Ok(())
}
