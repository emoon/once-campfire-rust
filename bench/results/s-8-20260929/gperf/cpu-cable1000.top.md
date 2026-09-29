# campfire base: cable1000

27110 samples at 1000 µs (27.11 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **tokio runtime / scheduling** | (all) | **72.8%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 67.5% |
| **cable** | (all) | **14.2%** |
| cable | malloc/free/realloc | 4.5% |
| **sqlite (C)** | (all) | **6.1%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 1.7% |
| **app (controllers/channels/jobs)** | (all) | **2.2%** |
| **kit (request/response plumbing)** | (all) | **1.3%** |
| **db models / queries** | (all) | **0.9%** |
| **rich text (Action Text pipeline)** | (all) | **0.9%** |
| **hyper / http** | (all) | **0.8%** |
| **askama render / view helpers** | (all) | **0.3%** |
| **json (serde_json)** | (all) | **0.2%** |
| **crypto / signing (rails_compat)** | (all) | **0.2%** |
| **gzip (miniz_oxide/crc32)** | (all) | **0.0%** |
| **other** | (all) | **0.0%** |

## Top self

| self | function |
|---|---|
| 67.5% | `__syscall_cancel_arch` |
| 1.8% | `syscall` |
| 1.1% | `futures_core::register` |
| 1.1% | `parking_lot::unlock` |
| 1.0% | `poll_next<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream:` |
| 0.8% | `campfire_cable::connection::run::<campfire::channels::CableUser>::{closure}` |
| 0.8% | `parking_lot::lock` |
| 0.8% | `__memcpy_avx512_unaligned_erms` |
| 0.7% | `try_advancing_head<campfire_cable::socket::Incoming>` |
| 0.6% | `munmap` |
| 0.5% | `do_register<&core::task::wake::Waker>` |
| 0.5% | `std::lock` |
| 0.4% | `cache_bin_alloc_impl` |
| 0.4% | `tokio::ref_count` |
| 0.4% | `as_ptr<futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<core::pin::Pin<alloc::boxed::Box<(dyn futures` |
| 0.4% | `<tokio::net::tcp::stream::TcpStream as tokio::io::async_write::AsyncWrite>::poll_write_vectored` |
| 0.4% | `libsqlite3_sys::sqlite3VdbeExec` |
| 0.3% | `core::atomic_load<u8>` |
| 0.3% | `tokio::bitand` |
| 0.3% | `tokio::ref_inc` |
| 0.3% | `{async_fn#0}<campfire_cable::socket::Frame>` |
| 0.3% | `atomic_store<*mut futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable` |
| 0.2% | `dequeue<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::S` |
| 0.2% | `clone<campfire_cable::socket::Payload, alloc::alloc::Global>` |
| 0.2% | `0x7fcb133f4d4c` |
| 0.2% | `pop<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 0.2% | `__memcmp_evex_movbe` |
| 0.2% | `sched_yield` |
| 0.2% | `sz_index2size_lookup_impl` |
| 0.2% | `as_ptr<futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin:` |
| 0.2% | `link<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin::Pin<alloc::boxed::Box<(dyn futures_core::stream::Stre` |
| 0.2% | `<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}` |
| 0.2% | `drop<campfire_cable::socket::Payload, alloc::alloc::Global>` |
| 0.2% | `libsqlite3_sys::btreeInitPage` |
| 0.2% | `__GI___mmap64` |
| 0.2% | `drop<futures_util::stream::futures_unordered::task::Task<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin::P` |
| 0.2% | `tokio::get_num_notify_waiters_calls` |
| 0.2% | `unwrap<core::ptr::non_null::NonNull<tokio::sync::broadcast::Waiter>>` |
| 0.2% | `<futures_util::stream::futures_unordered::FuturesUnordered<futures_util::stream::stream::into_future::StreamFuture<futures_util::abortable::Abortable<core::pin:` |
| 0.2% | `replace<core::option::Option<campfire_cable::connection::Close>>` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `__GI___clone3` |
| 100.0% | `start_thread` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 97.3% | `tokio::{closure}` |
| 97.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 97.2% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 97.2% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 97.2% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 97.2% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 97.2% | `tokio::run` |
| 97.2% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 97.2% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 96.3% | `tokio::poll` |
| 96.3% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 87.7% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 87.7% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 87.7% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 87.7% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 87.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 87.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 87.7% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 87.7% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 87.7% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 87.7% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 87.7% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 87.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 87.7% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 87.7% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 87.7% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 87.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 87.7% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 86.7% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 86.4% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 86.4% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 86.3% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 82.7% | `tokio::runtime::task::raw::poll::<<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}, alloc::sync::Arc<tokio::runtime::` |
| 82.6% | `poll_inner<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler:` |
| 82.6% | `poll<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler::multi` |
| 82.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{asy` |
| 82.6% | `poll_future<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler` |
| 82.6% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::` |
| 82.6% | `{closure}<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>, alloc::sync::Arc<tokio::runtime::scheduler::` |
| 82.6% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_b` |
| 82.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_cable::server::{impl#3}::call::{async_fn` |
| 82.5% | `with_mut<tokio::runtime::task::core::Stage<campfire_cable::server::{impl#3}::call::{async_fn#0}::{async_block_env#0}<campfire::channels::CableUser>>, core::task` |
| 82.5% | `<campfire_cable::server::Server<campfire::channels::CableUser>>::call::{closure}::{closure}` |
| 82.3% | `campfire_cable::connection::run::<campfire::channels::CableUser>::{closure}` |
| 68.3% | `{async_fn#0}<campfire::channels::CableUser>` |
| 68.1% | `<campfire_cable::socket::Writer<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>>::send::{closure}` |
| 67.8% | `<campfire_cable::socket::Writer<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>>::write::{closure}` |
| 67.7% | `syscall_cancel` |
| 67.7% | `internal_syscall_cancel` |
| 67.5% | `__syscall_cancel_arch` |
| 67.2% | `{async_block#0}<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>` |
| 67.2% | `poll<campfire_cable::socket::{impl#8}::write::{async_fn#0}::{async_block_env#0}<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgra` |
| 67.1% | `poll<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>` |
| 67.1% | `{async_fn#0}<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>` |
| 67.0% | `poll_write_vectored<tokio::io::split::WriteHalf<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>>` |
| 67.0% | `with_lock<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>, core::task::poll::Poll<core::result::Result<usize, core::io::error::Error>>, tokio::io::spli` |
| 67.0% | `poll_write_vectored<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>` |
| 66.6% | `<tokio::net::tcp::stream::TcpStream as tokio::io::async_write::AsyncWrite>::poll_write_vectored` |
| 66.5% | `{closure}<hyper_util::rt::tokio::TokioIo<hyper::upgrade::Upgraded>>` |
| 66.4% | `poll_write_vectored<hyper::upgrade::Upgraded>` |
| 66.4% | `hyper::poll_write_vectored` |
| 66.4% | `poll_write_vectored<(dyn hyper::upgrade::Io + core::marker::Send)>` |
| 66.4% | `poll_write_vectored<alloc::boxed::Box<(dyn hyper::upgrade::Io + core::marker::Send), alloc::alloc::Global>>` |
| 66.2% | `poll_write_io<usize, tokio::io::poll_evented::{impl#6}::poll_write_vectored::{closure_env#0}<mio::net::tcp::stream::TcpStream>>` |
| 66.2% | `poll_io<usize, tokio::io::poll_evented::{impl#6}::poll_write_vectored::{closure_env#0}<mio::net::tcp::stream::TcpStream>>` |
| 66.2% | `poll_write_vectored<mio::net::tcp::stream::TcpStream>` |
| 66.2% | `tokio::poll_write_vectored_priv` |
| 65.9% | `mio::{closure}` |
| 65.9% | `mio::write_vectored` |
| 65.9% | `writev` |
| 65.9% | `<std::sys::fd::unix::FileDesc>::write_vectored` |
| 65.9% | `do_io<std::net::tcp::TcpStream, mio::net::tcp::stream::{impl#4}::write_vectored::{closure_env#0}, usize>` |
| 65.9% | `{closure}<mio::net::tcp::stream::TcpStream>` |
| 65.9% | `<std::sys::net::connection::socket::unix::Socket>::write_vectored` |
| 65.9% | `<std::sys::net::connection::socket::TcpStream>::write_vectored` |
| 65.9% | `<&std::net::tcp::TcpStream as std::io::Write>::write_vectored` |
