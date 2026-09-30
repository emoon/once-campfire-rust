# campfire live9: cable10000 (perf, fp)

14018000 samples at 1 µs (14.02 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **cable** | (all) | **76.0%** |
| cable | malloc/free/realloc | 19.1% |
| **tokio runtime / scheduling** | (all) | **14.3%** |
| tokio runtime / scheduling | malloc/free/realloc | 4.0% |
| **hyper / http** | (all) | **3.8%** |
| **sqlite (C)** | (all) | **2.2%** |
| **app (controllers/channels/jobs)** | (all) | **1.2%** |
| **kit (request/response plumbing)** | (all) | **1.0%** |
| **rich text (Action Text pipeline)** | (all) | **0.5%** |
| **db models / queries** | (all) | **0.4%** |
| **askama render / view helpers** | (all) | **0.2%** |
| **crypto / signing (rails_compat)** | (all) | **0.2%** |
| **json (serde_json)** | (all) | **0.1%** |
| **gzip (miniz_oxide/crc32)** | (all) | **0.0%** |

## Top self

| self | function |
|---|---|
| 6.4% | `std::lock` |
| 5.2% | `as_ptr<futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<core::pin::Pin<alloc::boxed::Box<(dyn futures` |
| 5.2% | `try_advancing_head<campfire_cable::socket::Incoming>` |
| 5.0% | `std::unlock` |
| 4.4% | `try_poll<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core::result::Result<campfire_cable::socket::Frame, campfire_cable::pubsub::Rec` |
| 4.1% | `unwrap<core::ptr::non_null::NonNull<tokio::sync::broadcast::Waiter>>` |
| 3.9% | `pop<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 3.6% | `tokio::bitand` |
| 3.6% | `as_mut<campfire_cable::merge::Slot<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core::result::Resu` |
| 3.5% | `core::atomic_compare_exchange<usize>` |
| 3.5% | `<tokio::net::tcp::stream::TcpStream as tokio::io::async_write::AsyncWrite>::poll_write_vectored` |
| 3.3% | `<campfire_cable::socket::Writer<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>>::send::{closure}` |
| 3.0% | `replace<core::option::Option<campfire_cable::connection::Close>>` |
| 2.4% | `<campfire_cable::merge::SlotWaker>::enqueue` |
| 2.4% | `campfire_cable::connection::run::<campfire::channels::CableUser>::{closure}` |
| 2.3% | `std::done` |
| 2.1% | `tokio::poll` |
| 1.9% | `<campfire_cable::socket::Writer<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>>::write::{closure}` |
| 1.8% | `<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}` |
| 1.8% | `atomic_load_head_and_len_all<futures_util::stream::stream::into_future::StreamFuture<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=cam` |
| 1.6% | `{async_block#0}<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>` |
| 1.6% | `take_value<campfire_cable::pubsub::Subscriber, campfire_cable::pubsub::{impl#2}::deliveries::{closure}::{async_block_env#0}>` |
| 1.2% | `as_ref<alloc::sync::ArcInner<campfire_cable::merge::Ready>>` |
| 1.2% | `tokio::ref_count` |
| 1.1% | `get<tokio::sync::mpsc::chan::RxFields<campfire_cable::socket::Incoming>>` |
| 0.9% | `spin_next_all<futures_util::stream::stream::into_future::StreamFuture<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=campfire_cable::co` |
| 0.9% | `futures_core::register` |
| 0.9% | `parking_lot::lock` |
| 0.8% | `core::atomic_store<u8>` |
| 0.7% | `core::atomic_load<usize>` |
| 0.7% | `clone<campfire_cable::socket::Payload, alloc::alloc::Global>` |
| 0.6% | `drop<campfire_cable::socket::Payload, alloc::alloc::Global>` |
| 0.6% | `push_mut<campfire_cable::socket::Frame, alloc::alloc::Global>` |
| 0.6% | `parking_lot::unlock` |
| 0.6% | `project<campfire_cable::socket::Frame>` |
| 0.5% | `tokio::ref_inc` |
| 0.5% | `<tokio::sync::broadcast::Shared<campfire_cable::socket::Frame>>::notify_rx` |
| 0.4% | `<campfire_cable::merge::Merge<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<dyn futures_core::stream::Stream<Item = core::result::Result<c` |
| 0.4% | `project<campfire_cable::pubsub::Subscriber, campfire_cable::pubsub::{impl#2}::deliveries::{closure}::{async_block_env#0}>` |
| 0.4% | `core::clone` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `start_thread` |
| 100.0% | `__GI___clone3` |
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 98.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 98.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 98.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 98.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 98.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 98.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 98.9% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 98.9% | `tokio::run` |
| 98.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 98.9% | `tokio::poll` |
| 98.9% | `tokio::{closure}` |
| 98.4% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 98.4% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 98.4% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 98.4% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 98.4% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 98.4% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 98.4% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 98.4% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 98.4% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 98.4% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 98.4% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 98.4% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 98.4% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 98.4% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 98.4% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 98.4% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 98.4% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 94.1% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 94.1% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 94.1% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 94.0% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 72.6% | `tokio::runtime::task::raw::poll::<<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}, alloc::sync::Arc<tokio::runtime::` |
| 72.5% | `poll<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler::multi` |
| 72.5% | `poll_inner<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler:` |
| 72.3% | `poll_future<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler` |
| 72.3% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{asy` |
| 72.3% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_b` |
| 72.3% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::` |
| 72.3% | `{closure}<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler::` |
| 72.3% | `with_mut<tokio::runtime::task::core::Stage<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>>, core::task` |
| 72.3% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{async_fn` |
| 72.3% | `<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}` |
| 70.5% | `campfire_cable::connection::run::<campfire::channels::CableUser>::{closure}` |
| 32.3% | `{closure}<campfire::channels::CableUser>` |
| 32.3% | `poll<campfire_cable::connection::run::{async_fn#0}::__tokio_select_util::Out<core::option::Option<campfire_cable::socket::Incoming>, core::option::Option<core::` |
| 28.3% | `poll<campfire_cable::merge::Merge<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core::result::Resul` |
| 28.3% | `poll_next_unpin<campfire_cable::merge::Merge<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core::re` |
| 26.7% | `<campfire_cable::merge::Merge<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<dyn futures_core::stream::Stream<Item = core::result::Result<c` |
| 20.5% | `{async_fn#0}<campfire::channels::CableUser>` |
| 19.6% | `<campfire_cable::socket::Writer<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>>::send::{closure}` |
| 19.0% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 19.0% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 18.9% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 18.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 18.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 18.9% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 18.9% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 18.9% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 18.9% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 18.9% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 18.9% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 18.9% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 18.9% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 18.9% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 18.9% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 18.9% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 18.9% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 18.9% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 18.9% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 18.9% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 18.7% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 18.5% | `<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio_rustls::server::TlsStream<tokio::net::tcp:` |
| 18.5% | `campfire_kit::front::conn::serve_connection::<tokio_rustls::server::TlsStream<tokio::net::tcp::stream::TcpStream>>::{closure}::{closure}::{closure}` |
| 18.4% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 18.4% | `campfire_kit::front::serve_with::<campfire::app::serve::{closure}::{closure}>::{closure}::{closure}::{closure}` |
