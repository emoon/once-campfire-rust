# campfire fp: cable1000 (perf, fp)

10410000 samples at 1 µs (10.41 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **cable** | (all) | **48.8%** |
| cable | malloc/free/realloc | 15.1% |
| **tokio runtime / scheduling** | (all) | **18.7%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 1.7% |
| tokio runtime / scheduling | malloc/free/realloc | 0.9% |
| **sqlite (C)** | (all) | **12.2%** |
| sqlite (C) | malloc/free/realloc | 1.2% |
| sqlite (C) | memcpy/memmove/memset | 0.7% |
| **app (controllers/channels/jobs)** | (all) | **5.0%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 1.1% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 0.9% |
| **kit (request/response plumbing)** | (all) | **4.4%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.6% |
| **rich text (Action Text pipeline)** | (all) | **2.6%** |
| **hyper / http** | (all) | **2.4%** |
| **db models / queries** | (all) | **1.9%** |
| **askama render / view helpers** | (all) | **1.6%** |
| **crypto / signing (rails_compat)** | (all) | **1.2%** |
| **json (serde_json)** | (all) | **1.0%** |
| **gzip (miniz_oxide/crc32)** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 4.2% | `futures_core::register` |
| 3.4% | `parking_lot::unlock` |
| 3.3% | `parking_lot::lock` |
| 3.2% | `poll_next<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream:` |
| 2.7% | `__memcpy_avx512_unaligned_erms` |
| 2.6% | `campfire_cable::connection::run::<campfire::channels::CableUser>::{closure}` |
| 2.3% | `try_advancing_head<campfire_cable::socket::Incoming>` |
| 1.9% | `do_register<&core::task::wake::Waker>` |
| 1.6% | `std::lock` |
| 1.4% | `cache_bin_alloc_impl` |
| 1.3% | `tokio::ref_count` |
| 1.3% | `as_ptr<futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<core::pin::Pin<alloc::boxed::Box<(dyn futures` |
| 1.2% | `tokio::bitand` |
| 1.2% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.2% | `write<core::option::Option<core::ptr::non_null::NonNull<tokio::sync::notify::Waiter>>>` |
| 1.1% | `tokio::ref_inc` |
| 1.1% | `core::atomic_load<u8>` |
| 0.9% | `{async_fn#0}<campfire_cable::socket::Frame>` |
| 0.9% | `atomic_store<*mut futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable` |
| 0.8% | `drop<futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin::P` |
| 0.8% | `clone<campfire_cable::socket::Payload, alloc::alloc::Global>` |
| 0.8% | `<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}` |
| 0.8% | `<tokio::sync::broadcast::Receiver<()>>::recv_ref` |
| 0.8% | `pop<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 0.8% | `core::atomic_or<usize, usize>` |
| 0.7% | `<tokio::net::tcp::stream::TcpStream as tokio::io::async_write::AsyncWrite>::poll_write_vectored` |
| 0.7% | `link<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stre` |
| 0.7% | `<tokio::sync::broadcast::Receiver<campfire_cable::socket::Frame>>::recv_ref` |
| 0.7% | `perf-vdso.so-tZ0rzb+0xd4c` |
| 0.7% | `replace<core::option::Option<campfire_cable::connection::Close>>` |
| 0.7% | `tokio::get_num_notify_waiters_calls` |
| 0.7% | `drop<campfire_cable::socket::Payload, alloc::alloc::Global>` |
| 0.7% | `core::atomic_compare_exchange_weak<u8>` |
| 0.6% | `unwrap<core::ptr::non_null::NonNull<tokio::sync::broadcast::Waiter>>` |
| 0.6% | `__memcmp_evex_movbe` |
| 0.6% | `as_ptr<futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin:` |
| 0.6% | `<futures_util::stream::select_all::SelectAll<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<dyn futures_core::stream::Stream<Item = core::r` |
| 0.6% | `sz_index2size_lookup_impl` |
| 0.6% | `<campfire_cable::socket::Writer<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>>::send::{closure}` |
| 0.6% | `get<tokio::sync::mpsc::chan::RxFields<campfire_cable::socket::Incoming>>` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 100.0% | `__GI___clone3` |
| 100.0% | `start_thread` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 94.8% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 94.8% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 94.8% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 94.8% | `tokio::{closure}` |
| 94.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 94.8% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 94.8% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 94.8% | `tokio::run` |
| 94.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 94.7% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 94.7% | `tokio::poll` |
| 71.7% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 71.7% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 71.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 71.7% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 71.7% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 71.7% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 71.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 71.7% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 71.7% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 71.7% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 71.7% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 71.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 71.7% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 71.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 71.7% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 71.7% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 71.7% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 70.0% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 70.0% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 70.0% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 69.8% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 59.2% | `tokio::runtime::task::raw::poll::<<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}, alloc::sync::Arc<tokio::runtime::` |
| 59.1% | `poll<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler::multi` |
| 59.0% | `poll_inner<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler:` |
| 58.8% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::` |
| 58.8% | `poll_future<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler` |
| 58.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{asy` |
| 58.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{async_fn` |
| 58.7% | `{closure}<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler::` |
| 58.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_b` |
| 58.7% | `with_mut<tokio::runtime::task::core::Stage<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>>, core::task` |
| 58.7% | `<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}` |
| 57.7% | `campfire_cable::connection::run::<campfire::channels::CableUser>::{closure}` |
| 26.0% | `{closure}<campfire::channels::CableUser>` |
| 26.0% | `poll<campfire_cable::connection::run::{async_fn#0}::__tokio_select_util::Out<core::option::Option<campfire_cable::socket::Incoming>, core::option::Option<core::` |
| 24.6% | `campfire::{closure}` |
| 24.5% | `poll<futures_util::stream::select_all::SelectAll<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core` |
| 24.5% | `poll_next_unpin<futures_util::stream::select_all::SelectAll<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Strea` |
| 23.4% | `<futures_util::stream::select_all::SelectAll<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<dyn futures_core::stream::Stream<Item = core::r` |
| 20.7% | `poll_next<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream:` |
| 20.7% | `poll_next_unpin<futures_util::stream::futures_unordered::FuturesUnordered<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abort` |
| 14.9% | `now_or_never<futures_util::stream::stream::next::Next<futures_util::stream::select_all::SelectAll<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed` |
| 14.2% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<(), campfire::controllers::messages::broa` |
| 14.2% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broa` |
| 14.2% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages:` |
| 14.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 14.2% | `catch_unwind<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harne` |
| 14.2% | `call_once<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtim` |
| 14.2% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 14.2% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages` |
| 14.2% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::message` |
| 14.2% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>,` |
| 14.2% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(),` |
| 14.2% | `with<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 14.2% | `{closure}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 13.8% | `poll_next<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core::result::Result<campfire_cable::socket::Frame, campfire_cable::pubsub::Re` |
| 11.1% | `poll<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core::result::Result<campfire_cable::socket::Fra` |
| 11.1% | `try_poll<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core::result::Result<campfire_cable::socket::Frame, campfire_cable::pubsub::Rec` |
| 11.1% | `poll_next_unpin<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stream<Item=core::result::Result<campfire_cable::` |
| 10.3% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 10.3% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 10.3% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 10.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 10.2% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 10.2% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
