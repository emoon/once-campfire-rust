# campfire b: post_message

14739 samples at 1000 µs (14.74 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **45.8%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 16.2% |
| sqlite (C) | malloc/free/realloc | 1.5% |
| sqlite (C) | memcpy/memmove/memset | 1.1% |
| **tokio runtime / scheduling** | (all) | **10.5%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 7.7% |
| **gzip (miniz_oxide/crc32)** | (all) | **10.0%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.4% |
| **app (controllers/channels/jobs)** | (all) | **8.3%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 1.8% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.4% |
| **kit (request/response plumbing)** | (all) | **6.6%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.0% |
| kit (request/response plumbing) | malloc/free/realloc | 0.6% |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.5% |
| **db models / queries** | (all) | **5.4%** |
| db models / queries | syscalls (read/write/epoll/futex) | 3.0% |
| **rich text (Action Text pipeline)** | (all) | **4.0%** |
| **cable** | (all) | **3.1%** |
| **askama render / view helpers** | (all) | **2.1%** |
| **hyper / http** | (all) | **1.6%** |
| **json (serde_json)** | (all) | **1.4%** |
| **crypto / signing (rails_compat)** | (all) | **1.2%** |
| **other** | (all) | **0.1%** |

## Top self

| self | function |
|---|---|
| 14.4% | `__syscall_cancel_arch` |
| 8.1% | `syscall` |
| 5.8% | `munmap` |
| 3.9% | `__memcpy_avx512_unaligned_erms` |
| 2.3% | `__GI___mmap64` |
| 1.7% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.5% | `__GI___lll_lock_wake` |
| 1.4% | `__memset_avx512_unaligned_erms` |
| 1.3% | `libsqlite3_sys::btreeInitPage` |
| 1.3% | `cache_bin_alloc_impl` |
| 1.1% | `libsqlite3_sys::walFindFrame.constprop.0` |
| 1.0% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 1.0% | `futex_wait` |
| 1.0% | `next_code_point<core::slice::iter::Iter<u8>>` |
| 0.9% | `zlib_rs::longest_match_help<false>` |
| 0.9% | `__fcntl64_nocancel_adjusted` |
| 0.8% | `lll_mutex_unlock_optimized` |
| 0.8% | `libsqlite3_sys::walChecksumBytes` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.7% | `serde_json::ser::format_escaped_str_contents::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>` |
| 0.6% | `core::wrapping_sub` |
| 0.5% | `imalloc_fastpath` |
| 0.5% | `_int_malloc` |
| 0.5% | `zlib_rs::send_bits` |
| 0.5% | `zlib_rs::is_match<8>` |
| 0.4% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 0.4% | `__strlen_evex512` |
| 0.4% | `libsqlite3_sys::pcache1TruncateUnsafe` |
| 0.4% | `core::next<u8>` |
| 0.4% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.4% | `__pthread_mutex_lock` |
| 0.4% | `next_match<core::str::pattern::MultiCharEqSearcher<[char, 3]>>` |
| 0.4% | `libsqlite3_sys::columnName` |
| 0.4% | `_rjem_je_arena_ralloc_no_move` |
| 0.4% | `zlib_rs::insert_string` |
| 0.4% | `sz_index2size_lookup_impl` |
| 0.4% | `parking_lot::lock` |
| 0.3% | `_rjem_sdallocx` |
| 0.3% | `campfire_cable::json::escape_html_entities` |
| 0.3% | `lll_mutex_lock_optimized` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `start_thread` |
| 99.9% | `__GI___clone3` |
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 80.1% | `tokio::{closure}` |
| 80.0% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 80.0% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 80.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 80.0% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 80.0% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 80.0% | `tokio::run` |
| 80.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 80.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 76.1% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 76.1% | `tokio::poll` |
| 46.8% | `campfire::{closure}` |
| 38.6% | `rusqlite::step` |
| 38.5% | `libsqlite3_sys::sqlite3_step` |
| 38.5% | `libsqlite3_sys::sqlite3Step` |
| 38.4% | `libsqlite3_sys::sqlite3VdbeExec` |
| 30.8% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 30.8% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 30.8% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 30.8% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 30.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 30.8% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 30.8% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 30.8% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 30.8% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 30.8% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 30.8% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 30.8% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 30.8% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 30.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 30.8% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 30.8% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 30.8% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 29.1% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 28.3% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 28.3% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 28.3% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 27.3% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 27.3% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 27.2% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 27.2% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 27.2% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 27.2% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 27.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 27.2% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 27.2% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 27.2% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 27.2% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 27.1% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 27.1% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 27.1% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 26.8% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 26.8% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 26.8% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 26.8% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 26.8% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 26.7% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 25.5% | `rusqlite::next` |
| 24.0% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 24.0% | `<rusqlite::row::Rows as fallible_streaming_iterator::FallibleStreamingIterator>::advance (.10623)` |
| 21.6% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages:` |
| 21.6% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<(), campfire::controllers::messages::broa` |
| 21.6% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broa` |
| 21.6% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::message` |
| 21.6% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages` |
| 21.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 21.6% | `catch_unwind<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harne` |
| 21.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 21.6% | `call_once<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtim` |
| 21.6% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(),` |
| 21.5% | `with<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 21.5% | `{closure}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 21.5% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>,` |
| 20.7% | `rusqlite::get_expected_row` |
| 19.9% | `campfire_db::{closure}` |
| 18.5% | `{closure}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>` |
