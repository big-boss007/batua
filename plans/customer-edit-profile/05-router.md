# Router

Add to `src/services/admin/mod.rs`:
```rust
.route("/admin/merchants/{merchant_id}/customers/{customer_id}", put(handler::update_customer))
```
