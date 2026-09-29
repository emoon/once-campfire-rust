# campfire base: post_message

308 samples at 1000 µs (0.31 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **41.9%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 14.3% |
| sqlite (C) | malloc/free/realloc | 1.0% |
| sqlite (C) | memcpy/memmove/memset | 1.0% |
| **app (controllers/channels/jobs)** | (all) | **12.0%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 2.9% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 2.6% |
| **tokio runtime / scheduling** | (all) | **11.7%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 8.8% |
| **kit (request/response plumbing)** | (all) | **8.4%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.6% |
| kit (request/response plumbing) | malloc/free/realloc | 1.3% |
| **db models / queries** | (all) | **7.8%** |
| db models / queries | syscalls (read/write/epoll/futex) | 4.5% |
| db models / queries | memcpy/memmove/memset | 0.6% |
| **gzip (miniz_oxide/crc32)** | (all) | **7.1%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.3% |
| **rich text (Action Text pipeline)** | (all) | **3.6%** |
| rich text (Action Text pipeline) | malloc/free/realloc | 1.3% |
| **cable** | (all) | **1.9%** |
| **askama render / view helpers** | (all) | **1.9%** |
| askama render / view helpers | memcpy/memmove/memset | 0.6% |
| **json (serde_json)** | (all) | **1.6%** |
| **hyper / http** | (all) | **1.0%** |
| **crypto / signing (rails_compat)** | (all) | **1.0%** |

## Top self

| self | function |
|---|---|
| 18.2% | `__syscall_cancel_arch` |
| 8.8% | `syscall` |
| 5.8% | `__memcpy_avx512_unaligned_erms` |
| 4.9% | `munmap` |
| 2.9% | `libsqlite3_sys::sqlite3VdbeExec` |
| 2.3% | `cache_bin_alloc_impl` |
| 1.3% | `_rjem_je_arena_ralloc_no_move` |
| 1.3% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.3% | `__memset_avx512_unaligned_erms` |
| 1.3% | `zlib_rs::longest_match_help<false>` |
| 1.0% | `get<std::sys::thread_local::native::eager::State>` |
| 1.0% | `core::is_ascii_uppercase` |
| 1.0% | `libsqlite3_sys::sqlite3DbMallocRawNN` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.6% | `serde_json::ser::format_escaped_str_contents::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>` |
| 0.6% | `__GI___mmap64` |
| 0.6% | `tcache_get_n` |
| 0.6% | `libsqlite3_sys::sqlite3VdbeMemRelease` |
| 0.6% | `__strlen_evex512` |
| 0.6% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.6% | `core::add<u8>` |
| 0.6% | `libsqlite3_sys::unixShmLock` |
| 0.6% | `__GI___lll_lock_wake` |
| 0.6% | `core::swap_chunk<8>` |
| 0.6% | `sz_index2size_lookup_impl` |
| 0.6% | `next_code_point<core::slice::iter::Iter<u8>>` |
| 0.6% | `libsqlite3_sys::walFindFrame.constprop.0` |
| 0.6% | `libsqlite3_sys::btreeInitPage` |
| 0.3% | `core::wrapping_sub` |
| 0.3% | `libsqlite3_sys::lockBtree` |
| 0.3% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<core::option::Option<campfire_db::m` |
| 0.3% | `libsqlite3_sys::sqlite3VdbeMemShallowCopy` |
| 0.3% | `core::lt` |
| 0.3% | `as_mut<hyper::server::conn::http1::Connection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util::ServiceFn<campfire_kit::` |
| 0.3% | `libsqlite3_sys::decodeFlags` |
| 0.3% | `core::eq<&str>` |
| 0.3% | `to_vec<campfire_richtext::dom::Attr, alloc::alloc::Global>` |
| 0.3% | `parking_lot::lock` |
| 0.3% | `serialize<&mut serde_json::ser::Serializer<&mut alloc::vec::Vec<u8, alloc::alloc::Global>, serde_json::ser::CompactFormatter>>` |
| 0.3% | `libsqlite3_sys::walIndexAppend` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 100.0% | `start_thread` |
| 100.0% | `__GI___clone3` |
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 76.6% | `tokio::{closure}` |
| 76.6% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 76.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 76.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 76.6% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 76.6% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 76.6% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 76.6% | `tokio::run` |
| 76.6% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 72.7% | `tokio::poll` |
| 72.7% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 43.5% | `campfire::{closure}` |
| 34.4% | `libsqlite3_sys::sqlite3_step` |
| 34.4% | `libsqlite3_sys::sqlite3VdbeExec` |
| 34.4% | `libsqlite3_sys::sqlite3Step` |
| 34.4% | `rusqlite::step` |
| 32.8% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 32.8% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 32.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 32.8% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 32.8% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 32.8% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 32.8% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 32.8% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 32.8% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 32.8% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 32.8% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 32.8% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 32.8% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 32.8% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 32.8% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 32.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 32.8% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 31.2% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 30.5% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 30.5% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 30.5% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 28.9% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 28.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 28.9% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 28.9% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 28.9% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 28.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 28.9% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 28.9% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 28.9% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 28.9% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 28.9% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 28.9% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 28.9% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 28.9% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 28.6% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 28.6% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 28.2% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 28.2% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 28.2% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 28.2% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 24.0% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 23.7% | `campfire_db::{closure}` |
| 22.1% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}:` |
| 22.1% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>>` |
| 22.1% | `{closure}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>` |
| 22.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::ope` |
| 22.1% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>>` |
| 22.1% | `run_write<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::messages::create_message::{async_f` |
| 22.1% | `<<campfire_db::database::Database>::write<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::me` |
| 22.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::open::{c` |
| 22.1% | `std::sys::backtrace::__rust_begin_short_backtrace::<<campfire_db::database::Database>::open::{closure}, ()>` |
| 22.1% | `call_once<(&rusqlite::Connection, &campfire_db::database::Env), (dyn core::ops::function::FnOnce<(&rusqlite::Connection, &campfire_db::database::Env), Output=()` |
| 22.1% | `call_once<(), campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>` |
| 22.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>, ()>` |
| 22.1% | `<std::thread::lifecycle::spawn_unchecked<<campfire_db::database::Database>::open::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>::call_once::{shi` |
| 22.1% | `{closure}<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::messages::create_message::{async_f` |
| 22.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>, ()>` |
| 19.5% | `rusqlite::next` |
| 18.5% | `<rusqlite::row::Rows as fallible_streaming_iterator::FallibleStreamingIterator>::advance (.10624)` |
