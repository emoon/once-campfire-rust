# campfire b: post_message

15781 samples at 1000 µs (15.78 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **46.7%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 18.4% |
| sqlite (C) | malloc/free/realloc | 1.8% |
| sqlite (C) | memcpy/memmove/memset | 1.0% |
| **app (controllers/channels/jobs)** | (all) | **9.7%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 2.1% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.8% |
| **tokio runtime / scheduling** | (all) | **9.7%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 7.1% |
| **gzip (miniz_oxide/crc32)** | (all) | **8.8%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.6% |
| **kit (request/response plumbing)** | (all) | **6.5%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.1% |
| kit (request/response plumbing) | malloc/free/realloc | 0.7% |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.6% |
| **db models / queries** | (all) | **5.5%** |
| db models / queries | syscalls (read/write/epoll/futex) | 3.3% |
| **rich text (Action Text pipeline)** | (all) | **3.9%** |
| **askama render / view helpers** | (all) | **2.4%** |
| askama render / view helpers | memcpy/memmove/memset | 0.7% |
| **cable** | (all) | **2.2%** |
| **hyper / http** | (all) | **1.9%** |
| **json (serde_json)** | (all) | **1.5%** |
| **crypto / signing (rails_compat)** | (all) | **1.2%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 17.0% | `__syscall_cancel_arch` |
| 7.8% | `syscall` |
| 6.6% | `munmap` |
| 5.0% | `__memcpy_avx512_unaligned_erms` |
| 2.7% | `__GI___mmap64` |
| 1.9% | `cache_bin_alloc_impl` |
| 1.6% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.5% | `__memset_avx512_unaligned_erms` |
| 1.3% | `libsqlite3_sys::btreeInitPage` |
| 1.0% | `__GI___lll_lock_wake` |
| 0.9% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 0.9% | `__fcntl64_nocancel_adjusted` |
| 0.8% | `zlib_rs::longest_match_help<false>` |
| 0.8% | `futex_wait` |
| 0.7% | `lll_mutex_unlock_optimized` |
| 0.7% | `next_code_point<core::slice::iter::Iter<u8>>` |
| 0.7% | `libsqlite3_sys::walFindFrame.constprop.0` |
| 0.6% | `serde_json::ser::format_escaped_str_contents::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>` |
| 0.6% | `__memcmp_evex_movbe` |
| 0.6% | `libsqlite3_sys::walChecksumBytes` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `_int_malloc` |
| 0.5% | `_rjem_je_arena_ralloc_no_move` |
| 0.4% | `imalloc_fastpath` |
| 0.4% | `__strlen_evex512` |
| 0.4% | `zlib_rs::insert_string` |
| 0.4% | `core::wrapping_sub` |
| 0.4% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 0.3% | `__libc_malloc2` |
| 0.3% | `lll_mutex_lock_optimized` |
| 0.3% | `zlib_rs::is_match<8>` |
| 0.3% | `__pthread_mutex_lock` |
| 0.3% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.3% | `zlib_rs::send_bits` |
| 0.3% | `libsqlite3_sys::columnName` |
| 0.3% | `libsqlite3_sys::pcache1TruncateUnsafe` |
| 0.3% | `core::next<u8>` |
| 0.3% | `libsqlite3_sys::columnMem` |
| 0.3% | `core::copy_nonoverlapping<u8>` |
| 0.3% | `core_arch::_mm_movemask_epi8` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `start_thread` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `__GI___clone3` |
| 79.3% | `tokio::{closure}` |
| 79.3% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 79.3% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 79.3% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 79.3% | `tokio::run` |
| 79.3% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 79.3% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 79.3% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 79.3% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 75.7% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 75.6% | `tokio::poll` |
| 47.4% | `campfire::{closure}` |
| 40.3% | `rusqlite::step` |
| 40.3% | `libsqlite3_sys::sqlite3_step` |
| 40.2% | `libsqlite3_sys::sqlite3Step` |
| 40.1% | `libsqlite3_sys::sqlite3VdbeExec` |
| 29.3% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 29.3% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 29.3% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 29.3% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 29.3% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 29.3% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 29.3% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 29.3% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 29.3% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 29.3% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 29.3% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 29.3% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 29.3% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 29.3% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 29.3% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 29.3% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 29.3% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 27.7% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 27.2% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 27.2% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 27.2% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 26.2% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 26.2% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 26.1% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 26.1% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 26.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 26.1% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 26.1% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 26.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 26.1% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 26.1% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 26.1% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 26.1% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 26.0% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 26.0% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 25.9% | `rusqlite::next` |
| 25.8% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 25.8% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 25.7% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 25.7% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 25.7% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 25.7% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 24.6% | `<rusqlite::row::Rows as fallible_streaming_iterator::FallibleStreamingIterator>::advance (.10623)` |
| 22.9% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 21.5% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broa` |
| 21.5% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<(), campfire::controllers::messages::broa` |
| 21.5% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages:` |
| 21.5% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages` |
| 21.4% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::message` |
| 21.4% | `catch_unwind<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harne` |
| 21.4% | `call_once<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtim` |
| 21.4% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>,` |
| 21.4% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 21.4% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 21.4% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(),` |
| 21.4% | `{closure}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 21.4% | `with<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 21.1% | `rusqlite::get_expected_row` |
| 20.7% | `campfire_db::{closure}` |
| 19.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>>` |
