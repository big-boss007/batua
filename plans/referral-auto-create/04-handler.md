# Handler

## Status: SKIPPED

### Reason
No new HTTP handlers are needed. The auto-creation logic lives in the earn service's `do_process_earn()` helper, not in an HTTP handler. The existing referral program CRUD handlers just need their types updated to include the new field (handled in the storage layer phase).

### Notes
- `create_program` and `update_program` handlers already accept JSON bodies — adding `code_creation_trigger` to the request types is sufficient
- The earn flow is triggered internally by event processing, not by an HTTP request
