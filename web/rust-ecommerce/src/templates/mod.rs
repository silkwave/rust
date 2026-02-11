use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "user_form.html")]
pub struct UserFormTemplate;

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<UserResponse>,
}

#[derive(Template)]
#[template(path = "user_detail.html")]
pub struct UserDetailTemplate {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Template)]
#[template(path = "user_success.html")]
pub struct UserSuccessTemplate {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Template)]
#[template(path = "user_not_found.html")]
pub struct UserNotFoundTemplate;

#[derive(Template)]
#[template(path = "health.html")]
pub struct HealthTemplate;

use crate::models::UserResponse;
