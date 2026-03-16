use std::sync::Arc;

use axum::{Router, middleware};
use tower_http::trace::TraceLayer;

use crate::{
    AppState,
    handlers::{auth::auth_handler, chats::chats_handler, user::users_handler},
    middleware::auth,
    websocket::ws_routes,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let public_routes = Router::<Arc<AppState>>::new().nest("/auth", auth_handler());

    let protected_routes = Router::<Arc<AppState>>::new()
        .merge(users_handler())
        .merge(chats_handler())
        .layer(middleware::from_fn_with_state(app_state.clone(), auth));

    let api_routes = Router::<Arc<AppState>>::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http());

    Router::<Arc<AppState>>::new()
        .nest("/api", api_routes)
        .merge(ws_routes())
        .with_state(app_state)
}
