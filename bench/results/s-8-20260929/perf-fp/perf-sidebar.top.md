# campfire fp: sidebar (perf, fp)

34514000 samples at 1 µs (34.51 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **gzip (miniz_oxide/crc32)** | (all) | **45.9%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 2.0% |
| **sqlite (C)** | (all) | **17.1%** |
| sqlite (C) | malloc/free/realloc | 0.5% |
| **app (controllers/channels/jobs)** | (all) | **10.3%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 3.2% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.6% |
| **askama render / view helpers** | (all) | **8.9%** |
| askama render / view helpers | memcpy/memmove/memset | 1.4% |
| askama render / view helpers | malloc/free/realloc | 1.3% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 0.5% |
| **crypto / signing (rails_compat)** | (all) | **6.7%** |
| **kit (request/response plumbing)** | (all) | **5.4%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.6% |
| kit (request/response plumbing) | malloc/free/realloc | 0.6% |
| **db models / queries** | (all) | **1.9%** |
| **hyper / http** | (all) | **1.8%** |
| **tokio runtime / scheduling** | (all) | **1.5%** |
| **json (serde_json)** | (all) | **0.5%** |
| **other** | (all) | **0.0%** |

## Top self

| self | function |
|---|---|
| 10.7% | `zlib_rs::longest_match_help<false>` |
| 7.3% | `zlib_rs::is_match<8>` |
| 3.9% | `__memcpy_avx512_unaligned_erms` |
| 2.9% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 2.8% | `zlib_rs::insert_string` |
| 2.0% | `libsqlite3_sys::sqlite3VdbeExec` |
| 2.0% | `__strlen_evex512` |
| 1.8% | `__memset_avx512_unaligned_erms` |
| 1.7% | `zlib_rs::send_bits` |
| 1.5% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.5% | `core_arch::_mm_add_epi32` |
| 1.4% | `core::next<u8>` |
| 1.4% | `zlib_rs::braid_core<5>` |
| 1.4% | `libsqlite3_sys::columnName` |
| 1.3% | `imalloc_fastpath` |
| 1.2% | `core_arch::_mm_shuffle_epi32<14>` |
| 1.1% | `zlib_rs::quick_insert_value` |
| 1.1% | `zlib_rs::deflate::longest_match::longest_match` |
| 1.0% | `core_arch::_mm_alignr_epi8<4>` |
| 0.9% | `_rjem_je_arena_ralloc_no_move` |
| 0.9% | `zlib_rs::hash_calc` |
| 0.9% | `cache_bin_alloc_impl` |
| 0.8% | `core::index<u8>` |
| 0.8% | `core::eq` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.7% | `core_arch::_mm_sha256msg1_epu32` |
| 0.7% | `libsqlite3_sys::columnMem` |
| 0.6% | `zlib_rs::as_slice<u16, 65536>` |
| 0.6% | `_rjem_je_arena_ralloc` |
| 0.6% | `libsqlite3_sys::sqlite3_column_count` |
| 0.6% | `zlib_rs::{closure}` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.6% | `copied<[u8, 8], core::array::TryFromSliceError>` |
| 0.5% | `core::split_at_checked<u8>` |
| 0.5% | `libsqlite3_sys::sqlite3_column_name` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `zlib_rs::insert_match` |
| 0.4% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.4% | `free_fastpath` |
| 0.4% | `core::eq_ignore_ascii_case` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 100.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 100.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 100.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 100.0% | `start_thread` |
| 100.0% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 100.0% | `__GI___clone3` |
| 100.0% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 100.0% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 100.0% | `tokio::run` |
| 100.0% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 100.0% | `tokio::{closure}` |
| 99.8% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 99.8% | `tokio::poll` |
| 78.5% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 78.5% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 78.5% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 78.5% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 78.5% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 78.5% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 78.5% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 78.5% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 78.5% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 78.5% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 78.5% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 78.5% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 78.5% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 78.5% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 78.5% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 78.5% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 78.5% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 78.3% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 78.3% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 78.3% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 78.3% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 78.3% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 78.3% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 78.3% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 78.2% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 78.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 78.2% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 78.2% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 78.2% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 78.2% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 78.2% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 78.2% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 78.2% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 78.2% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 78.2% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 77.9% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 77.9% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 77.8% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 77.8% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 77.8% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 77.8% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 77.1% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 45.6% | `axum_core::poll_frame` |
| 45.6% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 45.6% | `poll_frame<axum_core::body::Body>` |
| 45.6% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 45.6% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 45.5% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 45.5% | `campfire_kit::poll_frame` |
| 44.6% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 44.6% | `campfire_kit::{async_block#0}` |
| 44.6% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 44.6% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 44.6% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 44.1% | `campfire_kit::compress` |
| 42.0% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 42.0% | `write_to_spare_capacity_of_vec<core::result::Result<flate2::mem::Status, flate2::mem::CompressError>, flate2::mem::{impl#1}::compress_vec::{closure_env#0}>` |
| 42.0% | `flate2::compress_vec` |
| 42.0% | `flate2::{closure}` |
| 42.0% | `compress_uninit<flate2::ffi::zlib_rs::Deflate>` |
| 42.0% | `flate2::compress_uninit` |
| 42.0% | `flate2::compress` |
| 41.9% | `<zlib_rs::stable::Deflate>::compress` |
| 41.9% | `zlib_rs::compress_uninit` |
| 41.9% | `zlib_rs::deflate` |
