use crate::logging::{
    log_request_complete, log_request_start, log_template_render, log_user_creation,
    log_user_lookup, log_user_not_found,
};
use crate::models::{create_user, get_user_by_id, CreateUserRequest, UserResponse};
use crate::templates::{
    HealthTemplate, IndexTemplate, UserDetailTemplate, UserFormTemplate, UserNotFoundTemplate,
    UserSuccessTemplate, UsersTemplate,
};
use askama::Template;
use axum::{extract::Path, response::Html, routing::get, Form, Router};

// 핸들러들
pub async fn main_page() -> Html<String> {
    log_request_start("/", "GET");
    log_template_render("index.html");

    let template = IndexTemplate;
    let result = Html(template.render().unwrap());

    log_request_complete("/", "GET", 200);
    result
}

pub async fn create_user_form() -> Html<String> {
    log_request_start("/users/new", "GET");
    log_template_render("user_form.html");

    let template = UserFormTemplate;
    let result = Html(template.render().unwrap());

    log_request_complete("/users/new", "GET", 200);
    result
}

pub async fn handle_create_user(Form(request): Form<CreateUserRequest>) -> Html<String> {
    log_request_start("/users", "POST");
    log_user_creation(&request.username, &request.email);

    let user = create_user(request).await;
    log_template_render("user_success.html");

    let template = UserSuccessTemplate {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
    };
    let result = Html(template.render().unwrap());

    log_request_complete("/users", "POST", 200);
    result
}

pub async fn list_users() -> Html<String> {
    log_request_start("/users", "GET");
    log_template_render("users.html");

    // 샘플 데이터
    let users = vec![
        UserResponse {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        },
        UserResponse {
            id: 2,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            created_at: "2024-01-02T00:00:00Z".to_string(),
        },
    ];

    let template = UsersTemplate { users };
    let result = Html(template.render().unwrap());

    log_request_complete("/users", "GET", 200);
    result
}

pub async fn get_user_details(Path(user_id): Path<i64>) -> Html<String> {
    log_request_start(&format!("/users/{}", user_id), "GET");
    log_user_lookup(user_id);

    match get_user_by_id(user_id).await {
        Some(user) => {
            log_template_render("user_detail.html");
            let template = UserDetailTemplate {
                id: user.id,
                username: user.username,
                email: user.email,
                created_at: user.created_at,
            };
            let result = Html(template.render().unwrap());
            log_request_complete(&format!("/users/{}", user_id), "GET", 200);
            result
        }
        None => {
            log_user_not_found(user_id);
            log_template_render("user_not_found.html");
            let template = UserNotFoundTemplate;
            let result = Html(template.render().unwrap());
            log_request_complete(&format!("/users/{}", user_id), "GET", 404);
            result
        }
    }
}

pub async fn health_check() -> Html<String> {
    log_request_start("/health", "GET");
    log_template_render("health.html");

    let template = HealthTemplate;
    let result = Html(template.render().unwrap());

    log_request_complete("/health", "GET", 200);
    result
}

// 라우터 설정
pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/users", get(list_users).post(handle_create_user))
        .route("/users/new", get(create_user_form))
        .route("/users/:id", get(get_user_details))
        .route("/health", get(health_check))
}
