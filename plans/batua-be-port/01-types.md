# Types

## Status: SKIPPED

### Reason

This is a verbatim port, not a new service. All type files (hand-written
`types.rs` in each service, plus the generated `src/generated/types/`)
are copied unchanged from the source repo. No type design happens here.

### Notes

- Currently `src/generated/types/mod.rs` is empty and `src/generated/mod.rs`
  declares `pub mod types;`. Both are copied as-is.
- If type generation is reintroduced later, that should be planned in its
  own Skulls session.
