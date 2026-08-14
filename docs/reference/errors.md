# Errors

Typed errors (Rust `doldskrift::Error`):

`InvalidHeader`, `UnsupportedVersion`, `UnsupportedMode`, `InvalidCodepoint`, `InvalidMapping`, `MappingCollision`, `ChecksumMismatch`, `TruncatedPayload`, `InvalidUtf8`, `InvalidManifest`, `FontGenerationError`, `IoError`, `InvalidLength`, `NegotiationFailed`, `VisionError`, `Other`.

Malformed input must return an error — never panic. Fuzz targets cover decoder, container, manifest, stream, mapping.
