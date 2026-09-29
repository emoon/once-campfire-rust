# campfire view-3: post_message

52 samples at 1000 µs (0.05 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **44.2%** |
| sqlite (C) | syscalls (read/write/epoll/futex) | 21.2% |
| sqlite (C) | memcpy/memmove/memset | 1.9% |
| sqlite (C) | malloc/free/realloc | 1.9% |
| **kit (request/response plumbing)** | (all) | **11.5%** |
| kit (request/response plumbing) | malloc/free/realloc | 3.8% |
| **gzip (miniz_oxide/crc32)** | (all) | **11.5%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 3.8% |
| **db models / queries** | (all) | **7.7%** |
| db models / queries | syscalls (read/write/epoll/futex) | 3.8% |
| **tokio runtime / scheduling** | (all) | **5.8%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 1.9% |
| **crypto / signing (rails_compat)** | (all) | **5.8%** |
| crypto / signing (rails_compat) | memcpy/memmove/memset | 1.9% |
| **app (controllers/channels/jobs)** | (all) | **5.8%** |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 3.8% |
| **rich text (Action Text pipeline)** | (all) | **3.8%** |
| **hyper / http** | (all) | **1.9%** |
| **askama render / view helpers** | (all) | **1.9%** |

## Top self

| self | function |
|---|---|
| 21.2% | `__syscall_cancel_arch` |
| 5.8% | `libsqlite3_sys::walIndexAppend` |
| 5.8% | `syscall` |
| 3.8% | `__memcmp_evex_movbe` |
| 3.8% | `__memset_avx512_unaligned_erms` |
| 3.8% | `__memcpy_avx512_unaligned_erms` |
| 1.9% | `libsqlite3_sys::sqlite3_column_name` |
| 1.9% | `libsqlite3_sys::fts5DataWrite` |
| 1.9% | `<campfire::integrations::web_push::pool::Pool>::deliver_later::{closure}` |
| 1.9% | `core::from_str` |
| 1.9% | `remaining<bytes::bytes::Bytes>` |
| 1.9% | `alloc::new` |
| 1.9% | `<u8 as core::slice::cmp::SlicePartialEq<u8>>::equal_same_length` |
| 1.9% | `core::ptr::drop_glue::<<campfire_kit::front::compression::Compression>::apply::{closure}>` |
| 1.9% | `core::copy_nonoverlapping<u8>` |
| 1.9% | `__fcntl64_nocancel_adjusted` |
| 1.9% | `campfire_kit::front::serve_with::<campfire::app::serve::{closure}::{closure}>::{closure}::{closure}::{closure}` |
| 1.9% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<bool, campfire::concerns::reject_banned_i` |
| 1.9% | `core::next<u8>` |
| 1.9% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 1.9% | `munmap` |
| 1.9% | `libsqlite3_sys::releaseMemArray.part.0` |
| 1.9% | `drop<str, alloc::alloc::Global>` |
| 1.9% | `parse_fragment<campfire_richtext::dom::Sink>` |
| 1.9% | `current_memory<alloc::alloc::Global>` |
| 1.9% | `rails_compat::signed_id::verifier` |
| 1.9% | `<campfire_db::models::message::Message>::plain_text_body` |
| 1.9% | `_int_malloc` |
| 1.9% | `core::to_ascii_lowercase` |
| 1.9% | `core::get_end<char>` |
| 1.9% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 1.9% | `zlib_rs::longest_match_help<false>` |
| 1.9% | `core_arch::_mm_shuffle_epi32<14>` |
| 1.9% | `unsafe_subtendril<tendril::fmt::UTF8, tendril::tendril::NonAtomic>` |
| 1.9% | `lll_mutex_unlock_optimized` |

## Top inclusive

| incl | function |
|---|---|
| 100.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 100.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 100.0% | `start_thread` |
| 100.0% | `__GI___clone3` |
| 71.2% | `tokio::poll` |
| 71.2% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 71.2% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 71.2% | `tokio::{closure}` |
| 71.2% | `tokio::run` |
| 71.2% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 71.2% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 71.2% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 71.2% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 71.2% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 71.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 38.5% | `libsqlite3_sys::sqlite3VdbeExec` |
| 38.5% | `rusqlite::step` |
| 38.5% | `libsqlite3_sys::sqlite3Step` |
| 38.5% | `libsqlite3_sys::sqlite3_step` |
| 36.5% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 36.5% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 36.5% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 36.5% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 36.5% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 36.5% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 36.5% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 36.5% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 36.5% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 36.5% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 36.5% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 36.5% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 36.5% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 36.5% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 36.5% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 36.5% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 36.5% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 36.5% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 36.5% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 36.5% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 36.5% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 32.7% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 32.7% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 32.7% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 32.7% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 32.7% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 32.7% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 32.7% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 32.7% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 32.7% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 32.7% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 32.7% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 32.7% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 32.7% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 32.7% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 32.7% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 32.7% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 32.7% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 32.7% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 32.7% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 32.7% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 28.8% | `campfire::{closure}` |
| 28.8% | `{closure}<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::messages::create_message::{async_f` |
| 28.8% | `campfire_db::{closure}` |
| 28.8% | `{closure}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>` |
| 28.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>, ()>` |
| 28.8% | `<<campfire_db::database::Database>::write<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::me` |
| 28.8% | `<std::thread::lifecycle::spawn_unchecked<<campfire_db::database::Database>::open::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>::call_once::{shi` |
| 28.8% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>>` |
| 28.8% | `std::sys::backtrace::__rust_begin_short_backtrace::<<campfire_db::database::Database>::open::{closure}, ()>` |
| 28.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>, ()>` |
| 28.8% | `call_once<(), campfire_db::database::{impl#4}::open::{closure}::{closure_env#0}>` |
| 28.8% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}:` |
| 28.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::ope` |
| 28.8% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::open::{closure_env#0}, ()>>` |
| 28.8% | `call_once<(&rusqlite::Connection, &campfire_db::database::Env), (dyn core::ops::function::FnOnce<(&rusqlite::Connection, &campfire_db::database::Env), Output=()` |
| 28.8% | `run_write<(campfire_db::models::message::Message, core::option::Option<campfire_storage::blob::Blob>), campfire::controllers::messages::create_message::{async_f` |
| 28.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<campfire_db::database::{impl#4}::open::{c` |
| 28.8% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 26.9% | `libsqlite3_sys::sqlite3VdbeHalt` |
| 25.0% | `libsqlite3_sys::vdbeCommit` |
