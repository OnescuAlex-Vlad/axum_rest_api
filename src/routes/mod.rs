use serde::{Deserialize, Serialize};

pub mod create_user;
pub mod get_users;
pub mod login;
pub mod logout;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    username: String,
    email: String,
    password: String,
    car_details: String,
    bank_details: String
}
