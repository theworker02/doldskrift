//! Typed glyph / semantic token stream (DSK/2).

use super::Value;
use crate::{Error, Result};

/// High-level token kinds (control plane).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenKind {
    /// Null value.
    Null = 0x00,
    /// Boolean false.
    BoolFalse = 0x01,
    /// Boolean true.
    BoolTrue = 0x02,
    /// Signed integer.
    Integer = 0x03,
    /// Floating-point.
    Float = 0x04,
    /// UTF-8 string.
    String = 0x05,
    /// Raw bytes.
    Bytes = 0x06,
    /// Begin array.
    ArrayStart = 0x07,
    /// End array.
    ArrayEnd = 0x08,
    /// Begin map.
    MapStart = 0x09,
    /// End map.
    MapEnd = 0x0A,
    /// Timestamp.
    Timestamp = 0x0B,
    /// UUID.
    Uuid = 0x0C,
    /// URI.
    Uri = 0x0D,
    /// Identifier.
    Identifier = 0x0E,
    /// End of stream / document.
    End = 0xFF,
}

/// A semantic token with optional payload.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticToken {
    /// Kind tag.
    pub kind: TokenKind,
    /// Optional payload bytes.
    pub payload: Vec<u8>,
}

fn write_uvarint(out: &mut Vec<u8>, mut n: u64) {
    loop {
        let mut b = (n & 0x7f) as u8;
        n >>= 7;
        if n != 0 {
            b |= 0x80;
        }
        out.push(b);
        if n == 0 {
            break;
        }
    }
}

fn read_uvarint(input: &[u8], i: &mut usize) -> Result<u64> {
    let mut result = 0u64;
    let mut shift = 0u32;
    loop {
        if *i >= input.len() {
            return Err(Error::TruncatedPayload { needed: 1 });
        }
        let b = input[*i];
        *i += 1;
        result |= u64::from(b & 0x7f) << shift;
        if b & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(Error::Semantic("uvarint overflow".into()));
        }
    }
}

/// Encode a value as a compact token byte stream.
pub fn encode_tokens(value: &Value) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    encode_into(value, &mut out)?;
    out.push(TokenKind::End as u8);
    Ok(out)
}

fn encode_into(value: &Value, out: &mut Vec<u8>) -> Result<()> {
    match value {
        Value::Null => out.push(TokenKind::Null as u8),
        Value::Bool(false) => out.push(TokenKind::BoolFalse as u8),
        Value::Bool(true) => out.push(TokenKind::BoolTrue as u8),
        Value::Integer(i) => {
            out.push(TokenKind::Integer as u8);
            out.extend_from_slice(&i.to_be_bytes());
        }
        Value::Float(f) => {
            out.push(TokenKind::Float as u8);
            out.extend_from_slice(&f.to_bits().to_be_bytes());
        }
        Value::String(s) => {
            out.push(TokenKind::String as u8);
            write_uvarint(out, s.len() as u64);
            out.extend_from_slice(s.as_bytes());
        }
        Value::Bytes(b) => {
            out.push(TokenKind::Bytes as u8);
            write_uvarint(out, b.len() as u64);
            out.extend_from_slice(b);
        }
        Value::Array(a) => {
            out.push(TokenKind::ArrayStart as u8);
            write_uvarint(out, a.len() as u64);
            for v in a {
                encode_into(v, out)?;
            }
            out.push(TokenKind::ArrayEnd as u8);
        }
        Value::Map(m) => {
            out.push(TokenKind::MapStart as u8);
            write_uvarint(out, m.len() as u64);
            for (k, v) in m {
                // key as string
                out.push(TokenKind::String as u8);
                write_uvarint(out, k.len() as u64);
                out.extend_from_slice(k.as_bytes());
                encode_into(v, out)?;
            }
            out.push(TokenKind::MapEnd as u8);
        }
        Value::Timestamp(t) => {
            out.push(TokenKind::Timestamp as u8);
            out.extend_from_slice(&t.to_be_bytes());
        }
        Value::Uuid(u) => {
            out.push(TokenKind::Uuid as u8);
            out.extend_from_slice(u);
        }
        Value::Uri(s) => {
            out.push(TokenKind::Uri as u8);
            write_uvarint(out, s.len() as u64);
            out.extend_from_slice(s.as_bytes());
        }
        Value::Identifier(s) => {
            out.push(TokenKind::Identifier as u8);
            write_uvarint(out, s.len() as u64);
            out.extend_from_slice(s.as_bytes());
        }
    }
    Ok(())
}

/// Decode a token stream into a value (consumes until End or single top-level value).
pub fn decode_tokens(bytes: &[u8]) -> Result<Value> {
    let mut i = 0;
    let v = decode_one(bytes, &mut i)?;
    // optional End
    if i < bytes.len() && bytes[i] == TokenKind::End as u8 {
        i += 1;
    }
    let _ = i;
    Ok(v)
}

