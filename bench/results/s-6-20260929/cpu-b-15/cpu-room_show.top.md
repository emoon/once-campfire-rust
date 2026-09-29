# campfire b: room_show

21094 samples at 1000 µs (21.09 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **17.6%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.0% |
| sqlite (C) | malloc/free/realloc | 0.8% |
| sqlite (C) | memcpy/memmove/memset | 0.5% |
| **askama render / view helpers** | (all) | **17.5%** |
| askama render / view helpers | memcpy/memmove/memset | 5.4% |
| askama render / view helpers | malloc/free/realloc | 1.8% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 1.4% |
| **app (controllers/channels/jobs)** | (all) | **16.8%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 4.7% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 2.6% |
| **kit (request/response plumbing)** | (all) | **15.3%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 3.7% |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 2.1% |
| kit (request/response plumbing) | malloc/free/realloc | 1.2% |
| **tokio runtime / scheduling** | (all) | **11.6%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 9.3% |
| **crypto / signing (rails_compat)** | (all) | **7.8%** |
| **db models / queries** | (all) | **6.3%** |
| db models / queries | syscalls (read/write/epoll/futex) | 4.0% |
| **gzip (miniz_oxide/crc32)** | (all) | **4.1%** |
| **hyper / http** | (all) | **2.5%** |
| **json (serde_json)** | (all) | **0.4%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 9.4% | `syscall` |
| 8.7% | `__memcpy_avx512_unaligned_erms` |
| 6.7% | `__syscall_cancel_arch` |
| 4.1% | `__memcmp_evex_movbe` |
| 1.7% | `core_arch::_mm_add_epi32` |
| 1.7% | `cache_bin_alloc_impl` |
| 1.6% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.4% | `core_arch::_mm_shuffle_epi32<14>` |
| 1.1% | `__strlen_evex512` |
| 1.1% | `imalloc_fastpath` |
| 1.0% | `core_arch::_mm_alignr_epi8<4>` |
| 1.0% | `core_arch::_mm512_clmulepi64_epi128<0>` |
| 1.0% | `core_arch::_mm_sha256msg1_epu32` |
| 0.8% | `_rjem_je_arena_ralloc_no_move` |
| 0.8% | `_rjem_je_arena_ralloc` |
| 0.7% | `core_arch::_mm512_xor_si512` |
| 0.6% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.6% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.6% | `libsqlite3_sys::columnName` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.5% | `core_arch::_mm_sha256rnds2_epu32` |
| 0.5% | `__fcntl64_nocancel_adjusted` |
| 0.5% | `libsqlite3_sys::columnMem` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `core::fmt::write` |
| 0.5% | `<alloc::raw_vec::RawVecInner>::finish_grow` |
| 0.4% | `memchr::forward` |
| 0.4% | `core_arch::_mm512_clmulepi64_epi128<17>` |
| 0.4% | `alloc::fmt::format::format_inner` |
| 0.4% | `<jiff::fmt::strtime::Display as core::fmt::Display>::fmt` |
| 0.4% | `_rjem_realloc` |
| 0.4% | `libsqlite3_sys::sqlite3_column_count` |
| 0.4% | `__GI___lll_lock_wake` |
| 0.4% | `_rjem_sdallocx` |
| 0.4% | `write_item<jiff::fmt::strtime::DefaultCustom>` |
| 0.4% | `core::get_offset_len_noubcheck<u8>` |
| 0.4% | `do_rallocx` |
| 0.4% | `binary_search_by<(&str, &str), campfire_assets::helpers::digested_path::{closure_env#0}>` |
| 0.3% | `free_fastpath` |
| 0.3% | `<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `__GI___clone3` |
| 99.9% | `start_thread` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `tokio::{closure}` |
| 99.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 99.9% | `tokio::run` |
| 99.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 99.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 99.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 99.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 99.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 95.2% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 95.1% | `tokio::poll` |
| 63.2% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 63.2% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 63.2% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 63.2% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 63.2% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 63.2% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 63.2% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 63.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 63.2% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 63.2% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 63.2% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 63.2% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 63.2% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 63.2% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 63.2% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 63.2% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 63.2% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 62.7% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 62.6% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 62.6% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 62.6% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 62.5% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 62.5% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 62.4% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 62.4% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 62.4% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 62.4% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 62.4% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 62.4% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 62.4% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 62.4% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 62.4% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 62.3% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 62.3% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 62.3% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 61.9% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 61.9% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 61.9% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 61.9% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 61.9% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 61.8% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 56.2% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 52.4% | `<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio_rustls::server::TlsStream<tokio::net::tcp:` |
| 52.4% | `campfire_kit::front::conn::serve_connection::<tokio_rustls::server::TlsStream<tokio::net::tcp::stream::TcpStream>>::{closure}::{closure}::{closure}` |
| 52.1% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 52.0% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::alloc::Global>>` |
| 52.0% | `campfire_kit::front::serve_with::<campfire::app::serve::{closure}::{closure}>::{closure}::{closure}::{closure}` |
| 51.9% | `<campfire_kit::front::handler::Handler>::call::{closure}` |
| 51.6% | `campfire_kit::{async_fn#0}` |
| 50.6% | `<campfire_kit::front::handler::Handler>::proxy::{closure}` |
| 50.3% | `poll<axum::routing::Router<()>, http::request::Request<axum_core::body::Body>>` |
| 50.1% | `<axum::routing::route::RouteFuture<core::convert::Infallible> as core::future::future::Future>::poll` |
| 50.1% | `poll<tower::util::boxed_clone_sync::BoxCloneSyncService<http::request::Request<axum_core::body::Body>, http::response::Response<axum_core::body::Body>, core::co` |
| 50.0% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 50.0% | `<axum::util::MapIntoResponseFuture<tower::util::map_err::MapErrFuture<axum::middleware::from_fn::ResponseFuture, <core::convert::Infallible as core::convert::In` |
| 50.0% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 50.0% | `poll<axum::middleware::from_fn::ResponseFuture, fn(core::convert::Infallible) -> core::convert::Infallible>` |
| 50.0% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 50.0% | `try_poll<axum::middleware::from_fn::ResponseFuture, http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 50.0% | `poll<axum::middleware::from_fn::ResponseFuture>` |
| 50.0% | `axum::poll` |
| 50.0% | `<axum::middleware::from_fn::FromFn<campfire_kit::deflater::deflater, (), axum::routing::route::Route, (http::request::Request<axum_core::body::Body>,)> as tower` |
| 47.4% | `campfire::{closure}` |
| 45.5% | `<axum::middleware::from_fn::Next>::run::{closure}` |
| 45.4% | `<axum::util::MapIntoResponseFuture<axum::routing::route::RouteFuture<core::convert::Infallible>> as core::future::future::Future>::poll` |
