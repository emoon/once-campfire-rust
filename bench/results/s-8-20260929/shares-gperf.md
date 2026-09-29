| hot spot | room_show | messages_page | sidebar | post_message | cable1000 |
|---|---|---|---|---|---|
| splice PageParts::new (all) | 13.9% | 7.8% | 0.3% | 0.5% | 0.1% |
|   splice text SHA-256 | 6.9% | 0.1% | 0.0% | 0.0% | 0.0% |
|   splice locate memcmp | 5.8% | 6.4% | 0.0% | 0.0% | 0.0% |
| splice gzip (CRC + copy) | 3.8% | 4.1% | 0.0% | 0.0% | 0.0% |
| splice etag | 0.8% | 0.0% | 0.0% | 0.0% | 0.0% |
| full deflate (zlib-rs) | 0.0% | 0.0% | 42.6% | 9.9% | 0.0% |
| views (campfire_views::, all) | 24.3% | 14.4% | 11.7% | 2.5% | 0.5% |
| askama templates (all) | 15.8% | 0.0% | 8.1% | 1.6% | 0.3% |
|   realloc under render (output/key growth) | 3.0% | 0.0% | 2.2% | 0.3% | 0.1% |
|   memcpy under render (fragments into page) | 5.3% | 0.0% | 1.1% | 0.3% | 0.1% |
|   fragment cache keys | 5.3% | 10.6% | 0.3% | 0.3% | 0.1% |
|   asset digest lookup | 3.1% | 0.4% | 0.9% | 0.3% | 0.0% |
|   Attrs render | 2.8% | 0.0% | 4.0% | 0.0% | 0.0% |
| sqlite + rusqlite (all) | 15.4% | 18.4% | 13.4% | 44.6% | 6.1% |
|   rusqlite by-name column lookup (Row::get(&str)) | 4.1% | 5.0% | 3.5% | 1.7% | 0.2% |
|   Message::last_page / page_* (query + rows) | 10.3% | 12.5% | 0.0% | 0.0% | 0.0% |
|   Room::original | 1.8% | 2.7% | 1.5% | 0.0% | 0.0% |
|   Blob::attached | 1.1% | 1.4% | 0.8% | 1.3% | 0.2% |
|   mmap/munmap remap | 0.0% | 0.0% | 0.0% | 8.9% | 0.9% |
|   FTS insert (create_in_index) | 0.0% | 0.0% | 0.0% | 7.4% | 1.2% |
|   RichTextRecord::find_for | 0.0% | 0.0% | 0.0% | 6.3% | 0.8% |
| DB-4 format! in paging SQL | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| DB-8 Timestamp encode/parse | 3.6% | 4.6% | 2.1% | 1.7% | 0.2% |
| signing: cookie verify | 0.9% | 1.1% | 0.6% | 0.5% | 0.2% |
| signing: signed_id / avatar | 1.1% | 0.7% | 1.4% | 0.5% | 0.1% |
| signing: turbo stream name | 0.3% | 0.0% | 0.2% | 0.0% | 0.0% |
| signing: GlobalId | 0.1% | 0.0% | 0.0% | 0.1% | 0.0% |
| STORE-2 verifier construction (generate_key etc) | 0.3% | 0.1% | 0.3% | 0.1% | 0.0% |
| request log (tracing fmt) | 3.3% | 3.1% | 2.2% | 1.7% | 0.2% |
| WEB-9 routing (recognize, normalize_path) | 1.7% | 1.7% | 0.9% | 0.6% | 0.2% |
| WEB-2 user agent | 0.8% | 0.8% | 0.5% | 0.0% | 0.0% |
| KIT-1 params (Ctx::new, install_path_params) | 0.4% | 0.6% | 0.3% | 0.5% | 0.2% |
| hyper/axum outside the app (excl. deflater, Ctx, handler) | 6.1% | 6.5% | 4.5% | 3.1% | 67.0% |
| spawn_blocking (spawn side, user) | 5.2% | 6.2% | 2.6% | 3.0% | 0.6% |
| DB reader pool checkout | 0.5% | 1.0% | 0.3% | 0.8% | 0.0% |
| malloc/free/realloc (self) | 8.0% | 7.1% | 6.8% | 4.7% | 1.3% |
| memcpy (self) | 7.9% | 7.4% | 3.2% | 3.5% | 0.8% |
| memcmp (self) | 4.2% | 3.9% | 0.7% | 0.7% | 0.2% |
| strlen (self) | 1.1% | 1.3% | 1.0% | 0.6% | 0.1% |
| cable: tokio broadcast | 0.0% | 0.0% | 0.0% | 0.0% | 4.1% |
| cable: SelectAll / FuturesUnordered | 0.0% | 0.0% | 0.0% | 0.0% | 8.7% |
| cable: hub / pubsub | 0.0% | 0.0% | 0.0% | 0.0% | 4.0% |
| cable: socket writer | 0.0% | 0.0% | 0.0% | 0.0% | 68.1% |
