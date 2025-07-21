# mimi-content

A complete implementation of MIMI content from [draft-ietf-mimi-content-06](https://www.ietf.org/archive/id/draft-ietf-mimi-content-06.html) in pure Rust.

License: Apache 2

Status: Being used in a product.

Supports:
- wasm
- nested parts and external parts
- GFM-MIMI markdown
- the status format (for message delivery, read receipts, etc.)
- generating message IDs
- generating franking tags (via feature flag)
- tests against example messages in the draft
