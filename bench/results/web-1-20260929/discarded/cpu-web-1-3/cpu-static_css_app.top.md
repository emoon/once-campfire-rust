# campfire web-1: static_css_app

27278 samples at 1000 µs (27.28 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **gzip (miniz_oxide/crc32)** | (all) | **61.7%** |
| gzip (miniz_oxide/crc32) | memcpy/memmove/memset | 17.0% |
| gzip (miniz_oxide/crc32) | malloc/free/realloc | 0.6% |
| **tokio runtime / scheduling** | (all) | **17.9%** |
| tokio runtime / scheduling | syscalls (read/write/epoll/futex) | 13.5% |
| **kit (request/response plumbing)** | (all) | **8.2%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 1.5% |
| kit (request/response plumbing) | malloc/free/realloc | 1.4% |
| **app (controllers/channels/jobs)** | (all) | **6.3%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 1.5% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 0.9% |
| **hyper / http** | (all) | **5.7%** |
| **other** | (all) | **0.2%** |

## Top self

| self | function |
|---|---|
| 14.3% | `__memset_avx512_unaligned_erms` |
| 13.2% | `__syscall_cancel_arch` |
| 9.6% | `<zlib_rs::deflate::Heap>::pqdownheap` |
| 4.4% | `__memcpy_avx512_unaligned_erms` |
| 3.0% | `zlib_rs::quick_insert_value` |
| 3.0% | `zlib_rs::send_bits` |
| 2.9% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 1.6% | `zlib_rs::longest_match_help<false>` |
| 1.6% | `core::split_at_checked<u8>` |
| 1.2% | `zlib_rs::insert_string` |
| 1.1% | `<zlib_rs::deflate::BitWriter>::send_tree` |
| 1.1% | `core::index<u8>` |
| 1.0% | `zlib_rs::insert_match` |
| 1.0% | `__memcmp_evex_movbe` |
| 1.0% | `zlib_rs::as_slice<u16, 65536>` |
| 1.0% | `zlib_rs::gen_bitlen<573>` |
| 0.8% | `core::next<u8>` |
| 0.8% | `zlib_rs::{closure}` |
| 0.8% | `zlib_rs::braid_core<5>` |
| 0.8% | `zlib_rs::deflate::scan_tree` |
| 0.7% | `zlib_rs::deflate::gen_codes` |
| 0.7% | `imalloc_fastpath` |
| 0.6% | `<zlib_rs::deflate::State>::init_block` |
| 0.6% | `zlib_rs::push_lit` |
| 0.6% | `<zlib_rs::deflate::Heap>::construct_huffman_tree` |
| 0.6% | `zlib_rs::tally_lit_help` |
| 0.5% | `0x7fac3dab6d4c` |
| 0.5% | `zlib_rs::hash_calc` |
| 0.5% | `core::reverse_bits` |
| 0.5% | `zlib_rs::initialize` |
| 0.5% | `core::swap_chunk<8>` |
| 0.4% | `cache_bin_alloc_impl` |
| 0.4% | `do_register<&core::task::wake::Waker>` |
| 0.4% | `core::is_empty<usize>` |
| 0.3% | `parking_lot::lock` |
| 0.3% | `zlib_rs::deflate::fill_window` |
| 0.3% | `parking_lot::unlock` |
| 0.3% | `<zlib_rs::deflate::BitWriter>::send_bits_overflow` |
| 0.3% | `zlib_rs::deflate::longest_match::longest_match` |
| 0.3% | `sz_index2size_lookup_impl` |

## Top inclusive

| incl | function |
|---|---|
| 99.8% | `start_thread` |
| 99.8% | `__GI___clone3` |
| 99.8% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 99.8% | `tokio::run` |
| 99.8% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 99.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 99.8% | `tokio::{closure}` |
| 99.8% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 99.8% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 99.8% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 99.8% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 99.8% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 99.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 99.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 99.8% | `poll<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blocking:` |
| 99.8% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure}>, to` |
| 99.8% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 99.8% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::blo` |
| 99.8% | `tokio::poll` |
| 99.8% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::` |
| 99.8% | `with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closur` |
| 99.8% | `{closure}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bloc` |
| 99.8% | `poll<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}, ()>` |
| 99.8% | `set<tokio::runtime::scheduler::Context, tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}, ()>` |
| 99.8% | `{closure}<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 99.8% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::s` |
| 99.8% | `try_with<tokio::runtime::context::Context, tokio::runtime::context::set_scheduler::{closure_env#0}<(), tokio::runtime::scheduler::multi_thread::worker::run::{cl` |
| 99.8% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{clos` |
| 99.8% | `set_scheduler<(), tokio::runtime::scheduler::multi_thread::worker::run::{closure}::{closure_env#0}>` |
| 99.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<toki` |
| 99.8% | `poll_future<tokio::runtime::blocking::task::BlockingTask<tokio::runtime::scheduler::multi_thread::worker::{impl#0}::launch::{closure_env#0}>, tokio::runtime::bl` |
| 99.8% | `enter_runtime<tokio::runtime::scheduler::multi_thread::worker::run::{closure_env#0}, ()>` |
| 98.2% | `<tokio::runtime::scheduler::multi_thread::worker::Context>::run_task` |
| 98.1% | `with_budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler:` |
| 98.1% | `budget<core::result::Result<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core, alloc::alloc::Global>, ()>, tokio::runtime::scheduler::mult` |
| 98.1% | `run<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>` |
| 98.1% | `tokio::runtime::task::raw::poll::<campfire_kit::front::conn::accept_loop<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serv` |
| 98.0% | `poll<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_kit::fr` |
| 97.9% | `poll_inner<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_k` |
| 97.8% | `call_once<core::task::poll::Poll<()>, tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_b` |
| 97.8% | `poll_future<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_` |
| 97.8% | `catch_unwind<core::task::poll::Poll<()>, core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::fr` |
| 97.8% | `{closure}<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn#0}::{closure_env#0}, campfire_ki` |
| 97.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{async_fn` |
| 97.8% | `with_mut<tokio::runtime::task::core::Stage<campfire_kit::front::conn::accept_loop::{async_fn#0}::{async_block_env#0}<campfire_kit::front::serve_plain::{async_fn` |
| 97.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<campfire_kit::front::conn::accept_loop::{asy` |
| 97.8% | `campfire_kit::front::conn::accept_loop::<campfire_kit::front::serve_plain::{closure}::{closure}, campfire_kit::front::conn::serve_connection<tokio::net::tcp::st` |
| 97.7% | `campfire_kit::front::conn::serve_connection::<tokio::net::tcp::stream::TcpStream>::{closure}` |
| 97.7% | `{closure}<tokio::net::tcp::stream::TcpStream>` |
| 97.7% | `poll<campfire_kit::front::conn::serve_connection::{async_fn#0}::__tokio_select_util::Out<core::result::Result<(), hyper::error::Error>, (), ()>, campfire_kit::f` |
| 96.4% | `<core::pin::Pin<&mut hyper::server::conn::http1::UpgradeableConnection<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, hyper::service::util:` |
| 96.4% | `poll<hyper_util::rt::tokio::TokioIo<tokio::net::tcp::stream::TcpStream>, axum_core::body::Body, hyper::service::util::ServiceFn<campfire_kit::front::conn::serve` |
| 96.3% | `poll_catch<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 96.3% | `poll<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::net::t` |
| 96.3% | `poll_inner<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::` |
| 96.1% | `poll_loop<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection::{async_fn#0}::{closure_env#0}<tokio::n` |
| 79.2% | `<hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<hyper::service::util::ServiceFn<campfire_kit::front::conn::serve_connection<tokio::ne` |
| 51.7% | `axum_core::poll_frame` |
| 51.7% | `poll_frame<bytes::bytes::Bytes, axum_core::error::Error>` |
| 51.7% | `<http_body_util::combinators::map_err::MapErr<campfire_kit::front::conn::InFlight<campfire_kit::front::conn::Deadline<axum_core::body::Body>>, <axum_core::error` |
| 51.7% | `poll_frame<axum_core::body::Body>` |
| 51.7% | `poll_frame<campfire_kit::front::conn::Deadline<axum_core::body::Body>>` |
| 50.9% | `<http_body_util::combinators::map_err::MapErr<axum_core::body::StreamBody<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataSt` |
| 50.9% | `poll_frame<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::a` |
| 50.9% | `try_poll_next<futures_util::stream::unfold::Unfold<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc` |
| 50.9% | `poll_next<core::option::Option<(axum_core::body::BodyDataStream, flate2::gz::write::GzEncoder<alloc::vec::Vec<u8, alloc::alloc::Global>>)>, campfire_kit::deflat` |
| 50.7% | `campfire_kit::{async_block#0}` |
| 47.2% | `campfire_kit::compress` |
| 45.9% | `<flate2::mem::Compress as flate2::zio::Ops>::run_vec` |
| 45.9% | `write_to_spare_capacity_of_vec<core::result::Result<flate2::mem::Status, flate2::mem::CompressError>, flate2::mem::{impl#1}::compress_vec::{closure_env#0}>` |
| 45.9% | `flate2::compress_vec` |
| 45.9% | `compress_uninit<flate2::ffi::zlib_rs::Deflate>` |
| 45.9% | `flate2::compress_uninit` |
| 45.9% | `flate2::{closure}` |
| 43.1% | `flate2::compress` |
| 43.0% | `<zlib_rs::stable::Deflate>::compress` |
| 43.0% | `zlib_rs::compress_uninit` |
| 42.9% | `zlib_rs::deflate` |
| 42.6% | `zlib_rs::deflate::algorithm::medium::deflate_medium` |
| 29.1% | `flush<alloc::vec::Vec<u8, alloc::alloc::Global>>` |
