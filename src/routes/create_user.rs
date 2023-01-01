use super::{RequestCreateUser, ResponseDataUser, ResponseUser};
use crate::queries::user_queries;
use crate::{
    database::users,
    utilities::{
        app_error::AppError, hash::hash_password, jwt::create_token, token_wrapper::TokenWrapper,
    },
};
use axum::{extract::State, Json};
use sea_orm::{DatabaseConnection, Set};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.email = Set(request_user.email.clone());
    new_user.car_details = Set(request_user.car_details.clone());
    new_user.bank_details = Set(request_user.bank_details.clone());
    new_user.token = Set(Some(create_token(&jwt_secret.0, request_user.username)?));
    let user = user_queries::save_active_user(&db, new_user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap(),
        },
    }))
}