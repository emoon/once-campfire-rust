# campfire f-4: sidebar

21238 samples at 1000 µs (21.24 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **gzip (miniz_oxide/crc32)** | (all) | **41.8%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.7% |
| **sqlite (C)** | (all) | **14.7%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.3% |
| sqlite (C) | malloc/free/realloc | 0.7% |
| **app (controllers/channels/jobs)** | (all) | **11.0%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 3.4% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.0% |
| **tokio runtime / scheduling** | (all) | **7.5%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 6.1% |
| **askama render / view helpers** | (all) | **7.2%** |
| askama render / view helpers | memcpy/memmove/memset | 1.3% |
| askama render / view helpers | malloc/free/realloc | 0.8% |
| **kit (request/response plumbing)** | (all) | **6.2%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.8% |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.5% |
| kit (request/response plumbing) | malloc/free/realloc | 0.5% |
| **crypto / signing (rails_compat)** | (all) | **5.6%** |
| **db models / queries** | (all) | **3.8%** |
| db models / queries | syscalls (read/write/epoll/futex) | 2.7% |
| **hyper / http** | (all) | **1.6%** |
| **json (serde_json)** | (all) | **0.4%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 9.3% | `zlib_rs::longest_match_help<false>` |
| 6.9% | `zlib_rs::is_match<8>` |
| 5.9% | `syscall` |
| 4.6% | `__syscall_cancel_arch` |
| 3.2% | `__memcpy_avx512_unaligned_erms` |
| 2.6% | `zlib_rs::insert_string` |
| 2.2% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 1.8% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.7% | `zlib_rs::send_bits` |
| 1.5% | `__memset_avx512_unaligned_erms` |
| 1.3% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.3% | `core::next<u8>` |
| 1.3% | `zlib_rs::braid_core<5>` |
| 1.2% | `core_arch::_mm_add_epi32` |
| 1.2% | `__strlen_evex512` |
| 1.1% | `zlib_rs::quick_insert_value` |
| 1.1% | `zlib_rs::hash_calc` |
| 1.0% | `imalloc_fastpath` |
| 0.9% | `zlib_rs::deflate::longest_match::longest_match` |
| 0.9% | `core_arch::_mm_shuffle_epi32<14>` |
| 0.8% | `core_arch::_mm_alignr_epi8<4>` |
| 0.8% | `core::eq` |
| 0.8% | `cache_bin_alloc_impl` |
| 0.7% | `core::index<u8>` |
| 0.7% | `core_arch::_mm_sha256msg1_epu32` |
| 0.7% | `_rjem_je_arena_ralloc_no_move` |
| 0.7% | `core::split_at_checked<u8>` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.6% | `libsqlite3_sys::columnName` |
| 0.6% | `__GI___lll_lock_wake` |
| 0.6% | `zlib_rs::insert_match` |
| 0.6% | `copied<[u8, 8], core::array::TryFromSliceError>` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.5% | `_rjem_je_arena_ralloc` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.4% | `futex_wait` |
| 0.4% | `<alloc::raw_vec::RawVecInner>::finish_grow` |
| 0.4% | `free_fastpath` |
| 0.4% | `campfire_views::helpers::html::push_escaped` |
| 0.4% | `<core::fmt::Arguments>::estimated_capacity` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `__GI___clone3` |
| 99.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 99.9% | `tokio::run` |
| 99.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 99.9% | `start_thread` |
| 99.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 99.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `tokio::{closure}` |
| 99.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 99.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 99.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 96.7% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 96.7% | `tokio::poll` |
| 78.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 78.0% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 78.0% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 78.0% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 78.0% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 78.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 78.0% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 78.0% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 78.0% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 78.0% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 78.0% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 78.0% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 78.0% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 78.0% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 78.0% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 78.0% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 78.0% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 77.7% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 77.7% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 77.7% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 77.7% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 77.7% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 77.7% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 77.6% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 77.6% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 77.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 77.6% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 77.6% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 77.6% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 77.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 77.6% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 77.6% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 77.6% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 77.6% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 77.6% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 77.3% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 77.3% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 77.3% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 77.3% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 77.3% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 77.3% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 73.7% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 43.5% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 43.5% | `axum_core::poll_frame` |
| 43.5% | `poll_frame<axum_core::body::Body>` |
| 43.5% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 43.5% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 43.4% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 43.4% | `campfire_kit::poll_frame` |
| 40.8% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 40.8% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 40.8% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 40.8% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 40.8% | `campfire_kit::{async_block#0}` |
| 40.3% | `campfire_kit::compress` |
| 38.7% | `write_to_spare_capacity_of_vec<core::result::Result<flate2::mem::Status, flate2::mem::CompressError>, flate2::mem::{impl#1}::compress_vec::{closure_env#0}>` |
| 38.7% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 38.7% | `flate2::compress_vec` |
| 38.7% | `compress_uninit<flate2::ffi::zlib_rs::Deflate>` |
| 38.7% | `flate2::compress_uninit` |
| 38.7% | `flate2::{closure}` |
| 38.4% | `flate2::compress` |
| 38.4% | `<zlib_rs::stable::Deflate>::compress` |
| 38.4% | `zlib_rs::compress_uninit` |
| 38.4% | `zlib_rs::deflate` |
