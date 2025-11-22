pub mod commands;
pub mod responses;

#[derive(Default)]
pub struct User {
    nickname: String,
    password: String,
}
