# campfire base: post_message

16350 samples at 1000 µs (16.35 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **45.9%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 16.2% |
| sqlite (C) | malloc/free/realloc | 1.6% |
| sqlite (C) | memcpy/memmove/memset | 0.8% |
| **tokio runtime / scheduling** | (all) | **10.3%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 7.7% |
| **gzip (miniz_oxide/crc32)** | (all) | **10.0%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.3% |
| **app (controllers/channels/jobs)** | (all) | **8.2%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 2.1% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.2% |
| **kit (request/response plumbing)** | (all) | **6.7%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.2% |
| kit (request/response plumbing) | malloc/free/realloc | 0.6% |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.6% |
| **db models / queries** | (all) | **5.5%** |
| db models / queries | syscalls (read/write/epoll/futex) | 3.2% |
| **rich text (Action Text pipeline)** | (all) | **3.6%** |
| **cable** | (all) | **3.1%** |
| **askama render / view helpers** | (all) | **2.0%** |
| askama render / view helpers | memcpy/memmove/memset | 0.5% |
| **hyper / http** | (all) | **1.8%** |
| **json (serde_json)** | (all) | **1.5%** |
| **crypto / signing (rails_compat)** | (all) | **1.3%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 14.6% | `__syscall_cancel_arch` |
| 8.4% | `syscall` |
| 6.3% | `munmap` |
| 3.9% | `__memcpy_avx512_unaligned_erms` |
| 2.6% | `__GI___mmap64` |
| 1.6% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.4% | `libsqlite3_sys::btreeInitPage` |
| 1.3% | `next_code_point<core::slice::iter::Iter<u8>>` |
| 1.2% | `__GI___lll_lock_wake` |
| 1.2% | `__memset_avx512_unaligned_erms` |
| 1.1% | `__fcntl64_nocancel_adjusted` |
| 1.0% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.0% | `futex_wait` |
| 1.0% | `libsqlite3_sys::walFindFrame.constprop.0` |
| 0.9% | `zlib_rs::longest_match_help<false>` |
| 0.9% | `libsqlite3_sys::walChecksumBytes` |
| 0.8% | `lll_mutex_unlock_optimized` |
| 0.8% | `cache_bin_alloc_impl` |
| 0.8% | `serde_json::ser::format_escaped_str_contents::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.6% | `core::wrapping_sub` |
| 0.6% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 0.5% | `imalloc_fastpath` |
| 0.5% | `zlib_rs::is_match<8>` |
| 0.5% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.5% | `next_match<core::str::pattern::MultiCharEqSearcher<[char, 3]>>` |
| 0.5% | `_int_malloc` |
| 0.5% | `zlib_rs::send_bits` |
| 0.4% | `zlib_rs::insert_string` |
| 0.4% | `core::next<u8>` |
| 0.4% | `lll_mutex_lock_optimized` |
| 0.4% | `_rjem_je_arena_ralloc_no_move` |
| 0.4% | `libsqlite3_sys::pcache1TruncateUnsafe` |
| 0.4% | `__strlen_evex512` |
| 0.4% | `__pthread_mutex_lock` |
| 0.3% | `sz_index2size_lookup_impl` |
| 0.3% | `_rjem_sdallocx` |
| 0.3% | `libsqlite3_sys::columnName` |
| 0.3% | `free_fastpath` |
| 0.3% | `0x7fb0d492cd4c` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `start_thread` |
| 99.9% | `__GI___clone3` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 80.6% | `tokio::{closure}` |
| 80.6% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 80.6% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 80.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 80.6% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 80.6% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 80.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 80.6% | `tokio::run` |
| 80.6% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 76.7% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 76.7% | `tokio::poll` |
| 46.4% | `campfire::{closure}` |
| 39.1% | `rusqlite::step` |
| 39.1% | `libsqlite3_sys::sqlite3_step` |
| 39.0% | `libsqlite3_sys::sqlite3Step` |
| 38.9% | `libsqlite3_sys::sqlite3VdbeExec` |
| 31.1% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 31.1% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 31.1% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 31.1% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 31.1% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 31.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 31.1% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 31.1% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 31.1% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 31.1% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 31.1% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 31.1% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 31.1% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 31.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 31.1% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 31.1% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 31.1% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 29.4% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 28.8% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 28.8% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 28.7% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 27.8% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 27.8% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 27.7% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 27.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 27.7% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 27.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 27.7% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 27.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 27.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 27.7% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 27.7% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 27.6% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 27.6% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 27.6% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 27.3% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 27.3% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 27.3% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 27.3% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 27.3% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 27.2% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 26.3% | `rusqlite::next` |
| 25.0% | `<rusqlite::row::Rows as fallible_streaming_iterator::FallibleStreamingIterator>::advance (.10624)` |
| 24.5% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 21.7% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages:` |
| 21.7% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<(), campfire::controllers::messages::broa` |
| 21.7% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broa` |
| 21.7% | `call_once<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtim` |
| 21.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 21.7% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages` |
| 21.7% | `catch_unwind<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harne` |
| 21.7% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(),` |
| 21.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 21.7% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>,` |
| 21.7% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::message` |
| 21.7% | `with<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 21.7% | `{closure}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 21.2% | `rusqlite::get_expected_row` |
| 19.3% | `campfire_db::{closure}` |
| 17.7% | `{closure}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>` |
