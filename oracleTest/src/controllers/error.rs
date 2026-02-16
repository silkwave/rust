//! Controller 계층의 에러 처리를 담당하는 모듈

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tracing::error;

use crate::services::board_service::ServiceError;

/// 컨트롤러에서 발생하는 에러를 통합적으로 다루는 열거형
pub enum ControllerError {
    ServiceError(ServiceError),
    IoError(std::io::Error),
}

/// ServiceError를 ControllerError로 변환
impl From<ServiceError> for ControllerError {
    fn from(err: ServiceError) -> Self {
        ControllerError::ServiceError(err)
    }
}

/// std::io::Error를 ControllerError로 변환
impl From<std::io::Error> for ControllerError {
    fn from(err: std::io::Error) -> Self {
        ControllerError::IoError(err)
    }
}

/// ControllerError를 Axum의 Response로 변환하는 로직
/// - 이 구현을 통해 핸들러에서 `?` 연산자로 에러를 쉽게 반환할 수 있습니다.
impl IntoResponse for ControllerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ControllerError::ServiceError(service_error) => {
                // ServiceError를 HTTP 상태 코드로 매핑
                match service_error {
                    ServiceError::NotFound => (
                        StatusCode::NOT_FOUND,
                        "요청한 리소스를 찾을 수 없습니다.".to_string(),
                    ),
                    ServiceError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
                    ServiceError::DatabaseError(db_err) => {
                        error!("데이터베이스 오류 발생: {:?}", db_err);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "서버 내부 오류가 발생했습니다.".to_string(),
                        )
                    }
                }
            }
            ControllerError::IoError(io_err) => {
                error!("I/O 오류 발생: {:?}", io_err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "파일을 읽는 중 오류가 발생했습니다.".to_string(),
                )
            }
        };

        // 에러 응답을 JSON 형식으로 생성
        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
