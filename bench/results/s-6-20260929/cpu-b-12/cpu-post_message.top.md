# campfire b: post_message

15695 samples at 1000 µs (15.70 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **46.9%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 18.1% |
| sqlite (C) | malloc/free/realloc | 1.4% |
| sqlite (C) | memcpy/memmove/memset | 1.0% |
| **app (controllers/channels/jobs)** | (all) | **10.1%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 2.2% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 2.0% |
| **tokio runtime / scheduling** | (all) | **9.9%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 7.3% |
| **gzip (miniz_oxide/crc32)** | (all) | **8.7%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.5% |
| **kit (request/response plumbing)** | (all) | **6.3%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 0.9% |
| kit (request/response plumbing) | malloc/free/realloc | 0.8% |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.6% |
| **db models / queries** | (all) | **5.2%** |
| db models / queries | syscalls (read/write/epoll/futex) | 3.0% |
| **rich text (Action Text pipeline)** | (all) | **4.1%** |
| **cable** | (all) | **2.5%** |
| **askama render / view helpers** | (all) | **2.3%** |
| askama render / view helpers | memcpy/memmove/memset | 0.6% |
| **hyper / http** | (all) | **1.5%** |
| **json (serde_json)** | (all) | **1.4%** |
| **crypto / signing (rails_compat)** | (all) | **1.1%** |
| **other** | (all) | **0.0%** |

## Top self

| self | function |
|---|---|
| 17.0% | `__syscall_cancel_arch` |
| 7.5% | `syscall` |
| 6.3% | `munmap` |
| 4.9% | `__memcpy_avx512_unaligned_erms` |
| 2.5% | `__GI___mmap64` |
| 1.6% | `cache_bin_alloc_impl` |
| 1.6% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.4% | `__memset_avx512_unaligned_erms` |
| 1.4% | `libsqlite3_sys::btreeInitPage` |
| 1.0% | `__fcntl64_nocancel_adjusted` |
| 1.0% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.0% | `__GI___lll_lock_wake` |
| 0.9% | `next_code_point<core::slice::iter::Iter<u8>>` |
| 0.9% | `libsqlite3_sys::walFindFrame.constprop.0` |
| 0.8% | `libsqlite3_sys::walChecksumBytes` |
| 0.8% | `zlib_rs::longest_match_help<false>` |
| 0.7% | `futex_wait` |
| 0.7% | `serde_json::ser::format_escaped_str_contents::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.7% | `lll_mutex_unlock_optimized` |
| 0.5% | `_rjem_je_arena_ralloc_no_move` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `imalloc_fastpath` |
| 0.5% | `__strlen_evex512` |
| 0.5% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.4% | `zlib_rs::is_match<8>` |
| 0.4% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 0.4% | `core::next<u8>` |
| 0.4% | `core::wrapping_sub` |
| 0.4% | `_int_malloc` |
| 0.4% | `lll_mutex_lock_optimized` |
| 0.4% | `__pthread_mutex_lock` |
| 0.3% | `libsqlite3_sys::pcache1TruncateUnsafe` |
| 0.3% | `zlib_rs::insert_string` |
| 0.3% | `core::copy_nonoverlapping<u8>` |
| 0.3% | `next_match<core::str::pattern::MultiCharEqSearcher<[char, 3]>>` |
| 0.3% | `free_fastpath` |
| 0.3% | `core::eq<u8>` |
| 0.3% | `zlib_rs::send_bits` |
| 0.3% | `current_memory<alloc::alloc::Global>` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `start_thread` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `__GI___clone3` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 79.0% | `tokio::{closure}` |
| 78.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 78.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 78.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 78.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 78.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 78.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 78.9% | `tokio::run` |
| 78.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 75.2% | `tokio::poll` |
| 75.2% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 47.4% | `campfire::{closure}` |
| 40.5% | `rusqlite::step` |
| 40.5% | `libsqlite3_sys::sqlite3_step` |
| 40.5% | `libsqlite3_sys::sqlite3Step` |
| 40.3% | `libsqlite3_sys::sqlite3VdbeExec` |
| 28.6% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 28.6% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 28.6% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 28.6% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 28.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 28.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 28.6% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 28.6% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 28.6% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 28.6% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 28.6% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 28.6% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 28.6% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 28.6% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 28.6% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 28.6% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 28.6% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 27.3% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 26.8% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 26.8% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 26.7% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 26.0% | `rusqlite::next` |
| 25.8% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 25.7% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 25.7% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 25.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 25.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 25.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 25.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 25.7% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 25.7% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 25.7% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 25.7% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 25.6% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 25.6% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 25.6% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 25.4% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 25.4% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 25.4% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 25.4% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 25.4% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 25.3% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 24.6% | `<rusqlite::row::Rows as fallible_streaming_iterator::FallibleStreamingIterator>::advance (.10623)` |
| 22.5% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 22.1% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<(), campfire::controllers::messages::broa` |
| 22.1% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broa` |
| 22.1% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages:` |
| 22.1% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages` |
| 22.1% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::message` |
| 22.1% | `call_once<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtim` |
| 22.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 22.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 22.1% | `catch_unwind<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harne` |
| 22.1% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(),` |
| 22.0% | `{closure}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 22.0% | `with<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 22.0% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>,` |
| 21.3% | `rusqlite::get_expected_row` |
| 21.0% | `campfire_db::{closure}` |
| 19.2% | `{closure}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>` |
