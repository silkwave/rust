//! Controllers 모듈: HTTP 요청 처리 및 응답
//!
//! Spring MVC의 @RestController와 동일한 역할을 합니다.

pub mod board_controller; // 게시판 관련 HTTP 요청을 처리하는 핸들러 함수들
pub mod dto; // 데이터 전송 객체 (Request/Response 모델)
pub mod error; // 컨트롤러 계층의 에러 처리
