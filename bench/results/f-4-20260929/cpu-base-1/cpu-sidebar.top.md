# campfire base: sidebar

21056 samples at 1000 µs (21.06 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **gzip (miniz_oxide/crc32)** | (all) | **41.9%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.7% |
| **sqlite (C)** | (all) | **15.1%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.2% |
| sqlite (C) | malloc/free/realloc | 0.7% |
| **app (controllers/channels/jobs)** | (all) | **10.8%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 3.2% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 0.9% |
| **tokio runtime / scheduling** | (all) | **7.4%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 5.9% |
| **askama render / view helpers** | (all) | **7.2%** |
| askama render / view helpers | memcpy/memmove/memset | 1.5% |
| askama render / view helpers | malloc/free/realloc | 0.6% |
| **kit (request/response plumbing)** | (all) | **6.0%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.6% |
| **crypto / signing (rails_compat)** | (all) | **5.8%** |
| **db models / queries** | (all) | **3.8%** |
| db models / queries | syscalls (read/write/epoll/futex) | 2.8% |
| **hyper / http** | (all) | **1.6%** |
| **json (serde_json)** | (all) | **0.4%** |
| **other** | (all) | **0.0%** |

## Top self

| self | function |
|---|---|
| 9.5% | `zlib_rs::longest_match_help<false>` |
| 6.5% | `zlib_rs::is_match<8>` |
| 5.6% | `syscall` |
| 4.6% | `__syscall_cancel_arch` |
| 3.3% | `__memcpy_avx512_unaligned_erms` |
| 2.5% | `zlib_rs::insert_string` |
| 2.3% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 1.8% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.6% | `zlib_rs::send_bits` |
| 1.6% | `__memset_avx512_unaligned_erms` |
| 1.4% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.4% | `zlib_rs::braid_core<5>` |
| 1.3% | `core::next<u8>` |
| 1.2% | `zlib_rs::deflate::longest_match::longest_match` |
| 1.2% | `core_arch::_mm_add_epi32` |
| 1.1% | `imalloc_fastpath` |
| 1.0% | `__strlen_evex512` |
| 1.0% | `zlib_rs::quick_insert_value` |
| 1.0% | `zlib_rs::hash_calc` |
| 1.0% | `core_arch::_mm_shuffle_epi32<14>` |
| 0.8% | `core_arch::_mm_alignr_epi8<4>` |
| 0.8% | `cache_bin_alloc_impl` |
| 0.8% | `_rjem_je_arena_ralloc_no_move` |
| 0.8% | `core::split_at_checked<u8>` |
| 0.7% | `core::index<u8>` |
| 0.7% | `core::eq` |
| 0.7% | `core_arch::_mm_sha256msg1_epu32` |
| 0.7% | `libsqlite3_sys::columnName` |
| 0.7% | `zlib_rs::insert_match` |
| 0.6% | `__memcmp_evex_movbe` |
| 0.5% | `_rjem_je_arena_ralloc` |
| 0.5% | `libsqlite3_sys::sqlite3_column_count` |
| 0.5% | `__GI___lll_lock_wake` |
| 0.5% | `lll_mutex_unlock_optimized` |
| 0.5% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `libsqlite3_sys::columnMem` |
| 0.4% | `zlib_rs::as_slice<u16, 65536>` |
| 0.4% | `{closure}<(&u8, &u8), usize, core::ops::try_trait::NeverShortCircuit<usize>, zlib_rs::deflate::compare256::rust::compare256::{closure_env#0}, core::ops::try_tra` |
| 0.4% | `campfire_views::helpers::html::push_escaped` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 100.0% | `tokio::{closure}` |
| 100.0% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 100.0% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 100.0% | `start_thread` |
| 100.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 100.0% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 100.0% | `tokio::run` |
| 100.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 100.0% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 100.0% | `__GI___clone3` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 97.0% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 97.0% | `tokio::poll` |
| 77.7% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 77.7% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 77.7% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 77.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 77.7% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 77.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 77.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 77.7% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 77.7% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 77.7% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 77.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 77.7% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 77.7% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 77.7% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 77.7% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 77.7% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 77.7% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 77.4% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 77.4% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 77.4% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 77.4% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 77.4% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 77.4% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 77.3% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 77.3% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 77.3% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 77.3% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 77.3% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 77.3% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 77.3% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 77.3% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 77.3% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 77.3% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 77.3% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 77.3% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 77.0% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 77.0% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 77.0% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 77.0% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 77.0% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 76.9% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 73.3% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 43.3% | `axum_core::poll_frame` |
| 43.3% | `poll_frame<axum_core::body::Body>` |
| 43.3% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 43.3% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 43.3% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 43.2% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 43.2% | `campfire_kit::poll_frame` |
| 40.9% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 40.9% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 40.9% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 40.9% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 40.9% | `campfire_kit::{async_block#0}` |
| 40.3% | `campfire_kit::compress` |
| 38.6% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 38.6% | `write_to_spare_capacity_of_vec<core::result::Result<flate2::mem::Status, flate2::mem::CompressError>, flate2::mem::{impl#1}::compress_vec::{closure_env#0}>` |
| 38.6% | `flate2::{closure}` |
| 38.6% | `flate2::compress_vec` |
| 38.6% | `compress_uninit<flate2::ffi::zlib_rs::Deflate>` |
| 38.6% | `flate2::compress_uninit` |
| 38.3% | `flate2::compress` |
| 38.3% | `<zlib_rs::stable::Deflate>::compress` |
| 38.3% | `zlib_rs::compress_uninit` |
| 38.3% | `zlib_rs::deflate` |
