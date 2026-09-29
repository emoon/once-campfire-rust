| hot spot | room_show | messages_page | sidebar | post_message |
|---|---|---|---|---|
| splice PageParts::new (all) | 16.8% | 9.8% | 0.4% | 0.8% |
|   splice text SHA-256 | 8.2% | 0.0% | 0.0% | 0.0% |
|   splice locate memcmp | 7.2% | 8.2% | 0.0% | 0.0% |
| splice gzip (CRC + copy) | 4.5% | 5.1% | 0.0% | 0.0% |
| splice etag | 0.8% | 0.0% | 0.0% | 0.0% |
| full deflate (zlib-rs) | 0.0% | 0.0% | 45.1% | 14.1% |
| views (campfire_views::, all) | 28.7% | 21.1% | 12.5% | 3.8% |
| askama templates (all) | 19.2% | 0.0% | 8.5% | 2.6% |
|   realloc under render (output/key growth) | 3.8% | 0.0% | 2.4% | 0.6% |
|   memcpy under render (fragments into page) | 6.5% | 0.0% | 1.1% | 0.6% |
|   fragment cache keys | 5.8% | 15.3% | 0.3% | 0.2% |
|   asset digest lookup | 4.0% | 0.4% | 0.9% | 0.5% |
|   Attrs render | 3.1% | 0.0% | 4.4% | 0.0% |
| sqlite + rusqlite (all) | 19.2% | 20.7% | 16.5% | 30.8% |
|   rusqlite by-name column lookup (Row::get(&str)) | 6.4% | 7.6% | 7.3% | 2.5% |
|   Message::last_page / page_* (query + rows) | 17.6% | 20.2% | 0.0% | 0.0% |
|   Room::original | 1.9% | 1.9% | 1.3% | 0.0% |
|   Blob::attached | 1.1% | 1.0% | 0.6% | 1.3% |
|   mmap/munmap remap | 0.0% | 0.1% | 0.0% | 0.4% |
|   FTS insert (create_in_index) | 0.0% | 0.0% | 0.0% | 5.5% |
|   RichTextRecord::find_for | 0.0% | 0.0% | 0.0% | 5.5% |
| DB-4 format! in paging SQL | 0.0% | 0.0% | 0.0% | 0.0% |
| DB-8 Timestamp encode/parse | 6.0% | 6.6% | 3.9% | 2.8% |
| signing: cookie verify | 1.2% | 1.2% | 0.7% | 0.9% |
| signing: signed_id / avatar | 1.0% | 0.7% | 1.6% | 1.0% |
| signing: turbo stream name | 0.3% | 0.0% | 0.3% | 0.0% |
| signing: GlobalId | 0.2% | 0.0% | 0.1% | 0.2% |
| STORE-2 verifier construction (generate_key etc) | 0.2% | 0.2% | 0.3% | 0.2% |
| request log (tracing fmt) | 1.3% | 1.6% | 0.9% | 0.9% |
| WEB-9 routing (recognize, normalize_path) | 2.6% | 2.3% | 1.0% | 1.0% |
| WEB-2 user agent | 0.9% | 0.8% | 0.5% | 0.0% |
| KIT-1 params (Ctx::new, install_path_params) | 0.5% | 0.7% | 0.4% | 1.3% |
| hyper/axum outside the app (excl. deflater, Ctx, handler) | 2.0% | 2.2% | 1.3% | 2.1% |
| spawn_blocking (spawn side, user) | 0.5% | 0.8% | 0.3% | 0.8% |
| DB reader pool checkout | 0.1% | 0.3% | 0.1% | 0.2% |
| malloc/free/realloc (self) | 10.0% | 9.3% | 7.4% | 7.7% |
| memcpy (self) | 9.4% | 10.1% | 3.9% | 5.9% |
| memcmp (self) | 5.0% | 4.7% | 0.7% | 1.3% |
| strlen (self) | 2.0% | 2.3% | 2.4% | 0.9% |
| cable: tokio broadcast | 0.0% | 0.0% | 0.0% | 0.0% |
| cable: SelectAll / FuturesUnordered | 0.0% | 0.0% | 0.0% | 0.0% |
| cable: hub / pubsub | 0.0% | 0.0% | 0.0% | 0.0% |
| cable: socket writer | 0.0% | 0.0% | 0.0% | 0.0% |
