# Before (legacy)
- `unwrap_or(-1)` used as a magic default for missing integer fields.
- `unwrap_or(0)` masked absence of values behind a zero default.
- Success or failure tracked only via a boolean flag.
- Config applied partially, allowing silent acceptance of missing fields.
- Unknown keys were silently ignored.
- No way to identify which specific error occurred.
- Magic sentinel values encoded error states.
- Duplicate keys were not checked.
- Error messages were absent or opaque.
- No per-key validation; all input was accepted as-is.
- Errors were not domain-specific, collapsing all cases together.
- Inconsistent defaulting led to unpredictable runtime behavior.

# After (refactored)
- Parsing returns `Result<Config, ConfigError>` for explicit success/failure.
- Optional fields live in `PartialConfig` as `Option`, separating missing from defaulted values.
- A domain-focused `ConfigError` enum is derived with `thiserror`.
- Each key is validated and controlled individually.
- Duplicate keys are detected and reported.
- Unknown keys raise `UnknownKey` instead of being ignored.
- Errors carry human-readable messages.
- Magic numbers are removed; typed errors replace sentinel values.
- Clear reporting of which key failed validation.
- Partial application happens only through explicit merge steps, not silently.
- Defaults are applied in one place instead of scattered `unwrap_or` calls.
- Callers use idiomatic Rust error handling for better ergonomics.
