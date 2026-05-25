# Handler

Add `update_customer` handler to `src/services/admin/handler.rs`:
- Path params: merchant_id, customer_id
- Body: UpdateCustomerRequest
- Calls storage::update_customer
- Returns updated customer JSON
