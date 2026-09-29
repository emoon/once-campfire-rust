# campfire tip: post_message (perf, fp)

14769000 samples at 1 µs (14.77 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **33.4%** |
| sqlite (C) | malloc/free/realloc | 2.0% |
| sqlite (C) | memcpy/memmove/memset | 1.1% |
| sqlite (C) | syscalls (read/write/epoll/futex) | 0.7% |
| **gzip (miniz_oxide/crc32)** | (all) | **17.3%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 2.0% |
| **kit (request/response plumbing)** | (all) | **11.5%** |
| kit (request/response plumbing) | malloc/free/realloc | 1.3% |
| kit (request/response plumbing) | memcpy/memmove/memset | 1.0% |
| **app (controllers/channels/jobs)** | (all) | **9.8%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 2.4% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.7% |
| **rich text (Action Text pipeline)** | (all) | **6.5%** |
| rich text (Action Text pipeline) | malloc/free/realloc | 0.9% |
| **crypto / signing (rails_compat)** | (all) | **5.3%** |
| crypto / signing (rails_compat) | malloc/free/realloc | 1.6% |
| crypto / signing (rails_compat) | memcpy/memmove/memset | 0.6% |
| **db models / queries** | (all) | **3.9%** |
| **askama render / view helpers** | (all) | **3.4%** |
| askama render / view helpers | malloc/free/realloc | 0.6% |
| askama render / view helpers | memcpy/memmove/memset | 0.6% |
| **tokio runtime / scheduling** | (all) | **3.0%** |
| **json (serde_json)** | (all) | **3.0%** |
| **hyper / http** | (all) | **2.7%** |
| **cable** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 4.7% | `__memcpy_avx512_unaligned_erms` |
| 2.9% | `libsqlite3_sys::sqlite3VdbeExec` |
| 2.2% | `lll_mutex_unlock_optimized` |
| 1.9% | `__memset_avx512_unaligned_erms` |
| 1.9% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.7% | `zlib_rs::longest_match_help<false>` |
| 1.5% | `libsqlite3_sys::walChecksumBytes` |
| 1.3% | `format_escaped_str_contents<&mut alloc::vec::Vec<u8, alloc::alloc::Global>, rails_compat::json::RubyFormatter>` |
| 1.3% | `<rails_compat::json::RubyFormatter as serde_json::ser::Formatter>::write_string_fragment::<&mut alloc::vec::Vec<u8>>` |
| 1.2% | `imalloc_fastpath` |
| 1.2% | `__memcmp_evex_movbe` |
| 1.1% | `__strlen_evex512` |
| 1.0% | `cache_bin_alloc_impl` |
| 1.0% | `__pthread_mutex_lock` |
| 1.0% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 0.9% | `libsqlite3_sys::walFindFrame.constprop.0` |
| 0.9% | `zlib_rs::is_match<8>` |
| 0.8% | `lll_mutex_lock_optimized` |
| 0.8% | `core::next<u8>` |
| 0.8% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.8% | `zlib_rs::send_bits` |
| 0.7% | `zlib_rs::insert_string` |
| 0.6% | `_rjem_je_arena_ralloc_no_move` |
| 0.6% | `_int_malloc` |
| 0.6% | `core::copy_nonoverlapping<u8>` |
| 0.5% | `_rjem_sdallocx` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `core::add<u8>` |
| 0.5% | `free_fastpath` |
| 0.5% | `libsqlite3_sys::columnMem` |
| 0.5% | `libsqlite3_sys::pcache1TruncateUnsafe` |
| 0.5% | `core::str::validations::run_utf8_validation` |
| 0.5% | `core_arch::_mm_shuffle_epi32<14>` |
| 0.5% | `libsqlite3_sys::columnName` |
| 0.4% | `zlib_rs::quick_insert_value` |
| 0.4% | `perf-vdso.so-gEe8A9+0xd4c` |
| 0.4% | `core_arch::_mm_add_epi32` |
| 0.4% | `core::index<u8>` |
| 0.4% | `__libc_malloc2` |
| 0.4% | `core::split_at_checked<u8>` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `__GI___clone3` |
| 100.0% | `start_thread` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 87.1% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 87.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 87.1% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 87.1% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 87.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 87.1% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 87.1% | `tokio::{closure}` |
| 87.1% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 87.1% | `tokio::run` |
| 87.0% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 87.0% | `tokio::poll` |
| 71.1% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 71.1% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 71.1% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 71.1% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 71.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 71.1% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 71.1% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 71.1% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 71.1% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 71.1% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 71.1% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 71.1% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 71.1% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 71.1% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 71.1% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 71.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 71.1% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 70.1% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 70.1% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 70.1% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 70.0% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 69.2% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 69.2% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 69.1% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 69.1% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 69.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 69.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 69.1% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 69.1% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 69.1% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 69.1% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 69.1% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 69.0% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 69.0% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 69.0% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 68.5% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 68.5% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 68.4% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 68.4% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 68.4% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 68.3% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 67.5% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 65.8% | `<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio_rustls::server::TlsStream<tokio::net::tcp:` |
| 65.8% | `campfire_kit::front::conn::serve_connection::<tokio_rustls::server::TlsStream<tokio::net::tcp::stream::TcpStream>>::{closure}::{closure}::{closure}` |
| 65.6% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 65.4% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::alloc::Global>>` |
| 65.4% | `campfire_kit::front::serve_with::<campfire::app::serve::{closure}::{closure}>::{closure}::{closure}::{closure}` |
| 65.2% | `<campfire_kit::front::handler::Handler>::call::{closure}` |
| 64.9% | `campfire_kit::{async_fn#0}` |
| 64.2% | `<campfire_kit::front::handler::Handler>::proxy::{closure}` |
| 63.8% | `poll<axum::routing::Router<()>, http::request::Request<axum_core::body::Body>>` |
| 63.5% | `<axum::routing::route::RouteFuture<core::convert::Infallible> as core::future::future::Future>::poll` |
| 63.5% | `poll<tower::util::boxed_clone_sync::BoxCloneSyncService<http::request::Request<axum_core::body::Body>, http::response::Response<axum_core::body::Body>, core::co` |
| 63.3% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 63.2% | `<axum::util::MapIntoResponseFuture<tower::util::map_err::MapErrFuture<axum::middleware::from_fn::ResponseFuture, <core::convert::Infallible as core::convert::In` |
| 63.2% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 63.2% | `poll<axum::middleware::from_fn::ResponseFuture, fn(core::convert::Infallible) -> core::convert::Infallible>` |
| 63.2% | `poll<futures_util::future::try_future::into_future::IntoFuture<axum::middleware::from_fn::ResponseFuture>, futures_util::fns::MapErrFn<fn(core::convert::Infalli` |
| 63.2% | `try_poll<axum::middleware::from_fn::ResponseFuture, http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 63.2% | `poll<axum::middleware::from_fn::ResponseFuture>` |
| 63.2% | `axum::poll` |
| 63.2% | `<axum::middleware::from_fn::FromFn<campfire_kit::deflater::deflater, (), axum::routing::route::Route, (http::request::Request<axum_core::body::Body>,)> as tower` |
| 48.8% | `campfire::{closure}` |
| 45.0% | `<axum::middleware::from_fn::Next>::run::{closure}` |
| 44.9% | `<axum::util::MapIntoResponseFuture<axum::routing::route::RouteFuture<core::convert::Infallible>> as core::future::future::Future>::poll` |
