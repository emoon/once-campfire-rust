# campfire fpbfd: messages_page (perf, dwarf)

31810000 samples at 1 µs (31.81 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **22.6%** |
| **askama render / view helpers** | (all) | **21.8%** |
| askama render / view helpers | memcpy/memmove/memset | 6.4% |
| askama render / view helpers | malloc/free/realloc | 2.7% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 1.3% |
| **app (controllers/channels/jobs)** | (all) | **19.1%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 5.5% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 2.5% |
| **kit (request/response plumbing)** | (all) | **18.2%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 5.0% |
| kit (request/response plumbing) | malloc/free/realloc | 1.2% |
| **gzip (miniz_oxide/crc32)** | (all) | **5.7%** |
| **db models / queries** | (all) | **3.8%** |
| **hyper / http** | (all) | **3.3%** |
| **tokio runtime / scheduling** | (all) | **2.9%** |
| **crypto / signing (rails_compat)** | (all) | **1.7%** |
| **json (serde_json)** | (all) | **0.5%** |
| **other** | (all) | **0.4%** |

## Top self

| self | function |
|---|---|
| 10.1% | `__memcpy_avx512_unaligned_erms` |
| 4.7% | `__memcmp_evex_movbe` |
| 2.3% | `__strlen_evex512` |
| 2.1% | `libsqlite3_sys::sqlite3VdbeExec` |
| 1.6% | `libsqlite3_sys::columnName` |
| 1.5% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 1.4% | `core_arch::_mm512_clmulepi64_epi128<0>` |
| 1.3% | `core_arch::_mm512_xor_si512` |
| 1.3% | `imalloc_fastpath` |
| 1.1% | `write_item<jiff::fmt::strtime::DefaultCustom>` |
| 1.1% | `libsqlite3_sys::columnMem` |
| 1.0% | `memchr::forward` |
| 1.0% | `cache_bin_alloc_impl` |
| 0.9% | `libsqlite3_sys::sqlite3_column_count` |
| 0.9% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.9% | `_rjem_je_arena_ralloc_no_move` |
| 0.9% | `_rjem_je_arena_ralloc` |
| 0.7% | `<rusqlite::statement::Statement>::value_ref` |
| 0.7% | `core_arch::_mm512_clmulepi64_epi128<17>` |
| 0.7% | `core::eq<u8, u8>` |
| 0.6% | `core::fmt::write` |
| 0.6% | `lll_mutex_unlock_optimized` |
| 0.6% | `<alloc::string::String as core::fmt::Write>::write_str (.10295)` |
| 0.5% | `<alloc::raw_vec::RawVecInner>::finish_grow` |
| 0.5% | `std::unlock` |
| 0.5% | `sz_index2size_lookup_impl` |
| 0.5% | `clone<alloc::string::String, alloc::alloc::Global>` |
| 0.5% | `_rjem_realloc` |
| 0.5% | `with_ranker<&memchr::arch::all::packedpair::DefaultFrequencyRank>` |
| 0.5% | `core::to_ascii_lowercase` |
| 0.5% | `libsqlite3_sys::sqlite3GetVarint` |
| 0.5% | `libsqlite3_sys::sqlite3_column_name` |
| 0.5% | `memchr::new` |
| 0.5% | `core::get_offset_len_noubcheck<u8>` |
| 0.4% | `do_rallocx` |
| 0.4% | `drop<alloc::string::String, alloc::alloc::Global>` |
| 0.4% | `core::lt` |
| 0.4% | `memchr::find_large_imp` |
| 0.4% | `memchr::cmp` |
| 0.4% | `format_one<jiff::fmt::strtime::DefaultCustom>` |

## Top inclusive

| incl | function |
|---|---|
| 57.7% | `campfire::{closure}` |
| 44.3% | `tokio::{closure}` |
| 44.1% | `tokio::poll` |
| 44.0% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 44.0% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 44.0% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 44.0% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 44.0% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 44.0% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 44.0% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 44.0% | `start_thread` |
| 44.0% | `tokio::run` |
| 44.0% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 44.0% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 44.0% | `__GI___clone3` |
| 43.8% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 39.3% | `campfire::{async_fn#0}` |
| 36.4% | `campfire_kit::adapter::dispatch::<campfire::app::dispatch_with_fragment_cache>::{closure}` |
| 25.2% | `campfire::app::dispatch_with_fragment_cache::{closure}` |
| 25.2% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>> + core::mark` |
| 25.1% | `poll<campfire::controllers::dispatch::{async_fn_env#0}>` |
| 25.1% | `with<core::task::poll::Poll<core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>>, campfire_views::fragment_cache::{impl#5}::poll:` |
| 25.0% | `{closure}<campfire::controllers::dispatch::{async_fn_env#0}>` |
| 24.8% | `<axum::routing::route::RouteFuture<core::convert::Infallible> as core::future::future::Future>::poll` |
| 24.8% | `poll<tower::util::boxed_clone_sync::BoxCloneSyncService<http::request::Request<axum_core::body::Body>, http::response::Response<axum_core::body::Body>, core::co` |
| 24.7% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 24.6% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 24.3% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::alloc::Global>>` |
| 24.3% | `poll<core::panic::unwind_safe::AssertUnwindSafe<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit::res` |
| 24.3% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<futures_util::future::future::catch_unwind::{impl#1}::poll::{closure_env#0}<core::panic::unwind_safe::As` |
| 24.3% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<futures_util::future::future::catch_unwind::{impl#1}::poll::{closure_env#0}<core::panic::unwind_safe::AssertU` |
| 24.3% | `catch_unwind<core::task::poll::Poll<core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>>, core::panic::unwind_safe::AssertUnwindS` |
| 24.3% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error` |
| 24.3% | `call_once<core::task::poll::Poll<core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>>, futures_util::future::future::catch_unwind` |
| 24.3% | `{closure}<core::panic::unwind_safe::AssertUnwindSafe<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit` |
| 24.0% | `core::fmt::write` |
| 23.0% | `campfire::controllers::messages::index::{closure}` |
| 22.9% | `<campfire_kit::adapter::ActionHandler<campfire::app::dispatch_with_fragment_cache> as axum::handler::Handler<campfire_kit::adapter::ActionMarker, campfire_kit::` |
| 22.6% | `<axum::util::MapIntoResponseFuture<axum::handler::future::IntoServiceFuture<core::pin::Pin<alloc::boxed::Box<dyn core::future::future::Future<Output = http::res` |
| 22.6% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 22.6% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 20.8% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<alloc::vec::Vec<campfire_db::models::mess` |
| 20.8% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_db::models::mess` |
| 20.8% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_db::models:` |
| 20.8% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_db::models` |
| 20.8% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_db::model` |
| 20.8% | `call_once<core::task::poll::Poll<core::result::Result<alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, campfire_db::error::Error>>,` |
| 20.8% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 20.8% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 20.8% | `catch_unwind<core::task::poll::Poll<core::result::Result<alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, campfire_db::error::Error` |
| 20.7% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, campfire` |
| 20.7% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<all` |
| 20.7% | `{closure}<alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, campfire::controllers::messages::find_paged_messages::{async_fn#0}::{clo` |
| 20.7% | `with<alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, campfire::controllers::messages::find_paged_messages::{async_fn#0}::{closure_` |
| 20.4% | `<core::fmt::rt::Argument>::fmt` |
| 20.2% | `<campfire_db::models::message::Message>::page_before` |
| 20.2% | `campfire_db::sql::query_all::<campfire_db::models::message::Message, &[&dyn rusqlite::types::to_sql::ToSql], <campfire_db::models::message::Message>::from_row>` |
| 20.0% | `from_iter<campfire_db::models::message::Message, rusqlite::error::Error, alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, rusqlite:` |
| 20.0% | `collect<core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusq` |
| 20.0% | `core::iter::adapters::try_process::<rusqlite::row::MappedRows<<campfire_db::models::message::Message>::from_row>, campfire_db::models::message::Message, core::r` |
| 20.0% | `{closure}<campfire_db::models::message::Message, rusqlite::error::Error, alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, rusqlite:` |
| 20.0% | `collect<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusqlite::error::Error>>, core::result:` |
| 20.0% | `from_iter<campfire_db::models::message::Message, core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<ca` |
| 19.5% | `spec_extend<campfire_db::models::message::Message, core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<` |
| 19.5% | `extend_desugared<campfire_db::models::message::Message, alloc::alloc::Global, core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Ro` |
| 19.1% | `<core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<<campfire_db::models::message::Message>::from_row>, core::result::Result<core::convert::Infallible` |
| 19.1% | `try_fold<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusqlite::error::Error>>, core::result` |
| 19.1% | `try_for_each<core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message,` |
| 19.1% | `try_fold<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusqlite::error::Error>>, (), core::it` |
| 19.0% | `next<campfire_db::models::message::Message, fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusqlite::error::Error>>` |
| 18.2% | `<axum::middleware::from_fn::Next>::run::{closure}` |
| 18.2% | `<axum::util::MapIntoResponseFuture<axum::routing::route::RouteFuture<core::convert::Infallible>> as core::future::future::Future>::poll` |
| 16.5% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_views::mess` |
| 16.5% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_views::messages:` |
| 16.5% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<alloc::vec::Vec<campfire_views::messages:` |
| 16.4% | `call_once<core::task::poll::Poll<core::result::Result<alloc::vec::Vec<campfire_views::messages::MessageItem, alloc::alloc::Global>, campfire_db::error::Error>>,` |
| 16.4% | `catch_unwind<core::task::poll::Poll<core::result::Result<alloc::vec::Vec<campfire_views::messages::MessageItem, alloc::alloc::Global>, campfire_db::error::Error` |
| 16.4% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_views::mes` |
| 16.4% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 16.4% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<alloc::vec::Vec<campfire_views::me` |
