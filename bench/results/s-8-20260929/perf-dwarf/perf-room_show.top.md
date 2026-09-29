# campfire fpbfd: room_show (perf, dwarf)

32407000 samples at 1 µs (32.41 CPU-s)

## By owner (nearest recognizable frame) and leaf kind

| owner | leaf | share |
|---|---|---|
| **sqlite (C)** | (all) | **20.8%** |
| **askama render / view helpers** | (all) | **19.6%** |
| askama render / view helpers | memcpy/memmove/memset | 6.5% |
| askama render / view helpers | malloc/free/realloc | 2.0% |
| askama render / view helpers | syscalls (read/write/epoll/futex) | 0.6% |
| **app (controllers/channels/jobs)** | (all) | **19.2%** |
| app (controllers/channels/jobs) | malloc/free/realloc | 5.5% |
| app (controllers/channels/jobs) | memcpy/memmove/memset | 2.6% |
| **kit (request/response plumbing)** | (all) | **15.6%** |
| kit (request/response plumbing) | memcpy/memmove/memset | 4.2% |
| kit (request/response plumbing) | malloc/free/realloc | 1.2% |
| **crypto / signing (rails_compat)** | (all) | **10.7%** |
| **gzip (miniz_oxide/crc32)** | (all) | **5.1%** |
| **db models / queries** | (all) | **3.1%** |
| **hyper / http** | (all) | **2.8%** |
| **tokio runtime / scheduling** | (all) | **2.2%** |
| **json (serde_json)** | (all) | **0.5%** |
| **other** | (all) | **0.2%** |

## Top self

| self | function |
|---|---|
| 9.4% | `__memcpy_avx512_unaligned_erms` |
| 5.0% | `__memcmp_evex_movbe` |
| 2.7% | `core_arch::_mm_add_epi32` |
| 2.1% | `core_arch::_mm_shuffle_epi32<14>` |
| 2.0% | `libsqlite3_sys::sqlite3VdbeExec` |
| 2.0% | `__strlen_evex512` |
| 1.7% | `imalloc_fastpath` |
| 1.6% | `core_arch::_mm_alignr_epi8<4>` |
| 1.4% | `core_arch::_mm_sha256msg1_epu32` |
| 1.4% | `libsqlite3_sys::columnName` |
| 1.2% | `core_arch::_mm512_clmulepi64_epi128<0>` |
| 1.2% | `core_arch::_mm512_xor_si512` |
| 1.1% | `libsqlite3_sys::sqlite3BtreeTableMoveto` |
| 1.1% | `cache_bin_alloc_impl` |
| 1.0% | `_rjem_je_arena_ralloc_no_move` |
| 1.0% | `libsqlite3_sys::columnMem` |
| 0.9% | `memchr::forward` |
| 0.8% | `_rjem_je_arena_ralloc` |
| 0.7% | `sz_index2size_lookup_impl` |
| 0.7% | `<core::fmt::Arguments>::estimated_capacity` |
| 0.7% | `core_arch::_mm_sha256rnds2_epu32` |
| 0.7% | `lll_mutex_unlock_optimized` |
| 0.6% | `libsqlite3_sys::sqlite3_column_count` |
| 0.6% | `<rusqlite::statement::Statement>::value_ref` |
| 0.6% | `_rjem_sdallocx` |
| 0.5% | `binary_search_by<(&str, &str), campfire_assets::helpers::digested_path::{closure_env#0}>` |
| 0.5% | `core::eq<u8, u8>` |
| 0.5% | `core::fmt::write` |
| 0.5% | `free_fastpath` |
| 0.5% | `<alloc::string::String as core::fmt::Write>::write_str (.10295)` |
| 0.5% | `libsqlite3_sys::sqlite3_column_name` |
| 0.4% | `core::get_offset_len_noubcheck<u8>` |
| 0.4% | `core_arch::_mm512_clmulepi64_epi128<17>` |
| 0.4% | `libsqlite3_sys::sqlite3GetVarint` |
| 0.4% | `clone<alloc::string::String, alloc::alloc::Global>` |
| 0.4% | `core_arch::_mm_sha256msg2_epu32` |
| 0.4% | `_rjem_realloc` |
| 0.4% | `memchr::new` |
| 0.4% | `core::from_ascii_radix` |
| 0.4% | `core::to_ascii_lowercase` |

## Top inclusive

