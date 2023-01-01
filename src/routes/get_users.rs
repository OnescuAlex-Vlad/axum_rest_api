use crate::database::users::Entity as Users;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    email: String,
    car_details: String,
    bank_details: String,
}

pub async fn get_one_user(
    Path(user_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let user = Users::find_by_id(user_id).one(&db).await.unwrap();
    if let Some(user) = user {
        Ok(Json(ResponseUser {
            id: user.id,
            username: user.username,
            email: user.email,
            car_details: user.car_details,
            bank_details: user.bank_details,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_users(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseUser>>, StatusCode> {
    let users = Users::find()
        .all(&db)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_user| ResponseUser {
            id: db_user.id,
            username: db_user.username,
            email: db_user.email,
            car_details: db_user.car_details,
            bank_details: db_user.bank_details,
        })
        .collect();
    Ok(Json(users))
}
