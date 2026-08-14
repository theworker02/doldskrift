//! Structured CLI errors for agents (`DOLD_JSON_ERRORS=1`).

use std::fmt;

/// Stable error envelope for agent consumers.
#[derive(Debug)]
pub struct CliError {
    pub code: &'static str,
    pub message: String,
    pub hint: Option<&'static str>,
    pub command: Option<&'static str>,
}

impl CliError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            hint: None,
            command: None,
        }
    }

    pub fn hint(mut self, hint: &'static str) -> Self {
        self.hint = Some(hint);
        self
    }

    pub fn command(mut self, command: &'static str) -> Self {
        self.command = Some(command);
        self
    }

    pub fn to_json(&self) -> String {
        let mut obj = serde_json::json!({
            "schema": "doldskrift.error/1",
            "code": self.code,
            "message": self.message,
        });
        if let Some(h) = self.hint {
            obj["hint"] = serde_json::Value::String(h.into());
        }
        if let Some(c) = self.command {
            obj["command"] = serde_json::Value::String(c.into());
        }
        serde_json::to_string_pretty(&obj).unwrap_or_else(|_| {
            format!(
                r#"{{"schema":"doldskrift.error/1","code":"{}","message":"{}"}}"#,
                self.code, self.message
            )
        })
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliError {}

/// True when agents request JSON stderr (`DOLD_JSON_ERRORS=1` or `true`).
pub fn json_errors_enabled() -> bool {
    match std::env::var("DOLD_JSON_ERRORS") {
        Ok(v) => {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "yes" || v == "on"
        }
        Err(_) => false,
    }
}

/// Classify a free-form error string into a stable code + optional hint.
pub fn classify(err: &str) -> CliError {
    let lower = err.to_ascii_lowercase();
    if lower.contains("protected")
        || lower.contains("accessdenied")
        || lower.contains("access denied")
        || lower.contains("aead")
        || lower.contains("gate")
    {
        return CliError::new("protected_refuse", err)
            .hint("Open/Neural ≠ encryption. Protected AEAD is not shipping; use inspect.")
            .command("decode|open");
    }
    if lower.contains("validation failed") || lower.contains("validate") {
        return CliError::new("validation_failed", err)
            .hint("Run `dold validate --json` for structured checks.")
            .command("validate");
    }
    if lower.contains("empty input") || lower.contains("no .txt") || lower.contains("no .dsk") {
        return CliError::new("empty_input", err).hint("Provide --text, a file, or stdin.");
    }
    if lower.contains("no dsk-carrier") || lower.contains("missing dsk-carrier") {
        return CliError::new("missing_carrier", err)
            .hint("Surface SVGs must embed a dsk-carrier (Open decode path).")
            .command("convert|postcard|ambient");
    }
    if lower.contains("parse") || lower.contains("magic") || lower.contains("checksum") {
        return CliError::new("bad_container", err)
            .hint("Expected a valid .dsk or Open surface carrier.")
            .command("inspect|validate");
    }
    if lower.contains("unknown format") || lower.contains("unsupported") {
        return CliError::new("unsupported_format", err)
            .hint("See `dold convert --help` for supported from/to values.")
            .command("convert");
    }
    if lower.contains("config") {
        return CliError::new("config_error", err)
            .hint("Try `dold config show` or `dold config init`.")
            .command("config");
    }
    CliError::new("cli_error", err).hint("Run `dold guide` or `dold commands` for orientation.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_protected() {
        let e = classify("Protected object: Gate AEAD not implemented");
        assert_eq!(e.code, "protected_refuse");
        let j = e.to_json();
        assert!(j.contains("doldskrift.error/1"));
        assert!(j.contains("protected_refuse"));
    }

    #[test]
    fn classifies_carrier() {
        assert_eq!(
            classify("postcard: no dsk-carrier in SVG").code,
            "missing_carrier"
        );
    }
}
