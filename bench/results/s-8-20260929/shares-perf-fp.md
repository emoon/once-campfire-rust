| hot spot | room_show | messages_page | sidebar | post_message | cable1000 |
|---|---|---|---|---|---|
| splice PageParts::new (all) | 13.0% | 5.7% | 0.4% | 0.8% | 0.4% |
|   splice text SHA-256 | 8.2% | 0.1% | 0.0% | 0.0% | 0.0% |
|   splice locate memcmp | 3.7% | 4.1% | 0.0% | 0.0% | 0.0% |
| splice gzip (CRC + copy) | 3.8% | 4.5% | 0.0% | 0.0% | 0.0% |
| splice etag | 0.8% | 0.0% | 0.0% | 0.0% | 0.0% |
| full deflate (zlib-rs) | 0.0% | 0.0% | 44.2% | 13.0% | 0.0% |
| views (campfire_views::, all) | 28.5% | 21.3% | 12.5% | 3.8% | 1.8% |
| askama templates (all) | 18.8% | 0.0% | 8.7% | 2.6% | 1.0% |
|   realloc under render (output/key growth) | 4.2% | 0.0% | 2.7% | 0.6% | 0.2% |
|   memcpy under render (fragments into page) | 6.8% | 0.0% | 1.0% | 0.5% | 0.3% |
|   fragment cache keys | 5.8% | 15.3% | 0.3% | 0.3% | 0.3% |
|   asset digest lookup | 3.4% | 0.2% | 0.7% | 0.6% | 0.2% |
|   Attrs render | 3.0% | 0.0% | 4.4% | 0.0% | 0.0% |
| sqlite + rusqlite (all) | 16.0% | 17.4% | 13.7% | 29.1% | 11.4% |
|   rusqlite by-name column lookup (Row::get(&str)) | 5.5% | 6.2% | 6.1% | 2.4% | 0.9% |
|   Message::last_page / page_* (query + rows) | 17.5% | 19.7% | 0.0% | 0.0% | 0.0% |
|   Room::original | 1.8% | 2.0% | 1.4% | 0.0% | 0.0% |
|   Blob::attached | 1.0% | 1.1% | 0.6% | 1.5% | 0.4% |
|   mmap/munmap remap | 0.0% | 0.0% | 0.0% | 0.3% | 0.0% |
|   FTS insert (create_in_index) | 0.0% | 0.0% | 0.0% | 5.8% | 2.2% |
|   RichTextRecord::find_for | 0.0% | 0.0% | 0.0% | 5.1% | 2.0% |
| DB-4 format! in paging SQL | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| DB-8 Timestamp encode/parse | 5.8% | 6.5% | 3.5% | 2.8% | 0.9% |
| signing: cookie verify | 1.1% | 1.2% | 0.7% | 0.8% | 0.8% |
| signing: signed_id / avatar | 1.0% | 0.7% | 1.7% | 1.0% | 0.3% |
| signing: turbo stream name | 0.3% | 0.0% | 0.3% | 0.0% | 0.0% |
| signing: GlobalId | 0.2% | 0.0% | 0.1% | 0.2% | 0.0% |
| STORE-2 verifier construction (generate_key etc) | 0.2% | 0.1% | 0.3% | 0.1% | 0.1% |
| request log (tracing fmt) | 1.6% | 1.6% | 0.8% | 1.1% | 0.3% |
| WEB-9 routing (recognize, normalize_path) | 2.6% | 2.5% | 1.0% | 0.9% | 0.5% |
| WEB-2 user agent | 0.8% | 0.9% | 0.5% | 0.0% | 0.0% |
| KIT-1 params (Ctx::new, install_path_params) | 0.4% | 0.6% | 0.3% | 1.1% | 0.5% |
| hyper/axum outside the app (excl. deflater, Ctx, handler) | 1.9% | 2.3% | 1.3% | 2.1% | 4.4% |
| spawn_blocking (spawn side, user) | 0.6% | 0.9% | 0.3% | 0.7% | 0.5% |
| DB reader pool checkout | 0.1% | 0.2% | 0.0% | 0.1% | 0.0% |
| malloc/free/realloc (self) | 9.7% | 9.1% | 7.6% | 7.3% | 4.0% |
| memcpy (self) | 10.3% | 9.9% | 3.9% | 6.0% | 2.7% |
| memcmp (self) | 5.2% | 5.2% | 0.7% | 1.2% | 0.6% |
| strlen (self) | 1.8% | 2.1% | 2.0% | 0.8% | 0.4% |
| cable: tokio broadcast | 0.0% | 0.0% | 0.0% | 0.0% | 14.6% |
| cable: SelectAll / FuturesUnordered | 0.0% | 0.0% | 0.0% | 0.0% | 27.6% |
| cable: hub / pubsub | 0.0% | 0.0% | 0.0% | 0.0% | 13.5% |
| cable: socket writer | 0.0% | 0.0% | 0.0% | 0.0% | 8.7% |
