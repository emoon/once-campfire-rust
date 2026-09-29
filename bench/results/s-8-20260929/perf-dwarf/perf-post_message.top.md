# campfire fpbfd: post_message (perf, dwarf)

14256000 samples at 1 µs (14.26 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **31.5%** |
| sqlite (C) | malloc/free/realloc | 2.4% |
| sqlite (C) | memcpy/memmove/memset | 1.4% |
| sqlite (C) | syscalls (read/write/epoll/futex) | 0.6% |
| **gzip (miniz_oxide/crc32)** | (all) | **14.6%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.9% |
| **app (controllers/channels/jobs)** | (all) | **13.7%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 3.0% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.8% |
| **kit (request/response plumbing)** | (all) | **10.6%** |
| kit (request/response plumbing) | malloc/free/realloc | 1.2% |
| kit (request/response plumbing) | memcpy/memmove/memset | 1.2% |
| **rich text (Action Text pipeline)** | (all) | **6.2%** |
| rich text (Action Text pipeline) | memcpy/memmove/memset | 0.8% |
| rich text (Action Text pipeline) | malloc/free/realloc | 0.7% |
| **cable** | (all) | **5.1%** |
| **db models / queries** | (all) | **3.9%** |
| **tokio runtime / scheduling** | (all) | **3.4%** |
| **hyper / http** | (all) | **2.9%** |
| **crypto / signing (rails_compat)** | (all) | **2.8%** |
| **askama render / view helpers** | (all) | **2.7%** |
| askama render / view helpers | memcpy/memmove/memset | 0.7% |
| **json (serde_json)** | (all) | **2.4%** |
| **other** | (all) | **0.2%** |

## Top self

| self | function |
|---|---|
| 5.9% | `__memcpy_avx512_unaligned_erms` |
| 2.5% | `libsqlite3_sys::sqlite3VdbeExec` |
| 2.0% | `next_code_point<core::slice::iter::Iter<u8>>` |
| 1.9% | `cache_bin_alloc_impl` |
| 1.8% | `__memset_avx512_unaligned_erms` |
| 1.7% | `libsqlite3_sys::walFindFrame.constprop.0` |
| 1.6% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.4% | `lll_mutex_unlock_optimized` |
| 1.4% | `zlib_rs::longest_match_help<false>` |
| 1.3% | `__memcmp_evex_movbe` |
| 1.2% | `libsqlite3_sys::walChecksumBytes` |
| 1.1% | `imalloc_fastpath` |
| 1.1% | `serde_json::ser::format_escaped_str_contents::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>` |
| 1.1% | `next_match<core::str::pattern::MultiCharEqSearcher<[char, 3]>>` |
| 1.0% | `core::wrapping_sub` |
| 0.9% | `__strlen_evex512` |
| 0.9% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 0.8% | `_int_malloc` |
| 0.7% | `zlib_rs::is_match<8>` |
| 0.7% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.7% | `zlib_rs::insert_string` |
| 0.7% | `sz_index2size_lookup_impl` |
| 0.7% | `__pthread_mutex_lock` |
| 0.6% | `zlib_rs::send_bits` |
| 0.6% | `free_fastpath` |
| 0.6% | `core::next<u8>` |
| 0.5% | `lll_mutex_lock_optimized` |
| 0.5% | `core_arch::_mm_add_epi32` |
| 0.5% | `_rjem_je_arena_ralloc_no_move` |
| 0.5% | `_rjem_sdallocx` |
| 0.5% | `__libc_malloc2` |
| 0.4% | `core_arch::_mm_shuffle_epi32<14>` |
| 0.4% | `libsqlite3_sys::columnName` |
| 0.4% | `current_memory<alloc::alloc::Global>` |
| 0.4% | `core::eq<u8>` |
| 0.4% | `alloc::push` |
| 0.4% | `libsqlite3_sys::sqlite3DbMallocRawNN` |
| 0.4% | `zlib_rs::encode_len` |
| 0.4% | `zlib_rs::quick_insert_value` |
| 0.4% | `parking_lot::lock` |

## Top inclusive

| incl | function |
|---|---|
| 57.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 57.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 57.8% | `start_thread` |
| 56.5% | `__GI___clone3` |
| 50.3% | `campfire::{closure}` |
| 46.1% | `tokio::{closure}` |
| 45.9% | `tokio::run` |
| 45.9% | `tokio::poll` |
| 45.8% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 45.6% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 45.4% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 45.4% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 45.4% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 45.4% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 45.4% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 45.4% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 28.0% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<(), campfire::controllers::messages::broa` |
| 28.0% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broa` |
| 28.0% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages:` |
| 28.0% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages` |
| 28.0% | `catch_unwind<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harne` |
| 28.0% | `call_once<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtim` |
| 28.0% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::message` |
| 28.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 28.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 27.9% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>,` |
| 27.9% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(),` |
| 27.9% | `with<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 27.9% | `{closure}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 22.1% | `rusqlite::step` |
| 22.1% | `libsqlite3_sys::sqlite3_step` |
| 22.0% | `libsqlite3_sys::sqlite3Step` |
| 21.9% | `libsqlite3_sys::sqlite3VdbeExec` |
| 16.4% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 16.4% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 16.4% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 16.4% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 16.4% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 16.3% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 16.3% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 15.3% | `rusqlite::next` |
| 14.4% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 14.4% | `axum_core::poll_frame` |
| 14.3% | `poll_frame<axum_core::body::Body>` |
| 14.2% | `<axum::routing::route::RouteFuture<core::convert::Infallible> as core::future::future::Future>::poll` |
| 14.2% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 14.2% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 14.2% | `poll<tower::util::boxed_clone_sync::BoxCloneSyncService<http::request::Request<axum_core::body::Body>, http::response::Response<axum_core::body::Body>, core::co` |
| 14.1% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 14.1% | `campfire_kit::poll_frame` |
| 14.1% | `<rusqlite::row::Rows as fallible_streaming_iterator::FallibleStreamingIterator>::advance (.10624)` |
| 14.0% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 13.2% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 13.2% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 13.2% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 13.2% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 13.1% | `campfire_db::{closure}` |
| 13.1% | `campfire_kit::{async_block#0}` |
| 13.1% | `<campfire::controllers::presenters::Presenter>::message` |
| 12.9% | `campfire::{async_fn#0}` |
| 12.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<campfire_db::database::Database>::open::{closure}, ()>` |
| 12.8% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}:` |
| 12.8% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>>` |
| 12.8% | `{closure}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>` |
| 12.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::ope` |
| 12.8% | `<std::thread::lifecycle::spawn_unchecked<<campfire_db::database::Database>::open::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>::call_once::{shi` |
| 12.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::open::{c` |
| 12.7% | `<<campfire_db::database::Database>::write<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::me` |
| 12.7% | `{closure}<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::messages::create_message::{async_f` |
| 12.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>, ()>` |
| 12.7% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>>` |
| 12.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>, ()>` |
| 12.7% | `call_once<(&rusqlite::Connection, &campfire_db::database::Env), (dyn core::ops::function::FnOnce<(&rusqlite::Connection, &campfire_db::database::Env), Output=()` |
| 12.7% | `call_once<(), campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>` |
| 12.7% | `run_write<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::messages::create_message::{async_f` |
| 12.6% | `campfire_kit::compress` |
| 12.2% | `rusqlite::get_expected_row` |
| 12.2% | `campfire::renderable_message` |
| 12.1% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 12.1% | `flate2::compress_vec` |
