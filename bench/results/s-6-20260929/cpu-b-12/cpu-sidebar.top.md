# campfire b: sidebar

20282 samples at 1000 µs (20.28 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **gzip (miniz_oxide/crc32)** | (all) | **38.2%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.8% |
| **sqlite (C)** | (all) | **16.2%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.5% |
| sqlite (C) | malloc/free/realloc | 1.1% |
| sqlite (C) | memcpy/memmove/memset | 0.6% |
| **app (controllers/channels/jobs)** | (all) | **12.7%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 3.5% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.3% |
| **tokio runtime / scheduling** | (all) | **7.8%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 6.3% |
| **askama render / view helpers** | (all) | **7.2%** |
| askama render / view helpers | memcpy/memmove/memset | 1.6% |
| askama render / view helpers | malloc/free/realloc | 0.8% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 0.5% |
| **kit (request/response plumbing)** | (all) | **6.6%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.8% |
| kit (request/response plumbing) | malloc/free/realloc | 0.7% |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.6% |
| **crypto / signing (rails_compat)** | (all) | **5.3%** |
| **db models / queries** | (all) | **3.9%** |
| db models / queries | syscalls (read/write/epoll/futex) | 2.5% |
| **hyper / http** | (all) | **1.7%** |
| **json (serde_json)** | (all) | **0.5%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 8.3% | `zlib_rs::longest_match_help<false>` |
| 5.8% | `zlib_rs::is_match<8>` |
| 5.5% | `syscall` |
| 5.1% | `__syscall_cancel_arch` |
| 4.0% | `__memcpy_avx512_unaligned_erms` |
| 2.5% | `zlib_rs::insert_string` |
| 2.2% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 1.9% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.7% | `__memset_avx512_unaligned_erms` |
| 1.7% | `cache_bin_alloc_impl` |
| 1.3% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.3% | `zlib_rs::send_bits` |
| 1.2% | `zlib_rs::quick_insert_value` |
| 1.1% | `core::next<u8>` |
| 1.1% | `zlib_rs::braid_core<5>` |
| 1.1% | `core_arch::_mm_add_epi32` |
| 1.0% | `zlib_rs::deflate::longest_match::longest_match` |
| 1.0% | `__strlen_evex512` |
| 1.0% | `imalloc_fastpath` |
| 0.9% | `zlib_rs::hash_calc` |
| 0.8% | `_rjem_je_arena_ralloc_no_move` |
| 0.8% | `core_arch::_mm_shuffle_epi32<14>` |
| 0.7% | `core::split_at_checked<u8>` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.7% | `core::index<u8>` |
| 0.7% | `libsqlite3_sys::columnName` |
| 0.7% | `__GI___lll_lock_wake` |
| 0.6% | `core_arch::_mm_alignr_epi8<4>` |
| 0.6% | `core_arch::_mm_sha256msg1_epu32` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `core::eq` |
| 0.5% | `lll_mutex_unlock_optimized` |
| 0.5% | `_rjem_je_arena_ralloc` |
| 0.5% | `alloc::fmt::format::format_inner` |
| 0.5% | `zlib_rs::insert_match` |
| 0.5% | `futex_wait` |
| 0.5% | `zlib_rs::as_slice<u16, 65536>` |
| 0.4% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.4% | `{closure}<(&u8, &u8), usize, core::ops::try_trait::NeverShortCircuit<usize>, zlib_rs::deflate::compare256::rust::compare256::{closure_env#0}, core::ops::try_tra` |
| 0.4% | `_rjem_sdallocx` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `start_thread` |
| 99.9% | `tokio::run` |
| 99.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 99.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 99.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 99.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 99.9% | `tokio::{closure}` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 99.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 99.9% | `__GI___clone3` |
| 97.0% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 97.0% | `tokio::poll` |
| 75.0% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 75.0% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 75.0% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 75.0% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 75.0% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 75.0% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 75.0% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 75.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 75.0% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 75.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 75.0% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 75.0% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 75.0% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 75.0% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 75.0% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 75.0% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 75.0% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 74.6% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 74.5% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 74.5% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 74.5% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 74.5% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 74.5% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 74.5% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 74.4% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 74.4% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 74.4% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 74.4% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 74.4% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 74.4% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 74.4% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 74.4% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 74.4% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 74.4% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 74.4% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 74.2% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 74.2% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 74.1% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 74.1% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 74.1% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 74.1% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 70.2% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 39.7% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 39.7% | `axum_core::poll_frame` |
| 39.6% | `poll_frame<axum_core::body::Body>` |
| 39.6% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 39.6% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 39.5% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 39.5% | `campfire_kit::poll_frame` |
| 37.0% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 37.0% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 37.0% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 37.0% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 37.0% | `campfire_kit::{async_block#0}` |
| 36.4% | `campfire_kit::compress` |
| 35.0% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 35.0% | `write_to_spare_capacity_of_vec<core::result::Result<flate2::mem::Status, flate2::mem::CompressError>, flate2::mem::{impl#1}::compress_vec::{closure_env#0}>` |
| 35.0% | `flate2::compress_vec` |
| 35.0% | `flate2::compress_uninit` |
| 35.0% | `flate2::{closure}` |
| 35.0% | `compress_uninit<flate2::ffi::zlib_rs::Deflate>` |
| 34.8% | `flate2::compress` |
| 34.8% | `<zlib_rs::stable::Deflate>::compress` |
| 34.8% | `zlib_rs::compress_uninit` |
| 34.8% | `zlib_rs::deflate` |
