# campfire web-1: static_css

5190 samples at 1000 µs (5.19 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **kit (request/response plumbing)** | (all) | **49.0%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 15.4% |
| kit (request/response plumbing) | memcpy/memmove/memset | 3.4% |
| kit (request/response plumbing) | malloc/free/realloc | 3.4% |
| **tokio runtime / scheduling** | (all) | **35.0%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 25.0% |
| **hyper / http** | (all) | **12.1%** |
| hyper / http | malloc/free/realloc | 0.5% |
| **app (controllers/channels/jobs)** | (all) | **3.9%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 1.3% |

## Top self

| self | function |
|---|---|
| 35.5% | `__syscall_cancel_arch` |
| 3.7% | `__memcpy_avx512_unaligned_erms` |
| 3.1% | `syscall` |
| 2.2% | `0x7fac3dab6d4c` |
| 2.1% | `core::fmt::write` |
| 1.8% | `core::sync::atomic::atomic_load::<u32>` |
| 1.8% | `<alloc::string::String as core::fmt::Write>::write_str` |
| 1.2% | `append_elements<u8, alloc::alloc::Global>` |
| 0.9% | `parking_lot::unlock` |
| 0.8% | `parking_lot::lock` |
| 0.7% | `write_prefix<dyn core::fmt::Write>` |
| 0.7% | `drop<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>` |
| 0.7% | `imalloc_fastpath` |
| 0.7% | `clone<http::header::value::HeaderValue>` |
| 0.7% | `campfire_kit::{async_fn#0}` |
| 0.6% | `bytes::release_shared` |
| 0.6% | `<core::fmt::Formatter>::pad` |
| 0.6% | `<core::fmt::Formatter>::new` |
| 0.5% | `<core::ptr::non_null::NonNull<core::fmt::rt::Argument>>::add` |
| 0.5% | `nu_ansi_term::eq` |
| 0.5% | `write_to_any<str, dyn core::fmt::Write>` |
| 0.5% | `do_register<&core::task::wake::Waker>` |
| 0.5% | `<tracing_subscriber::fmt::format::DefaultVisitor as tracing_core::field::Visit>::record_debug` |
| 0.4% | `cache_bin_alloc_impl` |
| 0.4% | `<char as core::fmt::Display>::fmt` |
| 0.4% | `write<http::header::map::Bucket<http::header::value::HeaderValue>>` |
| 0.4% | `tokio::ref_count` |
| 0.4% | `http::write` |
| 0.4% | `__memcmp_evex_movbe` |
| 0.3% | `<nu_ansi_term::display::AnsiGenericString<str> as core::fmt::Display>::fmt` |
| 0.3% | `tokio::slot_for` |
| 0.3% | `<hyper::proto::h1::role::Server as hyper::proto::h1::Http1Transaction>::parse` |
| 0.3% | `<tracing_core::field::DisplayValue<&alloc::string::String> as tracing_core::field::Value>::record` |
| 0.3% | `_rjem_je_arena_ralloc_no_move` |
| 0.3% | `core::memchr_naive` |
| 0.3% | `core::add<u8>` |
| 0.3% | `tokio::clone` |
| 0.3% | `<nu_ansi_term::ansi::Prefix as core::fmt::Display>::fmt` |
| 0.3% | `sz_index2size_lookup_impl` |
| 0.3% | `http::header::name::parse_hdr` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 100.0% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 100.0% | `start_thread` |
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 100.0% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 100.0% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 100.0% | `tokio::{closure}` |
| 100.0% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 100.0% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 100.0% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 100.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 100.0% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 100.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 100.0% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 100.0% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 100.0% | `tokio::run` |
| 100.0% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 100.0% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 100.0% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 100.0% | `tokio::poll` |
| 100.0% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 100.0% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 100.0% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 100.0% | `__GI___clone3` |
| 100.0% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 100.0% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 100.0% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 100.0% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 100.0% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 100.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 100.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 95.2% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 95.0% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 95.0% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 94.9% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 94.8% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 94.8% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 93.9% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 93.7% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 93.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 93.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 93.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 93.7% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 93.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 93.7% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 93.6% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 93.6% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 93.5% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 93.5% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 90.8% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 90.8% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 90.8% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 90.8% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 90.8% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 90.0% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 58.8% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 35.7% | `internal_syscall_cancel` |
| 35.7% | `syscall_cancel` |
| 35.5% | `__syscall_cancel_arch` |
| 35.4% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 35.4% | `axum_core::poll_frame` |
| 35.4% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 35.3% | `poll_frame<axum_core::body::Body>` |
| 35.3% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 34.6% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::handler::LoggedBody, <axum_core::error::Error>::new<axum_core::error::Error>> as http_body::B` |
| 34.6% | `campfire_kit::poll_frame` |
| 34.5% | `<campfire_kit::front::handler::LogEntry>::write` |
| 33.9% | `tracing::{closure}` |
| 33.9% | `get_default<(), tracing_core::event::{impl#0}::dispatch::{closure_env#0}>` |
| 33.9% | `tracing_core::event` |
| 33.9% | `tracing_core::{closure}` |
| 33.9% | `tracing_core::dispatch` |
| 33.9% | `<tracing_subscriber::fmt::Subscriber<tracing_subscriber::fmt::format::DefaultFields, tracing_subscriber::fmt::format::Format, tracing_subscriber::filter::env::E` |
| 33.9% | `event<tracing_subscriber::filter::env::EnvFilter, tracing_subscriber::layer::layered::Layered<tracing_subscriber::fmt::fmt_layer::Layer<tracing_subscriber::regi` |
| 33.9% | `event<tracing_subscriber::fmt::fmt_layer::Layer<tracing_subscriber::registry::sharded::Registry, tracing_subscriber::fmt::format::DefaultFields, tracing_subscri` |
| 33.9% | `on_event<tracing_subscriber::registry::sharded::Registry, tracing_subscriber::fmt::format::DefaultFields, tracing_subscriber::fmt::format::Format<tracing_subscr` |
| 33.9% | `<std::thread::local::LocalKey<core::cell::RefCell<alloc::string::String>>>::with::<<tracing_subscriber::fmt::fmt_layer::Layer<tracing_subscriber::registry::shar` |
| 33.9% | `try_with<core::cell::RefCell<alloc::string::String>, tracing_subscriber::fmt::fmt_layer::{impl#12}::on_event::{closure_env#0}<tracing_subscriber::registry::shar` |
| 33.8% | `{closure}<tracing_subscriber::registry::sharded::Registry, tracing_subscriber::fmt::format::DefaultFields, tracing_subscriber::fmt::format::Format<tracing_subsc` |
