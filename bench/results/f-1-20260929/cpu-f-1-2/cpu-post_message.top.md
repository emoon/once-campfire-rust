# campfire f-1: post_message

16145 samples at 1000 µs (16.14 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **44.4%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 16.4% |
| sqlite (C) | malloc/free/realloc | 1.4% |
| sqlite (C) | memcpy/memmove/memset | 0.9% |
| **tokio runtime / scheduling** | (all) | **11.0%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 8.1% |
| **gzip (miniz_oxide/crc32)** | (all) | **10.3%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 1.3% |
| **app (controllers/channels/jobs)** | (all) | **8.6%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 2.0% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 1.2% |
| **kit (request/response plumbing)** | (all) | **6.3%** |
| kit (request/response plumbing) | syscalls (read/write/epoll/futex) | 1.1% |
| kit (request/response plumbing) | memcpy/memmove/memset | 0.6% |
| kit (request/response plumbing) | malloc/free/realloc | 0.5% |
| **db models / queries** | (all) | **5.6%** |
| db models / queries | syscalls (read/write/epoll/futex) | 3.3% |
| **rich text (Action Text pipeline)** | (all) | **3.9%** |
| **cable** | (all) | **3.2%** |
| **askama render / view helpers** | (all) | **1.7%** |
| **crypto / signing (rails_compat)** | (all) | **1.7%** |
| **hyper / http** | (all) | **1.6%** |
| **json (serde_json)** | (all) | **1.6%** |
| **other** | (all) | **0.0%** |

## Top self

| self | function |
|---|---|
| 14.9% | `__syscall_cancel_arch` |
| 8.7% | `syscall` |
| 5.4% | `munmap` |
| 3.9% | `__memcpy_avx512_unaligned_erms` |
| 2.4% | `__GI___mmap64` |
| 1.8% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.4% | `__GI___lll_lock_wake` |
| 1.3% | `libsqlite3_sys::btreeInitPage` |
| 1.3% | `cache_bin_alloc_impl` |
| 1.3% | `next_code_point<core::slice::iter::Iter<u8>>` |
| 1.2% | `__memset_avx512_unaligned_erms` |
| 1.2% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 0.9% | `futex_wait` |
| 0.9% | `zlib_rs::longest_match_help<false>` |
| 0.9% | `__fcntl64_nocancel_adjusted` |
| 0.9% | `libsqlite3_sys::walFindFrame.constprop.0` |
| 0.9% | `libsqlite3_sys::walChecksumBytes` |
| 0.8% | `serde_json::ser::format_escaped_str_contents::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>` |
| 0.7% | `lll_mutex_unlock_optimized` |
| 0.7% | `__memcmp_evex_movbe` |
| 0.6% | `core::wrapping_sub` |
| 0.6% | `imalloc_fastpath` |
| 0.5% | `__strlen_evex512` |
| 0.5% | `zlib_rs::is_match<8>` |
| 0.5% | `_int_malloc` |
| 0.5% | `zlib_rs::send_bits` |
| 0.4% | `_rjem_je_arena_ralloc_no_move` |
| 0.4% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 0.4% | `zlib_rs::insert_string` |
| 0.4% | `sz_index2size_lookup_impl` |
| 0.4% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 0.4% | `core::next<u8>` |
| 0.4% | `libsqlite3_sys::pcache1TruncateUnsafe` |
| 0.4% | `next_match<core::str::pattern::MultiCharEqSearcher<[char, 3]>>` |
| 0.4% | `lll_mutex_lock_optimized` |
| 0.3% | `alloc::push` |
| 0.3% | `zlib_rs::braid_core<5>` |
| 0.3% | `_rjem_sdallocx` |
| 0.3% | `zlib_rs::quick_insert_value` |
| 0.3% | `__pthread_mutex_lock` |

## Top inclusive

| incl | function |
|---|---|
| 99.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.9% | `start_thread` |
| 99.9% | `__GI___clone3` |
| 80.6% | `tokio::{closure}` |
| 80.6% | `tokio::run` |
| 80.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 80.6% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 80.6% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 80.6% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 80.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 80.6% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 80.6% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 76.5% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 76.5% | `tokio::poll` |
| 46.0% | `campfire::{closure}` |
| 37.9% | `rusqlite::step` |
| 37.9% | `libsqlite3_sys::sqlite3_step` |
| 37.8% | `libsqlite3_sys::sqlite3Step` |
| 37.7% | `libsqlite3_sys::sqlite3VdbeExec` |
| 31.7% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 31.7% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 31.7% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 31.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 31.7% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 31.7% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 31.7% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 31.7% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 31.7% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 31.7% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 31.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 31.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 31.7% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 31.7% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 31.7% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 31.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 31.7% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 29.8% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 29.1% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 29.1% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 29.0% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 28.0% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 28.0% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 27.9% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 27.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 27.9% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 27.9% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 27.9% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 27.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 27.9% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 27.9% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 27.9% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 27.8% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 27.8% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 27.8% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 27.6% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 27.5% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 27.5% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 27.5% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 27.5% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 27.5% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 25.0% | `rusqlite::next` |
| 24.7% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 23.8% | `<rusqlite::row::Rows as fallible_streaming_iterator::FallibleStreamingIterator>::advance (.10632)` |
| 21.7% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broa` |
| 21.7% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<(), campfire::controllers::messages::broa` |
| 21.7% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages:` |
| 21.6% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages` |
| 21.6% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::message` |
| 21.6% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 21.6% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 21.6% | `catch_unwind<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harne` |
| 21.6% | `call_once<core::task::poll::Poll<core::result::Result<(), campfire_db::error::Error>>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtim` |
| 21.6% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(),` |
| 21.6% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>,` |
| 21.6% | `{closure}<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 21.6% | `with<(), campfire::controllers::messages::broadcast_create::{async_fn#0}::{closure_env#0}>` |
| 20.2% | `rusqlite::get_expected_row` |
| 19.4% | `campfire_db::{closure}` |
| 17.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::open::{c` |
