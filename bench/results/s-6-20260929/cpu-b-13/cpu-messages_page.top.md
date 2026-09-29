# campfire b: messages_page

21088 samples at 1000 µs (21.09 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **20.5%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.4% |
| sqlite (C) | malloc/free/realloc | 0.8% |
| **kit (request/response plumbing)** | (all) | **17.3%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 3.8% |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 2.4% |
| kit (request/response plumbing) | malloc/free/realloc | 1.4% |
| **askama render / view helpers** | (all) | **16.3%** |
| askama render / view helpers | memcpy/memmove/memset | 4.5% |
| askama render / view helpers | malloc/free/realloc | 1.9% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 1.7% |
| **app (controllers/channels/jobs)** | (all) | **15.4%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 4.1% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.8% |
| **tokio runtime / scheduling** | (all) | **14.0%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 10.8% |
| **db models / queries** | (all) | **7.8%** |
| db models / queries | syscalls (read/write/epoll/futex) | 4.9% |
| **gzip (miniz_oxide/crc32)** | (all) | **4.1%** |
| **hyper / http** | (all) | **2.8%** |
| **crypto / signing (rails_compat)** | (all) | **1.3%** |
| **json (serde_json)** | (all) | **0.5%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 11.8% | `syscall` |
| 7.5% | `__memcpy_avx512_unaligned_erms` |
| 6.9% | `__syscall_cancel_arch` |
| 3.5% | `__memcmp_evex_movbe` |
| 2.1% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.6% | `cache_bin_alloc_impl` |
| 1.6% | `__strlen_evex512` |
| 1.1% | `libsqlite3_sys::columnName` |
| 1.0% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.9% | `core_arch::_mm512_clmulepi64_epi128<0>` |
| 0.8% | `libsqlite3_sys::columnMem` |
| 0.8% | `imalloc_fastpath` |
| 0.8% | `core_arch::_mm512_xor_si512` |
| 0.8% | `memchr::forward` |
| 0.7% | `<jiff::fmt::strtime::Display as core::fmt::Display>::fmt` |
| 0.7% | `write_item<jiff::fmt::strtime::DefaultCustom>` |
| 0.7% | `_rjem_je_arena_ralloc_no_move` |
| 0.6% | `_rjem_je_arena_ralloc` |
| 0.6% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.5% | `libsqlite3_sys::sqlite3_column_count` |
| 0.5% | `__fcntl64_nocancel_adjusted` |
| 0.4% | `__GI___lll_lock_wake` |
| 0.4% | `<alloc::raw_vec::RawVecInner>::finish_grow` |
| 0.4% | `core_arch::_mm512_clmulepi64_epi128<17>` |
| 0.4% | `futex_wait` |
| 0.4% | `sz_index2size_lookup_impl` |
| 0.4% | `sched_yield` |
| 0.4% | `0x7f218f191d4c` |
| 0.4% | `core::eq<u8, u8>` |
| 0.4% | `clone<alloc::string::String, alloc::alloc::Global>` |
| 0.4% | `_rjem_realloc` |
| 0.4% | `memchr::find_large_imp` |
| 0.4% | `alloc::fmt::format::format_inner` |
| 0.4% | `core::fmt::write` |
| 0.4% | `libsqlite3_sys::sqlite3GetVarint` |
| 0.3% | `memchr::new` |
| 0.3% | `core::sync::atomic::atomic_load::<u32>` |
| 0.3% | `core::is_ascii_uppercase` |
| 0.3% | `core::get_offset_len_noubcheck<u8>` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `tokio::{closure}` |
| 99.9% | `start_thread` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `tokio::run` |
| 99.9% | `__GI___clone3` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 99.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 99.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 99.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 99.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 99.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 99.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 93.6% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 93.5% | `tokio::poll` |
| 56.8% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 56.8% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 56.8% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 56.8% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 56.8% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 56.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 56.8% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 56.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 56.8% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 56.8% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 56.8% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 56.8% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 56.8% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 56.8% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 56.8% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 56.8% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 56.8% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 56.0% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 55.9% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 55.9% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 55.8% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 55.8% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 55.8% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 55.8% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 55.7% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 55.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 55.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 55.7% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 55.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 55.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 55.7% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 55.7% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 55.6% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 55.6% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 55.6% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 55.1% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 55.1% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 55.1% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 55.1% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 55.1% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 55.0% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 49.3% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 46.2% | `campfire::{closure}` |
| 45.4% | `<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio_rustls::server::TlsStream<tokio::net::tcp:` |
| 45.4% | `campfire_kit::front::conn::serve_connection::<tokio_rustls::server::TlsStream<tokio::net::tcp::stream::TcpStream>>::{closure}::{closure}::{closure}` |
| 45.0% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 44.8% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::alloc::Global>>` |
| 44.8% | `campfire_kit::front::serve_with::<campfire::app::serve::{closure}::{closure}>::{closure}::{closure}::{closure}` |
| 44.7% | `<campfire_kit::front::handler::Handler>::call::{closure}` |
| 44.3% | `campfire_kit::{async_fn#0}` |
| 43.3% | `<campfire_kit::front::handler::Handler>::proxy::{closure}` |
| 43.0% | `poll<axum::routing::Router<()>, http::request::Request<axum_core::body::Body>>` |
| 42.8% | `<axum::routing::route::RouteFuture<core::convert::Infallible> as core::future::future::Future>::poll` |
| 42.7% | `poll<tower::util::boxed_clone_sync::BoxCloneSyncService<http::request::Request<axum_core::body::Body>, http::response::Response<axum_core::body::Body>, core::co` |
| 42.7% | `<axum::util::MapIntoResponseFuture<tower::util::map_err::MapErrFuture<axum::middleware::from_fn::ResponseFuture, <core::convert::Infallible as core::convert::In` |
| 42.7% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 42.6% | `poll<axum::middleware::from_fn::ResponseFuture, fn(core::convert::Infallible) -> core::convert::Infallible>` |
| 42.6% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 42.6% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 42.6% | `axum::poll` |
| 42.6% | `poll<axum::middleware::from_fn::ResponseFuture>` |
| 42.6% | `try_poll<axum::middleware::from_fn::ResponseFuture, http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 42.6% | `<axum::middleware::from_fn::FromFn<campfire_kit::deflater::deflater, (), axum::routing::route::Route, (http::request::Request<axum_core::body::Body>,)> as tower` |
| 37.9% | `<axum::middleware::from_fn::Next>::run::{closure}` |
| 37.8% | `<axum::util::MapIntoResponseFuture<axum::routing::route::RouteFuture<core::convert::Infallible>> as core::future::future::Future>::poll` |
