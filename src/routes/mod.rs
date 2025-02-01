pub mod add_url;

use crate::{routes::add_url::get_add_url_router, state::AppContext};
use axum::Router;

pub fn establish_app_routes() -> Router<AppContext> {
    Router::new().nest("/url", get_add_url_router())
}
