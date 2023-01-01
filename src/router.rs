use crate::{
    app_state::AppState,
    middleware::require_authentication::require_authentication,
    routes::{get_users::{get_users, get_one_user}, create_user::create_user, login::login, logout::logout}};
use axum::{
    middleware,
    routing::{post, get},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/api/v1/users", get(get_users))
        .route("/api/v1/users/:user_id", get(get_one_user))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}