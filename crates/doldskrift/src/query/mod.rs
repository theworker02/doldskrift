//! Simple path queries over semantic maps.

use crate::semantic::Value;
use crate::{Error, Result};

/// Resolve a dotted path like `user.name` against a map-rooted value.
pub fn get_path<'a>(root: &'a Value, path: &str) -> Result<&'a Value> {
    let mut cur = root;
    for part in path.split('.').filter(|p| !p.is_empty()) {
        match cur {
            Value::Map(m) => {
                cur = m
                    .get(part)
                    .ok_or_else(|| Error::Query(format!("missing key '{part}'")))?;
            }
            Value::Array(a) => {
                let idx: usize = part
                    .parse()
                    .map_err(|_| Error::Query(format!("bad array index '{part}'")))?;
                cur = a
                    .get(idx)
                    .ok_or_else(|| Error::Query(format!("index {idx} out of range")))?;
            }
            _ => return Err(Error::Query(format!("cannot traverse into {cur:?}"))),
        }
    }
    Ok(cur)
}