fn decode_one(input: &[u8], i: &mut usize) -> Result<Value> {
    if *i >= input.len() {
        return Err(Error::TruncatedPayload { needed: 1 });
    }
    let tag = input[*i];
    *i += 1;
    match tag {
        x if x == TokenKind::Null as u8 => Ok(Value::Null),
        x if x == TokenKind::BoolFalse as u8 => Ok(Value::Bool(false)),
        x if x == TokenKind::BoolTrue as u8 => Ok(Value::Bool(true)),
        x if x == TokenKind::Integer as u8 => {
            if *i + 8 > input.len() {
                return Err(Error::TruncatedPayload { needed: 8 });
            }
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&input[*i..*i + 8]);
            *i += 8;
            Ok(Value::Integer(i64::from_be_bytes(buf)))
        }
        x if x == TokenKind::Float as u8 => {
            if *i + 8 > input.len() {
                return Err(Error::TruncatedPayload { needed: 8 });
            }
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&input[*i..*i + 8]);
            *i += 8;
            Ok(Value::Float(f64::from_bits(u64::from_be_bytes(buf))))
        }
        x if x == TokenKind::String as u8
            || x == TokenKind::Uri as u8
            || x == TokenKind::Identifier as u8 =>
        {
            let len = read_uvarint(input, i)? as usize;
            if *i + len > input.len() {
                return Err(Error::TruncatedPayload { needed: len });
            }
            let s = std::str::from_utf8(&input[*i..*i + len])
                .map_err(|_| Error::InvalidUtf8 { offset: *i })?
                .to_owned();
            *i += len;
            Ok(match tag {
                t if t == TokenKind::Uri as u8 => Value::Uri(s),
                t if t == TokenKind::Identifier as u8 => Value::Identifier(s),
                _ => Value::String(s),
            })
        }
        x if x == TokenKind::Bytes as u8 => {
            let len = read_uvarint(input, i)? as usize;
            if *i + len > input.len() {
                return Err(Error::TruncatedPayload { needed: len });
            }
            let b = input[*i..*i + len].to_vec();
            *i += len;
            Ok(Value::Bytes(b))
        }
        x if x == TokenKind::ArrayStart as u8 => {
            let len = read_uvarint(input, i)? as usize;
            let mut arr = Vec::with_capacity(len);
            for _ in 0..len {
                arr.push(decode_one(input, i)?);
            }
            if *i >= input.len() || input[*i] != TokenKind::ArrayEnd as u8 {
                return Err(Error::Semantic("missing ArrayEnd".into()));
            }
            *i += 1;
            Ok(Value::Array(arr))
        }
        x if x == TokenKind::MapStart as u8 => {
            let len = read_uvarint(input, i)? as usize;
            let mut map = std::collections::BTreeMap::new();
            for _ in 0..len {
                let key = match decode_one(input, i)? {
                    Value::String(s) => s,
                    other => {
                        return Err(Error::Semantic(format!(
                            "map key must be string, got {other:?}"
                        )))
                    }
                };
                let val = decode_one(input, i)?;
                map.insert(key, val);
            }
            if *i >= input.len() || input[*i] != TokenKind::MapEnd as u8 {
                return Err(Error::Semantic("missing MapEnd".into()));
            }
            *i += 1;
            Ok(Value::Map(map))
        }
        x if x == TokenKind::Timestamp as u8 => {
            if *i + 8 > input.len() {
                return Err(Error::TruncatedPayload { needed: 8 });
            }
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&input[*i..*i + 8]);
            *i += 8;
            Ok(Value::Timestamp(i64::from_be_bytes(buf)))
        }
        x if x == TokenKind::Uuid as u8 => {
            if *i + 16 > input.len() {
                return Err(Error::TruncatedPayload { needed: 16 });
            }
            let mut u = [0u8; 16];
            u.copy_from_slice(&input[*i..*i + 16]);
            *i += 16;
            Ok(Value::Uuid(u))
        }
        x if x == TokenKind::End as u8 => Err(Error::Semantic("unexpected End".into())),
        other => Err(Error::Semantic(format!("unknown token 0x{other:02x}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn roundtrip_task() {
        let mut m = BTreeMap::new();
        m.insert("task".into(), Value::String("compile".into()));
        m.insert("priority".into(), Value::Integer(4));
        m.insert("retry".into(), Value::Bool(true));
        let v = Value::Map(m);
        let bytes = encode_tokens(&v).unwrap();
        let back = decode_tokens(&bytes).unwrap();
        assert_eq!(v, back);
    }
}
