# Storage Layer

Add `update_customer` to `src/services/admin/storage.rs`:
- UPDATE customers SET name = $2, email = $3, updated_at = NOW() WHERE id = $1
- Verify customer belongs to merchant via wallet join
- Return updated customer fields
