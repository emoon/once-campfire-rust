# campfire fpbfd: sidebar (perf, dwarf)

34463000 samples at 1 µs (34.46 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **gzip (miniz_oxide/crc32)** | (all) | **45.6%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 2.0% |
| **sqlite (C)** | (all) | **17.5%** |
| sqlite (C) | malloc/free/realloc | 0.5% |
| **app (controllers/channels/jobs)** | (all) | **12.5%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 3.9% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.2% |
| **askama render / view helpers** | (all) | **7.5%** |
| askama render / view helpers | memcpy/memmove/memset | 1.8% |
| askama render / view helpers | malloc/free/realloc | 0.7% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 0.5% |
| **crypto / signing (rails_compat)** | (all) | **6.6%** |
| **kit (request/response plumbing)** | (all) | **5.0%** |
| kit (request/response plumbing) | malloc/free/realloc | 0.6% |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.5% |
| **hyper / http** | (all) | **1.8%** |
| **db models / queries** | (all) | **1.6%** |
| **tokio runtime / scheduling** | (all) | **1.3%** |
| **json (serde_json)** | (all) | **0.4%** |
| **other** | (all) | **0.2%** |

## Top self

| self | function |
|---|---|
| 10.4% | `zlib_rs::longest_match_help<false>` |
| 7.6% | `zlib_rs::is_match<8>` |
| 3.9% | `__memcpy_avx512_unaligned_erms` |
| 3.0% | `zlib_rs::insert_string` |
| 2.8% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 2.4% | `__strlen_evex512` |
| 1.8% | `__memset_avx512_unaligned_erms` |
| 1.8% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.6% | `zlib_rs::send_bits` |
| 1.5% | `core::next<u8>` |
| 1.4% | `core_arch::_mm_add_epi32` |
| 1.4% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.4% | `zlib_rs::braid_core<5>` |
| 1.2% | `libsqlite3_sys::columnName` |
| 1.2% | `imalloc_fastpath` |
| 1.1% | `zlib_rs::quick_insert_value` |
| 1.1% | `core_arch::_mm_shuffle_epi32<14>` |
| 1.1% | `core_arch::_mm_alignr_epi8<4>` |
| 0.9% | `_rjem_je_arena_ralloc_no_move` |
| 0.9% | `zlib_rs::hash_calc` |
| 0.9% | `zlib_rs::deflate::longest_match::longest_match` |
| 0.8% | `cache_bin_alloc_impl` |
| 0.8% | `core::index<u8>` |
| 0.8% | `core_arch::_mm_sha256msg1_epu32` |
| 0.7% | `_rjem_je_arena_ralloc` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.6% | `core::eq` |
| 0.6% | `libsqlite3_sys::sqlite3_column_count` |
| 0.6% | `libsqlite3_sys::columnMem` |
| 0.6% | `core::split_at_checked<u8>` |
| 0.6% | `zlib_rs::as_slice<u16, 65536>` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.6% | `libsqlite3_sys::sqlite3_column_name` |
| 0.6% | `sz_index2size_lookup_impl` |
| 0.6% | `copied<[u8, 8], core::array::TryFromSliceError>` |
| 0.5% | `core::eq_ignore_ascii_case` |
| 0.5% | `zlib_rs::insert_match` |
| 0.5% | `zlib_rs::{closure}` |
| 0.5% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.4% | `free_fastpath` |

## Top inclusive

| incl | function |
|---|---|
| 46.7% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 46.7% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 46.7% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 46.7% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 46.7% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 46.6% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 46.6% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 45.4% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 45.4% | `axum_core::poll_frame` |
| 45.4% | `poll_frame<axum_core::body::Body>` |
| 45.3% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 45.3% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 45.3% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 45.3% | `campfire_kit::poll_frame` |
| 44.3% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 44.3% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 44.3% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 44.3% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 44.3% | `campfire_kit::{async_block#0}` |
| 43.8% | `campfire_kit::compress` |
| 42.0% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 42.0% | `write_to_spare_capacity_of_vec<core::result::Result<flate2::mem::Status, flate2::mem::CompressError>, flate2::mem::{impl#1}::compress_vec::{closure_env#0}>` |
| 42.0% | `flate2::compress_vec` |
| 42.0% | `flate2::{closure}` |
| 42.0% | `compress_uninit<flate2::ffi::zlib_rs::Deflate>` |
| 42.0% | `flate2::compress_uninit` |
| 41.7% | `flate2::compress` |
| 41.7% | `<zlib_rs::stable::Deflate>::compress` |
| 41.7% | `zlib_rs::compress_uninit` |
| 41.7% | `zlib_rs::deflate` |
| 41.6% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 40.5% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 40.5% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 40.5% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 36.7% | `write<alloc::vec::Vec<u8, alloc::alloc::Global>>` |
| 36.7% | `<flate2::gz::write::GzEncoder<alloc::vec::Vec<u8>> as std::io::Write>::write_all` |
| 35.2% | `campfire::{closure}` |
| 34.8% | `write_with_status<alloc::vec::Vec<u8, alloc::alloc::Global>, flate2::mem::Compress>` |
| 34.8% | `write<alloc::vec::Vec<u8, alloc::alloc::Global>, flate2::mem::Compress>` |
| 26.2% | `campfire::{async_fn#0}` |
| 24.1% | `campfire_kit::adapter::dispatch::<campfire::app::dispatch_with_fragment_cache>::{closure}` |
| 22.1% | `tokio::{closure}` |
| 22.0% | `tokio::poll` |
| 22.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 22.0% | `tokio::run` |
| 22.0% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 22.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 22.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 22.0% | `start_thread` |
| 22.0% | `__GI___clone3` |
| 22.0% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 22.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 22.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 22.0% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 22.0% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 21.9% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 21.6% | `zlib_rs::deflate::longest_match::longest_match` |
| 20.6% | `zlib_rs::longest_match_help<false>` |
| 18.7% | `campfire::app::dispatch_with_fragment_cache::{closure}` |
| 18.7% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>> + core::mark` |
| 18.7% | `poll<campfire::controllers::dispatch::{async_fn_env#0}>` |
| 18.7% | `with<core::task::poll::Poll<core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>>, campfire_views::fragment_cache::{impl#5}::poll:` |
| 18.7% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<(campfire::controllers::presenters::accou` |
| 18.7% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(campfire::controllers::presenters::accou` |
| 18.6% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(campfire::controllers::presenters::` |
| 18.6% | `{closure}<campfire::controllers::dispatch::{async_fn_env#0}>` |
| 18.6% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(campfire::controllers::presenters:` |
| 18.6% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(campfire::controllers::presenters` |
| 18.6% | `call_once<core::task::poll::Poll<core::result::Result<(campfire::controllers::presenters::accounts::Sidebar, bool), campfire_db::error::Error>>, tokio::runtime:` |
| 18.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 18.6% | `catch_unwind<core::task::poll::Poll<core::result::Result<(campfire::controllers::presenters::accounts::Sidebar, bool), campfire_db::error::Error>>, core::panic:` |
| 18.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 18.6% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(campfire::controllers::presenters::accounts::Sidebar, bool), campfire::controllers::u` |
| 18.6% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(ca` |
| 18.6% | `{closure}<(campfire::controllers::presenters::accounts::Sidebar, bool), campfire::controllers::users::sidebars::show::{async_fn#0}::{closure_env#0}>` |
| 18.6% | `with<(campfire::controllers::presenters::accounts::Sidebar, bool), campfire::controllers::users::sidebars::show::{async_fn#0}::{closure_env#0}>` |
| 18.0% | `with<core::result::Result<campfire::controllers::presenters::accounts::Sidebar, campfire_db::error::Error>, campfire::controllers::users::sidebars::show::{async` |
| 18.0% | `campfire::sidebar` |
| 17.8% | `campfire::controllers::users::sidebars::show::{closure}` |
| 17.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<futures_util::future::future::catch_unwind::{impl#1}::poll::{closure_env#0}<core::panic::unwind_safe::AssertU` |
