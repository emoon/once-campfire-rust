# campfire base: room_show

22218 samples at 1000 µs (22.22 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **askama render / view helpers** | (all) | **17.8%** |
| askama render / view helpers | memcpy/memmove/memset | 5.6% |
| askama render / view helpers | malloc/free/realloc | 1.8% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 1.3% |
| **sqlite (C)** | (all) | **16.1%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 0.9% |
| **app (controllers/channels/jobs)** | (all) | **15.7%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 4.5% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.9% |
| **kit (request/response plumbing)** | (all) | **15.1%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 3.4% |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 2.2% |
| kit (request/response plumbing) | malloc/free/realloc | 1.0% |
| **tokio runtime / scheduling** | (all) | **11.9%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 9.5% |
| **crypto / signing (rails_compat)** | (all) | **9.2%** |
| **db models / queries** | (all) | **6.7%** |
| db models / queries | syscalls (read/write/epoll/futex) | 4.5% |
| **gzip (miniz_oxide/crc32)** | (all) | **4.6%** |
| **hyper / http** | (all) | **2.4%** |
| **json (serde_json)** | (all) | **0.4%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 9.9% | `syscall` |
| 7.9% | `__memcpy_avx512_unaligned_erms` |
| 6.8% | `__syscall_cancel_arch` |
| 4.0% | `__memcmp_evex_movbe` |
| 2.3% | `core_arch::_mm_add_epi32` |
| 1.6% | `core_arch::_mm_shuffle_epi32<14>` |
| 1.4% | `core_arch::_mm_alignr_epi8<4>` |
| 1.4% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.4% | `imalloc_fastpath` |
| 1.2% | `__strlen_evex512` |
| 1.1% | `core_arch::_mm_sha256msg1_epu32` |
| 1.0% | `core_arch::_mm512_clmulepi64_epi128<0>` |
| 1.0% | `core_arch::_mm512_xor_si512` |
| 0.9% | `cache_bin_alloc_impl` |
| 0.8% | `_rjem_je_arena_ralloc_no_move` |
| 0.7% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.7% | `libsqlite3_sys::columnName` |
| 0.7% | `core_arch::_mm_sha256rnds2_epu32` |
| 0.7% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.7% | `_rjem_je_arena_ralloc` |
| 0.6% | `libsqlite3_sys::columnMem` |
| 0.6% | `memchr::forward` |
| 0.6% | `core_arch::_mm512_clmulepi64_epi128<17>` |
| 0.6% | `sz_index2size_lookup_impl` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.5% | `__fcntl64_nocancel_adjusted` |
| 0.5% | `<alloc::raw_vec::RawVecInner>::finish_grow` |
| 0.4% | `libsqlite3_sys::sqlite3_column_count` |
| 0.4% | `_rjem_sdallocx` |
| 0.4% | `core::fmt::write` |
| 0.4% | `core::sync::atomic::atomic_load::<u32>` |
| 0.4% | `free_fastpath` |
| 0.4% | `<rusqlite::statement::Statement>::value_ref` |
| 0.4% | `__GI___lll_lock_wake` |
| 0.4% | `memchr::find_large_imp` |
| 0.4% | `_rjem_realloc` |
| 0.4% | `core::get_offset_len_noubcheck<u8>` |
| 0.4% | `<alloc::string::String as core::fmt::Write>::write_str (.10303)` |
| 0.4% | `binary_search_by<(&str, &str), campfire_assets::helpers::digested_path::{closure_env#0}>` |
| 0.4% | `write_item<jiff::fmt::strtime::DefaultCustom>` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `start_thread` |
| 99.9% | `__GI___clone3` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 99.9% | `tokio::{closure}` |
| 99.9% | `tokio::run` |
| 99.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 99.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 99.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 99.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 99.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 95.0% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 95.0% | `tokio::poll` |
| 65.9% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 65.9% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 65.9% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 65.9% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 65.9% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 65.9% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 65.9% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 65.9% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 65.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 65.9% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 65.9% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 65.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 65.9% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 65.9% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 65.9% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 65.9% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 65.9% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 65.4% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 65.4% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 65.4% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 65.3% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 65.3% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 65.3% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 65.3% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 65.2% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 65.2% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 65.2% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 65.2% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 65.2% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 65.2% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 65.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 65.2% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 65.1% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 65.1% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 65.1% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 64.7% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 64.7% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 64.7% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 64.7% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 64.7% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 64.6% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 59.0% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 55.2% | `<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio_rustls::server::TlsStream<tokio::net::tcp:` |
| 55.2% | `campfire_kit::front::conn::serve_connection::<tokio_rustls::server::TlsStream<tokio::net::tcp::stream::TcpStream>>::{closure}::{closure}::{closure}` |
| 54.8% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 54.7% | `campfire_kit::front::serve_with::<campfire::app::serve::{closure}::{closure}>::{closure}::{closure}::{closure}` |
| 54.7% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::alloc::Global>>` |
| 54.6% | `<campfire_kit::front::handler::Handler>::call::{closure}` |
| 54.4% | `campfire_kit::{async_fn#0}` |
| 53.6% | `<campfire_kit::front::handler::Handler>::proxy::{closure}` |
| 53.3% | `poll<axum::routing::Router<()>, http::request::Request<axum_core::body::Body>>` |
| 53.1% | `<axum::routing::route::RouteFuture<core::convert::Infallible> as core::future::future::Future>::poll` |
| 53.1% | `poll<tower::util::boxed_clone_sync::BoxCloneSyncService<http::request::Request<axum_core::body::Body>, http::response::Response<axum_core::body::Body>, core::co` |
| 53.0% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 53.0% | `<axum::util::MapIntoResponseFuture<tower::util::map_err::MapErrFuture<axum::middleware::from_fn::ResponseFuture, <core::convert::Infallible as core::convert::In` |
| 52.9% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 52.9% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 52.9% | `poll<axum::middleware::from_fn::ResponseFuture, fn(core::convert::Infallible) -> core::convert::Infallible>` |
| 52.9% | `axum::poll` |
| 52.9% | `try_poll<axum::middleware::from_fn::ResponseFuture, http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 52.9% | `poll<axum::middleware::from_fn::ResponseFuture>` |
| 52.9% | `<axum::middleware::from_fn::FromFn<campfire_kit::deflater::deflater, (), axum::routing::route::Route, (http::request::Request<axum_core::body::Body>,)> as tower` |
| 48.0% | `<axum::middleware::from_fn::Next>::run::{closure}` |
| 47.9% | `<axum::util::MapIntoResponseFuture<axum::routing::route::RouteFuture<core::convert::Infallible>> as core::future::future::Future>::poll` |
| 47.7% | `<axum::middleware::from_fn::FromFn<campfire_kit::adapter::rails_middleware, campfire_kit::app::Kit, axum::routing::route::Route, (axum::extract::state::State<ca` |
