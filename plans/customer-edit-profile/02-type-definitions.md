# Type Definitions

## Backend
Add to `src/services/admin/types.rs`:
```rust
#[derive(Debug, Deserialize)]
pub struct UpdateCustomerRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}
```

## Frontend
Add to `customers/remote.ts`: `updateCustomer(merchantId, customerId, { name, email })` function.