| incl | function |
|---|---|
| 55.1% | `campfire::{closure}` |
| 50.0% | `campfire::{async_fn#0}` |
| 47.4% | `campfire_kit::adapter::dispatch::<campfire::app::dispatch_with_fragment_cache>::{closure}` |
| 36.1% | `tokio::{closure}` |
| 35.9% | `tokio::poll` |
| 35.9% | `<alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output = ()> + core::marker::Send> as core::ops::function::FnOnce<()>>::call_once` |
| 35.9% | `<std::thread::lifecycle::spawn_unchecked<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>::{closure} as core::ops::function::FnOnce<()>>:` |
| 35.9% | `catch_unwind<(), core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::` |
| 35.9% | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 35.9% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}:` |
| 35.9% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{imp` |
| 35.9% | `call_once<(), std::thread::lifecycle::spawn_unchecked::{closure}::{closure_env#0}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>>` |
| 35.9% | `{closure}<tokio::runtime::blocking::pool::{impl#6}::spawn_thread::{closure_env#0}, ()>` |
| 35.9% | `std::sys::backtrace::__rust_begin_short_backtrace::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure}, ()>` |
| 35.9% | `start_thread` |
| 35.9% | `tokio::run` |
| 35.8% | `__GI___clone3` |
| 35.6% | `run<tokio::runtime::blocking::schedule::BlockingSchedule>` |
| 31.5% | `<axum::routing::route::RouteFuture<core::convert::Infallible> as core::future::future::Future>::poll` |
| 31.4% | `poll<tower::util::boxed_clone_sync::BoxCloneSyncService<http::request::Request<axum_core::body::Body>, http::response::Response<axum_core::body::Body>, core::co` |
| 31.4% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<http::response::Response<axum_core::body::Body>, core::convert::Infallible>` |
| 31.2% | `tokio::runtime::task::raw::poll::<tokio::runtime::blocking::task::BlockingTask<<campfire_db::database::Database>::read<campfire_views::rooms::ShowView, campfire` |
| 31.2% | `poll<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<campfire_views::rooms::ShowView, campfire` |
| 31.2% | `{closure}<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<campfire_views::rooms::ShowView, cam` |
| 31.2% | `poll_inner<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<campfire_views::rooms::ShowView, ca` |
| 31.1% | `poll_future<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<campfire_views::rooms::ShowView, c` |
| 31.1% | `catch_unwind<core::task::poll::Poll<core::result::Result<campfire_views::rooms::ShowView, campfire_db::error::Error>>, core::panic::unwind_safe::AssertUnwindSaf` |
| 31.1% | `call_once<core::task::poll::Poll<core::result::Result<campfire_views::rooms::ShowView, campfire_db::error::Error>>, tokio::runtime::task::harness::poll_future::` |
| 31.1% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask<camp` |
| 31.1% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<tokio::runtime::task::harness::poll_future::{closure_env#0}<tokio::runtime::blocking::task::BlockingTask` |
| 31.1% | `poll<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<campfire_views::rooms::ShowView, campfire::controllers::rooms::render_show::{async_fn#` |
| 31.1% | `with_mut<tokio::runtime::task::core::Stage<tokio::runtime::blocking::task::BlockingTask<campfire_db::database::{impl#4}::read::{async_fn#0}::{closure_env#0}<cam` |
| 31.1% | `{closure}<campfire_views::rooms::ShowView, campfire::controllers::rooms::render_show::{async_fn#0}::{closure_env#0}>` |
| 31.1% | `with<campfire_views::rooms::ShowView, campfire::controllers::rooms::render_show::{async_fn#0}::{closure_env#0}>` |
| 27.5% | `campfire::app::dispatch_with_fragment_cache::{closure}` |
| 27.5% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>> + core::mark` |
| 27.4% | `poll<campfire::controllers::dispatch::{async_fn_env#0}>` |
| 27.4% | `with<core::task::poll::Poll<core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>>, campfire_views::fragment_cache::{impl#5}::poll:` |
| 27.3% | `{closure}<campfire::controllers::dispatch::{async_fn_env#0}>` |
| 27.2% | `poll<core::panic::unwind_safe::AssertUnwindSafe<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit::res` |
| 27.2% | `do_call<core::panic::unwind_safe::AssertUnwindSafe<futures_util::future::future::catch_unwind::{impl#1}::poll::{closure_env#0}<core::panic::unwind_safe::AssertU` |
| 27.2% | `catch_unwind<core::panic::unwind_safe::AssertUnwindSafe<futures_util::future::future::catch_unwind::{impl#1}::poll::{closure_env#0}<core::panic::unwind_safe::As` |
| 27.2% | `catch_unwind<core::task::poll::Poll<core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>>, core::panic::unwind_safe::AssertUnwindS` |
| 27.2% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error` |
| 27.2% | `call_once<core::task::poll::Poll<core::result::Result<campfire_kit::response::Response, campfire_kit::error::Error>>, futures_util::future::future::catch_unwind` |
| 27.2% | `{closure}<core::panic::unwind_safe::AssertUnwindSafe<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=core::result::Result<campfire_kit` |
| 26.0% | `campfire::controllers::rooms::show::{closure}` |
| 25.8% | `<axum::middleware::from_fn::Next>::run::{closure}` |
| 25.8% | `<axum::util::MapIntoResponseFuture<axum::routing::route::RouteFuture<core::convert::Infallible>> as core::future::future::Future>::poll` |
| 24.6% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 24.5% | `poll<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::alloc::Global>>` |
| 24.0% | `<axum::middleware::from_fn::FromFn<campfire::app::public_files, (), axum::routing::route::Route, (http::request::Request<axum_core::body::Body>,)> as tower_serv` |
| 23.3% | `<axum::util::MapIntoResponseFuture<axum::handler::future::IntoServiceFuture<core::pin::Pin<alloc::boxed::Box<dyn core::future::future::Future<Output = http::res` |
| 23.3% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 23.3% | `poll<core::pin::Pin<alloc::boxed::Box<(dyn core::future::future::Future<Output=http::response::Response<axum_core::body::Body>> + core::marker::Send), alloc::al` |
| 23.2% | `<campfire_kit::adapter::ActionHandler<campfire::app::dispatch_with_fragment_cache> as axum::handler::Handler<campfire_kit::adapter::ActionMarker, campfire_kit::` |
| 22.8% | `{async_fn#0}<campfire::controllers::rooms::render_show::{async_fn#0}::{closure_env#1}, campfire::controllers::rooms::render_show::{async_fn#0}::{closure_env#2}>` |
| 20.0% | `render<campfire::controllers::rooms::render_show::{async_fn#0}::{closure_env#1}>` |
| 19.3% | `render_with_values<campfire_views::rooms::Show>` |
| 19.3% | `render<campfire_views::rooms::Show>` |
| 19.2% | `<campfire_views::rooms::Show as askama::Template>::render_into_with_values::<alloc::string::String>` |
| 18.5% | `<campfire_kit::ctx::Ctx>::finish` |
| 17.6% | `<campfire_db::models::message::Message>::last_page` |
| 17.6% | `campfire_db::sql::query_all::<campfire_db::models::message::Message, [i64, 1], <campfire_db::models::message::Message>::from_row>` |
| 17.5% | `{closure}<campfire_db::models::message::Message, rusqlite::error::Error, alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, rusqlite:` |
| 17.5% | `collect<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusqlite::error::Error>>, core::result:` |
| 17.5% | `collect<core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusq` |
| 17.5% | `core::iter::adapters::try_process::<rusqlite::row::MappedRows<<campfire_db::models::message::Message>::from_row>, campfire_db::models::message::Message, core::r` |
| 17.5% | `from_iter<campfire_db::models::message::Message, rusqlite::error::Error, alloc::vec::Vec<campfire_db::models::message::Message, alloc::alloc::Global>, rusqlite:` |
| 17.5% | `from_iter<campfire_db::models::message::Message, core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<ca` |
| 17.2% | `extend_desugared<campfire_db::models::message::Message, alloc::alloc::Global, core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Ro` |
| 17.2% | `spec_extend<campfire_db::models::message::Message, core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<` |
| 17.2% | `campfire_kit::rack_etag` |
| 16.8% | `campfire_kit::new` |
| 16.8% | `<core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<<campfire_db::models::message::Message>::from_row>, core::result::Result<core::convert::Infallible` |
| 16.7% | `try_for_each<core::iter::adapters::GenericShunt<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message,` |
| 16.7% | `try_fold<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusqlite::error::Error>>, core::result` |
| 16.7% | `try_fold<rusqlite::row::MappedRows<fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusqlite::error::Error>>, (), core::it` |
| 16.6% | `next<campfire_db::models::message::Message, fn(&rusqlite::row::Row) -> core::result::Result<campfire_db::models::message::Message, rusqlite::error::Error>>` |
| 16.4% | `core::fmt::write` |
