# campfire a: sidebar

21254 samples at 1000 µs (21.25 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **gzip (miniz_oxide/crc32)** | (all) | **42.8%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.8% |
| **sqlite (C)** | (all) | **14.3%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.1% |
| sqlite (C) | malloc/free/realloc | 0.6% |
| **app (controllers/channels/jobs)** | (all) | **10.8%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 3.3% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 0.9% |
| **tokio runtime / scheduling** | (all) | **7.5%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 6.0% |
| **askama render / view helpers** | (all) | **6.8%** |
| askama render / view helpers | memcpy/memmove/memset | 1.4% |
| askama render / view helpers | malloc/free/realloc | 0.6% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 0.6% |
| **crypto / signing (rails_compat)** | (all) | **5.8%** |
| **kit (request/response plumbing)** | (all) | **5.8%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.5% |
| kit (request/response plumbing) | malloc/free/realloc | 0.5% |
| **db models / queries** | (all) | **4.2%** |
| db models / queries | syscalls (read/write/epoll/futex) | 2.9% |
| **hyper / http** | (all) | **1.7%** |
| **json (serde_json)** | (all) | **0.3%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 9.5% | `zlib_rs::longest_match_help<false>` |
| 6.9% | `zlib_rs::is_match<8>` |
| 5.7% | `syscall` |
| 4.7% | `__syscall_cancel_arch` |
| 3.3% | `__memcpy_avx512_unaligned_erms` |
| 2.8% | `zlib_rs::insert_string` |
| 2.3% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 1.7% | `zlib_rs::send_bits` |
| 1.7% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.7% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.6% | `__memset_avx512_unaligned_erms` |
| 1.5% | `core::next<u8>` |
| 1.5% | `zlib_rs::braid_core<5>` |
| 1.3% | `core_arch::_mm_add_epi32` |
| 1.2% | `__strlen_evex512` |
| 1.1% | `zlib_rs::quick_insert_value` |
| 1.1% | `zlib_rs::hash_calc` |
| 1.1% | `imalloc_fastpath` |
| 1.0% | `core_arch::_mm_shuffle_epi32<14>` |
| 1.0% | `zlib_rs::deflate::longest_match::longest_match` |
| 0.9% | `core_arch::_mm_alignr_epi8<4>` |
| 0.9% | `core::split_at_checked<u8>` |
| 0.7% | `core::index<u8>` |
| 0.7% | `cache_bin_alloc_impl` |
| 0.7% | `_rjem_je_arena_ralloc_no_move` |
| 0.7% | `core::eq` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.6% | `copied<[u8, 8], core::array::TryFromSliceError>` |
| 0.6% | `libsqlite3_sys::columnName` |
| 0.6% | `__memcmp_evex_movbe` |
| 0.6% | `core_arch::_mm_sha256msg1_epu32` |
| 0.6% | `__GI___lll_lock_wake` |
| 0.5% | `zlib_rs::insert_match` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `_rjem_je_arena_ralloc` |
| 0.5% | `zlib_rs::as_slice<u16, 65536>` |
| 0.4% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.4% | `{closure}<(&u8, &u8), usize, core::ops::try_trait::NeverShortCircuit<usize>, zlib_rs::deflate::compare256::rust::compare256::{closure_env#0}, core::ops::try_tra` |
| 0.4% | `<alloc::raw_vec::RawVecInner>::finish_grow` |
| 0.4% | `libsqlite3_sys::sqlite3_column_count` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `start_thread` |
| 99.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 99.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 99.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 99.9% | `tokio::{closure}` |
| 99.9% | `tokio::run` |
| 99.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `__GI___clone3` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 99.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 99.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 97.0% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 96.9% | `tokio::poll` |
| 78.9% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 78.9% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 78.9% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 78.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 78.9% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 78.9% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 78.9% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 78.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 78.9% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 78.9% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 78.9% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 78.9% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 78.9% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 78.9% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 78.9% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 78.9% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 78.9% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 78.5% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 78.5% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 78.5% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 78.5% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 78.5% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 78.4% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 78.4% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 78.4% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 78.4% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 78.4% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 78.4% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 78.4% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 78.4% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 78.4% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 78.4% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 78.4% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 78.4% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 78.4% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 78.2% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 78.1% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 78.1% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 78.1% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 78.1% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 78.1% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 74.4% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 44.2% | `poll_frame<axum_core::body::Body>` |
| 44.2% | `axum_core::poll_frame` |
| 44.2% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 44.2% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 44.2% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 44.1% | `campfire_kit::poll_frame` |
| 44.1% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 41.7% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 41.7% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 41.7% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 41.7% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 41.7% | `campfire_kit::{async_block#0}` |
| 41.2% | `campfire_kit::compress` |
| 39.5% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 39.5% | `write_to_spare_capacity_of_vec<core::result::Result<flate2::mem::Status, flate2::mem::CompressError>, flate2::mem::{impl#1}::compress_vec::{closure_env#0}>` |
| 39.5% | `flate2::compress_vec` |
| 39.5% | `compress_uninit<flate2::ffi::zlib_rs::Deflate>` |
| 39.5% | `flate2::compress_uninit` |
| 39.5% | `flate2::{closure}` |
| 39.2% | `flate2::compress` |
| 39.2% | `zlib_rs::compress_uninit` |
| 39.2% | `<zlib_rs::stable::Deflate>::compress` |
| 39.2% | `zlib_rs::deflate` |
