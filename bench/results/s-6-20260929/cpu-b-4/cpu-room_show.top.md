# campfire b: room_show

22140 samples at 1000 µs (22.14 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **askama render / view helpers** | (all) | **17.5%** |
| askama render / view helpers | memcpy/memmove/memset | 5.4% |
| askama render / view helpers | malloc/free/realloc | 1.8% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 1.2% |
| **sqlite (C)** | (all) | **16.9%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.0% |
| sqlite (C) | malloc/free/realloc | 0.5% |
| **kit (request/response plumbing)** | (all) | **15.6%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 3.7% |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 2.5% |
| kit (request/response plumbing) | malloc/free/realloc | 1.0% |
| **app (controllers/channels/jobs)** | (all) | **15.3%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 4.3% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 2.1% |
| **tokio runtime / scheduling** | (all) | **11.9%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 9.7% |
| **crypto / signing (rails_compat)** | (all) | **9.0%** |
| **db models / queries** | (all) | **6.4%** |
| db models / queries | syscalls (read/write/epoll/futex) | 4.1% |
| **gzip (miniz_oxide/crc32)** | (all) | **4.5%** |
| **hyper / http** | (all) | **2.4%** |
| **json (serde_json)** | (all) | **0.5%** |
| **other** | (all) | **0.0%** |

## Top self

| self | function |
|---|---|
| 9.4% | `syscall` |
| 8.0% | `__memcpy_avx512_unaligned_erms` |
| 7.4% | `__syscall_cancel_arch` |
| 4.1% | `__memcmp_evex_movbe` |
| 2.2% | `core_arch::_mm_add_epi32` |
| 1.6% | `core_arch::_mm_shuffle_epi32<14>` |
| 1.4% | `core_arch::_mm_alignr_epi8<4>` |
| 1.4% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.3% | `imalloc_fastpath` |
| 1.3% | `__strlen_evex512` |
| 1.1% | `core_arch::_mm_sha256msg1_epu32` |
| 1.0% | `core_arch::_mm512_clmulepi64_epi128<0>` |
| 0.9% | `core_arch::_mm512_xor_si512` |
| 0.9% | `cache_bin_alloc_impl` |
| 0.9% | `libsqlite3_sys::columnName` |
| 0.8% | `_rjem_je_arena_ralloc_no_move` |
| 0.7% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.7% | `memchr::forward` |
| 0.7% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.7% | `core_arch::_mm_sha256rnds2_epu32` |
| 0.7% | `_rjem_je_arena_ralloc` |
| 0.6% | `libsqlite3_sys::columnMem` |
| 0.6% | `__fcntl64_nocancel_adjusted` |
| 0.6% | `core_arch::_mm512_clmulepi64_epi128<17>` |
| 0.6% | `libsqlite3_sys::sqlite3_column_count` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.6% | `sz_index2size_lookup_impl` |
| 0.4% | `core::fmt::write` |
| 0.4% | `<alloc::raw_vec::RawVecInner>::finish_grow` |
| 0.4% | `clone<alloc::string::String, alloc::alloc::Global>` |
| 0.4% | `__GI___lll_lock_wake` |
| 0.4% | `memchr::new` |
| 0.4% | `libsqlite3_sys::sqlite3_column_name` |
| 0.4% | `free_fastpath` |
| 0.4% | `_rjem_sdallocx` |
| 0.4% | `<alloc::string::String as core::fmt::Write>::write_str (.10294)` |
| 0.4% | `core::is_ascii_uppercase` |
| 0.4% | `do_rallocx` |
| 0.4% | `core::get_offset_len_noubcheck<u8>` |
| 0.4% | `alloc::fmt::format::format_inner` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 100.0% | `start_thread` |
| 100.0% | `__GI___clone3` |
| 100.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 100.0% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 100.0% | `tokio::run` |
| 100.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 100.0% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 100.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 100.0% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 100.0% | `tokio::{closure}` |
| 100.0% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 95.0% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 95.0% | `tokio::poll` |
| 65.3% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 65.3% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 65.3% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 65.3% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 65.3% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 65.3% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 65.3% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 65.3% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 65.3% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 65.3% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 65.3% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 65.3% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 65.3% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 65.3% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 65.3% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 65.3% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 65.3% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 64.9% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 64.8% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 64.8% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 64.7% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 64.7% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 64.7% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 64.7% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 64.6% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 64.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 64.6% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 64.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 64.6% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 64.6% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 64.6% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 64.6% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 64.6% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 64.6% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 64.6% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 64.2% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 64.2% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 64.2% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 64.2% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 64.2% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 64.1% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 58.1% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 54.1% | `<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio_rustls::server::TlsStream<tokio::net::tcp:` |
| 54.1% | `campfire_kit::front::conn::serve_connection::<tokio_rustls::server::TlsStream<tokio::net::tcp::stream::TcpStream>>::{closure}::{closure}::{closure}` |
| 53.9% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 53.7% | `campfire_kit::front::serve_with::<campfire::app::serve::{closure}::{closure}>::{closure}::{closure}::{closure}` |
| 53.7% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::alloc::Global>>` |
| 53.6% | `<campfire_kit::front::handler::Handler>::call::{closure}` |
| 53.4% | `campfire_kit::{async_fn#0}` |
| 52.7% | `<campfire_kit::front::handler::Handler>::proxy::{closure}` |
| 52.4% | `poll<axum::routing::Router<()>, http::request::Request<axum_core::body::Body>>` |
| 52.2% | `<axum::routing::route::RouteFuture<core::convert::Infallible> as core::future::future::Future>::poll` |
| 52.2% | `poll<tower::util::boxed_clone_sync::BoxCloneSyncService<http::request::Request<axum_core::body::Body>, http::response::Response<axum_core::body::Body>, core::co` |
| 52.1% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 52.1% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 52.1% | `<axum::util::MapIntoResponseFuture<tower::util::map_err::MapErrFuture<axum::middleware::from_fn::ResponseFuture, <core::convert::Infallible as core::convert::In` |
| 52.1% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 52.1% | `poll<axum::middleware::from_fn::ResponseFuture, fn(core::convert::Infallible) -> core::convert::Infallible>` |
| 52.1% | `try_poll<axum::middleware::from_fn::ResponseFuture, http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 52.1% | `poll<axum::middleware::from_fn::ResponseFuture>` |
| 52.1% | `axum::poll` |
| 52.0% | `<axum::middleware::from_fn::FromFn<campfire_kit::deflater::deflater, (), axum::routing::route::Route, (http::request::Request<axum_core::body::Body>,)> as tower` |
| 47.2% | `<axum::middleware::from_fn::Next>::run::{closure}` |
| 47.1% | `<axum::util::MapIntoResponseFuture<axum::routing::route::RouteFuture<core::convert::Infallible>> as core::future::future::Future>::poll` |
| 46.9% | `<axum::middleware::from_fn::FromFn<campfire_kit::adapter::rails_middleware, campfire_kit::app::Kit, axum::routing::route::Route, (axum::extract::state::State<ca` |
