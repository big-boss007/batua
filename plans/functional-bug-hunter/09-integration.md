# Phase 9: Integration & Verification

## Objective
Run all tests, verify they pass, identify any real bugs found.

## Steps

1. Run `cargo test` — verify all new unit tests pass
2. Run `cargo test -- --test functional_tests` — verify integration tests pass
3. Catalog any test failures as discovered bugs
4. For each bug found: document file, line, expected vs actual, suggested fix
5. Fix bugs if straightforward, or flag for review

## Bug Report Template
```
BUG-XXX: {one-line summary}
- Test: {test_id}
- File: {path}:{line}
- Expected: {what should happen}
- Actual: {what happens}
- Severity: {P0/P1/P2}
- Category: {boundary/null/arithmetic/state/race}
- Fix: {suggested code change}
```
