use std::sync::Arc;

mod api;
mod app;
mod log;
mod role;
mod store;
mod version_ordering;

use app::App;
use log::Log;
pub use role::Role;
pub use version_ordering::VersionOrdering;

pub fn get() -> Arc<App> {
    App::get()
}
