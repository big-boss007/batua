# Router

## Status: SKIPPED

### Reason

Verbatim port. `src/main.rs` (`get_router()`) and each service's `mod.rs`
declaring its sub-router are copied unchanged. Router parity is verified
by booting the server and observing all expected routes register.
