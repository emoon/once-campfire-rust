# campfire view-3: sidebar

1222 samples at 1000 µs (1.22 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **gzip (miniz_oxide/crc32)** | (all) | **39.9%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.2% |
| **sqlite (C)** | (all) | **14.3%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.6% |
| sqlite (C) | malloc/free/realloc | 1.4% |
| **app (controllers/channels/jobs)** | (all) | **11.8%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 3.1% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 0.8% |
| **askama render / view helpers** | (all) | **7.5%** |
| askama render / view helpers | memcpy/memmove/memset | 1.6% |
| askama render / view helpers | malloc/free/realloc | 0.7% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 0.6% |
| **kit (request/response plumbing)** | (all) | **7.1%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.8% |
| **tokio runtime / scheduling** | (all) | **6.5%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 5.6% |
| **crypto / signing (rails_compat)** | (all) | **6.3%** |
| **db models / queries** | (all) | **4.4%** |
| db models / queries | syscalls (read/write/epoll/futex) | 2.9% |
| **hyper / http** | (all) | **1.6%** |
| **json (serde_json)** | (all) | **0.5%** |

## Top self

| self | function |
|---|---|
| 7.7% | `zlib_rs::longest_match_help<false>` |
| 6.5% | `syscall` |
| 5.8% | `zlib_rs::is_match<8>` |
| 4.2% | `__syscall_cancel_arch` |
| 3.7% | `__memcpy_avx512_unaligned_erms` |
| 2.9% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 2.6% | `zlib_rs::insert_string` |
| 2.3% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.9% | `zlib_rs::send_bits` |
| 1.7% | `core::next<u8>` |
| 1.6% | `zlib_rs::braid_core<5>` |
| 1.4% | `zlib_rs::hash_calc` |
| 1.3% | `zlib_rs::quick_insert_value` |
| 1.3% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.2% | `core_arch::_mm_add_epi32` |
| 1.1% | `core_arch::_mm_sha256msg1_epu32` |
| 1.1% | `imalloc_fastpath` |
| 1.1% | `cache_bin_alloc_impl` |
| 1.1% | `core_arch::_mm_alignr_epi8<4>` |
| 1.0% | `sz_index2size_lookup_impl` |
| 0.9% | `libsqlite3_sys::columnName` |
| 0.8% | `core::split_at_checked<u8>` |
| 0.8% | `_rjem_je_arena_ralloc_no_move` |
| 0.8% | `__memset_avx512_unaligned_erms` |
| 0.7% | `core::fmt::write` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.7% | `core_arch::_mm_shuffle_epi32<14>` |
| 0.6% | `core::index<u8>` |
| 0.6% | `__GI___lll_lock_wake` |
| 0.6% | `futex_wait` |
| 0.6% | `_rjem_sdallocx` |
| 0.6% | `alloc::fmt::format::format_inner` |
| 0.5% | `core::spec_eq<u8, u8, 8>` |
| 0.5% | `zlib_rs::insert_match` |
| 0.5% | `lll_mutex_unlock_optimized` |
| 0.5% | `__strlen_evex512` |
| 0.4% | `zlib_rs::fizzle_matches` |
| 0.4% | `zlib_rs::as_slice<u16, 65536>` |
| 0.4% | `zlib_rs::encode_dist` |
| 0.4% | `zlib_rs::compress_block_help` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 100.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 100.0% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 100.0% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 100.0% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 100.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 100.0% | `tokio::run` |
| 100.0% | `tokio::{closure}` |
| 100.0% | `start_thread` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 100.0% | `__GI___clone3` |
| 100.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 96.9% | `tokio::poll` |
| 96.9% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 76.8% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 76.8% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 76.8% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 76.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 76.8% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 76.8% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 76.8% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 76.8% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 76.8% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 76.8% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 76.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 76.8% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 76.8% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 76.8% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 76.8% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 76.8% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 76.8% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 76.7% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 76.6% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 76.6% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 76.5% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 76.5% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 76.5% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 76.5% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 76.5% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 76.5% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 76.5% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 76.5% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 76.5% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 76.5% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 76.5% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 76.5% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 76.4% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 76.4% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 76.4% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 76.4% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 76.4% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 76.4% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 76.4% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 76.4% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 76.4% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 73.3% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 42.6% | `poll_frame<axum_core::body::Body>` |
| 42.6% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 42.6% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 42.6% | `axum_core::poll_frame` |
| 42.6% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 42.6% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 42.6% | `campfire_kit::poll_frame` |
| 39.4% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 39.4% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 39.4% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 39.4% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 39.4% | `campfire_kit::{async_block#0}` |
| 38.6% | `campfire_kit::compress` |
| 37.0% | `flate2::compress_uninit` |
| 37.0% | `write_to_spare_capacity_of_vec<core::result::Result<flate2::mem::Status, flate2::mem::CompressError>, flate2::mem::{impl#1}::compress_vec::{closure_env#0}>` |
| 37.0% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 37.0% | `flate2::{closure}` |
| 37.0% | `compress_uninit<flate2::ffi::zlib_rs::Deflate>` |
| 37.0% | `flate2::compress_vec` |
| 36.9% | `flate2::compress` |
| 36.8% | `zlib_rs::compress_uninit` |
| 36.8% | `zlib_rs::deflate` |
| 36.8% | `<zlib_rs::stable::Deflate>::compress` |
